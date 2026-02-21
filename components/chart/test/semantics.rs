use std::{fs, path::Path};

fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "check2" => include_str!("../check2.md"),
        "readme" => include_str!("../src/README.md"),
        "component_manifest" => include_str!("../Component.toml"),
        "component_rbi" => include_str!("../Component.rbi"),
        "docs_display_extra" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/display_extra.rs")
        }
        "docs_components_pages" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "docs_components_shell" => {
            include_str!("../../../apps/docs-app/src/pages/components/shell.rs")
        }
        "docs_perf_probe" => include_str!("../../../apps/docs-app/src/perf_probe.rs"),
        "docs_playground" => include_str!("../../../apps/docs-app/src/playground.rs"),
        "docs_components_coverage_e2e" => {
            include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs")
        }
        "docs_theme_visual_baseline_page" => {
            include_str!(
                "../../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs"
            )
        }
        "docs_theme_visual_baseline_e2e" => {
            include_str!("../../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs")
        }
        "docs_heroui_strategy" => {
            include_str!("../../../docs/spec/heroui-parameter-design-strategy.md")
        }
        "state_primitives_chart" => {
            include_str!("../../../crates/ui-state-primitives/src/chart.rs")
        }
        "visual_active_highlight" => {
            include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs")
        }
        "ui_components_css" => include_str!("../../../crates/ui-components/src/css.rs"),
        "ui_components_root" => include_str!("../../../crates/ui-components/src/root.rs"),
        "headless_controllable_state" => {
            include_str!("../../../crates/ui-headless/src/controllable_state.rs")
        }
        "headless_controllable_state_tests" => {
            include_str!("../../../crates/ui-headless/src/test/controllable_state.rs")
        }
        "headless_id_provider" => include_str!("../../../crates/ui-headless/src/id_provider.rs"),
        "headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "platform_check_script" => {
            include_str!("../../../scripts/check-ui-components-platforms.sh")
        }
        "performance_check_script" => {
            include_str!("../../../scripts/check-ui-components-performance.sh")
        }
        "view_macro_check_script" => {
            include_str!("../../../scripts/check-ui-components-view-macro.sh")
        }
        "inner_html_check_script" => {
            include_str!("../../../scripts/check-ui-components-inner-html.sh")
        }
        "dx_check_script" => include_str!("../../../scripts/check-ui-components-dx.sh"),
        "contract_hygiene_check_script" => {
            include_str!("../../../scripts/check-ui-components-contract-hygiene.sh")
        }
        "streaming_check_script" => {
            include_str!("../../../scripts/check-ui-components-streaming.sh")
        }
        "engineering_check_script" => {
            include_str!("../../../scripts/check-ui-components-engineering.sh")
        }
        "component_files_check_script" => {
            include_str!("../../../scripts/check-ui-components-component-files.sh")
        }
        "entrypoints_check_script" => {
            include_str!("../../../scripts/check-ui-components-entrypoints.sh")
        }
        "todo" => include_str!("../../../docs/plan/TODO.md"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "ui_motion_non_wasm_stub_tests" => {
            include_str!("../../../crates/ui-motion/tests/non_wasm_stub.rs")
        }
        "ui_motion_spring" => include_str!("../../../crates/ui-motion/src/spring.rs"),
        "ui_motion_web" => include_str!("../../../crates/ui-motion/src/web.rs"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn chart_module_exports_stable_ui_components_surface_without_dom_details() {
    let module = load_source("mod");

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ChartKind, ChartPoint, DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE};",
        "pub use motion::ChartMotion;",
        "pub use view::Chart;",
    ] {
        assert!(
            module.contains(required),
            "chart module should keep stable export `{required}`",
        );
    }

    for forbidden in ["web_sys", "web-sys", "NodeRef", "HtmlElement", "Element"] {
        assert!(
            !module.contains(forbidden),
            "chart public module should not expose DOM detail `{forbidden}`",
        );
    }
}

#[test]
fn chart_semantics_tests_are_migrated_to_component_local_test_directory() {
    let module = load_source("mod");

    assert!(
        module.contains("#[path = \"../test/semantics.rs\"]")
            && module.contains("mod semantics_tests;"),
        "chart should wire component-local semantics tests from components/chart/test/semantics.rs",
    );
}

#[test]
fn chart_layer_files_follow_logic_view_style_motion_boundaries() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");

    assert!(
        logic.contains("pub use ui_state_primitives::chart::{"),
        "logic.rs should only consume ui-state-primitives chart contract.",
    );
    for required in [
        "pub struct ChartInputBoundary",
        "pub fn normalize_input_boundary(",
        "pub struct ChartStateBoundary",
        "pub fn derive_state_from_boundary(",
        "pub fn normalize_interaction_index(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should centralize chart normalization helper `{required}`.",
        );
    }
    assert!(
        !logic.contains("view!"),
        "logic.rs should not render view structure.",
    );

    for required in [
        "let normalized = logic::normalize_input_boundary(logic::ChartInputBoundary {",
        "let state = Signal::derive(move || {",
        "logic::derive_state_from_boundary(logic::ChartStateBoundary {",
        "use_chart(ChartOptions {",
        "logic::normalize_interaction_index(index, point_count, is_disabled)",
        "data-kind=move || semantics.get().attrs.data_kind",
        "handlers.on_key_down(",
        "motion::attach_motion(",
    ] {
        assert!(
            view.contains(required),
            "view.rs should keep assembly-only responsibility via `{required}`",
        );
    }

    for forbidden in [
        "pub struct ChartStateInput",
        "pub struct ChartState",
        "pub fn resolve_state(",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not redefine state primitive `{forbidden}`",
        );
    }

    assert!(
        styles.contains("pub const CSS: &str =")
            && styles.contains("var(--ui-")
            && !styles.contains("on:click"),
        "styles.rs should stay token-first static CSS without event logic.",
    );

    for required in [
        "pub type ChartMotion =",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "ui_motion::spring::sanitize_config",
    ] {
        assert!(
            motion.contains(required),
            "motion.rs should map semantic state to shared motion runtime via `{required}`",
        );
    }

    for forbidden in ["aria-", "role=", "on:click", "on:keydown"] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs must not encode A11y/event semantics `{forbidden}`",
        );
    }
}

#[test]
fn chart_api_naming_uses_is_on_default_prefixes_without_alias_drift() {
    let view = load_source("view");

    for required in [
        "on_active_index_change: Option<Callback<usize>>",
        "on_action: Option<Callback<String>>",
        "is_disabled: bool",
        "is_show_grid: bool",
        "default_active_index: Option<usize>",
    ] {
        assert!(
            view.contains(required),
            "chart api should contain stable naming contract `{required}`",
        );
    }

    for forbidden in [
        "disabled: bool",
        "is_disabled.unwrap_or(disabled)",
        "show_grid: bool",
    ] {
        assert!(
            !view.contains(forbidden),
            "chart api should not keep alias drift `{forbidden}`",
        );
    }
}

#[test]
fn chart_active_index_control_contract_is_paired_and_single_sourced() {
    let view = load_source("view");
    let headless = load_source("headless_controllable_state");
    let headless_tests = load_source("headless_controllable_state_tests");

    for required in [
        "active_index: Option<Signal<usize>>",
        "default_active_index: Option<usize>",
        "on_active_index_change: Option<Callback<usize>>",
        "let is_controlled = active_index.is_some();",
        "let active_state = use_controllable_state(",
        "active_index,",
        "Some(default_active_index),",
        "on_active_index_change,",
        "let active_index = active_state.value;",
        "let request_active_index_change = active_state.request_change;",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-uncontrolled=move || semantics.get().attrs.data_uncontrolled",
    ] {
        assert!(
            view.contains(required),
            "chart controlled/uncontrolled contract should include `{required}`",
        );
    }

    for forbidden in ["set_active_index.set(", "signal(default_active_index)"] {
        assert!(
            !view.contains(forbidden),
            "chart view must not create parallel active-index source `{forbidden}`",
        );
    }

    for required in [
        "let (uncontrolled_value, set_uncontrolled_value) = signal(default_value.unwrap_or_default());",
        "let is_controlled = value.is_some();",
        "let value = value.unwrap_or(uncontrolled_value.into());",
        "if !is_controlled {",
        "set_uncontrolled_value.set(next);",
    ] {
        assert!(
            headless.contains(required),
            "headless controllable primitive should enforce `{required}`",
        );
    }

    for required in [
        "fn controlled_open_does_not_update_internal_state()",
        "fn controlled_open_ignores_default_open_value()",
        "fn controlled_state_without_on_change_is_read_only()",
    ] {
        assert!(
            headless_tests.contains(required),
            "headless regression coverage should include `{required}`",
        );
    }
}

#[test]
fn chart_default_value_priority_is_normalized_via_logic_only() {
    let view = load_source("view");
    let logic = load_source("logic");

    assert!(
        logic.contains("pub fn normalize_input_boundary(")
            && logic.contains("default_active_index(point_count, input.default_active_index)"),
        "logic.rs should centralize default_active_index normalization in input boundary helper.",
    );

    assert!(
        view.contains(
            "let normalized = logic::normalize_input_boundary(logic::ChartInputBoundary {"
        ) && view.contains("let default_active_index = normalized.default_active_index;"),
        "view.rs should consume normalized default via logic helper.",
    );

    for forbidden in [
        "default_active_index.unwrap_or",
        "active_index.unwrap_or",
        "is_show_grid.unwrap_or",
        "if default_active_index.is_none()",
        "match default_active_index",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs must not introduce local default fallback branch `{forbidden}`",
        );
    }
}

#[test]
fn chart_state_normalization_is_concentrated_in_logic_layer() {
    let view = load_source("view");
    let logic = load_source("logic");
    let styles = load_source("styles");

    for required in [
        "normalize_input_boundary(",
        "derive_state_from_boundary(",
        "normalize_interaction_index(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should provide centralized normalization helper `{required}`",
        );
    }

    for required in [
        "logic::normalize_input_boundary(logic::ChartInputBoundary {",
        "logic::derive_state_from_boundary(logic::ChartStateBoundary {",
        "logic::normalize_interaction_index(index, point_count, is_disabled)",
    ] {
        assert!(
            view.contains(required),
            "view.rs should consume logic normalization helper `{required}`",
        );
    }

    for forbidden in [
        "logic::normalize_points(points).into()",
        "logic::value_domain(points.as_ref())",
        "logic::resolve_state(ChartStateInput {",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs must not rebuild normalization branch `{forbidden}`",
        );
    }

    assert!(
        !styles.contains("ChartStateInput")
            && !styles.contains("normalize_input_boundary")
            && !styles.contains("derive_state_from_boundary"),
        "styles.rs should only consume state markers, not normalization logic.",
    );
}

#[test]
fn chart_discrete_state_axes_are_type_constrained() {
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "#[prop(optional)] kind: ChartKind,",
        "state.get().kind == ChartKind::Line",
        "pub use ui_state_primitives::chart::{",
        "ChartKind,",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "chart discrete state contract should include typed axis `{required}`",
        );
    }

    for forbidden in [
        "kind: Option<String>",
        "kind: String",
        "mode: Option<String>",
        "status: Option<String>",
        "variant: Option<String>",
        "if is_bar && is_line",
        "Option<bool>",
    ] {
        assert!(
            !view.contains(forbidden),
            "chart view should not model mutually exclusive state via `{forbidden}`",
        );
    }

    for forbidden in ["from_str(", ".parse::<ChartKind>()", "match kind.as_str()"] {
        assert!(
            !logic.contains(forbidden),
            "chart logic should not accept free-form string state via `{forbidden}`",
        );
    }
}

#[test]
fn chart_state_primitives_source_is_ui_state_primitives_only() {
    let logic = load_source("logic");
    let view = load_source("view");

    assert!(
        logic.contains("pub use ui_state_primitives::chart::{"),
        "logic.rs should source chart primitives only from ui-state-primitives.",
    );

    for forbidden in [
        "use crate::store",
        "use crate::app_state",
        "use app_state::",
        "use redux",
        "use pinia",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "chart component should not bind business store contract `{forbidden}`",
        );
    }
}

#[test]
fn chart_has_no_async_interaction_contract_and_requires_no_loading_protocol() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "async fn ",
        "Future<",
        "Resource<",
        "use_async_action",
        "spawn_local",
        "spawn(",
        "is_loading",
        "on_retry",
        "error:",
        "aria-busy",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "chart should remain N/A for async loading protocol and not contain `{forbidden}`",
        );
    }
}

#[test]
fn chart_dx_paradox_keeps_simple_default_api_and_short_hello_world() {
    let view = load_source("view");
    let readme = load_source("readme");
    let docs = load_source("docs_display_extra");

    assert!(
        view.contains("points: Vec<ChartPoint>,"),
        "chart default usage should only require `points` input.",
    );

    for forbidden in ["#[prop(optional)] state:", "state: Signal<", "state="] {
        assert!(
            !view.contains(forbidden),
            "chart public api should not require internal state wiring `{forbidden}`",
        );
    }

    let hello_world_one_line = "<Chart points=vec![ChartPoint::new(\"jan\", \"Jan\", 12.0), ChartPoint::new(\"feb\", \"Feb\", 18.5), ChartPoint::new(\"mar\", \"Mar\", 17.2)] />";
    assert!(
        readme.contains(hello_world_one_line),
        "README hello world should stay within 5 lines and be copy-paste ready.",
    );
    assert!(
        docs.contains("title=\"Hello World\"") && docs.contains(hello_world_one_line),
        "docs-app should keep a short hello-world default path.",
    );
}

#[test]
fn chart_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("docs_playground");

    for required in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            source.contains(required),
            "playground should keep CSS hot-reload contract marker `{required}`.",
        );
    }
}

#[test]
fn chart_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let source = load_source("docs_display_extra");

    for required in [
        "CHART_WORKBENCH_STORAGE_KEY",
        "fn load_chart_workbench_state() -> Option<ChartWorkbenchState>",
        "fn save_chart_workbench_state(state: ChartWorkbenchState)",
        "fn clear_chart_workbench_state()",
        "description=\"Workbench canvas: scoped CSS live-edit + optional state persistence across reload.\"",
        "test_css_source=chart_test_css_source",
        "test_config_signal=workbench_config",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "\"Persist workbench state\"",
        "Effect::new(move |_| {",
        "save_chart_workbench_state(ChartWorkbenchState {",
        "clear_chart_workbench_state();",
        "data-slot=\"chart-workbench\"",
        "data-slot=\"chart-workbench-canvas\"",
    ] {
        assert!(
            source.contains(required),
            "chart workbench should keep DX marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(required),
            "chart workbench persistence should keep platform guard `{required}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("dx_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce `{required}`.",
        );
    }
}

#[test]
fn chart_docs_product_copy_paste_ready_contract_is_documented_and_scripted_locally() {
    let docs_display = load_source("docs_display_extra");
    let playground_source = load_source("docs_playground");
    let code_block_view = include_str!("../../../components/code-block/src/view.rs");
    let script_source = load_source("dx_check_script");
    let check2_source = load_source("check2");

    for required in [
        "pub(super) fn chart() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Comparison Matrix (Bar / Line / Disabled / Empty)\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "let chart_imports =",
        "use ui_components::{Chart, ChartKind, ChartPoint};",
        "code_imports=chart_imports.clone()",
        "data-slot=\"chart-streaming-policy\"",
        "Streaming Optional; fallback=snapshot.",
        "data-slot=\"chart-source-first\"",
        "data-slot=\"chart-copy-ready-hint\"",
        "data-slot=\"chart-source-paths\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
    ] {
        assert!(
            docs_display.contains(required),
            "chart docs-product surface should include `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should keep `{required}`.",
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view.contains(required),
            "CodeBlock one-click copy affordance should keep `{required}`.",
        );
    }

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_docs_product_copy_paste_ready_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_docs_product_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(required),
            "DX gate script should include docs-product command `{required}`.",
        );
    }

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "chart_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "chart_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "chart_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "chart_docs_product_copy_paste_ready_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 docs-product section should include `{required}`.",
        );
    }
}

#[test]
fn chart_check2_documents_docs_sync_and_state_matrix_rules_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 docs-sync/state-matrix section should include `{required}`.",
        );
    }
}

#[test]
fn chart_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults_locally() {
    let docs_source = load_source("docs_display_extra");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let primitive_source = load_source("state_primitives_chart");

    for required in [
        "title=\"Hello World\"",
        "title=\"Comparison Matrix (Bar / Line / Disabled / Empty)\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "data-slot=\"chart-parameter-matrix\"",
        "Parameter Matrix (API Names + Defaults)",
        "data-slot=\"chart-state-matrix-summary\"",
        "Size/variant: N/A for Chart",
        "id_base=\"docs-chart-matrix-disabled\".to_string()",
        "is_disabled=true",
        "active_index=controlled_active.clone()",
        "on_active_index_change=on_controlled_active_change.clone()",
        "kind=ChartKind::Line",
    ] {
        assert!(
            docs_source.contains(required),
            "chart docs examples/state matrix should keep marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] kind: ChartKind,",
        "#[prop(optional)] default_active_index: Option<usize>,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional, default = true)] is_show_grid: bool,",
        "#[prop(optional, into)] id_base: Option<String>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
    ] {
        assert!(
            view_source.contains(required),
            "chart view public API should keep `{required}` for docs/runtime sync.",
        );
    }

    for required in [
        "pub fn normalize_input_boundary(input: ChartInputBoundary) -> ChartNormalizedInput",
        "let default_active_index = default_active_index(point_count, input.default_active_index);",
        "pub const DEFAULT_ID_BASE: &str = \"ui-chart\";",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Chart\";",
        "pub fn default_active_index(point_count: usize, requested: Option<usize>) -> usize",
        "requested.unwrap_or(0)",
    ] {
        assert!(
            logic_source.contains(required) || primitive_source.contains(required),
            "chart logic/default source should keep `{required}` for docs consistency.",
        );
    }

    for forbidden in [
        "default_kind=",
        "on_active_change=",
        "is_open=",
        "default_is_open",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "chart docs should avoid stale aliased API token `{forbidden}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_docs_sync_and_state_matrix_contract_locally() {
    let script_source = load_source("dx_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_docs_sync_and_state_matrix_contract",
    ] {
        assert!(
            script_source.contains(required),
            "dx check script should include docs-sync/state-matrix command `{required}`.",
        );
    }
}

#[test]
fn chart_check2_marks_docs_sync_and_state_matrix_contract_complete_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "components/chart/test/semantics.rs::chart_check2_documents_docs_sync_and_state_matrix_rules_locally",
        "components/chart/test/semantics.rs::chart_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_docs_sync_and_state_matrix_contract_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_docs_sync_and_state_matrix_rules",
        "crates/ui-components/tests/chart_semantics.rs::chart_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_docs_sync_and_state_matrix_contract_complete",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 docs-sync/state-matrix section should reference `{required}`.",
        );
    }
}

#[test]
fn chart_check2_documents_documentation_as_product_rules_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 documentation-as-product section should include `{required}`.",
        );
    }
}

#[test]
fn chart_documentation_entry_exists_with_beginner_first_progression_locally() {
    let readme = load_source("readme");
    let docs_source = load_source("docs_display_extra");

    for required in [
        "# Chart",
        "## Hello World（最小可用）",
        "<Chart points=vec![ChartPoint::new(\"jan\", \"Jan\", 12.0), ChartPoint::new(\"feb\", \"Feb\", 18.5), ChartPoint::new(\"mar\", \"Mar\", 17.2)] />",
        "先传 `points` 即可运行，后续再按需开启受控、动作、动效等高级参数。",
        "## 常见用法",
        "## 再进阶（受控 + 语义 + 动效）",
        "apps/docs-app/src/pages/components/pages/display_extra.rs",
    ] {
        assert!(
            readme.contains(required),
            "chart README beginner-first documentation should include `{required}`.",
        );
    }

    for required in [
        "pub(super) fn chart() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            docs_source.contains(required),
            "chart docs-app entry should include `{required}`.",
        );
    }

    let hello_pos = docs_source
        .find("title=\"Hello World\"")
        .expect("chart docs should include Hello World playground");
    let controlled_pos = docs_source
        .find("title=\"Controlled vs Uncontrolled Contrast\"")
        .expect("chart docs should include controlled/uncontrolled playground");
    assert!(
        hello_pos < controlled_pos,
        "chart docs should keep beginner Hello World path before advanced controlled path",
    );

    for forbidden in [
        "必须先理解 ui-state-primitives",
        "必须先理解 ui-headless",
        "Only source code is provided",
    ] {
        assert!(
            !readme.contains(forbidden) && !docs_source.contains(forbidden),
            "chart documentation should avoid architecture-first barrier `{forbidden}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_documentation_as_product_contract_locally() {
    let script_source = load_source("dx_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_documentation_entry_exists_with_beginner_first_progression",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_documentation_as_product_contract",
    ] {
        assert!(
            script_source.contains(required),
            "dx check script should include documentation-as-product command `{required}`.",
        );
    }
}

#[test]
fn chart_check2_marks_documentation_as_product_contract_complete_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "components/chart/test/semantics.rs::chart_check2_documents_documentation_as_product_rules_locally",
        "components/chart/test/semantics.rs::chart_documentation_entry_exists_with_beginner_first_progression_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_documentation_as_product_contract_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_documentation_as_product_rules",
        "crates/ui-components/tests/chart_semantics.rs::chart_documentation_entry_exists_with_beginner_first_progression",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_documentation_as_product_contract_complete",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 documentation-as-product section should reference `{required}`.",
        );
    }
}

#[test]
fn chart_check2_documents_interactive_playground_rules_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 interactive-playground section should include `{required}`.",
        );
    }
}

#[test]
fn chart_docs_app_provides_interactive_playground_for_props_state_and_preview_locally() {
    let docs_source = load_source("docs_display_extra");

    for required in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "description=\"Workbench canvas: scoped CSS live-edit + optional state persistence across reload.\"",
        "id_base=\"docs-chart-kind\".to_string()",
        "id_base=\"docs-chart-dataset\".to_string()",
        "<Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>",
        "<Switch checked=workbench_is_show_grid set_checked=set_workbench_is_show_grid>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_lang set_checked=set_workbench_lang>",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "data-slot=\"chart-workbench\"",
        "data-slot=\"chart-workbench-canvas\"",
        "\"workbench last action: \"",
        "test_config_signal=workbench_config",
        "test_css_source=chart_test_css_source",
        "code_signal=workbench_code",
    ] {
        assert!(
            docs_source.contains(required),
            "chart docs interactive playground should include `{required}`.",
        );
    }
}

#[test]
fn chart_interactive_playground_reuses_repeatable_semantic_e2e_flow_locally() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_chart_contract.spec.mjs");
    let docs_source = load_source("docs_display_extra");

    for required in [
        "docs-app chart key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "[data-slot=\"chart-workbench-canvas\"] [data-slot=\"chart\"]",
        "chart-workbench-toggle-disabled",
        "await page.keyboard.press(\"ArrowRight\");",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"ready\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "chart interactive playground e2e flow should include `{required}`.",
        );
    }

    for required in [
        "data-slot=\"chart-workbench-canvas\"",
        "data-slot=\"chart-workbench-toggle-disabled\"",
        "data-slot=\"chart-e2e-controlled-line\"",
    ] {
        assert!(
            docs_source.contains(required),
            "chart docs should expose interactive/e2e semantic anchor `{required}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "chart interactive e2e flow should avoid brittle wait `{forbidden}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_interactive_playground_contract_locally() {
    let script_source = load_source("dx_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(required),
            "dx check script should include interactive-playground command `{required}`.",
        );
    }
}

#[test]
fn chart_check2_marks_interactive_playground_contract_complete_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "components/chart/test/semantics.rs::chart_check2_documents_interactive_playground_rules_locally",
        "components/chart/test/semantics.rs::chart_docs_app_provides_interactive_playground_for_props_state_and_preview_locally",
        "components/chart/test/semantics.rs::chart_interactive_playground_reuses_repeatable_semantic_e2e_flow_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_interactive_playground_contract_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_interactive_playground_rules",
        "crates/ui-components/tests/chart_semantics.rs::chart_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "crates/ui-components/tests/chart_semantics.rs::chart_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_interactive_playground_contract_complete",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 interactive-playground section should reference `{required}`.",
        );
    }
}

#[test]
fn chart_check2_documents_source_first_copy_paste_ready_rules_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 source-first section should include `{required}`.",
        );
    }
}

#[test]
fn chart_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies_locally() {
    let docs_source = load_source("docs_display_extra");
    let playground_source = load_source("docs_playground");
    let code_block_view = include_str!("../../../components/code-block/src/view.rs");

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"chart-source-first\"",
        "data-slot=\"chart-copy-ready-hint\"",
        "data-slot=\"chart-source-paths\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "component-chart",
        "inject-css",
        "components/chart/src/mod.rs",
        "components/chart/src/logic.rs",
        "components/chart/src/view.rs",
        "components/chart/src/styles.rs",
        "components/chart/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(required),
            "chart source-first docs should include `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
        "data-slot=\"playground-toggle-code\"",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should include `{required}`.",
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view.contains(required),
            "CodeBlock should keep copy button marker `{required}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_source_first_copy_paste_ready_contract_locally() {
    let script_source = load_source("dx_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_source_first_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(required),
            "dx check script should include source-first command `{required}`.",
        );
    }
}

#[test]
fn chart_check2_marks_source_first_copy_paste_ready_contract_complete_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "components/chart/test/semantics.rs::chart_check2_documents_source_first_copy_paste_ready_rules_locally",
        "components/chart/test/semantics.rs::chart_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_source_first_copy_paste_ready_contract_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_source_first_copy_paste_ready_rules",
        "crates/ui-components/tests/chart_semantics.rs::chart_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_source_first_copy_paste_ready_contract_complete",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 source-first section should reference `{required}`.",
        );
    }
}

#[test]
fn chart_check2_documents_heroui_benchmark_docs_sync_rules_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 heroui benchmark docs-sync section should include `{required}`.",
        );
    }
}

#[test]
fn chart_heroui_strategy_and_component_docs_are_synchronized_and_indexable_locally() {
    let strategy_source = load_source("docs_heroui_strategy");
    let pages_source = load_source("docs_components_pages");
    let docs_source = load_source("docs_display_extra");
    let readme_source = load_source("readme");

    for required in [
        "### Chart 同步记录（2026-02-20）",
        "参数模型同步：`Chart` 维持 display data-visual primitive 定位",
        "component_doc!(\"Chart\", \"chart\", \"Display\", display_extra::chart)",
        "#/components/chart",
        "`components/chart/src/README.md` 提供等价组件文档入口",
        "display_extra.rs::chart()",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(required),
            "chart heroui strategy doc should include synchronization marker `{required}`.",
        );
    }

    for required in [
        "component_doc!(",
        "\"Chart\"",
        "\"chart\"",
        "display_extra::chart",
    ] {
        assert!(
            pages_source.contains(required),
            "chart docs index should expose marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn chart() -> AnyView {",
        "title=\"Chart\"",
        "slug=\"chart\"",
    ] {
        assert!(
            docs_source.contains(required),
            "chart docs-app page should remain indexable via `{required}`.",
        );
    }

    assert!(
        readme_source.contains("# Chart"),
        "chart README should remain an equivalent component doc entry.",
    );
}

#[test]
fn chart_dx_check_script_covers_heroui_benchmark_docs_sync_contract_locally() {
    let script_source = load_source("dx_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
    ] {
        assert!(
            script_source.contains(required),
            "dx check script should include heroui benchmark docs-sync command `{required}`.",
        );
    }
}

#[test]
fn chart_check2_marks_heroui_benchmark_docs_sync_contract_complete_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "components/chart/test/semantics.rs::chart_check2_documents_heroui_benchmark_docs_sync_rules_locally",
        "components/chart/test/semantics.rs::chart_heroui_strategy_and_component_docs_are_synchronized_and_indexable_locally",
        "components/chart/test/semantics.rs::chart_dx_check_script_covers_heroui_benchmark_docs_sync_contract_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_heroui_benchmark_docs_sync_contract_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_heroui_benchmark_docs_sync_rules",
        "crates/ui-components/tests/chart_semantics.rs::chart_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "crates/ui-components/tests/chart_semantics.rs::chart_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_heroui_benchmark_docs_sync_contract_complete",
        "scripts/check-ui-components-dx.sh",
        "docs/spec/heroui-parameter-design-strategy.md",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 heroui benchmark docs-sync section should reference `{required}`.",
        );
    }
}

#[test]
fn chart_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let cargo_source = include_str!("../../../crates/ui-components/Cargo.toml");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");
    let check2_source = load_source("check2");
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "chart should keep spec/serde path as N/A for simple component scope",
    );
    assert!(
        cargo_source.contains("component-chart = [\"dep:ui-chart\"]"),
        "component-chart feature should stay minimal and avoid serde fan-out",
    );
    assert!(
        !cargo_source.contains("component-chart = [\"dep:serde\"")
            && !cargo_source.contains("component-chart = [\"dep:serde_json\""),
        "chart should not opt into serde dependencies without schema contract",
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "chart engineering serde/spec N/A path should avoid `{forbidden}`",
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
            "chart checklist should keep engineering governance marker `{required}`",
        );
    }
}

#[test]
fn chart_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = include_str!("../../../crates/ui-components/Cargo.toml");
    let button_view_source = include_str!("../../button/src/view.rs");
    let combined = [
        load_source("mod"),
        load_source("logic"),
        load_source("view"),
        load_source("styles"),
        load_source("motion"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing marker `{required}`",
        );
    }

    assert!(
        !cargo_source.contains("chart-wasm-debug"),
        "chart should not define component-local tracing feature without local debug event contract",
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::chart::",
        "const CHART_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "chart should avoid ad-hoc tracing semantic drift token `{forbidden}`",
        );
    }
}

#[test]
fn chart_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");

    for source in [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
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
                "chart should not leak runtime marker `{forbidden}`",
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "chart module boundary should not leak web_sys types",
    );
}

#[test]
fn chart_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("engineering_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            script_source.contains(required),
            "engineering check script should enforce `{required}`",
        );
    }
}

#[test]
fn chart_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade_locally()
 {
    let check2_source = load_source("check2");
    let manifest_source = load_source("component_manifest");
    let rbi_source = load_source("component_rbi");
    let protocol_source = include_str!("../src/protocol.rs");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");

    for required in [
        "pub enum ChartComponentSchemaVersion {",
        "V1,",
        "pub struct ChartComponentSpec {",
        "pub schema_version: ChartComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(required),
            "chart protocol should keep v1 schema marker `{required}`",
        );
    }

    for required in [
        "schema = \"ui.chart.agent-contract/v1\"",
        "schema_version = \"data-ui-schema-version\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "chart Component.toml should keep v1 contract marker `{required}`",
        );
    }

    for required in [
        "agent_contract_schema \"ui.chart.agent-contract/v1\"",
        "streaming_policy {",
        "fallback: \"snapshot\"",
    ] {
        assert!(
            rbi_source.contains(required),
            "chart RBI should keep stable contract marker `{required}`",
        );
    }

    let combined = [
        mod_source,
        logic_source,
        view_source,
        styles_source,
        motion_source,
        protocol_source,
        manifest_source,
        rbi_source,
    ]
    .join("\n");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "SchemaRegistry",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "agent-contract/v2",
        "contract.v2",
    ] {
        assert!(
            !combined.contains(forbidden),
            "without major breaking upgrade, chart should not introduce migration marker `{forbidden}`",
        );
    }

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Chart` 变更未引入跨大版本 API 破坏升级",
        "schema = \"ui.chart.agent-contract/v1\"",
        "chart_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade_locally",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 version-migration section should include `{required}`",
        );
    }
}

#[test]
fn chart_version_deprecation_migration_script_covers_engineering_gate_locally() {
    let engineering_script = load_source("engineering_check_script");
    let marker = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        engineering_script.contains(marker),
        "engineering check script should enforce `{marker}`",
    );
}

#[test]
fn chart_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", ".unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "chart non-test source should forbid rust-hygiene anti-pattern `{forbidden}`",
        );
    }
}

#[test]
fn chart_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for forbidden in [
        "common.chart_aria_label.to_string()",
        "String::from(\"Chart\")",
        "\"ui-chart\".to_string()",
        "\"ui-chart--line\".to_string()",
        "\"ui-chart--bar\".to_string()",
        "\"ui-chart--disabled\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "chart string hotspot contract should avoid `{forbidden}`",
        );
    }
}

#[test]
fn chart_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = include_str!("../../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("engineering_check_script");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene gate script should enforce `{required}`",
        );
    }

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(required),
            "engineering check script should enforce `{required}`",
        );
    }
}

#[test]
fn chart_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "chart_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "chart_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "chart_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
        "RUST_HYGIENE_SCOPE=\"components/chart\"",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 rust-hygiene section should reference `{required}`",
        );
    }
}

#[test]
fn chart_composite_api_prefers_typed_item_spec_over_parallel_arrays() {
    let view = load_source("view");
    let primitives = load_source("state_primitives_chart");
    let readme = load_source("readme");

    assert!(
        view.contains("points: Vec<ChartPoint>,"),
        "chart should consume a typed item collection via `points`.",
    );
    for required in [
        "pub struct ChartPoint {",
        "pub id: String,",
        "pub label: String,",
        "pub value: f64,",
    ] {
        assert!(
            primitives.contains(required),
            "ChartPoint should bind item semantics in one struct via `{required}`",
        );
    }

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "panels: Vec<",
        "children: Children",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !view.contains(forbidden) && !readme.contains(forbidden),
            "chart api should not drift to parallel-array/slot convention `{forbidden}`",
        );
    }
}

#[test]
fn chart_macro_micro_dragging_contract_is_na_for_non_dragging_chart() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    for required in [
        "on:pointerenter=on_enter",
        "on:click=on_click",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view.contains(required),
            "chart should keep discrete interaction path `{required}`",
        );
    }

    for forbidden in [
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "DragStart",
        "DragEnd",
        "Action::DragEnd",
        "Dragging",
        "requestAnimationFrame",
        "raf",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "chart non-dragging contract must not contain `{forbidden}`",
        );
    }
}

#[test]
fn chart_two_pass_rendering_contract_is_na_for_logic_and_idempotent_in_visual_driver() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");
    let visual_driver = load_source("visual_active_highlight");

    for forbidden in [
        "getBoundingClientRect",
        "offset_top",
        "offset_height",
        "Rect",
        "Measure",
        "Rectification",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "chart view/logic should stay free of geometry measurement pipeline token `{forbidden}`",
        );
    }

    for required in [
        "attach_active_highlight_motion(",
        "sync_measured_layout(",
        "let unchanged =",
        "if unchanged {",
    ] {
        assert!(
            motion.contains("attach_motion(") && visual_driver.contains(required),
            "visual measurement should stay inside shared driver with convergence guard `{required}`",
        );
    }
}

#[test]
fn chart_registration_protocol_is_na_for_static_point_collection() {
    let view = load_source("view");
    let logic = load_source("logic");
    let headless_chart = include_str!("../../../crates/ui-headless/src/chart.rs");

    for required in [
        "points: Vec<ChartPoint>,",
        "let indices: StoredValue<Vec<usize>> = StoredValue::new((0..point_count).collect());",
    ] {
        assert!(
            view.contains(required),
            "chart should derive item order from static point collection via `{required}`",
        );
    }

    for forbidden in [
        "RegistrationContext",
        "Register(",
        "Unregister(",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !headless_chart.contains(forbidden),
            "chart should not implement dynamic registry protocol token `{forbidden}`",
        );
    }
}

#[test]
fn chart_slot_projection_policy_is_na_for_single_tree_component() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    for required in [
        "data-slot=\"chart\"",
        "data-slot=\"chart-plot\"",
        "data-slot=\"chart-legend\"",
    ] {
        assert!(
            view.contains(required),
            "chart should keep a single composed tree marker `{required}`",
        );
    }

    for forbidden in [
        "Children",
        "RenderMode",
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "NotifyVisible",
        "on_hidden",
        "on_visible",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "chart should not implement slot projection lifecycle token `{forbidden}`",
        );
    }
}

#[test]
fn chart_env_stream_contract_is_na_for_logic_and_isolated_in_visual_driver() {
    let view = load_source("view");
    let logic = load_source("logic");
    let visual_driver = load_source("visual_active_highlight");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "BreakpointChanged",
        "ColorSchemeChanged",
        "VisibilityChanged",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "chart logic/view should not consume raw env stream token `{forbidden}`",
        );
    }

    for required in ["ResizeObserver", "sync_measured_layout("] {
        assert!(
            visual_driver.contains(required),
            "visual driver should isolate env sampling via `{required}`",
        );
    }
}

#[test]
fn chart_event_light_cone_contract_is_na_without_bulk_selection_bus() {
    let view = load_source("view");
    let logic = load_source("logic");
    let headless_chart = include_str!("../../../crates/ui-headless/src/chart.rs");

    for required in [
        "let apply_headless_action = Callback::new(move |action: ChartKeyAction| match action {",
        "ChartKeyAction::MoveTo(next)",
        "ChartKeyAction::Activate(current)",
        "request_active_index_change.run(index);",
    ] {
        assert!(
            view.contains(required),
            "chart should keep single-point action path `{required}`",
        );
    }

    for forbidden in [
        "ContextBus",
        "SelectorSubscription",
        "SelectionState::All",
        "select_all",
        "bulk_select",
        "broadcast(",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !headless_chart.contains(forbidden),
            "chart should not include event-light-cone bulk protocol token `{forbidden}`",
        );
    }
}

#[test]
fn chart_causality_bus_contract_is_na_without_trace_id_chain() {
    let view = load_source("view");
    let logic = load_source("logic");
    let headless_chart = include_str!("../../../crates/ui-headless/src/chart.rs");

    for required in [
        "ChartKeyAction::MoveTo(next)",
        "ChartKeyAction::Activate(current)",
        "request_active_index_change.run(index);",
    ] {
        assert!(
            view.contains(required),
            "chart should keep local causal flow via `{required}`",
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "broadcast(",
        "subscribe(",
        "SignalBus",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !headless_chart.contains(forbidden),
            "chart should not expose cross-system causality bus token `{forbidden}`",
        );
    }
}

#[test]
fn chart_a11y_i18n_locale_contract_is_headless_driven_with_ui_root_injection_entry() {
    let view = load_source("view");
    let logic = load_source("logic");
    let headless_chart = include_str!("../../../crates/ui-headless/src/chart.rs");
    let headless_a11y = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let headless_i18n_common = include_str!("../../../crates/ui-headless/src/i18n/common.rs");
    let ui_root = include_str!("../../../crates/ui-components/src/root.rs");

    for required in [
        "CommonStrings",
        "use_ui_i18n",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "resolve_aria_label_with_fallback(",
        "common.chart_aria_label",
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
        "handlers.on_key_down(",
    ] {
        assert!(
            view.contains(required),
            "chart view should keep a11y+i18n+locale contract `{required}`",
        );
    }

    for required in [
        "pub fn resolve_aria_label_with_fallback(",
        "normalize_aria_label(aria_label.or(i18n_aria_label))",
    ] {
        assert!(
            logic.contains(required),
            "chart logic should centralize aria label fallback via `{required}`",
        );
    }

    for required in [
        "use crate::a11y::{A11yDirection, region_attrs};",
        "let region = region_attrs(options.aria_label, options.lang, options.dir);",
    ] {
        assert!(
            headless_chart.contains(required),
            "chart headless should map region a11y contract via `{required}`",
        );
    }

    for required in [
        "pub fn region_attrs(",
        "pub fn locale_attrs(",
        "pub struct RegionA11yAttrs",
    ] {
        assert!(
            headless_a11y.contains(required),
            "shared a11y helpers should come from ui-headless a11y.rs via `{required}`",
        );
    }

    for required in [
        "pub chart_aria_label: Arc<str>,",
        "chart_aria_label: \"Chart\".into(),",
    ] {
        assert!(
            headless_i18n_common.contains(required),
            "common i18n bundle should provide chart fallback text via `{required}`",
        );
    }

    for required in ["#[prop(optional)] i18n: UiI18n,", "provide_ui_i18n(i18n);"] {
        assert!(
            ui_root.contains(required),
            "UiRoot should keep i18n injection entrypoint via `{required}`",
        );
    }

    for forbidden in ["\"Quarterly growth line chart\"", "\"last action: \""] {
        assert!(
            !view.contains(forbidden),
            "chart component view should not hardcode app/business text `{forbidden}`",
        );
    }
}

#[test]
fn chart_state_markers_are_observable_queryable_and_enumerated() {
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "data-state=move || semantics.get().attrs.data_state",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-uncontrolled=move || semantics.get().attrs.data_uncontrolled",
        "data-active-index=move || state.get().active_index.to_string()",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-disabled=is_disabled.then_some(\"true\")",
        "aria-pressed=move || {",
        "data-index=index",
    ] {
        assert!(
            view.contains(required),
            "chart should expose stable semantic marker `{required}`",
        );
    }

    for required in [
        "pub enum ChartActiveValueSource",
        "pub enum ChartInteractionSource",
        "pub const fn as_attr(self) -> &'static str",
        "Self::Default => \"default\"",
        "Self::External => \"external\"",
        "Self::Interaction => \"interaction\"",
        "Self::None => \"none\"",
        "Self::Focus => \"focus\"",
        "Self::Pointer => \"pointer\"",
        "Self::Keyboard => \"keyboard\"",
        "pub const fn initial_active_value_source(is_controlled: bool) -> ChartActiveValueSource",
        "pub const fn interaction_active_value_source(",
    ] {
        assert!(
            logic.contains(required),
            "chart marker values should stay in closed enum set via `{required}`",
        );
    }
}

#[test]
fn chart_styles_depend_on_explicit_state_markers_without_fragile_dom_or_inline_business_style() {
    let styles = load_source("styles");
    let view = load_source("view");

    for required in [
        ".ui-chart__bar[data-active=\"true\"]",
        ".ui-chart__dot[data-active=\"true\"]",
        ".ui-chart[data-disabled=\"true\"]",
        ".ui-chart[data-empty=\"true\"]",
        ".ui-chart[data-custom-class=\"true\"]",
        ".ui-chart--disabled",
        ".ui-chart--empty",
        ".ui-chart--custom-class",
    ] {
        assert!(
            styles.contains(required),
            "chart styles should branch on explicit semantic marker `{required}`",
        );
    }

    for forbidden in [
        ":nth-child(",
        ":first-child",
        ":last-child",
        ":only-child",
        ":has(",
    ] {
        assert!(
            !styles.contains(forbidden),
            "chart styles should not rely on fragile dom selector `{forbidden}`",
        );
    }

    assert!(
        !view.contains("style="),
        "chart view should not push business styling through inline style attributes",
    );
}

#[test]
fn chart_cascade_layer_and_runtime_style_contract_is_enforced_locally() {
    let css_entry_source = load_source("ui_components_css");
    let root_source = load_source("ui_components_root");
    let view_source = load_source("view");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-chart\")]",
        "out.push_str(crate::chart::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(required),
            "ui-components css entry should keep cascade-layer marker `{required}`",
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep centralized css injection contract `{required}`",
        );
    }

    for forbidden in [
        " style=",
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
    ] {
        assert!(
            !view_source.contains(forbidden),
            "chart view should not include plain inline style token `{forbidden}`",
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
                "chart runtime style should only set css custom properties; found `style:{key}` at line {}",
                line_index + 1
            );
        }
    }
}

#[test]
fn chart_cascade_layer_check_script_covers_contract_locally() {
    let script_source = load_source("contract_hygiene_check_script");

    let required = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(required),
        "contract-hygiene check script should enforce `{required}`",
    );
}

#[test]
fn chart_check2_marks_cascade_layer_contract_complete_locally() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "chart check2 should mark cascade-layer gate complete",
    );

    for required in [
        "chart_cascade_layer_and_runtime_style_contract_is_enforced",
        "chart_cascade_layer_check_script_covers_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "components/chart/src/view.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 cascade-layer section should reference `{required}`",
        );
    }
}

#[test]
fn chart_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("styles");
    let theme_css_source = include_str!("../../../crates/ui-theme/src/css.rs");

    for required in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-bg, var(--ui-fallback-bg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-checkbox-group-motion-duration, var(--ui-fallback-checkbox-group-motion-duration))",
        "var(--ui-checkbox-group-motion-easing, var(--ui-fallback-checkbox-group-motion-easing))",
    ] {
        assert!(
            styles_source.contains(required),
            "chart styles should keep defensive fallback chain marker `{required}`",
        );
    }

    for required in [
        "--ui-fallback-space-2xs:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-component-height-100:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-radius-sm:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-bg-muted:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-accent:",
        "--ui-fallback-checkbox-group-motion-duration:",
        "--ui-fallback-checkbox-group-motion-easing:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css should provide fallback terminal `{required}`",
        );
    }

    for forbidden in ["0.75rem", "0.5rem", "14rem", "160ms", "2px solid", "#"] {
        assert!(
            !styles_source.contains(forbidden),
            "chart styles should avoid raw terminal token `{forbidden}`",
        );
    }
}

#[test]
fn chart_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_source("contract_hygiene_check_script");

    let required = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(required),
        "contract-hygiene check script should enforce `{required}`",
    );
}

#[test]
fn chart_check2_marks_defensive_variables_contract_complete() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "chart check2 should mark defensive-variables gate complete",
    );

    for required in [
        "chart_styles_use_defensive_variable_fallback_chain",
        "chart_defensive_variables_check_script_covers_style_fallback_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "components/chart/src/styles.rs",
        "crates/ui-theme/src/css.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 defensive-variables section should reference `{required}`",
        );
    }
}

#[test]
fn chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance()
 {
    let view = load_source("view");
    let motion = load_source("motion");
    let visual_driver = load_source("visual_active_highlight");
    let repo_semantics = include_str!("../../../crates/ui-components/tests/chart_semantics.rs");

    for required in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-uncontrolled=move || semantics.get().attrs.data_uncontrolled",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "on:keydown=on_key_down",
        "on:pointerenter=on_enter",
        "on:click=on_click",
    ] {
        assert!(
            view.contains(required),
            "chart semantic matrix should include branch marker `{required}`",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(required),
            "chart motion should keep wasm/ssr split via `{required}`",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            visual_driver.contains(required),
            "visual driver should keep platform split contract `{required}`",
        );
    }

    let forbidden_tokens = vec![
        ["assert", "snapshot"].join("_"),
        "insta::".to_string(),
        "to_match_image_snapshot".to_string(),
    ];
    for token in forbidden_tokens {
        assert!(
            !repo_semantics.contains(&token),
            "chart semantic contract tests should not rely on visual snapshot token `{token}`",
        );
    }
}

#[test]
fn chart_component_file_responsibilities_stay_layered_and_non_overlapping() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let motion = load_source("motion");

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ChartKind, ChartPoint, DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE};",
        "pub use motion::ChartMotion;",
        "pub use view::Chart;",
    ] {
        assert!(
            module.contains(required),
            "mod.rs should keep minimal export boundary `{required}`",
        );
    }

    for forbidden in ["view!", "NodeRef", "on:click", "on:keydown", ".ui-chart__"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not contain view/dom/style detail `{forbidden}`",
        );
    }

    for required in ["pub const CSS: &str =", "var(--ui-"] {
        assert!(
            styles.contains(required),
            "styles.rs should remain token-first static css via `{required}`",
        );
    }
    for forbidden in ["on:click", "on:keydown", "view!", "use_chart("] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not include interaction/render logic `{forbidden}`",
        );
    }

    for required in [
        "logic::normalize_input_boundary(",
        "logic::derive_state_from_boundary(",
        "use_chart(ChartOptions {",
        "motion::attach_motion(",
    ] {
        assert!(
            view.contains(required),
            "view.rs should assemble logic/headless/motion via `{required}`",
        );
    }
    for forbidden in [
        "pub struct ChartStateInput",
        "pub struct ChartState",
        "pub fn resolve_state(",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not redefine primitive state type `{forbidden}`",
        );
    }

    for required in [
        "pub type ChartMotion =",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "ui_motion::spring::sanitize_config",
    ] {
        assert!(
            motion.contains(required),
            "motion.rs should map semantic state to shared motion contract `{required}`",
        );
    }
    for forbidden in [
        "role=",
        "aria-",
        "on:click",
        "on:keydown",
        "SpringAnimator::new(",
    ] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should not own a11y/events/engine internals `{forbidden}`",
        );
    }

    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(required)
                .exists(),
            "chart component directory should contain required file `{required}`",
        );
    }

    for forbidden in ["src/render.rs", "src/spec.rs"] {
        assert!(
            !Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(forbidden)
                .exists(),
            "chart component directory should keep `{forbidden}` absent for this checklist scope",
        );
    }
}

#[test]
fn chart_component_files_check_script_covers_responsibility_contract_locally() {
    let script_source = load_source("component_files_check_script");
    let required = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_component_file_responsibilities_stay_layered_and_non_overlapping";
    assert!(
        script_source.contains(required),
        "component-files check script should enforce `{required}`",
    );
}

#[test]
fn chart_check2_marks_component_file_responsibility_contract_complete_locally() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] 组件目录标准文件落点正确。"),
        "chart check2 should mark component-file-responsibility gate complete",
    );

    for required in [
        "chart_component_file_responsibilities_stay_layered_and_non_overlapping",
        "chart_component_files_check_script_covers_responsibility_contract",
        "scripts/check-ui-components-component-files.sh",
        "components/chart/src/mod.rs",
        "components/chart/src/logic.rs",
        "components/chart/src/styles.rs",
        "components/chart/src/view.rs",
        "components/chart/src/motion.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 component-files section should reference `{required}`",
        );
    }
}

#[test]
fn chart_file_placement_discipline_is_strict_for_struct_first_scope_locally() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files: Vec<String> = fs::read_dir(&src_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {src_dir:?}: {e}"))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("dir entry read failed: {e}"));
            entry.file_name().to_string_lossy().into_owned()
        })
        .collect();
    files.sort();

    assert_eq!(
        files,
        vec![
            "logic.rs",
            "mod.rs",
            "motion.rs",
            "protocol.rs",
            "styles.rs",
            "view.rs",
        ],
        "chart src should keep strict file-placement discipline and only include layered core files plus protocol.rs",
    );

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !files.iter().any(|name| name == forbidden),
            "chart src should keep `{forbidden}` absent unless checklist scope changes",
        );
    }
}

#[test]
fn chart_component_files_check_script_covers_file_placement_discipline_locally() {
    let script_source = load_source("component_files_check_script");
    let required = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_file_placement_discipline_is_strict_for_struct_first_scope";
    assert!(
        script_source.contains(required),
        "component-files check script should enforce `{required}`",
    );
}

#[test]
fn chart_check2_marks_file_placement_discipline_contract_complete_locally() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "chart check2 should mark file-placement-discipline gate complete",
    );

    for required in [
        "chart_file_placement_discipline_is_strict_for_struct_first_scope",
        "chart_component_files_check_script_covers_file_placement_discipline",
        "scripts/check-ui-components-component-files.sh",
        "components/chart/src/mod.rs",
        "components/chart/src/logic.rs",
        "components/chart/src/styles.rs",
        "components/chart/src/view.rs",
        "components/chart/src/motion.rs",
        "components/chart/src/protocol.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 file-placement section should reference `{required}`",
        );
    }
}

#[test]
fn chart_token_first_styles_are_aggregated_via_ui_root_without_utility_or_css_in_rust_defaults() {
    let styles = load_source("styles");
    let view = load_source("view");
    let css_registry = load_source("ui_components_css");
    let ui_root = load_source("ui_components_root");

    for required in [
        "pub const CSS: &str =",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
    ] {
        assert!(
            styles.contains(required),
            "chart styles should stay token-first and static via `{required}`",
        );
    }

    assert!(
        css_registry.contains("out.push_str(crate::chart::styles::CSS);"),
        "ui-components css registry should aggregate chart styles through styles.rs",
    );
    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(required),
            "UiRoot should provide centralized css injection path `{required}`",
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "tailwind",
        "tw-",
        "style=",
        "styled(",
        "css!(",
    ] {
        assert!(
            !view.contains(forbidden) && !styles.contains(forbidden),
            "chart component should not adopt utility-first/CSS-in-Rust default token `{forbidden}`",
        );
    }
}

#[test]
fn chart_visual_desire_contract_uses_theme_baseline_page_snapshot_and_heroui_alignment() {
    let docs_pages = load_source("docs_components_pages");
    let theme_baseline_page = load_source("docs_theme_visual_baseline_page");
    let theme_baseline_e2e = load_source("docs_theme_visual_baseline_e2e");
    let heroui_strategy = load_source("docs_heroui_strategy");

    for required in [
        "component_doc!(",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
    ] {
        assert!(
            docs_pages.contains(required),
            "docs-app should expose theme visual baseline route via `{required}`",
        );
    }

    for required in [
        "description=\"Default theme visual baseline for hierarchy, contrast, and interaction cues.",
        "title=\"Default Theme Visual Baseline\"",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            theme_baseline_page.contains(required),
            "theme visual baseline page should keep visual-quality proof `{required}`",
        );
    }

    for required in [
        "const visualMode = process.env.E2E_VISUAL_BASELINE ?? \"off\";",
        "page.goto(\"/#/components/theme-visual-baseline\")",
        "docs-app: theme visual baseline renders button/input/overlay",
        "docs-app: theme visual baseline screenshots",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            theme_baseline_e2e.contains(required),
            "visual baseline e2e contract should include `{required}`",
        );
    }

    for required in [
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "输出统一参数分层规范",
        "不是只能改根节点 class",
        "HeroUI 参数设计风格对齐策略",
    ] {
        assert!(
            heroui_strategy.contains(required),
            "heroui strategy should preserve visual-language alignment boundary `{required}`",
        );
    }
}

#[test]
fn chart_tree_shaking_contract_keeps_component_feature_gates_and_budget_ci_pipeline() {
    let chart_module = load_source("mod");
    let ui_components_cargo = include_str!("../../../crates/ui-components/Cargo.toml");
    let ui_components_lib = include_str!("../../../crates/ui-components/src/lib.rs");
    let ui_components_css = load_source("ui_components_css");
    let tree_shaking_script = include_str!("../../../scripts/check-ui-components-tree-shaking.sh");
    let tree_shaking_budget = include_str!("../../../scripts/tree_shaking_budget.env");
    let ci_workflow = include_str!("../../../.github/workflows/ci.yml");

    assert!(
        ui_components_cargo.contains("component-chart = [\"dep:ui-chart\"]"),
        "component-chart should stay as an explicit package-level feature gate",
    );

    let chart_export = ui_components_lib
        .find("pub use ui_chart as chart;")
        .expect("ui-components lib should export chart module");
    let chart_export_cfg = ui_components_lib[..chart_export]
        .rfind("#[cfg(feature = \"component-chart\")]")
        .expect("chart export should be cfg-gated in lib.rs");
    assert!(
        chart_export - chart_export_cfg < 80,
        "chart export should stay immediately behind component-chart cfg gate",
    );

    let chart_css = ui_components_css
        .find("out.push_str(crate::chart::styles::CSS);")
        .expect("css registry should include chart styles aggregation");
    let chart_css_cfg = ui_components_css[..chart_css]
        .rfind("#[cfg(feature = \"component-chart\")]")
        .expect("chart css aggregation should be cfg-gated in css.rs");
    assert!(
        chart_css - chart_css_cfg < 80,
        "chart css aggregation should stay behind component-chart cfg gate",
    );

    for required in [
        "CHART_MIN_FEATURES=\"component-chart,inject-css\"",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "CHART_TREE_OUTPUT",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$CHART_MIN_FEATURES\"",
        "if ! grep -q 'feature \"component-chart\" (command-line)' <<<\"$CHART_TREE_OUTPUT\"; then",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$CHART_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$CHART_TREE_OUTPUT\"; then",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$CHART_MIN_FEATURES\"",
        "cargo tree -e features -i ui-components -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking budget script should retain gate `{required}`",
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(required),
            "tree-shaking budget env should define `{required}`",
        );
    }

    for required in [
        "- name: Tree Shaking Budget",
        "run: ./scripts/check-ui-components-tree-shaking.sh",
    ] {
        assert!(
            ci_workflow.contains(required),
            "ci workflow should enforce tree-shaking budget via `{required}`",
        );
    }

    for forbidden in [
        "ComponentRegistry",
        "register_component(",
        "all_components::",
    ] {
        assert!(
            !chart_module.contains(forbidden),
            "chart source mode should not require central registry token `{forbidden}`",
        );
    }
}

#[test]
fn chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget_locally() {
    let tree_shaking_script = include_str!("../../../scripts/check-ui-components-tree-shaking.sh");
    let tree_shaking_budget = include_str!("../../../scripts/tree_shaking_budget.env");

    for required in [
        "CHART_MIN_FEATURES=\"component-chart,inject-css\"",
        "chart_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes",
        "chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "chart_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "CHART_TREE_OUTPUT",
        "if ! grep -q 'feature \"component-chart\" (command-line)' <<<\"$CHART_TREE_OUTPUT\"; then",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$CHART_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$CHART_TREE_OUTPUT\"; then",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$CHART_MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking script should enforce `{required}`",
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(required),
            "tree-shaking budget env should define `{required}`",
        );
    }
}

#[test]
fn chart_check2_marks_tree_shaking_feature_pruning_contract_complete_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-chart = [\"dep:ui-chart\"]",
        "#[cfg(feature = \"component-chart\")]",
        "pub use ui_chart as chart;",
        "out.push_str(crate::chart::styles::CSS);",
        "chart_tree_shaking_contract_keeps_component_feature_gates_and_budget_ci_pipeline",
        "chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget_locally",
        "chart_check2_marks_tree_shaking_feature_pruning_contract_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes",
        "crates/ui-components/tests/chart_semantics.rs::chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-chart,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "scripts/check-ui-components-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 tree-shaking section should reference `{required}`",
        );
    }
}

#[test]
fn chart_type_system_and_semantic_markers_form_machine_readable_contract_feedback_loop() {
    let logic = load_source("logic");
    let view = load_source("view");
    let local_semantics = include_str!("../test/semantics.rs");
    let repo_semantics = include_str!("../../../crates/ui-components/tests/chart_semantics.rs");

    for required in [
        "pub enum ChartKind",
        "pub enum ChartActiveValueSource",
        "pub enum ChartInteractionSource",
        "pub fn normalize_input_boundary(",
        "pub fn derive_state_from_boundary(",
        "pub fn normalize_interaction_index(",
    ] {
        assert!(
            logic.contains(required),
            "type-safe and normalized logic contract should contain `{required}`",
        );
    }

    for forbidden in ["match kind.as_str()", "from_str(", ".parse::<ChartKind>()"] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "chart should avoid string protocol for discrete state token `{forbidden}`",
        );
    }

    for required in [
        "#[prop(optional)] kind: ChartKind,",
        "data-state=move || semantics.get().attrs.data_state",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
    ] {
        assert!(
            view.contains(required),
            "machine-readable semantic marker contract should expose `{required}`",
        );
    }

    for required in [
        "fn chart_discrete_state_axes_are_type_constrained()",
        "fn chart_state_markers_are_observable_queryable_and_enumerated()",
    ] {
        assert!(
            local_semantics.contains(required) && repo_semantics.contains(required),
            "feedback loop should keep mirrored semantic regressions via `{required}`",
        );
    }
}

#[test]
fn chart_focus_stack_overlay_contract_is_na_for_non_overlay_component() {
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");

    for required in [
        "let legend_ref: NodeRef<html::Div> = NodeRef::new();",
        "let highlight_ref: NodeRef<html::Div> = NodeRef::new();",
        "attach_motion(ChartMotionAttach {",
    ] {
        assert!(
            view.contains(required) || motion.contains(required),
            "chart node refs should remain scoped to visual highlight path `{required}`",
        );
    }

    for forbidden in [
        "<Overlay",
        "role=\"dialog\"",
        "FallbackTo",
        "Selector",
        "FocusStack",
        "restore_focus",
        "focus_restore",
        "document.body",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "chart should not carry overlay focus-stack token `{forbidden}`",
        );
    }
}

#[test]
fn chart_foreign_zone_escape_hatch_contract_is_na_without_imperative_third_party_runtime() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");

    for forbidden in [
        "ECharts",
        "echarts",
        "mapbox",
        "leaflet",
        "google.maps",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "foreign_instance",
        "imperative_runtime",
        "extern \"C\"",
        "wasm_bindgen",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden),
            "chart should not contain foreign-zone integration token `{forbidden}`",
        );
    }

    for required in [
        "pub use logic::{ChartKind, ChartPoint, DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE};",
        "pub use motion::ChartMotion;",
        "pub use view::Chart;",
    ] {
        assert!(
            module.contains(required),
            "chart public api should stay pure and not expose third-party runtime via `{required}`",
        );
    }
}

#[test]
fn chart_hydration_discontinuity_uses_deterministic_id_provider_seed_for_generated_ids() {
    let logic = load_source("logic");
    let view = load_source("view");
    let root = load_source("ui_components_root");
    let id_provider = load_source("headless_id_provider");

    for required in [
        "use_ui_id_provider,",
        "let generated_id_base = use_ui_id_provider()",
        "next_prefixed_id(logic::DEFAULT_ID_BASE)",
        "let id_base = logic::resolve_id_base(id_base, generated_id_base);",
        "id_base: Some(id_base),",
        "pub fn resolve_id_base(id_base: Option<String>, generated_id_base: String) -> String",
        "normalize_optional_text(id_base).unwrap_or(generated_id_base)",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "chart hydration contract should include deterministic id wiring `{required}`",
        );
    }

    for forbidden in [
        "SystemTime::now",
        "UNIX_EPOCH",
        "now()",
        "rand::",
        "thread_rng",
        "Uuid::new_v4",
        "uuid::Uuid",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "chart should not use hydration-unstable entropy source `{forbidden}`",
        );
    }

    assert!(
        root.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should provide deterministic id provider seed for hydration alignment",
    );
    for required in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
    ] {
        assert!(
            id_provider.contains(required),
            "ui-headless id-provider contract should expose `{required}`",
        );
    }
}

#[test]
fn chart_platform_contract_covers_default_ssr_wasm_compile_paths_and_non_wasm_source_guards() {
    let view = load_source("view");
    let motion = load_source("motion");
    let platform_script = load_source("platform_check_script");

    for required in [
        "cargo check -p ui-components",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-components --no-default-features --features component-chart,inject-css",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-chart,inject-css",
        "components/chart/src/mod.rs",
        "components/chart/src/logic.rs",
        "components/chart/src/styles.rs",
        "components/chart/src/view.rs",
        "components/chart/src/motion.rs",
        "chart motion must keep explicit wasm/non-wasm branches",
        "'#[cfg(target_arch = \"wasm32\")]' components/chart/src/motion.rs",
        "'#[cfg(not(target_arch = \"wasm32\"))]' components/chart/src/motion.rs",
    ] {
        assert!(
            platform_script.contains(required),
            "platform gate script should include chart compile/source guard `{required}`",
        );
    }

    for forbidden in ["web_sys", "web-sys"] {
        assert!(
            !view.contains(forbidden) && !motion.contains(forbidden),
            "chart non-wasm path should not depend on browser API token `{forbidden}`",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion.contains(required),
            "chart motion should keep explicit platform cfg branch `{required}`",
        );
    }
}

#[test]
fn chart_ui_headless_web_ssr_feature_mutex_is_compile_error_guarded() {
    let view = load_source("view");
    let headless_lib = load_source("headless_lib");
    let platform_script = load_source("platform_check_script");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless feature mutex should be guarded via `{required}`",
        );
    }

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "if cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "rg -n \"mutually exclusive\"",
        "cargo check -p ui-components --no-default-features --features component-chart,inject-css",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-chart,inject-css",
    ] {
        assert!(
            platform_script.contains(required),
            "platform gate should enforce ui-headless mutex and chart compile paths via `{required}`",
        );
    }

    assert!(
        view.contains("use ui_headless::{"),
        "chart should keep consuming ui-headless contract under mutex discipline",
    );
}

#[test]
fn chart_ui_motion_non_wasm_stub_contract_keeps_ssr_and_tooling_paths_compilable() {
    let chart_motion = load_source("motion");
    let ui_motion_lib = load_source("ui_motion_lib");
    let ui_motion_non_wasm_stub_tests = load_source("ui_motion_non_wasm_stub_tests");
    let platform_script = load_source("platform_check_script");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion non-wasm no-op contract should contain `{required}`",
        );
    }

    for required in [
        "non_wasm_web_backend_prefers_reduced_motion",
        "non_wasm_web_backend_animate_is_safe_noop",
        "#![cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            ui_motion_non_wasm_stub_tests.contains(required),
            "ui-motion non-wasm regression suite should contain `{required}`",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            chart_motion.contains(required),
            "chart motion should keep predictable non-wasm downgrade via `{required}`",
        );
    }

    for required in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script.contains(required),
            "platform gate should enforce ui-motion non-wasm/wasm compile-test path `{required}`",
        );
    }
}

#[test]
fn chart_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let view = load_source("view");
    let motion = load_source("motion");
    let visual_driver = load_source("visual_active_highlight");
    let ui_motion_spring = load_source("ui_motion_spring");
    let ui_motion_web = load_source("ui_motion_web");
    let platform_script = load_source("platform_check_script");

    for required in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring.contains(required),
            "spring runtime should keep reduced-motion immediate-settle contract `{required}`",
        );
    }

    for required in [
        "pub fn prefers_reduced_motion() -> bool {",
        "if prefers_reduced_motion() {",
    ] {
        assert!(
            ui_motion_web.contains(required),
            "wasm web backend should branch on reduced-motion via `{required}`",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(required),
            "chart motion should keep explicit ssr/wasm split and safe downgrade `{required}`",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            visual_driver.contains(required),
            "active highlight driver should keep ssr/wasm split `{required}`",
        );
    }

    for required in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-uncontrolled=move || semantics.get().attrs.data_uncontrolled",
    ] {
        assert!(
            view.contains(required),
            "chart semantic markers should remain stable across ssr/wasm branches `{required}`",
        );
    }
    assert!(
        !view.contains("cfg(target_arch"),
        "chart view should avoid splitting semantic output by target_arch to keep hydration consistency",
    );

    assert!(
        platform_script
            .contains("chart_reduced_motion_ssr_wasm_branches_keep_semantics_consistent"),
        "platform script should execute chart reduced-motion/ssr/wasm regression gate",
    );
}

#[test]
fn chart_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe_locally() {
    let motion = load_source("motion");
    let motion_unit_test_source = include_str!("motion.rs");
    let view = load_source("view");
    let ui_motion_spring = load_source("ui_motion_spring");
    let ui_motion_lib = load_source("ui_motion_lib");
    let visual_driver = load_source("visual_active_highlight");
    let check2_source = load_source("check2");

    for required in [
        "pub type ChartMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;",
        "pub fn sanitize_motion(motion: ChartMotion) -> ChartMotion {",
        "ui_motion::spring::sanitize_config(",
        "ui_motion::presets::spring_slide()",
        "pub fn attach_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(required),
            "chart motion contract should keep marker `{required}`",
        );
    }

    for required in [
        "fn sanitize_motion_preserves_contract()",
        "fn sanitize_motion_falls_back_for_invalid_spring_values()",
        "stiffness: f64::NAN,",
        "damping: -1.0,",
    ] {
        assert!(
            motion_unit_test_source.contains(required),
            "chart motion unit tests should keep `{required}`",
        );
    }

    for required in [
        "let motion = motion::sanitize_motion(motion);",
        "motion::attach_motion(",
    ] {
        assert!(
            view.contains(required),
            "chart view should sanitize and attach motion contract via `{required}`",
        );
    }

    for required in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            ui_motion_spring.contains(required),
            "ui-motion spring should keep reduced-motion immediate-settle branch `{required}`",
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion non-wasm no-op contract should include `{required}`",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            visual_driver.contains(required),
            "active highlight shared driver should keep platform split marker `{required}`",
        );
    }

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "chart_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(required),
            "chart checklist should keep motion contractualization evidence `{required}`",
        );
    }
}

#[test]
fn chart_motion_contract_platform_script_covers_guard_locally() {
    let platform_script = load_source("platform_check_script");

    let required = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        platform_script.contains(required),
        "platform script should enforce `{required}`",
    );
}

#[test]
fn chart_check2_marks_motion_contract_complete_locally() {
    let check2_source = load_source("check2");

    for required in [
        "chart_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "chart_motion_contract_platform_script_covers_guard",
        "scripts/check-ui-components-platforms.sh",
        "components/chart/src/motion.rs",
        "components/chart/src/view.rs",
        "crates/ui-motion/src/spring.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 motion section should reference `{required}`",
        );
    }
}

#[test]
fn chart_ui_components_fixed_entry_files_follow_layered_boundaries_locally() {
    let lib_source = include_str!("../../../crates/ui-components/src/lib.rs");
    let css_source = load_source("ui_components_css");
    let root_source = load_source("ui_components_root");
    let active_highlight_source = load_source("visual_active_highlight");
    let controllable_state_source = load_source("headless_controllable_state");
    let presence_source = include_str!("../../../crates/ui-headless/src/presence.rs");
    let a11y_source = include_str!("../../../crates/ui-headless/src/a11y.rs");

    for required in [
        "#[cfg(feature = \"component-chart\")]",
        "pub use ui_chart as chart;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui-components lib entry should keep marker `{required}`",
        );
    }

    for forbidden in [
        "pub mod css;",
        "leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`",
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-chart\")]",
        "out.push_str(crate::chart::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css registry should keep feature-gated marker `{required}`",
        );
    }

    for required in [
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep centralized theme/i18n marker `{required}`",
        );
    }

    for required in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight shared primitive should keep marker `{required}`",
        );
    }

    for forbidden in [
        "ComboBox",
        "Accordion",
        "Button",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`",
        );
    }

    for forbidden in [
        "../../../crates/ui-components/src/overlay_open.rs",
        "../../../crates/ui-components/src/presence.rs",
        "../../../crates/ui-components/src/a11y.rs",
    ] {
        assert!(
            !Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(forbidden)
                .exists(),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`",
        );
    }

    for required in [
        "../../../crates/ui-headless/src/controllable_state.rs",
        "../../../crates/ui-headless/src/presence.rs",
        "../../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(required)
                .exists(),
            "ui-headless canonical primitive file should exist: `{required}`",
        );
    }

    for required in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(required)
                || presence_source.contains(required)
                || a11y_source.contains(required),
            "headless canonical primitive files should keep marker `{required}`",
        );
    }
}

#[test]
fn chart_entrypoints_check_script_covers_fixed_entrypoint_contract_locally() {
    let script_source = load_source("entrypoints_check_script");

    let required = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(required),
        "entrypoints check script should enforce `{required}`",
    );
}

#[test]
fn chart_check2_marks_ui_components_fixed_entry_files_contract_complete_locally() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] `ui-components` 固定入口文件落点正确。"),
        "chart check2 should mark fixed-entry-files gate complete",
    );

    for required in [
        "chart_ui_components_fixed_entry_files_follow_layered_boundaries",
        "chart_entrypoints_check_script_covers_fixed_entrypoint_contract",
        "scripts/check-ui-components-entrypoints.sh",
        "crates/ui-components/src/lib.rs",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "crates/ui-visual-primitive/src/active_highlight.rs",
        "crates/ui-headless/src/controllable_state.rs",
        "crates/ui-headless/src/presence.rs",
        "crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 fixed-entry-files section should reference `{required}`",
        );
    }
}

#[test]
fn chart_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("docs_components_shell");
    let perf_probe_source = load_source("docs_perf_probe");
    let coverage_source = load_source("docs_components_coverage_e2e");
    let todo_source = load_source("todo");
    let check2_source = load_source("check2");
    let script_source = load_source("performance_check_script");
    let view_source = load_source("view");

    for required in [
        "\"chart\" => UiPerfBudget {",
        "max_mount_ms: 34.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
    ] {
        assert!(
            shell_source.contains(required),
            "chart docs shell should keep explicit performance budget `{required}`",
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
            "UiPerfProbe should expose budget/violation observability marker `{required}`",
        );
    }

    for required in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(required),
            "docs coverage e2e should include blocking perf assertion `{required}`",
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(required),
            "perf governance follow-up plan should keep `{required}`",
        );
    }

    for required in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 should keep performance governance marker `{required}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_performance_governance_contract_is_budgeted_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`",
    );

    for required in [
        "logic::derive_state_from_boundary(logic::ChartStateBoundary {",
        "motion::attach_motion(",
        "data-active-index=move || state.get().active_index.to_string()",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            view_source.contains(required),
            "chart view should expose attribution marker `{required}` for perf triage",
        );
    }
}

#[test]
fn chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally()
 {
    let local_semantics = include_str!("../test/semantics.rs");
    let aggregated_semantics =
        include_str!("../../../crates/ui-components/tests/chart_semantics.rs");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let todo_source = load_source("todo");

    for required_test in [
        "fn chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance()",
        "fn chart_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally()",
    ] {
        assert!(
            local_semantics.contains(required_test),
            "chart local semantics/performance suite should include `{required_test}`",
        );
    }

    for required_test in [
        "fn chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance()",
        "fn chart_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            aggregated_semantics.contains(required_test),
            "chart aggregated semantics/performance suite should include `{required_test}`",
        );
    }

    for marker in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "on:focus=on_focus",
        "on:pointerenter=on_pointer_enter",
        "on:click=on_click",
        "on:keydown=on_key_down",
        "Self::Focus => \"focus\"",
    ] {
        assert!(
            view_source.contains(marker) || logic_source.contains(marker),
            "chart semantic/focus-flow regression should include marker `{marker}`",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`",
        );
    }
}

#[test]
fn chart_semantics_priority_contract_is_documented_and_scripted_locally() {
    let local_semantics = include_str!("../test/semantics.rs");
    let aggregated_semantics =
        include_str!("../../../crates/ui-components/tests/chart_semantics.rs");
    let view_source = load_source("view");
    let script_source = load_source("performance_check_script");
    let check2_source = load_source("check2");

    for required in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "on:focus=on_focus",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(required),
            "chart semantic-first contract should expose `{required}`",
        );
    }

    for required in [
        "fn chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance()",
        "chart semantic contract tests should not rely on visual snapshot token",
    ] {
        assert!(
            local_semantics.contains(required) && aggregated_semantics.contains(required),
            "chart local/aggregated semantics suites should keep `{required}`",
        );
    }

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_semantics_tests_priority_rules",
    ] {
        assert!(
            script_source.contains(required),
            "performance check script should include semantic-first command `{required}`",
        );
    }

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/chart/test/semantics.rs::chart_semantics_priority_contract_is_documented_and_scripted_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_semantics_tests_prioritize_data_aria_role_and_state_source_over_visual_snapshot",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_semantics_tests_priority_rules",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 semantic-first section should include `{required}`",
        );
    }
}

#[test]
fn chart_check2_documents_e2e_selector_and_stable_wait_rules_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 e2e-selector section should include `{required}`",
        );
    }
}

#[test]
fn chart_e2e_selector_contract_uses_semantic_markers_and_stable_waits_locally() {
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_chart_contract.spec.mjs");
    let docs_source = load_source("docs_display_extra");

    for required in [
        "docs-app chart uses semantic selectors with wasm-stable ready waits",
        "docs-app chart interaction uses semantic ready and settled breakpoints",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-slot=\"chart-e2e-controlled-line\"] [data-slot=\"chart\"]",
        "[data-slot=\"chart-e2e-state-disabled\"] [data-slot=\"chart\"]",
        "[data-slot=\"chart-workbench-canvas\"]",
        "[data-slot=\"chart-workbench-toggle-lang\"] [data-slot=\"switch\"]",
    ] {
        assert!(
            e2e_contract.contains(required),
            "chart e2e selector contract should include marker `{required}`",
        );
    }

    for required in [
        "data-slot=\"chart-e2e-controlled-line\"",
        "data-slot=\"chart-e2e-state-disabled\"",
        "data-slot=\"chart-workbench-canvas\"",
        "data-slot=\"chart-workbench-toggle-lang\"",
    ] {
        assert!(
            docs_source.contains(required),
            "chart docs should expose stable semantic selector anchor `{required}`",
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
            "chart e2e selector contract should avoid fragile wait/selector `{forbidden}`",
        );
    }
}

#[test]
fn chart_e2e_contract_covers_ready_and_settled_conditions_for_chart_interaction_locally() {
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_chart_contract.spec.mjs");

    for required in [
        "toHaveAttribute(\"data-state\", \"ready\")",
        "toHaveAttribute(\"data-active-interaction-source\", \"keyboard\")",
        "toHaveAttribute(\"data-active-interaction-source\", \"pointer\")",
        "toHaveAttribute(\"data-ui-state\", \"ready\")",
        "await langSwitch.click();",
    ] {
        assert!(
            e2e_contract.contains(required),
            "chart e2e contract should keep ready/settled semantic breakpoint `{required}`",
        );
    }
}

#[test]
fn chart_e2e_check_script_covers_selector_and_settled_wait_contract_locally() {
    let script_source = include_str!("../../../scripts/check-ui-components-e2e-chart.sh");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_e2e_contract_covers_ready_and_settled_conditions_for_chart_interaction",
    ] {
        assert!(
            script_source.contains(required),
            "chart e2e check script should include `{required}`",
        );
    }
}

#[test]
fn chart_check2_marks_e2e_selector_stability_item_complete_locally() {
    let check2_source = load_source("check2");
    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "chart check2 should mark e2e selector stability item complete",
    );

    for required in [
        "components/chart/test/semantics.rs::chart_check2_documents_e2e_selector_and_stable_wait_rules_locally",
        "components/chart/test/semantics.rs::chart_e2e_selector_contract_uses_semantic_markers_and_stable_waits_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_e2e_selector_and_stable_wait_rules",
        "crates/ui-components/tests/chart_semantics.rs::chart_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "e2e/tests/docs_app_chart_contract.spec.mjs",
        "scripts/check-ui-components-e2e-chart.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 e2e selector stability section should reference `{required}`",
        );
    }
}

#[test]
fn chart_check2_documents_e2e_repeatable_key_flow_rules_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 repeatable-key-flow section should include `{required}`",
        );
    }
}

#[test]
fn chart_e2e_key_flow_is_repeatable_and_failure_points_are_semantic_locally() {
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_chart_contract.spec.mjs");

    for required in [
        "docs-app chart key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "chart-workbench-toggle-disabled",
        "toHaveAttribute(\"data-active-index\", \"2\")",
        "toHaveAttribute(\"data-active-interaction-source\", \"keyboard\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toHaveAttribute(\"data-disabled\", \"false\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"ready\")",
        "await page.keyboard.press(\"ArrowRight\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_contract.contains(required),
            "chart repeatable key-flow contract should include marker `{required}`",
        );
    }
}

#[test]
fn chart_e2e_check_script_covers_repeatable_key_flow_contract_locally() {
    let script_source = include_str!("../../../scripts/check-ui-components-e2e-chart.sh");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script_source.contains(required),
            "chart e2e check script should include repeatable-key-flow marker `{required}`",
        );
    }
}

#[test]
fn chart_check2_marks_e2e_repeatable_key_flow_item_complete_locally() {
    let check2_source = load_source("check2");
    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "chart check2 should mark repeatable e2e key-flow item complete",
    );

    for required in [
        "components/chart/test/semantics.rs::chart_check2_documents_e2e_repeatable_key_flow_rules_locally",
        "components/chart/test/semantics.rs::chart_e2e_key_flow_is_repeatable_and_failure_points_are_semantic_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_e2e_repeatable_key_flow_item_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_e2e_repeatable_key_flow_rules",
        "crates/ui-components/tests/chart_semantics.rs::chart_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_e2e_repeatable_key_flow_item_complete",
        "scripts/check-ui-components-e2e-chart.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 repeatable key-flow section should reference `{required}`",
        );
    }
}

#[test]
fn chart_semantics_and_performance_script_covers_contract_locally() {
    let script_source = load_source("performance_check_script");

    for marker in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`",
        );
    }
}

#[test]
fn chart_check2_marks_semantics_and_performance_regression_contract_complete_locally() {
    let check2_source = load_source("check2");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance",
        "chart_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "chart check2 semantic/performance section should include `{marker}`",
        );
    }
}

#[test]
fn chart_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("view");
    let script_source = load_source("view_macro_check_script");

    for required in [
        "fn render_chart_plot(",
        "fn render_chart_legend(",
        "let plot = render_chart_plot(",
        "let legend = render_chart_legend(",
        "{plot}",
        "{legend}",
    ] {
        assert!(
            view_source.contains(required),
            "chart view macro split should include `{required}`",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "chart should keep a single public component boundary",
    );

    assert!(
        view_source.matches("view! {").count() <= 6,
        "chart view should keep macro count bounded after semantic subrender split",
    );

    assert!(
        view_source.lines().count() <= 520,
        "chart view.rs should stay bounded; split further if this grows significantly",
    );

    let script_needle = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`",
    );
}

#[test]
fn chart_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("view");
    let script_source = load_source("view_macro_check_script");

    for required in [
        "fn render_chart_plot(",
        "fn render_chart_legend(",
        ") -> impl IntoView {",
        "pub fn Chart(",
    ] {
        assert!(
            view_source.contains(required),
            "chart functional split should include `{required}`",
        );
    }

    for forbidden in [
        "#[component]\nfn render_chart_plot(",
        "#[component]\nfn render_chart_legend(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "chart local fragments should stay plain functions, not extra components `{forbidden}`",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "chart should keep one public component boundary",
    );

    for required in [
        "data-slot=\"chart\"",
        "data-slot=\"chart-plot\"",
        "data-slot=\"chart-legend\"",
        "data-slot=\"chart-legend-item\"",
    ] {
        assert!(
            view_source.contains(required),
            "chart semantic markers should stay stable after function split `{required}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`",
    );
}

#[test]
fn chart_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("view");
    let script_source = load_source("view_macro_check_script");

    for required in [
        "const CHART_PLOT_VIEWBOX: &str = \"0 0 100 56\";",
        "const CHART_GRID_LINE_CLASS: &str = \"ui-chart__grid-line\";",
        "const CHART_GRID_LINES: [(&str, &str, &str, &str); 4] = [",
        "fn render_chart_grid_lines() -> impl IntoView",
        "viewBox=CHART_PLOT_VIEWBOX",
        "{render_chart_grid_lines()}",
    ] {
        assert!(
            view_source.contains(required),
            "chart static fragments should be constantized via `{required}`",
        );
    }

    for required in [
        "role=\"img\"",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-slot=\"chart-grid\"",
        "data-slot=\"chart-plot\"",
    ] {
        assert!(
            view_source.contains(required),
            "chart static fragment refactor should keep a11y/semantic marker `{required}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_static_fragments_are_constantized_with_stable_semantics";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`",
    );
}

#[test]
fn chart_inner_html_usage_is_explicitly_na_and_guarded() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");
    let view_source = load_source("view");
    let docs_chart_source = load_source("docs_display_extra");
    let docs_shell_source = load_source("docs_components_shell");
    let check2_source = load_source("check2");
    let script_source = load_source("inner_html_check_script");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "format!(\"<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_chart_source.contains(forbidden),
            "chart component/docs page should stay free from html injection token `{forbidden}`",
        );
    }

    assert!(
        docs_shell_source.contains("<div data-slot=\"component-readme\" inner_html=html></div>"),
        "shared docs shell should keep the single trusted inner_html mount for readme rendering",
    );
    assert!(
        !docs_shell_source.contains("\"chart\" => Some("),
        "chart should stay out of readme inner_html whitelist in shared shell",
    );
    for required in [
        "const ACCORDION_README_MD: &str = include_str!(",
        "const CHECKBOX_README_MD: &str = include_str!(",
        "const MODAL_README_MD: &str = include_str!(",
    ] {
        assert!(
            docs_shell_source.contains(required),
            "docs shell trusted markdown whitelist should include static source marker `{required}`",
        );
    }

    for required in [
        "`inner_html` 使用约束：仅允许注入受信任静态常量",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart checklist should keep inner_html safety governance marker `{required}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_inner_html_usage_is_explicitly_na_and_guarded";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should include `{script_needle}`",
    );
}

#[test]
fn chart_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = include_str!("../../../crates/ui-components/Cargo.toml");
    let crate_root_source = include_str!("../../../crates/ui-components/src/lib.rs");
    let button_view_source = include_str!("../../button/src/view.rs");
    let docs_app_source = include_str!("../../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = include_str!("../../../crates/ui-headless/src/trace.rs");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let motion_source = load_source("motion");
    let check2_source = load_source("check2");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "wasm debug capability should stay feature-gated via `button-wasm-debug`",
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path",
    );

    for required in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(required),
            "ui-components root should keep wasm debug isolation marker `{required}`",
        );
    }

    for required in [
        "data-debug-source=source.clone()",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "data-debug-timestamp-ms=format!(\"{:.0}\", event.timestamp_ms)",
        "data-slot=\"button-debug-replay\"",
        "request_replay.run(event.source)",
    ] {
        assert!(
            button_view_source.contains(required),
            "shared button wasm debug path should keep trace/replay marker `{required}`",
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
            "docs debug visual entry should keep `{required}`",
        );
    }

    for required in [
        "events.push(event);",
        ".into_iter()",
        ".take(40)",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::Note",
        "UiTraceEventKind::Inspect",
        "trace.emit(",
    ] {
        assert!(
            trace_source.contains(required) || debug_overlay_source.contains(required),
            "global trace timeline/replay evidence should keep marker `{required}`",
        );
    }

    for required in [
        "data-active-index=move || state.get().active_index.to_string()",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-motion-source=motion_source",
        "on:pointerenter=on_enter",
        "on:click=on_click",
        "on:keydown=on_key_down",
        "apply_headless_action.run(action);",
        "pub enum ChartInteractionSource",
        "Self::Focus => \"focus\"",
        "Self::Pointer => \"pointer\"",
        "Self::Keyboard => \"keyboard\"",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "chart should keep machine-readable state/source/interaction marker `{required}` for debug attribution",
        );
    }

    for forbidden in [
        "chart-wasm-debug",
        "wasm_debug",
        "render_debug_panel(",
        "data-debug-source",
        "request_replay.run(",
        "trace.emit(",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "chart should not duplicate shared wasm debug runtime token `{forbidden}`",
        );
    }

    for required in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(required),
            "chart checklist should keep wasm-debug governance contract marker `{required}`",
        );
    }
}

#[test]
fn chart_wasm_debug_check_script_covers_shared_contract() {
    let script_source = include_str!("../../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm debug check script should enforce `{needle}`",
    );
}

#[test]
fn chart_does_not_introduce_spec_rs_without_stable_schema_contract_need() {
    let module = load_source("mod");
    let check2 = load_source("check2");
    let readme = load_source("readme");
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "chart should not add spec.rs without stable external schema contract",
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !module.contains(forbidden),
            "chart module should not export spec layer token `{forbidden}`",
        );
    }

    for required in [
        "`spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "components/chart/src/README.md",
        "components/chart/check2.md",
    ] {
        assert!(
            check2.contains(required),
            "check2 should document spec discipline evidence `{required}`",
        );
    }

    assert!(
        readme.contains("Chart"),
        "chart should keep component documentation in README instead of spec.rs",
    );
}

#[test]
fn chart_hyper_structure_builder_spec_is_not_applicable_for_simple_component_locally() {
    let module = load_source("mod");
    let check2 = load_source("check2");
    let readme = load_source("readme");
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "chart should keep `spec.rs` absent because this component has no complex external schema contract",
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "ChartSpec::new(",
        ".render()",
    ] {
        assert!(
            !module.contains(forbidden),
            "chart module should not expose hyper-structure builder token `{forbidden}` for simple-component scope",
        );
    }

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A：chart 当前不属于“复杂组件（稳定外部 schema/版本化 spec 契约）”范围",
        "components/chart/src/README.md",
    ] {
        assert!(
            check2.contains(required),
            "check2 should keep hyper-structure N/A marker `{required}`",
        );
    }

    assert!(
        readme.contains("Chart"),
        "chart should keep docs in README instead of introducing spec builder layer",
    );
}

#[test]
fn chart_component_files_check_script_covers_hyper_structure_builder_spec_na_locally() {
    let script_source = load_source("component_files_check_script");
    let required = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(required),
        "component-files check script should enforce `{required}`",
    );
}

#[test]
fn chart_check2_marks_hyper_structure_builder_spec_na_complete_locally() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "chart check2 should mark hyper-structure builder item complete with explicit N/A rationale",
    );

    for required in [
        "chart_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "chart_component_files_check_script_covers_hyper_structure_builder_spec_na",
        "scripts/check-ui-components-component-files.sh",
        "components/chart/src/spec.rs（不存在）",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 hyper-structure section should reference `{required}`",
        );
    }
}

#[test]
fn chart_context_compression_manifest_and_rbi_projection_are_present_and_current_locally() {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Component.toml");
    let rbi_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Component.rbi");
    assert!(
        manifest_path.exists(),
        "chart context-compression manifest should exist at `{}`",
        manifest_path.display()
    );
    assert!(
        rbi_path.exists(),
        "chart RBI signature projection should exist at `{}`",
        rbi_path.display()
    );

    let manifest_source = load_source("component_manifest");
    let rbi_source = load_source("component_rbi");

    for required in [
        "id = \"ui-chart\"",
        "name = \"Chart\"",
        "kind = \"snapshot\"",
        "manifest_version = 1",
        "rbi = \"Component.rbi\"",
        "mod_rs = \"src/mod.rs\"",
        "logic_rs = \"src/logic.rs\"",
        "styles_rs = \"src/styles.rs\"",
        "view_rs = \"src/view.rs\"",
        "motion_rs = \"src/motion.rs\"",
        "protocol_rs = \"src/protocol.rs\"",
        "context_compression_manifest = true",
        "rbi_signature_projection = true",
        "schema = \"ui.chart.agent-contract/v1\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "chart Component.toml should keep context-compression marker `{required}`",
        );
    }

    for required in [
        "component \"ui-chart\"",
        "signature Chart(",
        "mode: \"snapshot\"",
        "fallback: \"snapshot\"",
        "agent_contract_schema \"ui.chart.agent-contract/v1\"",
        "\"data-active-index\"",
        "\"data-active-value-source\"",
        "\"data-active-interaction-source\"",
        "\"data-motion-source\"",
    ] {
        assert!(
            rbi_source.contains(required),
            "chart RBI projection should include interface signature marker `{required}`",
        );
    }
}

#[test]
fn chart_component_files_check_script_covers_context_compression_manifest_and_rbi_locally() {
    let script_source = load_source("component_files_check_script");
    let required = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(required),
        "component-files check script should enforce `{required}`",
    );
}

#[test]
fn chart_check2_marks_context_compression_manifest_and_rbi_contract_complete_locally() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "chart check2 should mark context-compression manifest/rbi item complete",
    );

    for required in [
        "components/chart/Component.toml",
        "components/chart/Component.rbi",
        "chart_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "chart_component_files_check_script_covers_context_compression_manifest_and_rbi",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 context-compression section should reference `{required}`",
        );
    }
}

#[test]
fn chart_agent_contract_is_schema_typed_and_machine_readable_locally() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let manifest_source = load_source("component_manifest");

    for required in [
        "pub const CHART_AGENT_SCHEMA: &str = \"ui.chart.agent-contract\";",
        "pub enum ChartAgentSchemaVersion",
        "pub enum ChartAgentIntent",
        "pub enum ChartAgentAction",
        "pub enum ChartAgentKind",
        "pub enum ChartAgentState",
        "pub enum ChartAgentSource",
        "pub enum ChartAgentStreamSupport",
        "pub enum ChartAgentStreamFallback",
        "pub enum ChartAgentOutputStatus",
        "pub struct ChartAgentContract",
        "pub struct ChartAgentContractInput",
        "pub const fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(required),
            "chart logic should keep typed agent-contract marker `{required}`",
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
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            view_source.contains(required),
            "chart view should expose machine-readable agent-contract marker `{required}`",
        );
    }

    for required in [
        "schema = \"ui.chart.agent-contract/v1\"",
        "schema = \"data-ui-schema\"",
        "intent = \"data-ui-intent\"",
        "action = \"data-ui-action\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "chart Component.toml should keep agent-contract schema marker `{required}`",
        );
    }
}

#[test]
fn chart_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing_locally() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for required in [
        "kind: match input.state.kind {",
        "state: match input.state.state_attr {",
        "source: ChartAgentSource::StatePrimitives,",
        "stream_support: ChartAgentStreamSupport::Optional,",
        "stream_fallback: ChartAgentStreamFallback::Snapshot,",
        "output_status: ChartAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(required),
            "chart agent contract should derive field from typed state/source axis via `{required}`",
        );
    }

    for forbidden in [
        "data-ui-schema=\"ui.chart.agent-contract\"",
        "data-ui-intent=\"chart.interaction\"",
        "data-ui-action=\"navigate.activate\"",
        "format!(\"ui.chart",
        "schema_version.to_string()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "chart view should avoid free-form agent-contract string splicing token `{forbidden}`",
        );
    }
}

#[test]
fn chart_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally() {
    let manifest_source = load_source("component_manifest");
    let view_source = load_source("view");

    for required in [
        "[agent_contract_whitelist]",
        "logic::normalize_input_boundary(...)",
        "logic::derive_state_from_boundary(...)",
        "logic::resolve_agent_contract(...)",
        "use_chart(...)",
        "motion::attach_motion(...)",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            manifest_source.contains(required) || view_source.contains(required),
            "chart should keep whitelist-safe render path marker `{required}`",
        );
    }

    for forbidden in ["inner_html=", "<script", "javascript:", "eval("] {
        assert!(
            !view_source.contains(forbidden),
            "chart view must reject script-injection-prone token `{forbidden}`",
        );
    }
}

#[test]
fn chart_contract_hygiene_script_covers_agent_contract_schema_contract_locally() {
    let script_source = load_source("contract_hygiene_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(required),
            "contract-hygiene script should enforce `{required}`",
        );
    }
}

#[test]
fn chart_check2_documents_agent_contract_schema_governance_rules_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "chart_check2_documents_agent_contract_schema_governance_rules",
        "chart_agent_contract_is_schema_typed_and_machine_readable",
        "chart_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "chart_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 should keep agent-contract governance marker `{required}`",
        );
    }
}

#[test]
fn chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes_locally() {
    let check2_source = load_source("check2");
    let view_source = load_source("view");
    let script_source = load_source("streaming_check_script");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`Chart` 不是 LLM 正文渲染组件，定义仅用于与上层 LLM 输出协议对齐。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 should keep streaming-definition marker `{required}`",
        );
    }

    for required in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
    ] {
        assert!(
            view_source.contains(required),
            "chart view should expose snapshot-compatible stream marker `{required}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`",
    );
}

#[test]
fn chart_streaming_script_covers_two_mode_definition_contract_locally() {
    let script_source = load_source("streaming_check_script");
    let needle = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`",
    );
}

#[test]
fn chart_check2_marks_streaming_two_mode_definition_complete_locally() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "chart check2 should mark streaming two-mode definition gate complete",
    );

    for required in [
        "chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes_locally",
        "chart_streaming_script_covers_two_mode_definition_contract_locally",
        "chart_check2_marks_streaming_two_mode_definition_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "crates/ui-components/tests/chart_semantics.rs::chart_streaming_script_covers_two_mode_definition_contract",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_streaming_two_mode_definition_complete",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 streaming section should reference `{required}`",
        );
    }
}

#[test]
fn chart_check2_documents_snapshot_as_default_baseline_capability_locally() {
    let check2_source = load_source("check2");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "N/A：`Chart` 不直接渲染 LLM 正文；组件侧能力定义为“消费完整配置并一次性稳定渲染”。",
        "chart_check2_documents_snapshot_as_default_baseline_capability_locally",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 should keep snapshot-baseline marker `{required}`",
        );
    }
}

#[test]
fn chart_snapshot_baseline_consumes_complete_result_and_renders_stably_locally() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let check2_source = load_source("check2");
    let script_source = load_source("streaming_check_script");
    let combined = format!("{view_source}\n{logic_source}");

    for required in [
        "let normalized = logic::normalize_input_boundary(logic::ChartInputBoundary {",
        "let state = Signal::derive(move || {",
        "logic::derive_state_from_boundary(logic::ChartStateBoundary {",
        "let semantics = Signal::derive(move || {",
        "use_chart(ChartOptions {",
        "data-state=move || semantics.get().attrs.data_state",
        "data-active-index=move || state.get().active_index.to_string()",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
    ] {
        assert!(
            view_source.contains(required),
            "chart snapshot baseline should keep stable complete-result render marker `{required}`",
        );
    }

    for required in [
        "pub fn normalize_input_boundary(input: ChartInputBoundary) -> ChartNormalizedInput",
        "pub fn derive_state_from_boundary(input: ChartStateBoundary) -> ChartState",
        "pub fn normalize_interaction_index(",
        "stream_fallback: ChartAgentStreamFallback::Snapshot,",
    ] {
        assert!(
            logic_source.contains(required),
            "chart logic should keep snapshot-baseline normalization marker `{required}`",
        );
    }

    for forbidden in [
        "streaming_chunk",
        "token_delta",
        "partial token",
        "use_ai_space_state",
    ] {
        assert!(
            !combined.contains(forbidden),
            "chart snapshot baseline should avoid incremental streaming marker `{forbidden}`",
        );
    }

    for required in [
        "chart_snapshot_baseline_consumes_complete_result_and_renders_stably_locally",
        "data-ui-stream-mode=\"snapshot\"",
        "data-active-index=move || state.get().active_index.to_string()",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 snapshot section should reference `{required}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`",
    );
}

#[test]
fn chart_streaming_script_covers_snapshot_baseline_contract_locally() {
    let script_source = load_source("streaming_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(required),
            "streaming check script should enforce `{required}`",
        );
    }
}

#[test]
fn chart_check2_marks_snapshot_baseline_capability_complete_locally() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "chart check2 should mark snapshot baseline gate complete",
    );

    for required in [
        "chart_check2_documents_snapshot_as_default_baseline_capability_locally",
        "chart_snapshot_baseline_consumes_complete_result_and_renders_stably_locally",
        "chart_streaming_script_covers_snapshot_baseline_contract_locally",
        "chart_check2_marks_snapshot_baseline_capability_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_snapshot_as_default_baseline_capability",
        "crates/ui-components/tests/chart_semantics.rs::chart_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "crates/ui-components/tests/chart_semantics.rs::chart_streaming_script_covers_snapshot_baseline_contract",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_snapshot_baseline_capability_complete",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 snapshot section should reference `{required}`",
        );
    }
}

#[test]
fn chart_check2_documents_streaming_required_optional_classification_rules_locally() {
    let check2_source = load_source("check2");
    let script_source = load_source("streaming_check_script");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "N/A：`Chart` 归类为 `Streaming Optional`，仅消费 `Snapshot`，并固定 `fallback=snapshot`。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 should keep required/optional classification marker `{required}`",
        );
    }

    for script_required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_required),
            "streaming check script should enforce `{script_required}`",
        );
    }
}

#[test]
fn chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous_locally() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");

    for required in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-source=move || semantics.get().attrs.data_source",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "chart optional-streaming scope should keep semantic continuity marker `{required}`",
        );
    }

    for required in [
        "pub enum ChartAgentOutputStatus",
        "ChartAgentOutputStatus::Verified",
        "output_status: ChartAgentOutputStatus,",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "chart optional-streaming scope should expose explicit output-status marker `{required}`",
        );
    }
}

#[test]
fn chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer_locally() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let mod_source = load_source("mod");
    let motion_source = load_source("motion");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "network_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "chart should keep validation/retry/resilience policy outside component layer; found `{forbidden}`",
        );
    }
}

#[test]
fn chart_streaming_script_covers_required_optional_classification_contract_locally() {
    let script_source = load_source("streaming_check_script");

    for required in [
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(required),
            "streaming check script should enforce `{required}`",
        );
    }
}

#[test]
fn chart_check2_marks_streaming_required_optional_classification_complete_locally() {
    let check2_source = load_source("check2");

    assert!(
        check2_source.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
        "chart check2 should mark required/optional classification gate complete",
    );

    for required in [
        "chart_check2_documents_streaming_required_optional_classification_rules_locally",
        "chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous_locally",
        "chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer_locally",
        "chart_streaming_script_covers_required_optional_classification_contract_locally",
        "chart_check2_marks_streaming_required_optional_classification_complete_locally",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_documents_streaming_required_optional_classification_rules",
        "crates/ui-components/tests/chart_semantics.rs::chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "crates/ui-components/tests/chart_semantics.rs::chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "crates/ui-components/tests/chart_semantics.rs::chart_streaming_script_covers_required_optional_classification_contract",
        "crates/ui-components/tests/chart_semantics.rs::chart_check2_marks_streaming_required_optional_classification_complete",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 should keep required/optional classification evidence marker `{required}`",
        );
    }
}
