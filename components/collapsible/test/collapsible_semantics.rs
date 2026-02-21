use std::fs;
use std::path::Path;

fn workspace_dir() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
        .to_path_buf()
}

fn load_source(rel_path: &str) -> String {
    if let Some(component_rel_path) = rel_path.strip_prefix("src/collapsible/") {
        let path = workspace_dir()
            .join("components/collapsible/src")
            .join(component_rel_path);
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn collapsible_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/collapsible/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Collapsible internals should stay private; found `{needle}`.",
        );
    }

    for forbidden in ["pub fn ", "impl ", "struct "] {
        assert!(
            !source.contains(forbidden),
            "Collapsible module boundary should not carry implementation token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_does_not_introduce_spec_rs_for_simple_contract() {
    let module_source = load_source("src/collapsible/mod.rs");
    let readme = load_source("src/collapsible/README.md");
    let spec_path = workspace_dir().join("components/collapsible/src/spec.rs");

    assert!(
        !spec_path.exists(),
        "collapsible is a simple disclosure component; spec.rs should not exist at {spec_path:?}.",
    );
    for forbidden in ["mod spec", "pub mod spec", "pub use spec::"] {
        assert!(
            !module_source.contains(forbidden),
            "collapsible module should not expose optional spec wiring token `{forbidden}`.",
        );
    }
    assert!(
        readme.contains("## API"),
        "simple component usage/contract should remain in README/checklist docs.",
    );
}

#[test]
fn collapsible_component_directory_standard_files_stay_in_canonical_layout() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = workspace_dir()
            .join("components/collapsible/src")
            .join(required);
        assert!(
            path.exists(),
            "collapsible component directory should keep required file `{path:?}`.",
        );
    }
    for forbidden in ["render.rs", "spec.rs"] {
        let path = workspace_dir()
            .join("components/collapsible/src")
            .join(forbidden);
        assert!(
            !path.exists(),
            "collapsible component directory should not introduce forbidden file `{path:?}`.",
        );
    }

    let mod_source = load_source("src/collapsible/mod.rs");
    let logic_source = load_source("src/collapsible/logic.rs");
    let styles_source = load_source("src/collapsible/styles.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let motion_source = load_source("src/collapsible/motion.rs");

    for required in [
        "mod logic;",
        "mod motion;",
        "mod styles;",
        "mod view;",
        "pub use view::Collapsible;",
        "pub use ui_disclosure::DisclosureMotion as CollapsibleMotion;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep stable export boundary marker `{required}`.",
        );
    }
    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub mod motion",
        "pub mod protocol",
        "pub fn ",
        "impl ",
        "struct ",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should remain a thin boundary file; found forbidden token `{forbidden}`.",
        );
    }

    for required in [
        "pub use ui_state_primitives::collapsible::{",
        "pub fn normalize_open_state_options(",
        "pub fn normalize_is_disabled(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization and primitive mapping marker `{required}`.",
        );
    }
    for forbidden in ["view!", "NodeRef<", "web_sys"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include rendering/DOM token `{forbidden}`.",
        );
    }

    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should consume token-first css variables.",
    );
    for forbidden in ["#", ":nth-child(", ":nth-of-type(", ":has(", "on:click"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not contain forbidden style token `{forbidden}`.",
        );
    }

    for required in [
        "logic::resolve_state(CollapsibleStateInput {",
        "use_button(ButtonOptions {",
        "use_focus_ring(FocusRingOptions {",
        "use_hover(HoverOptions {",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep structure + headless mount marker `{required}`.",
        );
    }
    for forbidden in ["ui_state_primitives::", "fn render("] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid forbidden token `{forbidden}`.",
        );
    }

    for required in [
        "pub fn sanitize_motion(",
        "pub fn attach_indicator_motion(",
        "pub fn attach_panel_motion(",
        "ui_disclosure::motion::attach_indicator_motion(",
        "ui_disclosure::motion::attach_panel_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep motion contract mapping marker `{required}`.",
        );
    }
    for forbidden in [
        "ui_motion::spring::SpringAnimator",
        "requestAnimationFrame",
        "keyframe",
        "aria-",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not contain forbidden engine/semantic token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_component_directory_standard_files_stay_in_canonical_layout";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "collapsible_component_directory_standard_files_stay_in_canonical_layout",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible checklist should keep directory-layout governance marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_file_layout_discipline_keeps_canonical_component_directory() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = workspace_dir()
            .join("components/collapsible/src")
            .join(required);
        assert!(
            path.exists(),
            "file-layout discipline requires `{path:?}` to exist.",
        );
    }
    for forbidden in ["render.rs", "spec.rs"] {
        let path = workspace_dir()
            .join("components/collapsible/src")
            .join(forbidden);
        assert!(
            !path.exists(),
            "file-layout discipline forbids `{path:?}` for simple collapsible scope.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_file_layout_discipline_keeps_canonical_component_directory";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should gate `{script_needle}`.",
    );

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "collapsible_file_layout_discipline_keeps_canonical_component_directory",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should preserve file-layout discipline marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_hyper_structure_builder_spec_rs_is_not_applicable_for_simple_component() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let mod_source = load_source("src/collapsible/mod.rs");
    let readme_source = load_source("src/collapsible/README.md");

    let spec_path = workspace_dir().join("components/collapsible/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "hyper-structure builder is not applicable for simple collapsible; unexpected `{spec_path:?}` found.",
    );
    for forbidden in ["mod spec", "pub mod spec", "pub use spec::", "Spec::new("] {
        assert!(
            !mod_source.contains(forbidden),
            "collapsible module should not expose hyper-structure builder token `{forbidden}`.",
        );
    }
    assert!(
        !readme_source.contains("Spec::new("),
        "simple collapsible docs should not imply mandatory hyper-structure builder API.",
    );

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_hyper_structure_builder_spec_rs_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should gate `{script_needle}`.",
    );

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`Collapsible` 为简单 disclosure 组件，无复杂 schema builder 需求）",
        "collapsible_hyper_structure_builder_spec_rs_is_not_applicable_for_simple_component",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep hyper-structure builder N/A marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let manifest_source = load_source("src/collapsible/Component.toml");
    let rbi_source = load_source("src/collapsible/collapsible.rbi");
    let component_toml_path = workspace_dir().join("components/collapsible/src/Component.toml");
    let component_rbi_path = workspace_dir().join("components/collapsible/src/collapsible.rbi");

    assert!(
        component_toml_path.exists(),
        "context compression manifest should exist at `{component_toml_path:?}`.",
    );
    assert!(
        component_rbi_path.exists(),
        "RBI signature projection should exist at `{component_rbi_path:?}`.",
    );

    for required in [
        "schema_version = \"1\"",
        "name = \"Collapsible\"",
        "crate = \"ui-collapsible\"",
        "rbi = \"collapsible.rbi\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub use crate::{",
        "CollapsibleAgentContract",
        "pub const COLLAPSIBLE_AGENT_SCHEMA: &str;",
        "pub fn Collapsible(",
        "open: Option<leptos::prelude::Signal<bool>>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "motion: CollapsibleMotion",
    ] {
        assert!(
            rbi_source.contains(required),
            "collapsible.rbi should keep signature projection marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "components/collapsible/src/Component.toml",
        "components/collapsible/src/collapsible.rbi",
        "collapsible_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should include context-compression evidence marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_agent_contract_schema_governance_rules() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "collapsible_agent_contract_is_schema_typed_and_machine_readable",
        "collapsible_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "collapsible_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep collapsible Agent Contract governance marker `{required}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene script should include `{script_needle}`.",
        );
    }
}

#[test]
fn collapsible_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/collapsible/logic.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let component_manifest = load_source("src/collapsible/Component.toml");
    let component_rbi = load_source("src/collapsible/collapsible.rbi");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let check2_source = load_source("../../components/collapsible/check2.md");

    for typed_source in [
        "pub const COLLAPSIBLE_AGENT_SCHEMA: &str = \"ui.collapsible.agent-contract\";",
        "pub enum CollapsibleAgentSchemaVersion",
        "pub enum CollapsibleAgentIntent",
        "pub enum CollapsibleAgentAction",
        "pub enum CollapsibleAgentState",
        "pub enum CollapsibleAgentSource",
        "pub struct CollapsibleAgentContract",
        "pub struct CollapsibleAgentContractInput",
        "fn resolve_agent_state(input: CollapsibleAgentContractInput) -> CollapsibleAgentState",
        "pub fn resolve_agent_contract(input: CollapsibleAgentContractInput) -> CollapsibleAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "collapsible Agent Contract should stay type-derived via `{typed_source}`.",
        );
    }

    for marker in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-state-source=move || agent_contract.get().state_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-open-value-source=move || agent_contract.get().open_value_source",
        "data-ui-open-change-source=move || agent_contract.get().open_change_source",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            view_source.contains(marker),
            "collapsible view should mount Agent Contract marker `{marker}`.",
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"agent_contract_whitelist_boundary\"",
        "[[agent_contract]]",
        "schema = \"ui.collapsible.agent-contract.v1\"",
        "intent = \"collapsible.interaction\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "COLLAPSIBLE_AGENT_SCHEMA",
        "CollapsibleAgentContract",
        "resolve_agent_contract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "collapsible context-compression assets should keep Agent Contract marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_is_schema_typed_and_machine_readable";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );

    assert!(
        check2_source.contains("collapsible_agent_contract_is_schema_typed_and_machine_readable"),
        "check2 should reference machine-readable Agent Contract regression.",
    );
}

#[test]
fn collapsible_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("src/collapsible/logic.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let check2_source = load_source("../../components/collapsible/check2.md");

    for marker in [
        "Self::V1 => \"v1\"",
        "Self::CollapsibleInteraction => \"collapsible.interaction\"",
        "Self::Toggle => \"toggle\"",
        "Self::Open => \"open\"",
        "Self::Disabled => \"disabled\"",
        "Self::StatePrimitives => \"state-primitives\"",
    ] {
        assert!(
            logic_source.contains(marker),
            "collapsible Agent Contract should keep closed typed mapping marker `{marker}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "data-ui-intent=\"",
        "data-ui-action=\"",
        "data-ui-state=\"",
        "data-ui-source=\"",
        "format!(\"data-ui-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "collapsible view should not splice free-form Agent Contract marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );

    assert!(
        check2_source.contains(
            "collapsible_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        ),
        "check2 should reference type-derived Agent Contract regression.",
    );
}

#[test]
fn collapsible_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/collapsible/view.rs");
    let component_manifest = load_source("src/collapsible/Component.toml");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let check2_source = load_source("../../components/collapsible/check2.md");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"logic::resolve_state(...)\"",
        "\"logic::resolve_agent_contract(...)\"",
        "\"collapsible_motion::attach_indicator_motion(...)\"",
        "\"collapsible_motion::attach_panel_motion(...)\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
        "name = \"agent_contract_whitelist_boundary\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "collapsible Component.toml should keep whitelist boundary marker `{required}`.",
        );
    }

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "collapsible render path should stay whitelist-safe and injection-free; found `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_render_path_is_whitelist_safe_and_script_injection_free";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );

    assert!(
        check2_source.contains(
            "collapsible_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        ),
        "check2 should reference whitelist-safe Agent Contract regression.",
    );
}

#[test]
fn collapsible_streaming_term_is_limited_to_llm_output_render_modes() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let logic_source = load_source("src/collapsible/logic.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let component_manifest = load_source("src/collapsible/Component.toml");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "collapsible_streaming_term_is_limited_to_llm_output_render_modes",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should record streaming-term governance marker `{required}`.",
        );
    }

    for marker in [
        "pub enum CollapsibleAgentStreamMode",
        "Self::Streaming => \"streaming\"",
        "Self::Snapshot => \"snapshot\"",
        "stream_support: CollapsibleAgentStreamSupport::Unsupported,",
        "stream_fallback: CollapsibleAgentStreamFallback::Snapshot,",
        "stream_mode: CollapsibleAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic.rs should keep LLM render mode marker `{marker}`.",
        );
    }

    for marker in [
        "[streaming_policy]",
        "term_scope = \"llm-output-rendering\"",
        "defined_modes = [\"streaming\", \"snapshot\"]",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
    ] {
        assert!(
            component_manifest.contains(marker),
            "Component.toml should keep streaming-term scope marker `{marker}`.",
        );
    }

    for marker in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should expose machine-readable stream marker `{marker}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_streaming_term_is_limited_to_llm_output_render_modes";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );
}

#[test]
fn collapsible_snapshot_is_foundational_and_complete_config_renders_stably() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let logic_source = load_source("src/collapsible/logic.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let component_manifest = load_source("src/collapsible/Component.toml");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "collapsible_snapshot_is_foundational_and_complete_config_renders_stably",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep snapshot-foundation marker `{marker}`.",
        );
    }

    for marker in [
        "pub fn Collapsible(",
        "id_base: String,",
        "title: String,",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] motion: CollapsibleMotion,",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] dir: Option<String>",
        "children: Children,",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should keep complete-config snapshot render marker `{marker}`.",
        );
    }

    for marker in [
        "Self::Verified => \"verified\"",
        "output_status: CollapsibleAgentOutputStatus::Verified,",
        "Self::Snapshot => \"snapshot\"",
        "stream_mode: CollapsibleAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic.rs should keep snapshot/output-status marker `{marker}`.",
        );
    }

    for marker in [
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should expose snapshot output marker `{marker}`.",
        );
    }

    for marker in [
        "name = \"snapshot_rendering\"",
        "name = \"streaming_optional_fallback_snapshot\"",
        "[streaming_policy]",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            component_manifest.contains(marker),
            "Component.toml should keep snapshot-foundation marker `{marker}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_snapshot_is_foundational_and_complete_config_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );
}

#[test]
fn collapsible_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let logic_source = load_source("src/collapsible/logic.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let component_manifest = load_source("src/collapsible/Component.toml");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "collapsible_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep streaming-requirement marker `{marker}`.",
        );
    }

    for marker in [
        "stream_support: CollapsibleAgentStreamSupport::Unsupported,",
        "stream_fallback: CollapsibleAgentStreamFallback::Snapshot,",
        "stream_mode: CollapsibleAgentStreamMode::Snapshot,",
        "output_status: CollapsibleAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic.rs should keep optional-streaming marker `{marker}`.",
        );
    }

    for marker in [
        "[streaming_policy]",
        "required = false",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
    ] {
        assert!(
            component_manifest.contains(marker),
            "Component.toml should keep optional-streaming policy marker `{marker}`.",
        );
    }

    for marker in [
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "role=aria.attrs.role",
        "aria-expanded=trigger_a11y.aria_expanded",
        "aria-controls=trigger_a11y.aria_controls",
        "data-state=move || state.get().state_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should keep output-status and semantic continuity marker `{marker}`.",
        );
    }

    for forbidden in [
        "retry",
        "reconnect",
        "validate_stream_chunk",
        "stream parser",
        "network error",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "stream validation/recovery should stay upstream; found component-local token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );
}

#[test]
fn collapsible_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/collapsible/mod.rs");
    let crate_source = load_source("src/lib.rs");
    let cargo_source = load_source("Cargo.toml");

    assert!(
        module_source.contains("pub use view::Collapsible;"),
        "collapsible module should export `Collapsible`.",
    );
    assert!(
        module_source.contains("CollapsibleMotion"),
        "collapsible module should expose a motion contract alias.",
    );
    assert!(
        crate_source.contains("pub use ui_collapsible as collapsible;"),
        "crate root should re-export external ui-collapsible crate as `collapsible`.",
    );
    assert!(
        crate_source.contains("pub use collapsible::{Collapsible, CollapsibleMotion};"),
        "crate root prelude should re-export `Collapsible` and `CollapsibleMotion`.",
    );
    assert!(
        cargo_source.contains("component-collapsible = [\"dep:ui-collapsible\"]"),
        "component-collapsible feature should depend on dep:ui-collapsible after extraction.",
    );
    assert!(
        cargo_source.contains(
            "ui-collapsible = { path = \"../../components/collapsible\", optional = true }"
        ),
        "ui-components Cargo.toml should include optional ui-collapsible dependency.",
    );
}

#[test]
fn collapsible_logic_exposes_state_helpers() {
    let source = load_source("src/collapsible/logic.rs");

    for needle in [
        "pub use ui_state_primitives::collapsible::{",
        "CollapsibleOpenState",
        "CollapsibleOpenStateOptions",
        "normalize_optional_text",
        "normalize_id_base",
        "resolve_title",
        "resolve_aria_label",
        "resolve_state",
        "use_collapsible_open_state",
        "CollapsibleOpenValueSource",
        "CollapsibleOpenChangeSource",
        "normalize_open_value_source",
        "normalize_open_change_source",
        "normalize_dir",
        "pub fn compose_class_name(class_name: Option<String>, state: CollapsibleState)",
        "DEFAULT_ID_BASE",
        "DEFAULT_TITLE",
    ] {
        assert!(
            source.contains(needle),
            "Collapsible logic should include `{needle}` for centralized normalization/state contracts.",
        );
    }

    assert!(
        !source.contains("pub fn resolve_state(input: CollapsibleStateInput)"),
        "Collapsible state primitive implementation should live in ui-state-primitives.",
    );
    for forbidden in ["NodeRef<", "web_sys", "color-mix("] {
        assert!(
            !source.contains(forbidden),
            "Collapsible logic should avoid DOM/style implementation token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_view_uses_logic_state_and_motion_contracts() {
    let source = load_source("src/collapsible/view.rs");

    for needle in [
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] dir: Option<String>",
        "logic::use_collapsible_open_state(",
        "logic::normalize_open_state_options(",
        "logic::normalize_open_value_source(open_input, default_open)",
        "let dir = logic::normalize_dir(dir);",
        "let locale = locale_attrs(lang, dir);",
        "let open_change_source = RwSignal::new(logic::CollapsibleOpenChangeSource::Initial);",
        "logic::normalize_status(open.get(), is_disabled)",
        "logic::normalize_open_mode(open_state.with(|state| state.is_controlled()))",
        "logic::normalize_label_source(has_custom_aria_label)",
        "logic::normalize_class_source(has_custom_class_name)",
        "logic::normalize_motion_source(has_custom_motion)",
        "let open_change_source = open_change_source.get();",
        "logic::should_emit_open_change(current, next)",
        "logic::compute_next_open(open.get_untracked())",
        "open_change_source.set(logic::normalize_open_change_source(true));",
        "logic::apply_open_change(state, open_prop.map(|value| value.get_untracked()), next);",
        "open_state.update(|state| state.sync_controlled(open_prop.map(|value| value.get())))",
        "open_change_source.set(logic::normalize_open_change_source(false));",
        "let trigger_a11y = disclosure_trigger_attrs(open, panel_id.clone(), locale.lang.clone(), dir);",
        "logic::normalize_id_base(id_base)",
        "logic::resolve_title(title)",
        "logic::resolve_aria_label(&title, aria_label)",
        "logic::normalize_is_disabled(is_disabled, disabled)",
        "logic::resolve_state(CollapsibleStateInput {",
        "logic::compose_class_name(normalized_class_name.get_value(), state.get())",
        "data-slot=\"collapsible\"",
        "data-state=move || state.get().state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-open-value-source=move || state.get().open_value_source_attr",
        "data-open-change-source=move || state.get().open_change_source_attr",
        "data-custom-motion=move || state.get().motion_source.is_custom().then_some(\"true\")",
        "data-custom-class=move || state.get().class_source.is_custom().then_some(\"true\")",
        "crate::motion::attach_indicator_motion(indicator_ref, open, motion);",
        "crate::motion::attach_panel_motion(panel_ref, panel_surface_ref, open, panel_hidden, motion);",
        "use_button(ButtonOptions {",
        "use_focus_ring(FocusRingOptions {",
        "use_hover(HoverOptions {",
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
        "lang=move || locale_lang.get_value()",
        "dir=move || locale_dir.get_value()",
        "lang=trigger_a11y.lang",
        "dir=trigger_a11y.dir",
        "aria-expanded=trigger_a11y.aria_expanded",
        "aria-controls=trigger_a11y.aria_controls",
        "on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())",
        "on:keydown=move |ev| {",
        "on:keyup=move |ev| {",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| {",
        "data-hovered=move || if hover.is_hovered.get() { Some(\"true\") } else { None }",
        "data-pressed=move || if aria.is_pressed.get() { Some(\"true\") } else { None }",
    ] {
        assert!(
            source.contains(needle),
            "Collapsible view should include `{needle}` for stable state/source/motion contracts.",
        );
    }

    assert!(
        !source.contains("use_controllable_open_state_traced("),
        "Collapsible should consume open-state primitives from ui-state-primitives instead of ui-headless local controllable state wiring.",
    );
    assert!(
        !source.contains("ui_state_primitives::"),
        "Collapsible view should consume state primitives via component logic boundary rather than directly binding primitive modules.",
    );
    for forbidden in ["is_loading", "aria-busy", "retry", "use_async_action"] {
        assert!(
            !source.contains(forbidden),
            "collapsible is non-async; view should not include async protocol token `{forbidden}`.",
        );
    }
    assert!(
        !source.contains("ui_disclosure::motion::attach_indicator_motion("),
        "Collapsible motion attachment should go through local motion.rs contract mapping.",
    );
    assert!(
        !source.contains("ui_disclosure::motion::attach_panel_motion("),
        "Collapsible motion attachment should go through local motion.rs contract mapping.",
    );
    for forbidden in [
        "\"Advanced options\"",
        "\"Disabled section\"",
        "\"Interactive collapsible\"",
        "\"Panel content with disclosure-level semantics.\"",
    ] {
        assert!(
            !source.contains(forbidden),
            "Collapsible view should not hardcode user-facing copy `{forbidden}`; copy should come from props/injection.",
        );
    }
}

#[test]
fn collapsible_motion_maps_to_disclosure_contract_without_engine_reimplementation() {
    let source = load_source("src/collapsible/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DisclosureMotion) -> DisclosureMotion {",
        "ui_disclosure::motion::sanitize_motion(motion)",
        "pub fn attach_indicator_motion(",
        "ui_disclosure::motion::attach_indicator_motion(",
        "pub fn attach_panel_motion(",
        "ui_disclosure::motion::attach_panel_motion(",
    ] {
        assert!(
            source.contains(needle),
            "Collapsible motion contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "ui_motion::spring::SpringAnimator",
        "requestAnimationFrame",
        "keyframe",
    ] {
        assert!(
            !source.contains(forbidden),
            "Collapsible motion should not reimplement animation engine token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_css_contains_state_mode_and_motion_markers() {
    let css = load_source("src/collapsible/styles.rs");

    for needle in [
        ".ui-collapsible {",
        ".ui-collapsible--state-open",
        ".ui-collapsible[data-state=\"disabled\"]",
        ".ui-collapsible[data-open-mode=\"controlled\"]",
        ".ui-collapsible[data-motion-source=\"custom\"]",
        ".ui-collapsible[data-custom-motion=\"true\"]",
        ".ui-collapsible--custom-class",
        "@media (forced-colors: active)",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            css.contains(needle),
            "Collapsible CSS should include `{needle}` selector.",
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type(", ":has("] {
        assert!(
            !css.contains(forbidden),
            "Collapsible CSS should avoid fragile structural selector `{forbidden}`; rely on semantic data/class markers.",
        );
    }
}

#[test]
fn collapsible_css_consumes_ui_theme_variables_without_private_color_tokens() {
    let css = load_source("src/collapsible/styles.rs");

    for needle in [
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
    ] {
        assert!(
            css.contains(needle),
            "Collapsible CSS should consume theme variable `{needle}`.",
        );
    }

    assert!(
        !css.contains('#'),
        "Collapsible CSS should not hardcode hex colors; consume ui-theme variables instead.",
    );
}

#[test]
fn collapsible_view_avoids_business_inline_styles() {
    let view = load_source("src/collapsible/view.rs");

    assert!(
        !view.contains("style="),
        "Collapsible view should avoid business inline styles; runtime tuning should use semantic markers and CSS variables.",
    );
}

#[test]
fn collapsible_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("#[cfg(feature = \"component-collapsible\")]"),
        "ui-components css aggregator should keep collapsible styles behind component feature gates.",
    );
    assert!(
        source.contains("out.push_str(crate::collapsible::styles::CSS);"),
        "ui-components css aggregator should include collapsible styles.",
    );
}

#[test]
fn collapsible_css_aggregation_flows_through_ui_root_injection_gate() {
    let root_source = load_source("src/root.rs");

    assert!(
        root_source.contains("if inject_components_css.get_value()"),
        "UiRoot should keep component CSS injection behind the inject_components_css gate.",
    );
    assert!(
        root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should inject aggregated component styles through crate::css::push_components_css.",
    );
}

#[test]
fn collapsible_cascade_layer_and_runtime_style_contract_is_enforced() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let logic_source = load_source("src/collapsible/logic.rs");
    let motion_source = load_source("src/collapsible/motion.rs");

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-collapsible\")]",
        "out.push_str(crate::collapsible::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css aggregation should keep cascade-layer marker `{needle}`.",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep components css injection path marker `{needle}`.",
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style:top=",
        "style:left=",
        "style:right=",
        "style:bottom=",
        "style:width=",
        "style:height=",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "collapsible view/logic should not embed plain inline style token `{forbidden}`.",
        );
    }

    for (line_index, line) in view_source.lines().enumerate() {
        if let Some(pos) = line.find("style:") {
            let key = line[pos + "style:".len()..]
                .split(|c: char| c == '=' || c.is_whitespace() || c == '>')
                .next()
                .unwrap_or_default()
                .trim();
            assert!(
                key.starts_with("--"),
                "collapsible runtime style should only set css custom properties; found `style:{key}` at line {}",
                line_index + 1,
            );
        }
    }

    for forbidden in [
        " top:",
        " left:",
        " right:",
        " bottom:",
        " width:",
        " height:",
        " padding:",
        " margin:",
        " background:",
        " border:",
        " color:",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "collapsible motion mapping should avoid non-variable inline style token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "collapsible_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(needle),
            "collapsible check2 should keep cascade-layer governance marker `{needle}`.",
        );
    }
}

#[test]
fn collapsible_component_layer_avoids_utility_first_and_css_in_rust_defaults() {
    let css = load_source("src/collapsible/styles.rs");

    for forbidden in ["@apply", "tailwind", " tw-", "styled(", "style!"] {
        assert!(
            !css.contains(forbidden),
            "component CSS should not adopt utility-first/CSS-in-Rust default token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_visual_desire_contract_has_feedback_cues_and_shared_theme_baseline() {
    let styles = load_source("src/collapsible/styles.rs");
    let view = load_source("src/collapsible/view.rs");
    let docs_collapsible =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let theme_baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let theme_baseline_e2e = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "color-mix(in oklch",
        "transition:",
        "border-color var(--ui-collapsible-motion-duration) var(--ui-collapsible-motion-easing)",
        "background-color var(--ui-collapsible-motion-duration) var(--ui-collapsible-motion-easing)",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            styles.contains(needle),
            "collapsible styles should include visual quality cue `{needle}`.",
        );
    }

    for needle in [
        "use_focus_ring(FocusRingOptions { is_disabled })",
        "use_hover(HoverOptions { is_disabled })",
        "class:ui-disclosure__trigger--focus-visible=move || focus_ring.is_focus_visible.get()",
        "data-hovered=move || if hover.is_hovered.get() { Some(\"true\") } else { None }",
        "data-pressed=move || if aria.is_pressed.get() { Some(\"true\") } else { None }",
    ] {
        assert!(
            view.contains(needle),
            "collapsible view should mount feedback contract `{needle}`.",
        );
    }

    for needle in [
        "title=\"Hello World\"",
        "title=\"Interactive Playground (Display + Config + Code + CSS Test)\"",
        "docs-collapsible-interactive",
    ] {
        assert!(
            docs_collapsible.contains(needle),
            "collapsible docs page should include visual validation path `{needle}`.",
        );
    }

    for needle in [
        "slug=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            theme_baseline_page.contains(needle),
            "shared theme visual baseline page should include `{needle}`.",
        );
    }

    for needle in [
        "docs-app: theme visual baseline screenshots",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            theme_baseline_e2e.contains(needle),
            "shared theme visual baseline e2e should include `{needle}`.",
        );
    }
}

#[test]
fn collapsible_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/collapsible/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
    ] {
        assert!(
            styles_source.contains(needle),
            "collapsible styles should keep defensive fallback chain marker `{needle}`.",
        );
    }

    for needle in [
        "--ui-fallback-disabled-opacity:",
        "--ui-fallback-text-field-motion-duration:",
        "--ui-fallback-text-field-motion-easing:",
        "--ui-fallback-accent:",
        "--ui-fallback-border:",
        "--ui-fallback-bg:",
        "--ui-fallback-accent-soft:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme css should provide fallback terminal `{needle}`.",
        );
    }

    for forbidden in [
        "opacity: 0.72;",
        "border-color 200ms ease",
        "background-color 200ms ease",
        "box-shadow 200ms ease",
        "#",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "collapsible styles should avoid raw terminal token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "collapsible_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(needle),
            "collapsible check2 should keep defensive-variable governance marker `{needle}`.",
        );
    }
}

#[test]
fn collapsible_tree_shaking_contract_keeps_feature_gates_explicit() {
    let cargo = load_source("Cargo.toml");
    let lib = load_source("src/lib.rs");
    let css = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");

    assert!(
        cargo.contains("component-collapsible = [\"dep:ui-collapsible\"]"),
        "ui-components should expose component-level feature gate for collapsible.",
    );
    assert!(
        cargo.contains(
            "ui-collapsible = { path = \"../../components/collapsible\", optional = true }"
        ),
        "ui-collapsible dependency should remain optional for feature-level tree shaking.",
    );

    let export_needle = "pub use ui_collapsible as collapsible;";
    let export_idx = lib
        .find(export_needle)
        .unwrap_or_else(|| panic!("lib.rs should contain `{export_needle}` export"));
    let cfg_idx = lib[..export_idx]
        .rfind("#[cfg(feature = \"component-collapsible\")]")
        .unwrap_or_else(|| panic!("collapsible export should be feature-gated in lib.rs"));
    assert!(
        cfg_idx < export_idx,
        "component-collapsible cfg should guard collapsible export in lib.rs.",
    );
    assert_eq!(
        lib.matches(export_needle).count(),
        1,
        "lib.rs should not duplicate collapsible export paths that might bypass feature gates.",
    );

    let css_push_needle = "out.push_str(crate::collapsible::styles::CSS);";
    let css_push_idx = css.find(css_push_needle).unwrap_or_else(|| {
        panic!("css.rs should aggregate collapsible CSS via `{css_push_needle}`")
    });
    let css_cfg_idx = css[..css_push_idx]
        .rfind("#[cfg(feature = \"component-collapsible\")]")
        .unwrap_or_else(|| panic!("collapsible CSS aggregation should be feature-gated in css.rs"));
    assert!(
        css_cfg_idx < css_push_idx,
        "component-collapsible cfg should guard collapsible CSS push in css.rs.",
    );
    assert!(
        css.contains("#[cfg(feature = \"inject-css\")]"),
        "css aggregation entry should remain behind inject-css feature gate.",
    );

    assert!(
        web_demo_cargo
            .contains("ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }"),
        "web-demo should opt into scoped feature bundle instead of default all-components.",
    );
    assert!(
        !web_demo_cargo.contains("\"all-components\""),
        "web-demo dependency config should not force all-components feature.",
    );
}

#[test]
fn collapsible_api_stays_non_composite_and_explicit() {
    let source = load_source("src/collapsible/view.rs");
    let docs = load_source("src/collapsible/README.md");

    assert!(
        source.contains("children: Children"),
        "collapsible should keep explicit composition via `children`.",
    );

    for forbidden in [
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "#[prop(optional)] items:",
        "labels + children",
        "titles + panels",
        "ItemSpec",
    ] {
        assert!(
            !source.contains(forbidden),
            "collapsible should not expose parallel-array composite API token `{forbidden}`.",
        );
        assert!(
            !docs.contains(forbidden),
            "collapsible docs should not promote composite shorthand token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_macro_micro_duality_is_not_applicable_without_drag_contract() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");

    for forbidden in [
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "Dragging",
        "DragEnd",
        "requestAnimationFrame",
        "Action::DragEnd",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible has no drag interaction; view.rs should not contain `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "collapsible has no drag macro-state; logic.rs should not contain `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "collapsible has no drag micro-loop; motion.rs should not contain `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_two_pass_rendering_is_not_applicable_without_geometry_measurement() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");

    for forbidden in [
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "ResizeObserver",
        "IntersectionObserver",
        "Intent",
        "Rectification",
        "Measure",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible has no geometry two-pass rendering; view.rs should not contain `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "collapsible has no geometry rectification pass; logic.rs should not contain `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "collapsible motion should not embed layout measurement token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_focus_stack_gc_is_not_applicable_without_overlay_layering() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let docs = load_source("src/collapsible/README.md");

    for forbidden in [
        "Overlay",
        "FocusManager",
        "FocusStack",
        "FallbackTo",
        "restore_focus",
        "focus_restore",
        "previously_focused",
        "document.body",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible is not an overlay stack; view.rs should not contain `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "collapsible is not an overlay stack; logic.rs should not contain `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "collapsible is not an overlay stack; motion.rs should not contain `{forbidden}`.",
        );
        assert!(
            !docs.contains(forbidden),
            "collapsible docs should not imply overlay focus-stack token `{forbidden}`.",
        );
    }

    for needle in [
        "indicator_ref: NodeRef<html::Span>",
        "panel_ref: NodeRef<html::Div>",
        "panel_surface_ref: NodeRef<html::Div>",
        "crate::motion::attach_indicator_motion(indicator_ref, open, motion);",
        "crate::motion::attach_panel_motion(panel_ref, panel_surface_ref, open, panel_hidden, motion);",
    ] {
        assert!(
            view.contains(needle),
            "NodeRef usage should stay motion-local in view.rs via `{needle}`.",
        );
    }

    for forbidden in [
        "trigger_ref",
        "restore_target_ref",
        "focus_restore_ref",
        "previous_focus_ref",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible should not keep private focus-restore NodeRef token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_escape_hatches_are_not_applicable_without_foreign_imperative_runtime() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let module = load_source("src/collapsible/mod.rs");
    let docs = load_source("src/collapsible/README.md");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "google.maps",
        "AMap",
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "foreign_instance",
        "external_instance",
        "imperative_instance",
        "js_sys::Object",
        "wasm_bindgen::JsValue",
        "web_sys::HtmlCanvasElement",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible has no foreign imperative runtime; view.rs should not contain `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "collapsible has no foreign imperative runtime; logic.rs should not contain `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "collapsible has no foreign imperative runtime; motion.rs should not contain `{forbidden}`.",
        );
        assert!(
            !module.contains(forbidden),
            "collapsible public module should not expose foreign runtime token `{forbidden}`.",
        );
        assert!(
            !docs.contains(forbidden),
            "collapsible docs should not imply foreign runtime token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_hydration_ids_are_deterministic_without_time_or_random_seed() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let primitive = load_source("../../crates/ui-state-primitives/src/collapsible.rs");
    let disclosure_logic = load_source("../../components/disclosure/src/logic.rs");

    for forbidden in [
        "now(",
        "SystemTime",
        "UNIX_EPOCH",
        "uuid",
        "UUID",
        "rand::",
        "thread_rng",
        "random(",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not derive hydration ids from non-deterministic token `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not derive hydration ids from non-deterministic token `{forbidden}`.",
        );
        assert!(
            !primitive.contains(forbidden),
            "state primitive should not derive id normalization from non-deterministic token `{forbidden}`.",
        );
        assert!(
            !disclosure_logic.contains(forbidden),
            "disclosure ids should not derive ids from non-deterministic token `{forbidden}`.",
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let ids = DisclosureIds::new(&id_base);",
        "id=trigger_id.clone()",
        "id=panel_id",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should keep deterministic hydration id chain `{needle}`.",
        );
    }

    assert!(
        primitive.contains("pub fn normalize_id_base(value: String) -> String"),
        "id normalization should stay in ui-state-primitives.",
    );
    assert!(
        disclosure_logic.contains("trigger_id: format!(\"{id_base}-trigger\")"),
        "disclosure ids should deterministically derive trigger id from id_base.",
    );
    assert!(
        disclosure_logic.contains("panel_id: format!(\"{id_base}-panel\")"),
        "disclosure ids should deterministically derive panel id from id_base.",
    );
}

#[test]
fn collapsible_ssr_cross_platform_contracts_keep_non_wasm_paths_safe() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let motion_lib = load_source("../../crates/ui-motion/src/lib.rs");

    for forbidden in [
        "web_sys::",
        "window.",
        "document.",
        "HtmlCanvasElement",
        "wasm_bindgen::JsValue",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should keep non-wasm path free of browser-only token `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "logic.rs should keep non-wasm path free of browser-only token `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "motion.rs should keep non-wasm path free of browser-only token `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should keep explicit web/ssr mutual-exclusion guard `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib.contains(needle),
            "ui-motion should keep wasm/non-wasm explicit branch contract `{needle}`.",
        );
    }
}

#[test]
fn collapsible_ui_headless_web_ssr_feature_mutex_contract_is_preserved() {
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let cargo = load_source("../../components/collapsible/Cargo.toml");
    let view = load_source("src/collapsible/view.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should preserve web/ssr mutex guard `{needle}`.",
        );
    }

    assert!(
        cargo.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "collapsible should consume ui-headless as a dependency boundary.",
    );
    assert!(
        !cargo.contains("features = [\"web\", \"ssr\"]"),
        "component dependency config must not explicitly enable both web+ssr on ui-headless.",
    );

    for needle in [
        "use_button",
        "use_focus_ring",
        "use_hover",
        "disclosure_trigger_attrs",
        "locale_attrs",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should consume headless contracts via `{needle}`.",
        );
    }
}

#[test]
fn collapsible_ui_motion_non_wasm_noop_contract_is_preserved() {
    let motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let component_motion = load_source("src/collapsible/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib.contains(needle),
            "ui-motion should preserve non-wasm no-op contract token `{needle}`.",
        );
    }

    for needle in [
        "ui_disclosure::motion::sanitize_motion(",
        "ui_disclosure::motion::attach_indicator_motion(",
        "ui_disclosure::motion::attach_panel_motion(",
    ] {
        assert!(
            component_motion.contains(needle),
            "component motion mapping should delegate to shared disclosure contract `{needle}`.",
        );
    }

    for forbidden in ["panic!", "unwrap()", "expect("] {
        assert!(
            !component_motion.contains(forbidden),
            "component motion non-wasm path should not assume runtime handles via `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_reduced_motion_ssr_wasm_contracts_stay_convergent() {
    let view = load_source("src/collapsible/view.rs");
    let styles = load_source("src/collapsible/styles.rs");
    let disclosure_motion = load_source("../../components/disclosure/src/motion.rs");
    let spring = load_source("../../crates/ui-motion/src/spring.rs");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        "transition: none;",
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            styles.contains(needle) || spring.contains(needle),
            "reduced-motion contract should keep token `{needle}` in css/runtime motion paths.",
        );
    }

    for needle in [
        "let ids = DisclosureIds::new(&id_base);",
        "let panel_hidden = RwSignal::new(!open.get_untracked());",
        "hidden=move || panel_hidden.get()",
        "aria-controls=trigger_a11y.aria_controls",
        "aria-labelledby=trigger_id",
    ] {
        assert!(
            view.contains(needle),
            "SSR/hydration-compatible output should preserve semantic token `{needle}`.",
        );
    }

    for forbidden in ["now()", "SystemTime::now", "Uuid::new", "rand::thread_rng"] {
        assert!(
            !view.contains(forbidden),
            "view.rs should keep hydration path deterministic; found forbidden token `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_indicator_motion(",
        "pub fn attach_panel_motion(",
        "is_hidden.set(!is_open.get());",
    ] {
        assert!(
            disclosure_motion.contains(needle),
            "disclosure motion should preserve wasm/non-wasm branch contract token `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-open-value-source=move || state.get().open_value_source_attr",
        "data-open-change-source=move || state.get().open_change_source_attr",
        "aria-expanded=trigger_a11y.aria_expanded",
    ] {
        assert!(
            view.contains(needle),
            "semantic markers must remain stable across SSR/wasm paths via `{needle}`.",
        );
    }

    assert!(
        !view.contains("#[cfg("),
        "view.rs should avoid target-specific semantic forks; keep a single semantic contract output.",
    );
}

#[test]
fn collapsible_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let mod_source = load_source("src/collapsible/mod.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let motion_source = load_source("src/collapsible/motion.rs");
    let styles_source = load_source("src/collapsible/styles.rs");
    let disclosure_motion_source = load_source("../../components/disclosure/src/motion.rs");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "pub use ui_disclosure::DisclosureMotion as CollapsibleMotion;",
        "#[prop(optional)] motion: CollapsibleMotion,",
        "crate::motion::sanitize_motion(motion);",
    ] {
        assert!(
            mod_source.contains(needle) || view_source.contains(needle),
            "collapsible public/view layer should expose built-in motion contract marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: DisclosureMotion) -> DisclosureMotion {",
        "ui_disclosure::motion::sanitize_motion(motion)",
        "pub fn attach_indicator_motion(",
        "ui_disclosure::motion::attach_indicator_motion(",
        "pub fn attach_panel_motion(",
        "ui_disclosure::motion::attach_panel_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "collapsible motion adapter should keep contract mapping marker `{needle}`.",
        );
    }

    for needle in ["SpringConfig {", "stiffness: 260.0,", "damping: 18.0,"] {
        assert!(
            disclosure_motion_source.contains(needle),
            "shared disclosure motion contract should keep built-in spring marker `{needle}`.",
        );
    }

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        "transition: none;",
    ] {
        assert!(
            styles_source.contains(needle),
            "collapsible styles should keep reduced-motion downgrade marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should keep non-wasm safe no-op marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "collapsible_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
    ] {
        assert!(
            check2_source.contains(needle),
            "collapsible check2 should keep motion-contract governance marker `{needle}`.",
        );
    }
}

#[test]
fn collapsible_ui_components_fixed_entry_files_follow_contract() {
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for required in [
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-collapsible\")]",
        "pub use ui_collapsible as collapsible;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui-components lib.rs should keep fixed entry marker `{required}`.",
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-collapsible\")]",
        "out.push_str(crate::collapsible::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css.rs should keep fixed entry marker `{required}`.",
        );
    }

    for required in [
        "out.push_str(css::BASE_CSS);",
        "theme.get().to_css_variables()",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
    ] {
        assert!(
            root_source.contains(required),
            "ui-components root.rs should keep fixed entry marker `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight primitive should keep shared capability marker `{required}`.",
        );
    }
    assert!(
        !active_highlight_source.contains("collapsible"),
        "active_highlight primitive should stay component-agnostic and avoid component business tokens.",
    );

    for rel in [
        "crates/ui-components/src/overlay_open.rs",
        "crates/ui-components/src/presence.rs",
        "crates/ui-components/src/a11y.rs",
    ] {
        let path = workspace_dir().join(rel);
        assert!(
            !path.exists(),
            "ui-components fixed entry contract forbids `{path:?}`; state/a11y primitives must live in ui-headless.",
        );
    }

    for rel in [
        "crates/ui-headless/src/controllable_state.rs",
        "crates/ui-headless/src/presence.rs",
        "crates/ui-headless/src/a11y.rs",
    ] {
        let path = workspace_dir().join(rel);
        assert!(
            path.exists(),
            "ui-headless source-of-truth file `{path:?}` must exist for fixed entry contract.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_ui_components_fixed_entry_files_follow_contract";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "collapsible_ui_components_fixed_entry_files_follow_contract",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible checklist should keep fixed-entry governance marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_registration_protocol_is_not_applicable_for_single_item_disclosure() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let docs = load_source("src/collapsible/README.md");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible is not a dynamic item-collection; view.rs should not contain `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "collapsible is not a dynamic item-collection; logic.rs should not contain `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "collapsible is not a dynamic item-collection; motion.rs should not contain `{forbidden}`.",
        );
        assert!(
            !docs.contains(forbidden),
            "collapsible docs should not imply registration protocol token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_slot_projection_policy_is_not_applicable_for_single_panel_disclosure() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let docs = load_source("src/collapsible/README.md");

    for forbidden in ["Lazy", "KeepAlive", "Eager", "NotifyHidden"] {
        assert!(
            !view.contains(forbidden),
            "collapsible has no slot-projection policy contract; view.rs should not contain `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "collapsible has no slot-projection lifecycle contract; logic.rs should not contain `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "collapsible has no slot-projection lifecycle contract; motion.rs should not contain `{forbidden}`.",
        );
        assert!(
            !docs.contains(forbidden),
            "collapsible docs should not imply slot projection token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_env_streams_are_not_applicable_without_responsive_sampling_contract() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let docs = load_source("src/collapsible/README.md");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "on:resize",
        "debounce",
        "throttle",
        "BreakpointChanged",
        "Action::BreakpointChanged",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible has no env-stream sampling contract; view.rs should not contain `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "collapsible has no env-stream action projection; logic.rs should not contain `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "collapsible motion should not embed env-stream token `{forbidden}`.",
        );
        assert!(
            !docs.contains(forbidden),
            "collapsible docs should not imply env-stream token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_event_light_cone_is_not_applicable_for_single_disclosure_state_axis() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let docs = load_source("src/collapsible/README.md");

    for forbidden in [
        "Context Bus + Selector",
        "ContextBus",
        "SelectionState::All",
        "SelectionState",
        "prop drilling",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible has no large-collection batch semantics; view.rs should not contain `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "collapsible has no large-collection state compression; logic.rs should not contain `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "collapsible motion should not embed event-light-cone token `{forbidden}`.",
        );
        assert!(
            !docs.contains(forbidden),
            "collapsible docs should not imply event-light-cone token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_causality_bus_is_not_applicable_without_derived_broadcast_graph() {
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let docs = load_source("src/collapsible/README.md");

    for forbidden in [
        "TraceId",
        "CausalityBus",
        "broadcast",
        "subscriber",
        "publish",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible has no derived causality bus graph; view.rs should not contain `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "collapsible has no derived causality bus graph; logic.rs should not contain `{forbidden}`.",
        );
        assert!(
            !motion.contains(forbidden),
            "collapsible motion should not embed causality-bus token `{forbidden}`.",
        );
        assert!(
            !docs.contains(forbidden),
            "collapsible docs should not imply causality-bus token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "pub(super) fn collapsible() -> AnyView",
        "title=\"Collapsible\"",
        "slug=\"collapsible\"",
        "title=\"Hello World\"",
        "State + Source Markers",
        "data-open-mode",
    ] {
        assert!(
            source.contains(needle),
            "collections_groups docs page should contain `{needle}` for Collapsible.",
        );
    }
}

#[test]
fn collapsible_docs_hello_world_uses_default_api_path() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let hello_code_section = source
        .split_once("let hello_code = Signal::derive(move || {")
        .and_then(|(_, tail)| tail.split_once("    let basic_code = Signal::derive(move || {"))
        .map(|(section, _)| section)
        .unwrap_or_else(|| {
            panic!("collapsible docs page should define hello_code before basic_code")
        });

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "id_base=\"docs-collapsible-hello\".to_string()",
        "title=\"Hello World\".to_string()",
        "<div>\"Panel content.\"</div>",
        "<Collapsible id_base=\"docs-collapsible-hello\".into() title=\"Hello World\".into()>",
    ] {
        assert!(
            source.contains(needle),
            "collapsible docs hello-world path should contain `{needle}`.",
        );
    }

    for forbidden in [
        "open=open.into()",
        "on_open_change=on_open_change",
        "#[prop(optional)] state:",
    ] {
        assert!(
            !hello_code_section.contains(forbidden),
            "hello-world code path should not require `{forbidden}`.",
        );
    }

    let hello_idx = source
        .find("<Playground title=\"Hello World\" code_signal=hello_code>")
        .unwrap_or_else(|| panic!("hello-world playground should exist"));
    let controlled_idx = source
        .find("<Playground title=\"Controlled Collapsible\" code_signal=basic_code>")
        .unwrap_or_else(|| panic!("controlled playground should exist"));
    assert!(
        hello_idx < controlled_idx,
        "hello-world playground should appear before controlled/advanced examples.",
    );
}

#[test]
fn collapsible_docs_disabled_custom_motion_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "title=\"Disabled + Custom Motion\"",
        "id_base=\"docs-collapsible-disabled\".to_string()",
        "is_disabled=true",
        "class_name=\"docs-collapsible-custom\".to_string()",
        "let custom_motion = CollapsibleMotion {",
        "panel_offset_y_px: 6.0",
        "motion=custom_motion",
    ] {
        assert!(
            source.contains(needle),
            "collapsible disabled/custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn collapsible_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id_base=\"docs-collapsible-markers\".to_string()",
        "aria_label=\"Advanced settings panel\".to_string()",
        "class_name=\"docs-collapsible-state\".to_string()",
        "let marker_motion = CollapsibleMotion {",
        "panel_offset_y_px: 8.0",
        "motion=marker_motion",
        "Open mode, label source, class source, and motion source are explicit.",
    ] {
        assert!(
            source.contains(needle),
            "collapsible state/source playground should contain `{needle}`."
        );
    }
}

#[test]
fn collapsible_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "pub(super) fn collapsible() -> AnyView",
        "title=\"Collapsible\"",
        "slug=\"collapsible\"",
        "title=\"Hello World\"",
        "title=\"Controlled Collapsible\"",
        "title=\"Disabled + Custom Motion\"",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            source.contains(needle),
            "collapsible docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn collapsible_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "id_base=\"docs-collapsible-hello\".to_string()",
        "<Playground title=\"Controlled Collapsible\" code_signal=basic_code>",
        "id_base=\"docs-collapsible\".to_string()",
        "open=open.into()",
        "on_open_change=on_open_change",
        "<Playground title=\"Disabled + Custom Motion\" code_signal=states_code>",
        "id_base=\"docs-collapsible-disabled\".to_string()",
        "is_disabled=true",
        "class_name=\"docs-collapsible-custom\".to_string()",
        "motion=custom_motion",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-collapsible-markers\".to_string()",
        "aria_label=\"Advanced settings panel\".to_string()",
        "class_name=\"docs-collapsible-state\".to_string()",
        "motion=marker_motion",
    ] {
        assert!(
            source.contains(needle),
            "collapsible docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn collapsible_e2e_contract_covers_semantic_matrix_without_snapshot_dependency() {
    let source = load_source("../../e2e/tests/docs_app_collapsible.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "data-open-mode\", \"controlled\"",
        "data-open-mode\", \"uncontrolled\"",
        "data-state\", \"disabled\"",
        "data-open-value-source\", \"external\"",
        "data-open-value-source\", \"primitive\"",
        "data-open-change-source\", \"initial\"",
        "controlledTrigger.click()",
        "page.keyboard.press(\"Enter\")",
        "aria-expanded",
    ] {
        assert!(
            source.contains(needle),
            "collapsible e2e matrix should include `{needle}` semantic coverage.",
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot("] {
        assert!(
            !source.contains(forbidden),
            "collapsible e2e contract should not depend on visual snapshot token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2 = load_source("../../components/collapsible/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should keep e2e selector/stable-wait rule `{required}`.",
        );
    }
}

#[test]
fn collapsible_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e = load_source("../../e2e/tests/docs_app_collapsible.spec.mjs");

    for marker in [
        "const COLLAPSIBLE_PAGE = '[data-component=\"collapsible\"]';",
        "const COLLAPSIBLE_ROOT = '[data-slot=\"collapsible\"]';",
        "const COLLAPSIBLE_TRIGGER = '[data-slot=\"collapsible-trigger\"]';",
        "const COLLAPSIBLE_PANEL = '[data-slot=\"collapsible-panel\"]';",
        "await waitForWasmReady(page);",
        "body:not(:has(#boot))",
        "data-open-mode=\"controlled\"",
        "data-state=\"disabled\"",
        "data-open-value-source",
        "data-open-change-source",
        "aria-expanded",
    ] {
        assert!(
            e2e.contains(marker),
            "collapsible e2e selector contract should include semantic marker `{marker}`.",
        );
    }

    for forbidden in [
        ".docs-page-title",
        "getByText(",
        "text=",
        "nth-child(",
        "waitForTimeout(",
        "setTimeout(",
    ] {
        assert!(
            !e2e.contains(forbidden),
            "collapsible e2e selector contract should avoid brittle/non-semantic marker `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_e2e_contract_covers_ready_and_settled_conditions_for_disclosure_paths() {
    let e2e = load_source("../../e2e/tests/docs_app_collapsible.spec.mjs");

    for marker in [
        "async function waitForWasmReady(page) {",
        "async function expectCollapsibleReady(root, state) {",
        "async function expectCollapsibleSettledOpen(root, trigger, panel) {",
        "async function expectCollapsibleSettledClosed(root, trigger, panel) {",
        "await expectCollapsibleSettledOpen(",
        "await expectCollapsibleSettledClosed(",
        "controlledTrigger.click()",
        "page.keyboard.press(\"Enter\")",
        "toBeVisible()",
        "toBeHidden()",
    ] {
        assert!(
            e2e.contains(marker),
            "collapsible e2e ready/settled contract should include `{marker}`.",
        );
    }
}

#[test]
fn collapsible_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script = load_source("../../scripts/check-ui-components-e2e-collapsible.sh");

    for marker in [
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_contract_covers_ready_and_settled_conditions_for_disclosure_paths",
    ] {
        assert!(
            script.contains(marker),
            "collapsible e2e check script should enforce `{marker}`.",
        );
    }
}

#[test]
fn collapsible_check2_marks_e2e_selector_stability_item_complete() {
    let check2 = load_source("../../components/collapsible/check2.md");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "components/collapsible/test/semantics.rs::collapsible_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/collapsible/test/semantics.rs::collapsible_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/collapsible/test/semantics.rs::collapsible_e2e_contract_covers_ready_and_settled_conditions_for_disclosure_paths",
        "components/collapsible/test/semantics.rs::collapsible_e2e_check_script_covers_selector_and_settled_wait_contract",
        "components/collapsible/test/collapsible_semantics.rs::collapsible_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "scripts/check-ui-components-e2e-collapsible.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(marker),
            "collapsible check2 e2e selector stability section should include `{marker}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_repeatable_keyflow_regression_rules() {
    let check2 = load_source("../../components/collapsible/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should keep repeatable keyflow regression rule `{required}`.",
        );
    }
}

#[test]
fn collapsible_e2e_regression_suite_contains_repeatable_disclosure_keyflow() {
    let e2e = load_source("../../e2e/tests/docs_app_collapsible.spec.mjs");

    for marker in [
        "await expectCollapsibleSettledOpen(",
        "controlledTrigger.click()",
        "await expectCollapsibleSettledClosed(",
        "await controlledTrigger.focus()",
        "page.keyboard.press(\"Enter\")",
        "await expectCollapsibleSettledOpen(",
    ] {
        assert!(
            e2e.contains(marker),
            "collapsible e2e repeatable keyflow should include `{marker}`.",
        );
    }
}

#[test]
fn collapsible_e2e_regression_failures_map_to_semantic_contract_breakpoints() {
    let e2e = load_source("../../e2e/tests/docs_app_collapsible.spec.mjs");

    for marker in [
        "async function expectCollapsibleReady(root, state) {",
        "toHaveAttribute(\"data-state\", state)",
        "toHaveAttribute(\"aria-expanded\", \"true\")",
        "toHaveAttribute(\"aria-expanded\", \"false\")",
        "toBeVisible()",
        "toBeHidden()",
        "data-open-change-source",
    ] {
        assert!(
            e2e.contains(marker),
            "collapsible e2e contract breakpoints should include `{marker}`.",
        );
    }

    assert!(
        !e2e.contains("page.screenshot("),
        "collapsible e2e repeatable regression should not degrade to screenshot-only diff.",
    );
}

#[test]
fn collapsible_e2e_regression_prioritizes_focus_and_keyboard_risk_paths() {
    let e2e = load_source("../../e2e/tests/docs_app_collapsible.spec.mjs");
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for marker in [
        "await controlledTrigger.focus()",
        "await expect(controlledTrigger).toBeFocused()",
        "await page.keyboard.press(\"Enter\")",
    ] {
        assert!(
            e2e.contains(marker),
            "collapsible high-risk path regression should include `{marker}`.",
        );
    }

    for unsupported in ["overlay", "popover", "async request", "retry"] {
        assert!(
            !docs.contains(unsupported),
            "collapsible docs should not claim unrelated high-risk path `{unsupported}`.",
        );
    }
}

#[test]
fn collapsible_e2e_check_script_covers_repeatable_keyflow_regression_contract() {
    let script = load_source("../../scripts/check-ui-components-e2e-collapsible.sh");

    for marker in [
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_repeatable_keyflow_regression_rules",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_suite_contains_repeatable_disclosure_keyflow",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_failures_map_to_semantic_contract_breakpoints",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_prioritizes_focus_and_keyboard_risk_paths",
    ] {
        assert!(
            script.contains(marker),
            "collapsible e2e check script should enforce repeatable keyflow marker `{marker}`.",
        );
    }
}

#[test]
fn collapsible_check2_marks_repeatable_keyflow_regression_item_complete() {
    let check2 = load_source("../../components/collapsible/check2.md");

    for marker in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "components/collapsible/test/semantics.rs::collapsible_check2_documents_repeatable_keyflow_regression_rules",
        "components/collapsible/test/semantics.rs::collapsible_e2e_regression_suite_contains_repeatable_disclosure_keyflow",
        "components/collapsible/test/semantics.rs::collapsible_e2e_regression_failures_map_to_semantic_contract_breakpoints",
        "components/collapsible/test/semantics.rs::collapsible_e2e_regression_prioritizes_focus_and_keyboard_risk_paths",
        "components/collapsible/test/semantics.rs::collapsible_e2e_check_script_covers_repeatable_keyflow_regression_contract",
        "components/collapsible/test/collapsible_semantics.rs::collapsible_e2e_regression_suite_contains_repeatable_disclosure_keyflow",
        "scripts/check-ui-components-e2e-collapsible.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(marker),
            "collapsible check2 repeatable keyflow regression section should include `{marker}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_semantics_first_testing_rules() {
    let check2 = load_source("../../components/collapsible/check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should keep semantics-first testing rule `{required}`.",
        );
    }
}

#[test]
fn collapsible_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = include_str!("../../../components/collapsible/test/semantics.rs");
    let logic_tests_source = include_str!("../../../components/collapsible/test/logic.rs");
    let module = load_source("src/collapsible/mod.rs");
    let test_mod = include_str!("../../../components/collapsible/test/mod.rs");

    for required in [
        "collapsible_view_mounts_headless_contract_and_semantic_markers",
        "collapsible_semantics_matrix_prefers_contract_assertions_over_snapshots",
        "collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            semantics_source.contains(required),
            "collapsible semantics suite should keep contract-first assertion `{required}`.",
        );
    }

    for required in [
        "normalize_discrete_axes_maps_boolean_inputs_to_enums",
        "open_state_switch_between_controlled_and_uncontrolled_is_stable",
        "normalize_open_state_options_prioritizes_open_then_default_then_primitive_fallback",
        "apply_open_change_uses_primitive_controlled_semantics",
    ] {
        assert!(
            logic_tests_source.contains(required),
            "collapsible logic regression should keep semantic matrix axis `{required}`.",
        );
    }

    assert!(
        module.contains("#[path = \"../test/mod.rs\"]"),
        "collapsible module should keep `*_semantics.rs` test entry point.",
    );
    assert!(
        test_mod.contains("mod semantics;"),
        "collapsible test module should include semantics test module.",
    );

    for forbidden in [
        "insta::",
        "assert_snapshot!",
        "assert_debug_snapshot!",
        ".to_match_snapshot(",
    ] {
        assert!(
            !semantics_source.contains(forbidden) && !logic_tests_source.contains(forbidden),
            "semantic suite should not rely on snapshot-only assertion `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view = load_source("src/collapsible/view.rs");
    let local_semantics = include_str!("../../../components/collapsible/test/semantics.rs");
    let aggregated_semantics = load_source("tests/collapsible_semantics.rs");

    for marker in [
        "role=aria.attrs.role",
        "aria-expanded=trigger_a11y.aria_expanded",
        "aria-controls=trigger_a11y.aria_controls",
        "aria-disabled=aria.attrs.aria_disabled",
        "data-state=move || state.get().state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-open-value-source=move || state.get().open_value_source_attr",
        "data-open-change-source=move || state.get().open_change_source_attr",
    ] {
        assert!(
            view.contains(marker),
            "collapsible view should keep semantic marker `{marker}`.",
        );
        assert!(
            local_semantics.contains(marker),
            "collapsible local semantics tests should cover semantic marker `{marker}` changes.",
        );
        assert!(
            aggregated_semantics.contains(marker),
            "collapsible aggregated semantics tests should cover semantic marker `{marker}` changes.",
        );
    }
}

#[test]
fn collapsible_semantics_first_testing_script_covers_contract() {
    let contract_hygiene_script =
        load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_first_testing_script_covers_contract",
    ] {
        assert!(
            contract_hygiene_script.contains(marker),
            "contract-hygiene script should include `{marker}`.",
        );
    }
}

#[test]
fn collapsible_machine_readable_state_contract_is_type_driven_and_marker_stable() {
    let primitive = load_source("../../crates/ui-state-primitives/src/collapsible.rs");
    let primitive_tests = load_source("../../crates/ui-state-primitives/src/test/collapsible.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let view = load_source("src/collapsible/view.rs");
    let logic_tests = load_source("../../components/collapsible/test/logic.rs");

    for needle in [
        "pub enum CollapsibleStatus",
        "pub enum CollapsibleOpenMode",
        "pub enum CollapsibleLabelSource",
        "pub enum CollapsibleClassSource",
        "pub enum CollapsibleMotionSource",
        "pub enum CollapsibleOpenValueSource",
        "pub enum CollapsibleOpenChangeSource",
        "pub struct CollapsibleStateInput",
    ] {
        assert!(
            primitive.contains(needle),
            "state primitive should model discrete axes with typed contract `{needle}`.",
        );
    }

    for needle in [
        "pub fn normalize_status(is_open: bool, is_disabled: bool) -> CollapsibleStatus",
        "pub fn normalize_open_mode(is_controlled: bool) -> CollapsibleOpenMode",
        "pub fn normalize_label_source(has_custom_aria_label: bool) -> CollapsibleLabelSource",
        "pub fn normalize_class_source(has_custom_class_name: bool) -> CollapsibleClassSource",
        "pub fn normalize_motion_source(has_custom_motion: bool) -> CollapsibleMotionSource",
        "pub fn normalize_open_value_source(",
        "pub fn normalize_open_change_source(is_interaction: bool) -> CollapsibleOpenChangeSource",
        "logic::resolve_state(CollapsibleStateInput {",
    ] {
        assert!(
            logic.contains(needle) || view.contains(needle),
            "logic/view should normalize into typed state contract `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-open-value-source=move || state.get().open_value_source_attr",
        "data-open-change-source=move || state.get().open_change_source_attr",
    ] {
        assert!(
            view.contains(needle),
            "view should expose machine-readable semantic marker `{needle}`.",
        );
    }

    for needle in [
        "discrete_state_enums_lock_mutually_exclusive_axes",
        "normalize_discrete_axes_maps_boolean_inputs_to_enums",
    ] {
        assert!(
            primitive_tests.contains(needle) || logic_tests.contains(needle),
            "tests should provide direct feedback for contract breakage via `{needle}`.",
        );
    }
}

#[test]
fn collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo = load_source("../../docs/plan/TODO.md");
    let script = load_source("../../scripts/check-ui-components-performance.sh");
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let styles = load_source("src/collapsible/styles.rs");
    let check2 = load_source("../../components/collapsible/check2.md");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "\"collapsible\" => UiPerfBudget {",
        "max_mount_ms: 34.0,",
        "max_update_ms: Some(11.0),",
        "max_heap_kb: Some(576.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell.contains(needle),
            "component shell should keep performance budget contract token `{needle}`.",
        );
    }

    assert!(
        pages.contains("component_doc!(\n        \"Collapsible\",\n        \"collapsible\","),
        "docs pages registry should keep collapsible in coverage traversal.",
    );

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "\"mount-only\"",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe.contains(needle),
            "UiPerfProbe should expose repeatable perf regression marker `{needle}`.",
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage.contains(needle),
            "docs coverage e2e should enforce blocking perf assertion `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo.contains(needle),
            "performance governance should keep explicit render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2.contains(needle),
            "checklist should keep perf budget/follow-up governance token `{needle}`.",
        );
    }

    assert!(
        script.contains(
            "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking",
        ),
        "performance gate script should include collapsible governance contract test.",
    );

    for needle in [
        "data-state=move || state.get().state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-open-value-source=move || state.get().open_value_source_attr",
        "data-open-change-source=move || state.get().open_change_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().motion_source.is_custom().then_some(\"true\")",
    ] {
        assert!(
            view.contains(needle),
            "view should expose perf attribution marker `{needle}` for root-cause triage.",
        );
    }
    for needle in [
        "pub fn normalize_status(",
        "pub fn normalize_open_mode(",
        "pub fn normalize_open_value_source(",
        "pub fn normalize_open_change_source(",
        "pub fn resolve_state(",
    ] {
        assert!(
            logic.contains(needle),
            "logic should keep attributable state normalization token `{needle}`.",
        );
    }
    for needle in [
        "attach_indicator_motion",
        "attach_panel_motion",
        "sanitize_motion",
    ] {
        assert!(
            motion.contains(needle),
            "motion should keep attributable animation path token `{needle}`.",
        );
    }
    for needle in [
        ".ui-collapsible[data-state=\"open\"]",
        ".ui-collapsible[data-state=\"disabled\"]",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            styles.contains(needle),
            "styles should keep attributable visual path token `{needle}`.",
        );
    }

    let view_memo_count = view.matches("Memo::new(").count();
    assert!(
        view_memo_count <= 2,
        "collapsible view reactive budget exceeded: expected <= 2 `Memo::new`, found {view_memo_count}.",
    );
    let view_signal_derive_count = view.matches("Signal::derive(").count();
    assert!(
        view_signal_derive_count <= 1,
        "collapsible view reactive budget exceeded: expected <= 1 `Signal::derive`, found {view_signal_derive_count}.",
    );
    let view_effect_count = view.matches("Effect::new(").count();
    assert!(
        view_effect_count <= 2,
        "collapsible view reactive budget exceeded: expected <= 2 `Effect::new`, found {view_effect_count}.",
    );

    let motion_effect_count = motion.matches("Effect::new(").count();
    assert_eq!(
        motion_effect_count, 0,
        "collapsible motion adapter should avoid local effect loops; found {motion_effect_count}.",
    );
}

#[test]
fn collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let local_semantics = include_str!("../../../components/collapsible/test/semantics.rs");
    let aggregated_semantics = load_source("tests/collapsible_semantics.rs");
    let view = load_source("src/collapsible/view.rs");
    let e2e = load_source("../../e2e/tests/docs_app_collapsible.spec.mjs");
    let todo = load_source("../../docs/plan/TODO.md");
    let script = load_source("../../scripts/check-ui-components-performance.sh");
    let check2 = load_source("../../components/collapsible/check2.md");

    for required_test in [
        "fn collapsible_view_mounts_headless_contract_and_semantic_markers()",
        "fn collapsible_semantics_matrix_prefers_contract_assertions_over_snapshots()",
        "fn collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && aggregated_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests."
        );
    }

    for marker in [
        "role=aria.attrs.role",
        "aria-expanded=trigger_a11y.aria_expanded",
        "aria-controls=trigger_a11y.aria_controls",
        "aria-disabled=aria.attrs.aria_disabled",
        "data-state=move || state.get().state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-open-value-source=move || state.get().open_value_source_attr",
        "data-open-change-source=move || state.get().open_change_source_attr",
        "use_focus_ring(FocusRingOptions { is_disabled })",
        "class:ui-disclosure__trigger--focus-visible=move || focus_ring.is_focus_visible.get()",
        "on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())",
        "on:keydown=move |ev| {",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
    ] {
        assert!(
            view.contains(marker),
            "collapsible view should expose aria/data/focus semantic marker `{marker}`."
        );
    }

    for marker in [
        "data-open-mode\", \"controlled\"",
        "data-open-mode\", \"uncontrolled\"",
        "data-state\", \"disabled\"",
        "controlledTrigger.click()",
        "page.keyboard.press(\"Enter\")",
        "aria-expanded",
    ] {
        assert!(
            e2e.contains(marker),
            "collapsible e2e semantic matrix should include marker `{marker}`.",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo.contains(marker),
            "render_count follow-up governance should include `{marker}`.",
        );
    }

    for marker in [
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            script.contains(marker),
            "performance gate script should include `{marker}`.",
        );
    }

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking",
    ] {
        assert!(
            check2.contains(marker),
            "checklist should keep semantics/performance governance marker `{marker}`.",
        );
    }
}

#[test]
fn collapsible_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view = load_source("src/collapsible/view.rs");
    let script = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2 = load_source("../../components/collapsible/check2.md");

    for needle in [
        "fn render_trigger(",
        "fn render_panel(",
        "let trigger = render_trigger(",
        "let panel = render_panel(",
        "data-slot=SLOT_COLLAPSIBLE_TRIGGER",
        "data-slot=SLOT_COLLAPSIBLE_PANEL",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should split macro-heavy layout via semantic sub-block token `{needle}`.",
        );
    }

    let view_macro_count = view.matches("view! {").count();
    assert!(
        view_macro_count <= 3,
        "collapsible view macro complexity regression: expected <= 3 `view!` blocks, found {view_macro_count}.",
    );

    let component_macro_count = view.matches("#[component]").count();
    assert_eq!(
        component_macro_count, 1,
        "collapsible should keep exactly one public component entry; found {component_macro_count}.",
    );

    assert!(
        script.contains(
            "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_view_macro_complexity_is_split_into_semantic_subrenders",
        ),
        "view-macro gate script should include collapsible macro complexity test target.",
    );

    for needle in [
        "`view!` 宏复杂度受控",
        "复杂结构按语义子块拆分",
        "避免巨型单块 `view!`",
    ] {
        assert!(
            check2.contains(needle),
            "checklist should keep macro complexity governance token `{needle}`.",
        );
    }
}

#[test]
fn collapsible_view_functional_split_prefers_plain_functions_over_local_components() {
    let view = load_source("src/collapsible/view.rs");
    let script = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2 = load_source("../../components/collapsible/check2.md");

    for needle in [
        "fn render_trigger(",
        "fn render_panel(",
        ") -> impl IntoView {",
        "pub fn Collapsible(",
    ] {
        assert!(
            view.contains(needle),
            "collapsible function-first split should include `{needle}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn render_trigger(",
        "#[component]\nfn render_panel(",
        "let trigger_view = move || {",
        "let panel_view = move || {",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible helper fragment should remain plain function and avoid `{forbidden}`.",
        );
    }

    assert_eq!(
        view.matches("#[component]").count(),
        1,
        "collapsible should keep exactly one public component boundary.",
    );

    for needle in [
        "const SLOT_COLLAPSIBLE: &str = \"collapsible\";",
        "const SLOT_COLLAPSIBLE_TRIGGER: &str = \"collapsible-trigger\";",
        "const SLOT_COLLAPSIBLE_LABEL: &str = \"collapsible-label\";",
        "const SLOT_COLLAPSIBLE_INDICATOR: &str = \"collapsible-indicator\";",
        "const SLOT_COLLAPSIBLE_PANEL: &str = \"collapsible-panel\";",
        "const SLOT_COLLAPSIBLE_PANEL_SURFACE: &str = \"collapsible-panel-surface\";",
        "data-slot=SLOT_COLLAPSIBLE",
        "data-slot=SLOT_COLLAPSIBLE_TRIGGER",
        "data-slot=SLOT_COLLAPSIBLE_LABEL",
        "data-slot=SLOT_COLLAPSIBLE_INDICATOR",
        "data-slot=SLOT_COLLAPSIBLE_PANEL",
        "data-slot=SLOT_COLLAPSIBLE_PANEL_SURFACE",
    ] {
        assert!(
            view.contains(needle),
            "collapsible semantic marker should stay stable after function split `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"),
        "checklist should mark function-first split contract as completed.",
    );
}

#[test]
fn collapsible_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view = load_source("src/collapsible/view.rs");
    let script = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2 = load_source("../../components/collapsible/check2.md");

    for needle in [
        "const SLOT_COLLAPSIBLE: &str = \"collapsible\";",
        "const SLOT_COLLAPSIBLE_TRIGGER: &str = \"collapsible-trigger\";",
        "const SLOT_COLLAPSIBLE_LABEL: &str = \"collapsible-label\";",
        "const SLOT_COLLAPSIBLE_INDICATOR: &str = \"collapsible-indicator\";",
        "const SLOT_COLLAPSIBLE_PANEL: &str = \"collapsible-panel\";",
        "const SLOT_COLLAPSIBLE_PANEL_SURFACE: &str = \"collapsible-panel-surface\";",
        "const ARIA_HIDDEN_TRUE: &str = \"true\";",
        "const COLLAPSIBLE_INDICATOR_GLYPH: &str = \"›\";",
        "data-slot=SLOT_COLLAPSIBLE",
        "data-slot=SLOT_COLLAPSIBLE_TRIGGER",
        "data-slot=SLOT_COLLAPSIBLE_LABEL",
        "data-slot=SLOT_COLLAPSIBLE_INDICATOR",
        "data-slot=SLOT_COLLAPSIBLE_PANEL",
        "data-slot=SLOT_COLLAPSIBLE_PANEL_SURFACE",
        "aria-hidden=ARIA_HIDDEN_TRUE",
        "{COLLAPSIBLE_INDICATOR_GLYPH}",
    ] {
        assert!(
            view.contains(needle),
            "collapsible static fragment contract should keep constantized token `{needle}`.",
        );
    }

    for forbidden in [
        "<svg",
        "<path",
        "inner_html=",
        "dangerously_set_inner_html",
        "markdown_to_html(",
    ] {
        assert!(
            !view.contains(forbidden),
            "collapsible simple layout should avoid heavy static fragment token `{forbidden}`.",
        );
    }

    assert_eq!(
        view.matches("COLLAPSIBLE_INDICATOR_GLYPH").count(),
        2,
        "collapsible indicator glyph should keep one constant source and one render usage.",
    );

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"),
        "checklist should mark static fragment contract as completed.",
    );
}

#[test]
fn collapsible_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let script = load_source("../../scripts/check-ui-components-inner-html.sh");
    let check2 = load_source("../../components/collapsible/check2.md");
    let docs_page =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let docs_shell = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    for rel_path in [
        "src/collapsible/mod.rs",
        "src/collapsible/logic.rs",
        "src/collapsible/styles.rs",
        "src/collapsible/view.rs",
        "src/collapsible/motion.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html=",
            "set_inner_html(",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "collapsible source `{rel_path}` should forbid raw-html injection token `{forbidden}`.",
            );
        }
    }

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
    ] {
        assert!(
            !docs_page.contains(forbidden),
            "collapsible docs page should avoid raw-html injection token `{forbidden}`.",
        );
    }

    assert!(
        docs_shell.contains("<div data-slot=\"component-readme\" inner_html=html></div>"),
        "docs shell should keep the single trusted markdown inner_html mount.",
    );
    assert!(
        !docs_shell.contains("\"collapsible\" => Some("),
        "collapsible should stay out of docs-shell inner_html whitelist.",
    );

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script.contains(script_needle),
        "inner-html check script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"),
        "checklist should mark inner_html contract as completed.",
    );
}

#[test]
fn collapsible_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na()
{
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let check2_source = load_source("../../components/collapsible/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Restore original CSS\"",
        "data-playground-scope=scope_id.clone()",
        "<div class=\"playground__preview-stage\">{children()}</div>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep DX hot-style/isolated-canvas marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn collapsible() -> AnyView",
        "title=\"Interactive Playground (Display + Config + Code + CSS Test)\"",
        "test_css_source=test_css_source",
        "test_config_signal=actual_config",
        "controls=move || view!",
        "id_base=\"docs-collapsible-interactive\".to_string()",
        "\"Use Mode switch to compare controlled vs uncontrolled state source.\"",
        "slug=\"collapsible\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "collapsible docs should keep interactive workbench/context marker `{needle}`.",
        );
    }

    for forbidden in [
        "COLLAPSIBLE_WORKBENCH_STORAGE_KEY",
        "load_collapsible_workbench_state(",
        "save_collapsible_workbench_state(",
        "clear_collapsible_workbench_state(",
        "Persist collapsible workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "collapsible keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent.",
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "collapsible_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible checklist should keep DX governance marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        script_source.contains(script_needle),
        "DX gate script should include `{script_needle}`.",
    );
}

#[test]
fn collapsible_engineering_contract_uses_serde_protocol_and_structured_schema_defaults() {
    let component_cargo = load_source("../../components/collapsible/Cargo.toml");
    let protocol = load_source("../../components/collapsible/src/protocol.rs");
    let mod_source = load_source("src/collapsible/mod.rs");
    let logic_source = load_source("src/collapsible/logic.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let styles_source = load_source("src/collapsible/styles.rs");
    let motion_source = load_source("src/collapsible/motion.rs");
    let check2_source = load_source("../../components/collapsible/check2.md");

    assert!(
        component_cargo.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "collapsible protocol schema should use serde derive dependency.",
    );
    assert!(
        !component_cargo.contains("serde_json"),
        "collapsible should avoid serde_json fan-out without explicit protocol migration need.",
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum CollapsibleComponentSchemaVersion {",
        "V1,",
        "pub struct CollapsibleComponentSpec {",
        "pub schema_version: CollapsibleComponentSchemaVersion,",
        "#[serde(default)]",
    ] {
        assert!(
            protocol.contains(needle),
            "collapsible protocol should keep structured serde schema marker `{needle}`.",
        );
    }

    let combined_non_protocol =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde_json::",
        "SchemaError",
        "from_json(",
        "to_json_result(",
    ] {
        assert!(
            !combined_non_protocol.contains(forbidden),
            "non-protocol collapsible layers should avoid schema/migration token `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible checklist should keep engineering governance marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
{
    let ui_components_cargo = load_source("Cargo.toml");
    let button_view = load_source("src/button/view.rs");
    let combined = [
        load_source("src/collapsible/mod.rs"),
        load_source("src/collapsible/logic.rs"),
        load_source("src/collapsible/view.rs"),
        load_source("src/collapsible/styles.rs"),
        load_source("src/collapsible/motion.rs"),
        load_source("../../components/collapsible/src/protocol.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            ui_components_cargo.contains(required) || button_view.contains(required),
            "engineering baseline should keep canonical tracing marker `{required}`.",
        );
    }

    assert!(
        !ui_components_cargo.contains("collapsible-wasm-debug"),
        "collapsible should not define a component-local tracing/debug feature without dedicated replay contract.",
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::collapsible::",
        "const COLLAPSIBLE_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "collapsible should avoid ad-hoc tracing semantic drift token `{forbidden}`.",
        );
    }
}

#[test]
fn collapsible_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let component_cargo = load_source("../../components/collapsible/Cargo.toml");
    let mod_source = load_source("src/collapsible/mod.rs");
    let logic_source = load_source("src/collapsible/logic.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let styles_source = load_source("src/collapsible/styles.rs");
    let motion_source = load_source("src/collapsible/motion.rs");
    let protocol_source = load_source("../../components/collapsible/src/protocol.rs");

    for source in [
        &component_cargo,
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
        &protocol_source,
    ] {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "collapsible should not leak runtime marker `{forbidden}`.",
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "collapsible public module boundary should not leak web_sys types.",
    );
}

#[test]
fn collapsible_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    let section_start = docs_source
        .find("pub(super) fn collapsible() -> AnyView {")
        .unwrap_or_else(|| panic!("collections_groups docs should contain collapsible section"));
    let section = &docs_source[section_start..];

    for needle in [
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"collapsible-streaming-policy\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "code_imports=collapsible_imports.clone()",
        "component-collapsible",
        "inject-css",
    ] {
        assert!(
            section.contains(needle),
            "collapsible docs product contract should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "missing_import_lines(&raw, &imports)",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }
}

#[test]
fn collapsible_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source = load_source("src/code_block/view.rs");

    for needle in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"collapsible-source-first-contract\"",
        "data-slot=\"collapsible-copy-ready-hint\"",
        "data-slot=\"collapsible-source-paths\"",
        "components/collapsible/src/mod.rs",
        "components/collapsible/src/logic.rs",
        "components/collapsible/src/view.rs",
        "components/collapsible/src/styles.rs",
        "components/collapsible/src/motion.rs",
        "features: component-collapsible + inject-css",
    ] {
        assert!(
            docs_source.contains(needle),
            "collapsible source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy-ready pipeline should keep `{needle}`.",
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep one-click copy affordance marker `{needle}`.",
        );
    }
}

#[test]
fn collapsible_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_docs_product_copy_paste_ready_rules",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_docs_product_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce collapsible docs-product marker `{needle}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_docs_product_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Hello World / State Matrix / Controlled vs Uncontrolled Contrast / Streaming / Snapshot Contract / Source-first Starter (Copy-Paste Ready)",
        "compose_copy_ready_code",
        "component-collapsible",
        "collapsible_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "collapsible_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "collapsible_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "collapsible check2 docs-product section should contain `{needle}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible check2 should keep source-first copy-paste rule `{required}`.",
        );
    }
}

#[test]
fn collapsible_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source = load_source("src/code_block/view.rs");

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "description=\"Copy-ready starter with import completion, source path hints, and minimal feature flags.\"",
        "code_signal=source_first_code",
        "code_imports=collapsible_imports.clone()",
        "test_source_path=\"components/collapsible/src/view.rs\".to_string()",
        "data-slot=\"collapsible-source-first-contract\"",
        "data-slot=\"collapsible-copy-ready-hint\"",
        "data-slot=\"collapsible-source-paths\"",
        "components/collapsible/src/mod.rs",
        "components/collapsible/src/logic.rs",
        "components/collapsible/src/view.rs",
        "components/collapsible/src/styles.rs",
        "components/collapsible/src/motion.rs",
        "features: component-collapsible + inject-css",
        "let source_first_code = Signal::derive(move || {",
        "\"  id_base=\\\"docs-collapsible-source-first\\\".into()\".to_string()",
        "\"  title=\\\"Source-first starter\\\".into()\".to_string()",
        "\"  default_open=true\".to_string()",
        "\"  motion=CollapsibleMotion {\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "collapsible source-first docs contract should contain `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "missing_import_lines(&raw, &imports)",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(required),
            "playground copy-ready pipeline should contain `{required}`.",
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(required),
            "CodeBlock should keep one-click copy markers `{required}`.",
        );
    }
}

#[test]
fn collapsible_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: collapsible source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_source_first_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include source-first marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "collapsible check2 should mark source-first copy-paste-ready item complete."
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"collapsible-source-first-contract\"",
        "components/collapsible/src/mod.rs",
        "components/collapsible/src/logic.rs",
        "components/collapsible/src/view.rs",
        "components/collapsible/src/styles.rs",
        "components/collapsible/src/motion.rs",
        "component-collapsible + inject-css",
        "collapsible_check2_documents_source_first_copy_paste_ready_rules",
        "collapsible_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "collapsible_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible check2 source-first section should reference `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible check2 heroui docs-sync section should include `{required}`.",
        );
    }
}

#[test]
fn collapsible_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let readme_source = load_source("../../components/collapsible/src/README.md");

    for required in [
        "### Collapsible 同步记录（2026-02-20）",
        "`open/default_open/on_open_change`",
        "`is_disabled.unwrap_or(disabled)`",
        "component_doc!(\"Collapsible\", \"collapsible\", \"Collections\", collections_groups::collapsible)",
        "apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible()",
        "docs/research/spectrum-heroui-style-interface-study.md`。",
        "仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            strategy_source.contains(required),
            "HeroUI strategy doc should include collapsible sync marker `{required}`.",
        );
    }

    assert!(
        docs_registry_source.contains(
            "component_doc!(\"Collapsible\", \"collapsible\", \"Collections\", collections_groups::collapsible)",
        ),
        "docs component registry should expose collapsible entrypoint.",
    );

    for required in [
        "pub(super) fn collapsible() -> AnyView {",
        "title=\"Collapsible\"",
        "slug=\"collapsible\"",
        "title=\"Parameter Matrix\"",
        "title=\"State Matrix\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "title=\"Interactive Playground (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            docs_page_source.contains(required),
            "collapsible docs page should keep synced marker `{required}`.",
        );
    }

    for required in [
        "# Collapsible",
        "## 文档入口",
        "`/#/components/collapsible`",
    ] {
        assert!(
            readme_source.contains(required),
            "collapsible README should keep docs entry marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: collapsible heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include heroui docs-sync marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    assert!(
        check2_source.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "collapsible check2 should mark heroui docs-sync item complete."
    );

    for required in [
        "docs/spec/heroui-parameter-design-strategy.md",
        "### Collapsible 同步记录（2026-02-20）",
        "component_doc!(\"Collapsible\", \"collapsible\", \"Collections\", collections_groups::collapsible)",
        "apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible",
        "components/collapsible/src/README.md",
        "collapsible_check2_documents_heroui_benchmark_docs_sync_rules",
        "collapsible_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "collapsible_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible check2 heroui docs-sync section should retain marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible check2 should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn collapsible_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let logic_source = load_source("src/collapsible/logic.rs");

    for required in [
        "pub(super) fn collapsible() -> AnyView {",
        "title=\"Hello World\"",
        "title=\"Parameter Matrix\"",
        "data-slot=\"collapsible-parameter-matrix\"",
        "open + on_open_change + default_open",
        "is_disabled.unwrap_or(disabled)",
        "title=\"State Matrix\"",
        "id_base=\"docs-collapsible-matrix-default\".to_string()",
        "id_base=\"docs-collapsible-matrix-disabled\".to_string()",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "id_base=\"docs-collapsible-contrast-uncontrolled\".to_string()",
        "id_base=\"docs-collapsible-contrast-controlled\".to_string()",
        "default_open=true",
        "default_open=false",
        "is_disabled=true",
    ] {
        assert!(
            docs_source.contains(required),
            "collapsible docs should keep API/default/state-matrix marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] open: Option<Signal<bool>>,",
        "#[prop(optional)] default_open: Option<bool>,",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>,",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] motion: CollapsibleMotion,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional, into)] dir: Option<String>,",
    ] {
        assert!(
            view_source.contains(required),
            "collapsible public API contract should keep `{required}` for docs sync.",
        );
    }

    for required in [
        "open (controlled) > default_open (uncontrolled seed) > primitive fallback.",
        "CollapsibleOpenStateOptions { open, default_open }",
        "pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {",
        "is_disabled.unwrap_or(disabled)",
    ] {
        assert!(
            logic_source.contains(required),
            "collapsible logic normalization should keep `{required}` for docs default/source sync.",
        );
    }
}

#[test]
fn collapsible_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: collapsible docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include docs-sync/state-matrix marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "collapsible check2 should mark docs-sync/state-matrix checklist item complete."
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible",
        "title=\"Parameter Matrix\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "open/on_open_change/default_open/is_disabled/disabled/motion/aria_label/class_name/lang/dir",
        "components/collapsible/src/view.rs",
        "components/collapsible/src/logic.rs",
        "components/collapsible/test/semantics.rs::collapsible_check2_documents_docs_sync_and_state_matrix_rules",
        "components/collapsible/test/semantics.rs::collapsible_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "components/collapsible/test/semantics.rs::collapsible_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible check2 docs-sync/state-matrix section should reference `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn collapsible_documentation_entry_exists_with_beginner_first_progression() {
    let docs_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let readme_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/collapsible/src/README.md");
    let readme_source = load_source("../../components/collapsible/src/README.md");

    assert!(
        readme_path.exists()
            || (docs_registry_source.contains("component_doc!(")
                && docs_registry_source.contains("\"Collapsible\"")
                && docs_registry_source.contains("\"collapsible\"")
                && docs_registry_source.contains("collections_groups::collapsible")),
        "collapsible should provide README or an equivalent docs-app entrypoint.",
    );

    for required in [
        "# Collapsible",
        "## Hello World（最小可用）",
        "## API (Table)",
        "## 文档入口",
        "`/#/components/collapsible`",
    ] {
        assert!(
            readme_source.contains(required),
            "collapsible README should keep beginner-first marker `{required}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn collapsible() -> AnyView {")
        .unwrap_or_else(|| panic!("collections_groups docs should contain collapsible section"));
    let section = &docs_source[section_start..];

    for required in [
        "title=\"Collapsible\"",
        "slug=\"collapsible\"",
        "Start with Hello World, then move to controlled/state matrix examples",
        "title=\"Hello World\"",
        "title=\"Controlled Collapsible\"",
        "title=\"Disabled + Custom Motion\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"State + Source Markers\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            section.contains(required),
            "collapsible docs should keep beginner-first documentation marker `{required}`.",
        );
    }

    let hello_pos = section
        .find("title=\"Hello World\"")
        .expect("collapsible docs should include Hello World playground.");
    let controlled_pos = section
        .find("title=\"Controlled Collapsible\"")
        .expect("collapsible docs should include controlled playground.");
    let matrix_pos = section
        .find("title=\"State Matrix\"")
        .expect("collapsible docs should include state matrix playground.");
    let contrast_pos = section
        .find("title=\"Controlled vs Uncontrolled Contrast\"")
        .expect("collapsible docs should include controlled/uncontrolled contrast.");
    let markers_pos = section
        .find("title=\"State + Source Markers\"")
        .expect("collapsible docs should include marker playground.");
    let source_first_pos = section
        .find("title=\"Source-first Starter (Copy-Paste Ready)\"")
        .expect("collapsible docs should include source-first playground.");

    assert!(
        hello_pos < controlled_pos
            && controlled_pos < matrix_pos
            && matrix_pos < contrast_pos
            && contrast_pos < markers_pos
            && markers_pos < source_first_pos,
        "collapsible docs should present default path before advanced path.",
    );
}

#[test]
fn collapsible_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: collapsible documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include documentation-as-product marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_marks_documentation_as_product_item_complete() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "collapsible check2 should mark documentation-as-product checklist item complete."
    );

    for required in [
        "components/collapsible/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible",
        "Start with Hello World, then move to controlled/state matrix examples",
        "Hello World -> Controlled Collapsible -> State Matrix -> Controlled vs Uncontrolled Contrast -> State + Source Markers -> Source-first Starter",
        "components/collapsible/test/semantics.rs::collapsible_check2_documents_documentation_as_product_rules",
        "components/collapsible/test/semantics.rs::collapsible_documentation_entry_exists_with_beginner_first_progression",
        "components/collapsible/test/semantics.rs::collapsible_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible check2 documentation-as-product section should reference `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible check2 interactive-playground section should include `{required}`.",
        );
    }
}

#[test]
fn collapsible_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for required in [
        "title=\"Interactive Playground (Display + Config + Code + CSS Test)\"",
        "controls=move || view! {",
        "data-slot=\"collapsible-workbench-controls\"",
        "data-slot=\"collapsible-workbench-preview\"",
        "data-slot=\"collapsible-workbench-controlled-state\"",
        "data-slot=\"collapsible-workbench-default-state\"",
        "id_base=\"docs-collapsible-interactive-mode\".to_string()",
        "id_base=\"docs-collapsible-interactive-motion\".to_string()",
        "Switch checked=controlled_open set_checked=set_controlled_open",
        "Switch checked=default_open_preview set_checked=set_default_open_preview",
        "Switch checked=disabled_preview set_checked=set_disabled_preview",
        "Switch checked=custom_label set_checked=set_custom_label",
        "Switch checked=custom_class set_checked=set_custom_class",
        "test_config_signal=actual_config",
        "mode: \"{}\"",
        "motion_source: \"{}\"",
    ] {
        assert!(
            docs_source.contains(required),
            "collapsible docs interactive playground should include `{required}`.",
        );
    }
}

#[test]
fn collapsible_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_collapsible.spec.mjs");

    for required in [
        "docs-app: collapsible interactive playground updates props/state and replays deterministically",
        "Interactive Playground (Display + Config + Code + CSS Test)",
        "[data-slot=\"playground-toggle-settings\"]",
        "[data-slot=\"collapsible-workbench-controls\"]",
        "[data-slot=\"segmented-control-option\"]",
        "[data-slot=\"switch\"]",
        "[data-slot=\"collapsible-workbench-preview\"]",
        "[data-slot=\"collapsible-workbench-controlled-state\"]",
        "[data-slot=\"playground-toggle-code\"]",
        "[data-slot=\"playground-code\"] [data-slot=\"code-block-code\"]",
        "await controlledMode.click();",
        "await controlledOpenSwitch.focus();",
        "await page.keyboard.press(\"Space\");",
        "await expect(codeBlock).toContainText(\"is_disabled=true\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "collapsible interactive e2e flow should include `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "toHaveScreenshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "collapsible interactive e2e flow should avoid flaky token `{forbidden}`.",
        );
    }

    let replay_count = e2e_source
        .matches("await page.goto(\"/#/components/collapsible\");")
        .count();
    assert!(
        replay_count >= 2,
        "collapsible interactive acceptance should be repeatable; expected >=2 flows, got {replay_count}."
    );

    for required in [
        "data-slot=\"collapsible-workbench-controls\"",
        "data-slot=\"collapsible-workbench-preview\"",
        "data-slot=\"collapsible-workbench-controlled-state\"",
    ] {
        assert!(
            docs_source.contains(required),
            "collapsible docs should expose stable interactive anchor `{required}`.",
        );
    }
}

#[test]
fn collapsible_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: collapsible interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include interactive-playground marker `{required}`.",
        );
    }
}

#[test]
fn collapsible_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_source("../../components/collapsible/check2.md");

    assert!(
        check2_source.contains(
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"
        ),
        "collapsible check2 should mark interactive-playground checklist item complete."
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible",
        "title=\"Interactive Playground (Display + Config + Code + CSS Test)\"",
        "data-slot=\"collapsible-workbench-controls\"",
        "data-slot=\"collapsible-workbench-preview\"",
        "AI Spec 相关联动示例：N/A（`collapsible` 组件无 `spec.rs` 与 Spec 输入协议）",
        "e2e/tests/docs_app_collapsible.spec.mjs::docs-app: collapsible interactive playground updates props/state and replays deterministically",
        "collapsible_check2_documents_interactive_playground_rules",
        "collapsible_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "collapsible_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "collapsible_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "collapsible check2 interactive-playground section should reference `{required}`.",
        );
    }
}

#[test]
fn collapsible_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("../../components/collapsible/src/Component.toml");
    let rbi_source = load_source("../../components/collapsible/src/collapsible.rbi");
    let mod_source = load_source("src/collapsible/mod.rs");
    let logic_source = load_source("src/collapsible/logic.rs");
    let view_source = load_source("src/collapsible/view.rs");
    let styles_source = load_source("src/collapsible/styles.rs");
    let motion_source = load_source("src/collapsible/motion.rs");
    let protocol_source = load_source("../../components/collapsible/src/protocol.rs");
    let check2_source = load_source("../../components/collapsible/check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Collapsible\"",
        "crate = \"ui-collapsible\"",
        "schema = \"ui.collapsible.agent-contract.v1\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "collapsible manifest should keep stable v1 schema marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn Collapsible(",
        "open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "collapsible RBI should keep stable public API marker `{needle}`.",
        );
    }

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{protocol_source}"
    );
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "collapsible should not introduce major-version migration marker `{forbidden}` in current scope.",
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `Collapsible` 未发生跨大版本 API 破坏升级）",
        "schema_version = \"1\"",
        "ui.collapsible.agent-contract.v1",
        "collapsible_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "collapsible/check2.md should keep version-migration governance marker `{needle}`.",
        );
    }
}

#[test]
fn collapsible_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    let marker = "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`.",
    );
}

#[test]
fn collapsible_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let check2 = load_source("../../components/collapsible/check2.md");
    let script = load_source("../../scripts/check-ui-components-wasm-debug.sh");
    let component_cargo = load_source("../../components/collapsible/Cargo.toml");
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let button_view = load_source("src/button/view.rs");
    let docs_app = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace = load_source("../ui-headless/src/trace.rs");
    let view = load_source("src/collapsible/view.rs");
    let logic = load_source("src/collapsible/logic.rs");
    let motion = load_source("src/collapsible/motion.rs");
    let readme = load_source("src/collapsible/README.md");

    for needle in ["[features]", "default = []"] {
        assert!(
            component_cargo.contains(needle),
            "collapsible crate feature boundary should include `{needle}`.",
        );
    }

    for forbidden in [
        "wasm-debug",
        "collapsible-wasm-debug",
        "collapsible_wasm_debug",
        "component-collapsible-wasm-debug",
    ] {
        assert!(
            !component_cargo.contains(forbidden),
            "collapsible crate should not expose component-local wasm debug feature `{forbidden}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components should keep shared wasm-debug feature marker `{needle}`.",
        );
    }

    let all_components_start = ui_components_cargo
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = ui_components_cargo[all_components_start..]
        .find("\n\ndev-all-components")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before dev-all-components");
    let all_components_block = &ui_components_cargo[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path.",
    );

    for forbidden in [
        "collapsible-wasm-debug =",
        "collapsible_wasm_debug =",
        "component-collapsible-wasm-debug",
        "component-collapsible\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui-components feature graph should not leak collapsible-specific debug toggle `{forbidden}`.",
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components root should keep shared wasm-debug isolation marker `{needle}`.",
        );
    }

    for needle in [
        "data-debug-source=source.clone()",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "data-debug-timestamp-ms=format!(\"{:.0}\", event.timestamp_ms)",
        "data-slot=\"button-debug-replay\"",
        "request_replay.run(event.source)",
    ] {
        assert!(
            button_view.contains(needle),
            "shared button wasm debug path should keep trace/replay marker `{needle}`.",
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app.contains(needle),
            "docs app should keep dev-only wasm debug visual entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "events.push(event);",
        "UiTraceEventKind::OpenChange",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            trace.contains(needle) || debug_overlay.contains(needle),
            "global trace/debug overlay should keep marker `{needle}`.",
        );
    }

    for needle in [
        "let trace = use_ui_trace();",
        "trace.emit(\"collapsible\", UiTraceEventKind::OpenChange { open: next });",
        "data-state=move || state.get().state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-open-value-source=move || state.get().open_value_source_attr",
        "data-open-change-source=move || state.get().open_change_source_attr",
        "on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())",
        "on:keydown=move |ev| {",
        "on:keyup=move |ev| {",
    ] {
        assert!(
            view.contains(needle),
            "collapsible should keep machine-readable state/source/interaction marker `{needle}` for debug attribution.",
        );
    }

    for forbidden in [
        "collapsible-wasm-debug",
        "wasm_debug",
        "render_debug_panel(",
        "data-debug-source",
        "request_replay.run(",
        "#[prop(optional)] debug",
        "debug_overlay",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !motion.contains(forbidden)
                && !readme.contains(forbidden),
            "collapsible runtime/public contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    assert!(
        script.contains(
            "cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
        ),
        "wasm-debug gate script should include collapsible wasm-debug contract target.",
    );

    for needle in [
        "- [x] WASM 调试要求",
        "collapsible_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should keep wasm-debug governance marker `{needle}`.",
        );
    }
}
