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

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_error_view_component_source(rel_path: &str) -> String {
    let path = workspace_dir().join("components/error-view").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_workspace_source(rel_path: &str) -> String {
    let path = workspace_dir().join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn component_path_exists(rel_path: &str) -> bool {
    workspace_dir()
        .join("components/error-view")
        .join(rel_path)
        .exists()
}

#[test]
fn ui_components_reexports_error_view_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-error_view\")]")
            && lib_source.contains("pub use ui_error_view as error_view;"),
        "ui-components should re-export the external ui-error-view crate as `error_view`.",
    );
    assert!(
        cargo_source.contains("component-error_view = [\"dep:ui-error-view\"]"),
        "component-error_view feature should depend on dep:ui-error-view after extraction.",
    );
    assert!(
        cargo_source.contains(
            "ui-error-view = { path = \"../../components/error-view\", optional = true }"
        ),
        "ui-components Cargo.toml should include the optional ui-error-view dependency.",
    );
}

#[test]
fn error_view_does_not_expose_logic_or_view_modules() {
    let source = load_error_view_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ErrorView internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn error_view_uses_logic_state_model() {
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/error_view.rs");

    for needle in [
        "pub use ui_state_primitives::error_view::{",
        "pub struct ErrorViewNormalizeInput",
        "pub struct ErrorViewNormalizedProps",
        "pub fn normalize_props(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "compact_source_attr",
        "bordered_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ErrorView logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_props(logic::ErrorViewNormalizeInput {",
        "logic::resolve_state(state_input.get_value())",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "error_view_attrs(visible, normalized.aria_label, lang, dir)",
        "motion::attach_motion(root_ref, visible, motion)",
    ] {
        assert!(
            view_source.contains(needle),
            "ErrorView view should derive state via logic/motion helpers; missing `{needle}`."
        );
    }

    for needle in [
        "pub enum ErrorViewTone",
        "pub struct ErrorViewStateInput",
        "pub struct ErrorViewState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ErrorView primitives should include `{needle}` in ui-state-primitives."
        );
    }
}

#[test]
fn error_view_component_files_respect_layered_responsibility_boundaries() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let styles_source = load_error_view_component_source("src/styles.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");

    for required in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use view::ErrorView;",
        "pub use motion::ErrorViewMotion;",
    ] {
        assert!(
            mod_source.contains(required),
            "ErrorView mod.rs should keep minimal stable export boundary via `{required}`."
        );
    }

    for forbidden in [
        "pub struct ErrorViewNormalizeInput",
        "pub fn normalize_props(",
        "pub fn attach_motion(",
        "#[component]",
        "view!",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "ErrorView mod.rs should avoid implementation detail token `{forbidden}`."
        );
    }

    for required in [
        "pub struct ErrorViewNormalizeInput",
        "pub struct ErrorViewNormalizedProps",
        "pub fn normalize_props(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "use ui_state_primitives::error_view as error_view_state;",
    ] {
        assert!(
            logic_source.contains(required),
            "ErrorView logic.rs should own normalization/derivation contract token `{required}`."
        );
    }

    for forbidden in [
        "view!",
        "NodeRef<",
        "web_sys",
        "set_property(",
        "style=",
        "on:click=",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ErrorView logic.rs should avoid view/dom/style token `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-error-view[data-state=\"visible\"]",
        ".ui-error-view[data-state=\"hidden\"]",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fg, var(--ui-fallback-fg))",
    ] {
        assert!(
            styles_source.contains(required),
            "ErrorView styles.rs should keep token-first static css token `{required}`."
        );
    }

    for forbidden in [
        "#[component]",
        "fn ",
        "on:click=",
        "Invalid value",
        "Error view",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "ErrorView styles.rs should avoid non-style/business token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "logic::normalize_props(logic::ErrorViewNormalizeInput {",
        "logic::resolve_state(state_input.get_value())",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "error_view_attrs(visible, normalized.aria_label, lang, dir)",
        "motion::attach_motion(root_ref, visible, motion)",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView view.rs should keep structure/headless mount token `{required}`."
        );
    }

    for forbidden in [
        "SpringAnimator::new(",
        "set_property(\"--ui-error-view-translate-y\"",
        "resolve_bool_axis(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView view.rs should avoid logic/motion-engine detail token `{forbidden}`."
        );
    }

    for required in [
        "pub struct ErrorViewMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new(",
        "set_property(\"--ui-error-view-translate-y\"",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(required),
            "ErrorView motion.rs should keep motion-contract mapping token `{required}`."
        );
    }

    for forbidden in [
        "#[component]",
        "view!",
        "error_view_attrs(",
        "data-state=",
        "role=",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "ErrorView motion.rs should avoid view/a11y semantics token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_does_not_introduce_spec_rs_for_simple_component() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let check2_source = load_error_view_component_source("check2.md");
    let protocol_source = load_error_view_component_source("src/protocol.rs");

    assert!(
        !component_path_exists("src/spec.rs"),
        "ErrorView is a simple display component and should not add `src/spec.rs`."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "Spec::new(",
        "render()",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "ErrorView should not expose component-level spec API token `{forbidden}`."
        );
    }

    assert!(
        component_path_exists("src/README.md")
            || check2_source.contains("`spec.rs` 只用于少数复杂组件"),
        "ErrorView docs should stay in README/check2 when spec.rs is intentionally absent."
    );

    for required in ["pub struct ErrorViewComponentSpec", "pub schema_version:"] {
        assert!(
            protocol_source.contains(required),
            "ErrorView protocol should keep explicit schema evolution token `{required}`."
        );
    }
    assert!(
        component_path_exists("test/protocol.rs"),
        "ErrorView should keep protocol contract test when schema_version exists."
    );
}

#[test]
fn error_view_component_files_check_script_covers_directory_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-component-files.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_component_files_respect_layered_responsibility_boundaries",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_does_not_introduce_spec_rs_for_simple_component",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn error_view_check2_marks_component_directory_standard_file_layout_complete() {
    let source = load_error_view_component_source("check2.md");

    assert!(
        source.contains("- [x] 组件目录标准文件落点正确。"),
        "error-view check2 should mark component-directory standard file layout gate complete."
    );

    for required in [
        "`<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "`<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "`<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "`<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "`<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "`<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
        "error_view_component_files_respect_layered_responsibility_boundaries",
        "error_view_does_not_introduce_spec_rs_for_simple_component",
        "error_view_component_files_check_script_covers_directory_contract",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            source.contains(required),
            "error-view check2 component-directory section should reference `{required}`."
        );
    }
}

#[test]
fn error_view_file_placement_discipline_is_strict_for_component_scope() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let check2_source = load_error_view_component_source("check2.md");
    let script_source = load_workspace_source("scripts/check-ui-components-component-files.sh");
    let src_dir = workspace_dir().join("components/error-view/src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "file-placement discipline requires standard component file `{required}`."
        );
    }

    assert!(
        !src_dir.join("render.rs").exists(),
        "file-placement discipline forbids render.rs drift in error-view component."
    );

    assert!(
        !src_dir.join("spec.rs").exists(),
        "file-placement discipline keeps spec.rs optional and absent for simple error-view component."
    );

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep standard file placement boundary marker `{required}`."
        );
    }

    for forbidden in [
        "mod render;",
        "pub mod render;",
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should avoid file-placement drift marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "error_view_file_placement_discipline_is_strict_for_component_scope",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep file-placement discipline evidence marker `{needle}`."
        );
    }
}

#[test]
fn error_view_file_placement_discipline_check_script_covers_semantics_gate() {
    let script_source = load_workspace_source("scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn error_view_check2_marks_file_placement_discipline_complete() {
    let source = load_error_view_component_source("check2.md");

    assert!(
        source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "error-view check2 should mark file-placement discipline gate complete."
    );

    for required in [
        "error_view_file_placement_discipline_is_strict_for_component_scope",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            source.contains(required),
            "error-view check2 file-placement section should reference `{required}`."
        );
    }
}

#[test]
fn error_view_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let readme_source = load_error_view_component_source("src/README.md");
    let check2_source = load_error_view_component_source("check2.md");
    let script_source = load_workspace_source("scripts/check-ui-components-component-files.sh");
    let src_dir = workspace_dir().join("components/error-view/src");

    assert!(
        !src_dir.join("spec.rs").exists(),
        "simple error-view component should not introduce Hyper-Structure Builder spec.rs."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "ErrorViewSpec",
        "Spec::new(",
        ".render(",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !readme_source.contains(forbidden),
            "simple error-view component should not expose hyper-structure builder marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`ErrorView` 为展示型简单组件，当前无复杂配置固化与 Builder 需求，不引入 `src/spec.rs` 与 `*Spec::new()...render()` 链路。若未来演进为复杂配置组件，再按契约补齐 `spec.rs` + 迁移说明 + 契约测试。）",
        "error_view_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep Hyper-Structure Builder N/A evidence marker `{needle}`."
        );
    }
}

#[test]
fn error_view_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let component_manifest_source = load_error_view_component_source("src/Component.toml");
    let rbi_source = load_error_view_component_source("src/error_view.rbi");
    let src_dir = workspace_dir().join("components/error-view/src");

    assert!(
        src_dir.join("Component.toml").exists(),
        "error-view context-compression contract requires src/Component.toml."
    );
    assert!(
        src_dir.join("error_view.rbi").exists(),
        "error-view context-compression contract requires src/error_view.rbi."
    );

    for needle in [
        "schema_version = \"1\"",
        "name = \"ErrorView\"",
        "crate = \"ui-error-view\"",
        "name = \"is_invalid\"",
        "name = \"tone\"",
        "name = \"is_compact\"",
        "name = \"is_bordered\"",
        "name = \"motion\"",
        "name = \"message\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"icon\"",
        "name = \"actions\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"children\"",
        "name = \"data-state\"",
        "name = \"data-tone\"",
        "name = \"data-message-source\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest_source.contains(needle),
            "Component.toml should keep manifest marker `{needle}`."
        );
    }

    for needle in [
        "pub use crate::motion::ErrorViewMotion;",
        "pub use ui_state_primitives::error_view::{",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub const DEFAULT_MESSAGE: &str;",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "pub fn ErrorView(",
        "is_invalid: bool,",
        "tone: Option<crate::ErrorViewTone>,",
        "is_compact: Option<bool>,",
        "is_bordered: Option<bool>,",
        "dir: Option<ui_headless::A11yDirection>,",
        "children: Option<leptos::children::Children>,",
    ] {
        assert!(
            rbi_source.contains(needle),
            "error_view.rbi should keep signature projection marker `{needle}`."
        );
    }
}

#[test]
fn error_view_component_files_check_script_covers_context_compression_manifest_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn error_view_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = load_error_view_component_source("check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "error-view check2 should mark context-compression contract gate complete."
    );

    for required in [
        "components/error-view/src/Component.toml",
        "components/error-view/src/error_view.rbi",
        "error_view_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "error_view_component_files_check_script_covers_context_compression_manifest_contract",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            source.contains(required),
            "error-view check2 context-compression section should reference `{required}`."
        );
    }
}

#[test]
fn error_view_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_error_view_component_source("check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "error_view_agent_contract_is_schema_typed_and_machine_readable",
        "error_view_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "error_view_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "error-view checklist should keep Agent Contract governance rule `{required}`.",
        );
    }
}

#[test]
fn error_view_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let component_manifest_source = load_error_view_component_source("src/Component.toml");

    for needle in [
        "pub const ERROR_VIEW_AGENT_SCHEMA: &str = \"ui.error-view.agent-contract\";",
        "pub enum ErrorViewAgentSchemaVersion",
        "pub enum ErrorViewAgentIntent",
        "pub enum ErrorViewAgentAction",
        "pub enum ErrorViewAgentState",
        "pub enum ErrorViewAgentSource",
        "pub enum ErrorViewAgentStateSource",
        "pub enum ErrorViewAgentActionSource",
        "pub enum ErrorViewAgentMotionSource",
        "pub enum ErrorViewAgentConfigPolicy",
        "pub struct ErrorViewAgentContractInput",
        "pub struct ErrorViewAgentContract",
        "pub fn resolve_agent_contract(input: ErrorViewAgentContractInput) -> ErrorViewAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "error-view logic should keep typed agent contract marker `{needle}`.",
        );
    }

    for needle in [
        "let agent_contract = Memo::new(move |_| {",
        "logic::resolve_agent_contract(logic::ErrorViewAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-state-source=move || agent_contract.get().state_source.as_str()",
        "data-ui-action-source=move || agent_contract.get().action_source.as_str()",
        "data-ui-motion-source=move || agent_contract.get().motion_source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "error-view view should mount schemaized agent marker `{needle}`.",
        );
    }

    for needle in [
        "name = \"agent_contract_schema_typed_markers\"",
        "name = \"agent_contract_whitelist_render_policy\"",
        "[[agent_contract_markers]]",
        "schema = \"ui.error-view.agent-contract.v1\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-config-policy\"",
    ] {
        assert!(
            component_manifest_source.contains(needle),
            "error-view Component.toml should keep schemaized marker declaration `{needle}`.",
        );
    }
}

#[test]
fn error_view_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");

    for typed_source in [
        "schema_version: ErrorViewAgentSchemaVersion::V1",
        "intent: ErrorViewAgentIntent::ErrorFeedback",
        "ErrorViewAgentAction::AnnounceWithActions",
        "ErrorViewAgentAction::AnnounceOnly",
        "ErrorViewAgentState::Visible",
        "ErrorViewAgentState::Hidden",
        "ErrorViewAgentSource::Custom",
        "ErrorViewAgentSource::Default",
        "state_source: ErrorViewAgentStateSource::InvalidProp",
        "ErrorViewAgentActionSource::ActionsSlot",
        "ErrorViewAgentActionSource::MessageOnly",
        "ErrorViewAgentMotionSource::Custom",
        "ErrorViewAgentMotionSource::Default",
        "config_policy: ErrorViewAgentConfigPolicy::Whitelist",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "error-view agent fields should stay type-derived via `{typed_source}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "schema_name: format!(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "error-view agent contract should avoid free-form schema splicing `{forbidden}`.",
        );
    }
}

#[test]
fn error_view_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let styles_source = load_error_view_component_source("src/styles.rs");
    let mod_source = load_error_view_component_source("src/mod.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let manifest_source = load_error_view_component_source("src/Component.toml");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "error-view render path should stay whitelist-safe without `{forbidden}`.",
        );
    }

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [\"render_content\", \"render_icon\", \"render_actions\"]",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            manifest_source.contains(required),
            "error-view manifest should keep whitelist policy token `{required}`.",
        );
    }
}

#[test]
fn error_view_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should include `{needle}`.",
        );
    }
}

#[test]
fn error_view_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_error_view_component_source("check2.md");
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let mod_source = load_error_view_component_source("src/mod.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let script_source = load_workspace_source("scripts/check-ui-components-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`ErrorView` 组件不直接渲染 LLM 正文输出",
    ] {
        assert!(
            checklist_source.contains(required),
            "error-view checklist should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-stream-mode",
        "data-stream-fallback",
        "data-output-status",
    ] {
        assert!(
            !combined.contains(forbidden),
            "error-view should avoid unrelated streaming protocol marker `{forbidden}` in component runtime path.",
        );
    }

    let script_needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`."
    );
}

#[test]
fn error_view_streaming_script_covers_two_mode_definition_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-streaming.sh");

    let needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`."
    );
}

#[test]
fn error_view_check2_marks_streaming_two_mode_definition_complete() {
    let source = load_error_view_component_source("check2.md");

    assert!(
        source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "error-view check2 should mark streaming two-mode definition gate complete."
    );

    for needle in [
        "error_view_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "error_view_streaming_script_covers_two_mode_definition_contract",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            source.contains(needle),
            "error-view check2 streaming-definition section should reference `{needle}`."
        );
    }
}

#[test]
fn error_view_check2_documents_snapshot_as_default_baseline_capability() {
    let checklist_source = load_error_view_component_source("check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "N/A：`ErrorView` 组件不直接渲染 LLM 正文输出",
    ] {
        assert!(
            checklist_source.contains(required),
            "error-view checklist should keep snapshot-baseline marker `{required}`.",
        );
    }
}

#[test]
fn error_view_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for marker in [
        "pub fn ErrorView(",
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] tone: Option<ErrorViewTone>",
        "#[prop(optional)] is_compact: Option<bool>",
        "#[prop(optional)] is_bordered: Option<bool>",
        "#[prop(optional)] motion: ErrorViewMotion",
        "#[prop(optional, into)] message: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] icon: Option<ViewFn>",
        "#[prop(optional, into)] actions: Option<ViewFn>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] children: Option<Children>",
        "let normalized = logic::normalize_props(logic::ErrorViewNormalizeInput {",
        "let state = Signal::derive(move || logic::resolve_state(state_input.get_value()));",
        "let a11y = error_view_attrs(visible, normalized.aria_label, lang, dir);",
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "error-view snapshot baseline should keep complete-result render marker `{marker}`.",
        );
    }

    for marker in [
        "pub struct ErrorViewNormalizeInput",
        "pub struct ErrorViewNormalizedProps",
        "pub fn normalize_props(input: ErrorViewNormalizeInput) -> ErrorViewNormalizedProps",
        "pub fn resolve_state(input: ErrorViewStateInput) -> ErrorViewState",
        "pub fn resolve_agent_contract(input: ErrorViewAgentContractInput) -> ErrorViewAgentContract",
    ] {
        assert!(
            logic_source.contains(marker),
            "error-view logic should keep snapshot-baseline normalization marker `{marker}`.",
        );
    }

    for marker in [
        "title=\"ErrorView\"",
        "slug=\"error-view\"",
        "title=\"Hello World\"",
        "code_signal=hello_code",
        "title=\"Invalid Visibility\"",
        "code_signal=basic_code",
        "title=\"Custom Content + Motion + Actions\"",
        "code_signal=state_code",
        "<ErrorView",
        "is_invalid=true",
        "is_invalid=false",
        "tone=ErrorViewTone::Neutral",
        "is_compact=true",
        "is_bordered=true",
        "class_name=\"docs-error-view-custom\".to_string()",
        "motion=ErrorViewMotion {",
        "hidden_translate_px: 12.0",
        "hidden_opacity: 0.0",
        "hidden_scale: 0.95",
    ] {
        assert!(
            docs_source.contains(marker),
            "error-view docs should keep snapshot baseline usage marker `{marker}`.",
        );
    }
}

#[test]
fn error_view_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn error_view_check2_marks_snapshot_baseline_capability_complete() {
    let source = load_error_view_component_source("check2.md");

    assert!(
        source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "error-view check2 should mark snapshot-baseline gate complete."
    );

    for needle in [
        "error_view_check2_documents_snapshot_as_default_baseline_capability",
        "error_view_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "error_view_streaming_script_covers_snapshot_baseline_contract",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            source.contains(needle),
            "error-view check2 snapshot-baseline section should reference `{needle}`."
        );
    }
}

#[test]
fn error_view_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_error_view_component_source("check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`ErrorView` 归类为 `Streaming Optional` 且当前实现为 `N/A`（`fallback=snapshot`）",
    ] {
        assert!(
            checklist_source.contains(required),
            "error-view checklist should keep streaming responsibility marker `{required}`.",
        );
    }
}

#[test]
fn error_view_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");

    for required in [
        "role=role",
        "aria-live=move || aria_live.get()",
        "aria-hidden=move || aria_hidden.get()",
        "aria-label=aria_label",
        "lang=lang",
        "dir=dir",
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "error-view should keep continuous role/aria/data semantics via `{required}` in optional-streaming scope.",
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-stream-mode",
        "data-stream-fallback",
        "data-output-status",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "error-view should not mount fake streaming status marker `{forbidden}` when stream protocol is N/A.",
        );
    }
}

#[test]
fn error_view_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry_count",
        "retry(",
        "reconnect",
        "backoff",
        "timeout",
        "fetch(",
        "reqwest::",
        "tokio::",
        "AbortController",
        "network_error",
    ] {
        assert!(
            !combined.contains(forbidden),
            "error-view should keep streaming validation/retry/resilience policy outside component layer; found `{forbidden}`.",
        );
    }
}

#[test]
fn error_view_streaming_script_covers_required_optional_classification_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn error_view_check2_marks_streaming_required_optional_classification_complete() {
    let source = load_error_view_component_source("check2.md");

    assert!(
        source.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
        "error-view check2 should mark streaming required/optional classification gate complete."
    );

    for needle in [
        "error_view_check2_documents_streaming_required_optional_classification_rules",
        "error_view_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "error_view_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "error_view_streaming_script_covers_required_optional_classification_contract",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            source.contains(needle),
            "error-view check2 streaming required/optional section should reference `{needle}`."
        );
    }
}

#[test]
fn error_view_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let styles_source = load_error_view_component_source("src/styles.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let component_manifest_source = load_error_view_component_source("src/Component.toml");
    let rbi_source = load_error_view_component_source("src/error_view.rbi");
    let combined = format!(
        "{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}\n{protocol_source}\n{component_manifest_source}\n{rbi_source}"
    );

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "error-view non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn error_view_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let styles_source = load_error_view_component_source("src/styles.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let combined = format!(
        "{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}\n{protocol_source}"
    );

    for forbidden in [".to_string()", "String::from(", ".to_owned()"] {
        assert!(
            !combined.contains(forbidden),
            "error-view string hotspot contract should avoid `{forbidden}` in non-test sources.",
        );
    }
}

#[test]
fn error_view_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_workspace_source("scripts/check-rust-hygiene.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "find crates apps -type f -name '*.rs' -path '*/src/*' | sort",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }
}

#[test]
fn error_view_engineering_script_covers_rust_hygiene_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn error_view_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_error_view_component_source("check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "error_view_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "error_view_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow",
        "error_view_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "error_view_engineering_script_covers_rust_hygiene_contract",
        "./scripts/check-rust-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "error-view check2 rust hygiene section should reference `{needle}`.",
        );
    }
}

#[test]
fn error_view_api_naming_contract_uses_is_prefix_and_no_alias_drift() {
    let view_source = load_error_view_component_source("src/view.rs");

    for needle in [
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] is_compact: Option<bool>",
        "#[prop(optional)] is_bordered: Option<bool>",
    ] {
        assert!(
            view_source.contains(needle),
            "ErrorView boolean public prop should follow `is_*` naming via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] invalid:",
        "#[prop(optional)] compact:",
        "#[prop(optional)] bordered:",
        "#[prop(optional, into)] on_invalid_change:",
        "#[prop(optional, into)] on_compact_change:",
        "#[prop(optional, into)] on_bordered_change:",
        "#[prop(optional)] default_invalid:",
        "#[prop(optional)] default_compact:",
        "#[prop(optional)] default_bordered:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView public API should avoid naming drift alias `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_controlled_or_uncontrolled_state_axes() {
    let view_source = load_error_view_component_source("src/view.rs");

    for required in [
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] is_compact: Option<bool>",
        "#[prop(optional)] is_bordered: Option<bool>",
        "let state = Signal::derive(move || logic::resolve_state(state_input.get_value()));",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView should stay display-oriented and keep static state inputs via `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional)] is_open:",
        "#[prop(optional)] default_open:",
        "#[prop(optional, into)] on_open_change:",
        "#[prop(optional, into)] on_value_change:",
        "#[prop(optional, into)] on_invalid_change:",
        "#[prop(optional, into)] on_compact_change:",
        "#[prop(optional, into)] on_bordered_change:",
        "#[prop(optional)] default_invalid:",
        "#[prop(optional)] default_compact:",
        "#[prop(optional)] default_bordered:",
        "use_controllable_state(",
        "use_controllable_open_state_traced(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView has no writable control axis and must not expose half-controlled API token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_defaults_are_normalized_once_in_logic_layer() {
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");

    for needle in [
        "pub fn normalize_props(input: ErrorViewNormalizeInput) -> ErrorViewNormalizedProps",
        "let tone = input.tone.unwrap_or_default();",
        "let (compact, compact_source_attr) = resolve_bool_axis(input.is_compact, false);",
        "let (bordered, bordered_source_attr) = resolve_bool_axis(input.is_bordered, false);",
        "let (message, has_custom_message) = normalize_message(input.message);",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
    ] {
        assert!(
            logic_source.contains(needle),
            "ErrorView logic should centralize defaults and priority via `{needle}`."
        );
    }

    for needle in [
        "let normalized = logic::normalize_props(logic::ErrorViewNormalizeInput {",
        "let message = StoredValue::new(normalized.message);",
        "let class_name = StoredValue::new(normalized.class_name);",
        "let state_input = StoredValue::new(normalized.state_input);",
    ] {
        assert!(
            view_source.contains(needle),
            "ErrorView view should only consume normalized outputs via `{needle}`."
        );
    }

    for forbidden in [
        "unwrap_or(",
        "unwrap_or_default()",
        "match tone",
        "match is_compact",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView view must not re-introduce local default fallback via `{forbidden}`."
        );
    }
}

#[test]
fn error_view_state_normalization_is_centralized_in_logic_and_styles_only_consume_markers() {
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let styles_source = load_error_view_component_source("src/styles.rs");

    for needle in [
        "pub struct ErrorViewNormalizeInput",
        "pub struct ErrorViewNormalizedProps",
        "pub fn normalize_props(input: ErrorViewNormalizeInput) -> ErrorViewNormalizedProps",
        "pub fn resolve_state(input: ErrorViewStateInput) -> ErrorViewState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ErrorViewState) -> String",
    ] {
        assert!(
            logic_source.contains(needle),
            "ErrorView logic should own centralized state normalization contract `{needle}`."
        );
    }

    assert_eq!(
        view_source
            .matches("logic::normalize_props(logic::ErrorViewNormalizeInput {")
            .count(),
        1,
        "ErrorView view should call logic::normalize_props exactly once at input boundary."
    );
    assert_eq!(
        view_source
            .matches("logic::resolve_state(state_input.get_value())")
            .count(),
        1,
        "ErrorView view should derive state via logic::resolve_state exactly once."
    );
    assert_eq!(
        view_source
            .matches("logic::compose_class_name(class_name.get_value(), state.get())")
            .count(),
        1,
        "ErrorView view should map derived state to classes via logic::compose_class_name exactly once."
    );

    for forbidden in [
        "data-tone=move || tone",
        "data-state=move || if is_invalid",
        "is_compact.unwrap_or",
        "is_bordered.unwrap_or",
        "on:click=",
        "on:keydown=",
        "on:pointerdown=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView view must not rebuild state-machine rules outside logic.rs via `{forbidden}`."
        );
    }

    for selector in [
        ".ui-error-view[data-state=\"visible\"]",
        ".ui-error-view[data-state=\"hidden\"]",
        ".ui-error-view[data-tone=\"negative\"]",
        ".ui-error-view[data-tone=\"neutral\"]",
        ".ui-error-view[data-compact=\"true\"]",
        ".ui-error-view[data-bordered=\"true\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "ErrorView styles should consume derived semantic marker `{selector}`."
        );
    }

    for forbidden in ["logic::", "resolve_state(", "if is_invalid", "match tone"] {
        assert!(
            !styles_source.contains(forbidden),
            "ErrorView styles must remain marker-consumer only and avoid state derivation token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_discrete_state_axes_stay_type_constrained_and_avoid_bool_explosion() {
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/error_view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");

    for needle in [
        "pub enum ErrorViewTone",
        "Neutral,",
        "Negative,",
        "pub tone: ErrorViewTone,",
        "#[prop(optional)] tone: Option<ErrorViewTone>,",
        "pub tone: Option<ErrorViewTone>,",
        "let tone = input.tone.unwrap_or_default();",
    ] {
        let found = primitive_source.contains(needle)
            || logic_source.contains(needle)
            || view_source.contains(needle);
        assert!(
            found,
            "ErrorView discrete tone axis should remain enum-based via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] tone: Option<String>",
        "#[prop(optional)] tone: Option<String>",
        "pub tone: Option<String>",
        "pub tone: String",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "#[prop(optional)] is_negative:",
        "#[prop(optional)] is_neutral:",
        "#[prop(optional)] is_success:",
        "#[prop(optional)] is_warning:",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ErrorView should avoid string/bool-explosion discrete axis token `{forbidden}`."
        );
    }

    for allowed_flag in [
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] is_compact: Option<bool>",
        "#[prop(optional)] is_bordered: Option<bool>",
    ] {
        assert!(
            view_source.contains(allowed_flag),
            "ErrorView should keep orthogonal binary flags via `{allowed_flag}`."
        );
    }
}

#[test]
fn error_view_consumes_ui_state_primitives_without_business_store_coupling() {
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let mod_source = load_error_view_component_source("src/mod.rs");
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/error_view.rs");

    for needle in [
        "use ui_state_primitives::error_view as error_view_state;",
        "pub use ui_state_primitives::error_view::{",
        "pub fn resolve_state(input: ErrorViewStateInput) -> ErrorViewState {",
        "error_view_state::resolve_state(input)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ErrorViewState) -> String",
        "error_view_state::compose_class_name(base_class_name, state)",
        "pub use ui_state_primitives::error_view::{ErrorViewState, ErrorViewStateInput, ErrorViewTone};",
    ] {
        let found = logic_source.contains(needle) || mod_source.contains(needle);
        assert!(
            found,
            "ErrorView should consume ui-state-primitives as source-of-truth via `{needle}`."
        );
    }

    for primitive_contract in [
        "pub struct ErrorViewStateInput",
        "pub struct ErrorViewState",
        "pub fn resolve_state(input: ErrorViewStateInput) -> ErrorViewState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ErrorViewState) -> String",
    ] {
        assert!(
            primitive_source.contains(primitive_contract),
            "ui-state-primitives should provide reusable contract `{primitive_contract}`."
        );
    }

    for forbidden in [
        "ui-error-view--visible",
        "ui-error-view--hidden",
        "tone_class:",
        "state_class:",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ErrorView logic should map/assemble inputs only and avoid re-implementing primitive state machine token `{forbidden}`."
        );
    }

    for forbidden in [
        "crate::store",
        "AppStore",
        "GlobalStore",
        "use_store(",
        "redux",
        "zustand",
        "pinia",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "ErrorView component should not couple to business store token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_async_loading_or_retry_protocol_axis() {
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");

    for required in [
        "#[prop(optional)] is_invalid: bool",
        "let visible = Signal::derive(move || state.get().is_visible);",
        "error_view_attrs(visible, normalized.aria_label, lang, dir)",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView should stay on synchronous visibility semantics via `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] is_loading:",
        "#[prop(optional)] loading:",
        "#[prop(optional)] is_disabled:",
        "#[prop(optional)] disabled:",
        "#[prop(optional, into)] on_retry:",
        "#[prop(optional, into)] on_error:",
        "#[prop(optional, into)] on_loading_change:",
        "aria-busy",
        "use_async_action(",
        "tokio::spawn",
        "spawn_local(",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should keep async interaction axis as N/A and avoid token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_emits_baseline_style_state_data_attributes() {
    let source = load_error_view_component_source("src/view.rs");

    for attr in [
        "data-slot=\"error-view\"",
        "data-slot=\"error-view-icon\"",
        "data-slot=\"error-view-content\"",
        "data-slot=\"error-view-text\"",
        "data-slot=\"error-view-actions\"",
        "data-tone=move || state.get().tone_attr",
        "data-tone-source=normalized.tone_source_attr",
        "data-state=move || state.get().state_attr",
        "data-invalid=move || state.get().is_visible.then_some(\"true\")",
        "data-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-compact=move || state.get().is_compact.then_some(\"true\")",
        "data-compact-source=normalized.compact_source_attr",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-bordered-source=normalized.bordered_source_attr",
        "data-icon=move || state.get().has_icon.then_some(\"true\")",
        "data-actions=move || state.get().has_actions.then_some(\"true\")",
        "data-content=move || state.get().content_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "role=role",
        "aria-live=move || aria_live.get()",
        "aria-hidden=move || aria_hidden.get()",
        "lang=lang",
        "dir=dir",
    ] {
        assert!(
            source.contains(attr),
            "ErrorView should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn error_view_styles_include_visibility_tone_and_markers() {
    let source = load_error_view_component_source("src/styles.rs");

    for selector in [
        ".ui-error-view--tone-negative",
        ".ui-error-view[data-tone=\"negative\"]",
        ".ui-error-view--tone-neutral",
        ".ui-error-view[data-tone=\"neutral\"]",
        ".ui-error-view--visible",
        ".ui-error-view[data-state=\"visible\"]",
        ".ui-error-view--hidden",
        ".ui-error-view[data-state=\"hidden\"]",
        ".ui-error-view--compact",
        ".ui-error-view[data-compact=\"true\"]",
        ".ui-error-view--bordered",
        ".ui-error-view[data-bordered=\"true\"]",
        ".ui-error-view--with-actions",
        ".ui-error-view[data-actions=\"true\"]",
        ".ui-error-view--custom-class",
        ".ui-error-view[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ErrorView styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn error_view_styles_depend_on_explicit_state_markers_and_avoid_fragile_dom_guesses() {
    let styles_source = load_error_view_component_source("src/styles.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");

    for required in [
        ".ui-error-view--visible",
        ".ui-error-view[data-state=\"visible\"]",
        ".ui-error-view--hidden",
        ".ui-error-view[data-state=\"hidden\"]",
        ".ui-error-view--tone-negative",
        ".ui-error-view[data-tone=\"negative\"]",
        ".ui-error-view--tone-neutral",
        ".ui-error-view[data-tone=\"neutral\"]",
        ".ui-error-view--compact",
        ".ui-error-view[data-compact=\"true\"]",
        ".ui-error-view--bordered",
        ".ui-error-view[data-bordered=\"true\"]",
        ".ui-error-view--with-actions",
        ".ui-error-view[data-actions=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "ErrorView styles should branch from explicit semantic markers via `{required}`."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":has(",
        ".ui-error-view .ui-error-view__",
        ".ui-error-view > .ui-error-view__",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "ErrorView styles should avoid fragile DOM-structure selector `{forbidden}`."
        );
    }

    for forbidden in [" style=", "style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView view should avoid runtime inline style business logic token `{forbidden}`."
        );
    }

    for required in [
        "set_property(\"--ui-error-view-translate-y\"",
        "set_property(\"--ui-error-view-opacity\"",
        "set_property(\"--ui-error-view-scale\"",
    ] {
        assert!(
            motion_source.contains(required),
            "ErrorView motion should only adjust CSS custom properties via `{required}`."
        );
    }

    for forbidden in [
        "set_property(\"top\"",
        "set_property(\"left\"",
        "set_property(\"right\"",
        "set_property(\"bottom\"",
        "set_property(\"width\"",
        "set_property(\"height\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "ErrorView motion should not write business layout style token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_styles_consume_ui_theme_tokens() {
    let source = load_error_view_component_source("src/styles.rs");

    for token_usage in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
    ] {
        assert!(
            source.contains(token_usage),
            "ErrorView styles should consume ui-theme token variable `{token_usage}`."
        );
    }
}

#[test]
fn error_view_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_error_view_component_source("src/styles.rs");
    let theme_css_source = load_workspace_source("crates/ui-theme/src/css.rs");

    for required in [
        "gap: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "margin-top: var(--ui-border-width, var(--ui-fallback-border-width));",
        "border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
    ] {
        assert!(
            styles_source.contains(required),
            "ErrorView styles should use defensive fallback-chain token `{required}`."
        );
    }

    for required in [
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-2xs:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-danger:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-border:",
        "--ui-fallback-bg-muted:",
        "--ui-fallback-accent:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-border-width:",
        "--ui-fallback-component-height-100:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css should provide fallback terminal `{required}`."
        );
    }

    for forbidden in [
        "var(--ui-space-xs);",
        "var(--ui-space-sm);",
        "var(--ui-space-2xs);",
        "var(--ui-radius-md);",
        "var(--ui-danger);",
        "var(--ui-fg);",
        "var(--ui-font-size-100, 12px)",
        "var(--ui-line-height-100, 16px)",
        "#ff",
        "#fff",
        "#000",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "ErrorView styles should not keep non-defensive terminal token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn error_view_check2_marks_defensive_variables_contract_complete() {
    let source = load_error_view_component_source("check2.md");

    assert!(
        source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "error-view check2 should mark defensive-variables gate complete."
    );

    for needle in [
        "error_view_styles_use_defensive_variable_fallback_chain",
        "error_view_defensive_variables_check_script_covers_style_fallback_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "components/error-view/src/styles.rs",
        "crates/ui-theme/src/css.rs",
    ] {
        assert!(
            source.contains(needle),
            "error-view check2 defensive-variables section should reference `{needle}`."
        );
    }
}

#[test]
fn error_view_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_source = load_ui_components_source("src/css.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-error_view\")]",
        "out.push_str(crate::error_view::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css aggregation should keep `{required}` for error-view @layer contract."
        );
    }

    for forbidden in [
        "style=\"",
        " style=",
        "style:top",
        "style:left",
        "style:right",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "error-view view should avoid non-css-variable inline style token `{forbidden}`."
        );
    }

    for required in [
        "set_property(\"--ui-error-view-translate-y\"",
        "set_property(\"--ui-error-view-opacity\"",
        "set_property(\"--ui-error-view-scale\"",
    ] {
        assert!(
            motion_source.contains(required),
            "error-view motion runtime style path should only touch css custom property `{required}`."
        );
    }

    for forbidden in [
        "set_property(\"top\"",
        "set_property(\"left\"",
        "set_property(\"right\"",
        "set_property(\"bottom\"",
        "set_property(\"width\"",
        "set_property(\"height\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "error-view motion should forbid non-variable runtime style write `{forbidden}`."
        );
    }
}

#[test]
fn error_view_cascade_layer_check_script_covers_runtime_style_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn error_view_check2_marks_cascade_layer_contract_complete() {
    let source = load_error_view_component_source("check2.md");

    assert!(
        source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "error-view check2 should mark cascade-layer gate complete."
    );

    for needle in [
        "error_view_cascade_layer_and_runtime_style_contract_is_enforced",
        "error_view_cascade_layer_check_script_covers_runtime_style_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "crates/ui-components/src/css.rs",
        "components/error-view/src/view.rs",
        "components/error-view/src/motion.rs",
    ] {
        assert!(
            source.contains(needle),
            "error-view check2 cascade-layer section should reference `{needle}`."
        );
    }
}

#[test]
fn error_view_token_first_style_contract_flows_through_styles_css_aggregator_and_ui_root() {
    let styles_source = load_error_view_component_source("src/styles.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let cargo_source = load_error_view_component_source("Cargo.toml");
    let css_source = load_ui_components_source("src/css.rs");
    let root_source = load_ui_components_source("src/root.rs");

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-error-view",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
    ] {
        assert!(
            styles_source.contains(required),
            "ErrorView token-first styles.rs contract should contain `{required}`."
        );
    }

    for forbidden in [
        "--error-view-",
        "tailwind",
        "tw-",
        "styled_components",
        "css!(",
        "stylex",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "ErrorView styles.rs should avoid private token utility/CSS-in-Rust token `{forbidden}`."
        );
    }

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-error_view\")]",
        "out.push_str(crate::error_view::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css aggregator should include ErrorView token `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] inject_components_css: bool",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should inject aggregated component css via `{required}`."
        );
    }

    for forbidden in [
        "class=\"flex ",
        "class=\"grid ",
        "class=\"p-",
        "class=\"m-",
        "class=\"text-",
        "class=\"bg-",
        "class=\"rounded-",
        "style=",
        "style:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView view.rs should avoid utility-first/default inline style token `{forbidden}`."
        );
    }

    for forbidden in ["stylist", "emotion", "styled-components", "linaria"] {
        assert!(
            !cargo_source.contains(forbidden),
            "ErrorView component crate should not depend on CSS-in-Rust default token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_motion_contract_is_present() {
    let source = load_error_view_component_source("src/motion.rs");

    for needle in [
        "pub struct ErrorViewMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-error-view-translate-y",
        "--ui-error-view-opacity",
        "--ui-error-view-scale",
    ] {
        assert!(
            source.contains(needle),
            "ErrorView motion should expose `{needle}` for spring-driven visibility transitions."
        );
    }
}

#[test]
fn error_view_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = load_error_view_component_source("src/motion.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let platform_script_source = load_workspace_source("scripts/check-ui-components-platforms.sh");

    for required in [
        "pub struct ErrorViewMotion",
        "spring: ui_motion::spring::SpringConfig,",
        "spring: ui_motion::presets::spring_soft()",
        "pub fn sanitize_motion(",
        "let stiffness = if spring.stiffness.is_finite() && spring.stiffness > 0.0 {",
        "let damping = if spring.damping.is_finite() && spring.damping > 0.0 {",
        "spring: ui_motion::spring::SpringConfig {",
        "stiffness,",
        "damping,",
        "mass,",
        "precision,",
        "pub fn attach_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "let Some((translate, opacity, scale)) = springs.get_value() else {",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>,",
        "_visible: leptos::prelude::Signal<bool>,",
        "_motion: ErrorViewMotion,",
    ] {
        assert!(
            motion_source.contains(required),
            "ErrorView motion contract should include `{required}`."
        );
    }

    assert!(
        view_source.contains("motion::attach_motion(root_ref, visible, motion)"),
        "ErrorView view should mount motion contract via attach_motion."
    );

    for required in [
        "echo \"[platform] error-view motion contractualization (spring contract + reduced-motion + non-wasm no-op)\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            platform_script_source.contains(required),
            "platform gate should enforce error-view motion contract via `{required}`."
        );
    }
}

#[test]
fn error_view_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_ui_components_source("src/lib.rs");
    let css_source = load_ui_components_source("src/css.rs");
    let root_source = load_ui_components_source("src/root.rs");
    let active_highlight_source =
        load_workspace_source("crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable =
        load_workspace_source("crates/ui-headless/src/controllable_state.rs");
    let headless_presence = load_workspace_source("crates/ui-headless/src/presence.rs");
    let headless_a11y = load_workspace_source("crates/ui-headless/src/a11y.rs");

    for needle in [
        "#[cfg(feature = \"component-error_view\")]",
        "pub use ui_error_view as error_view;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep stable export/gate marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod overlay_open;",
        "pub mod presence;",
        "pub mod a11y;",
        "pub use leptos::web_sys",
        "pub use web_sys::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not expose internal platform/details marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-error_view\")]",
        "out.push_str(crate::error_view::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]\npub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "css entry should keep feature-gated component aggregation marker `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should keep centralized theme/i18n/css injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight entry should keep shared style/motion contract marker `{needle}`."
        );
    }

    for forbidden in ["#[component]", "pub fn ErrorView(", "ui-error-view"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should remain generic shared utility, not component-business implementation: `{forbidden}`."
        );
    }

    assert!(
        manifest_dir
            .join("../ui-visual-primitive/src/active_highlight.rs")
            .exists(),
        "ui-components should keep shared `../ui-visual-primitive/src/active_highlight.rs` entry."
    );
    assert!(
        !manifest_dir.join("src/overlay_open.rs").exists(),
        "ui-components should not define `src/overlay_open.rs`; open-state primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/presence.rs").exists(),
        "ui-components should not define `src/presence.rs`; presence primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/a11y.rs").exists(),
        "ui-components should not define `src/a11y.rs`; shared a11y helpers belong to ui-headless."
    );

    for needle in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>>",
    ] {
        assert!(
            headless_controllable.contains(needle)
                || headless_presence.contains(needle)
                || headless_a11y.contains(needle),
            "headless layer should keep canonical primitive entry marker `{needle}`."
        );
    }
}

#[test]
fn error_view_entrypoints_check_script_covers_fixed_entry_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-entrypoints.sh");

    let needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn error_view_check2_marks_ui_components_fixed_entry_contract_complete() {
    let source = load_error_view_component_source("check2.md");

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
        "error_view_ui_components_fixed_entry_files_follow_layered_boundaries",
        "error_view_entrypoints_check_script_covers_fixed_entry_contract",
        "scripts/check-ui-components-entrypoints.sh",
    ] {
        assert!(
            source.contains(required),
            "ErrorView checklist should keep fixed-entry governance rule `{required}`."
        );
    }
}

#[test]
fn error_view_docs_page_covers_primary_playgrounds() {
    let source = load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn error_view() -> AnyView",
        "title=\"ErrorView\"",
        "slug=\"error-view\"",
        "description=\"baseline-style validation error container with centralized visibility/content/source state contracts and spring-driven motion markers.\"",
        "title=\"Hello World\"",
        "code_signal=hello_code",
        "title=\"Invalid Visibility\"",
        "code_signal=basic_code",
        "title=\"Custom Content + Motion + Actions\"",
        "code_signal=state_code",
        "<ErrorView",
    ] {
        assert!(
            source.contains(needle),
            "display_extra docs page should include `{needle}` for error_view primary playground coverage.",
        );
    }
}

#[test]
fn error_view_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let check2_source = load_error_view_component_source("check2.md");
    let dx_script_source = load_workspace_source("scripts/check-ui-components-dx.sh");

    for required in [
        "pub(super) fn error_view() -> AnyView {",
        "let error_view_imports =",
        "let state_matrix_code = Signal::derive(move || {",
        "let controlled_contrast_code = Signal::derive(move || {",
        "let stream_snapshot_code = Signal::derive(move || {",
        "let source_first_code = Signal::derive(move || {",
        "title=\"Hello World\"",
        "title=\"State Matrix (Tone / Compact / Source Markers)\"",
        "title=\"Controlled vs Uncontrolled Contrast (N/A for ErrorView)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=error_view_imports.clone()",
        "code_imports=error_view_imports",
        "Copy action auto-injects missing imports for direct run.",
        "streaming is optional and falls back to snapshot rendering",
        "ErrorView has no controlled/uncontrolled state axis",
    ] {
        assert!(
            docs_source.contains(required),
            "error-view docs page should provide copy-paste-ready playground matrix token `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
        "error-view checklist should mark docs-as-product copy-paste-ready item complete."
    );
    for required in [
        "`Hello World`",
        "`State Matrix (Tone / Compact / Source Markers)`",
        "`Controlled vs Uncontrolled Contrast (N/A for ErrorView)`",
        "`Streaming / Snapshot Contract`",
        "`Source-first Starter (Copy-Paste Ready)`",
        "`apps/docs-app/src/playground.rs::compose_copy_ready_code`",
        "error_view_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "error-view checklist should include concrete docs/copy-ready evidence `{required}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        dx_script_source.contains(script_needle),
        "dx gate script should include `{script_needle}`."
    );
}

#[test]
fn error_view_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = load_error_view_component_source("check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "error-view checklist should keep source-first copy-paste-ready rule `{required}`.",
        );
    }
}

#[test]
fn error_view_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_workspace_source("apps/docs-app/src/playground.rs");
    let code_block_view_source = load_workspace_source("components/code-block/src/view.rs");
    let readme_source = load_error_view_component_source("src/README.md");

    for needle in [
        "data-slot=\"error-view-source-first-contract\"",
        "<h3>\"Source-first / Copy-Paste Ready Contract\"</h3>",
        "<code>\"Show code\"</code>",
        "compose_copy_ready_code",
        "code_imports=error_view_imports",
        "Dependency baseline (Cargo.toml)",
        "component-error_view",
        "inject-css",
        "data-slot=\"error-view-source-paths\"",
        "components/error-view/src/mod.rs",
        "components/error-view/src/logic.rs",
        "components/error-view/src/view.rs",
        "components/error-view/src/styles.rs",
        "components/error-view/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "error-view source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
        "missing_import_lines(&raw, &imports)",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep one-click copy affordance token `{needle}`.",
        );
    }

    for needle in [
        "## Source-first",
        "组件源码：`components/error-view/src/{mod,logic,view,styles,motion}.rs`",
        "package feature：`component-error_view`（可选叠加 `inject-css`）",
    ] {
        assert!(
            readme_source.contains(needle),
            "error-view README should document source-first dependency/path marker `{needle}`.",
        );
    }
}

#[test]
fn error_view_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce source-first copy-paste-ready contract `{needle}`.",
        );
    }
}

#[test]
fn error_view_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_error_view_component_source("check2.md");

    for marker in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "error_view_check2_documents_source_first_copy_paste_ready_rules",
        "error_view_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "error_view_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "error-view checklist should keep source-first copy-paste-ready evidence marker `{marker}`.",
        );
    }
}

#[test]
fn error_view_check2_documents_heroui_benchmark_docs_sync_rules() {
    let checklist_source = load_error_view_component_source("check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "error-view checklist should keep heroui-benchmark docs-sync rule `{required}`.",
        );
    }
}

#[test]
fn error_view_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_workspace_source("docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_workspace_source("apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = load_error_view_component_source("src/README.md");

    for needle in [
        "### ErrorView 同步记录（2026-02-20）",
        "参数模型同步：`ErrorView` 维持 display feedback primitive 定位",
        "component_doc!(\"ErrorView\", \"error-view\", \"Display\", display_extra::error_view)",
        "#/components/error-view",
        "`components/error-view/src/README.md` 提供等价文档入口",
        "display_extra.rs::error_view()",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include error-view synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"ErrorView\"",
        "\"error-view\"",
        "display_extra::error_view",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose error-view entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn error_view() -> AnyView",
        "title=\"ErrorView\"",
        "slug=\"error-view\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app error-view page should stay indexable via marker `{needle}`.",
        );
    }

    assert!(
        readme_source.contains("# ErrorView"),
        "error-view README should remain an equivalent component doc entry."
    );
}

#[test]
fn error_view_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn error_view_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_error_view_component_source("check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "error_view_check2_documents_heroui_benchmark_docs_sync_rules",
        "error_view_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "error_view_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "error-view check2 should keep heroui-benchmark docs-sync evidence marker `{marker}`.",
        );
    }
}

#[test]
fn error_view_visual_desire_baseline_is_guarded_by_docs_and_e2e_snapshots() {
    let pages_registry_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages.rs");
    let baseline_page_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for required in [
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline().into_any()",
    ] {
        assert!(
            pages_registry_source.contains(required),
            "docs-app component pages registry should include visual baseline route token `{required}`."
        );
    }

    for required in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page_source.contains(required),
            "theme visual baseline page should include quality baseline token `{required}`."
        );
    }

    for required in [
        "await page.goto(\"/#/components/theme-visual-baseline\");",
        "ThemeVisualBaseline",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"overlay\"][data-state=\"open\"",
        "E2E_VISUAL_BASELINE",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            e2e_source.contains(required),
            "theme visual baseline e2e contract should include `{required}`."
        );
    }
}

#[test]
fn error_view_tree_shaking_contract_is_feature_gated_and_budgeted() {
    let ui_components_cargo = load_ui_components_source("Cargo.toml");
    let ui_components_lib = load_ui_components_source("src/lib.rs");
    let ui_components_css = load_ui_components_source("src/css.rs");
    let error_view_cargo = load_error_view_component_source("Cargo.toml");
    let web_demo_cargo = load_workspace_source("apps/web-demo/Cargo.toml");
    let tree_shaking_script = load_workspace_source("scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_workspace_source("scripts/tree_shaking_budget.env");
    let ci_source = load_workspace_source(".github/workflows/ci.yml");

    for required in [
        "component-error_view = [\"dep:ui-error-view\"]",
        "ui-error-view = { path = \"../../components/error-view\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui-components Cargo feature graph should include `{required}` for package-mode component gating."
        );
    }

    for required in [
        "#[cfg(feature = \"component-error_view\")]",
        "pub use ui_error_view as error_view;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components lib export should keep feature-gated error_view token `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-error_view\")]",
        "out.push_str(crate::error_view::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css aggregation should keep feature-gated error_view token `{required}`."
        );
    }

    assert!(
        error_view_cargo.contains("[features]\ndefault = []"),
        "ui-error-view component crate should keep source-mode minimal default feature surface."
    );
    assert!(
        !error_view_cargo.contains("ui-components"),
        "ui-error-view source crate should not depend on ui-components central registry path."
    );

    for required in [
        "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "default-features = false",
        "web-demo-components",
    ] {
        assert!(
            web_demo_cargo.contains(required),
            "web-demo should opt into explicit non-default feature set via `{required}`."
        );
    }

    for required in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "cargo tree -e features -i ui-components -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "source \"$BUDGET_FILE\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking gate script should include `{required}`."
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(required),
            "tree-shaking budget file should define `{required}`."
        );
    }

    assert!(
        ci_source.contains("run: ./scripts/check-ui-components-tree-shaking.sh"),
        "CI should execute tree-shaking budget gate script."
    );
}

#[test]
fn error_view_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let script_source = load_workspace_source("scripts/check-ui-components-tree-shaking.sh");

    for required in [
        "ERROR_VIEW_MIN_FEATURES=\"component-error_view,inject-css\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_tree_shaking_contract_is_feature_gated_and_budgeted",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "ERROR_VIEW_TREE_OUTPUT=\"$(cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$ERROR_VIEW_MIN_FEATURES\")\"",
        "feature \"component-error_view\" (command-line)",
        "feature \"inject-css\" (command-line)",
        "error-view minimal feature tree should not pull all-components",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$ERROR_VIEW_MIN_FEATURES\"",
    ] {
        assert!(
            script_source.contains(required),
            "tree-shaking gate script should enforce `{required}`.",
        );
    }
}

#[test]
fn error_view_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_error_view_component_source("check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-error_view",
        "crates/ui-components/src/lib.rs",
        "crates/ui-components/src/css.rs",
        "error_view_tree_shaking_contract_is_feature_gated_and_budgeted",
        "error_view_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "scripts/check-ui-components-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "error-view check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn error_view_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/error_view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_test_source = load_error_view_component_source("test/logic.rs");
    let semantics_source = load_error_view_component_source("test/semantics.rs");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");

    for required in [
        "pub enum ErrorViewTone",
        "pub struct ErrorViewStateInput",
        "pub struct ErrorViewState",
        "ErrorViewTone::Negative => \"negative\"",
        "ErrorViewTone::Neutral => \"neutral\"",
        "pub tone: ErrorViewTone,",
    ] {
        assert!(
            primitive_source.contains(required),
            "type-level machine-readable state contract should include primitive token `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] tone: Option<ErrorViewTone>,",
        "let tone = input.tone.unwrap_or_default();",
        "pub fn normalize_props(input: ErrorViewNormalizeInput) -> ErrorViewNormalizedProps",
        "pub fn resolve_state(input: ErrorViewStateInput) -> ErrorViewState",
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-compact-source=normalized.compact_source_attr",
        "data-bordered-source=normalized.bordered_source_attr",
    ] {
        let found = logic_source.contains(required) || view_source.contains(required);
        assert!(
            found,
            "logic/view contract should include normalized machine-readable token `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] tone: Option<String>",
        "#[prop(optional, into)] tone: Option<String>",
        "pub tone: String",
        "pub tone: Option<String>",
        "data-state=move || format!(",
        "data-tone=move || format!(",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !primitive_source.contains(forbidden),
            "type/marker contract should avoid weakly-typed or free-text token `{forbidden}`."
        );
    }

    for required in [
        "fn normalize_props_uses_logic_layer_as_single_default_source()",
        "fn error_view_discrete_state_axes_stay_type_constrained_and_avoid_bool_explosion()",
        "fn error_view_state_markers_use_closed_enumerable_contract_values()",
        "fn error_view_defaults_are_normalized_once_in_logic_layer()",
    ] {
        let found = logic_test_source.contains(required) || semantics_source.contains(required);
        assert!(
            found,
            "tests should keep directly locatable contract feedback token `{required}`."
        );
    }

    for required in [
        "data-slot=\"error-view\"",
        "data-state",
        "data-compact-source",
    ] {
        assert!(
            e2e_source.contains(required),
            "e2e machine-readable selector contract should include `{required}`."
        );
    }
}

#[test]
fn error_view_dx_default_path_stays_simple_and_hides_internal_state_machine_wiring() {
    let view_source = load_error_view_component_source("src/view.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional)] state_input:",
        "#[prop(optional)] headless_state:",
        "state=move ||",
        "ui_state_primitives::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView public API should not force manual state-machine wiring token `{forbidden}`."
        );
    }

    assert!(
        docs_source.contains("let hello_code = Signal::derive(move || {")
            && docs_source.contains("title=\"Hello World\"")
            && docs_source.contains("code_signal=hello_code")
            && docs_source.contains("message=\"Please enter a valid email address\".to_string()"),
        "ErrorView docs should provide minimal Hello World path before advanced examples."
    );

    let hello_block_start = docs_source
        .find("let hello_code = Signal::derive(move || {")
        .expect("display_extra.rs should define hello_code block for error_view");
    let hello_block = &docs_source[hello_block_start..];
    let raw_start = hello_block
        .find("r#\"")
        .expect("hello_code should use raw string literal");
    let raw_end = hello_block[raw_start + 3..]
        .find("\"#")
        .map(|offset| raw_start + 3 + offset)
        .expect("hello_code raw string literal should close");
    let hello_snippet = &hello_block[raw_start + 3..raw_end];
    let hello_line_count = hello_snippet.lines().count();

    assert!(
        hello_line_count <= 5,
        "ErrorView Hello World snippet should stay within 5 lines, got {hello_line_count} lines: `{hello_snippet}`."
    );
}

#[test]
fn error_view_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na() {
    let playground_source = load_workspace_source("apps/docs-app/src/playground.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let dx_script_source = load_workspace_source("scripts/check-ui-components-dx.sh");

    for required in [
        "let scope_selector = format!(\"[data-playground-scope=\\\"{scope_id}\\\"]\");",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "<div data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "<textarea",
        "class=\"playground__test-editor\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "Original CSS is loaded. Use :scope to target this playground only.",
        "on_press=on_reset_test_css",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground should keep DX hot-style-feedback + isolated-canvas token `{required}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "title=\"ErrorView\"",
        "slug=\"error-view\"",
        "title=\"Hello World\"",
        "code_signal=hello_code",
        "title=\"Invalid Visibility\"",
        "code_signal=basic_code",
        "title=\"Custom Content + Motion + Actions\"",
        "code_signal=state_code",
    ] {
        assert!(
            section.contains(required),
            "ErrorView docs should provide isolated demo/workbench entry token `{required}`."
        );
    }

    for forbidden in [
        "Persist workbench state",
        "workbench_persist_state",
        "load_chart_workbench_state",
        "save_chart_workbench_state",
        "localStorage",
        "sessionStorage",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView is non-complex interaction scope; optional persist-state workbench token `{forbidden}` should stay N/A."
        );
    }

    for required in [
        "echo \"[dx] contract: error-view playground css hot-reload + isolated canvas\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            dx_script_source.contains(required),
            "DX gate script should include ErrorView contract token `{required}`."
        );
    }
}

#[test]
fn error_view_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let protocol_test_source = load_error_view_component_source("test/protocol.rs");
    let headless_trace_source = load_workspace_source("crates/ui-headless/src/trace.rs");
    let debug_overlay_source = load_workspace_source("apps/docs-app/src/debug_overlay.rs");
    let engineering_script_source =
        load_workspace_source("scripts/check-ui-components-engineering.sh");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "pub enum ErrorViewComponentSchemaVersion",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct ErrorViewComponentSpec",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(default)]",
        "pub schema_version: ErrorViewComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(required),
            "ErrorView protocol serde/schema contract should include `{required}`."
        );
    }

    for required in [
        "fn assert_serde<T>()",
        "T: Serialize + DeserializeOwned,",
        "fn protocol_types_implement_serde_contract()",
        "assert_serde::<ErrorViewComponentSchemaVersion>();",
        "assert_serde::<ErrorViewComponentSpec>();",
    ] {
        assert!(
            protocol_test_source.contains(required),
            "ErrorView protocol tests should include serde contract token `{required}`."
        );
    }

    for required in [
        "pub enum UiTraceEventKind",
        "OpenChange",
        "Inspect",
        "Note",
        "pub struct UiTraceEvent",
        "pub ts_ms: u64",
        "pub component: &'static str",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
    ] {
        let found =
            headless_trace_source.contains(required) || debug_overlay_source.contains(required);
        assert!(
            found,
            "Tracing semantics should stay unified via shared ui-headless trace token `{required}`."
        );
    }

    for forbidden in [
        "tracing::",
        "#[instrument",
        "event!(",
        "span!(",
        "tokio::",
        "async_std::",
        "smol::",
        "Runtime",
        "JoinHandle",
        "spawn(",
        "spawn_local(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView component surface should avoid local tracing/runtime leakage token `{forbidden}`."
        );
    }

    for required in [
        "echo \"[engineering] contract: error-view serde protocol + tracing semantics + runtime boundary leakage\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
    ] {
        assert!(
            engineering_script_source.contains(required),
            "engineering gate script should include ErrorView contract token `{required}`."
        );
    }
}

#[test]
fn error_view_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_error_view_component_source("check2.md");
    let script_source = load_workspace_source("scripts/check-ui-components-engineering.sh");
    let readme_source = load_error_view_component_source("src/README.md");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let component_manifest = load_error_view_component_source("src/Component.toml");
    let rbi_source = load_error_view_component_source("src/error_view.rbi");

    for required in [
        "pub enum ErrorViewComponentSchemaVersion",
        "V1",
        "pub struct ErrorViewComponentSpec",
        "pub schema_version: ErrorViewComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(required),
            "error-view protocol should keep stable v1 marker `{required}` in non-breaking scope.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.error-view.agent-contract.v1\"",
        "values = [\"v1\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "error-view Component.toml should keep v1 registration marker `{required}` in current scope.",
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecation_window",
        "codemod",
        "schema_version = \"2\"",
        "agent-contract.v2",
    ] {
        assert!(
            !protocol_source.contains(forbidden)
                && !readme_source.contains(forbidden)
                && !component_manifest.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "without major breaking upgrade, error-view should not claim migration surface token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `ErrorView` 改动未引入跨大版本 API 破坏升级，组件协议与 Agent Contract 仍保持 `v1`（`components/error-view/src/protocol.rs` 的 `ErrorViewComponentSchemaVersion::V1`、`components/error-view/src/Component.toml` 的 `schema_version = \"1\"` 与 `ui.error-view.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/error-view/test/semantics.rs::error_view_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-components-engineering.sh` 新增对应 `cargo test` 目标。）",
        "error_view_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep codemod/registry migration marker `{needle}`.",
        );
    }
}

#[test]
fn error_view_is_not_composite_parent_item_api_surface() {
    let view_source = load_error_view_component_source("src/view.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "#[prop(optional)] items:",
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "ItemSpec",
        "<Parent>",
        "<Item",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView should not expose composite parent/item API token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "title=\"Hello World\"",
        "code_signal=hello_code",
        "<ErrorView",
    ] {
        assert!(
            section.contains(required),
            "ErrorView docs section should keep explicit single-component path marker `{required}`."
        );
    }

    for forbidden in [
        "labels=", "titles=", "panels=", "items=", "ItemSpec", "<Parent>", "<Item",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should avoid composite conventions `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_focus_stack_or_overlay_focus_restore_contract() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    assert!(
        view_source.contains("let root_ref: NodeRef<html::Div> = NodeRef::new();")
            && motion_source.contains("NodeRef<leptos::html::Div>"),
        "ErrorView should only keep local NodeRef for motion attach, not focus restore stack logic."
    );

    for forbidden in [
        "FocusManager",
        "focus_manager",
        "focus_stack",
        "FocusStack",
        "FallbackTo",
        "restore_focus",
        "focus_restore",
        "focus_return",
        "last_focused",
        "activeElement",
        "document.body",
        "on_focus_return",
        "OverlayStack",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should not include overlay focus-stack contract token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "FocusManager",
        "focus stack",
        "FallbackTo",
        "restore focus",
        "document.body",
        "overlay stack",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should not advertise focus-stack/overlay-restore token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_foreign_zone_escape_hatch_contract() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "echarts",
        "chart_instance",
        "map_instance",
        "mapbox",
        "leaflet",
        "google.maps",
        "third_party_instance",
        "imperative_instance",
        "on_foreign_ready",
        "foreign_handle",
        "foreign_ref",
        "JsValue",
        "web_sys::HtmlCanvasElement",
        "web_sys::HtmlIFrameElement",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should not include foreign-zone/imperative-instance escape-hatch token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "Map",
        "mapbox",
        "leaflet",
        "third-party instance",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should not advertise foreign escape-hatch token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_macro_micro_drag_duality_loop() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "on:pointermove",
        "pointermove",
        "mousemove",
        "requestAnimationFrame",
        "cancelAnimationFrame",
        "raf",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "ErrorView should not include macro/micro drag state-machine token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "drag",
        "pointermove",
        "mousemove",
        "requestAnimationFrame",
        "DragEnd",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should not advertise drag-loop token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_two_pass_measure_rectification_loop() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "Intent -> Measure",
        "Rectification",
        "measure(",
        "getBoundingClientRect",
        "get_bounding_client_rect",
        "offset_width",
        "offset_height",
        "client_width",
        "client_height",
        "ResizeObserver",
        "IntersectionObserver",
        "Tooltip",
        "Popover",
        "Menu",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "ErrorView should not include two-pass geometry token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "Intent -> Measure",
        "Rectification",
        "Tooltip",
        "Popover",
        "Menu",
        "getBoundingClientRect",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should not advertise two-pass geometry token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_collection_registration_protocol_contract() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
        "#[prop(optional)] items:",
        "Vec<Item",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should not include collection registration protocol token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should not advertise collection registration token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_slot_projection_keepalive_contract() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "SlotProjection",
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "on_hidden",
        "pause_polling",
        "resume_polling",
        "keep_alive",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should not include slot projection token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in ["Lazy", "KeepAlive", "Eager", "NotifyHidden", "keep_alive"] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should not advertise slot projection token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_env_stream_subscription_pipeline() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "EnvStream",
        "BreakpointChanged",
        "ThemeChanged",
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "on:resize",
        "on:scroll",
        "debounce",
        "throttle",
        "requestIdleCallback",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should not include env stream subscription token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "BreakpointChanged",
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "debounce",
        "throttle",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should not advertise env stream token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_event_light_cone_bulk_operation_pipeline() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "context_bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "prop drilling",
        "prop_drilling",
        "select_all",
        "bulk_select",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should not include event light-cone token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "Table",
        "Grid",
        "prop drilling",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should not advertise event light-cone token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_has_no_unified_causality_bus_trace_chain() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for forbidden in [
        "CausalityBus",
        "causality_bus",
        "TraceId",
        "trace_id",
        "broadcast",
        "subscriber",
        "publish",
        "dispatch_bus",
        "bus_event",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should not include unified causality-bus token `{forbidden}`."
        );
    }

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain error_view section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain pressable_feedback section after error_view")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "TraceId",
        "trace_id",
        "broadcast",
        "subscriber",
        "causality bus",
    ] {
        assert!(
            !section.contains(forbidden),
            "ErrorView docs section should not advertise causality-bus token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_platform_contract_covers_native_ssr_wasm_and_non_wasm_source_guards() {
    let script_source = load_workspace_source("scripts/check-ui-components-platforms.sh");
    let mod_source = load_error_view_component_source("src/mod.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let styles_source = load_error_view_component_source("src/styles.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");

    for required in [
        "echo \"[platform] compile-only: default native path\"",
        "echo \"[platform] compile-only: ssr native path\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[platform] compile-only: web wasm path (ui-headless)\"",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "echo \"[platform] compile-only: error-view native path\"",
        "cargo check -p ui-components --no-default-features --features component-error_view,inject-css",
        "echo \"[platform] compile-only: error-view wasm path\"",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-error_view,inject-css",
        "echo \"[platform] source guard: non-wasm error-view files must not reference web_sys\"",
        "components/error-view/src/mod.rs",
        "components/error-view/src/logic.rs",
        "components/error-view/src/styles.rs",
        "components/error-view/src/view.rs",
        "components/error-view/src/protocol.rs",
        "echo \"[platform] source guard: error-view motion must keep explicit wasm/non-wasm branches\"",
        "components/error-view/src/motion.rs",
    ] {
        assert!(
            script_source.contains(required),
            "platform gate should include `{required}` for error-view cross-platform contract."
        );
    }

    for forbidden in ["web_sys", "web-sys"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView non-wasm paths should avoid browser-only token `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let element: leptos::web_sys::HtmlElement = node.unchecked_into();",
    ] {
        assert!(
            motion_source.contains(required),
            "ErrorView motion should keep explicit platform branch token `{required}`."
        );
    }
}

#[test]
fn error_view_headless_web_ssr_feature_mutex_is_guarded_by_compile_error_contract() {
    let headless_lib_source = load_workspace_source("crates/ui-headless/src/lib.rs");
    let platform_script_source = load_workspace_source("scripts/check-ui-components-platforms.sh");
    let view_source = load_error_view_component_source("src/view.rs");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib_source.contains(required),
            "ui-headless feature mutex contract should include `{required}`."
        );
    }

    for required in [
        "use ui_headless::{A11yDirection, error_view_attrs};",
        "let a11y = error_view_attrs(visible, normalized.aria_label, lang, dir);",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView should consume ui-headless contract token `{required}`."
        );
    }

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "echo \"[platform] compile guard: ui-headless web+ssr must fail\"",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            platform_script_source.contains(required),
            "platform guard should enforce ui-headless web/ssr mutex via `{required}`."
        );
    }
}

#[test]
fn error_view_ui_motion_non_wasm_stub_contract_keeps_ssr_tooling_compilable() {
    let ui_motion_lib_source = load_workspace_source("crates/ui-motion/src/lib.rs");
    let platform_script_source = load_workspace_source("scripts/check-ui-components-platforms.sh");
    let motion_source = load_error_view_component_source("src/motion.rs");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib_source.contains(required),
            "ui-motion non-wasm stub contract should include `{required}`."
        );
    }

    for required in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(required),
            "platform gate should include ui-motion non-wasm/tooling guard `{required}`."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>,",
        "_visible: leptos::prelude::Signal<bool>,",
        "_motion: ErrorViewMotion,",
        "let Some((translate, opacity, scale)) = springs.get_value() else {",
    ] {
        assert!(
            motion_source.contains(required),
            "ErrorView motion should keep non-wasm safe-degrade/no-handle-assumption token `{required}`."
        );
    }

    let non_wasm_branch_start = motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]")
        .unwrap_or_else(|| panic!("error-view motion should contain non-wasm cfg branch"));
    let non_wasm_branch = &motion_source[non_wasm_branch_start..];
    assert!(
        !non_wasm_branch.contains("panic!("),
        "ErrorView non-wasm attach branch should remain panic-free predictable no-op."
    );
}

#[test]
fn error_view_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let motion_source = load_error_view_component_source("src/motion.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let platform_script_source = load_workspace_source("scripts/check-ui-components-platforms.sh");

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "translate.set_target(0.0);",
        "opacity.set_target(1.0);",
        "scale.set_target(1.0);",
        "translate.set_target(motion.hidden_translate_px);",
        "opacity.set_target(motion.hidden_opacity);",
        "scale.set_target(motion.hidden_scale);",
        "let Some((translate, opacity, scale)) = springs.get_value() else {",
        "pub fn attach_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>,",
        "_visible: leptos::prelude::Signal<bool>,",
        "_motion: ErrorViewMotion,",
    ] {
        assert!(
            motion_source.contains(required),
            "ErrorView motion reduced/ssr/wasm contract should include `{required}`."
        );
    }

    for required in [
        "data-slot=\"error-view\"",
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "role=role",
        "aria-live=move || aria_live.get()",
        "aria-hidden=move || aria_hidden.get()",
        "motion::attach_motion(root_ref, visible, motion)",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView semantic render contract should include `{required}` across SSR/wasm."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if cfg!",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView view semantics should not split by platform token `{forbidden}`."
        );
    }

    for required in [
        "echo \"[platform] error-view reduced-motion/ssr/wasm contract\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            platform_script_source.contains(required),
            "platform gate should cover error-view reduced/ssr/wasm contract token `{required}`."
        );
    }
}

#[test]
fn error_view_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_workspace_source("apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_workspace_source("apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_workspace_source("apps/docs-app/src/debug_overlay.rs");
    let check2_source = load_error_view_component_source("check2.md");
    let todo_source = load_workspace_source("docs/plan/TODO.md");
    let view_source = load_error_view_component_source("src/view.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let perf_script_source = load_workspace_source("scripts/check-ui-components-performance.sh");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "\"error-view\" => UiPerfBudget {",
        "max_mount_ms: 20.0,",
        "max_update_ms: Some(6.0),",
        "max_heap_kb: Some(320.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget token `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs coverage e2e should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based performance attribution token `{needle}`."
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
            check2_source.contains(needle),
            "ErrorView checklist should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up plan should keep `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "motion::attach_motion(root_ref, visible, motion)",
    ] {
        assert!(
            view_source.contains(needle),
            "ErrorView view should expose state/render/style attribution marker `{needle}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(",
        "ui_motion::spring::SpringAnimator::new(",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ErrorView motion should expose motion-path attribution marker `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn error_view_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let docs_shell_source = load_workspace_source("apps/docs-app/src/pages/components/shell.rs");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");
    let perf_script_source = load_workspace_source("scripts/check-ui-components-performance.sh");
    let check2_source = load_error_view_component_source("check2.md");
    let todo_source = load_workspace_source("docs/plan/TODO.md");

    for marker in [
        "role=role",
        "aria-live=move || aria_live.get()",
        "aria-hidden=move || aria_hidden.get()",
        "aria-label=aria_label",
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-compact-source=normalized.compact_source_attr",
        "data-bordered-source=normalized.bordered_source_attr",
        "data-actions=move || state.get().has_actions.then_some(\"true\")",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-slot=\"error-view-actions\"",
        "{actions}",
    ] {
        assert!(
            view_source.contains(marker),
            "error-view semantics/perf matrix should keep aria/data marker `{marker}`."
        );
    }

    for marker in [
        "Playground title=\"Custom Content + Motion + Actions\"",
        "actions=move || {",
        "<ui_components::Button variant=ui_components::ButtonVariant::Secondary>",
        "\"Retry\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "docs page should keep action-slot focus-path marker `{marker}`."
        );
    }

    for marker in [
        "const retryButton = custom.getByRole(\"button\", { name: \"Retry\" });",
        "await retryButton.focus();",
        "await expect(retryButton).toBeFocused();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "e2e contract should cover action-slot focus path marker `{marker}`."
        );
    }

    for marker in [
        "\"error-view\" => UiPerfBudget {",
        "max_mount_ms: 20.0,",
        "max_update_ms: Some(6.0),",
        "max_heap_kb: Some(320.0),",
    ] {
        assert!(
            docs_shell_source.contains(marker),
            "docs shell should preserve error-view perf budget marker `{marker}`."
        );
    }

    for marker in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(marker),
            "performance script should enforce `{marker}`."
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "TODO should keep render_count follow-up marker `{marker}`.",
        );
    }

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "error_view_tests_prioritize_semantic_contracts_over_visual_snapshots_and_cover_applicable_matrix",
        "error_view_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "error_view_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            check2_source.contains(marker),
            "error-view check2 semantics/perf section should reference `{marker}`.",
        );
    }

    assert!(
        logic_source.contains("pub fn resolve_state("),
        "logic should keep state-derivation path for attributable semantics/perf regressions.",
    );
}

#[test]
fn error_view_view_macro_complexity_is_bounded_with_semantic_subblocks() {
    let view_source = load_error_view_component_source("src/view.rs");
    let script_source = load_workspace_source("scripts/check-ui-components-view-macro.sh");

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 5,
        "ErrorView view macro count should stay bounded (<=5), got {view_macro_count}."
    );

    for required in [
        "let content = render_content(children, message);",
        "let icon = render_icon(icon);",
        "let actions = render_actions(actions);",
        "{content}",
        "data-slot=\"error-view\"",
        "data-slot=\"error-view-content\"",
        "data-slot=\"error-view-icon\"",
        "data-slot=\"error-view-actions\"",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView view should keep semantic subblock token `{required}`."
        );
    }

    for required in [
        "echo \"[view-macro] contract: error-view view macro complexity\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_view_macro_complexity_is_bounded_with_semantic_subblocks",
    ] {
        assert!(
            script_source.contains(required),
            "view-macro gate script should include `{required}`."
        );
    }
}

#[test]
fn error_view_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_error_view_component_source("src/view.rs");
    let script_source = load_workspace_source("scripts/check-ui-components-view-macro.sh");

    for required in [
        "fn render_content(children: Option<Children>, message: StoredValue<String>) -> AnyView {",
        "fn render_icon(icon: Option<StoredValue<ViewFn>>) -> Option<AnyView> {",
        "fn render_actions(actions: Option<StoredValue<ViewFn>>) -> Option<AnyView> {",
        "let content = render_content(children, message);",
        "let icon = render_icon(icon);",
        "let actions = render_actions(actions);",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView view should keep function-first split token `{required}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "ErrorView should keep exactly one component entry-point; local fragments should remain plain functions."
    );

    for forbidden in [
        "#[component]\nfn render_content",
        "#[component]\nfn render_icon",
        "#[component]\nfn render_actions",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView should not promote local fragment function `{forbidden}` into extra component abstraction."
        );
    }

    for required in [
        "data-slot=\"error-view\"",
        "data-slot=\"error-view-content\"",
        "data-slot=\"error-view-icon\"",
        "data-slot=\"error-view-actions\"",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView function split should keep stable semantic marker `{required}`."
        );
    }

    for required in [
        "echo \"[view-macro] contract: error-view function-first split\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            script_source.contains(required),
            "view-macro gate script should include function-first token `{required}`."
        );
    }
}

#[test]
fn error_view_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_error_view_component_source("src/view.rs");
    let script_source = load_workspace_source("scripts/check-ui-components-view-macro.sh");

    for required in [
        "fn render_content(children: Option<Children>, message: StoredValue<String>) -> AnyView {",
        "fn render_icon(icon: Option<StoredValue<ViewFn>>) -> Option<AnyView> {",
        "fn render_actions(actions: Option<StoredValue<ViewFn>>) -> Option<AnyView> {",
        "data-slot=\"error-view\"",
        "data-slot=\"error-view-content\"",
        "data-slot=\"error-view-icon\"",
        "data-slot=\"error-view-actions\"",
        "role=role",
        "aria-live=move || aria_live.get()",
        "aria-hidden=move || aria_hidden.get()",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView static-fragment contract should keep centralized semantic token `{required}`."
        );
    }

    for forbidden in [
        "<svg",
        "<footer",
        "inner_html",
        "Lorem ipsum",
        "Terms of Service",
        "Privacy Policy",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView simple layout should avoid complex static-fragment token `{forbidden}`."
        );
    }

    assert!(
        view_source
            .matches("data-slot=\"error-view-content\"")
            .count()
            <= 2,
        "ErrorView should avoid scattering repeated static content blocks across many `view!` fragments."
    );

    for required in [
        "echo \"[view-macro] contract: error-view static fragment constantization\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_static_fragments_are_constantized_or_absent_for_simple_layout",
    ] {
        assert!(
            script_source.contains(required),
            "view-macro gate script should include static-fragment token `{required}`."
        );
    }
}

#[test]
fn error_view_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let styles_source = load_error_view_component_source("src/styles.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");
    let script_source = load_workspace_source("scripts/check-ui-components-inner-html.sh");

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "dangerous_inner_html",
        "html=",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden)
                && !docs_source.contains(forbidden)
                && !e2e_source.contains(forbidden),
            "ErrorView should forbid raw html injection token `{forbidden}` in component/docs/e2e paths."
        );
    }

    for required in [
        "echo \"[inner-html] contract: error-view component/docs reject raw html injection\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_inner_html_usage_is_forbidden_in_component_and_docs_examples",
    ] {
        assert!(
            script_source.contains(required),
            "inner-html gate script should include ErrorView contract token `{required}`."
        );
    }
}

#[test]
fn error_view_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let docs_lib_source = load_workspace_source("apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_workspace_source("apps/docs-app/src/debug_overlay.rs");
    let headless_trace_source = load_workspace_source("crates/ui-headless/src/trace.rs");
    let ui_components_cargo = load_ui_components_source("Cargo.toml");
    let script_source = load_workspace_source("scripts/check-ui-components-wasm-debug.sh");

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_lib_source.contains(required),
            "docs-app should keep dev-only wasm debug entry token `{required}`."
        );
    }

    for required in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "ui_headless::UiTraceEventKind::Inspect",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "data-component=component",
        "data-kind=kind_attr",
        "events.into_iter()",
        ".rev()",
        ".take(40)",
    ] {
        assert!(
            debug_overlay_source.contains(required),
            "debug overlay should keep replay/trace observability token `{required}`."
        );
    }

    for required in [
        "pub enum UiTraceEventKind",
        "OpenChange",
        "Inspect",
        "Note",
        "pub struct UiTraceEvent",
        "pub ts_ms: u64",
        "pub component: &'static str",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            headless_trace_source.contains(required),
            "ui-headless trace contract should include `{required}` for timestamped event replay."
        );
    }

    for required in [
        "data-state=move || state.get().state_attr",
        "data-tone-source=normalized.tone_source_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView should expose traceable state/source marker `{required}` for debug inspect snapshots."
        );
    }

    for forbidden in [
        "use_ui_trace(",
        "provide_ui_trace(",
        "UiDebugOverlay",
        "cfg!(debug_assertions)",
        "tracing::",
        "error_view-wasm-debug",
        "error_view_wasm_debug",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView component crate should keep wasm debug capability out of runtime/public API via `{forbidden}`."
        );
    }

    for forbidden in [
        "error_view-wasm-debug",
        "error-view-wasm-debug",
        "error_view_wasm_debug",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui-components Cargo feature surface should avoid component-specific wasm debug pollution token `{forbidden}`."
        );
    }

    for required in [
        "echo \"[wasm-debug] contract: error-view reuses global wasm debug trace contract\"",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            script_source.contains(required),
            "wasm-debug gate script should include ErrorView token `{required}`."
        );
    }
}

#[test]
fn error_view_hydration_discontinuity_contract_avoids_time_random_and_local_id_generation() {
    let mod_source = load_error_view_component_source("src/mod.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let protocol_source = load_error_view_component_source("src/protocol.rs");
    let root_source = load_ui_components_source("src/root.rs");

    for forbidden in [
        "Instant::now",
        "SystemTime::now",
        "UNIX_EPOCH",
        "Date::now",
        "now()",
        "Uuid::new_v4",
        "uuid::Uuid",
        "rand::",
        "thread_rng",
        "random()",
        "getrandom",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should avoid non-deterministic hydration token `{forbidden}`."
        );
    }

    for forbidden in [
        "id_seed",
        "provide_ui_id_provider(",
        "use_ui_id_provider(",
        "next_id(",
        "next_dom_id(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "ErrorView should not implement local IdProvider plumbing token `{forbidden}`."
        );
    }

    assert!(
        root_source.contains("#[prop(optional, default = 1)] id_seed: u64,")
            && root_source.contains("provide_ui_id_provider(id_seed);"),
        "Deterministic SSR/hydration id seed should stay centralized in UiRoot IdProvider."
    );
}

#[test]
fn error_view_a11y_i18n_l10n_contract_reuses_headless_and_keeps_text_overrideable() {
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");
    let mod_source = load_error_view_component_source("src/mod.rs");
    let headless_test_source = load_workspace_source("crates/ui-headless/src/test/a11y.rs");

    for required in [
        "use ui_headless::{A11yDirection, error_view_attrs};",
        "#[prop(optional, into)] message: Option<String>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let a11y = error_view_attrs(visible, normalized.aria_label, lang, dir);",
        "role=role",
        "aria-live=move || aria_live.get()",
        "aria-hidden=move || aria_hidden.get()",
        "aria-label=aria_label",
        "lang=lang",
        "dir=dir",
        "{message.get_value()}",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView should expose A11y/i18n-l10n contract token `{required}`."
        );
    }

    for required in [
        "let (message, has_custom_message) = normalize_message(input.message);",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
    ] {
        assert!(
            logic_source.contains(required),
            "ErrorView logic should centralize text normalization token `{required}`."
        );
    }

    let required = "pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE};";
    assert!(
        mod_source.contains(required),
        "ErrorView module should re-export fallback text constants via `{required}`."
    );

    for forbidden in [
        "fn error_view_attrs(",
        "fn locale_attrs(",
        "role=\"alert\"",
        "aria-live=\"assertive\"",
        "Invalid value",
        "Error view",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ErrorView view should not hardcode A11y helper or user-facing fallback text `{forbidden}`."
        );
    }

    assert!(
        headless_test_source
            .contains("fn error_view_attrs_maps_live_region_visibility_and_locale()"),
        "ui-headless should keep coverage for shared error_view_attrs locale/live-region contract."
    );
}

#[test]
fn error_view_state_markers_use_closed_enumerable_contract_values() {
    let logic_source = load_error_view_component_source("src/logic.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/error_view.rs");

    for required in [
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-tone-source=normalized.tone_source_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-compact-source=normalized.compact_source_attr",
        "data-bordered-source=normalized.bordered_source_attr",
        "aria-live=move || aria_live.get()",
        "aria-hidden=move || aria_hidden.get()",
    ] {
        assert!(
            view_source.contains(required),
            "ErrorView view should expose stable semantic marker `{required}`."
        );
    }

    for required in [
        "if is_present { \"prop\" } else { \"default\" }",
        "return (value, \"is-prop\");",
        "(default_value, \"default\")",
    ] {
        assert!(
            logic_source.contains(required),
            "ErrorView logic should keep closed source-marker values via `{required}`."
        );
    }

    for required in [
        "(\"ui-error-view--visible\", \"visible\")",
        "(\"ui-error-view--hidden\", \"hidden\")",
        "ErrorViewTone::Negative => \"negative\"",
        "ErrorViewTone::Neutral => \"neutral\"",
        "message_source_attr = if input.has_children {",
        "\"none\"",
        "if input.has_custom_message {",
        "\"custom\"",
        "\"default\"",
        "aria_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
        "motion_source_attr: if input.has_custom_motion {",
    ] {
        assert!(
            primitive_source.contains(required),
            "ErrorView primitives should constrain marker values via `{required}`."
        );
    }

    for forbidden in [
        "data-state=move || format!(",
        "data-tone=move || format!(",
        "tone_attr: format!(",
        "state_attr: format!(",
        "message_source_attr: format!(",
        "aria_source_attr: format!(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !primitive_source.contains(forbidden),
            "ErrorView should avoid dynamic free-text semantic marker generation `{forbidden}`."
        );
    }
}

#[test]
fn error_view_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"State Matrix (Tone / Compact / Source Markers)\"",
        "message=\"Compact neutral tone contract\".to_string()",
        "message=\"Compact + bordered source markers\".to_string()",
        "title=\"Invalid Visibility\"",
        "is_invalid=true",
        "message=\"Please enter a valid email address\".to_string()",
        "is_invalid=false",
        "message=\"This error stays hidden until the field becomes invalid.\".to_string()",
        "title=\"Custom Content + Motion + Actions\"",
        "tone=ErrorViewTone::Neutral",
        "is_compact=true",
        "is_bordered=true",
        "class_name=\"docs-error-view-custom\".to_string()",
        "motion=ErrorViewMotion {",
        "hidden_translate_px: 12.0",
        "hidden_opacity: 0.0",
        "hidden_scale: 0.95",
        "variant=ui_components::ButtonVariant::Secondary",
        "\"Retry\"",
        "\"Validation failed. Check highlighted fields and retry.\"",
    ] {
        assert!(
            source.contains(needle),
            "error_view docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn error_view_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_error_view_component_source("check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "error-view checklist should keep docs-sync/state-matrix rule `{required}`."
        );
    }

    for marker in [
        "error_view_check2_documents_docs_sync_and_state_matrix_rules",
        "error_view_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "error_view_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "error_view/check2.md should keep docs-sync evidence marker `{marker}`."
        );
    }
}

#[test]
fn error_view_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let view_source = load_error_view_component_source("src/view.rs");
    let logic_source = load_error_view_component_source("src/logic.rs");

    for marker in [
        "pub(super) fn error_view() -> AnyView {",
        "title=\"Hello World\"",
        "title=\"State Matrix (Tone / Compact / Source Markers)\"",
        "title=\"Controlled vs Uncontrolled Contrast (N/A for ErrorView)\"",
        "Default API sync: tone defaults to negative, is_compact/is_bordered default to false",
        "is_invalid=true",
        "is_invalid=false",
        "tone=ErrorViewTone::Neutral",
        "is_compact=true",
        "is_bordered=true",
    ] {
        assert!(
            docs_source.contains(marker),
            "error-view docs examples should keep state-matrix/API sync marker `{marker}`."
        );
    }

    for marker in [
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] tone: Option<ErrorViewTone>",
        "#[prop(optional)] is_compact: Option<bool>",
        "#[prop(optional)] is_bordered: Option<bool>",
        "#[prop(optional)] motion: ErrorViewMotion",
        "#[prop(optional, into)] message: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] icon: Option<ViewFn>",
        "#[prop(optional, into)] actions: Option<ViewFn>",
    ] {
        assert!(
            view_source.contains(marker),
            "error-view view public API should keep `{marker}` for docs/runtime sync."
        );
    }

    for marker in [
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_MESSAGE",
        "let tone = input.tone.unwrap_or_default();",
        "let (compact, compact_source_attr) = resolve_bool_axis(input.is_compact, false);",
        "let (bordered, bordered_source_attr) = resolve_bool_axis(input.is_bordered, false);",
        "let (message, has_custom_message) = normalize_message(input.message);",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
    ] {
        assert!(
            logic_source.contains(marker),
            "error-view logic defaults should keep `{marker}` for docs consistency."
        );
    }

    for forbidden in [
        "default_is_invalid",
        "on_invalid_change",
        "default_compact",
        "on_compact_change",
        "default_bordered",
        "on_bordered_change",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "error-view docs should avoid stale/aliased API token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce docs-sync/state-matrix contract `{needle}`."
        );
    }
}

#[test]
fn error_view_check2_documents_documentation_as_product_rules() {
    let check2_source = load_error_view_component_source("check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "error-view checklist should keep documentation-as-product rule `{required}`."
        );
    }
}

#[test]
fn error_view_documentation_entry_exists_with_beginner_first_progression() {
    let readme = load_error_view_component_source("src/README.md");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for marker in [
        "# ErrorView",
        "## Hello World",
        "## 常见用法",
        "## 新手路径（先用起来，再进阶）",
        "## API 约定",
        "`is_invalid` + `message`",
        "is_invalid=true",
        "tone=ErrorViewTone::Neutral",
        "icon",
        "actions",
    ] {
        assert!(
            readme.contains(marker),
            "error-view README should include beginner-friendly marker `{marker}`."
        );
    }

    let readme_hello = readme
        .find("## Hello World")
        .expect("ErrorView README should include Hello World section");
    let readme_common = readme
        .find("## 常见用法")
        .expect("ErrorView README should include common usage section");
    let readme_progressive = readme
        .find("## 新手路径（先用起来，再进阶）")
        .expect("ErrorView README should include beginner-first progression section");
    let readme_api = readme
        .find("## API 约定")
        .expect("ErrorView README should include API section");

    assert!(
        readme_hello < readme_common
            && readme_common < readme_progressive
            && readme_progressive < readme_api,
        "ErrorView README should keep default path before advanced guidance."
    );

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .expect("display_extra docs should contain error_view section");
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .expect("display_extra docs should contain pressable_feedback section after error_view");
    let section = &section_tail[..section_end_rel];

    assert!(
        section.contains("title=\"ErrorView\"")
            && section.contains("slug=\"error-view\"")
            && section.contains("title=\"Hello World\"")
            && section.contains("title=\"Invalid Visibility\"")
            && section.contains("title=\"Custom Content + Motion + Actions\""),
        "ErrorView docs-app entry should exist and include beginner/common/advanced sections."
    );

    let docs_hello = section
        .find("title=\"Hello World\"")
        .expect("ErrorView docs should include Hello World playground");
    let docs_common = section
        .find("title=\"Invalid Visibility\"")
        .expect("ErrorView docs should include common usage playground");
    let docs_advanced = section
        .find("title=\"Custom Content + Motion + Actions\"")
        .expect("ErrorView docs should include advanced usage playground");

    assert!(
        docs_hello < docs_common && docs_common < docs_advanced,
        "ErrorView docs should keep beginner-first order before advanced controls."
    );
}

#[test]
fn error_view_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce documentation-as-product contract `{needle}`."
        );
    }
}

#[test]
fn error_view_check2_marks_documentation_as_product_contract_complete() {
    let check2_source = load_error_view_component_source("check2.md");

    for marker in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "error_view_check2_documents_documentation_as_product_rules",
        "error_view_documentation_entry_exists_with_beginner_first_progression",
        "error_view_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "error-view checklist should keep documentation-as-product evidence marker `{marker}`."
        );
    }
}

#[test]
fn error_view_check2_documents_interactive_playground_rules() {
    let check2_source = load_error_view_component_source("check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "error-view checklist should keep interactive-playground rule `{required}`."
        );
    }
}

#[test]
fn error_view_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    let section_start = docs_source
        .find("pub(super) fn error_view() -> AnyView {")
        .expect("display_extra docs should contain error_view section");
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn pressable_feedback() -> AnyView {")
        .expect("display_extra docs should contain pressable_feedback section after error_view");
    let section = &section_tail[..section_end_rel];

    for marker in [
        "title=\"Interactive Playground\"",
        "data-slot=\"error-view-workbench-controls\"",
        "data-slot=\"error-view-workbench-tone\"",
        "data-slot=\"error-view-workbench-message\"",
        "data-slot=\"error-view-workbench-toggle-invalid\"",
        "data-slot=\"error-view-workbench-toggle-compact\"",
        "data-slot=\"error-view-workbench-toggle-bordered\"",
        "data-slot=\"error-view-workbench\"",
        "data-slot=\"error-view-workbench-feedback\"",
        "test_config_signal=workbench_actual_config",
        "workbench_code = Signal::derive(move || {",
        "Switch checked=workbench_is_invalid",
        "Switch checked=workbench_is_compact",
        "Switch checked=workbench_is_bordered",
        "SegmentedControl",
    ] {
        assert!(
            section.contains(marker),
            "error-view docs interactive playground should keep marker `{marker}`."
        );
    }
}

#[test]
fn error_view_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");

    for marker in [
        "docs-app error-view interactive playground key flow is repeatable with semantic breakpoints",
        "data-slot=\"error-view-workbench-controls\"",
        "data-slot=\"error-view-workbench-tone\"",
        "data-slot=\"error-view-workbench-message\"",
        "data-slot=\"error-view-workbench-toggle-invalid\"",
        "data-slot=\"error-view-workbench-toggle-compact\"",
        "data-slot=\"error-view-workbench-toggle-bordered\"",
        "data-slot=\"error-view-workbench-feedback\"",
        "toHaveAttribute(\"data-tone\", \"neutral\")",
        "toHaveAttribute(\"data-state\", \"hidden\")",
        "toHaveAttribute(\"data-state\", \"visible\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "error-view interactive playground e2e flow should keep marker `{marker}`."
        );
    }
}

#[test]
fn error_view_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_docs_app_provides_interactive_playground_for_props_state_and_preview",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce interactive-playground contract `{needle}`."
        );
    }
}

#[test]
fn error_view_e2e_check_script_covers_interactive_playground_contract() {
    let script_source = load_workspace_source("scripts/check-ui-components-e2e-error-view.sh");

    for needle in [
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_e2e_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "e2e check script should enforce interactive-playground contract `{needle}`."
        );
    }
}

#[test]
fn error_view_check2_marks_interactive_playground_contract_complete() {
    let check2_source = load_error_view_component_source("check2.md");

    for marker in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "AI Spec 联动示例 N/A（`ErrorView` 非 AI Spec 输入组件）",
        "error_view_check2_documents_interactive_playground_rules",
        "error_view_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "error_view_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "error_view_dx_check_script_covers_interactive_playground_contract",
        "error_view_e2e_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "scripts/check-ui-components-e2e-error-view.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "error-view checklist should keep interactive-playground evidence marker `{marker}`."
        );
    }
}

#[test]
fn error_view_e2e_contract_uses_semantic_selectors_and_stable_waits() {
    let source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");

    for needle in [
        "/#/components/error-view",
        "body:not(:has(#boot))",
        "data-slot=\"error-view\"",
        "data-state",
        "data-compact-source",
        "data-bordered-source",
        "data-motion-source",
        "getByRole(\"button\", { name: \"Retry\" })",
    ] {
        assert!(
            source.contains(needle),
            "error-view e2e contract should include semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout",
        "setTimeout",
        "nth-child(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !source.contains(forbidden),
            "error-view e2e contract should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_error_view_component_source("check2.md");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "e2e/tests/docs_app_error_view_contract.spec.mjs",
        "body:not(:has(#boot))",
        "data-slot=\"error-view\"",
        "ready/settled",
        "error_view_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "error_view_e2e_flow_covers_ready_and_settled_semantic_breakpoints",
        "scripts/check-ui-components-e2e-error-view.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "error_view/check2.md should keep e2e stability marker `{needle}`."
        );
    }
}

#[test]
fn error_view_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");

    for needle in [
        "/#/components/error-view",
        "body:not(:has(#boot))",
        "[data-slot=\"error-view\"][data-state=\"visible\"][data-tone=\"negative\"]",
        "[data-slot=\"error-view\"][data-state=\"hidden\"][data-hidden=\"true\"]",
        "[data-slot=\"error-view\"][data-motion-source=\"custom\"][data-actions=\"true\"]",
        "toHaveAttribute(\"data-state\", \"visible\")",
        "toHaveAttribute(\"data-state\", \"hidden\")",
        "toHaveAttribute(\"aria-hidden\", \"true\")",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "getByRole(\"button\", { name: \"Retry\" })",
    ] {
        assert!(
            e2e_source.contains(needle),
            "error-view e2e semantic-selector contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        "nth-child(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "error-view e2e selector contract should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_e2e_flow_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");

    for needle in [
        "docs-app error-view motion path uses semantic ready/settled breakpoints",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-slot=\"error-view\"][data-state=\"hidden\"][aria-hidden=\"true\"]",
        "[data-slot=\"error-view\"][data-motion-source=\"custom\"][data-state=\"visible\"]",
        "toHaveCount(1)",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "error-view e2e flow should keep semantic ready/settled marker `{needle}`."
        );
    }
}

#[test]
fn error_view_check2_documents_e2e_repeatable_flow_rules() {
    let check2_source = load_error_view_component_source("check2.md");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "e2e/tests/docs_app_error_view_contract.spec.mjs",
        "docs-app error-view flow is repeatable with semantic failure breakpoints",
        "focus/keyboard",
        "overlay、async 路径当前 N/A",
        "error_view_e2e_flow_is_repeatable_and_failure_points_are_semantic",
        "error_view_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "scripts/check-ui-components-e2e-error-view.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "error_view/check2.md should keep repeatable e2e flow marker `{needle}`."
        );
    }
}

#[test]
fn error_view_e2e_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");

    for needle in [
        "docs-app error-view flow is repeatable with semantic failure breakpoints",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-slot=\"error-view\"][data-motion-source=\"custom\"][data-actions=\"true\"]",
        "const retryButton = custom.getByRole(\"button\", { name: \"Retry\" });",
        "await retryButton.focus();",
        "await expect(retryButton).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(custom).toHaveAttribute(\"data-state\", \"visible\");",
        "await page.reload();",
        "const retryAfterReload = customAfterReload.getByRole(\"button\", { name: \"Retry\" });",
    ] {
        assert!(
            e2e_source.contains(needle),
            "error-view repeatable e2e flow should include semantic breakpoint `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep(", "nth-child("] {
        assert!(
            !e2e_source.contains(forbidden),
            "error-view repeatable e2e flow should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn error_view_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");
    let check2_source = load_error_view_component_source("check2.md");
    let script_source = load_workspace_source("scripts/check-ui-components-e2e-error-view.sh");

    for needle in [
        "[data-slot=\"error-view\"][data-state=\"hidden\"][aria-hidden=\"true\"]",
        "[data-slot=\"error-view\"][data-motion-source=\"custom\"][data-state=\"visible\"]",
        "await retryButton.focus();",
        "await expect(retryButton).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "error-view high-risk e2e path should include semantic breakpoint `{needle}`."
        );
    }

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "overlay、async 路径当前 N/A",
        "error_view_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "scripts/check-ui-components-e2e-error-view.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "error_view/check2.md should keep high-risk e2e evidence marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "error-view e2e check script should include `{script_needle}`."
    );
}

#[test]
fn error_view_tests_prioritize_semantic_contracts_over_visual_snapshots_and_cover_applicable_matrix()
 {
    let semantics_source = load_error_view_component_source("test/semantics.rs");
    let motion_source = load_error_view_component_source("src/motion.rs");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_error_view_contract.spec.mjs");

    for required in [
        "fn error_view_emits_baseline_style_state_data_attributes()",
        "fn error_view_state_markers_use_closed_enumerable_contract_values()",
        "fn error_view_visual_desire_baseline_is_guarded_by_docs_and_e2e_snapshots()",
        "fn error_view_tree_shaking_contract_is_feature_gated_and_budgeted()",
        "fn error_view_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget()",
        "fn error_view_check2_marks_tree_shaking_feature_pruning_contract_complete()",
        "fn error_view_type_system_and_semantic_markers_form_machine_readable_contract()",
        "fn error_view_styles_use_defensive_variable_fallback_chain()",
        "fn error_view_cascade_layer_and_runtime_style_contract_is_enforced()",
        "fn error_view_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe()",
        "fn error_view_component_files_respect_layered_responsibility_boundaries()",
        "fn error_view_file_placement_discipline_is_strict_for_component_scope()",
        "fn error_view_file_placement_discipline_check_script_covers_semantics_gate()",
        "fn error_view_check2_marks_file_placement_discipline_complete()",
        "fn error_view_hyper_structure_builder_spec_is_not_applicable_for_simple_component()",
        "fn error_view_context_compression_manifest_and_rbi_projection_are_present_and_current()",
        "fn error_view_component_files_check_script_covers_context_compression_manifest_contract()",
        "fn error_view_check2_marks_context_compression_manifest_and_rbi_contract_complete()",
        "fn error_view_check2_documents_agent_contract_schema_governance_rules()",
        "fn error_view_agent_contract_is_schema_typed_and_machine_readable()",
        "fn error_view_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing()",
        "fn error_view_agent_contract_render_path_is_whitelist_safe_and_script_injection_free()",
        "fn error_view_contract_hygiene_script_covers_agent_contract_schema_guards()",
        "fn error_view_check2_documents_streaming_definition_is_llm_output_only_with_two_modes()",
        "fn error_view_streaming_script_covers_two_mode_definition_contract()",
        "fn error_view_check2_marks_streaming_two_mode_definition_complete()",
        "fn error_view_check2_documents_snapshot_as_default_baseline_capability()",
        "fn error_view_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "fn error_view_streaming_script_covers_snapshot_baseline_contract()",
        "fn error_view_check2_marks_snapshot_baseline_capability_complete()",
        "fn error_view_check2_documents_streaming_required_optional_classification_rules()",
        "fn error_view_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous()",
        "fn error_view_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer()",
        "fn error_view_streaming_script_covers_required_optional_classification_contract()",
        "fn error_view_check2_marks_streaming_required_optional_classification_complete()",
        "fn error_view_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()",
        "fn error_view_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow()",
        "fn error_view_rust_hygiene_script_enforces_repo_level_hygiene_guards()",
        "fn error_view_engineering_script_covers_rust_hygiene_contract()",
        "fn error_view_check2_marks_rust_hygiene_contract_complete()",
        "fn error_view_ui_components_fixed_entry_files_follow_layered_boundaries()",
        "fn error_view_token_first_style_contract_flows_through_styles_css_aggregator_and_ui_root()",
        "fn error_view_has_no_focus_stack_or_overlay_focus_restore_contract()",
        "fn error_view_has_no_foreign_zone_escape_hatch_contract()",
        "fn error_view_platform_contract_covers_native_ssr_wasm_and_non_wasm_source_guards()",
        "fn error_view_headless_web_ssr_feature_mutex_is_guarded_by_compile_error_contract()",
        "fn error_view_ui_motion_non_wasm_stub_contract_keeps_ssr_tooling_compilable()",
        "fn error_view_reduced_motion_ssr_wasm_branches_keep_semantics_consistent()",
        "fn error_view_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn error_view_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn error_view_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
        "fn error_view_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated()",
        "fn error_view_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na()",
        "fn error_view_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot()",
        "fn error_view_check2_documents_source_first_copy_paste_ready_rules()",
        "fn error_view_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies()",
        "fn error_view_dx_check_script_covers_source_first_copy_paste_ready_contract()",
        "fn error_view_check2_marks_source_first_copy_paste_ready_contract_complete()",
        "fn error_view_check2_documents_heroui_benchmark_docs_sync_rules()",
        "fn error_view_heroui_strategy_and_component_docs_are_synchronized_and_indexable()",
        "fn error_view_dx_check_script_covers_heroui_benchmark_docs_sync_contract()",
        "fn error_view_check2_marks_heroui_benchmark_docs_sync_contract_complete()",
        "fn error_view_check2_documents_docs_sync_and_state_matrix_rules()",
        "fn error_view_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults()",
        "fn error_view_dx_check_script_covers_docs_sync_and_state_matrix_contract()",
        "fn error_view_check2_documents_documentation_as_product_rules()",
        "fn error_view_documentation_entry_exists_with_beginner_first_progression()",
        "fn error_view_dx_check_script_covers_documentation_as_product_contract()",
        "fn error_view_check2_marks_documentation_as_product_contract_complete()",
        "fn error_view_check2_documents_interactive_playground_rules()",
        "fn error_view_docs_app_provides_interactive_playground_for_props_state_and_preview()",
        "fn error_view_interactive_playground_reuses_repeatable_semantic_e2e_flow()",
        "fn error_view_dx_check_script_covers_interactive_playground_contract()",
        "fn error_view_e2e_check_script_covers_interactive_playground_contract()",
        "fn error_view_check2_marks_interactive_playground_contract_complete()",
        "fn error_view_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries()",
        "fn error_view_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()",
        "fn error_view_view_macro_complexity_is_bounded_with_semantic_subblocks()",
        "fn error_view_view_functional_split_prefers_plain_functions_over_local_components()",
        "fn error_view_static_fragments_are_constantized_or_absent_for_simple_layout()",
        "fn error_view_inner_html_usage_is_forbidden_in_component_and_docs_examples()",
        "fn error_view_hydration_discontinuity_contract_avoids_time_random_and_local_id_generation()",
        "fn error_view_has_no_controlled_or_uncontrolled_state_axes()",
        "fn error_view_has_no_async_loading_or_retry_protocol_axis()",
        "fn error_view_has_no_macro_micro_drag_duality_loop()",
        "fn error_view_a11y_i18n_l10n_contract_reuses_headless_and_keeps_text_overrideable()",
        "fn error_view_check2_documents_e2e_selector_and_stable_wait_rules()",
        "fn error_view_e2e_selector_contract_uses_semantic_markers_and_stable_waits()",
        "fn error_view_e2e_flow_covers_ready_and_settled_semantic_breakpoints()",
        "fn error_view_check2_documents_e2e_repeatable_flow_rules()",
        "fn error_view_e2e_flow_is_repeatable_and_failure_points_are_semantic()",
        "fn error_view_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints()",
        "fn error_view_e2e_contract_uses_semantic_selectors_and_stable_waits()",
    ] {
        assert!(
            semantics_source.contains(required),
            "ErrorView semantics suite should include semantic-contract coverage token `{required}`."
        );
    }

    for required in [
        "data-slot=\"error-view\"",
        "data-state",
        "data-compact-source",
        "data-bordered-source",
    ] {
        assert!(
            e2e_source.contains(required),
            "ErrorView e2e should use semantic selector token `{required}`."
        );
    }

    for forbidden in [
        "toMatchSnapshot",
        "assert_snapshot!",
        "insta::assert_snapshot!",
        "image_snapshot",
        "pixelmatch",
        "waitForTimeout",
        "setTimeout",
    ] {
        assert!(
            !semantics_source.contains(forbidden) && !e2e_source.contains(forbidden),
            "ErrorView tests should avoid visual-snapshot/non-semantic token `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(required),
            "ErrorView should keep wasm/non-wasm branch coverage token `{required}`."
        );
    }
}

#[test]
fn error_view_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_error_view_component_source("src/view.rs");
    let local_semantics_source = load_error_view_component_source("test/semantics.rs");
    let suite_source = load_workspace_source("crates/ui-components/tests/error_view_semantics.rs");
    let check2_source = load_error_view_component_source("check2.md");
    let perf_script_source = load_workspace_source("scripts/check-ui-components-performance.sh");

    for marker in [
        "data-state=move || state.get().state_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "role=role",
        "aria-live=move || aria_live.get()",
        "aria-hidden=move || aria_hidden.get()",
    ] {
        assert!(
            view_source.contains(marker),
            "error-view semantic-priority contract should keep aria/data/source marker `{marker}`."
        );
    }

    for marker in [
        "fn error_view_emits_baseline_style_state_data_attributes()",
        "fn error_view_state_markers_use_closed_enumerable_contract_values()",
        "fn error_view_a11y_i18n_l10n_contract_reuses_headless_and_keeps_text_overrideable()",
        "fn error_view_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics_source.contains(marker),
            "error-view local *_semantics.rs should keep contract-first coverage marker `{marker}`."
        );
    }

    assert!(
        suite_source.contains("include!(\"../../../components/error-view/test/semantics.rs\");"),
        "ui-components error_view_semantics suite should include local component *_semantics.rs."
    );

    for forbidden in [
        "assert_snapshot!",
        "insta::assert",
        "toMatchSnapshot",
        "pixelmatch",
    ] {
        assert!(
            !local_semantics_source.contains(forbidden) && !suite_source.contains(forbidden),
            "error-view semantic-priority contract should avoid snapshot-only marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance gate script should include semantic-priority command `{script_needle}`."
    );

    for marker in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/error-view/test/semantics.rs",
        "error_view_emits_baseline_style_state_data_attributes",
        "error_view_state_markers_use_closed_enumerable_contract_values",
        "error_view_a11y_i18n_l10n_contract_reuses_headless_and_keeps_text_overrideable",
        "error_view_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "scripts/check-ui-components-performance.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "error_view/check2.md should keep semantic-test-priority evidence marker `{marker}`."
        );
    }
}

#[test]
fn error_view_check2_is_marked_complete() {
    let source = load_error_view_component_source("src/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "error_view/check2.md should not keep unchecked checklist items after completion."
    );
}
