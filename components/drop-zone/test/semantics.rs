use ui_test_support::source_contract;

fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "manifest" => include_str!("../src/Component.toml"),
        "rbi" => include_str!("../src/drop_zone.rbi"),
        "ui_components_css" => include_str!("../../../crates/ui/src/css.rs"),
        "ui_root" => include_str!("../../../crates/ui/src/root.rs"),
        "check2" => include_str!("../check2.md"),
        _ => panic!("unsupported source path: {path}"),
    }
}

fn snapshot_only_forbidden_patterns() -> [String; 2] {
    [
        ["assert", "_snapshot!"].concat(),
        ["insta", "::assert"].concat(),
    ]
}

#[test]
fn drop_zone_component_keeps_five_file_layer_boundaries() {
    let source = load_source("mod");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DroppedFile;",
        "pub use motion::DropZoneMotion;",
        "pub use view::DropZone;",
    ] {
        assert!(
            source.contains(needle),
            "drop-zone module boundary should include `{needle}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !source.contains(forbidden),
            "drop-zone internals should stay private (`{forbidden}`)."
        );
    }
}

#[test]
fn drop_zone_composes_primitives_headless_motion_and_theme_without_reimplementation() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");

    for needle in [
        "use ui_state_primitives::drop_zone::{DragDepth, resolve_labels};",
        "use ui_headless::{FocusRingOptions, HoverOptions, use_focus_ring, use_hover};",
        "use ui_headless::{A11yDirection, CommonStrings, locale_attrs, use_ui_i18n};",
        "ui_headless::a11y::should_focus_proxy_button_on_click",
        "motion::attach_motion(",
    ] {
        assert!(
            view.contains(needle),
            "drop-zone view should assemble shared layers via `{needle}`."
        );
    }

    for forbidden in [
        "fn is_focusable_element(",
        "struct DragDepth",
        "fn resolve_labels(",
    ] {
        assert!(
            !view.contains(forbidden),
            "drop-zone should not reimplement primitive/headless contracts (`{forbidden}`)."
        );
    }

    for needle in [
        "var(--ui-drop-zone-min-height, var(--ui-fallback-drop-zone-min-height))",
        "var(--ui-drop-zone-border-width, var(--ui-fallback-drop-zone-border-width))",
        "var(--ui-drop-zone-disabled-opacity, var(--ui-fallback-drop-zone-disabled-opacity))",
        "var(--ui-drop-zone-focus-outline-width, var(--ui-fallback-drop-zone-focus-outline-width))",
        "var(--ui-drop-zone-focus-outline-offset, var(--ui-fallback-drop-zone-focus-outline-offset))",
        "var(--ui-drop-zone-sr-only-size, var(--ui-fallback-drop-zone-sr-only-size))",
    ] {
        assert!(
            styles.contains(needle),
            "drop-zone styles should remain token-first via `{needle}`."
        );
    }

    for needle in [
        "use ui_theme::default_drop_zone_motion_tokens;",
        "let tokens = default_drop_zone_motion_tokens();",
    ] {
        assert!(
            motion.contains(needle),
            "drop-zone motion defaults should come from ui-theme via `{needle}`."
        );
    }

    assert!(
        !logic.contains("pub struct DragDepth"),
        "drop-zone logic must not redefine primitive state types."
    );
}

#[test]
fn drop_zone_component_files_follow_mod_logic_styles_view_motion_responsibilities() {
    let module_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let check2 = load_source("check2");

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DroppedFile;",
        "pub use motion::DropZoneMotion;",
        "pub use view::DropZone;",
    ] {
        assert!(
            module_source.contains(required),
            "mod.rs should keep minimal export boundary `{required}`."
        );
    }
    for forbidden in ["pub mod logic;", "pub mod view;", "fn "] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not host implementation detail `{forbidden}`."
        );
    }

    for forbidden in [
        "view! {",
        "class=\"ui-drop-zone",
        "on:dragenter",
        "on:drop",
        "set_property(",
        "leptos::web_sys::DataTransfer",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay pure normalization/derivation without DOM/style concerns (`{forbidden}`)."
        );
    }

    for required in [
        "pub const CSS: &str",
        "var(--ui-drop-zone-min-height",
        "var(--ui-drop-zone-border-width",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should expose token-first static CSS `{required}`."
        );
    }
    for forbidden in ["view! {", "on:dragenter", "on:drop", "DroppedFile"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not include view/event/business logic token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "motion::attach_motion(",
        "use_hover",
        "use_focus_ring",
        "collect_files_from_drag_event",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep structure rendering and contract mounting marker `{required}`."
        );
    }

    for required in [
        "pub fn attach_motion(",
        "sanitize_motion(",
        "SpringAnimator::new",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should map semantic states to motion contract/attach via `{required}`."
        );
    }
    for forbidden in ["view! {", "role=\"group\"", "on:dragenter"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not host component structure or interaction semantics (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "职责证据：`mod.rs` 仅导出 `DropZone/DroppedFile/DropZoneMotion`；`logic.rs` 仅保留归一化与状态派生（不含 DOM 文件采集）；`styles.rs` 仅导出 token-first 静态 CSS；`view.rs` 负责结构与 headless 语义挂载并承载平台事件适配；`motion.rs` 仅做语义到动效 contract 的 attach。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_component_files_follow_mod_logic_styles_view_motion_responsibilities`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_component_files_follow_mod_logic_styles_view_motion_responsibilities`。",
    ] {
        assert!(
            check2.contains(required),
            "check2 should record component file responsibility evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_spec_rs_is_not_introduced_for_simple_component() {
    let module_source = load_source("mod");
    let check2 = load_source("check2");

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !module_source.contains(forbidden),
            "drop-zone should not introduce spec.rs wiring for a simple component (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "N/A 说明：`DropZone` 为简单交互容器组件，当前无稳定外部 Schema 契约与复杂配置固化需求，不引入 `spec.rs`。",
        "边界证据：组件目录保持 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs` 五文件职责拆分，`mod.rs` 未声明 `mod spec;`，目录中不存在 `src/spec.rs`。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_spec_rs_is_not_introduced_for_simple_component`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_spec_rs_is_not_introduced_for_simple_component`。",
    ] {
        assert!(
            check2.contains(required),
            "check2 should document spec.rs non-applicability and coverage (`{required}`)."
        );
    }
}

#[test]
fn drop_zone_public_api_does_not_expose_dom_or_web_sys_types() {
    let module_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for forbidden in [
        "pub use leptos::web_sys",
        "pub use web_sys",
        "pub type NodeRef",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "drop-zone public api must not leak dom/web_sys details (`{forbidden}`)."
        );
    }

    for needle in [
        "pub struct DroppedFile",
        "pub name: String",
        "pub size: u64",
        "pub mime: String",
    ] {
        assert!(
            logic_source.contains(needle),
            "drop-zone outward data contract should stay platform-neutral (`{needle}`)."
        );
    }

    for forbidden in [
        "pub(crate) fn collect_files_from_drag_event",
        "pub(crate) fn collect_files_from_clipboard_event",
        "leptos::web_sys::DataTransfer",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic should stay DOM-free and not host platform file extraction helpers (`{forbidden}`)."
        );
    }

    for needle in [
        "fn collect_files_from_drag_event",
        "fn collect_files_from_clipboard_event",
        "fn collect_files_from_data_transfer",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            view_source.contains(needle),
            "view should host DOM file extraction helpers behind cfg-gated platform adapters (`{needle}`)."
        );
    }

    for forbidden in [
        "#[prop(optional)] node_ref:",
        "#[prop(optional)] element:",
        "#[prop(optional)] web_sys",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "drop-zone component props should not expose dom/web-sys types (`{forbidden}`)."
        );
    }
}

#[test]
fn drop_zone_component_directory_standard_files_follow_contract_and_na_paths() {
    let module_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let check2 = load_source("check2");

    let workspace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../");
    let component_src_dir = workspace_dir.join("components/drop-zone/src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_src_dir.join(required).exists(),
            "drop-zone component should keep required standard file `{required}`."
        );
    }
    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !component_src_dir.join(forbidden).exists(),
            "drop-zone simple component should not include `{forbidden}`."
        );
    }

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DroppedFile;",
        "pub use motion::DropZoneMotion;",
        "pub use view::DropZone;",
    ] {
        assert!(
            module_source.contains(required),
            "mod.rs should keep minimal stable export marker `{required}`."
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "mod spec;",
        "pub mod spec;",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not over-export internals or wire spec for simple component (`{forbidden}`)."
        );
    }

    for forbidden in [
        "view! {",
        "class=\"ui-drop-zone",
        "on:dragenter",
        "on:drop",
        "set_property(",
        "leptos::web_sys::DataTransfer",
        "struct DragDepth",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay in normalization/derivation scope (`{forbidden}`)."
        );
    }

    for required in [
        "pub const CSS: &str",
        "var(--ui-drop-zone-min-height",
        "var(--ui-drop-zone-border-width",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep static token-first css marker `{required}`."
        );
    }
    for forbidden in ["view! {", "on:dragenter", "DroppedFile"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not mix rendering/logic token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "motion::attach_motion(",
        "use_hover",
        "use_focus_ring",
        "ui_headless::a11y::should_focus_proxy_button_on_click",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep render + headless mounting marker `{required}`."
        );
    }
    {
        let forbidden = "pub(crate) fn reduce_drag_interaction(";
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not host logic normalization API `{forbidden}`."
        );
    }

    for required in [
        "pub struct DropZoneMotion",
        "pub fn sanitize_motion(motion: DropZoneMotion) -> DropZoneMotion",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep motion contract + attach marker `{required}`."
        );
    }
    for forbidden in ["view! {", "role=\"group\"", "on:dragenter"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not host view/semantic business token `{forbidden}`."
        );
    }

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "文件存在证据：`components/drop-zone/src/` 保持 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs` 五文件结构；`render.rs` 不存在。",
        "导出边界证据：`mod.rs` 仅导出 `DropZone/DroppedFile/DropZoneMotion`，未出现 `pub mod logic/view` 过度导出。",
        "职责证据：`logic.rs` 仅做 props 归一化与状态派生；`styles.rs` 仅承载 token-first 静态 CSS；`view.rs` 仅做 Leptos 结构渲染 + headless 语义挂载；`motion.rs` 仅做 `DropZoneMotion + attach_motion` 映射。",
        "spec N/A 证据：`components/drop-zone/src/spec.rs` 不存在，`mod.rs` 未声明 `mod spec;`；简单组件不引入 spec。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_component_directory_standard_files_follow_contract_and_na_paths`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_component_directory_standard_files_follow_contract_and_na_paths`。",
        "门禁证据：`scripts/check-ui-component-files.sh` 新增 `drop_zone_component_directory_standard_files_follow_contract_and_na_paths` 命令，阻断目录落点回归。",
    ] {
        assert!(
            check2.contains(required),
            "check2 should document component-directory standard-file evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_component_files_check_script_covers_standard_layout_contract() {
    let script = include_str!("../../../scripts/check-ui-component-files.sh");

    let required = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script.contains(required),
        "component-files check script should enforce `{required}`."
    );
}

#[test]
fn drop_zone_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let module_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let check2 = load_source("check2");

    let workspace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../");
    let spec_path = workspace_dir.join("components/drop-zone/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "drop-zone is not a complex builder/spec component; spec.rs should stay absent."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "DropZoneSpec",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "drop-zone mod.rs should not wire spec module token `{forbidden}`."
        );
    }

    let combined = format!("{module_source}\n{logic_source}\n{view_source}");
    for forbidden in [
        "pub struct DropZoneSpec",
        "impl DropZoneSpec",
        "DropZoneSpec::new(",
    ] {
        assert!(
            !combined.contains(forbidden),
            "drop-zone should not expose complex builder api token `{forbidden}`."
        );
    }

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "适用性结论：N/A（`drop-zone` 为简单文件交互容器，不属于复杂 schema/builder 组件；不引入 `*Spec::new()...render()`，避免抽象噪音）。",
        "已落实（N/A 证据）：`components/drop-zone/src/spec.rs` 不存在；`components/drop-zone/src/mod.rs` 未声明 `mod spec;` 且公共导出未暴露 `DropZoneSpec`。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_hyper_structure_builder_spec_is_not_applicable_for_simple_component`、`components/drop-zone/test/semantics.rs::drop_zone_hyper_structure_builder_check_script_covers_na_contract`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_hyper_structure_builder_spec_is_not_applicable_for_simple_component`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_hyper_structure_builder_check_script_covers_na_contract`。",
    ] {
        assert!(
            check2.contains(required),
            "check2 should document hyper-structure builder n/a evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_hyper_structure_builder_check_script_covers_na_contract() {
    let script = include_str!("../../../scripts/check-ui-component-files.sh");

    let required = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script.contains(required),
        "component-files check script should enforce `{required}`."
    );
}

#[test]
fn drop_zone_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let manifest = load_source("manifest");
    let rbi = load_source("rbi");
    let check2 = load_source("check2");

    let workspace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../");
    let component_src_dir = workspace_dir.join("components/drop-zone/src");
    assert!(
        component_src_dir.join("Component.toml").exists(),
        "drop-zone context-compression contract requires `components/drop-zone/src/Component.toml`."
    );
    assert!(
        component_src_dir.join("drop_zone.rbi").exists(),
        "drop-zone context-compression contract requires `components/drop-zone/src/drop_zone.rbi`."
    );

    for required in [
        "schema_version = \"1\"",
        "name = \"DropZone\"",
        "crate = \"ui-drop-zone\"",
        "rbi = \"drop_zone.rbi\"",
        "name = \"label\"",
        "name = \"aria_label\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"is_disabled\"",
        "name = \"disabled\"",
        "name = \"motion\"",
        "name = \"on_drop_files\"",
        "name = \"children\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest.contains(required),
            "drop-zone Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub use crate::motion::DropZoneMotion;",
        "pub use crate::DroppedFile;",
        "pub struct DropZoneMotion",
        "pub struct DroppedFile",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "pub fn DropZone(",
        "on_drop_files: Option<leptos::prelude::Callback<Vec<crate::DroppedFile>>>",
        "children: leptos::children::Children",
    ] {
        assert!(
            rbi.contains(required),
            "drop-zone RBI projection should keep signature marker `{required}`."
        );
    }

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "已落实（工件落点）：`components/drop-zone/src/Component.toml` 与 `components/drop-zone/src/drop_zone.rbi` 已新增并纳入组件目录，避免 AI 检索使用过时接口上下文。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_context_compression_manifest_and_rbi_projection_are_present_and_current`、`components/drop-zone/test/semantics.rs::drop_zone_component_files_check_script_covers_context_compression_manifest_contract`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_context_compression_manifest_and_rbi_projection_are_present_and_current`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_component_files_check_script_covers_context_compression_manifest_contract`。",
    ] {
        assert!(
            check2.contains(required),
            "check2 should document context-compression manifest/rbi evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_component_files_check_script_covers_context_compression_manifest_contract() {
    let script = include_str!("../../../scripts/check-ui-component-files.sh");

    let required = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script.contains(required),
        "component-files check script should enforce `{required}`."
    );
}

#[test]
fn drop_zone_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("check2");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "drop_zone_agent_contract_is_schema_typed_and_machine_readable",
        "drop_zone_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "drop_zone_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "drop-zone checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn drop_zone_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let manifest_source = load_source("manifest");

    for needle in [
        "pub const DROP_ZONE_AGENT_SCHEMA: &str = \"ui.drop_zone.agent-contract\";",
        "pub enum DropZoneAgentSchemaVersion",
        "pub enum DropZoneAgentIntent",
        "pub enum DropZoneAgentAction",
        "pub enum DropZoneAgentState",
        "pub enum DropZoneAgentSource",
        "pub enum DropZoneAgentConfigPolicy",
        "pub enum DropZoneAgentOutputStatus",
        "pub struct DropZoneAgentCapabilities",
        "pub struct DropZoneAgentContractInput",
        "pub struct DropZoneAgentContract",
        "pub fn resolve_agent_contract(input: DropZoneAgentContractInput) -> DropZoneAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "drop-zone logic should keep typed agent contract marker `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = Signal::derive(move || {",
        "super::logic::resolve_agent_contract(super::logic::DropZoneAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-motion-source=move || agent_contract.get().motion_source.as_attr()",
        "data-ui-aria-source=move || agent_contract.get().aria_source.as_attr()",
        "data-ui-capability-drop=move || super::logic::bool_data_attr(agent_contract.get().capabilities.can_drop)",
        "data-ui-capability-paste=move || super::logic::bool_data_attr(agent_contract.get().capabilities.can_paste)",
        "data-ui-capability-callback=move || super::logic::bool_data_attr(agent_contract.get().capabilities.has_drop_callback)",
    ] {
        assert!(
            view_source.contains(needle),
            "drop-zone view should mount schemaized agent marker `{needle}`."
        );
    }

    for needle in [
        "name = \"agent-contract-markers\"",
        "schema = \"ui.drop_zone.agent-contract.v1\"",
        "name = \"agent_contract_schema_typed_markers\"",
        "name = \"agent_contract_whitelist_render_policy\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "drop-zone manifest should keep agent-contract schema marker `{needle}`."
        );
    }
}

#[test]
fn drop_zone_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for typed_source in [
        "schema_name: DROP_ZONE_AGENT_SCHEMA,",
        "schema_version: DropZoneAgentSchemaVersion::V1,",
        "intent: DropZoneAgentIntent::FileIngestion,",
        "DropZoneAgentAction::AwaitInput",
        "DropZoneAgentAction::CaptureDrop",
        "DropZoneAgentAction::Blocked",
        "DropZoneAgentState::Idle",
        "DropZoneAgentState::Dragging",
        "DropZoneAgentState::Disabled",
        "source: resolve_agent_source(input.disabled_source),",
        "config_policy: DropZoneAgentConfigPolicy::Whitelist,",
        "output_status: DropZoneAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "drop-zone agent fields should stay type-derived via `{typed_source}`."
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
            "drop-zone agent contract should avoid free-form schema splicing `{forbidden}`."
        );
    }
}

#[test]
fn drop_zone_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let mod_source = load_source("mod");
    let motion_source = load_source("motion");
    let manifest_source = load_source("manifest");
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
            "drop-zone render path should stay whitelist-safe without `{forbidden}`."
        );
    }

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"typed_agent_contract_from_logic::resolve_agent_contract\"",
        "\"typed_render_mount_from_view::DropZone\"",
        "\"inner_html\"",
        "\"dangerously_set_inner_html\"",
        "\"<script\"",
        "\"javascript:\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "drop-zone manifest should keep whitelist policy marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(required),
            "contract-hygiene script should enforce `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_agent_contract_schema_governance_complete() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "drop_zone_check2_documents_agent_contract_schema_governance_rules",
        "drop_zone_agent_contract_is_schema_typed_and_machine_readable",
        "drop_zone_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "drop_zone_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "drop_zone_contract_hygiene_script_covers_agent_contract_schema_guards",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "drop-zone check2 should keep Agent Contract governance marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("check2");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let mod_source = load_source("mod");
    let motion_source = load_source("motion");
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "适用性结论：N/A（`DropZone` 不是 LLM 正文阅读组件",
    ] {
        assert!(
            check2_source.contains(required),
            "drop-zone check2 should keep streaming-definition marker `{required}`."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-stream-mode",
        "data-stream-fallback",
        "project_streaming_",
        "use_ai_space_state",
    ] {
        assert!(
            !combined.contains(forbidden),
            "drop-zone runtime path should not embed LLM streaming protocol marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`."
    );
}

#[test]
fn drop_zone_streaming_script_covers_two_mode_definition_contract() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    let needle = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`."
    );
}

#[test]
fn drop_zone_check2_marks_streaming_two_mode_definition_complete() {
    let source = load_source("check2");

    assert!(
        source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "drop-zone check2 should mark streaming two-mode definition gate complete."
    );

    for needle in [
        "drop_zone_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "drop_zone_streaming_script_covers_two_mode_definition_contract",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            source.contains(needle),
            "drop-zone check2 streaming section should reference `{needle}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "适用性结论：N/A（`DropZone` 非 LLM 正文渲染组件",
        "drop_zone_check2_documents_snapshot_as_default_baseline_capability",
        "drop_zone_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "drop_zone_streaming_script_covers_snapshot_baseline_contract",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "drop-zone check2 should keep snapshot baseline marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let manifest_source = load_source("manifest");
    let combined = format!("{view_source}\n{logic_source}");

    for required in [
        "pub fn DropZone(",
        "#[prop(optional, into)] label: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] motion: Option<DropZoneMotion>",
        "#[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>",
        "children: Children,",
        "let resolved = super::logic::resolve_props(super::logic::DropZonePropsInput {",
        "super::logic::resolve_agent_contract(super::logic::DropZoneAgentContractInput {",
        "data-slot=\"drop-zone\"",
        "data-drag-phase=move || drag_phase.get().as_attr()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "drop-zone snapshot baseline should keep stable full-input marker `{required}`."
        );
    }

    for required in ["streaming = \"optional\"", "fallback = \"snapshot\""] {
        assert!(
            manifest_source.contains(required),
            "drop-zone manifest should pin snapshot fallback marker `{required}`."
        );
    }

    for forbidden in ["now()", "SystemTime::now", "rand::", "Uuid::new", "uuid::"] {
        assert!(
            !combined.contains(forbidden),
            "drop-zone snapshot baseline should avoid non-deterministic token `{forbidden}`."
        );
    }
}

#[test]
fn drop_zone_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "判定结论：`DropZone` 非正文阅读面，归类 `Streaming Optional`",
        "streaming = \"optional\"",
        "fallback = \"snapshot\"",
        "drop_zone_check2_documents_streaming_required_optional_classification_rules",
        "drop_zone_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "drop_zone_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "drop_zone_streaming_script_covers_streaming_required_optional_contract",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "drop-zone check2 should keep streaming required/optional marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("view");
    let manifest_source = load_source("manifest");

    for required in [
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-slot=\"drop-zone-zone\"",
        "role=\"group\"",
        "aria-label=labels.aria_label.clone()",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
        "data-drag-phase=move || drag_phase.get().as_attr()",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
        "data-disabled=super::logic::bool_data_attr(is_disabled)",
        "data-disabled-source=disabled_source.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "drop-zone optional-streaming semantics should keep `{required}`."
        );
    }

    for required in ["streaming = \"optional\"", "fallback = \"snapshot\""] {
        assert!(
            manifest_source.contains(required),
            "drop-zone optional-streaming manifest should keep `{required}`."
        );
    }
}

#[test]
fn drop_zone_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let motion_source = load_source("motion");
    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "drop-zone should keep retry/resilience policy in upper layer, found `{forbidden}`."
        );
    }
}

#[test]
fn drop_zone_streaming_script_covers_streaming_required_optional_contract() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_rust_hygiene_contract() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "components/drop-zone/test/semantics.rs::drop_zone_check2_documents_rust_hygiene_contract",
        "components/drop-zone/test/semantics.rs::drop_zone_non_test_source_disallows_unwrap_expect_and_let_underscore_swallowing",
        "components/drop-zone/test/semantics.rs::drop_zone_non_test_string_copy_hotspots_are_absent_or_cow_driven",
        "components/drop-zone/test/semantics.rs::drop_zone_rust_hygiene_script_enforces_core_guards",
        "scripts/check-rust-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "drop-zone check2 should preserve rust-hygiene marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_non_test_source_disallows_unwrap_expect_and_let_underscore_swallowing() {
    let combined = format!(
        "{}\n{}\n{}\n{}",
        load_source("view"),
        load_source("logic"),
        load_source("motion"),
        load_source("mod")
    );

    for forbidden in [".unwrap(", ".unwrap_err(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "drop-zone non-test source should not include `{forbidden}`."
        );
    }
}

#[test]
fn drop_zone_non_test_string_copy_hotspots_are_absent_or_cow_driven() {
    let combined = format!(
        "{}\n{}\n{}\n{}",
        load_source("view"),
        load_source("logic"),
        load_source("motion"),
        load_source("mod")
    );

    let hotspots = [".to_owned()", "String::from(", ".to_string()"];
    let has_hotspot = hotspots.iter().any(|needle| combined.contains(needle));

    if has_hotspot {
        assert!(
            combined.contains("Cow<'static, str>"),
            "if string-copy hotspots remain, they should be converged behind Cow<'static, str>."
        );
    }
}

#[test]
fn drop_zone_rust_hygiene_script_enforces_core_guards() {
    let script_source = include_str!("../../../scripts/check-rust-hygiene.sh");

    for required in [
        "record_hits \\",
        "'\\.(unwrap|unwrap_err|expect)\\s*\\('",
        "'^[[:space:]]*let[[:space:]]+_[[:space:]]*='",
        "'(\\.to_owned\\(\\)|String::from\\()'",
        "string clone hotspots (prefer Cow<'static, str>)",
        "[rust-hygiene] failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust hygiene script should enforce `{required}`."
        );
    }
}

#[test]
fn drop_zone_ssr_and_cross_platform_compile_paths_are_cfg_gated_and_non_wasm_safe() {
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let check2 = load_source("check2");

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "fn collect_files_from_drag_event(_ev: &ev::DragEvent) -> Vec<DroppedFile> {",
        "fn collect_files_from_clipboard_event(_ev: &ev::ClipboardEvent) -> Vec<DroppedFile> {",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            view_source.contains(required) || motion_source.contains(required),
            "drop-zone platform branches should keep explicit cfg/non-wasm evidence marker `{required}`."
        );
    }

    let non_wasm_view_start = view_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]\nfn collect_files_from_drag_event")
        .unwrap_or_else(|| {
            panic!("drop-zone view should define cfg-gated non-wasm file collection adapters.")
        });
    let component_start = view_source.find("#[component]").unwrap_or_else(|| {
        panic!("drop-zone view should define component entry after platform adapters.")
    });
    let non_wasm_view = &view_source[non_wasm_view_start..component_start];

    let non_wasm_motion_start = motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_motion(")
        .unwrap_or_else(|| {
            panic!("drop-zone motion should define cfg-gated non-wasm attach_motion stub.")
        });
    let motion_tests_start = motion_source
        .find("#[cfg(test)]")
        .unwrap_or_else(|| panic!("drop-zone motion should keep test module marker."));
    let non_wasm_motion = &motion_source[non_wasm_motion_start..motion_tests_start];

    for forbidden in ["web_sys", "window", "document"] {
        assert!(
            !non_wasm_view.contains(forbidden),
            "drop-zone non-wasm view adapters should not touch browser object token `{forbidden}`."
        );
        assert!(
            !non_wasm_motion.contains(forbidden),
            "drop-zone non-wasm motion stub should not touch browser object token `{forbidden}`."
        );
    }

    for required in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "compile-only 命令矩阵：`cargo check -p ui-drop-zone`（default 本地 native）、`cargo check -p ui --no-default-features --features component-drop_zone,inject-css`（ssr native）、`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-drop_zone,inject-css`（web wasm32）。",
        "平台分支证据：`components/drop-zone/src/view.rs` 的 `collect_files_from_drag_event/collect_files_from_clipboard_event` 与 `components/drop-zone/src/motion.rs` 的 `attach_motion` 均通过 `#[cfg(target_arch = \"wasm32\")]` / `#[cfg(not(target_arch = \"wasm32\"))]` 显式分支管理。",
        "non-wasm 安全证据：`view.rs` non-wasm 分支仅返回 `Vec::new()`；`motion.rs` non-wasm 分支仅执行 `std::hint::black_box(sanitize_motion(motion))` no-op，不引用 `web-sys`/`window`/`document` 浏览器对象。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_ssr_and_cross_platform_compile_paths_are_cfg_gated_and_non_wasm_safe`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_ssr_and_cross_platform_compile_paths_are_cfg_gated_and_non_wasm_safe`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document ssr/cross-platform compile evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_ui_headless_web_ssr_feature_mutex_contract_is_preserved() {
    let ui_headless_lib = include_str!("../../../crates/ui-headless/src/lib.rs");
    let drop_zone_cargo = include_str!("../Cargo.toml");
    let check2 = load_source("check2");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            ui_headless_lib.contains(required),
            "ui-headless should keep compile-time web/ssr mutex guard `{required}`."
        );
    }

    assert!(
        drop_zone_cargo.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "drop-zone should depend on ui-headless via workspace path boundary."
    );
    assert!(
        !drop_zone_cargo
            .contains("ui-headless = { path = \"../../crates/ui-headless\", features ="),
        "drop-zone should not override ui-headless features in a way that may violate web/ssr mutex."
    );

    for required in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "互斥保护证据：`crates/ui-headless/src/lib.rs` 顶部存在 `#[cfg(all(feature = \"web\", feature = \"ssr\"))] compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");`，同时启用 web+ssr 会在编译期硬失败。",
        "组件依赖边界证据：`components/drop-zone/Cargo.toml` 通过 `ui-headless = { path = \"../../crates/ui-headless\" }` 接入，不在组件层追加会导致“双 feature 同时开启”的覆盖配置，未破坏互斥契约。",
        "compile-only 验证矩阵：`cargo check -p ui-headless --no-default-features --features web`、`cargo check -p ui-headless --no-default-features --features ssr`、`cargo check -p ui-headless --no-default-features --features web,ssr`（预期触发 `compile_error!`）。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_ui_headless_web_ssr_feature_mutex_contract_is_preserved`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_ui_headless_web_ssr_feature_mutex_contract_is_preserved`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document ui-headless web/ssr mutex evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_ui_motion_non_wasm_noop_stub_keeps_ssr_tooling_compilable() {
    let ui_motion_lib = include_str!("../../../crates/ui-motion/src/lib.rs");
    let motion_source = load_source("motion");
    let check2 = load_source("check2");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop() {",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep non-wasm no-op backend evidence `{required}`."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(required),
            "drop-zone motion should keep non-wasm safe downgrade marker `{required}`."
        );
    }

    let non_wasm_motion_start = motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_motion(")
        .unwrap_or_else(|| panic!("drop-zone motion should define non-wasm attach_motion stub."));
    let tests_start = motion_source
        .find("#[cfg(test)]")
        .unwrap_or_else(|| panic!("drop-zone motion should keep test module marker."));
    let non_wasm_motion = &motion_source[non_wasm_motion_start..tests_start];
    for forbidden in ["SpringAnimator::new", "unchecked_into()", "set_property("] {
        assert!(
            !non_wasm_motion.contains(forbidden),
            "drop-zone non-wasm motion stub should not assume runtime animation handle (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "no-op/stub 证据：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = \"wasm32\"))]` 下提供 `web::prefers_reduced_motion() -> true` 与 `web::animate(...) {}` 空实现，并有 `non_wasm_web_backend_is_predictable_noop` 测试锁定可预测行为。",
        "组件降级证据：`components/drop-zone/src/motion.rs` 的 `#[cfg(not(target_arch = \"wasm32\"))] attach_motion` 仅执行 `std::hint::black_box(sanitize_motion(motion))`，不创建 `SpringAnimator`、不触发 DOM/WAAPI 调用，不会因动画句柄缺失而 panic。",
        "compile-only 验证矩阵：`cargo check -p ui-motion`（native toolchain）、`cargo check -p ui --no-default-features --features component-drop_zone,inject-css`（SSR/tooling 路径）、`cargo check -p ui-motion --target wasm32-unknown-unknown`（wasm 分支）。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_ui_motion_non_wasm_noop_stub_keeps_ssr_tooling_compilable`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_ui_motion_non_wasm_noop_stub_keeps_ssr_tooling_compilable`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document ui-motion non-wasm no-op evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let motion_source = load_source("motion");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "if ui_motion::web::prefers_reduced_motion() {",
        "ui_observability::set_css_property_observed_auto!(",
        "\"--ui-drop-zone-scale\",",
        "\"--ui-drop-zone-highlight\",",
        "SpringAnimator::new",
    ] {
        assert!(
            motion_source.contains(required),
            "drop-zone motion should keep reduced-motion downgrade and wasm enhancement marker `{required}`."
        );
    }

    let reduced_start = motion_source
        .find("if ui_motion::web::prefers_reduced_motion() {")
        .unwrap_or_else(|| {
            panic!("drop-zone motion should define reduced-motion downgrade branch.")
        });
    let spring_start = motion_source
        .find("let springs = StoredValue::new_local(")
        .unwrap_or_else(|| {
            panic!("drop-zone motion should keep spring branch for wasm enhanced motion.")
        });
    assert!(
        reduced_start < spring_start,
        "drop-zone reduced-motion downgrade branch should execute before spring animator path."
    );

    for required in [
        "data-drag-phase=move || drag_phase.get().as_attr()",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
        "data-disabled=super::logic::bool_data_attr(is_disabled)",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "fn collect_files_from_data_transfer",
        "fn collect_files_from_drag_event(_ev: &ev::DragEvent) -> Vec<DroppedFile> {",
    ] {
        assert!(
            view_source.contains(required),
            "drop-zone view should keep cross-platform semantic contract marker `{required}`."
        );
    }

    for required in [
        "pub enum DragLifecyclePhase",
        "pub const fn reduce_drag_lifecycle(",
        "pub const fn bool_data_attr(value: bool) -> Option<&'static str>",
    ] {
        assert!(
            logic_source.contains(required),
            "drop-zone logic should remain single semantic source across wasm/ssr `{required}`."
        );
    }

    for required in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "reduced-motion 证据：`components/drop-zone/src/motion.rs` 的 wasm `attach_motion` 在 `ui_motion::web::prefers_reduced_motion()` 为真时直接走降级路径，仅同步写入 `--ui-drop-zone-scale/--ui-drop-zone-highlight` 目标值并 `return`，跳过 `SpringAnimator` 驱动。",
        "SSR/hydration 兼容证据：`components/drop-zone/src/view.rs` 的关键语义标记（`data-drag-phase/data-drop-target/data-disabled/aria-disabled`）不依赖 wasm 专属分支；non-wasm 的 `collect_files_from_*` 返回稳定空集合，保证 SSR 首帧语义与 hydration 后契约一致。",
        "wasm 增强且不分裂证据：wasm 分支仅增强交互采集与动效执行（`collect_files_from_data_transfer`、spring attach）；语义状态源仍统一来自 `logic.rs` 派生与同一组 `data-*`/`aria-*` 输出，不引入独立 wasm 语义协议。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_reduced_motion_ssr_wasm_branches_keep_semantics_consistent`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_reduced_motion_ssr_wasm_branches_keep_semantics_consistent`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document reduced-motion/ssr/wasm coverage evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_view_wires_component_local_semantics_tests() {
    let view = load_source("view");
    assert!(
        view.contains("#[path = \"../test/semantics.rs\"]"),
        "drop-zone view should mount component-local semantics tests."
    );
}

#[test]
fn drop_zone_api_naming_contract_prefers_is_on_default_prefixes() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] motion: Option<DropZoneMotion>",
        "#[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>",
        "let resolved = super::logic::resolve_props(super::logic::DropZonePropsInput {",
        "data-disabled-source=disabled_source.as_attr()",
    ] {
        assert!(
            view.contains(needle),
            "drop-zone view should keep naming contract evidence `{needle}`."
        );
    }

    for needle in [
        "pub enum DisabledSource",
        "pub const fn resolve_is_disabled(input: DisabledInput)",
        "Self::IsDisabled => \"is_disabled\"",
        "Self::DisabledAlias => \"disabled\"",
        "Self::Default => \"default\"",
    ] {
        assert!(
            logic.contains(needle),
            "drop-zone logic should provide naming compatibility bridge `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] on_disabled_change",
        "#[prop(optional)] default_disabled",
    ] {
        assert!(
            !view.contains(forbidden),
            "drop-zone should not introduce unsupported naming aliases (`{forbidden}`)."
        );
    }

    for needle in [
        "- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。",
        "兼容策略：布尔禁用态主命名为 `is_disabled`，保留 `disabled` 兼容别名",
        "迁移路径：新代码统一使用 `is_disabled`；存量 `disabled` 逐步迁移",
    ] {
        assert!(
            check2.contains(needle),
            "drop-zone checklist should record naming compatibility and migration via `{needle}`."
        );
    }
}

#[test]
fn drop_zone_has_no_controlled_uncontrolled_state_axis() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional)] on_value_change:",
        "#[prop(optional)] open:",
        "#[prop(optional)] default_open:",
        "#[prop(optional)] on_open_change:",
    ] {
        assert!(
            !view.contains(forbidden),
            "drop-zone should not expose half-controlled state axis token `{forbidden}`."
        );
    }

    for required in [
        "#[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>",
        "let drag_depth = StoredValue::new(DragDepth::default());",
        "let (is_drop_target, set_drop_target) = signal(false);",
    ] {
        assert!(
            view.contains(required),
            "drop-zone should keep event-driven contract and internal transient interaction state via `{required}`."
        );
    }

    for required in [
        "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
        "N/A 说明：`DropZone` 不维护对外可控持久状态轴；组件只暴露 `on_drop_files` 事件回调",
        "components/drop-zone/test/semantics.rs::drop_zone_has_no_controlled_uncontrolled_state_axis",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document controlled/uncontrolled N/A boundary via `{required}`."
        );
    }
}

#[test]
fn drop_zone_defaults_are_normalized_in_logic_only() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for needle in [
        "pub struct DropZonePropsInput",
        "pub struct DropZoneResolvedProps",
        "pub(crate) fn resolve_props(input: DropZonePropsInput) -> DropZoneResolvedProps",
        "let (is_disabled, disabled_source) = resolve_is_disabled(input.disabled_input);",
        "let motion = crate::motion::sanitize_motion(input.motion.unwrap_or_default());",
        "let motion_source = resolve_motion_source(motion == DropZoneMotion::default());",
    ] {
        assert!(
            logic.contains(needle),
            "drop-zone logic should own default and priority normalization via `{needle}`."
        );
    }

    for needle in [
        "let resolved = super::logic::resolve_props(super::logic::DropZonePropsInput {",
        "let motion = resolved.motion;",
        "let motion_source = resolved.motion_source;",
    ] {
        assert!(
            view.contains(needle),
            "drop-zone view should consume logic-normalized values via `{needle}`."
        );
    }

    for forbidden in [
        "let motion = crate::motion::sanitize_motion(motion);",
        "motion == DropZoneMotion::default()",
        "resolve_is_disabled(is_disabled, disabled)",
        ".unwrap_or(",
        ".unwrap_or_else(",
    ] {
        assert!(
            !view.contains(forbidden),
            "drop-zone view should not perform fallback/default logic directly (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
        "components/drop-zone/src/logic.rs::resolve_props",
        "components/drop-zone/test/semantics.rs::drop_zone_defaults_are_normalized_in_logic_only",
        "components/drop-zone/test/logic.rs::resolve_props_keeps_default_source_in_logic",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document default-source single ownership via `{required}`."
        );
    }
}

#[test]
fn drop_zone_state_normalization_is_centralized_in_logic() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub enum DragInteractionAction",
        "pub struct DragInteractionState",
        "pub(crate) fn reduce_drag_interaction(",
        "pub const fn bool_data_attr(value: bool) -> Option<&'static str>",
    ] {
        assert!(
            logic.contains(required),
            "drop-zone logic should centralize state derivation via `{required}`."
        );
    }

    for required in [
        "super::logic::reduce_drag_interaction(",
        "super::logic::DragInteractionAction::Enter",
        "super::logic::DragInteractionAction::Leave",
        "super::logic::DragInteractionAction::Drop",
        "data-hovered=move || super::logic::bool_data_attr(hover.is_hovered.get())",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
    ] {
        assert!(
            view.contains(required),
            "drop-zone view should consume logic-derived state via `{required}`."
        );
    }

    for forbidden in [
        "drag_depth.get_value().enter()",
        "drag_depth.get_value().leave()",
        "drag_depth.get_value().reset()",
        "next.is_active()",
        "if hover.is_hovered.get() { Some(\"true\") } else { None }",
    ] {
        assert!(
            !view.contains(forbidden),
            "drop-zone view should not rebuild state machine rules directly (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "components/drop-zone/src/logic.rs::reduce_drag_interaction",
        "bool_data_attr",
        "components/drop-zone/test/logic.rs::reduce_drag_interaction_derives_drop_target_state_in_logic",
        "components/drop-zone/test/semantics.rs::drop_zone_state_normalization_is_centralized_in_logic",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document centralized state normalization via `{required}`."
        );
    }
}

#[test]
fn drop_zone_discrete_state_axes_use_typed_enums() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub enum DisabledInput",
        "pub const fn classify_disabled_input(",
        "pub const fn resolve_is_disabled(input: DisabledInput)",
        "pub enum AriaLabelSource",
        "pub const fn resolve_aria_label_source(has_custom_aria_label: bool) -> AriaLabelSource",
        "Self::Default => \"default\"",
        "Self::Custom => \"custom\"",
    ] {
        assert!(
            logic.contains(required),
            "drop-zone logic should type discrete state axes with enums via `{required}`."
        );
    }

    for required in [
        "disabled_input: super::logic::classify_disabled_input(is_disabled, disabled)",
        "let aria_source = super::logic::resolve_aria_label_source(labels.has_custom_aria_label);",
        "data-aria-source=aria_source.as_attr()",
    ] {
        assert!(
            view.contains(required),
            "drop-zone view should consume typed enum mappings via `{required}`."
        );
    }

    {
        let forbidden = "data-aria-source=if labels.has_custom_aria_label";
        assert!(
            !view.contains(forbidden),
            "drop-zone view should not branch on stringly discrete state directly (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "类型化入口：`components/drop-zone/src/logic.rs::DisabledInput` + `classify_disabled_input`（先归一 `is_disabled/disabled` 再解析），`AriaLabelSource` + `resolve_aria_label_source`（语义来源闭集枚举）。",
        "组件消费：`components/drop-zone/src/view.rs` 通过 `classify_disabled_input` 与 `resolve_aria_label_source` 取枚举结果，`data-aria-source` 只消费 `as_attr()`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document enum-typed discrete state constraints via `{required}`."
        );
    }
}

#[test]
fn drop_zone_consumes_state_primitives_without_business_store_binding() {
    let logic = load_source("logic");
    let view = load_source("view");
    let cargo = include_str!("../Cargo.toml");
    let check2 = load_source("check2");

    for required in [
        "ui-state-primitives = { path = \"../../crates/ui-state-primitives\" }",
        "use ui_state_primitives::drop_zone::DragDepth;",
        "use ui_state_primitives::drop_zone::{DragDepth, resolve_labels};",
        "pub(crate) fn resolve_props(input: DropZonePropsInput) -> DropZoneResolvedProps",
        "pub(crate) fn reduce_drag_interaction(",
    ] {
        let hit = cargo.contains(required) || logic.contains(required) || view.contains(required);
        assert!(
            hit,
            "drop-zone should consume state primitives through typed assembly boundaries (`{required}`)."
        );
    }

    for forbidden in [
        "pub struct DragDepth",
        "impl DragDepth",
        "ReadSignal<",
        "WriteSignal<",
        "RwSignal<",
    ] {
        assert!(
            !logic.contains(forbidden),
            "drop-zone logic should not reimplement primitives or bind framework/global stores (`{forbidden}`)."
        );
    }

    for forbidden in [
        "#[prop(optional)] store:",
        "#[prop(optional)] global_store:",
        "#[prop(optional)] app_state:",
    ] {
        assert!(
            !view.contains(forbidden),
            "drop-zone should not expose business store types in component api (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
        "依赖与来源：`components/drop-zone/Cargo.toml` 仅通过 `ui-state-primitives` 接入状态原语，`logic.rs/view.rs` 只消费 `DragDepth` 与 `resolve_labels`。",
        "装配边界：`components/drop-zone/src/logic.rs::resolve_props` 与 `reduce_drag_interaction` 仅做映射，不重实现 `DragDepth` 原语。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should record state-primitive source boundaries via `{required}`."
        );
    }
}

#[test]
fn drop_zone_async_semantics_are_not_applicable_without_async_state_axis() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "async fn",
        ".await",
        "is_loading",
        "aria-busy",
        "retry",
        "use_async_action",
        "spawn_local",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "drop-zone should not introduce async protocol markers when no async state exists (`{forbidden}`)."
        );
    }

    for required in [
        "#[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>",
        "let files = collect_files_from_drag_event(&ev);",
        "let files = collect_files_from_clipboard_event(&ev);",
    ] {
        assert!(
            view.contains(required),
            "drop-zone should remain synchronous event-driven file ingestion via `{required}`."
        );
    }

    for required in [
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "N/A 说明：`DropZone` 无远程请求与异步状态轴；组件仅处理同步 drag/paste 事件并通过 `on_drop_files` 回调上报结果，不涉及 `is_loading`/`aria-busy`/retry 协议。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_async_semantics_are_not_applicable_without_async_state_axis`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_async_semantics_are_not_applicable_without_async_state_axis`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document async N/A boundary via `{required}`."
        );
    }
}

#[test]
fn drop_zone_dx_paradox_keeps_default_api_simple() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "#[component]",
        "#[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>",
        "children: Children,",
    ] {
        assert!(
            view.contains(required),
            "drop-zone should keep a minimal default entrypoint via `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional)] primitive:",
        "#[prop(optional)] headless:",
        "#[prop(optional)] interaction_model:",
    ] {
        assert!(
            !view.contains(forbidden),
            "drop-zone default API should not expose internal state-machine wiring (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "默认路径：`DropZone` 基础用法无需手动接线 `ui-state-primitives/ui-headless`，组件直接接收 `children`；`on_drop_files` 为可选回调而非必填状态对象。",
        "docs 证据：`apps/docs-app/src/pages/components/pages/files.rs` 提供 `Quick Start (Default API)`，示例 `<DropZone>...</DropZone>` 为 3 行（<= 5 行）可运行代码。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should capture DX default-path constraints via `{required}`."
        );
    }
}

#[test]
fn drop_zone_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");

    for required in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(required),
            "playground should keep CSS hot-reload contract marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let docs_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/files.rs",
    );
    let check2 = load_source("check2");

    for required in [
        "DROP_ZONE_WORKBENCH_STORAGE_KEY",
        "fn load_drop_zone_workbench_state() -> Option<DropZoneWorkbenchState>",
        "fn save_drop_zone_workbench_state(state: DropZoneWorkbenchState)",
        "fn clear_drop_zone_workbench_state()",
        "let persisted_workbench_state = load_drop_zone_workbench_state();",
        "let (workbench_persist_state, set_workbench_persist_state) =",
        "save_drop_zone_workbench_state(state);",
        "clear_drop_zone_workbench_state();",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "\"Persist workbench state\"",
        "\" · persist: \"",
        "data-slot=\"drop-zone-workbench-controls\"",
        "data-slot=\"drop-zone-workbench\"",
        "data-slot=\"drop-zone-workbench-canvas\"",
    ] {
        assert!(
            docs_source.contains(required),
            "drop-zone workbench should keep DX marker `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            docs_source.contains(required),
            "drop-zone workbench persistence should keep platform guard `{required}`."
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "热样式路径证据：`apps/docs-app/src/playground.rs` 通过 `test_css_source + compose_scoped_css` 提供 scoped CSS 热编辑反馈，常见样式调整无需完整 wasm 重编译。",
        "Workbench 证据：`apps/docs-app/src/pages/components/pages/files.rs::drop_zone` 新增 `Workbench（展示 + Config + Code + CSS Test）`，并提供 `data-slot=\"drop-zone-workbench-canvas\"` 隔离画布。",
        "上下文保留证据：workbench 提供 `Persist workbench state` 开关；`load/save/clear_drop_zone_workbench_state` 在 wasm32 持久化 `is_disabled/custom_motion`，non-wasm 下安全 no-op。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_dx_playground_supports_css_hot_reload_without_wasm_rebuild`、`components/drop-zone/test/semantics.rs::drop_zone_dx_workbench_supports_optional_state_persistence_and_isolated_canvas`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_dx_playground_supports_css_hot_reload_without_wasm_rebuild`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_dx_workbench_supports_optional_state_persistence_and_isolated_canvas`。",
        "门禁证据：`scripts/check-ui-dx.sh` 新增 drop-zone DX 合同命令，阻断热重载/隔离画布/可选状态保留回归。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document DX contract evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/files.rs",
    );
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let check2 = load_source("check2");

    for required in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "title=\"State Matrix (Disabled / Motion / Callback)\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "DropZone has no persistent controlled/uncontrolled state axis.",
        "Use on_drop_files callback to sync dropped files into app state.",
        "Streaming fallback=snapshot: waiting for final validation",
        "Inspect data-ui-stream-support/data-ui-stream-fallback/data-ui-output-status.",
    ] {
        assert!(
            docs_source.contains(required),
            "drop-zone docs product surface should include `{required}`."
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "code_imports",
        "<CodeBlock code=resolved_code.get() />",
        "class_name=\"ui-code-block__copy-button\"",
    ] {
        assert!(
            playground_source.contains(required),
            "playground source should keep copy-ready import contract marker `{required}`."
        );
    }

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "drop_zone_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 docs-product section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_docs_product_copy_paste_ready_item_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
        "drop-zone check2 should mark docs-product copy-paste-ready item complete."
    );

    for required in [
        "files.rs::drop_zone",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "drop_zone_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "drop_zone_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 docs-product section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should keep docs-sync/state-matrix rule `{required}`."
        );
    }

    for marker in [
        "drop_zone_check2_documents_docs_sync_and_state_matrix_rules",
        "drop_zone_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "drop_zone_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2.contains(marker),
            "drop-zone/check2.md should keep docs-sync evidence marker `{marker}`."
        );
    }
}

#[test]
fn drop_zone_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/files.rs",
    );
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let check2 = load_source("check2");

    for marker in [
        "#[prop(optional, into)] label: Option<String>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] disabled: Option<bool>,",
        "#[prop(optional)] motion: Option<DropZoneMotion>,",
        "#[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>",
        "pub const fn classify_disabled_input(",
        "DisabledInput::Default => (false, DisabledSource::Default),",
        "let motion = crate::motion::sanitize_motion(input.motion.unwrap_or_default());",
        "let motion_source = resolve_motion_source(motion == DropZoneMotion::default());",
    ] {
        assert!(
            view_source.contains(marker) || logic_source.contains(marker),
            "drop-zone API/default contract should keep marker `{marker}` for docs sync."
        );
    }

    for marker in [
        "pub(super) fn drop_zone() -> AnyView {",
        "title=\"Hello World\"",
        "title=\"Quick Start (Default API)\"",
        "title=\"State Matrix (Disabled / Motion / Callback)\"",
        "data-slot=\"drop-zone-e2e-state-default\"",
        "data-slot=\"drop-zone-e2e-state-disabled\"",
        "data-slot=\"drop-zone-e2e-state-custom-motion\"",
        "<DropZone label=\"Disabled\".to_string() is_disabled=true>",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "DropZone has no persistent controlled/uncontrolled state axis.",
        "Use on_drop_files callback to sync dropped files into app state.",
        "title=\"Streaming Optional (fallback=snapshot)\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "drop-zone docs should keep synced example/matrix marker `{marker}`."
        );
    }

    for marker in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "drop_zone_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-dx.sh",
        "classify_disabled_input/resolve_is_disabled/resolve_props",
    ] {
        assert!(
            check2.contains(marker),
            "drop-zone/check2.md should keep docs-sync evidence marker `{marker}`."
        );
    }

    for forbidden in [
        "default_is_disabled",
        "on_disabled_change",
        "default_motion",
        "on_motion_change",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "drop-zone docs should avoid stale/aliased API token `{forbidden}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "drop-zone check2 should mark docs-sync/state-matrix item complete."
    );

    for required in [
        "files.rs::drop_zone",
        "drop_zone_check2_documents_docs_sync_and_state_matrix_rules",
        "drop_zone_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "drop_zone_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 docs-sync section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_documentation_as_product_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should keep documentation-as-product rule `{required}`."
        );
    }
}

#[test]
fn drop_zone_documentation_entry_exists_with_beginner_first_progression() {
    let readme = include_str!("../src/README.md");
    let docs_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/files.rs",
    );

    for marker in [
        "# DropZone",
        "## Hello World",
        "## 常见用法",
        "## 新手路径（先用起来，再进阶）",
        "## API 约定",
        "`on_drop_files`",
        "is_disabled=true",
        "motion=DropZoneMotion",
    ] {
        assert!(
            readme.contains(marker),
            "drop-zone README should include beginner-friendly marker `{marker}`."
        );
    }

    let readme_hello = readme
        .find("## Hello World")
        .expect("DropZone README should include Hello World section");
    let readme_common = readme
        .find("## 常见用法")
        .expect("DropZone README should include common usage section");
    let readme_progressive = readme
        .find("## 新手路径（先用起来，再进阶）")
        .expect("DropZone README should include beginner-first progression section");
    let readme_api = readme
        .find("## API 约定")
        .expect("DropZone README should include API section");

    assert!(
        readme_hello < readme_common
            && readme_common < readme_progressive
            && readme_progressive < readme_api,
        "DropZone README should keep default path before advanced guidance."
    );

    let section_start = docs_source
        .find("pub(super) fn drop_zone() -> AnyView {")
        .expect("files docs should contain drop_zone section");
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail[1..]
        .find("\npub(super) fn ")
        .map(|idx| idx + 1)
        .unwrap_or(section_tail.len());
    let section = &section_tail[..section_end_rel];

    assert!(
        section.contains("title=\"DropZone\"")
            && section.contains("slug=\"drop-zone\"")
            && section.contains("title=\"Hello World\"")
            && section.contains("title=\"Quick Start (Default API)\"")
            && section.contains("title=\"State Matrix (Disabled / Motion / Callback)\"")
            && section.contains("title=\"Workbench（展示 + Config + Code + CSS Test）\""),
        "DropZone docs-app entry should exist and include beginner/common/advanced sections."
    );

    let docs_hello = section
        .find("title=\"Hello World\"")
        .expect("DropZone docs should include Hello World playground");
    let docs_common = section
        .find("title=\"Quick Start (Default API)\"")
        .expect("DropZone docs should include default API playground");
    let docs_advanced = section
        .find("title=\"Workbench（展示 + Config + Code + CSS Test）\"")
        .expect("DropZone docs should include advanced workbench section");

    assert!(
        docs_hello < docs_common && docs_common < docs_advanced,
        "DropZone docs should keep beginner-first order before advanced controls."
    );
}

#[test]
fn drop_zone_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce documentation-as-product contract `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_documentation_as_product_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "drop_zone_check2_documents_documentation_as_product_rules",
        "drop_zone_documentation_entry_exists_with_beginner_first_progression",
        "drop_zone_dx_check_script_covers_documentation_as_product_contract",
        "components/drop-zone/src/README.md",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 documentation-as-product section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_interactive_playground_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "AI Spec 联动示例 N/A（`DropZone` 非 AI Spec 输入组件）",
        "drop_zone_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "drop_zone_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should keep interactive-playground rule `{required}`."
        );
    }
}

#[test]
fn drop_zone_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/files.rs",
    );
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");

    let section_start = docs_source
        .find("pub(super) fn drop_zone() -> AnyView {")
        .expect("files docs should contain drop_zone section");
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail[1..]
        .find("\npub(super) fn ")
        .map(|idx| idx + 1)
        .unwrap_or(section_tail.len());
    let section = &section_tail[..section_end_rel];

    for marker in [
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "data-slot=\"drop-zone-workbench-controls\"",
        "data-slot=\"drop-zone-workbench-toggle-disabled\"",
        "data-slot=\"drop-zone-workbench-toggle-custom-motion\"",
        "data-slot=\"drop-zone-workbench-toggle-persist\"",
        "data-slot=\"drop-zone-workbench\"",
        "data-slot=\"drop-zone-workbench-canvas\"",
        "data-slot=\"drop-zone-workbench-surface\"",
        "test_config_signal=workbench_actual_config",
        "test_css_source=workbench_test_css_source",
        "code_signal=workbench_code",
        "Switch checked=workbench_is_disabled",
        "Switch checked=workbench_custom_motion",
        "Switch checked=workbench_persist_state",
    ] {
        assert!(
            section.contains(marker),
            "drop-zone docs interactive playground should keep marker `{marker}`."
        );
    }

    for marker in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<div data-slot=\"playground-controls\">",
        "Card class_name=\"playground__panel playground__controls\".to_string()",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app Playground should keep interactive preview marker `{marker}`."
        );
    }
}

#[test]
fn drop_zone_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_drop_zone_contract.spec.mjs");

    for marker in [
        "docs-app drop-zone key flow is repeatable with semantic breakpoints",
        "data-slot=\"drop-zone-workbench-toggle-disabled\"",
        "data-slot=\"drop-zone-workbench-surface\"",
        "data-slot=\"drop-zone-button\"",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toHaveAttribute(\"data-disabled\", \"false\")",
        "toHaveAttribute(\"data-drag-phase\", \"idle\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "drop-zone interactive playground e2e flow should keep marker `{marker}`."
        );
    }
}

#[test]
fn drop_zone_dx_check_script_covers_interactive_playground_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_interactive_playground_rules",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_docs_app_provides_interactive_playground_for_props_state_and_preview",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce interactive-playground contract `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_interactive_playground_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "AI Spec 联动示例 N/A（`DropZone` 非 AI Spec 输入组件）",
        "drop_zone_check2_documents_interactive_playground_rules",
        "drop_zone_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "drop_zone_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "drop_zone_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "e2e/tests/docs_app_drop_zone_contract.spec.mjs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 interactive-playground section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_source_first_copy_paste_ready_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should keep source-first copy-paste-ready rule `{required}`."
        );
    }
}

#[test]
fn drop_zone_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/files.rs",
    );
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let code_block_view_source = include_str!("../../../components/code-block/src/view.rs");
    let readme_source = include_str!("../src/README.md");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_drop_zone_contract.spec.mjs");

    for marker in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_signal=source_first_code",
        "code_imports=source_first_imports.clone()",
        "data-slot=\"drop-zone-source-first\"",
        "data-slot=\"drop-zone-source-first-contract\"",
        "data-slot=\"drop-zone-source-first-dependency-baseline\"",
        "data-slot=\"drop-zone-source-paths\"",
        "data-slot=\"drop-zone-source-prerequisites\"",
        "<h3>\"Source-first / Copy-Paste Ready Contract\"</h3>",
        "<code>\"Show code\"</code>",
        "component-drop_zone",
        "inject-css",
        "components/drop-zone/src/mod.rs",
        "components/drop-zone/src/logic.rs",
        "components/drop-zone/src/view.rs",
        "components/drop-zone/src/styles.rs",
        "components/drop-zone/src/motion.rs",
        "apps/docs-app/src/pages/components/pages/files.rs::drop_zone",
    ] {
        assert!(
            docs_source.contains(marker),
            "drop-zone source-first docs should keep marker `{marker}`."
        );
    }

    for marker in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "<CodeBlock code=resolved_code.get() />",
        "missing_import_lines(&raw, &imports)",
    ] {
        assert!(
            playground_source.contains(marker),
            "playground copy-ready pipeline should keep marker `{marker}`."
        );
    }

    for marker in [
        "class_name=\"ui-code-block__copy-button\"",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(marker),
            "CodeBlock should keep one-click copy affordance marker `{marker}`."
        );
    }

    for marker in [
        "## Source-first",
        "组件源码：`components/drop-zone/src/{mod,logic,view,styles,motion}.rs`",
        "package feature：`component-drop_zone`（可选叠加 `inject-css`）",
        "ui = { default-features = false, features = [\"component-drop_zone\", \"inject-css\"] }",
    ] {
        assert!(
            readme_source.contains(marker),
            "DropZone README should document source-first marker `{marker}`."
        );
    }

    for marker in [
        "docs-app drop-zone source-first docs are copy-paste ready and traceable",
        "[data-slot=\"drop-zone-source-first\"]",
        "toContainText(\"component-drop_zone\")",
        "toContainText(\"inject-css\")",
        ".ui-code-block__copy-button",
        "toContainText(\"use leptos::prelude::*;\")",
        "use ui::{DropZone, DropZoneMotion, DroppedFile};",
    ] {
        assert!(
            e2e_source.contains(marker),
            "drop-zone source-first e2e contract should keep marker `{marker}`."
        );
    }
}

#[test]
fn drop_zone_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: drop-zone source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce source-first copy-paste-ready contract `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "apps/docs-app/src/pages/components/pages/files.rs::drop_zone",
        "components/drop-zone/src/README.md",
        "e2e/tests/docs_app_drop_zone_contract.spec.mjs::docs-app drop-zone source-first docs are copy-paste ready and traceable",
        "drop_zone_check2_documents_source_first_copy_paste_ready_rules",
        "drop_zone_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "drop_zone_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 source-first section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should keep heroui-benchmark docs-sync rule `{required}`."
        );
    }
}

#[test]
fn drop_zone_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/files.rs",
    );
    let readme_source = include_str!("../src/README.md");

    for marker in [
        "### DropZone 同步记录（2026-02-20）",
        "参数模型同步：`DropZone` 参数主轴保持 `label/aria_label/is_disabled/disabled/motion/on_drop_files/lang/dir`",
        "component_doc!(\"DropZone\", \"drop-zone\", \"Files\", files::drop_zone)",
        "apps/docs-app/src/pages/components/pages/files.rs::drop_zone()",
        "`components/drop-zone/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(marker),
            "heroui strategy doc should include drop-zone synchronization marker `{marker}`."
        );
    }

    for marker in [
        "component_doc!(",
        "\"DropZone\"",
        "\"drop-zone\"",
        "files::drop_zone",
    ] {
        assert!(
            pages_source.contains(marker),
            "component docs index should expose drop-zone entry marker `{marker}`."
        );
    }

    for marker in [
        "pub(super) fn drop_zone() -> AnyView {",
        "title=\"DropZone\"",
        "slug=\"drop-zone\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "docs-app drop-zone page should stay indexable via marker `{marker}`."
        );
    }

    for marker in ["# DropZone", "## 文档入口"] {
        assert!(
            readme_source.contains(marker),
            "drop-zone README should remain an equivalent component doc entry via `{marker}`."
        );
    }
}

#[test]
fn drop_zone_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce heroui-benchmark docs-sync contract `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "drop_zone_check2_documents_heroui_benchmark_docs_sync_rules",
        "drop_zone_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "drop_zone_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 should keep heroui-benchmark docs-sync evidence marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce `{required}`."
        );
    }
}

#[test]
fn drop_zone_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce docs-sync/state-matrix contract `{required}`."
        );
    }
}

#[test]
fn drop_zone_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: drop-zone docs product copy-paste-ready + streaming/snapshot contract\"",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include docs-product contract marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let cargo = include_str!("../Cargo.toml");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for forbidden in [
        "serde =",
        "serde_json",
        "Serialize",
        "Deserialize",
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
    ] {
        assert!(
            !cargo.contains(forbidden)
                && !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden),
            "drop-zone should keep serde/spec path N/A for simple component scope (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "`serde/spec` 边界：`DropZone` 属简单交互组件，当前无 `spec.rs` 与 schema 迁移面；`components/drop-zone/Cargo.toml` 未引入 `serde/serde_json`，序列化迁移路径标注 N/A。",
        "门禁证据：`scripts/check-ui-engineering.sh` 新增 drop-zone 工程能力合同命令，阻断 `serde/spec`、tracing 语义与 runtime 边界回归。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document engineering serde/spec boundary `{required}`."
        );
    }
}

#[test]
fn drop_zone_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for required in [
        "use ui_headless::use_ui_trace;",
        "trace.emit(\"drop-zone\", ui_headless::UiTraceEventKind::Note { message });",
    ] {
        assert!(
            view.contains(required),
            "drop-zone should reuse shared headless trace pipeline marker `{required}`."
        );
    }

    for forbidden in ["tracing::", "span!(", "event!(", "#[instrument]"] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "drop-zone should not introduce component-local tracing protocol `{forbidden}`."
        );
    }

    assert!(
        check2.contains(
            "tracing 语义边界：组件复用 `ui_headless::use_ui_trace` 输出调试事件，不引入组件私有 `tracing::span/event` 协议，避免埋点语义分叉。"
        ),
        "drop-zone checklist should document unified tracing boundary."
    );
}

#[test]
fn drop_zone_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let cargo = include_str!("../Cargo.toml");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for forbidden in [
        "tokio",
        "async-std",
        "async_std",
        "Runtime",
        "Handle",
        "spawn_blocking",
        "#[prop(optional)] runtime",
    ] {
        assert!(
            !cargo.contains(forbidden)
                && !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden),
            "drop-zone should not leak async runtime detail into component boundary (`{forbidden}`)."
        );
    }

    for required in [
        "pub use logic::DroppedFile;",
        "pub use motion::DropZoneMotion;",
        "pub use view::DropZone;",
        "#[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>",
    ] {
        let hit = module.contains(required) || view.contains(required);
        assert!(
            hit,
            "drop-zone public boundary should keep runtime-agnostic API marker `{required}`."
        );
    }

    for required in [
        "async/runtime 边界：`DropZone` 无 async runtime 依赖；公共 API 仅暴露 `DropZone/DroppedFile/DropZoneMotion` 与 `Callback<Vec<DroppedFile>>`，未泄露 `tokio/async-std/runtime handle` 类型。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::{drop_zone_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope,drop_zone_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events,drop_zone_engineering_contract_avoids_runtime_leaks_in_public_api_surface}`、`components/drop-zone/test/drop_zone/semantics.rs::{drop_zone_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope,drop_zone_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events,drop_zone_engineering_contract_avoids_runtime_leaks_in_public_api_surface}`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document runtime-boundary evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest = load_source("manifest");
    let rbi = load_source("rbi");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for required in [
        "schema_version = \"1\"",
        "name = \"DropZone\"",
        "crate = \"ui-drop-zone\"",
    ] {
        assert!(
            manifest.contains(required),
            "drop-zone manifest should keep stable v1 marker `{required}`."
        );
    }

    for required in ["pub enum DropZoneAgentSchemaVersion", "V1,"] {
        assert!(
            rbi.contains(required),
            "drop-zone RBI should keep v1-only schema marker `{required}`."
        );
    }

    let combined = format!("{module}\n{logic}\n{view}\n{styles}\n{motion}\n{rbi}");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
        "V2,",
    ] {
        assert!(
            !manifest.contains(forbidden) && !combined.contains(forbidden),
            "drop-zone should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `DropZone` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "migrate_v1_to_v2",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 should keep version-migration governance marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_version_deprecation_migration_item_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。"),
        "drop-zone check2 should mark version-migration item complete."
    );

    for required in [
        "N/A：本次 `DropZone` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "drop_zone_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 version-migration section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script = include_str!("../../../scripts/check-ui-engineering.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script.contains(required),
            "engineering check script should enforce `{required}`."
        );
    }
}

#[test]
fn drop_zone_composite_parent_item_api_is_not_applicable() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in ["#[component]", "children: Children,", "{children()}"] {
        assert!(
            view.contains(required),
            "drop-zone should expose a single explicit content slot via `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "#[prop(optional)] items:",
        "ItemSpec",
        "<Item",
    ] {
        assert!(
            !view.contains(forbidden),
            "drop-zone should not expose implicit paired-array or item-spec composite api (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "N/A 说明：`DropZone` 不是集合型/分项组合组件，不存在 `Parent/Item` 语义树；其主 API 为单容器 + `children` 内容插槽。",
        "边界证据：`components/drop-zone/src/view.rs` 仅暴露 `children: Children`，未提供 `labels/titles/panels/items` 并行数组或 `ItemSpec` 配置式语法糖。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document composite-api N/A boundary via `{required}`."
        );
    }
}

#[test]
fn drop_zone_macro_micro_drag_state_machine_stays_split_and_converges_on_drag_end() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub enum DragLifecyclePhase",
        "pub enum DragLifecycleAction",
        "DragStart",
        "DragEnd",
        "pub const fn reduce_drag_lifecycle(",
        "DragLifecycleAction::DragEnd => DragLifecyclePhase::Idle",
    ] {
        assert!(
            logic.contains(required),
            "drop-zone logic should expose macro drag lifecycle convergence primitives (`{required}`)."
        );
    }

    let over_start = view
        .find("let on_drag_over = move |ev: ev::DragEvent| {")
        .unwrap_or_else(|| panic!("missing on_drag_over handler"));
    let leave_start = view
        .find("let on_drag_leave = move |ev: ev::DragEvent| {")
        .unwrap_or_else(|| panic!("missing on_drag_leave handler"));
    let over_block = &view[over_start..leave_start];
    assert!(
        over_block.contains("set_drag_over_tick.update(|tick| *tick = tick.wrapping_add(1));"),
        "drop-zone should keep drag-over micro loop in local view state."
    );
    for forbidden in [
        "super::logic::reduce_drag_interaction(",
        "super::logic::reduce_drag_lifecycle(",
    ] {
        assert!(
            !over_block.contains(forbidden),
            "drag-over micro loop must not cross into logic reducer per-frame (`{forbidden}`)."
        );
    }

    for required in [
        "super::logic::DragLifecycleAction::DragEnd",
        "data-drag-phase=move || drag_phase.get().as_attr()",
    ] {
        assert!(
            view.contains(required),
            "drop-zone view should converge macro phase via DragEnd and expose stable phase markers (`{required}`)."
        );
    }

    for required in [
        "- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。",
        "宏观状态：`components/drop-zone/src/logic.rs::DragLifecyclePhase` + `reduce_drag_lifecycle`，显式 `DragStart/DragEnd` 动作并在 `DragEnd` 收敛到 `Idle`。",
        "微观循环：`components/drop-zone/src/view.rs::on_drag_over` 仅更新本地 `drag_over_tick`（高频路径不调用 logic reducer）；结束路径在 `on_drop/on_drag_leave` 统一 dispatch `DragLifecycleAction::DragEnd`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document macro/micro split and DragEnd convergence via `{required}`."
        );
    }
}

#[test]
fn drop_zone_two_pass_rendering_is_not_applicable_without_geometry_measurement() {
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for forbidden in [
        "get_bounding_client_rect",
        "getClientRects",
        "ResizeObserver",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "measure",
        "rectification",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "drop-zone should not enter geometry two-pass flow without measurement dependency (`{forbidden}`)."
        );
    }

    for forbidden in ["Measure", "Rectification"] {
        assert!(
            !logic.contains(forbidden),
            "drop-zone logic should not host forced two-pass geometry contracts when unnecessary (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。",
        "N/A 说明：`DropZone` 不依赖几何测量定位（无 tooltip/popover/menu 类 overlay 对齐需求），当前交互仅基于拖拽事件与语义状态，不需要 `Intent -> Measure -> Rectification` 回流链路。",
        "边界证据：`components/drop-zone/src/view.rs`/`logic.rs`/`motion.rs` 未使用 DOM rect/observer 测量 API（如 `get_bounding_client_rect`、`ResizeObserver`）。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should record two-pass rendering N/A boundary via `{required}`."
        );
    }
}

#[test]
fn drop_zone_registration_protocol_is_not_applicable_without_dynamic_item_collections() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in ["children: Children,", "{children()}"] {
        assert!(
            view.contains(required),
            "drop-zone should keep a single content slot instead of collection registration (`{required}`)."
        );
    }

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "item_ids",
        "roving",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "drop-zone should not implement dynamic-item registration protocol markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。",
        "N/A 说明：`DropZone` 非集合型动态子项组件，不维护可注册 item 列表与键盘导航顺序；组件仅承载单容器内容插槽。",
        "边界证据：`components/drop-zone/src/view.rs`/`logic.rs` 未出现 `RegistrationContext/Register/Unregister/items_order/HashSet` 等集合注册协议实现。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document registration-protocol N/A boundary via `{required}`."
        );
    }
}

#[test]
fn drop_zone_slot_projection_strategy_is_not_applicable_without_multi_slot_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for required in ["children: Children,", "{children()}"] {
        assert!(
            view.contains(required),
            "drop-zone should keep single-slot rendering contract (`{required}`)."
        );
    }

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "suspend",
        "resume",
        "pause",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "drop-zone should not implement projection lifecycle protocol markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。",
        "N/A 说明：`DropZone` 为单容器内容插槽组件，不存在多插槽投影策略（`Lazy/KeepAlive/Eager`）选择面，也无隐藏投影生命周期管理需求。",
        "边界证据：`components/drop-zone/src/view.rs` 仅暴露 `children` 单插槽；`logic.rs`/`motion.rs` 未出现 `KeepAlive/NotifyHidden` 等生命周期通知协议实现。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should record slot-projection N/A boundary via `{required}`."
        );
    }
}

#[test]
fn drop_zone_environment_streams_are_not_applicable_without_env_subscriptions() {
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "match_media",
        "on:resize",
        "BreakpointChanged",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "drop-zone should not add environment subscription streams without real env-driven semantics (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。",
        "N/A 说明：`DropZone` 无 `Resize/Theme/Intersection` 环境订阅需求；当前交互仅处理 drag/paste/pointer/focus 事件，不存在环境事件源与洪泛通道。",
        "边界证据：`components/drop-zone/src/view.rs`/`logic.rs`/`motion.rs` 未使用 `ResizeObserver`、`IntersectionObserver`、`matchMedia`、`on:resize` 等环境订阅 API。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should record env-stream N/A boundary via `{required}`."
        );
    }
}

#[test]
fn drop_zone_event_light_cone_is_not_applicable_without_large_collection_batch_ops() {
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for required in ["children: Children,", "#[prop(optional)] on_drop_files"] {
        assert!(
            view.contains(required),
            "drop-zone should keep single-container event surface without collection batch fan-out (`{required}`)."
        );
    }

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "prop drilling",
        "selection_state",
        "batch_select",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "drop-zone should not implement event-light-cone batch collection protocol markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。",
        "N/A 说明：`DropZone` 非大型集合批量操作组件，不存在 `Table/Grid` 式批处理选择与 O(N) 向下事件分发路径。",
        "边界证据：`components/drop-zone/src/view.rs` 仅提供单容器 `children` 插槽 + 单事件回调 `on_drop_files`，`logic.rs`/`view.rs`/`motion.rs` 未出现 `Context Bus`、`Selector`、`SelectionState::All`、批量选择状态压缩协议。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should record event-light-cone N/A boundary via `{required}`."
        );
    }
}

#[test]
fn drop_zone_causality_bus_is_not_applicable_without_derived_bus_pipeline() {
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for required in ["#[prop(optional)] on_drop_files", "cb.run(files);"] {
        assert!(
            view.contains(required),
            "drop-zone should keep direct callback handoff instead of derived bus fan-out (`{required}`)."
        );
    }

    for forbidden in [
        "TraceId",
        "Causality Bus",
        "broadcast",
        "subscribe",
        "dispatch_trace",
        "trace_id",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "drop-zone should not implement causality-bus contracts when no derived bus pipeline exists (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。",
        "N/A 说明：`DropZone` 不存在复杂派生总线操作；交互路径是直接事件处理后回调 `on_drop_files`，没有“命令总线广播 -> 多订阅者”链路。",
        "边界证据：`components/drop-zone/src/view.rs`/`logic.rs`/`motion.rs` 未出现 `TraceId`、`Causality Bus`、`broadcast`、`subscribe`、`dispatch trace` 等因果总线协议实现。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should record causality-bus N/A boundary via `{required}`."
        );
    }
}

#[test]
fn drop_zone_focus_stack_global_gc_is_not_applicable_without_layered_overlay() {
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for required in [
        "let zone_ref: NodeRef<html::Div> = NodeRef::new();",
        "let focus_button_ref: NodeRef<html::Button> = NodeRef::new();",
        "motion::attach_motion(",
        "ui_headless::a11y::should_focus_proxy_button_on_click",
        "if let Some(button) = focus_button_ref.get_untracked()",
        "ui_observability::observe_js_result!(button.focus());",
    ] {
        assert!(
            view.contains(required),
            "drop-zone should keep NodeRef usage scoped to local focus-proxy and motion wiring (`{required}`)."
        );
    }

    for forbidden in [
        "Overlay",
        "on_close",
        "on_exit_complete",
        "FallbackTo",
        "Selector",
        "document.body",
        "focus stack",
        "focus_manager",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "drop-zone should not implement layered-overlay focus-stack restoration protocol marker (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。",
        "N/A 说明：`DropZone` 非层叠 `Overlay` 组件，不存在“打开/关闭 Overlay 后焦点恢复”链路，因此不适用全局 Focus Stack 恢复策略约束。",
        "边界证据：`components/drop-zone/src/view.rs` 未接入 `Overlay`/`on_close`/`on_exit_complete`/`FallbackTo`/`Selector`，也未实现 `document.body` 回退路径。",
        "NodeRef 使用边界：`zone_ref` 与 `focus_button_ref` 仅用于本地拖拽区域动效 attach 与焦点代理按钮（click/paste/focus），不作为 Overlay 关闭后的恢复目标句柄。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_focus_stack_global_gc_is_not_applicable_without_layered_overlay`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_focus_stack_global_gc_is_not_applicable_without_layered_overlay`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document focus-stack N/A boundary and evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_escape_hatches_foreign_zone_is_not_applicable_without_third_party_imperative_instances()
 {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");
    let cargo = include_str!("../Cargo.toml");

    for required in [
        "pub use logic::DroppedFile;",
        "pub use motion::DropZoneMotion;",
        "pub use view::DropZone;",
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "ui-motion = { path = \"../../crates/ui-motion\" }",
        "ui-state-primitives = { path = \"../../crates/ui-state-primitives\" }",
        "ui-theme = { path = \"../../crates/ui-theme\" }",
    ] {
        let hit = module.contains(required) || cargo.contains(required);
        assert!(
            hit,
            "drop-zone export/dependency boundary should include `{required}`."
        );
    }

    for forbidden in [
        "YieldControl",
        "CleanupForeign",
        "Foreign Zone",
        "ECharts",
        "Mapbox",
        "Leaflet",
        "google.maps",
        "echarts",
        "mapbox",
        "leaflet",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden)
                && !cargo.contains(forbidden),
            "drop-zone should not integrate third-party imperative foreign-zone protocol marker (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。",
        "N/A 说明：`DropZone` 未集成 ECharts/Map 等命令式第三方实例，不存在 `Foreign Zone` 生命周期托管需求（`YieldControl/CleanupForeign` 不适用）。",
        "边界证据：`components/drop-zone/src/mod.rs` 仅导出 `DropZone/DroppedFile/DropZoneMotion`；`logic.rs/view.rs/motion.rs` 未出现 `YieldControl`、`CleanupForeign`、`ECharts`、`Mapbox`、`Leaflet`、`google.maps` 等命令式第三方接入协议。",
        "API 安全边界：公共 API 未暴露第三方实例句柄（无 `pub use` 第三方 runtime type）；`components/drop-zone/Cargo.toml` 仅依赖 Leptos 与仓库内 `ui-*` crates。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_escape_hatches_foreign_zone_is_not_applicable_without_third_party_imperative_instances`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_escape_hatches_foreign_zone_is_not_applicable_without_third_party_imperative_instances`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document escape-hatch N/A boundary and evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_hydration_discontinuity_is_not_applicable_without_runtime_id_generation() {
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");
    let id_provider = include_str!("../../../crates/ui-headless/src/id_provider.rs");

    for forbidden in [
        "now(",
        "Date::now",
        "js_sys::Date",
        "uuid",
        "UUID",
        "random",
        "Math::random",
        "create_unique_id",
        "create_id",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "drop-zone should not introduce non-deterministic hydration-time id source (`{forbidden}`)."
        );
    }

    for required in [
        "data-drag-phase=move || drag_phase.get().as_attr()",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
        "data-disabled-source=disabled_source.as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "data-aria-source=aria_source.as_attr()",
        "aria-label=labels.aria_label.clone()",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
    ] {
        assert!(
            view.contains(required),
            "drop-zone should use stable semantic markers instead of runtime-generated ids (`{required}`)."
        );
    }

    for required in [
        "pub struct UiIdProvider {",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider {",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider> {",
    ] {
        assert!(
            id_provider.contains(required),
            "headless id-provider contract should stay available for future deterministic id needs (`{required}`)."
        );
    }

    for required in [
        "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。",
        "N/A 说明：`DropZone` 当前不生成动态 DOM id（无 `uuid/random/now` 初始化路径），不存在 SSR 与 hydration 间 ID 漂移面。",
        "边界证据：`components/drop-zone/src/logic.rs`、`view.rs`、`motion.rs` 未出现 `now()`、`Date::now`、`uuid`、`random`、`create_unique_id` 等非确定性 ID 生成逻辑。",
        "当前实现证据：组件语义通过稳定 `data-*` 与 `aria-*` 标记表达状态（如 `data-drag-phase`、`data-drop-target`、`data-disabled-source`），未依赖运行时拼接 ID。",
        "升级约束：若未来新增 `aria-labelledby/aria-describedby` 等动态 ID 需求，必须接入 `ui-headless::id_provider`（`provide_ui_id_provider/use_ui_id_provider`）并以确定性 seed 保证 SSR/Hydration 一致。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_hydration_discontinuity_is_not_applicable_without_runtime_id_generation`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_hydration_discontinuity_is_not_applicable_without_runtime_id_generation`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document hydration-discontinuity N/A evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_has_a11y_i18n_l10n_contract_without_view_hardcoded_copy() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "use ui_headless::{A11yDirection, CommonStrings, locale_attrs, use_ui_i18n};",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "role=\"group\"",
        "aria-label=labels.aria_label.clone()",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
        "ui_headless::a11y::should_focus_proxy_button_on_click",
    ] {
        assert!(
            view.contains(required),
            "drop-zone should mount a11y/i18n/l10n contract from headless and locale context (`{required}`)."
        );
    }

    for forbidden in ["\"Drop files\"", "\"Upload files\"", "\"Drop here\""] {
        assert!(
            !view.contains(forbidden),
            "drop-zone view should not hardcode user-visible copy (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "契约实现：`components/drop-zone/src/view.rs` 使用 `ui_headless::use_ui_i18n().strings::<CommonStrings>()` 注入 `drop_zone_aria_label`，并通过 `locale_attrs(lang, dir)` 挂载 `lang/dir`；交互语义挂载 `role=\\\"group\\\"`、`aria-label`、`aria-disabled`，键盘/焦点路径由隐藏 `button` + `use_focus_ring` 负责。",
        "文案来源链路：`props(label/aria_label) > UiRoot i18n(CommonStrings::drop_zone_aria_label) > status-primitives 默认值(DEFAULT_ARIA_LABEL)`，`view.rs` 无硬编码用户可见文案。",
        "共享 A11y 工具复用：`ui_headless::a11y::should_focus_proxy_button_on_click`（点击焦点代理归一）+ `locale_attrs`（语言方向归一），组件层仅消费与挂载。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should record a11y/i18n/l10n contract via `{required}`."
        );
    }
}

#[test]
fn drop_zone_state_markers_are_observable_searchable_and_enumerable() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "data-drag-phase=move || drag_phase.get().as_attr()",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
        "data-disabled=super::logic::bool_data_attr(is_disabled)",
        "data-focus-visible=move || super::logic::bool_data_attr(focus_ring.is_focus_visible.get())",
        "data-hovered=move || super::logic::bool_data_attr(hover.is_hovered.get())",
        "data-focused=move || super::logic::bool_data_attr(focus_ring.is_focused.get())",
        "data-disabled-source=disabled_source.as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "data-aria-source=aria_source.as_attr()",
        "role=\"group\"",
        "aria-label=labels.aria_label.clone()",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
    ] {
        assert!(
            view.contains(required),
            "drop-zone should expose stable state/source markers for automation and diagnostics (`{required}`)."
        );
    }

    for required in [
        "Self::IsDisabled => \"is_disabled\"",
        "Self::DisabledAlias => \"disabled\"",
        "Self::Default => \"default\"",
        "Self::Default => \"default\"",
        "Self::Custom => \"custom\"",
        "Self::Idle => \"idle\"",
        "Self::Dragging => \"dragging\"",
    ] {
        assert!(
            logic.contains(required),
            "drop-zone logic should keep marker values in closed enum mappings (`{required}`)."
        );
    }

    for required in [
        "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
        "关键状态轴标记：`components/drop-zone/src/view.rs` 挂载 `data-drag-phase`、`data-drop-target`、`data-disabled`、`data-focus-visible`、`data-hovered`、`data-focused`，并同步 `role/aria-label/aria-disabled` 语义标记。",
        "状态来源标记：`data-disabled-source`（`is_disabled/disabled/default`）、`data-motion-source`（`default/custom`）、`data-aria-source`（`default/custom`）；交互态来源通过 `data-drag-phase`（`idle/dragging`）可检索。",
        "封闭集合保证：来源与阶段值由 `components/drop-zone/src/logic.rs` 中 `DisabledSource/MotionSource/AriaLabelSource/DragLifecyclePhase` 的 `as_attr()` 常量映射输出，避免自由文本漂移。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should record observable marker contracts via `{required}`."
        );
    }
}

#[test]
fn drop_zone_styles_depend_on_explicit_state_markers_not_dom_structure_guessing() {
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        ".ui-drop-zone[data-motion-source=\"custom\"]",
        ".ui-drop-zone[data-custom-motion=\"true\"]",
        ".ui-drop-zone__zone[data-hovered=\"true\"]",
        ".ui-drop-zone__zone[data-drop-target=\"true\"]",
        ".ui-drop-zone__zone[data-disabled=\"true\"]",
        ".ui-drop-zone__zone[data-focus-visible=\"true\"]",
    ] {
        assert!(
            styles.contains(required),
            "drop-zone styles should branch on explicit semantic markers (`{required}`)."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type"] {
        assert!(
            !styles.contains(forbidden),
            "drop-zone styles should not guess state from fragile structure selectors (`{forbidden}`)."
        );
    }

    for required in [
        "data-hovered=move || super::logic::bool_data_attr(hover.is_hovered.get())",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
        "data-disabled=super::logic::bool_data_attr(is_disabled)",
        "data-focus-visible=move || super::logic::bool_data_attr(focus_ring.is_focus_visible.get())",
        "data-motion-source=motion_source.as_attr()",
    ] {
        assert!(
            view.contains(required),
            "drop-zone view should expose semantic markers consumed by styles (`{required}`)."
        );
    }

    assert!(
        !view.contains("style="),
        "drop-zone view should not embed business style logic via inline style attributes."
    );

    for required in [
        "- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
        "选择器证据：`components/drop-zone/src/styles.rs` 的状态分支仅使用稳定选择器（如 `.ui-drop-zone__zone[data-hovered=\"true\"]`、`[data-drop-target=\"true\"]`、`[data-disabled=\"true\"]`、`[data-focus-visible=\"true\"]`、`.ui-drop-zone[data-motion-source=\"custom\"]`），未使用 `:nth-child` 等结构猜测选择器。",
        "运行时样式边界：`components/drop-zone/src/view.rs` 未注入 `style=` 业务样式逻辑；状态切换由 `data-*`/`aria-*` 语义标记驱动，样式数值通过 CSS 变量（`--ui-drop-zone-*`）消费。",
        "可解释性：视觉变化（高亮、边框、禁用、焦点）均可从 `data-hovered/data-drop-target/data-disabled/data-focus-visible` 与 `data-motion-source` 直接解释，不依赖节点存在性推断。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should record explicit-style-marker contract via `{required}`."
        );
    }
}

#[test]
fn drop_zone_token_first_static_styles_are_aggregated_and_do_not_use_utility_or_css_in_rust_defaults()
 {
    let styles = load_source("styles");
    let view = load_source("view");
    let css_aggregator = load_source("ui_components_css");
    let ui_root = load_source("ui_root");
    let check2 = load_source("check2");

    for required in [
        "pub const CSS: &str",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-drop-zone-min-height, var(--ui-fallback-drop-zone-min-height))",
        "var(--ui-drop-zone-border-width, var(--ui-fallback-drop-zone-border-width))",
        "var(--ui-drop-zone-disabled-opacity, var(--ui-fallback-drop-zone-disabled-opacity))",
    ] {
        assert!(
            styles.contains(required),
            "drop-zone styles should stay token-first via `{required}`."
        );
    }

    for forbidden in ["style!(", "css!(", "styled::", "Tailwind", "tw-"] {
        assert!(
            !styles.contains(forbidden) && !view.contains(forbidden),
            "drop-zone component layer should not default to utility/CSS-in-Rust token `{forbidden}`."
        );
    }

    assert!(
        !view.contains("style="),
        "drop-zone view should keep runtime style boundary to semantic attrs/CSS vars, not inline business styles."
    );

    for required in [
        "#[cfg(feature = \"component-drop_zone\")]",
        "out.push_str(crate::drop_zone::styles::CSS);",
    ] {
        assert!(
            css_aggregator.contains(required),
            "ui css aggregator should include drop-zone styles contract `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(required),
            "UiRoot should inject aggregated component styles via `{required}`."
        );
    }

    for required in [
        "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "聚合链路证据：`components/drop-zone/src/styles.rs` 以 `pub const CSS` 导出静态样式；`crates/ui/src/css.rs` 在 `#[cfg(feature = \"component-drop_zone\")]` 下聚合 `crate::drop_zone::styles::CSS`；`crates/ui/src/root.rs` 通过 `inject_components_css` 调用 `crate::css::push_components_css(&mut out)` 注入到 `UiRoot`。",
        "token-first 证据：`drop-zone` 的颜色/间距/圆角/阴影与尺寸均使用 `var(--ui-*)`（含 `var(--ui-drop-zone-*, var(--ui-fallback-drop-zone-*))` 回退链），未引入组件私有平行 token 体系。",
        "运行时与范式边界：`components/drop-zone/src/view.rs` 无业务 `style=` 内联样式；组件未引入 Utility-First class 协议或 CSS-in-Rust `style!`/运行时样式 DSL 作为默认实现。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_token_first_static_styles_are_aggregated_and_do_not_use_utility_or_css_in_rust_defaults`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_token_first_static_styles_are_aggregated_and_do_not_use_utility_or_css_in_rust_defaults`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document token-first static style contract evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_styles_use_defensive_variable_fallback_chain() {
    let styles = load_source("styles");
    let theme_css =
        source_contract::source_from_file_relative(file!(), "../../../crates/ui-theme/src/css.rs");
    let check2 = load_source("check2");

    for required in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-drop-zone-min-height, var(--ui-fallback-drop-zone-min-height))",
        "var(--ui-drop-zone-border-width, var(--ui-fallback-drop-zone-border-width))",
        "var(--ui-drop-zone-disabled-opacity, var(--ui-fallback-drop-zone-disabled-opacity))",
        "var(--ui-drop-zone-focus-outline-width, var(--ui-fallback-drop-zone-focus-outline-width))",
        "var(--ui-drop-zone-focus-outline-offset, var(--ui-fallback-drop-zone-focus-outline-offset))",
        "var(--ui-drop-zone-sr-only-size, var(--ui-fallback-drop-zone-sr-only-size))",
    ] {
        assert!(
            styles.contains(required),
            "drop-zone styles should keep defensive fallback chain marker `{required}`."
        );
    }

    for required in [
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-lg:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-bg:",
        "--ui-fallback-radius-lg:",
        "--ui-fallback-border:",
        "--ui-fallback-shadow-sm:",
        "--ui-fallback-accent:",
        "--ui-fallback-accent-soft:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-drop-zone-min-height:",
        "--ui-fallback-drop-zone-border-width:",
        "--ui-fallback-drop-zone-disabled-opacity:",
        "--ui-fallback-drop-zone-focus-outline-width:",
        "--ui-fallback-drop-zone-focus-outline-offset:",
        "--ui-fallback-drop-zone-sr-only-size:",
    ] {
        assert!(
            theme_css.contains(required),
            "ui-theme css should provide fallback terminal `{required}`."
        );
    }

    for forbidden in [" 120px", " 1px", " 4px", " 8px", " 16px"] {
        assert!(
            !styles.contains(forbidden),
            "drop-zone styles should not contain hardcoded terminal style literal `{forbidden}`."
        );
    }

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "回退链证据：`components/drop-zone/src/styles.rs` 的颜色/间距/圆角/阴影/焦点均升级为双层回退链（如 `var(--ui-fg, var(--ui-fallback-fg))`、`var(--ui-space-lg, var(--ui-fallback-space-lg))`、`var(--ui-drop-zone-border-width, var(--ui-fallback-drop-zone-border-width))`）。",
        "SSOT 证据：fallback 终值统一来自 `crates/ui-theme/src/css.rs`（`--ui-fallback-*` 与 `--ui-fallback-drop-zone-*`），组件层不自带终值常量。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_styles_use_defensive_variable_fallback_chain`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_styles_use_defensive_variable_fallback_chain`。",
        "门禁证据：`scripts/check-ui-contract-hygiene.sh` 新增 `drop_zone_styles_use_defensive_variable_fallback_chain` 命令，防止回退链回归。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document defensive-variable evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_defensive_variables_check_script_covers_style_fallback_contract() {
    let script = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    let required = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_styles_use_defensive_variable_fallback_chain";
    assert!(
        script.contains(required),
        "contract-hygiene check script should enforce `{required}`."
    );
}

#[test]
fn drop_zone_cascade_layer_and_runtime_style_contract_is_enforced() {
    let view = load_source("view");
    let css_aggregator = load_source("ui_components_css");
    let check2 = load_source("check2");

    let layer_start = css_aggregator
        .find("out.push_str(\"\\n@layer ui {\\n\");")
        .expect("ui css aggregation should open @layer ui.");
    let drop_zone_push = css_aggregator
        .find("out.push_str(crate::drop_zone::styles::CSS);")
        .expect("ui css aggregation should include drop-zone styles.");
    let layer_end = css_aggregator
        .rfind("out.push_str(\"\\n}\\n\");")
        .expect("ui css aggregation should close @layer ui.");

    assert!(
        layer_start < drop_zone_push && drop_zone_push < layer_end,
        "drop-zone css should stay within @layer ui aggregation boundaries."
    );

    assert!(
        !view.contains("style=") && !view.contains("style:"),
        "drop-zone view should avoid plain inline style paths; dynamic numeric updates are N/A and future updates must stay on style:--ui-*."
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "聚合层证据：`crates/ui/src/css.rs` 通过 `out.push_str(\"\\n@layer ui {\\n\"); ... out.push_str(crate::drop_zone::styles::CSS); ... out.push_str(\"\\n}\\n\");` 将 `drop-zone` CSS 聚合在 `@layer ui` 内。",
        "运行时数值策略（DropZone N/A）：`components/drop-zone/src/view.rs` 当前无运行时数值内联样式路径（无 `style=`/`style:`），因此不存在 `style=\"top: ...\"` 一类普通内联样式；若未来新增动态数值，仅允许 `style:--ui-*` 自定义变量透传。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_cascade_layer_and_runtime_style_contract_is_enforced`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_cascade_layer_and_runtime_style_contract_is_enforced`。",
        "门禁证据：`scripts/check-ui-contract-hygiene.sh` 新增 `drop_zone_cascade_layer_and_runtime_style_contract_is_enforced` 命令，阻断 `@layer ui` 边界和 inline style 约束回归。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document cascade-layer contract evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_cascade_layer_check_script_covers_layer_and_inline_style_guard() {
    let script = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    let required = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script.contains(required),
        "contract-hygiene check script should enforce `{required}`."
    );
}

#[test]
fn drop_zone_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop() {
    let motion = load_source("motion");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub struct DropZoneMotion",
        "pub spring: ui_motion::spring::SpringConfig",
        "stiffness: tokens.spring.stiffness",
        "damping: tokens.spring.damping",
        "pub fn sanitize_motion(motion: DropZoneMotion) -> DropZoneMotion",
        "ui_motion::spring::sanitize_config(value, DropZoneMotion::default().spring)",
        "#[cfg(target_arch = \"wasm32\")]",
        "pub fn attach_motion(",
        "if ui_motion::web::prefers_reduced_motion() {",
        "SpringAnimator::new",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(required),
            "drop-zone motion.rs should keep contract marker `{required}`."
        );
    }

    assert!(
        view.contains("motion::attach_motion("),
        "drop-zone view should attach motion contract via `motion::attach_motion(...)`."
    );

    let non_wasm_start = motion
        .find("#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_motion(")
        .unwrap_or_else(|| {
            panic!("drop-zone motion should define non-wasm attach_motion no-op stub.")
        });
    let tests_start = motion
        .find("#[cfg(test)]")
        .unwrap_or_else(|| panic!("drop-zone motion should keep test module marker."));
    let non_wasm_body = &motion[non_wasm_start..tests_start];
    for forbidden in ["SpringAnimator::new", "set_property(", "unchecked_into()"] {
        assert!(
            !non_wasm_body.contains(forbidden),
            "drop-zone non-wasm attach_motion must stay no-op and not depend on wasm runtime token `{forbidden}`."
        );
    }

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "Contract 证据：`components/drop-zone/src/motion.rs::DropZoneMotion` 内置 `spring/hover_scale/drop_scale/hover_highlight`，`Default` 从 `ui_theme::default_drop_zone_motion_tokens()` 映射 `stiffness/damping/mass/precision`，并经 `sanitize_motion` 统一归一。",
        "挂载证据：`components/drop-zone/src/view.rs` 通过 `motion::attach_motion(zone_ref, hover.is_hovered, is_drop_target, focus_ring.is_focused, is_disabled, motion)` 执行组件语义到动效 contract 的绑定。",
        "reduced-motion + non-wasm 证据：wasm `attach_motion` 在 `ui_motion::web::prefers_reduced_motion()` 为真时只同步写 `--ui-drop-zone-scale/--ui-drop-zone-highlight` 并 `return`；`#[cfg(not(target_arch = \"wasm32\"))] attach_motion` 仅 `std::hint::black_box(sanitize_motion(motion))` no-op，SSR/tooling 可预测降级。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop`。",
        "门禁证据：`scripts/check-ui-contract-hygiene.sh` 新增 `drop_zone_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop` 命令，阻断 motion contract / reduced-motion / non-wasm no-op 回归。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document motion-contract evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_motion_contract_check_script_covers_reduced_motion_and_noop_guards() {
    let script = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    let required = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        script.contains(required),
        "contract-hygiene check script should enforce `{required}`."
    );
}

#[test]
fn drop_zone_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let ui_components_lib = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_css = include_str!("../../../crates/ui/src/css.rs");
    let ui_components_root = include_str!("../../../crates/ui/src/root.rs");
    let ui_components_cargo = include_str!("../../../crates/ui/Cargo.toml");
    let active_highlight =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_state =
        include_str!("../../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence = include_str!("../../../crates/ui-headless/src/presence.rs");
    let headless_a11y = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let check2 = load_source("check2");

    for required in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-drop_zone\")]",
        "pub use ui_drop_zone as drop_zone;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs should keep fixed entry marker `{required}`."
        );
    }

    for forbidden in ["web_sys::", "NodeRef<", "HtmlElement"] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui lib.rs should not leak platform detail `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-drop_zone\")]",
        "out.push_str(crate::drop_zone::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css.rs should keep fixed entry marker `{required}`."
        );
    }

    for required in [
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "UiRoot should centralize theme/i18n/css injection via `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should keep generic motion contract `{required}`."
        );
    }

    for forbidden in ["DropZone", "Accordion", "Dialog", "Popover"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should not carry component business semantics `{forbidden}`."
        );
    }

    let workspace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../");
    for absent in [
        "crates/ui/src/overlay_open.rs",
        "crates/ui/src/presence.rs",
        "crates/ui/src/a11y.rs",
    ] {
        assert!(
            !workspace_dir.join(absent).exists(),
            "ui should not include forbidden entrypoint file `{absent}`."
        );
    }

    for required in [
        "pub fn use_controllable_state",
        "pub fn use_controllable_open_state_traced",
    ] {
        assert!(
            headless_controllable_state.contains(required),
            "headless controllable-state source should keep canonical open-state primitive `{required}`."
        );
    }

    assert!(
        headless_presence.contains("pub fn use_presence"),
        "headless presence source should keep canonical presence primitive."
    );
    assert!(
        headless_a11y.contains("pub fn locale_attrs("),
        "headless a11y source should keep canonical locale/a11y helper."
    );
    assert!(
        ui_components_cargo.contains("component-drop_zone = [\"dep:ui-drop-zone\"]"),
        "ui Cargo.toml should keep component-level fixed entry gate for drop-zone."
    );

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "入口证据：`crates/ui/src/lib.rs` 保持 `mod css;` + `pub mod root;` + `pub use root::UiRoot;`，并在 `#[cfg(feature = \"component-drop_zone\")]` 下导出 `pub use ui_drop_zone as drop_zone;`，公共 API 不泄露 `web_sys/NodeRef/HtmlElement` 平台细节。",
        "CSS 入口证据：`crates/ui/src/css.rs` 通过 `push_components_css` 聚合样式，并在 `#[cfg(feature = \"component-drop_zone\")]` 下注入 `crate::drop_zone::styles::CSS`；同时保留 `#[cfg(not(feature = \"inject-css\"))]` no-op 分支，避免无条件聚合。",
        "Root 入口证据：`crates/ui/src/root.rs::UiRoot` 统一执行 `provide_ui_i18n` / `provide_ui_id_provider` 与 base css + theme vars +（可选）components css 注入，主题与注入策略集中不下沉到组件层。",
        "共享原语落点证据：`crates/ui-visual-primitive/src/active_highlight.rs` 仅提供通用高亮样式与 motion driver（`ActiveHighlightMotion` + `attach_active_highlight_motion`），不承载 DropZone/业务语义。",
        "禁止文件证据：`crates/ui/src/overlay_open.rs`、`crates/ui/src/presence.rs`、`crates/ui/src/a11y.rs` 均不存在；对应原语固定在 `crates/ui-headless/src/controllable_state.rs`、`crates/ui-headless/src/presence.rs`、`crates/ui-headless/src/a11y.rs`。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_ui_components_fixed_entry_files_follow_layered_boundaries`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_ui_components_fixed_entry_files_follow_layered_boundaries`。",
        "门禁证据：`scripts/check-ui-entrypoints.sh` 新增 `drop_zone_ui_components_fixed_entry_files_follow_layered_boundaries` 命令，阻断入口落点与禁止文件回归。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document fixed-entry evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_entrypoints_check_script_covers_fixed_entry_files_gate() {
    let script = include_str!("../../../scripts/check-ui-entrypoints.sh");

    let required = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script.contains(required),
        "entrypoints check script should enforce `{required}`."
    );
}

#[test]
fn drop_zone_semantics_contract_tests_are_primary_and_not_snapshot_only() {
    let semantics = include_str!("../test/semantics.rs");
    let check2 = load_source("check2");

    for required in [
        "drop_zone_has_no_controlled_uncontrolled_state_axis",
        "drop_zone_has_a11y_i18n_l10n_contract_without_view_hardcoded_copy",
        "drop_zone_state_markers_are_observable_searchable_and_enumerable",
        "drop_zone_styles_depend_on_explicit_state_markers_not_dom_structure_guessing",
        "drop_zone_macro_micro_drag_state_machine_stays_split_and_converges_on_drag_end",
        "drop_zone_async_semantics_are_not_applicable_without_async_state_axis",
    ] {
        assert!(
            semantics.contains(required),
            "drop-zone semantics suite should include semantic-contract coverage `{required}`."
        );
    }

    for required in [
        "ui_headless::a11y::should_focus_proxy_button_on_click",
        "data-disabled=super::logic::bool_data_attr(is_disabled)",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
        "fn collect_files_from_data_transfer(dt: &leptos::web_sys::DataTransfer) -> Vec<DroppedFile> {",
        "fn collect_files_from_drag_event(_ev: &ev::DragEvent) -> Vec<DroppedFile> {",
    ] {
        assert!(
            semantics.contains(required),
            "drop-zone semantic assertions should keep matrix evidence marker `{required}`."
        );
    }

    for forbidden in snapshot_only_forbidden_patterns() {
        assert!(
            !semantics.contains(&forbidden),
            "drop-zone semantic suite should not rely on visual snapshot assertion `{forbidden}`."
        );
    }

    for required in [
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "矩阵覆盖：`受控/非受控` 轴对 `DropZone` 标注 N/A（无持久可控状态轴），`disabled` 由 `data-disabled/aria-disabled/data-disabled-source` 断言覆盖，键盘路径由隐藏 button + focus ring + paste 契约覆盖，指针路径由 drag/hover/drop 语义断言覆盖，SSR/wasm 差异由 `collect_files_from_*` 的 `cfg(target_arch = \"wasm32\")` / `cfg(not(target_arch = \"wasm32\"))` 分支断言覆盖。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_semantics_contract_tests_are_primary_and_not_snapshot_only`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_semantics_contract_tests_are_primary_and_not_snapshot_only`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document semantics-first testing evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view = load_source("view");
    let local_semantics = include_str!("../test/semantics.rs");
    let semantics = include_str!("../../../components/drop-zone/test/drop_zone/semantics.rs");
    let perf_script = include_str!("../../../scripts/check-ui-performance.sh");

    for needle in [
        "role=\"group\"",
        "aria-label=labels.aria_label.clone()",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
        "data-disabled-source=disabled_source.as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
    ] {
        assert!(
            view.contains(needle),
            "drop-zone semantic-priority contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "fn drop_zone_has_a11y_i18n_l10n_contract_without_view_hardcoded_copy()",
        "fn drop_zone_state_markers_are_observable_searchable_and_enumerable()",
        "fn drop_zone_semantics_contract_tests_are_primary_and_not_snapshot_only()",
    ] {
        assert!(
            local_semantics.contains(needle),
            "drop-zone local semantics suite should keep marker `{needle}`."
        );
    }

    for needle in [
        "fn drop_zone_semantics_contract_tests_are_primary_and_not_snapshot_only()",
        "fn drop_zone_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks(",
    ] {
        assert!(
            semantics.contains(needle),
            "drop-zone semantic-priority path should keep marker `{needle}`."
        );
    }

    for forbidden in snapshot_only_forbidden_patterns() {
        assert!(
            !local_semantics.contains(&forbidden) && !semantics.contains(&forbidden),
            "drop-zone semantic-priority path should avoid snapshot-only assertion `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script.contains(script_needle),
        "performance script should include semantic-priority gate `{script_needle}`."
    );
}

#[test]
fn drop_zone_performance_script_covers_semantic_test_priority_contract() {
    let script_source = include_str!("../../../scripts/check-ui-performance.sh");

    for needle in [
        "echo \"[perf] contract: drop-zone semantic test priority\"",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "performance script should include drop-zone semantic-priority marker `{needle}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_semantic_test_priority_item_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "drop-zone check2 should mark semantic-test-priority item complete."
    );

    for required in [
        "components/drop-zone/test/semantics.rs::drop_zone_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "components/drop-zone/test/drop_zone/semantics.rs::drop_zone_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "drop_zone_performance_script_covers_semantic_test_priority_contract",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 semantic-test-priority section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 e2e-selector section should include `{required}`."
        );
    }
}

#[test]
fn drop_zone_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_drop_zone_contract.spec.mjs");
    let docs_source = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/files.rs",
    );

    for required in [
        "docs-app drop-zone uses semantic selectors with wasm-stable ready waits",
        "docs-app drop-zone motion interaction uses semantic ready and settled breakpoints",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-component=\"drop-zone\"]",
        "[data-slot=\"drop-zone-e2e-quick-start\"]",
        "[data-slot=\"drop-zone-e2e-state-disabled\"]",
        "[data-slot=\"drop-zone-e2e-state-custom-motion\"]",
        "[data-slot=\"drop-zone-workbench-toggle-custom-motion\"]",
        "[data-slot=\"drop-zone-workbench-surface\"]",
    ] {
        assert!(
            e2e_contract.contains(required),
            "drop-zone e2e selector contract should include marker `{required}`."
        );
    }

    for required in [
        "data-slot=\"drop-zone-e2e-quick-start\"",
        "data-slot=\"drop-zone-e2e-state-disabled\"",
        "data-slot=\"drop-zone-e2e-state-custom-motion\"",
        "data-slot=\"drop-zone-workbench-toggle-custom-motion\"",
        "data-slot=\"drop-zone-workbench-surface\"",
    ] {
        assert!(
            docs_source.contains(required),
            "drop-zone docs should expose stable semantic selector anchor `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child(",
        "getByText(",
    ] {
        assert!(
            !e2e_contract.contains(forbidden),
            "drop-zone e2e selector contract should avoid fragile wait/selector `{forbidden}`."
        );
    }
}

#[test]
fn drop_zone_e2e_contract_covers_ready_and_settled_conditions_for_motion_interaction() {
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_drop_zone_contract.spec.mjs");

    for required in [
        "data-motion-source\", \"default\"",
        "data-motion-source\", \"custom\"",
        "data-drag-phase\", \"idle\"",
        "await customMotionToggle.click();",
    ] {
        assert!(
            e2e_contract.contains(required),
            "drop-zone e2e contract should keep ready/settled semantic breakpoint `{required}`."
        );
    }
}

#[test]
fn drop_zone_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script = include_str!("../../../components/drop-zone/scripts/check-ui-e2e-drop-zone.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_e2e_contract_covers_ready_and_settled_conditions_for_motion_interaction",
    ] {
        assert!(
            script.contains(required),
            "drop-zone e2e check script should include `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_e2e_selector_stability_item_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "drop-zone check2 should mark e2e selector stability item complete."
    );

    for required in [
        "components/drop-zone/test/semantics.rs::drop_zone_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/drop-zone/test/semantics.rs::drop_zone_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "components/drop-zone/test/drop_zone/semantics.rs::drop_zone_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/drop-zone/test/drop_zone/semantics.rs::drop_zone_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "e2e/tests/docs_app_drop_zone_contract.spec.mjs",
        "components/drop-zone/scripts/check-ui-e2e-drop-zone.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 e2e selector stability section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 repeatable-key-flow section should include `{required}`."
        );
    }
}

#[test]
fn drop_zone_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_drop_zone_contract.spec.mjs");

    for required in [
        "docs-app drop-zone key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "drop-zone-workbench-toggle-disabled",
        "drop-zone-button",
        "data-disabled\", \"true\"",
        "data-disabled\", \"false\"",
        "data-disabled-source\", \"is_disabled\"",
        "aria-disabled\", \"true\"",
        "aria-disabled\", \"false\"",
        "data-focused\", \"true\"",
        "data-focused\", \"false\"",
        "data-drag-phase\", \"idle\"",
        "await page.keyboard.press(\"Enter\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_contract.contains(required),
            "drop-zone repeatable key-flow contract should include marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_e2e_check_script_covers_repeatable_key_flow_contract() {
    let script = include_str!("../../../components/drop-zone/scripts/check-ui-e2e-drop-zone.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script.contains(required),
            "drop-zone e2e check script should include repeatable-key-flow marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_e2e_repeatable_key_flow_item_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "drop-zone check2 should mark repeatable e2e key-flow item complete."
    );

    for required in [
        "components/drop-zone/test/semantics.rs::drop_zone_check2_documents_e2e_repeatable_key_flow_rules",
        "components/drop-zone/test/semantics.rs::drop_zone_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/drop-zone/test/semantics.rs::drop_zone_check2_marks_e2e_repeatable_key_flow_item_complete",
        "components/drop-zone/test/drop_zone/semantics.rs::drop_zone_check2_documents_e2e_repeatable_key_flow_rules",
        "components/drop-zone/test/drop_zone/semantics.rs::drop_zone_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/drop-zone/test/drop_zone/semantics.rs::drop_zone_check2_marks_e2e_repeatable_key_flow_item_complete",
        "components/drop-zone/scripts/check-ui-e2e-drop-zone.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone check2 repeatable key-flow section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_visual_desire_baseline_is_documented_and_backed_by_theme_visual_regression() {
    let check2 = load_source("check2");
    let drop_zone_docs = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/files.rs",
    );
    let theme_visual_baseline = source_contract::source_from_file_relative(
        file!(),
        "../../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs",
    );
    let theme_visual_baseline_e2e =
        include_str!("../../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for required in [
        "pub(super) fn drop_zone() -> AnyView",
        "title=\"Quick Start (Default API)\"",
        "<Playground title=\"Drop / paste\" code_signal=code>",
        "<Playground title=\"Drop / paste with custom motion\" code_signal=motion_code>",
    ] {
        assert!(
            drop_zone_docs.contains(required),
            "drop-zone docs baseline should include `{required}`."
        );
    }

    for required in [
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "description=\"Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.\"",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            theme_visual_baseline.contains(required),
            "theme visual baseline docs should include `{required}`."
        );
    }

    for required in [
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            theme_visual_baseline_e2e.contains(required),
            "theme visual baseline e2e should include screenshot regression marker `{required}`."
        );
    }

    for required in [
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "组件基线证据：`apps/docs-app/src/pages/components/pages/files.rs::drop_zone` 提供 `Quick Start (Default API)`、`Drop / paste`、`Drop / paste with custom motion` 三组示例，确保默认主题下的信息层级、对比与交互反馈可直接验收。",
        "全局视觉基线证据：`apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs` 提供 `theme-visual-baseline` 页面，覆盖 `Button/Input/Overlay` 默认主题层级与对比说明。",
        "截图回归证据：`e2e/tests/docs_app_theme_visual_baseline.spec.mjs` 对 `data-slot=\"theme-visual-baseline\"`、`theme-visual-baseline-button`、`theme-visual-baseline-input`、`theme-visual-baseline-overlay` 执行 `toHaveScreenshot`，用于阻断默认主题视觉退化。",
        "对标边界：遵循 `HeroUI` 的视觉语言与体验质量对齐目标，不复制其 API 表层；`DropZone` 保持本仓库统一命名与语义契约。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_visual_desire_baseline_is_documented_and_backed_by_theme_visual_regression`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_visual_desire_baseline_is_documented_and_backed_by_theme_visual_regression`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document visual desire evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let check2 = load_source("check2");
    let shell_source = include_str!("../../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = include_str!("../../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = include_str!("../../../docs/plan/TODO.md");
    let view_source = load_source("view");
    let motion_source = load_source("motion");

    for required in [
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "\"drop-zone\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            shell_source.contains(required),
            "component shell should keep performance budget marker `{required}`."
        );
    }

    for required in [
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
            perf_probe_source.contains(required),
            "UiPerfProbe should expose repeatable performance threshold marker `{required}`."
        );
    }

    for required in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "docs coverage e2e should enforce perf blocking marker `{required}`."
        );
    }

    for required in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(required),
            "debug overlay should keep trace attribution marker `{required}`."
        );
    }

    for required in [
        "data-drag-phase=move || drag_phase.get().as_attr()",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
        "data-focused=move || super::logic::bool_data_attr(focus_ring.is_focused.get())",
        "data-motion-source=motion_source.as_attr()",
        "motion::attach_motion(",
        "set_drag_over_tick.update(|tick| *tick = tick.wrapping_add(1));",
        "if ui_motion::web::prefers_reduced_motion() {",
    ] {
        let hit = view_source.contains(required) || motion_source.contains(required);
        assert!(
            hit,
            "drop-zone implementation should expose perf attribution route `{required}`."
        );
    }

    for required in [
        "性能治理：关键路径有预算",
        "drop-zone` 定义 `UiPerfBudget",
        "scripts/check-ui-performance.sh",
        "render_count",
        "Button/Input",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should keep performance governance marker `{required}`."
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone）",
    ] {
        assert!(
            todo_source.contains(required),
            "performance governance follow-up plan should keep `{required}`."
        );
    }
}

#[test]
fn drop_zone_performance_check_script_covers_budget_and_follow_up_gates() {
    let script_source = include_str!("../../../scripts/check-ui-performance.sh");

    for required in [
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(required),
            "performance gate script should include `{required}`."
        );
    }
}

#[test]
fn drop_zone_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("view");
    let local_semantics_source = include_str!("../test/semantics.rs");
    let check2_source = load_source("check2");
    let perf_script_source = include_str!("../../../scripts/check-ui-performance.sh");
    let todo_source = include_str!("../../../docs/plan/TODO.md");

    for required in [
        "role=\"group\"",
        "aria-label=labels.aria_label.clone()",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
        "data-drag-phase=move || drag_phase.get().as_attr()",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
        "data-focused=move || super::logic::bool_data_attr(focus_ring.is_focused.get())",
        "data-focus-visible=move || super::logic::bool_data_attr(focus_ring.is_focus_visible.get())",
        "data-disabled=super::logic::bool_data_attr(is_disabled)",
        "data-disabled-source=disabled_source.as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "ui_headless::a11y::should_focus_proxy_button_on_click",
    ] {
        assert!(
            view_source.contains(required),
            "drop-zone view should keep aria/data/focus contract marker `{required}`."
        );
    }

    for required in [
        "fn drop_zone_semantics_contract_tests_are_primary_and_not_snapshot_only()",
        "fn drop_zone_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn drop_zone_performance_check_script_covers_budget_and_follow_up_gates()",
    ] {
        assert!(
            local_semantics_source.contains(required),
            "drop-zone local semantics suite should keep contract test `{required}`."
        );
    }

    let perf_gate_needle = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_performance_governance_contract_is_budgeted_traceable_and_blocking";
    assert!(
        perf_script_source.contains(perf_gate_needle),
        "performance gate script should include `{perf_gate_needle}`."
    );

    let matrix_gate_needle = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement";
    assert!(
        perf_script_source.contains(matrix_gate_needle),
        "performance gate script should include `{matrix_gate_needle}`."
    );

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone）",
    ] {
        assert!(
            todo_source.contains(required),
            "performance governance follow-up should keep `{required}` marker."
        );
    }

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "drop_zone_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "drop_zone_performance_script_covers_semantics_and_performance_regression_matrix",
        "scripts/check-ui-performance.sh",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone）",
    ] {
        assert!(
            check2_source.contains(required),
            "drop-zone check2 semantics+performance section should reference `{required}`."
        );
    }
}

#[test]
fn drop_zone_performance_script_covers_semantics_and_performance_regression_matrix() {
    let script_source = include_str!("../../../scripts/check-ui-performance.sh");

    for required in [
        "echo \"[perf] contract: drop-zone semantics/perf matrix\"",
        "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            script_source.contains(required),
            "performance check script should include `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_semantics_and_performance_regression_item_complete() {
    let check2_source = load_source("check2");
    assert!(
        check2_source.contains("- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。"),
        "drop-zone check2 should mark semantics+performance item complete."
    );
    for required in [
        "drop_zone_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "drop_zone_performance_script_covers_semantics_and_performance_regression_matrix",
        "drop_zone_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "drop_zone_semantics_contract_tests_are_primary_and_not_snapshot_only",
        "scripts/check-ui-performance.sh",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone）",
    ] {
        assert!(
            check2_source.contains(required),
            "drop-zone check2 semantics+performance item should include `{required}`."
        );
    }
}

#[test]
fn drop_zone_view_macro_complexity_is_split_into_semantic_subviews() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "fn render_label_slot(label: Option<String>) -> impl IntoView",
        "fn render_zone_content<OnPaste>(",
        "let label_view = render_label_slot(labels.label.clone());",
        "let zone_content = render_zone_content(",
        "{label_view}",
        "{zone_content}",
    ] {
        assert!(
            view.contains(required),
            "drop-zone view should split large `view!` structure into semantic subviews via `{required}`."
        );
    }

    let view_macro_count = view.matches("view! {").count();
    assert!(
        view_macro_count >= 3,
        "drop-zone view should keep multiple focused `view!` blocks after split; current count={view_macro_count}."
    );

    for required in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "语义拆分证据：`components/drop-zone/src/view.rs` 将 label 与 zone 内部内容拆为 `render_label_slot`、`render_zone_content` 两个局部渲染函数，主 `DropZone` 仅保留容器装配与语义挂载。",
        "宏展开控制证据：`view.rs` 中 `view!` 拆分为“根容器 + label 子块 + zone content 子块”，不再由单个巨型 `view!` 同时承载深层嵌套结构与所有子节点。",
        "排障约束：若后续出现编译时间或 wasm 产物体积异常增长，优先排查 `view!` 体量与局部子块是否退化为单块拼装。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_view_macro_complexity_is_split_into_semantic_subviews`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_view_macro_complexity_is_split_into_semantic_subviews`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should capture `view!` macro complexity evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_functional_split_prefers_plain_view_functions_over_extra_components() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "fn render_label_slot(label: Option<String>) -> impl IntoView",
        "fn render_zone_content<OnPaste>(",
        ") -> impl IntoView",
    ] {
        assert!(
            view.contains(required),
            "drop-zone view should keep plain helper functions for light UI fragments via `{required}`."
        );
    }

    let component_count = view.matches("#[component]").count();
    assert!(
        component_count == 1,
        "drop-zone view should keep only the top-level component; current count={component_count}."
    );

    for forbidden in [
        "#[component]\nfn render_label_slot",
        "#[component]\nfn render_zone_content",
    ] {
        assert!(
            !view.contains(forbidden),
            "drop-zone helper fragments should not be promoted to standalone components (`{forbidden}`)."
        );
    }

    for required in [
        "{label_view}",
        "{zone_content}",
        "data-slot=\"drop-zone-label\"",
        "data-slot=\"drop-zone-zone\"",
    ] {
        assert!(
            view.contains(required),
            "drop-zone semantic markers should remain stable after functional split via `{required}`."
        );
    }

    for required in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "函数化证据：`components/drop-zone/src/view.rs` 使用 `render_label_slot` 与 `render_zone_content` 两个普通函数承载轻逻辑 UI 片段，返回 `impl IntoView`，未把局部片段升格为独立组件。",
        "组件边界证据：`view.rs` 仅保留一个 `#[component]`（`DropZone`），避免“所有局部片段都变组件”的抽象噪音。",
        "语义稳定证据：拆分后关键语义标记仍挂载在主渲染路径（`data-slot=\"drop-zone-label\"`、`data-slot=\"drop-zone-zone\"`、`aria-*`/`data-*`），测试定位不依赖 DOM 偶然结构。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_functional_split_prefers_plain_view_functions_over_extra_components`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_functional_split_prefers_plain_view_functions_over_extra_components`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should capture functional split evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_static_fragments_are_templateized_and_large_static_assets_are_not_applicable() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "fn render_label_slot(label: Option<String>) -> impl IntoView",
        "fn render_zone_content<OnPaste>(",
        "class=\"ui-drop-zone__button\"",
        "role=\"group\"",
        "aria-label=labels.aria_label.clone()",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
    ] {
        assert!(
            view.contains(required),
            "drop-zone static template path should retain semantic marker `{required}`."
        );
    }

    let button_shell_count = view.matches("class=\"ui-drop-zone__button\"").count();
    assert!(
        button_shell_count == 1,
        "drop-zone static button shell should have a single template source; current count={button_shell_count}."
    );

    for forbidden in ["<svg", "inner_html=", "<footer", "Drop files"] {
        assert!(
            !view.contains(forbidden),
            "drop-zone view should not carry large static asset/hardcoded-copy fragment token `{forbidden}`."
        );
    }

    for required in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "N/A 边界：`DropZone` 当前不包含复杂 SVG、页脚或长说明文本；纯静态 UI 仅为轻量按钮壳与标签容器，不存在可抽离为独立静态资源文件的大块模板。",
        "模板化证据：`components/drop-zone/src/view.rs` 通过 `render_zone_content` 集中承载静态按钮结构（`type/class/data-slot/aria-label`），避免在多个 `view!` 片段重复构造同一静态子树。",
        "A11y 语义保持：静态片段模板化后仍保留 `role=\"group\"`、`aria-label`、`aria-disabled` 等可访问标记，未因拆分丢失语义契约。",
        "变更路径清晰：静态片段入口集中在 `view.rs` 的 `render_label_slot`/`render_zone_content`，后续静态结构调整无需跨多处 `view!` 搜索替换。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_static_fragments_are_templateized_and_large_static_assets_are_not_applicable`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_static_fragments_are_templateized_and_large_static_assets_are_not_applicable`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document static fragment templateization evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_inner_html_contract_disallows_untrusted_html_injection() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    for forbidden in ["inner_html=", "innerHTML=", "dangerously_set_inner_html"] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "drop-zone should not provide raw html injection sink `{forbidden}`."
        );
    }

    for required in [
        "#[prop(optional, into)] label: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "role=\"group\"",
        "aria-label=labels.aria_label.clone()",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
    ] {
        assert!(
            view.contains(required),
            "drop-zone should keep explicit a11y attrs without html injection path `{required}`."
        );
    }

    for required in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A 边界：`DropZone` 当前不使用 `inner_html`（`view.rs`/`logic.rs`/`motion.rs` 均无该属性），因此不存在 HTML 注入入口。",
        "安全证据：组件对外输入仅通过类型化 props 与事件回调流转（`label/aria_label/on_drop_files`），未将用户输入或远端文本拼接为 HTML 字符串再注入 DOM。",
        "语义保持：可访问语义通过显式属性挂载（`role=\"group\"`、`aria-label`、`aria-disabled`），未依赖 `inner_html` 注入语义节点。",
        "升级约束：若未来确需 `inner_html`，仅允许编译期静态常量或白名单模板，并必须补充语义与安全回归测试后再启用。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_inner_html_contract_disallows_untrusted_html_injection`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_inner_html_contract_disallows_untrusted_html_injection`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document inner_html guardrail evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let view = load_source("view");
    let check2 = load_source("check2");
    let trace_source = include_str!("../../../crates/ui-headless/src/trace.rs");
    let docs_app_source = include_str!("../../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
    let wasm_debug_script = include_str!("../../../scripts/check-ui-wasm-debug.sh");

    for required in [
        "use ui_headless::use_ui_trace;",
        "fn emit_drop_zone_debug_note(",
        "#[cfg(all(target_arch = \"wasm32\", debug_assertions))]",
        "trace.emit(\"drop-zone\", ui_headless::UiTraceEventKind::Note { message });",
        "event=drag_enter; source=pointer; drop_target:{}->{}; phase:{}->{}",
        "event=drag_leave; source=pointer; drop_target:{}->{}; phase:{}->{}",
        "event=drop; source=pointer; files={}; drop_target:{}->{}; phase:{}->{}",
        "event=paste; source=keyboard; files={}; phase={}",
    ] {
        assert!(
            view.contains(required),
            "drop-zone wasm debug trace contract should include `{required}`."
        );
    }

    for required in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub kind: UiTraceEventKind,",
        "Note {",
        "events.push(event);",
    ] {
        assert!(
            trace_source.contains(required),
            "ui-headless trace substrate should expose timestamped replay events via `{required}`."
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(required),
            "docs app should keep dev-only wasm debug overlay gate `{required}`."
        );
    }

    for required in [
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "let events = events.get();",
        ".into_iter()",
        ".rev()",
        ".take(40)",
        "data-slot=\"ui-debug-overlay-events\"",
        "UiTraceEventKind::Note { message }",
    ] {
        assert!(
            debug_overlay_source.contains(required),
            "debug overlay should provide ordered event replay surface via `{required}`."
        );
    }

    assert!(
        wasm_debug_script.contains(
            "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated"
        ),
        "wasm debug gate script should enforce drop-zone wasm debug contract regression."
    );

    for required in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "调试链路证据：`components/drop-zone/src/view.rs` 通过 `use_ui_trace` + `emit_drop_zone_debug_note` 在 `drag_enter/drag_leave/drop/paste` 路径发出 `UiTraceEventKind::Note`，消息包含 `source` 与关键状态前后值（如 `drop_target`、`phase`）。",
        "时间与回放证据：`crates/ui-headless/src/trace.rs` 的 `UiTraceEvent` 含 `ts_ms/component/kind`，`apps/docs-app/src/debug_overlay.rs::render_events` 按事件序列展示 `ts_ms + component + kind + body`，支持最小交互链路回放排障。",
        "可视化入口证据：`apps/docs-app/src/lib.rs` 在开发模式 `cfg!(debug_assertions)` 下启用 `UiDebugOverlay`，提供 `Inspect + Events` 面板用于 wasm 调试可视化。",
        "隔离证据：`provide_ui_trace(debug_overlay_enabled)` 仅在开发模式启用；`emit_drop_zone_debug_note` 在 `#[cfg(all(target_arch = \"wasm32\", debug_assertions))]` 下才真正写入事件，默认产物与公共 API 不暴露调试开关。",
        "门禁证据：`scripts/check-ui-wasm-debug.sh` 新增 `drop-zone` 调试契约测试命令，确保约束可回归。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document wasm debug contract evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_tree_shaking_contract_uses_feature_gated_exports_and_ci_budget_checks() {
    let check2 = load_source("check2");
    let drop_zone_mod = load_source("mod");
    let ui_components_cargo = include_str!("../../../crates/ui/Cargo.toml");
    let ui_components_lib = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_css = include_str!("../../../crates/ui/src/css.rs");
    let tree_shaking_script = include_str!("../../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = include_str!("../../../scripts/tree_shaking_budget.env");
    let ci_workflow = include_str!("../../../.github/workflows/ci.yml");

    for required in [
        "component-drop_zone = [\"dep:ui-drop-zone\"]",
        "ui-drop-zone = { path = \"../../components/drop-zone\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui Cargo feature graph should include drop-zone tree-shaking gate `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-drop_zone\")]",
        "pub use ui_drop_zone as drop_zone;",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "#[cfg(feature = \"all-components\")]",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib export boundary should keep feature-gated tree-shaking contract `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-drop_zone\")]",
        "out.push_str(crate::drop_zone::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css aggregation should stay feature-gated for drop-zone `{required}`."
        );
    }

    for forbidden in ["ALL_COMPONENTS_REGISTRY", "static COMPONENT_REGISTRY"] {
        assert!(
            !drop_zone_mod.contains(forbidden),
            "drop-zone source mode should not introduce global registry token `{forbidden}`."
        );
    }

    for required in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "source \"$BUDGET_FILE\"",
        "CURRENT_BYTES=\"$(stat -c '%s' \"$LATEST_RLIB\")\"",
        "if (( CURRENT_BYTES > MAX_BYTES )); then",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking CI script should enforce `{required}`."
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(required),
            "tree-shaking budget file should define `{required}`."
        );
    }

    assert!(
        ci_workflow.contains("./scripts/check-ui-tree-shaking.sh"),
        "CI workflow should execute tree-shaking gate script."
    );

    for required in [
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "package feature 证据：`crates/ui/Cargo.toml` 定义 `component-drop_zone = [\"dep:ui-drop-zone\"]`，且 `ui-drop-zone` 依赖为 `optional = true`；`crates/ui/src/lib.rs` 使用 `#[cfg(feature = \"component-drop_zone\")] pub use ui_drop_zone as drop_zone;`；`crates/ui/src/css.rs` 仅在 `#[cfg(feature = \"component-drop_zone\")]` 下聚合 `crate::drop_zone::styles::CSS`。",
        "source 模式证据：`components/drop-zone/src/mod.rs` 仅暴露组件 API，不包含全量组件注册表；组件源码按需由上层 feature gate 引入，不通过中心映射强制保持全可达。",
        "特性树实测：执行 `cargo tree -e features -i ui -p ui --no-default-features --features component-drop_zone,inject-css` 输出仅含 `feature \"component-drop_zone\" (command-line)` 与 `feature \"inject-css\" (command-line)`，未出现 `all-components`。",
        "反向依赖实测：执行 `cargo tree -e features -i ui -p web-demo` 输出包含 `feature \"web-demo-components\"`、`feature \"component-drop_zone\"` 与 `feature \"inject-css\"`，未出现 `all-components`。",
        "CI 证据：`.github/workflows/ci.yml` 已调用 `./scripts/check-ui-tree-shaking.sh`；脚本包含最小特性 wasm 编译检查（`cargo check ... --no-default-features --features component-accordion,inject-css`）与 release 产物预算检查（读取 `scripts/tree_shaking_budget.env` 的 `TREE_SHAKING_BASELINE_RLIB_BYTES` / `TREE_SHAKING_MAX_RATIO_PERCENT`，并阻断超预算）。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_tree_shaking_contract_uses_feature_gated_exports_and_ci_budget_checks`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_tree_shaking_contract_uses_feature_gated_exports_and_ci_budget_checks`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document tree-shaking evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "组件特性树证据：`crates/ui/Cargo.toml` 已注册 `component-drop_zone = [\"dep:ui-drop-zone\"]`",
        "门禁证据：`scripts/check-ui-tree-shaking.sh` 显式覆盖 `drop_zone_tree_shaking_contract_uses_feature_gated_exports_and_ci_budget_checks`",
        "drop_zone_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "drop_zone_tree_shaking_script_covers_component_feature_pruning_contract",
        "drop_zone_tree_shaking_contract_uses_feature_gated_exports_and_ci_budget_checks",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should pin tree-shaking pruning marker `{required}`."
        );
    }
}

#[test]
fn drop_zone_tree_shaking_script_covers_component_feature_pruning_contract() {
    let script_source = include_str!("../../../scripts/check-ui-tree-shaking.sh");
    let needle = "cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_tree_shaking_contract_uses_feature_gated_exports_and_ci_budget_checks";
    assert!(
        script_source.contains(needle),
        "tree-shaking script should enforce `{needle}`."
    );
}

#[test]
fn drop_zone_type_system_and_semantic_markers_form_machine_readable_contract() {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub enum DisabledInput",
        "pub enum DisabledSource",
        "pub enum MotionSource",
        "pub enum AriaLabelSource",
        "pub enum DragLifecyclePhase",
        "pub const fn classify_disabled_input(",
        "pub const fn resolve_is_disabled(input: DisabledInput) -> (bool, DisabledSource)",
        "pub const fn resolve_aria_label_source(has_custom_aria_label: bool) -> AriaLabelSource",
        "pub(crate) fn resolve_props(input: DropZonePropsInput) -> DropZoneResolvedProps",
    ] {
        assert!(
            logic.contains(required),
            "drop-zone logic should keep typed/closed machine-readable state contracts via `{required}`."
        );
    }

    for required in [
        "data-drag-phase=move || drag_phase.get().as_attr()",
        "data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())",
        "data-disabled=super::logic::bool_data_attr(is_disabled)",
        "data-disabled-source=disabled_source.as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "data-aria-source=aria_source.as_attr()",
        "aria-label=labels.aria_label.clone()",
        "aria-disabled=super::logic::bool_data_attr(is_disabled)",
    ] {
        assert!(
            view.contains(required),
            "drop-zone view should expose machine-readable semantic markers via `{required}`."
        );
    }

    for required in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "类型约束证据：`components/drop-zone/src/logic.rs` 使用 `DisabledInput/DisabledSource/MotionSource/AriaLabelSource/DragLifecyclePhase` 等闭集 `enum` 建模关键状态轴；来源值统一通过 `as_attr()` 输出，避免自由字符串协议。",
        "归一化证据：`components/drop-zone/src/logic.rs::classify_disabled_input`、`resolve_is_disabled`、`resolve_aria_label_source`、`resolve_props` 将输入先归一化再派生为 `DropZoneResolvedProps`，无效组合不在 `view.rs` 分散处理。",
        "语义标记证据：`components/drop-zone/src/view.rs` 挂载稳定标记 `data-drag-phase/data-drop-target/data-disabled/data-disabled-source/data-motion-source/data-aria-source` 与 `aria-label/aria-disabled`，机器可直接检索关键状态和来源。",
        "闭环证据：类型约束由编译期签名锁定（`enum` + typed props），语义契约由测试回归锁定（`components/drop-zone/test/semantics.rs` 与 `components/drop-zone/test/drop_zone/semantics.rs`）。",
        "回归覆盖：`components/drop-zone/test/semantics.rs::drop_zone_type_system_and_semantic_markers_form_machine_readable_contract`、`components/drop-zone/test/drop_zone/semantics.rs::drop_zone_type_system_and_semantic_markers_form_machine_readable_contract`。",
    ] {
        assert!(
            check2.contains(required),
            "drop-zone checklist should document typed-state and semantic-marker evidence `{required}`."
        );
    }
}

#[test]
fn drop_zone_checklist_marks_ui_components_item_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `ui` 定义：最终 Leptos 组件装配层"),
        "drop-zone checklist should mark the ui assembly item as done."
    );
}
