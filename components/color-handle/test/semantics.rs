fn load_source(path: &str) -> &'static str {
    match path {
        "lib" => include_str!("../src/lib.rs"),
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "readme" => include_str!("../src/README.md"),
        "check2" => include_str!("../check2.md"),
        "todo_plan" => include_str!("../../../docs/plan/TODO.md"),
        "perf_script" => include_str!("../../../scripts/check-ui-components-performance.sh"),
        "docs_forms_color" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs")
        }
        "docs_pages_catalog" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "heroui_spec" => include_str!("../../../docs/spec/heroui-parameter-design-strategy.md"),
        "e2e_color_handle_contract" => {
            include_str!("../../../e2e/tests/docs_app_color_handle_contract.spec.mjs")
        }
        "legacy_semantics" => {
            include_str!("../../../crates/ui-components/tests/color_handle_semantics.rs")
        }
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn color_handle_semantics_tests_are_migrated_to_component_directory() {
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let legacy_semantics = load_source("legacy_semantics");
    let local_semantics = include_str!("semantics.rs");

    for source in [lib_source, mod_source] {
        assert!(
            source.contains("#[path = \"../test/semantics.rs\"]")
                && source.contains("mod semantics_tests;"),
            "color-handle should wire `components/color-handle/test/semantics.rs` from both lib/mod entrypoints.",
        );
    }

    assert!(
        legacy_semantics.contains("color_handle_"),
        "legacy ui-components semantics suite should still be readable during migration.",
    );
    assert!(
        local_semantics
            .contains("color_handle_semantics_tests_are_migrated_to_component_directory"),
        "component-local semantics suite should provide migration coverage.",
    );
}

#[test]
fn color_handle_module_keeps_ui_components_boundaries() {
    let mod_source = load_source("mod");

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ColorHandleState, ColorHandleStateInput, DEFAULT_ARIA_LABEL};",
        "pub use motion::ColorHandleMotion;",
        "pub use view::ColorHandle;",
    ] {
        assert!(
            mod_source.contains(required),
            "color-handle mod.rs should keep ui-components export boundary `{required}`.",
        );
    }
}

#[test]
fn color_handle_logic_and_view_follow_assembly_contract() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");

    assert!(
        logic_source.contains("pub use ui_state_primitives::color_handle::{"),
        "color-handle logic should consume ui-state-primitives instead of redefining state machines.",
    );

    for required in [
        "logic::resolve_state(ColorHandleStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "motion::attach_motion(None, props.get_value().motion)",
    ] {
        assert!(
            view_source.contains(required),
            "color-handle view should assemble shell semantics via `{required}`.",
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str =") && styles_source.contains("var(--ui-"),
        "color-handle styles should remain token-first static css.",
    );
    assert!(
        motion_source.contains("pub fn attach_motion("),
        "color-handle motion should expose semantic-to-runtime attach mapping.",
    );
}

#[test]
fn color_handle_public_surface_does_not_expose_dom_platform_types() {
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "HtmlElement",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "color-handle crate public entry should not expose `{forbidden}`.",
        );
        assert!(
            !mod_source.contains(forbidden),
            "color-handle ui-components module should not expose `{forbidden}`.",
        );
    }
}

#[test]
fn color_handle_checklist_marks_ui_components_definition_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `ui-components` 定义"),
        "color-handle check2 should mark ui-components definition as completed.",
    );
}

#[test]
fn color_handle_agent_contract_markers_are_schema_typed_and_machine_readable() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for required in [
        "pub enum ColorHandleAgentSchemaVersion",
        "pub enum ColorHandleAgentIntent",
        "pub enum ColorHandleAgentAction",
        "pub enum ColorHandleAgentStateAxis",
        "pub enum ColorHandleAgentSource",
        "pub struct ColorHandleAgentContract",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(required),
            "color-handle logic should provide typed agent-contract field `{required}`.",
        );
    }

    for required in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "color-handle view should mount schemaized agent marker `{required}`.",
        );
    }
}

#[test]
fn color_handle_agent_contract_render_path_stays_whitelist_safe() {
    let view_source = load_source("view");

    for forbidden in ["inner_html", "<script", "javascript:"] {
        assert!(
            !view_source.contains(forbidden),
            "color-handle agent-contract render path should stay whitelist-safe without `{forbidden}`.",
        );
    }
}

#[test]
fn color_handle_checklist_marks_agent_contract_schema_item_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。"),
        "color-handle check2 should mark Agent Contract schema governance item as completed.",
    );
}

#[test]
fn color_handle_semantic_regression_covers_aria_data_and_focus_path() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for required in [
        "role=move || a11y.get_value().role",
        "aria-label=move || a11y.get_value().aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "#[prop(optional, into)] is_focused: Option<bool>,",
        "is_focused=props.get_value().interaction_state.is_focused()",
    ] {
        assert!(
            view_source.contains(required),
            "color-handle semantic contract should mount aria/data/focus token `{required}`.",
        );
    }

    for required in [
        "Self::Focus => \"focus\"",
        "Self::Focused => \"focused\"",
        "if state.is_focused {",
        "ui-color-handle--focused",
    ] {
        assert!(
            logic_source.contains(required),
            "color-handle logic should keep focus-state contract token `{required}`.",
        );
    }

    for forbidden in ["assert_snapshot!", "insta::assert_snapshot!"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "semantic regression coverage must not rely on snapshot-only assertion `{forbidden}`.",
        );
    }
}

#[test]
fn color_handle_performance_regression_keeps_render_count_equivalent_evidence() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let check2_source = load_source("check2");
    let todo_source = load_source("todo_plan");
    let perf_script_source = load_source("perf_script");
    let combined = format!("{logic_source}\n{view_source}");

    assert!(
        combined.matches("Memo::new").count() <= 3,
        "color-handle render path should keep bounded reactive memo count (<=3).",
    );

    for forbidden in [
        "create_effect",
        "Effect::new",
        "watch(",
        "spawn_local",
        "request_animation_frame",
        "on:click",
        "on:input",
        "on:change",
        "on:keydown",
        "on:pointer",
    ] {
        assert!(
            !combined.contains(forbidden),
            "color-handle performance path should avoid unbounded render trigger `{forbidden}`.",
        );
    }

    for required in [
        "logic::resolve_state(ColorHandleStateInput {",
        "let class = Memo::new(",
        "let agent_contract =",
        "motion::attach_motion(None, props.get_value().motion)",
        "data-state=move || state.get().data_state_attr",
        "data-motion-source=motion_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "color-handle view should keep attributable render path token `{required}`.",
        );
    }

    assert!(
        perf_script_source.contains("perf_render_count_follow_up_is_tracked_in_plan"),
        "workspace perf gate script should keep render_count follow-up blocking guard.",
    );

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance TODO should keep render_count follow-up token `{needle}`.",
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-handle checklist should keep performance governance token `{needle}`.",
        );
    }
}

#[test]
fn color_handle_docs_page_is_copy_paste_ready_with_required_playgrounds() {
    let docs_source = load_source("docs_forms_color");

    for required in [
        "pub(super) fn color_handle() -> AnyView {",
        "Playground title=\"Hello World\" code_signal=hello_code",
        "Playground title=\"State Matrix\" code_signal=state_matrix_code",
        "title=\"Parameter Matrix Workbench (Display + Config + Code + CSS Test)\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "code_signal=controlled_vs_uncontrolled_code",
        "Playground title=\"Streaming Optional / Snapshot\" code_signal=output_mode_code",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "components/color-handle/src/view.rs",
        "components/color-handle/src/logic.rs",
        "components/color-handle/src/styles.rs",
    ] {
        assert!(
            docs_source.contains(required),
            "color-handle docs page should keep copy-paste-ready token `{required}`.",
        );
    }
}

#[test]
fn color_handle_docs_matrices_and_api_defaults_stay_in_sync_with_view_contract() {
    let docs_source = load_source("docs_forms_color");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let check2_source = load_source("check2");

    for required in [
        "Playground title=\"State Matrix\" code_signal=state_matrix_code",
        "data-slot=\"color-handle-state-matrix\"",
        "title=\"Parameter Matrix Workbench (Display + Config + Code + CSS Test)\"",
        "data-slot=\"color-handle-workbench-controls\"",
        "data-parameter-matrix=\"color-handle\"",
        "data-slot=\"color-handle-api-defaults\"",
        "is_disabled=workbench_disabled.get()",
        "is_focused=workbench_focused.get()",
        "is_dragging=workbench_dragging.get()",
        "is_loupe_visible=workbench_show_loupe.get()",
        "x_percent=workbench_x_percent.get()",
        "y_percent=workbench_y_percent.get()",
        "motion=motion",
    ] {
        assert!(
            docs_source.contains(required),
            "color-handle docs matrices should keep sync marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] is_disabled: Option<bool>,",
        "#[prop(optional, into)] is_focused: Option<bool>,",
        "#[prop(optional, into)] is_dragging: Option<bool>,",
        "#[prop(optional, into)] is_loupe_visible: Option<bool>,",
        "#[prop(optional, into)] x_percent: Option<f32>,",
        "#[prop(optional, into)] y_percent: Option<f32>,",
        "#[prop(optional, into)] motion: Option<ColorHandleMotion>,",
        "let props = logic::resolve_props(logic::ColorHandlePropsInput {",
    ] {
        assert!(
            view_source.contains(required),
            "color-handle view contract should keep API/default token `{required}`.",
        );
    }

    for required in [
        "pub const DEFAULT_IS_DISABLED: bool = false;",
        "pub const DEFAULT_IS_FOCUSED: bool = false;",
        "pub const DEFAULT_IS_DRAGGING: bool = false;",
        "pub const DEFAULT_IS_LOUPE_VISIBLE: bool = true;",
        "pub const DEFAULT_X_PERCENT: f32 = 50.0;",
        "pub const DEFAULT_Y_PERCENT: f32 = 50.0;",
        "pub fn resolve_props(input: ColorHandlePropsInput) -> ColorHandleResolvedProps {",
    ] {
        assert!(
            logic_source.contains(required),
            "color-handle logic contract should keep centralized default token `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "color-handle check2 should mark docs matrices sync item complete.",
    );
}

#[test]
fn color_handle_readme_is_beginner_friendly_and_progressive() {
    let readme_source = load_source("readme");
    let check2_source = load_source("check2");

    for required in [
        "## 先用起来（Hello World）",
        "默认路径只需要 `id_base + color`",
        "use ui_components::ColorHandle;",
        "<ColorHandle id_base=\"demo-color-handle\"",
        "## 常见用法（先基础，后进阶）",
        "基础状态切换（最常用）",
        "进阶参数（按需开启）",
    ] {
        assert!(
            readme_source.contains(required),
            "color-handle README should keep beginner-friendly token `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "color-handle check2 should mark beginner-friendly docs item complete.",
    );
}

#[test]
fn color_handle_e2e_contract_uses_semantic_selectors_and_wasm_safe_waits() {
    let docs_source = load_source("docs_forms_color");
    let e2e_source = load_source("e2e_color_handle_contract");
    let check2_source = load_source("check2");

    for required in [
        "data-slot=\"color-handle-workbench-controls\"",
        "data-slot=\"color-handle-workbench-disabled\"",
        "data-slot=\"color-handle-workbench-focused\"",
        "data-slot=\"color-handle-workbench-dragging\"",
        "data-slot=\"color-handle-workbench-show-loupe\"",
        "data-slot=\"color-handle-workbench-motion\"",
    ] {
        assert!(
            docs_source.contains(required),
            "color-handle docs controls should expose stable e2e marker `{required}`.",
        );
    }

    for required in [
        "body:not(:has(#boot))",
        "#docs-color-handle-workbench[data-slot=\"color-handle\"]",
        "[data-slot=\"color-handle-workbench-controls\"]",
        "[data-slot=\"color-handle-workbench-dragging\"]",
        "[data-slot=\"color-handle-workbench-motion\"]",
        "toHaveAttribute(\"data-state\", \"dragging\")",
        "toHaveAttribute(\"data-ui-action\", \"drag-update\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "toHaveAttribute(\"style\", /--ui-color-handle-motion-duration:\\s*420ms;/)",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-handle e2e contract should keep semantic ready/settled token `{required}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "color-handle e2e should not use fixed-sleep wait `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "color-handle check2 should mark e2e selector stability item complete.",
    );
}

#[test]
fn color_handle_e2e_key_flow_is_repeatable_and_semantic_breakpointed() {
    let e2e_source = load_source("e2e_color_handle_contract");
    let check2_source = load_source("check2");

    for required in [
        "docs-app color-handle flow uses semantic settled conditions for motion/drag state",
        "toHaveAttribute(\"data-state\", \"dragging\")",
        "toHaveAttribute(\"data-ui-action\", \"drag-update\")",
        "toHaveAttribute(\"data-ui-source\", \"drag-interaction\")",
        "toHaveAttribute(\"data-state\", \"focused\")",
        "toHaveAttribute(\"data-state\", \"color\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-handle e2e key-flow regression should keep semantic breakpoint token `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "color-handle check2 should mark key-flow regression item complete.",
    );
}

#[test]
fn color_handle_docs_interactive_playground_supports_live_preview_and_replayable_flow() {
    let docs_source = load_source("docs_forms_color");
    let e2e_source = load_source("e2e_color_handle_contract");
    let check2_source = load_source("check2");

    for required in [
        "title=\"Parameter Matrix Workbench (Display + Config + Code + CSS Test)\"",
        "code_signal=workbench_code",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"color-handle-workbench-controls\"",
        "data-parameter-matrix=\"color-handle\"",
        "data-slot=\"color-handle-workbench-color\"",
        "data-slot=\"color-handle-workbench-x\"",
        "data-slot=\"color-handle-workbench-y\"",
        "data-slot=\"color-handle-workbench-disabled\"",
        "data-slot=\"color-handle-workbench-focused\"",
        "data-slot=\"color-handle-workbench-dragging\"",
        "data-slot=\"color-handle-workbench-show-loupe\"",
        "data-slot=\"color-handle-workbench-motion\"",
    ] {
        assert!(
            docs_source.contains(required),
            "color-handle docs interactive playground should keep live-config token `{required}`.",
        );
    }

    for required in [
        "const controls = component.locator('[data-slot=\"color-handle-workbench-controls\"]').first();",
        "const draggingToggle = controls.locator('[data-slot=\"color-handle-workbench-dragging\"]').first();",
        "const motionRange = controls.locator('[data-slot=\"color-handle-workbench-motion\"]').first();",
        "await draggingToggle.check();",
        "await motionRange.fill(\"420\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-handle e2e replay should keep interactive-workbench token `{required}`.",
        );
    }

    assert!(
        check2_source
            .contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "color-handle check2 should mark docs interactive playground item complete.",
    );
}

#[test]
fn color_handle_source_first_docs_are_copy_paste_ready_and_synced() {
    let docs_source = load_source("docs_forms_color");
    let e2e_source = load_source("e2e_color_handle_contract");
    let check2_source = load_source("check2");

    for required in [
        "data-slot=\"color-handle-copy-ready\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "components/color-handle/src/view.rs",
        "components/color-handle/src/logic.rs",
        "components/color-handle/src/styles.rs",
    ] {
        assert!(
            docs_source.contains(required),
            "color-handle docs source-first card should keep token `{required}`.",
        );
    }

    for required in [
        "docs-app color-handle playground source is copy-paste ready",
        "getByRole(\"button\", { name: /Show code|Hide code/ })",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toContainText(\"use leptos::prelude::*;\")",
        "toContainText(\"use ui_components::*;\")",
        "toContainText(\"<ColorHandle\")",
        "toHaveAttribute(\"aria-label\", /Copy to clipboard/i)",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-handle copy-ready e2e should keep token `{required}`.",
        );
    }

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "color-handle check2 should mark source-first copy-ready item complete.",
    );
}

#[test]
fn color_handle_heroui_strategy_and_docs_entry_stay_synchronized() {
    let heroui_spec_source = load_source("heroui_spec");
    let docs_catalog_source = load_source("docs_pages_catalog");
    let docs_source = load_source("docs_forms_color");
    let check2_source = load_source("check2");

    for required in [
        "### ColorHandle 同步记录（2026-02-20）",
        "参数模型同步：`ColorHandle` 参数主轴保持",
        "docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!(\"ColorHandle\", \"color-handle\", \"Forms\", forms_color::color_handle)` 暴露入口；`#/components/color-handle` 可索引访问。",
        "研究文档补充判定：本轮仅为 ColorHandle 参数模型与文档入口对齐，未引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
    ] {
        assert!(
            heroui_spec_source.contains(required),
            "color-handle HeroUI strategy doc should keep sync token `{required}`.",
        );
    }

    for required in [
        "component_doc!(",
        "\"color-handle\"",
        "\"ColorHandle\"",
        "\"Forms\"",
        "forms_color::color_handle",
    ] {
        assert!(
            docs_catalog_source.contains(required),
            "color-handle docs catalog should keep entry token `{required}`.",
        );
    }

    assert!(
        docs_source.contains("pub(super) fn color_handle() -> AnyView {"),
        "color-handle docs page should remain reachable from docs catalog entry.",
    );

    assert!(
        check2_source.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "color-handle check2 should mark HeroUI strategy/doc sync item complete.",
    );
}

#[test]
fn color_handle_default_values_are_normalized_in_logic_only() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let check2_source = load_source("check2");

    for required in [
        "pub struct ColorHandlePropsInput {",
        "pub struct ColorHandleResolvedProps {",
        "pub fn resolve_props(input: ColorHandlePropsInput) -> ColorHandleResolvedProps {",
        "input.is_loupe_visible.unwrap_or(DEFAULT_IS_LOUPE_VISIBLE)",
        "input.x_percent.unwrap_or(DEFAULT_X_PERCENT)",
        "input.y_percent.unwrap_or(DEFAULT_Y_PERCENT)",
        "input.motion.unwrap_or_default()",
    ] {
        assert!(
            logic_source.contains(required),
            "color-handle logic should keep centralized default-normalization token `{required}`.",
        );
    }

    for forbidden in [
        "#[prop(optional, default =",
        "unwrap_or_default()",
        "unwrap_or(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-handle view should not keep fallback/default token `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。"),
        "color-handle check2 should mark single-source defaults item complete.",
    );
}

#[test]
fn color_handle_discrete_status_axis_is_type_constrained_with_enum() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let check2_source = load_source("check2");

    for required in [
        "pub enum ColorHandleInteractionState {",
        "pub const fn from_flags(is_disabled: bool, is_focused: bool, is_dragging: bool) -> Self {",
        "pub const fn is_disabled(self) -> bool {",
        "pub const fn is_focused(self) -> bool {",
        "pub const fn is_dragging(self) -> bool {",
        "interaction_state: ColorHandleInteractionState::from_flags(",
    ] {
        assert!(
            logic_source.contains(required),
            "color-handle logic should keep typed discrete-state token `{required}`.",
        );
    }

    for required in [
        "is_disabled: props.interaction_state.is_disabled(),",
        "is_focused: props.interaction_state.is_focused(),",
        "is_dragging: props.interaction_state.is_dragging(),",
    ] {
        assert!(
            view_source.contains(required),
            "color-handle view should consume typed interaction-state token `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。"),
        "color-handle check2 should mark discrete-state type-constraint item complete.",
    );
}
