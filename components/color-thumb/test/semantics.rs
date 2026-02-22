fn load_source(path: &str) -> &'static str {
    match path {
        "lib" => include_str!("../src/lib.rs"),
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "protocol" => include_str!("../src/protocol.rs"),
        "check2" => include_str!("../check2.md"),
        "check2_src" => include_str!("../src/check2.md"),
        "legacy_semantics" => {
            include_str!("../../../components/color-thumb/test/color_thumb_semantics.rs")
        }
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn color_thumb_semantics_tests_are_migrated_to_component_directory() {
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let legacy_semantics = load_source("legacy_semantics");
    let local_semantics = include_str!("semantics.rs");

    for source in [lib_source, mod_source] {
        assert!(
            source.contains("#[path = \"../test/semantics.rs\"]")
                && source.contains("mod semantics_tests;"),
            "color-thumb should wire `components/color-thumb/test/semantics.rs` from both lib/mod entrypoints.",
        );
    }

    assert!(
        legacy_semantics.contains("color_thumb_"),
        "legacy ui semantics suite should stay available during migration.",
    );
    assert!(
        local_semantics.contains("color_thumb_semantics_tests_are_migrated_to_component_directory"),
        "component-local semantics suite should provide migration coverage.",
    );
}

#[test]
fn color_thumb_module_keeps_ui_components_boundaries() {
    let mod_source = load_source("mod");

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ColorThumbState, ColorThumbStateInput, DEFAULT_ARIA_LABEL, DEFAULT_COLOR};",
        "pub use motion::ColorThumbMotion;",
        "pub use view::ColorThumb;",
    ] {
        assert!(
            mod_source.contains(required),
            "color-thumb mod.rs should keep ui export boundary `{required}`.",
        );
    }
}

#[test]
fn color_thumb_logic_view_styles_motion_follow_assembly_contract() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");

    assert!(
        logic_source.contains("pub use ui_state_primitives::color_thumb::{"),
        "color-thumb logic should consume ui-state-primitives instead of redefining state machines.",
    );
    for required in [
        "pub struct ColorThumbLogicInput",
        "ColorThumbInputSource",
        "ColorThumbAriaValueTextSource",
        "pub fn source_from_option<T>(",
        "ColorThumbInteractionState",
        "pub fn normalize_position_percent(",
        "pub fn resolve_component_state(",
        "pub fn interaction_state_from_flags(",
    ] {
        assert!(
            logic_source.contains(required),
            "color-thumb logic should keep centralized default normalization via `{required}`.",
        );
    }

    for required in [
        "use ui_headless::{A11yDirection, ColorThumbOptions, use_color_thumb};",
        "logic::interaction_state_from_flags(",
        "logic::resolve_component_state(logic::ColorThumbLogicInput {",
        "let (aria_value_text, aria_value_text_source) =",
        "aria_value_text_source,",
        "use_color_thumb(ColorThumbOptions {",
        "data-interaction-source=move || semantics.get().root_attrs.data_interaction_source",
        "data-aria-valuetext-source=move || semantics.get().root_attrs.data_aria_valuetext_source",
        "data-loupe-source=move || semantics.get().root_attrs.data_loupe_source",
        "data-x-source=move || semantics.get().root_attrs.data_x_source",
        "data-y-source=move || semantics.get().root_attrs.data_y_source",
        "let motion = motion::sanitize_motion(motion);",
        "motion::attach_motion(None, motion)",
    ] {
        assert!(
            view_source.contains(required),
            "color-thumb view should assemble shell semantics via `{required}`.",
        );
    }
    for forbidden in [
        "logic::resolve_state(ColorThumbStateInput {",
        "resolve_state(ColorThumbStateInput {",
        "ColorThumbStateInput {",
        "unwrap_or_else(|| \"None\".to_string())",
        "<Show when=move || semantics.get().state.loupe_visible>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-thumb view should not rebuild state normalization rule `{forbidden}`.",
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str =") && styles_source.contains("var(--ui-"),
        "color-thumb styles should remain token-first static css.",
    );

    for required in [
        "pub struct ColorThumbMotion",
        "pub spring: ui_motion::spring::SpringConfig",
        "pub fn sanitize_motion(",
        "ui_motion::spring::sanitize_config(",
        "pub fn attach_motion(",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-color-thumb-motion-stiffness",
        "--ui-color-thumb-motion-damping",
        "--ui-color-thumb-motion-mass",
        "--ui-color-thumb-motion-precision",
    ] {
        assert!(
            motion_source.contains(required),
            "color-thumb motion should keep semantic-to-runtime mapping `{required}`.",
        );
    }
}

#[test]
fn color_thumb_component_files_keep_single_responsibility() {
    let css_registry_source = include_str!("../../../crates/ui/src/css.rs");
    let ui_root_source = include_str!("../../../crates/ui/src/root.rs");
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::ColorThumb;",
    ] {
        assert!(
            mod_source.contains(required),
            "color-thumb mod.rs should keep export boundary marker `{required}`.",
        );
    }

    for forbidden in [
        "fn resolve_component_state(",
        "pub const CSS: &str",
        "view! {",
        "on:pointerdown",
        "mod spec;",
        "pub mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-thumb mod.rs should not carry implementation details like `{forbidden}`.",
        );
    }
    for forbidden in ["mod spec;", "pub mod spec;", "spec::"] {
        assert!(
            !lib_source.contains(forbidden),
            "color-thumb lib.rs should not expose spec module token `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ColorThumbLogicInput",
        "pub fn resolve_component_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "color-thumb logic.rs should keep normalization/derivation marker `{required}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "view! {",
        "on:pointerdown",
        "use ui_headless::",
        "pub const CSS: &str",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "color-thumb logic.rs should not include DOM/view/style detail `{forbidden}`.",
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str =")
            && styles_source.contains("var(--ui-")
            && styles_source.contains(".ui-color-thumb"),
        "color-thumb styles.rs should remain token-first static css contract.",
    );
    for forbidden in [
        "fn ",
        "use ui_headless::",
        "resolve_component_state(",
        "on:pointerdown",
        "web_sys::",
        "--tw-",
        "tailwind",
        "styled(",
        "Style::new(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "color-thumb styles.rs should not include runtime logic token `{forbidden}`.",
        );
    }

    for required in [
        "use ui_headless::{A11yDirection, ColorThumbOptions, use_color_thumb};",
        "logic::resolve_component_state(logic::ColorThumbLogicInput {",
        "use_color_thumb(ColorThumbOptions {",
    ] {
        assert!(
            view_source.contains(required),
            "color-thumb view.rs should mount structure + headless contract via `{required}`.",
        );
    }
    for forbidden in [
        "pub const CSS: &str",
        "request_animation_frame",
        "cancel_animation_frame",
        "web_sys::",
        "style! {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-thumb view.rs should not include style-engine detail `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ColorThumbMotion",
        "pub spring: ui_motion::spring::SpringConfig",
        "pub fn sanitize_motion(",
        "ui_motion::spring::sanitize_config(",
        "pub fn attach_motion(",
        "--ui-color-thumb-motion-stiffness",
        "--ui-color-thumb-motion-damping",
        "--ui-color-thumb-motion-mass",
        "--ui-color-thumb-motion-precision",
    ] {
        assert!(
            motion_source.contains(required),
            "color-thumb motion.rs should keep motion contract mapping via `{required}`.",
        );
    }
    for forbidden in [
        "SpringAnimator",
        "request_animation_frame",
        "cancel_animation_frame",
        "web_sys::",
        "wasm_bindgen::",
        "view! {",
        "@keyframes",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "color-thumb motion.rs should not include shared engine/view token `{forbidden}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-color_thumb\")]",
        "out.push_str(crate::color::thumb::styles::CSS);",
    ] {
        assert!(
            css_registry_source.contains(required),
            "ui css registry should aggregate color-thumb styles via `{required}`.",
        );
    }
    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root_source.contains(required),
            "UiRoot should inject aggregated component css via `{required}`.",
        );
    }
    for forbidden in [
        "tailwind",
        "--tw-",
        "styled_components",
        "emotion",
        "StyleSheet::",
    ] {
        assert!(
            !css_registry_source.contains(forbidden) && !ui_root_source.contains(forbidden),
            "component css pipeline should not depend on utility-first/css-in-rust token `{forbidden}`.",
        );
    }
}

#[test]
fn color_thumb_public_api_uses_prefix_naming_contract() {
    let view_source = load_source("view");

    for required in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_focused: bool",
        "#[prop(optional)] is_dragging: bool",
        "#[prop(optional)] is_loupe_visible: Option<bool>",
        "#[prop(optional, into)] aria_value_text: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional)] x_percent: Option<f32>",
        "#[prop(optional)] y_percent: Option<f32>",
    ] {
        assert!(
            view_source.contains(required),
            "color-thumb bool props should follow `is_*` naming; missing `{required}`.",
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] focused: bool",
        "#[prop(optional)] dragging: bool",
        "#[prop(optional)] loupe_visible: bool",
        "#[prop(optional)] show_loupe: bool",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-thumb should not expose legacy bool alias `{forbidden}`.",
        );
    }

    for forbidden in [
        "#[prop(optional, default = logic::DEFAULT_POSITION_PERCENT)] x_percent: f32",
        "#[prop(optional, default = logic::DEFAULT_POSITION_PERCENT)] y_percent: f32",
        "#[prop(optional, default = true)] is_loupe_visible: bool",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-thumb defaults should not be normalized in view.rs; found `{forbidden}`.",
        );
    }
}

#[test]
fn color_thumb_controlled_uncontrolled_pairing_is_not_applicable() {
    let view_source = load_source("view");
    let check2 = load_source("check2");
    let check2_src = load_source("check2_src");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "on_value_change: Callback",
        "on_value_change: impl",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-thumb should remain a stateless semantic shell; found controlled/uncontrolled axis marker `{forbidden}`.",
        );
    }

    for source in [check2, check2_src] {
        assert!(
            source.contains("- [x] 受控/非受控必须成对"),
            "color-thumb check2 should mark controlled/uncontrolled pairing item as completed.",
        );
        assert!(
            source.contains("N/A（已论证）"),
            "color-thumb check2 should record explicit N/A rationale for controlled/uncontrolled pairing.",
        );
    }
}

#[test]
fn color_thumb_public_surface_does_not_expose_dom_platform_types() {
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
            "color-thumb crate public entry should not expose `{forbidden}`.",
        );
        assert!(
            !mod_source.contains(forbidden),
            "color-thumb ui module should not expose `{forbidden}`.",
        );
    }
}

#[test]
fn color_thumb_semantic_tests_cover_interaction_and_platform_matrix() {
    let workspace_semantics = load_source("legacy_semantics");
    let headless_tests = include_str!("../../../crates/ui-headless/src/test/color_thumb.rs");
    let motion_tests = include_str!("motion.rs");

    for required in [
        "fn color_thumb_exposes_baseline_style_data_markers()",
        "data-state=move || semantics.get().root_attrs.data_state",
        "data-aria-valuetext-source=move || semantics.get().root_attrs.data_aria_valuetext_source",
        "assert_eq!(contract.root_attrs.role, \"slider\");",
        "assert_eq!(contract.root_attrs.data_state, \"dragging\");",
        "fn color_thumb_keyboard_contract_prevents_slider_navigation_defaults()",
        "on_key_down.run(\"ArrowLeft\".to_string())",
        "fn color_thumb_pointer_and_focus_handlers_are_callable()",
        "on_pointer_down.run(())",
        "on_pointer_cancel.run(())",
        "if ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            workspace_semantics.contains(required)
                || headless_tests.contains(required)
                || motion_tests.contains(required),
            "color-thumb semantic regression matrix should include `{required}`.",
        );
    }

    for forbidden in [
        "insta::assert_snapshot!",
        "insta::assert_debug_snapshot!",
        "assert_json_snapshot!",
        "toMatchSnapshot(",
    ] {
        assert!(
            !headless_tests.contains(forbidden) && !motion_tests.contains(forbidden),
            "color-thumb semantic verification should not depend on visual snapshot token `{forbidden}`.",
        );
    }
}

#[test]
fn color_thumb_performance_governance_budget_is_defined_and_blocking() {
    let check2 = load_source("check2");
    let check2_src = load_source("check2_src");
    let docs_shell_source = include_str!("../../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = include_str!("../../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = include_str!("../../../docs/plan/TODO.md");
    let script_source = include_str!("../../../scripts/check-ui-performance.sh");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");

    for source in [check2, check2_src] {
        assert!(
            source.contains("- [x] 性能治理：关键路径有预算"),
            "color-thumb check2 should mark performance governance item as completed.",
        );
        for needle in [
            "\"color-thumb\" => UiPerfBudget { max_mount_ms: 24.0, max_update_ms: Some(8.0), max_heap_kb: Some(384.0) }",
            "data-perf-mount-ms",
            "data-perf-budget-ms",
            "data-perf-observability",
            "data-perf-violation",
            "render_count",
            "mount-only 等价证据",
            "color_thumb_performance_governance_budget_is_defined_and_blocking",
            "color_thumb_performance_governance_contract_is_budgeted_traceable_and_blocking",
            "`scripts/check-ui-performance.sh`",
        ] {
            assert!(
                source.contains(needle),
                "color-thumb check2 performance section should include `{needle}`.",
            );
        }
    }

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "\"color-thumb\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            docs_shell_source.contains(needle),
            "docs shell should keep color-thumb perf budget token `{needle}`.",
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose marker `{needle}` for perf governance.",
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
            "docs e2e coverage should keep perf probe assertion `{needle}`.",
        );
    }

    assert!(
        todo_source.contains("render_count") && todo_source.contains("mount-only 等价证据"),
        "docs TODO should keep render_count follow-up tracking item.",
    );

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_performance_governance_contract_is_budgeted_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );

    for needle in [
        "data-state=move || semantics.get().root_attrs.data_state",
        "data-interaction-source=move || semantics.get().root_attrs.data_interaction_source",
        "data-motion-source=motion_source",
        "--ui-color-thumb-handle-duration",
        "--ui-color-thumb-loupe-duration",
        "pub fn resolve_runtime_motion(",
    ] {
        assert!(
            view_source.contains(needle)
                || styles_source.contains(needle)
                || motion_source.contains(needle),
            "color-thumb source should keep perf attribution anchor `{needle}`.",
        );
    }
}

#[test]
fn color_thumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let local_semantics = include_str!("semantics.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let check2 = load_source("check2");
    let check2_src = load_source("check2_src");
    let view_source = load_source("view");
    let headless_tests = include_str!("../../../crates/ui-headless/src/test/color_thumb.rs");
    let todo_source = include_str!("../../../docs/plan/TODO.md");
    let script_source = include_str!("../../../scripts/check-ui-performance.sh");

    for required_test in [
        "fn color_thumb_semantic_tests_cover_interaction_and_platform_matrix()",
        "fn color_thumb_performance_governance_budget_is_defined_and_blocking()",
        "fn color_thumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && legacy_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests.",
        );
    }

    for marker in [
        "role=move || semantics.get().root_attrs.role",
        "aria-label=move || semantics.get().root_attrs.aria_label",
        "data-state=move || semantics.get().root_attrs.data_state",
        "data-focused=move || semantics.get().root_attrs.data_focused",
        "data-interaction-source=move || semantics.get().root_attrs.data_interaction_source",
        "on:focus=move |_| semantics.get().handlers.on_focus.run(())",
        "on:blur=move |_| semantics.get().handlers.on_blur.run(())",
        "on:keydown=move |ev| {",
    ] {
        assert!(
            view_source.contains(marker),
            "color-thumb view should expose semantic/focus marker `{marker}`.",
        );
    }

    for required in [
        "fn color_thumb_pointer_and_focus_handlers_are_callable()",
        "enabled.handlers.on_focus.run(());",
        "enabled.handlers.on_blur.run(());",
        "assert!(enabled.handlers.on_key_down.run(\"ArrowLeft\".to_string()));",
    ] {
        assert!(
            headless_tests.contains(required),
            "color-thumb headless suite should keep focus-flow contract token `{required}`.",
        );
    }

    for required in ["render_count", "mount-only 等价证据"] {
        assert!(
            todo_source.contains(required),
            "render_count tracking contract should keep `{required}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(script_needle),
            "performance check script should include `{script_needle}`.",
        );
    }

    for source in [check2, check2_src] {
        for check_needle in [
            "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
            "color_thumb_semantic_tests_cover_interaction_and_platform_matrix",
            "color_thumb_performance_governance_budget_is_defined_and_blocking",
            "color_thumb_performance_governance_contract_is_budgeted_traceable_and_blocking",
            "color_thumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
            "render_count",
            "mount-only 等价证据",
        ] {
            assert!(
                source.contains(check_needle),
                "color-thumb check2 semantic/performance entry should keep `{check_needle}`.",
            );
        }
    }
}

#[test]
fn color_thumb_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only() {
    let check2 = load_source("check2");
    let check2_src = load_source("check2_src");
    let view_source = load_source("view");
    let local_semantics = include_str!("semantics.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "role=move || semantics.get().root_attrs.role",
        "aria-label=move || semantics.get().root_attrs.aria_label",
        "aria-disabled=move || semantics.get().root_attrs.aria_disabled",
        "aria-valuetext=move || semantics.get().root_attrs.aria_valuetext",
        "data-state=move || semantics.get().root_attrs.data_state",
        "data-interaction-source=move || semantics.get().root_attrs.data_interaction_source",
        "data-aria-source=move || semantics.get().root_attrs.data_aria_source",
        "data-aria-valuetext-source=move || semantics.get().root_attrs.data_aria_valuetext_source",
        "on:keydown=move |ev| {",
    ] {
        assert!(
            view_source.contains(required),
            "color-thumb view should keep semantic-priority marker `{required}`.",
        );
    }

    for required in [
        "fn color_thumb_semantic_tests_cover_interaction_and_platform_matrix()",
        "fn color_thumb_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "fn color_thumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn color_thumb_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only()",
    ] {
        assert!(
            local_semantics.contains(required) && legacy_semantics.contains(required),
            "semantic-priority suite should include `{required}` in local + aggregated tests.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should enforce `{script_needle}`.",
    );

    for source in [check2, check2_src] {
        for required in [
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
            "color_thumb_semantic_tests_cover_interaction_and_platform_matrix",
            "color_thumb_snapshot_baseline_consumes_complete_result_and_renders_stably",
            "color_thumb_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only",
            "`scripts/check-ui-contract-hygiene.sh`",
        ] {
            assert!(
                source.contains(required),
                "color-thumb semantic-priority checklist should include `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_check2_documents_e2e_selector_and_stable_wait_rules() {
    let script_source =
        include_str!("../../../components/color-thumb/scripts/check-ui-e2e-color-thumb.sh");

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
            "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
            "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
            "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
            "color_thumb_check2_documents_e2e_selector_and_stable_wait_rules",
            "color_thumb_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
            "color_thumb_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints",
            "components/color-thumb/scripts/check-ui-e2e-color-thumb.sh",
        ] {
            assert!(
                source.contains(required),
                "color-thumb check2 should keep e2e-selector/stable-wait governance marker `{required}`.",
            );
        }
    }

    for script_needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(script_needle),
            "e2e-color-thumb gate script should include `{script_needle}`.",
        );
    }
}

#[test]
fn color_thumb_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_color_thumb_contract.spec.mjs");
    let script_source =
        include_str!("../../../components/color-thumb/scripts/check-ui-e2e-color-thumb.sh");

    for required in [
        "const COLOR_THUMB_PAGE = \"/#/components/color-thumb\";",
        "const WASM_READY_SELECTOR = \"body:not(:has(#boot))\";",
        "[data-component=\"color-thumb\"] #docs-color-thumb-idle[data-slot=\"color-thumb\"]",
        "data-slot=\"color-thumb-handle\"",
        "data-slot=\"color-thumb-fill\"",
        "data-slot=\"color-thumb-loupe\"",
        "data-slot=\"color-thumb-loupe-fill\"",
        "data-ui-schema",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-output-status",
        "data-interaction-source",
        "data-aria-source",
        "data-loupe-source",
        "data-x-source",
        "data-y-source",
        "role",
        "aria-label",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-thumb e2e contract should include semantic selector/wait marker `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "color-thumb e2e contract should avoid brittle selector/wait token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_selector_contract_uses_semantic_markers_and_stable_waits";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-thumb gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_color_thumb_contract.spec.mjs");
    let script_source =
        include_str!("../../../components/color-thumb/scripts/check-ui-e2e-color-thumb.sh");

    for required in [
        "focused/dragging/disabled/custom branches stay on semantic ready and settled breakpoints",
        "#docs-color-thumb-focused[data-slot=\"color-thumb\"]",
        "#docs-color-thumb-dragging[data-slot=\"color-thumb\"]",
        "#docs-color-thumb-disabled[data-slot=\"color-thumb\"]",
        "#docs-color-thumb-custom[data-slot=\"color-thumb\"]",
        "toHaveAttribute(\"data-state\", \"focused\")",
        "toHaveAttribute(\"data-state\", \"dragging\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-ui-action\", \"idle\")",
        "toHaveAttribute(\"data-ui-action\", \"focus\")",
        "toHaveAttribute(\"data-ui-action\", \"drag\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "focus()",
        "toBeFocused()",
        "dispatchEvent(\"pointerdown\")",
        "dispatchEvent(\"pointerup\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-thumb e2e motion/settled contract should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-thumb gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_check2_documents_repeatable_e2e_regression_collection() {
    let script_source =
        include_str!("../../../components/color-thumb/scripts/check-ui-e2e-color-thumb.sh");

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
            "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
            "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
            "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
            "docs-app color-thumb key flow is repeatable and failures map to semantic breakpoints",
            "docs-app color-thumb high-risk paths keep focus keyboard and disabled branches semantically explicit",
            "overlay/async N/A",
            "color_thumb_check2_documents_repeatable_e2e_regression_collection",
            "color_thumb_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
            "color_thumb_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
            "components/color-thumb/scripts/check-ui-e2e-color-thumb.sh",
        ] {
            assert!(
                source.contains(required),
                "color-thumb check2 should keep repeatable e2e regression marker `{required}`.",
            );
        }
    }

    for script_needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_repeatable_e2e_regression_collection",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(script_needle),
            "e2e-color-thumb gate script should include `{script_needle}`.",
        );
    }
}

#[test]
fn color_thumb_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_color_thumb_contract.spec.mjs");
    let script_source =
        include_str!("../../../components/color-thumb/scripts/check-ui-e2e-color-thumb.sh");

    for required in [
        "key flow is repeatable and failures map to semantic breakpoints",
        "root.focus()",
        "toBeFocused()",
        "keyboard.press(\"ArrowRight\")",
        "data-ui-action\", \"idle\"",
        "data-ui-state\", \"idle\"",
        "data-ui-source\", \"default\"",
        "data-ui-output-status\", \"verified\"",
        "await page.reload();",
        "keyboard.press(\"ArrowLeft\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "repeatable color-thumb e2e flow should include semantic breakpoint `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-thumb gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_color_thumb_contract.spec.mjs");
    let script_source =
        include_str!("../../../components/color-thumb/scripts/check-ui-e2e-color-thumb.sh");

    for required in [
        "high-risk paths keep focus keyboard and disabled branches semantically explicit",
        "focusedRoot.focus()",
        "keyboard.press(\"ArrowRight\")",
        "data-state\", \"focused\"",
        "data-focused\", \"true\"",
        "data-ui-action\", \"focus\"",
        "dispatchEvent(\"pointerdown\")",
        "data-state\", \"dragging\"",
        "data-dragging\", \"true\"",
        "data-ui-action\", \"drag\"",
        "dispatchEvent(\"pointerup\")",
        "data-state\", \"disabled\"",
        "data-disabled\", \"true\"",
        "aria-disabled\", \"true\"",
        "tabindex\", \"-1\"",
        "data-ui-output-status\", \"verified\"",
    ] {
        assert!(
            e2e_source.contains(required),
            "high-risk color-thumb e2e path should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-thumb gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("view");
    let check2 = load_source("check2");
    let check2_src = load_source("check2_src");
    let script_source = include_str!("../../../scripts/check-ui-view-macro.sh");

    for needle in [
        "fn render_decorative_swatch(color: Option<String>, class_name: &str) -> AnyView",
        "render_decorative_swatch(color.get_value(), CLASS_COLOR_THUMB_SWATCH)",
        "render_decorative_swatch(color.get_value(), CLASS_COLOR_THUMB_LOUPE_SWATCH)",
    ] {
        assert!(
            view_source.contains(needle),
            "color-thumb view.rs should keep macro-splitting token `{needle}`.",
        );
    }

    assert!(
        !view_source.contains("if let Some(color) = color.get_value()"),
        "color-thumb view.rs should not keep duplicated inline color branch nesting.",
    );

    for source in [check2, check2_src] {
        assert!(
            source.contains("- [x] `view!` 宏复杂度受控"),
            "color-thumb check2 should mark view-macro complexity item as completed.",
        );
        assert!(
            source.contains("`render_decorative_swatch(color, class_name)`"),
            "color-thumb check2 should include explicit local subrender extraction evidence.",
        );
        assert!(
            source.contains("`if let Some(color) = color.get_value()`"),
            "color-thumb check2 should include explicit rationale for removing nested inline branches.",
        );
        assert!(
            source.contains("color_thumb_view_macro_complexity_is_split_into_semantic_subrenders"),
            "color-thumb check2 should include explicit view-macro regression test evidence.",
        );
        assert!(
            source.contains("`scripts/check-ui-view-macro.sh`"),
            "color-thumb check2 should include explicit view-macro gate script evidence.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("view");
    let check2 = load_source("check2");
    let check2_src = load_source("check2_src");
    let script_source = include_str!("../../../scripts/check-ui-view-macro.sh");

    assert!(
        view_source.contains(
            "fn render_decorative_swatch(color: Option<String>, class_name: &str) -> AnyView"
        ),
        "color-thumb view.rs should expose plain function split helper for simple repeated fragments.",
    );
    assert!(
        view_source.contains("#[component]\npub fn ColorThumb("),
        "color-thumb view.rs should keep a single public component entrypoint.",
    );
    assert_eq!(
        view_source.match_indices("#[component]").count(),
        1,
        "color-thumb view.rs should avoid adding extra local #[component] fragments.",
    );
    assert!(
        !view_source.contains("#[component]\nfn render_decorative_swatch("),
        "color-thumb split helper should remain plain function, not a local #[component].",
    );

    for source in [check2, check2_src] {
        for needle in [
            "- [x] 函数式拆分优先",
            "`render_decorative_swatch(color, class_name) -> AnyView`",
            "`#[component] pub fn ColorThumb(...)`",
            "color_thumb_view_functional_split_prefers_plain_functions_over_local_components",
            "`scripts/check-ui-view-macro.sh`",
        ] {
            assert!(
                source.contains(needle),
                "color-thumb check2 function-split section should include `{needle}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("view");
    let check2 = load_source("check2");
    let check2_src = load_source("check2_src");
    let script_source = include_str!("../../../scripts/check-ui-view-macro.sh");

    for forbidden in [
        "inner_html=",
        "<footer",
        "<nav",
        "<article",
        "<section",
        "<path",
        "let markdown",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-thumb view.rs should avoid heavy inline static fragments in simple layout `{forbidden}`.",
        );
    }

    for needle in [
        "const SLOT_COLOR_THUMB: &str = \"color-thumb\";",
        "const SLOT_COLOR_THUMB_HANDLE: &str = \"color-thumb-handle\";",
        "const SLOT_COLOR_THUMB_FILL: &str = \"color-thumb-fill\";",
        "const SLOT_COLOR_THUMB_LOUPE: &str = \"color-thumb-loupe\";",
        "const SLOT_COLOR_THUMB_LOUPE_FILL: &str = \"color-thumb-loupe-fill\";",
        "const CLASS_COLOR_THUMB_HANDLE: &str = \"ui-color-thumb__handle\";",
        "const CLASS_COLOR_THUMB_FILL: &str = \"ui-color-thumb__fill\";",
        "const CLASS_COLOR_THUMB_LOUPE: &str = \"ui-color-thumb__loupe\";",
        "const CLASS_COLOR_THUMB_LOUPE_FILL: &str = \"ui-color-thumb__loupe-fill\";",
        "const CLASS_COLOR_THUMB_SWATCH: &str = \"ui-color-thumb__swatch\";",
        "const CLASS_COLOR_THUMB_LOUPE_SWATCH: &str = \"ui-color-thumb__loupe-swatch\";",
        "const BOOL_TRUE: &str = \"true\";",
        "data-slot=SLOT_COLOR_THUMB",
        "data-slot=SLOT_COLOR_THUMB_HANDLE",
        "data-slot=SLOT_COLOR_THUMB_FILL",
        "data-slot=SLOT_COLOR_THUMB_LOUPE",
        "data-slot=SLOT_COLOR_THUMB_LOUPE_FILL",
        "class=CLASS_COLOR_THUMB_HANDLE",
        "class=CLASS_COLOR_THUMB_FILL",
        "class=CLASS_COLOR_THUMB_LOUPE",
        "class=CLASS_COLOR_THUMB_LOUPE_FILL",
        "render_decorative_swatch(color.get_value(), CLASS_COLOR_THUMB_SWATCH)",
        "render_decorative_swatch(color.get_value(), CLASS_COLOR_THUMB_LOUPE_SWATCH)",
        "aria-hidden=BOOL_TRUE",
    ] {
        assert!(
            view_source.contains(needle),
            "color-thumb static fragment constantization should keep `{needle}`.",
        );
    }

    for source in [check2, check2_src] {
        for needle in [
            "- [x] 静态片段常量化",
            "`SLOT_COLOR_THUMB*`、`CLASS_COLOR_THUMB*`、`BOOL_TRUE`",
            "color_thumb_static_fragments_are_constantized_or_absent_for_simple_layout",
            "`scripts/check-ui-view-macro.sh`",
        ] {
            assert!(
                source.contains(needle),
                "color-thumb check2 static-fragment section should include `{needle}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let script_source = include_str!("../../../scripts/check-ui-inner-html.sh");

    for (rel_path, source) in [
        (
            "../../components/color-thumb/src/mod.rs",
            load_source("mod"),
        ),
        (
            "../../components/color-thumb/src/logic.rs",
            load_source("logic"),
        ),
        (
            "../../components/color-thumb/src/styles.rs",
            load_source("styles"),
        ),
        (
            "../../components/color-thumb/src/motion.rs",
            load_source("motion"),
        ),
        (
            "../../components/color-thumb/src/view.rs",
            load_source("view"),
        ),
        (
            "../../apps/docs-app/src/pages/components/pages/forms_color.rs",
            include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs"),
        ),
    ] {
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "dangerouslySetInnerHTML",
        ] {
            assert!(
                !source.contains(forbidden),
                "ColorThumb path `{rel_path}` must not inject raw html; found `{forbidden}`."
            );
        }
    }

    for source in [load_source("check2"), load_source("check2_src")] {
        for needle in [
            "- [x] `inner_html` 使用约束",
            "仅允许编译期常量或明确白名单内容进入 `inner_html`",
            "使用 `inner_html` 的节点必须补语义测试与安全回归说明",
            "color_thumb_inner_html_usage_is_forbidden_in_component_and_docs_examples",
            "`scripts/check-ui-inner-html.sh`",
        ] {
            assert!(
                source.contains(needle),
                "ColorThumb checklist should keep inner_html security contract marker `{needle}`."
            );
        }
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should enforce ColorThumb contract marker `{script_needle}`.",
    );
}

#[test]
fn color_thumb_wasm_debug_contract_reuses_shared_trace_and_stays_feature_isolated() {
    let cargo_source = include_str!("../../../crates/ui/Cargo.toml");
    let crate_root_source = include_str!("../../../crates/ui/src/lib.rs");
    let docs_app_source = include_str!("../../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = include_str!("../../../crates/ui-headless/src/trace.rs");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let motion_source = load_source("motion");

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep shared wasm-debug marker `{needle}`.",
        );
    }
    assert!(
        !cargo_source.contains("color-thumb-wasm-debug")
            && !cargo_source.contains("color_thumb-wasm-debug")
            && !cargo_source.contains("color_thumb_wasm_debug"),
        "ColorThumb should not expose a component-local wasm-debug feature.",
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui root should keep wasm-debug isolation marker `{needle}`.",
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs-app should keep debug-only wasm trace visual entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "global debug overlay should keep marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "global trace model should keep typed source/timestamp marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || semantics.get().root_attrs.data_state",
        "data-interaction-source=move || semantics.get().root_attrs.data_interaction_source",
        "data-aria-valuetext-source=move || semantics.get().root_attrs.data_aria_valuetext_source",
        "data-x-source=move || semantics.get().root_attrs.data_x_source",
        "data-y-source=move || semantics.get().root_attrs.data_y_source",
        "on:pointerdown=move |_| semantics.get().handlers.on_pointer_down.run(())",
        "on:pointerup=move |_| semantics.get().handlers.on_pointer_up.run(())",
        "on:pointercancel=move |_| semantics.get().handlers.on_pointer_cancel.run(())",
        "on:focus=move |_| semantics.get().handlers.on_focus.run(())",
        "on:blur=move |_| semantics.get().handlers.on_blur.run(())",
        "on:keydown=move |ev| {",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorThumb should expose machine-readable trace/replay markers via `{needle}`.",
        );
    }

    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "trace.emit(",
        "use_ui_trace(",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorThumb production contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    for source in [load_source("check2"), load_source("check2_src")] {
        for needle in [
            "WASM 调试要求：关键状态可追踪",
            "开发模式下至少能追踪关键状态变更来源与前后值",
            "关键交互链路应支持最小可复现记录",
            "调试开关默认不进入生产包体与公共 API",
            "color_thumb_wasm_debug_contract_reuses_shared_trace_and_stays_feature_isolated",
            "`scripts/check-ui-wasm-debug.sh`",
        ] {
            assert!(
                source.contains(needle),
                "ColorThumb checklist should keep wasm-debug governance marker `{needle}`.",
            );
        }
    }
}

#[test]
fn color_thumb_wasm_debug_check_script_covers_shared_contract() {
    let script_source = include_str!("../../../scripts/check-ui-wasm-debug.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_wasm_debug_contract_reuses_shared_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce ColorThumb contract marker `{needle}`.",
    );
}

#[test]
fn color_thumb_dx_playground_supports_css_hot_reload_and_context_with_optional_persist_na() {
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let dx_script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "let scope_selector = format!(\"[data-playground-scope=\\\"{scope_id}\\\"]\");",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "<div data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "class=\"playground__test-editor\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "data-slot=\"playground-controls\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground should keep DX hot-style-feedback + isolated-canvas token `{required}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn color_thumb() -> AnyView {")
        .unwrap_or_else(|| panic!("forms_color docs should contain color_thumb section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn color_editor() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("forms_color docs should contain color_editor section after color_thumb")
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "title=\"ColorThumb\"",
        "slug=\"color-thumb\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Focused + Dragging + Position\"",
        "Playground title=\"Disabled + Custom Class + Loupe Off\"",
        "id_base=\"docs-color-thumb-hello\".to_string()",
        "id_base=\"docs-color-thumb-idle\".to_string()",
        "id_base=\"docs-color-thumb-focused\".to_string()",
        "id_base=\"docs-color-thumb-dragging\".to_string()",
        "id_base=\"docs-color-thumb-disabled\".to_string()",
        "id_base=\"docs-color-thumb-custom\".to_string()",
        "is_focused=true",
        "is_dragging=true",
        "is_disabled=true",
        "is_loupe_visible=false",
        "class_name=\"docs-color-thumb-custom\".to_string()",
    ] {
        assert!(
            section.contains(required),
            "ColorThumb docs should provide isolated demo/playground token `{required}`.",
        );
    }

    for forbidden in [
        "Persist workbench state",
        "workbench_persist_state",
        "load_color_thumb_workbench_state",
        "save_color_thumb_workbench_state",
        "clear_color_thumb_workbench_state",
        "localStorage",
        "sessionStorage",
    ] {
        assert!(
            !section.contains(forbidden),
            "ColorThumb keeps optional persisted workbench state as N/A; token `{forbidden}` should stay absent.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_dx_playground_supports_css_hot_reload_and_context_with_optional_persist_na";
    assert!(
        dx_script_source.contains(
            "echo \"[dx] contract: color-thumb playground css hot-reload + isolated demo\""
        ) && dx_script_source.contains(script_needle),
        "DX gate script should include ColorThumb contract markers.",
    );

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
            "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
            "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
            "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
            "color_thumb_dx_playground_supports_css_hot_reload_and_context_with_optional_persist_na",
            "`scripts/check-ui-dx.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep DX governance rule `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_dx_check_script_covers_hot_reload_and_demo_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_dx_playground_supports_css_hot_reload_and_context_with_optional_persist_na";
    assert!(
        script_source.contains(needle),
        "DX check script should enforce ColorThumb contract marker `{needle}`.",
    );
}

#[test]
fn color_thumb_check2_documents_docs_sync_and_state_matrix_rules() {
    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
            "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
            "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
            "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
            "color_thumb_check2_documents_docs_sync_and_state_matrix_rules",
            "color_thumb_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
            "`scripts/check-ui-dx.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep docs-sync/state-matrix rule `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let view_source = load_source("view");
    let logic_source = load_source("logic");

    for required in [
        "pub(super) fn color_thumb() -> AnyView",
        "Playground title=\"Hello World\" code_signal=hello_code",
        "Playground title=\"State Matrix\"",
        "data-slot=\"color-thumb-state-matrix\"",
        "id_base=\"docs-color-thumb-matrix-idle\".to_string()",
        "id_base=\"docs-color-thumb-matrix-focused\".to_string()",
        "id_base=\"docs-color-thumb-matrix-dragging\".to_string()",
        "id_base=\"docs-color-thumb-matrix-disabled\".to_string()",
        "id_base=\"docs-color-thumb-matrix-custom\".to_string()",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "data-slot=\"color-thumb-controlled-vs-uncontrolled\"",
        "id_base=\"docs-color-thumb-controlled-like\".to_string()",
        "id_base=\"docs-color-thumb-uncontrolled-like\".to_string()",
        "is_disabled=true",
        "is_focused=true",
        "is_dragging=true",
        "is_loupe_visible=false",
        "x_percent=22.0",
        "y_percent=72.0",
        "x_percent=52.0",
        "y_percent=44.0",
        "x_percent=82.0",
        "y_percent=28.0",
        "class_name=\"docs-color-thumb-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "ColorThumb docs should keep API/default/state-matrix marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] color: Option<String>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_focused: bool",
        "#[prop(optional)] is_dragging: bool",
        "#[prop(optional)] x_percent: Option<f32>",
        "#[prop(optional)] y_percent: Option<f32>",
        "#[prop(optional)] is_loupe_visible: Option<bool>",
        "logic::resolve_component_state(logic::ColorThumbLogicInput {",
        "interaction_state: logic::interaction_state_from_flags(",
        "is_loupe_visible,",
        "x_percent,",
        "y_percent,",
    ] {
        assert!(
            view_source.contains(required),
            "ColorThumb view API contract should keep `{required}`.",
        );
    }

    for required in [
        "pub fn normalize_position_percent(value: Option<f32>) -> f32",
        "None => DEFAULT_POSITION_PERCENT,",
        "pub fn interaction_state_from_flags(",
        "pub fn resolve_component_state(input: ColorThumbLogicInput) -> ColorThumbState",
        "show_loupe: input.is_loupe_visible.unwrap_or(true),",
        "x_percent: normalize_position_percent(input.x_percent),",
        "y_percent: normalize_position_percent(input.y_percent),",
    ] {
        assert!(
            logic_source.contains(required),
            "ColorThumb logic default/normalization contract should keep `{required}`.",
        );
    }
}

#[test]
fn color_thumb_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");
    for needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_thumb_check2_documents_documentation_as_product_rules() {
    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
            "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
            "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
            "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
            "color_thumb_check2_documents_documentation_as_product_rules",
            "color_thumb_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
            "color_thumb_docs_are_beginner_friendly_with_default_then_advanced_path",
            "color_thumb_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
            "`scripts/check-ui-contract-hygiene.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep documentation-as-product rule `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/README.md");
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    assert!(
        readme_path.exists(),
        "color-thumb should provide README as documentation entry.",
    );
    assert!(
        docs_source.contains("pub(super) fn color_thumb() -> AnyView"),
        "docs-app should expose color_thumb docs entry function.",
    );
}

#[test]
fn color_thumb_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = include_str!("../src/README.md");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");
    let section_start = docs_source
        .find("pub(super) fn color_thumb() -> AnyView {")
        .unwrap_or_else(|| panic!("forms_color docs should contain color_thumb section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn color_editor() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("forms_color docs should contain color_editor section after color_thumb")
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "title=\"ColorThumb\"",
        "slug=\"color-thumb\"",
        "Playground title=\"Hello World\" code_signal=hello_code",
        "Playground title=\"Focused + Dragging + Position\" code_signal=basic_code",
        "Playground title=\"Disabled + Custom Class + Loupe Off\" code_signal=states_code",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
    ] {
        assert!(
            section.contains(required),
            "color-thumb docs should keep beginner-to-advanced marker `{required}`.",
        );
    }

    let hello_pos = section
        .find("Playground title=\"Hello World\" code_signal=hello_code")
        .expect("docs should include hello-world playground.");
    let basic_pos = section
        .find("Playground title=\"Focused + Dragging + Position\" code_signal=basic_code")
        .expect("docs should include common focused/dragging path.");
    let states_pos = section
        .find("Playground title=\"Disabled + Custom Class + Loupe Off\" code_signal=states_code")
        .expect("docs should include common disabled/custom path.");
    let matrix_pos = section
        .find("title=\"State Matrix\"")
        .expect("docs should include state-matrix playground.");
    let controlled_pos = section
        .find("title=\"Controlled vs Uncontrolled (N/A)\"")
        .expect("docs should include controlled-vs-uncontrolled N/A explanation.");
    assert!(
        hello_pos < basic_pos
            && basic_pos < states_pos
            && states_pos < matrix_pos
            && matrix_pos < controlled_pos,
        "docs should present default usage before advanced explanations.",
    );

    for required in [
        "## Hello World",
        "## 常见用法",
        "## 进阶参数",
        "阅读顺序建议：先跑 `Hello World`，再看常见状态组合，最后按需启用进阶参数。",
        "默认路径不需要手动接线 `ui-state-primitives` 或 `ui-headless` 状态机。",
    ] {
        assert!(
            readme_source.contains(required),
            "README should keep beginner-friendly guidance marker `{required}`.",
        );
    }

    let readme_hello_pos = readme_source
        .find("## Hello World")
        .expect("README should include hello-world section.");
    let readme_common_pos = readme_source
        .find("## 常见用法")
        .expect("README should include common usage section.");
    let readme_advanced_pos = readme_source
        .find("## 进阶参数")
        .expect("README should include advanced section.");
    assert!(
        readme_hello_pos < readme_common_pos && readme_common_pos < readme_advanced_pos,
        "README should present default path before advanced guidance.",
    );

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_are_beginner_friendly_with_default_then_advanced_path";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = include_str!("../src/README.md");

    for required in [
        "Playground title=\"Hello World\" code_signal=hello_code",
        "<ColorThumb id_base=\"docs-color-thumb-hello\".to_string() />",
        "## Hello World",
        "<ColorThumb id_base=\"demo-color-thumb\".to_string() />",
    ] {
        assert!(
            docs_source.contains(required) || readme_source.contains(required),
            "color-thumb hello-world path should keep zero-threshold marker `{required}`.",
        );
    }

    for forbidden in [
        "ui_state_primitives",
        "use_color_thumb(",
        "state=...",
        "logic::",
    ] {
        assert!(
            !readme_source.contains(forbidden),
            "color-thumb README hello-world path should avoid architecture-wiring token `{forbidden}`.",
        );
    }
}

#[test]
fn color_thumb_check2_documents_interactive_playground_rules() {
    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
            "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
            "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
            "Playground 作为验收面，需可重复复现关键交互路径。",
            "color_thumb_check2_documents_interactive_playground_rules",
            "color_thumb_docs_app_provides_interactive_playground_for_props_state_and_preview",
            "color_thumb_interactive_playground_reuses_repeatable_semantic_e2e_flow",
            "color_thumb_dx_check_script_covers_interactive_playground_contract",
            "color_thumb_e2e_check_script_covers_interactive_playground_contract",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep interactive-playground rule `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let section_start = docs_source
        .find("pub(super) fn color_thumb() -> AnyView {")
        .unwrap_or_else(|| panic!("forms_color docs should contain color_thumb section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn color_editor() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("forms_color docs should contain color_editor section after color_thumb")
        });
    let section = &section_tail[..section_end_rel];

    for marker in [
        "title=\"Interactive Workbench (DX)\"",
        "data-slot=\"color-thumb-workbench-controls\"",
        "data-slot=\"color-thumb-workbench-input-color\"",
        "data-slot=\"color-thumb-workbench-input-x\"",
        "data-slot=\"color-thumb-workbench-input-y\"",
        "data-slot=\"color-thumb-workbench-replay-controls\"",
        "data-slot=\"color-thumb-workbench-replay-idle\"",
        "data-slot=\"color-thumb-workbench-replay-drag\"",
        "data-slot=\"color-thumb-workbench-spec-input\"",
        "data-slot=\"color-thumb-workbench\"",
        "data-slot=\"color-thumb-workbench-canvas\"",
        "data-slot=\"color-thumb-workbench-spec-preview\"",
        "data-slot=\"color-thumb-workbench-spec-state\"",
        "data-slot=\"color-thumb-workbench-state\"",
        "Switch checked=workbench_disabled",
        "Switch checked=workbench_focused",
        "Switch checked=workbench_dragging",
        "Switch checked=workbench_loupe_visible",
        "Switch checked=workbench_custom_class",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "workbench_code = Signal::derive(move || {",
        "Spec input -> preview",
    ] {
        assert!(
            section.contains(marker),
            "color-thumb docs interactive playground should keep marker `{marker}`.",
        );
    }
}

#[test]
fn color_thumb_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_color_thumb_contract.spec.mjs");

    for marker in [
        "docs-app color-thumb interactive playground updates props/state and links spec input to preview",
        "data-slot=\"color-thumb-workbench-controls\"",
        "data-slot=\"color-thumb-workbench-input-x\"",
        "data-slot=\"color-thumb-workbench-input-y\"",
        "getByRole(\"checkbox\", { name: \"Disabled\" })",
        "data-slot=\"color-thumb-workbench-replay-drag\"",
        "data-slot=\"color-thumb-workbench-spec-input\"",
        "data-slot=\"color-thumb-workbench-spec-state\"",
        "toContainText(\"spec: ok\")",
        "docs-app color-thumb key flow is repeatable and failures map to semantic breakpoints",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "color-thumb interactive playground e2e flow should keep marker `{marker}`.",
        );
    }
}

#[test]
fn color_thumb_dx_check_script_covers_interactive_playground_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_interactive_playground_rules",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_app_provides_interactive_playground_for_props_state_and_preview",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce interactive-playground contract `{needle}`.",
        );
    }
}

#[test]
fn color_thumb_e2e_check_script_covers_interactive_playground_contract() {
    let script_source =
        include_str!("../../../components/color-thumb/scripts/check-ui-e2e-color-thumb.sh");

    for needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_e2e_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "e2e check script should enforce interactive-playground contract `{needle}`.",
        );
    }
}

#[test]
fn color_thumb_check2_documents_source_first_copy_paste_ready_rules() {
    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
            "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
            "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
            "文档代码与当前实现必须同步，防止示例漂移。",
            "color_thumb_check2_documents_source_first_copy_paste_ready_rules",
            "color_thumb_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
            "color_thumb_dx_check_script_covers_source_first_copy_paste_ready_contract",
            "color_thumb_check2_marks_source_first_copy_paste_ready_contract_complete",
            "`scripts/check-ui-dx.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep source-first copy-paste-ready rule `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = include_str!("../src/README.md");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");

    let section_start = docs_source
        .find("pub(super) fn color_thumb() -> AnyView {")
        .unwrap_or_else(|| panic!("forms_color docs should contain color_thumb section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn color_editor() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("forms_color docs should contain color_editor section after color_thumb")
        });
    let section = &section_tail[..section_end_rel];

    for needle in [
        "data-slot=\"color-thumb-source-first-contract\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "<code>\"code_imports\"</code>",
        "data-slot=\"color-thumb-source-first-copy-action\"",
        "<code>\"Show code + Copy\"</code>",
        "data-slot=\"color-thumb-source-paths\"",
        "<code>\"components/color-thumb/src/mod.rs\"</code>",
        "<code>\"components/color-thumb/src/view.rs\"</code>",
        "<code>\"components/color-thumb/src/logic.rs\"</code>",
        "<code>\"components/color-thumb/src/styles.rs\"</code>",
        "<code>\"components/color-thumb/src/motion.rs\"</code>",
        "data-slot=\"color-thumb-source-prerequisites\"",
        "<code>\"component-color_thumb\"</code>",
        "<code>\"inject-css\"</code>",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=color_thumb_imports.clone()",
    ] {
        assert!(
            section.contains(needle),
            "color-thumb source-first docs should contain `{needle}`.",
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
            "playground copy-ready pipeline should keep `{needle}`.",
        );
    }

    for needle in [
        "## Source-first / Copy-Paste Ready",
        "Show code + Copy",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "components/color-thumb/src/{mod,logic,view,styles,motion}.rs",
        "component-color_thumb",
        "inject-css",
    ] {
        assert!(
            readme_source.contains(needle),
            "ColorThumb README should document source-first dependency/path marker `{needle}`.",
        );
    }
}

#[test]
fn color_thumb_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce source-first copy-paste-ready contract `{needle}`.",
        );
    }
}

#[test]
fn color_thumb_check2_marks_source_first_copy_paste_ready_contract_complete() {
    for source in [load_source("check2"), load_source("check2_src")] {
        for marker in [
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
            "color_thumb_check2_documents_source_first_copy_paste_ready_rules",
            "color_thumb_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
            "color_thumb_dx_check_script_covers_source_first_copy_paste_ready_contract",
            "`scripts/check-ui-dx.sh`",
        ] {
            assert!(
                source.contains(marker),
                "color-thumb checklist should keep source-first copy-ready evidence marker `{marker}`.",
            );
        }
    }
}

#[test]
fn color_thumb_check2_documents_heroui_benchmark_docs_sync_rules() {
    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
            "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
            "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
            "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep heroui-benchmark docs-sync rule `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = include_str!("../src/README.md");

    for needle in [
        "### ColorThumb 同步记录（2026-02-20）",
        "参数模型同步：`ColorThumb` 参数主轴保持",
        "component_doc!(\"ColorThumb\", \"color-thumb\", \"Forms\", forms_color::color_thumb)",
        "#/components/color-thumb",
        "`components/color-thumb/src/README.md` 提供等价文档入口",
        "forms_color.rs::color_thumb()",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include color-thumb synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"ColorThumb\"",
        "\"color-thumb\"",
        "forms_color::color_thumb",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose color-thumb entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn color_thumb() -> AnyView {",
        "title=\"ColorThumb\"",
        "slug=\"color-thumb\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app color-thumb page should stay indexable via marker `{needle}`.",
        );
    }

    assert!(
        readme_source.contains("# ColorThumb"),
        "color-thumb README should remain an equivalent component doc entry.",
    );
}

#[test]
fn color_thumb_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn color_thumb_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    for source in [load_source("check2"), load_source("check2_src")] {
        for marker in [
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
            "color_thumb_check2_documents_heroui_benchmark_docs_sync_rules",
            "color_thumb_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
            "color_thumb_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
            "docs/spec/heroui-parameter-design-strategy.md",
            "`scripts/check-ui-dx.sh`",
        ] {
            assert!(
                source.contains(marker),
                "color-thumb checklist should keep heroui-benchmark docs-sync evidence marker `{marker}`.",
            );
        }
    }
}

#[test]
fn color_thumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let dx_script_source = include_str!("../../../scripts/check-ui-dx.sh");

    let section_start = docs_source
        .find("pub(super) fn color_thumb() -> AnyView {")
        .unwrap_or_else(|| panic!("forms_color docs should contain color_thumb section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn color_editor() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("forms_color docs should contain color_editor section after color_thumb")
        });
    let section = &section_tail[..section_end_rel];

    for needle in [
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "id_base=\"docs-color-thumb-matrix-idle\".to_string()",
        "id_base=\"docs-color-thumb-matrix-focused\".to_string()",
        "id_base=\"docs-color-thumb-matrix-dragging\".to_string()",
        "id_base=\"docs-color-thumb-matrix-disabled\".to_string()",
        "id_base=\"docs-color-thumb-matrix-custom\".to_string()",
        "id_base=\"docs-color-thumb-controlled-like\".to_string()",
        "id_base=\"docs-color-thumb-uncontrolled-like\".to_string()",
        "id_base=\"docs-color-thumb-snapshot\".to_string()",
        "id_base=\"docs-color-thumb-source-first\".to_string()",
        "data-slot=\"color-thumb-output-mode\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "code_imports=color_thumb_imports.clone()",
        "data-slot=\"color-thumb-source-first-contract\"",
        "data-slot=\"color-thumb-source-paths\"",
        "component-color_thumb",
        "inject-css",
    ] {
        assert!(
            section.contains(needle),
            "ColorThumb docs product contract should include `{needle}`.",
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
            "Playground copy-ready pipeline should include `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        dx_script_source.contains(script_needle),
        "DX gate script should include docs-as-product contract `{script_needle}`.",
    );

    for source in [load_source("check2"), load_source("check2_src")] {
        for needle in [
            "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
            "Controlled vs Uncontrolled (N/A)",
            "color_thumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
            "`scripts/check-ui-dx.sh`",
            "compose_copy_ready_code",
        ] {
            assert!(
                source.contains(needle),
                "ColorThumb check2 docs-as-product evidence should include `{needle}`.",
            );
        }
    }
}

#[test]
fn color_thumb_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        script_source.contains(needle),
        "DX check script should enforce docs product copy-paste-ready marker `{needle}`.",
    );
}

#[test]
fn color_thumb_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("styles");
    let theme_css_source = include_str!("../../../crates/ui-theme/src/css.rs");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "var(--ui-color-thumb-x-start, var(--ui-fallback-color-thumb-x-start))",
        "var(--ui-color-thumb-x-center, var(--ui-fallback-color-thumb-x-center))",
        "var(--ui-color-thumb-x-end, var(--ui-fallback-color-thumb-x-end))",
        "var(--ui-color-thumb-y-start, var(--ui-fallback-color-thumb-y-start))",
        "var(--ui-color-thumb-y-center, var(--ui-fallback-color-thumb-y-center))",
        "var(--ui-color-thumb-y-end, var(--ui-fallback-color-thumb-y-end))",
        "var(--ui-color-thumb-handle-size, var(--ui-fallback-color-thumb-handle-size))",
        "var(--ui-color-thumb-loupe-size, var(--ui-fallback-color-thumb-loupe-size))",
        "var(--ui-color-thumb-radius-full, var(--ui-fallback-color-thumb-radius-full))",
        "--ui-color-thumb-handle-border-width,",
        "var(--ui-fallback-color-thumb-handle-border-width)",
        "var(--ui-color-thumb-loupe-border-width, var(--ui-fallback-color-thumb-loupe-border-width))",
        "var(--ui-color-thumb-loupe-padding, var(--ui-fallback-color-thumb-loupe-padding))",
        "--ui-color-thumb-loupe-hidden-offset,",
        "var(--ui-fallback-color-thumb-loupe-hidden-offset)",
        "var(--ui-color-thumb-loupe-hidden-scale, var(--ui-fallback-color-thumb-loupe-hidden-scale))",
        "var(--ui-color-thumb-disabled-opacity, var(--ui-fallback-color-thumb-disabled-opacity))",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep defensive variable fallback chain `{required}`."
        );
    }

    for forbidden in [
        "left: 16%;",
        "left: 84%;",
        "top: 16%;",
        "top: 84%;",
        "inline-size: 1.125rem;",
        "inline-size: 1.875rem;",
        "border-radius: var(--ui-radius-full, 999px);",
        "padding: 2px;",
        "border: 2px solid",
        "border: 1px solid",
        "translateY(0.2rem)",
        "opacity: 0.58;",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid hardcoded size terminal token `{forbidden}`."
        );
    }

    for required in [
        "--ui-fallback-color-thumb-x-start:",
        "--ui-fallback-color-thumb-x-center:",
        "--ui-fallback-color-thumb-x-end:",
        "--ui-fallback-color-thumb-y-start:",
        "--ui-fallback-color-thumb-y-center:",
        "--ui-fallback-color-thumb-y-end:",
        "--ui-fallback-color-thumb-handle-size:",
        "--ui-fallback-color-thumb-loupe-size:",
        "--ui-fallback-color-thumb-radius-full:",
        "--ui-fallback-color-thumb-handle-border-width:",
        "--ui-fallback-color-thumb-loupe-border-width:",
        "--ui-fallback-color-thumb-loupe-padding:",
        "--ui-fallback-color-thumb-loupe-hidden-offset:",
        "--ui-fallback-color-thumb-loupe-hidden-scale:",
        "--ui-fallback-color-thumb-disabled-opacity:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css should remain SSOT for color-thumb fallback terminal `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should enforce ColorThumb defensive-variable marker `{script_needle}`.",
    );

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] 样式孤岛防御（Defensive Variables）",
            "color_thumb_styles_use_defensive_variable_fallback_chain",
            "`scripts/check-ui-contract-hygiene.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep defensive-variable governance marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_contract_hygiene_script_covers_defensive_variable_chain() {
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(
            "echo \"[contract-hygiene] contract: color-thumb styles keep defensive fallback chain with ui-theme SSOT terminals\""
        ) && script_source.contains(needle),
        "contract-hygiene script should enforce ColorThumb defensive-variable marker `{needle}`.",
    );
}

#[test]
fn color_thumb_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_source = include_str!("../../../crates/ui/src/css.rs");
    let root_source = include_str!("../../../crates/ui/src/root.rs");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_thumb\")]",
        "out.push_str(crate::color::thumb::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui css aggregation should keep cascade-layer marker `{required}`.",
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep components css injection path marker `{required}`.",
        );
    }

    assert!(
        view_source.contains("style=move || style_vars.get_value()"),
        "runtime styling should stay css-variable-only via `style_vars` contract.",
    );
    for required in [
        "--ui-color-thumb-handle-duration",
        "--ui-color-thumb-loupe-duration",
    ] {
        assert!(
            motion_source.contains(required),
            "motion mapping should expose css-variable-only runtime marker `{required}`.",
        );
    }

    let combined = format!("{view_source}\n{motion_source}");
    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
        "style=\"background:",
        "style=\"transform:",
        "style=\"opacity:",
        " top:",
        " left:",
        " width:",
        " height:",
        " background:",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorThumb runtime style should not leak non-custom-property inline token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should enforce ColorThumb cascade-layer marker `{script_needle}`.",
    );

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] 级联层覆盖（`@layer ui`）",
            "color_thumb_cascade_layer_and_runtime_style_contract_is_enforced",
            "`scripts/check-ui-contract-hygiene.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep cascade-layer governance marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_contract_hygiene_script_covers_cascade_layer_and_runtime_style_contract() {
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(
            "echo \"[contract-hygiene] contract: color-thumb css is aggregated in @layer ui and runtime style is css-variable-only\""
        ) && script_source.contains(needle),
        "contract-hygiene script should enforce ColorThumb cascade-layer marker `{needle}`.",
    );
}

#[test]
fn color_thumb_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop() {
    let motion_source = load_source("motion");
    let view_source = load_source("view");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "pub spring: ui_motion::spring::SpringConfig",
        "ui_motion::presets::spring_soft()",
        "ui_motion::spring::sanitize_config(",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-color-thumb-motion-stiffness",
        "--ui-color-thumb-motion-damping",
        "--ui-color-thumb-motion-mass",
        "--ui-color-thumb-motion-precision",
    ] {
        assert!(
            motion_source.contains(required),
            "ColorThumb motion contract should include `{required}`.",
        );
    }

    assert!(
        view_source.contains("motion::attach_motion(None, motion)"),
        "ColorThumb view should attach motion contract via style vars.",
    );

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should enforce ColorThumb motion-contract marker `{script_needle}`.",
    );

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] Motion 合同化",
            "color_thumb_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
            "`scripts/check-ui-contract-hygiene.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep motion-contract governance marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_contract_hygiene_script_covers_motion_contract() {
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        script_source
            .contains("echo \"[contract-hygiene] contract: color-thumb motion contract is built-in and safely attached across reduced-motion + non-wasm\"")
            && script_source.contains(needle),
        "contract-hygiene script should enforce ColorThumb motion-contract marker `{needle}`.",
    );
}

#[test]
fn color_thumb_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let component_manifest = include_str!("../src/Component.toml");
    let component_rbi = include_str!("../src/color_thumb.rbi");

    for typed_source in [
        "pub enum ColorThumbAgentSchema",
        "pub enum ColorThumbAgentSchemaVersion",
        "pub enum ColorThumbStreamSupport",
        "pub enum ColorThumbStreamFallback",
        "pub enum ColorThumbOutputStatus",
        "pub enum ColorThumbIntent",
        "pub enum ColorThumbUiAction",
        "pub struct ColorThumbAgentContract",
        "pub fn resolve_ui_action(state: ColorThumbState) -> ColorThumbUiAction",
        "pub fn resolve_agent_contract() -> ColorThumbAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "ColorThumb Agent Contract should stay type-derived via `{typed_source}`.",
        );
    }

    for marker in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get()",
        "data-ui-state=move || semantics.get().root_attrs.data_state",
        "data-ui-source=move || semantics.get().root_attrs.data_interaction_source",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "ColorThumb view should mount Agent Contract marker `{marker}`.",
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.color-thumb.agent-contract.v1\"",
        "intent = \"pick-color-point\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-stream-support\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-output-status\"",
        "ColorThumbAgentContract",
        "resolve_agent_contract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "ColorThumb context-compression assets should keep Agent Contract marker `{required}`.",
        );
    }

    for forbidden in [
        "format!(\"data-ui-schema",
        "format!(\"data-ui-intent",
        "format!(\"data-ui-state",
        "format!(\"data-ui-source",
        "format!(\"data-ui-stream-support",
        "format!(\"data-ui-stream-fallback",
        "format!(\"data-ui-output-status",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "ColorThumb Agent Contract should avoid free-form schema token `{forbidden}`.",
        );
    }

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
            "color_thumb_agent_contract_is_schema_typed_and_machine_readable",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep Agent Contract evidence `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let component_manifest = include_str!("../src/Component.toml");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [",
        "\"logic::resolve_component_state(...)\"",
        "\"logic::resolve_agent_contract()\"",
        "\"logic::resolve_ui_action(...)\"",
        "\"use_color_thumb(...)\"",
        "\"motion::sanitize_motion(...)\"",
        "\"motion::attach_motion(...)\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "ColorThumb manifest should keep whitelist-safe render path marker `{required}`.",
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "ColorThumb Agent Contract render path should forbid `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene gate script should include `{script_needle}`.",
        );
    }

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "color_thumb_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
            "白名单能力边界",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep Agent Contract whitelist evidence `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
            "`Streaming`：LLM 还在生成，界面边生成边显示。",
            "`Snapshot`：LLM 全部生成完成后，一次性显示。",
            "N/A：`ColorThumb` 不是 LLM 正文渲染组件",
            "token-by-token streaming 协议",
            "color_thumb_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
            "`scripts/check-ui-streaming.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep LLM-only streaming definition marker `{required}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let docs_source =
        include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
            "所有组件都应能消费“完整生成结果”并稳定渲染。",
            "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
            "color_thumb_snapshot_baseline_consumes_complete_result_and_renders_stably",
            "`scripts/check-ui-streaming.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep snapshot-baseline marker `{required}`.",
            );
        }
    }

    for marker in [
        "pub fn ColorThumb(",
        "let color = logic::sanitize_color(color);",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_aria_value_text(aria_value_text, color.clone())",
        "let class_name = logic::normalize_optional_text(class_name);",
        "logic::normalize_optional_text(lang)",
        "logic::resolve_component_state(logic::ColorThumbLogicInput {",
        "use_color_thumb(ColorThumbOptions {",
        "data-state=move || semantics.get().root_attrs.data_state",
        "data-disabled=move || semantics.get().root_attrs.data_disabled",
        "data-focused=move || semantics.get().root_attrs.data_focused",
        "data-dragging=move || semantics.get().root_attrs.data_dragging",
        "data-loupe-visible=move || semantics.get().root_attrs.data_loupe_visible",
        "data-has-color=move || semantics.get().root_attrs.data_has_color",
        "data-x=move || semantics.get().root_attrs.data_x",
        "data-y=move || semantics.get().root_attrs.data_y",
        "data-interaction-source=move || semantics.get().root_attrs.data_interaction_source",
        "data-aria-source=move || semantics.get().root_attrs.data_aria_source",
        "data-aria-valuetext-source=move || semantics.get().root_attrs.data_aria_valuetext_source",
        "data-class-source=move || semantics.get().root_attrs.data_class_source",
        "data-loupe-source=move || semantics.get().root_attrs.data_loupe_source",
        "data-x-source=move || semantics.get().root_attrs.data_x_source",
        "data-y-source=move || semantics.get().root_attrs.data_y_source",
        "aria-label=move || semantics.get().root_attrs.aria_label",
        "aria-disabled=move || semantics.get().root_attrs.aria_disabled",
        "aria-valuetext=move || semantics.get().root_attrs.aria_valuetext",
    ] {
        assert!(
            view_source.contains(marker),
            "ColorThumb view should keep snapshot baseline render marker `{marker}`.",
        );
    }

    for marker in [
        "pub fn resolve_component_state(input: ColorThumbLogicInput) -> ColorThumbState",
        "pub fn normalize_position_percent(value: Option<f32>) -> f32",
        "sanitize_percent(value)",
        "source_from_option(input.is_loupe_visible)",
        "source_from_option(input.x_percent)",
        "source_from_option(input.y_percent)",
    ] {
        assert!(
            logic_source.contains(marker),
            "ColorThumb logic should keep snapshot normalization marker `{marker}`.",
        );
    }

    for marker in [
        "title=\"ColorThumb\"",
        "slug=\"color-thumb\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Focused + Dragging + Position\" code_signal=basic_code>",
        "<Playground title=\"Disabled + Custom Class + Loupe Off\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(marker),
            "docs color page should keep snapshot-ready baseline usage marker `{marker}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_thumb_check2_documents_streaming_required_optional_classification_rules() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
            "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
            "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
            "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
            "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
            "`ColorThumb` 归类为 `Streaming Optional`",
            "data-ui-stream-support=\\\"optional\\\"",
            "data-ui-stream-fallback=\\\"snapshot\\\"",
            "data-ui-output-status=\\\"verified\\\"",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep streaming-classification marker `{required}`.",
            );
        }
    }

    for script_needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`.",
        );
    }
}

#[test]
fn color_thumb_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("view");

    for required in [
        "role=move || semantics.get().root_attrs.role",
        "aria-label=move || semantics.get().root_attrs.aria_label",
        "aria-disabled=move || semantics.get().root_attrs.aria_disabled",
        "aria-valuetext=move || semantics.get().root_attrs.aria_valuetext",
        "data-state=move || semantics.get().root_attrs.data_state",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get()",
        "data-ui-source=move || semantics.get().root_attrs.data_interaction_source",
        "data-ui-state=move || semantics.get().root_attrs.data_state",
    ] {
        assert!(
            view_source.contains(required),
            "ColorThumb should keep continuous aria/data semantics via `{required}` in optional-streaming scope.",
        );
    }
}

#[test]
fn color_thumb_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let mod_source = load_source("mod");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let motion_source = load_source("motion");
    let styles_source = load_source("styles");
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");
    let combined =
        format!("{mod_source}\n{view_source}\n{logic_source}\n{motion_source}\n{styles_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "network_error",
        "transport_error",
        "abort_controller",
        "exponential_backoff",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorThumb should keep validation/retry/resilience orchestration out of component layer; found `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming gate script should include `{script_needle}`.",
        );
    }
}

#[test]
fn color_thumb_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2_source = load_source("check2");
    let check2_src_source = load_source("check2_src");
    let script_source = include_str!("../../../scripts/check-ui-entrypoints.sh");
    let lib_source = include_str!("../../../crates/ui/src/lib.rs");
    let css_source = include_str!("../../../crates/ui/src/css.rs");
    let root_source = include_str!("../../../crates/ui/src/root.rs");
    let active_highlight_source =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_a11y_source = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let headless_presence_source = include_str!("../../../crates/ui-headless/src/presence.rs");
    let headless_controllable_state_source =
        include_str!("../../../crates/ui-headless/src/controllable_state.rs");

    for required in [
        "#[cfg(feature = \"component-color_thumb\")]",
        "pub mod color_thumb;",
        "pub use crate::color_thumb as thumb;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib entry should keep feature-gated color-thumb public surface `{required}`.",
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub use wasm_bindgen",
        "pub use leptos::web_sys",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui lib entry should not expose platform detail `{forbidden}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_thumb\")]",
        "out.push_str(crate::color::thumb::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui css entry should keep feature-gated layered aggregation marker `{required}`.",
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot entry should keep centralized theme/i18n/css injection marker `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight should stay shared motion primitive marker `{required}`.",
        );
    }

    for forbidden in ["ui-color-thumb", "ui-button", "ui-checkbox", "data-slot="] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should not carry component business semantics `{forbidden}`.",
        );
    }

    let required = "pub fn aria_controls_when_open(";
    assert!(
        headless_a11y_source.contains(required),
        "headless canonical a11y path should keep `{required}`.",
    );
    for required in ["pub struct Presence {", "pub fn use_presence("] {
        assert!(
            headless_presence_source.contains(required),
            "headless canonical presence path should keep `{required}`.",
        );
    }
    for required in [
        "pub fn use_controllable_state<T>(",
        "pub struct ControllableState<T>",
    ] {
        assert!(
            headless_controllable_state_source.contains(required),
            "headless canonical controllable-state path should keep `{required}`.",
        );
    }

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    for forbidden in [
        workspace_dir.join("crates/ui/src/overlay_open.rs"),
        workspace_dir.join("crates/ui/src/presence.rs"),
        workspace_dir.join("crates/ui/src/a11y.rs"),
    ] {
        assert!(
            !forbidden.exists(),
            "ui forbidden fixed entrypoint file should stay absent: {forbidden:?}",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(script_needle),
        "entrypoint gate script should include `{script_needle}`.",
    );

    for source in [check2_source, check2_src_source] {
        for required in [
            "- [x] `ui` 固定入口文件落点正确。",
            "color_thumb_ui_components_fixed_entry_files_follow_layered_boundaries",
            "`scripts/check-ui-entrypoints.sh`",
        ] {
            assert!(
                source.contains(required),
                "checklist should keep fixed-entrypoint governance marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_entrypoints_script_covers_fixed_entry_files_contract() {
    let script_source = include_str!("../../../scripts/check-ui-entrypoints.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(
            "echo \"[entrypoints] contract: color-thumb fixed entry files and forbidden file guards\""
        ) && script_source.contains(needle),
        "entrypoint script should enforce ColorThumb fixed-entrypoint marker `{needle}`.",
    );
}

#[test]
fn color_thumb_component_directory_standard_files_follow_contract_and_na_paths() {
    let check2_source = load_source("check2");
    let check2_src_source = load_source("check2_src");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    for required in [
        workspace_dir.join("components/color-thumb/src/mod.rs"),
        workspace_dir.join("components/color-thumb/src/logic.rs"),
        workspace_dir.join("components/color-thumb/src/styles.rs"),
        workspace_dir.join("components/color-thumb/src/view.rs"),
        workspace_dir.join("components/color-thumb/src/motion.rs"),
    ] {
        assert!(
            required.exists(),
            "color-thumb should keep required component file: {required:?}",
        );
    }

    for forbidden in [
        workspace_dir.join("components/color-thumb/src/render.rs"),
        workspace_dir.join("components/color-thumb/src/spec.rs"),
    ] {
        assert!(
            !forbidden.exists(),
            "color-thumb should keep forbidden/simple-N/A component file absent: {forbidden:?}",
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ColorThumbState, ColorThumbStateInput, DEFAULT_ARIA_LABEL, DEFAULT_COLOR};",
        "pub use motion::ColorThumbMotion;",
        "pub use view::ColorThumb;",
    ] {
        assert!(
            mod_source.contains(required),
            "color-thumb mod.rs should keep minimal stable export marker `{required}`.",
        );
    }

    for forbidden in [
        "pub mod view;",
        "pub mod logic;",
        "pub mod spec;",
        "mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-thumb mod.rs should avoid over-export marker `{forbidden}`.",
        );
    }

    for forbidden in ["mod spec;", "pub mod spec;", "spec::"] {
        assert!(
            !lib_source.contains(forbidden),
            "color-thumb lib.rs should keep spec.rs out of public surface marker `{forbidden}`.",
        );
    }

    for required in [
        "pub use ui_state_primitives::color_thumb::{",
        "pub fn source_from_option<T>(",
        "pub fn resolve_component_state(",
        "pub fn interaction_state_from_flags(",
    ] {
        assert!(
            logic_source.contains(required),
            "color-thumb logic.rs should keep normalization/derivation marker `{required}`.",
        );
    }

    for forbidden in [
        "use leptos",
        "web_sys::",
        "view! {",
        "data-slot=",
        "pub const CSS",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "color-thumb logic.rs should stay non-view/non-style marker `{forbidden}`.",
        );
    }

    for required in ["pub const CSS: &str", "var(--ui-", ".ui-color-thumb"] {
        assert!(
            styles_source.contains(required),
            "color-thumb styles.rs should keep static token-first marker `{required}`.",
        );
    }

    for forbidden in ["use leptos", "#[component]", "on:click=", "web_sys::"] {
        assert!(
            !styles_source.contains(forbidden),
            "color-thumb styles.rs should avoid runtime/view marker `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "pub fn ColorThumb(",
        "use ui_headless::{A11yDirection, ColorThumbOptions, use_color_thumb};",
        "view! {",
        "data-slot=SLOT_COLOR_THUMB",
        "let motion = motion::sanitize_motion(motion);",
        "let style_vars = StoredValue::new(motion::attach_motion(None, motion));",
    ] {
        assert!(
            view_source.contains(required),
            "color-thumb view.rs should keep render + headless mount marker `{required}`.",
        );
    }

    for forbidden in ["render.rs", "include_str!(\"./render.rs\")"] {
        assert!(
            !view_source.contains(forbidden),
            "color-thumb view.rs should not drift to render.rs marker `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ColorThumbMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "color-thumb motion.rs should keep semantic->motion mapping marker `{required}`.",
        );
    }

    for forbidden in [
        "data-slot=",
        "role=",
        "on:click=",
        "pub const CSS",
        "use ui_headless",
        "SpringAnimator",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "color-thumb motion.rs should avoid view/headless/style marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for source in [check2_source, check2_src_source] {
        for required in [
            "- [x] 组件目录标准文件落点正确。",
            "color_thumb_component_directory_standard_files_follow_contract_and_na_paths",
            "`scripts/check-ui-component-files.sh`",
        ] {
            assert!(
                source.contains(required),
                "checklist should keep component-file governance marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_component_files_script_covers_standard_layout_contract() {
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(
            "echo \"[component-files] contract: color-thumb standard file layout + scoped responsibilities\""
        ) && script_source.contains(needle),
        "component-files script should enforce ColorThumb standard-layout marker `{needle}`.",
    );
}

#[test]
fn color_thumb_file_placement_discipline_contract_is_explicit_for_interactive_component_scope() {
    let check2_source = load_source("check2");
    let check2_src_source = load_source("check2_src");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let component_src_dir = workspace_dir.join("components/color-thumb/src");

    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_src_dir.join(required_file).exists(),
            "file-placement discipline requires `{required_file}` in color-thumb source directory.",
        );
    }
    for forbidden_file in ["render.rs", "spec.rs"] {
        assert!(
            !component_src_dir.join(forbidden_file).exists(),
            "color-thumb should keep `{forbidden_file}` absent in current scope.",
        );
    }

    assert!(
        mod_source.contains("pub(crate) mod logic;")
            && mod_source.contains("pub mod motion;")
            && mod_source.contains("pub mod styles;")
            && mod_source.contains("mod view;"),
        "mod.rs should keep canonical module boundary for file-placement discipline.",
    );

    assert!(
        logic_source.contains("pub fn resolve_component_state(")
            && styles_source.contains("pub const CSS: &str =")
            && view_source.contains("#[component]")
            && motion_source.contains("pub struct ColorThumbMotion"),
        "logic/styles/view/motion should keep canonical responsibility anchors.",
    );

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_file_placement_discipline_contract_is_explicit_for_interactive_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for source in [check2_source, check2_src_source] {
        for required in [
            "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
            "color_thumb_file_placement_discipline_contract_is_explicit_for_interactive_component_scope",
            "`scripts/check-ui-component-files.sh`",
        ] {
            assert!(
                source.contains(required),
                "checklist should keep file-placement-discipline marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_component_files_script_covers_file_placement_discipline_contract() {
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_file_placement_discipline_contract_is_explicit_for_interactive_component_scope";
    assert!(
        script_source.contains(
            "echo \"[component-files] contract: color-thumb file-placement discipline in AI struct-first section\""
        ) && script_source.contains(needle),
        "component-files script should enforce ColorThumb file-placement marker `{needle}`.",
    );
}

#[test]
fn color_thumb_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let protocol_source = load_source("protocol");
    let check2_source = load_source("check2");
    let check2_src_source = load_source("check2_src");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    for forbidden in ["mod spec", "pub mod spec", "spec.rs", "Spec::new()"] {
        assert!(
            !lib_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "color-thumb should not expose simple-component spec surface `{forbidden}`.",
        );
    }

    for candidate in [
        workspace_dir.join("components/color-thumb/src/spec.rs"),
        workspace_dir.join("crates/ui/src/color_thumb/spec.rs"),
        workspace_dir.join("crates/ui/src/color/thumb/spec.rs"),
    ] {
        assert!(
            !candidate.exists(),
            "color-thumb should not introduce `spec.rs` without complex schema need: {candidate:?}",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for source in [check2_source, check2_src_source] {
        for required in [
            "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
            "N/A（已论证）",
            "color_thumb_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
            "`scripts/check-ui-component-files.sh`",
        ] {
            assert!(
                source.contains(required),
                "color-thumb checklist should keep hyper-structure-builder marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_component_files_script_covers_hyper_structure_builder_na_contract() {
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(
            "echo \"[component-files] contract: color-thumb hyper-structure builder spec contract is explicitly N/A\""
        ) && script_source.contains(needle),
        "component-files script should enforce ColorThumb hyper-structure-builder marker `{needle}`.",
    );
}

#[test]
fn color_thumb_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2_source = load_source("check2");
    let check2_src_source = load_source("check2_src");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let component_manifest = include_str!("../src/Component.toml");
    let component_rbi = include_str!("../src/color_thumb.rbi");
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    for candidate in [
        workspace_dir.join("components/color-thumb/src/Component.toml"),
        workspace_dir.join("components/color-thumb/src/color_thumb.rbi"),
    ] {
        assert!(
            candidate.exists(),
            "color-thumb context-compression asset should exist: {candidate:?}",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"ColorThumb\"",
        "crate = \"ui-color-thumb\"",
        "name = \"id_base\"",
        "name = \"color\"",
        "name = \"is_disabled\"",
        "name = \"is_focused\"",
        "name = \"is_dragging\"",
        "name = \"x_percent\"",
        "name = \"y_percent\"",
        "name = \"is_loupe_visible\"",
        "name = \"motion\"",
        "name = \"aria_label\"",
        "name = \"aria_value_text\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-thumb Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub type ColorThumbState = ui_state_primitives::color_thumb::ColorThumbState;",
        "pub type ColorThumbStateInput = ui_state_primitives::color_thumb::ColorThumbStateInput;",
        "pub type ColorThumbInteractionState = ui_state_primitives::color_thumb::ColorThumbInteractionState;",
        "pub type ColorThumbInputSource = ui_state_primitives::color_thumb::ColorThumbInputSource;",
        "pub type ColorThumbAriaValueTextSource =",
        "pub type ColorThumbMotion = crate::ColorThumbMotion;",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub const DEFAULT_COLOR: &str;",
        "pub fn ColorThumb(",
        "id_base: String,",
        "color: Option<String>,",
        "is_disabled: bool,",
        "x_percent: Option<f32>,",
        "y_percent: Option<f32>,",
        "motion: ColorThumbMotion,",
        "dir: Option<ui_headless::A11yDirection>,",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "color_thumb.rbi should keep signature-projection marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for source in [check2_source, check2_src_source] {
        for required in [
            "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
            "color_thumb_context_compression_manifest_and_rbi_projection_are_present_and_current",
            "`scripts/check-ui-component-files.sh`",
        ] {
            assert!(
                source.contains(required),
                "color-thumb checklist should keep context-compression marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_component_files_script_covers_context_compression_manifest_and_rbi_contract() {
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(
            "echo \"[component-files] contract: color-thumb context-compression manifest + rbi projection\""
        ) && script_source.contains(needle),
        "component-files script should enforce ColorThumb context-compression marker `{needle}`.",
    );
}

#[test]
fn color_thumb_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let protocol_source = load_source("protocol");
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");
    let cargo_source = include_str!("../../../crates/ui/Cargo.toml");
    let trace_source = include_str!("../../../crates/ui-headless/src/trace.rs");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "pub enum ThumbComponentSchemaVersion",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct ThumbComponentSpec",
        "#[serde(default)]",
        "pub schema_version: ThumbComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(required),
            "ColorThumb protocol should keep structured serde contract marker `{required}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "pub enum UiTraceEventKind {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            cargo_source.contains(needle) || trace_source.contains(needle),
            "engineering baseline should keep unified tracing marker `{needle}`.",
        );
    }

    let combined = format!(
        "{lib_source}\n{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}"
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::color_thumb::",
        "const COLOR_THUMB_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorThumb should avoid component-local tracing drift token `{forbidden}`.",
        );
    }

    for forbidden in [
        "tokio",
        "tokio::",
        "async_std",
        "async_std::",
        "async-std",
        "smol::",
        "runtime::Handle",
        "spawn_blocking(",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorThumb engineering contract should not leak runtime marker `{forbidden}`.",
        );
    }

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
            "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
            "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
            "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
            "color_thumb_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
            "`scripts/check-ui-engineering.sh`",
        ] {
            assert!(
                source.contains(required),
                "ColorThumb checklist should keep engineering governance rule `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let protocol_source = load_source("protocol");
    let component_manifest = include_str!("../src/Component.toml");
    let script_source = include_str!("../../../scripts/check-ui-engineering.sh");

    for required in [
        "pub enum ThumbComponentSchemaVersion",
        "V1",
        "pub struct ThumbComponentSpec",
        "pub schema_version: ThumbComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(required),
            "color-thumb protocol should keep stable v1 marker `{required}` in non-breaking scope.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.color-thumb.agent-contract.v1\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "Component.toml should keep v1 schema marker `{required}` in current scope.",
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecation_window",
        "codemod",
    ] {
        assert!(
            !protocol_source.contains(forbidden) && !component_manifest.contains(forbidden),
            "without major breaking upgrade, color-thumb should not claim migration path token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for source in [load_source("check2"), load_source("check2_src")] {
        for needle in [
            "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
            "N/A：本次 `ColorThumb` 改动未引入跨大版本 API 破坏升级",
            "`components/color-thumb/src/protocol.rs` 仍仅声明 `ThumbComponentSchemaVersion::V1` 与 `ThumbComponentSpec`",
            "`components/color-thumb/src/Component.toml` 保持 `schema_version = \"1\"` 与 `schema = \"ui.color-thumb.agent-contract.v1\"`",
            "color_thumb_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
            "`scripts/check-ui-engineering.sh`",
        ] {
            assert!(
                source.contains(needle),
                "checklist should keep codemod/registry migration marker `{needle}`.",
            );
        }
    }
}

#[test]
fn color_thumb_engineering_check_script_covers_contract() {
    let script_source = include_str!("../../../scripts/check-ui-engineering.sh");
    let needle = "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        script_source.contains(
            "echo \"[engineering] contract: color-thumb serde protocol + tracing semantics + runtime boundary leakage\""
        ) && script_source.contains(needle),
        "engineering check script should enforce ColorThumb contract marker `{needle}`.",
    );
}

#[test]
fn color_thumb_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
{
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "color-thumb non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn color_thumb_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-color-thumb\")",
        ".map(|class_name| class_name.as_ref())",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(required),
            "color-thumb logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    assert!(
        view_source.contains("let class_name = Cow::Borrowed(class_name).into_owned();"),
        "color-thumb view should avoid class-name to_string churn via Cow bridge.",
    );

    for forbidden in [
        "\"ui-color-thumb\".to_string()",
        "\"ui-color-thumb--disabled\".to_string()",
        "\"ui-color-thumb--focused\".to_string()",
        "\"ui-color-thumb--dragging\".to_string()",
        "\"ui-color-thumb--custom-class\".to_string()",
        "String::from(\"ui-color-thumb\")",
        "class_name.to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-thumb should avoid string clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn color_thumb_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = include_str!("../../../scripts/check-rust-hygiene.sh");
    let engineering_script = include_str!("../../../scripts/check-ui-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }

    for source in [load_source("check2"), load_source("check2_src")] {
        for required in [
            "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
            "color_thumb_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
            "color_thumb_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
            "color_thumb_rust_hygiene_script_enforces_repo_level_hygiene_guards",
            "`scripts/check-ui-engineering.sh`",
            "`./scripts/check-rust-hygiene.sh`",
        ] {
            assert!(
                source.contains(required),
                "color-thumb checklist should keep rust-hygiene evidence marker `{required}`.",
            );
        }
    }
}

#[test]
fn color_thumb_checklist_marks_ui_components_definition_complete() {
    let check2 = load_source("check2");
    let check2_src = load_source("check2_src");

    for source in [check2, check2_src] {
        assert!(
            source.contains("- [x] `ui` 定义"),
            "color-thumb check2 should mark ui definition as completed.",
        );
        assert!(
            source.contains("- [x] API 命名契约统一"),
            "color-thumb check2 should mark API naming contract as completed.",
        );
        assert!(
            source.contains("- [x] 默认值单一来源"),
            "color-thumb check2 should mark single-source default normalization as completed.",
        );
        assert!(
            source.contains("- [x] 状态归一化集中"),
            "color-thumb check2 should mark centralized state-normalization item as completed.",
        );
        assert!(
            source.contains("- [x] 离散状态必须类型约束"),
            "color-thumb check2 should mark discrete-state enum typing item as completed.",
        );
        assert!(
            source.contains("- [x] 状态原语来源正确"),
            "color-thumb check2 should mark status-primitives source item as completed.",
        );
        assert!(
            source.contains("- [x] 如果无异步相关，直接打勾"),
            "color-thumb check2 should mark async semantics item as completed.",
        );
        assert!(
            source.contains("组件无远程请求与异步状态"),
            "color-thumb check2 should include explicit N/A rationale for async semantics item.",
        );
        assert!(
            source.contains("- [x] API 易用性验收标准（DX Paradox）"),
            "color-thumb check2 should mark DX paradox item as completed.",
        );
        assert!(
            source.contains("基础调用仅需 `<ColorThumb id_base=... />`"),
            "color-thumb check2 should include explicit rationale for DX paradox item.",
        );
        assert!(
            source.contains("- [x] 组合型组件主 API 必须“显示优于约定”"),
            "color-thumb check2 should mark composite API item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 是原子组件"),
            "color-thumb check2 should include explicit N/A rationale for composite API item.",
        );
        assert!(
            source.contains("- [x] 宏观/微观双状态机（Macro/Micro Duality）"),
            "color-thumb check2 should mark macro/micro duality item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 仅消费上游 `is_dragging` 快照"),
            "color-thumb check2 should include explicit N/A rationale for macro/micro duality item.",
        );
        assert!(
            source.contains("- [x] 几何两段式渲染（Two-Pass Rendering）"),
            "color-thumb check2 should mark two-pass rendering item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 不承载 overlay/tooltip/menu 布局"),
            "color-thumb check2 should include explicit N/A rationale for two-pass rendering item.",
        );
        assert!(
            source.contains("- [x] 集合注册协议（Registration Protocol）"),
            "color-thumb check2 should mark registration protocol item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 不管理动态子项集合"),
            "color-thumb check2 should include explicit N/A rationale for registration protocol item.",
        );
        assert!(
            source.contains("- [x] 插槽投影策略（Slot Projection）"),
            "color-thumb check2 should mark slot projection item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 不接受子内容投影"),
            "color-thumb check2 should include explicit N/A rationale for slot projection item.",
        );
        assert!(
            source.contains("- [x] 环境订阅流（Env Streams）"),
            "color-thumb check2 should mark env stream item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 不订阅 `Resize/Theme/Intersection` 等环境流"),
            "color-thumb check2 should include explicit N/A rationale for env stream item.",
        );
        assert!(
            source.contains("- [x] 事件光锥（Event Light Cone）"),
            "color-thumb check2 should mark event light cone item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 非大型集合组件"),
            "color-thumb check2 should include explicit N/A rationale for event light cone item.",
        );
        assert!(
            source.contains("- [x] 统一因果总线（Causality Bus）"),
            "color-thumb check2 should mark causality bus item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 不实现跨订阅者派生总线或命令广播链路"),
            "color-thumb check2 should include explicit N/A rationale for causality bus item.",
        );
        assert!(
            source.contains("- [x] 焦点全局栈（Focus Stack & GC）"),
            "color-thumb check2 should mark focus-stack item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 不是 overlay 容器"),
            "color-thumb check2 should include explicit N/A rationale for focus-stack item.",
        );
        assert!(
            source.contains("`FallbackTo/Selector` 这类全局焦点恢复职责应由 `ui-headless` 焦点管理与上游 overlay 组件统一承载"),
            "color-thumb check2 should route global focus recovery responsibility to headless + overlay owner.",
        );
        assert!(
            source.contains("- [x] 受控外交特区（Escape Hatches）"),
            "color-thumb check2 should mark escape-hatches item as completed.",
        );
        assert!(
            source.contains("ColorThumb` 不集成 ECharts/Map 等命令式第三方实例"),
            "color-thumb check2 should include explicit N/A rationale for escape-hatches item.",
        );
        assert!(
            source.contains("不存在 `Foreign Zone`、`YieldControl`、`CleanupForeign` 相关接入面"),
            "color-thumb check2 should state no foreign-zone integration surface exists for color-thumb.",
        );
        assert!(
            source.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）"),
            "color-thumb check2 should mark hydration-discontinuity item as completed.",
        );
        assert!(
            source.contains("根节点 `id` 直接使用调用方传入的 `id_base`"),
            "color-thumb check2 should include explicit deterministic id_base rationale.",
        );
        assert!(
            source.contains("未调用 `now()`、`Uuid::new_v4` 或等价随机源"),
            "color-thumb check2 should explicitly state no time/random id initialization is used.",
        );
        assert!(
            source.contains("`IdProvider`（`UiIdProvider`）在宿主层注入并传入组件"),
            "color-thumb check2 should route stable id allocation to upstream id provider.",
        );
        assert!(
            source.contains("- [x] SSR 与跨平台检查"),
            "color-thumb check2 should mark SSR + cross-platform item as completed.",
        );
        assert!(
            source.contains(
                "`components/color-thumb/src/*.rs` 未引用 `web-sys`/`wasm_bindgen`/`js_sys`"
            ),
            "color-thumb check2 should include explicit non-wasm no-web-sys evidence.",
        );
        assert!(
            source.contains(
                "`crates/ui-motion/src/lib.rs` 的 `wasm32/non-wasm cfg` 与 non-wasm no-op"
            ),
            "color-thumb check2 should include explicit wasm/non-wasm cfg evidence for motion backend.",
        );
        assert!(
            source.contains("`crates/ui-headless/src/lib.rs` 的 `web/ssr` 互斥 `compile_error!`"),
            "color-thumb check2 should include explicit web/ssr mutual-exclusion evidence.",
        );
        assert!(
            source.contains("`cargo check -p ui-color-thumb --target wasm32-unknown-unknown` 与 `cargo check -p ui-headless --no-default-features --features ssr`"),
            "color-thumb check2 should document compile-only command attempts for wasm + ssr paths.",
        );
        assert!(
            source.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护"),
            "color-thumb check2 should mark ui-headless web/ssr mutual-exclusion item as completed.",
        );
        assert!(
            source.contains(
                "`#[cfg(all(feature = \"web\", feature = \"ssr\"))] compile_error!(...)`"
            ),
            "color-thumb check2 should include explicit compile_error mutual-exclusion evidence.",
        );
        assert!(
            source.contains("`ColorThumb` 仅在 `components/color-thumb/src/view.rs` 消费 `ui_headless::{use_color_thumb, ColorThumbOptions}`"),
            "color-thumb check2 should include explicit headless-consumption-only integration evidence.",
        );
        assert!(
            source.contains("`cargo check -p ui-headless --no-default-features --features ssr` 与 `cargo check -p ui-headless --no-default-features --features web,ssr`"),
            "color-thumb check2 should include explicit command evidence for ssr/web+ssr headless checks.",
        );
        assert!(
            source.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub"),
            "color-thumb check2 should mark ui-motion non-wasm no-op/stub item as completed.",
        );
        assert!(
            source.contains("`#[cfg(not(target_arch = \"wasm32\"))] pub mod web { ... }`"),
            "color-thumb check2 should include explicit non-wasm cfg stub evidence for ui-motion.",
        );
        assert!(
            source.contains(
                "`prefers_reduced_motion()` 固定返回 `true`，`animate(&(), ..)` 为 no-op stub"
            ),
            "color-thumb check2 should include explicit predictable non-wasm motion behavior evidence.",
        );
        assert!(
            source.contains("`non_wasm_web_backend_is_predictable_noop` 与 `components/color-thumb/test/motion.rs` 的 `attach_motion_exports_css_variables`"),
            "color-thumb check2 should include explicit tests covering non-wasm motion downgrade path.",
        );
        assert!(
            source.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支"),
            "color-thumb check2 should mark reduced-motion/SSR/wasm coverage item as completed.",
        );
        assert!(
            source.contains("`resolve_runtime_motion` 在 `ui_motion::web::prefers_reduced_motion()` 为真时将时长降级到 `1ms`"),
            "color-thumb check2 should include explicit reduced-motion downgrade evidence.",
        );
        assert!(
            source.contains("`crates/ui-motion/src/web.rs` 在 wasm 分支 `prefers-reduced-motion` 命中时直接跳过 `animate(...)`"),
            "color-thumb check2 should include explicit wasm reduced-motion skip evidence.",
        );
        assert!(
            source.contains(
                "`role/aria/data-*` 语义全部来自 `use_color_thumb`，不依赖 wasm 专属分支"
            ),
            "color-thumb check2 should include explicit SSR/wasm semantic-contract consistency evidence.",
        );
        assert!(
            source.contains(
                "`cargo test -p ui-color-thumb color_thumb_semantic_tests_cover_interaction_and_platform_matrix -- --nocapture` 已通过"
            ),
            "color-thumb check2 should include explicit reduced-motion/platform-matrix test command evidence.",
        );
        assert!(
            source.contains("- [x] 性能治理：关键路径有预算"),
            "color-thumb check2 should mark performance governance item as completed.",
        );
        assert!(
            source.contains(
                "\"color-thumb\" => UiPerfBudget { max_mount_ms: 24.0, max_update_ms: Some(8.0), max_heap_kb: Some(384.0) }"
            ),
            "color-thumb check2 should include explicit color-thumb performance budget evidence.",
        );
        assert!(
            source.contains(
                "`data-perf-mount-ms`、`data-perf-budget-ms`、`data-perf-observability`、`data-perf-violation`"
            ),
            "color-thumb check2 should include explicit perf probe/e2e observability evidence.",
        );
        assert!(
            source.contains(
                "`view.rs`/`styles.rs`/`motion.rs` 暴露 `data-state`、`data-*-source`、`--ui-color-thumb-*` 与 `resolve_runtime_motion`"
            ),
            "color-thumb check2 should include explicit attribution-path evidence for state/render/style/motion.",
        );
        assert!(
            source.contains(
                "`docs/plan/TODO.md` 的“建立 `render_count` 自动化回归（Button/Input/Accordion"
            ) && source.contains("mount-only 等价证据"),
            "color-thumb check2 should include explicit render_count follow-up tracking evidence.",
        );
        assert!(
            source.contains("color_thumb_performance_governance_budget_is_defined_and_blocking"),
            "color-thumb check2 should include explicit component-local performance test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_performance_governance_contract_is_budgeted_traceable_and_blocking"
            ),
            "color-thumb check2 should include explicit ui performance test evidence.",
        );
        assert!(
            source.contains("`scripts/check-ui-performance.sh`"),
            "color-thumb check2 should include explicit performance gate script evidence.",
        );
        assert!(
            source.contains("- [x] 存在 A11y 实现、国际化与本地化实现"),
            "color-thumb check2 should mark a11y + i18n/l10n item as completed.",
        );
        assert!(
            source.contains("`view.rs` 不再硬编码 `\"None\"`"),
            "color-thumb check2 should include explicit rationale for removing hardcoded view text.",
        );
        assert!(
            source.contains("- [x] 状态可观测、可检索、可验证"),
            "color-thumb check2 should mark state observability item as completed.",
        );
        assert!(
            source.contains("`x/y/loupe/aria-valuetext/class/interaction`"),
            "color-thumb check2 should include explicit rationale for source marker coverage.",
        );
        assert!(
            source.contains("- [x] 样式依赖显式状态（`data-*`/class）"),
            "color-thumb check2 should mark explicit-state styling item as completed.",
        );
        assert!(
            source.contains("`loupe` 节点常驻并由 `data-loupe-visible` 控制显隐"),
            "color-thumb check2 should include explicit rationale for semantic-marker-driven visual toggles.",
        );
        assert!(
            source.contains("- [x] 测试验证“语义契约”而不只验证视觉快照"),
            "color-thumb check2 should mark semantics-over-snapshot item as completed.",
        );
        assert!(
            source.contains("`role/aria/data-state/source markers`"),
            "color-thumb check2 should include explicit rationale for semantic marker coverage.",
        );
        assert!(
            source.contains("`disabled + keyboard + pointer handlers`"),
            "color-thumb check2 should include explicit rationale for interaction-path matrix coverage.",
        );
        assert!(
            source.contains("当前未引入视觉快照断言，语义契约断言为主"),
            "color-thumb check2 should explicitly state snapshot assertions are not the primary contract.",
        );
        assert!(
            source.contains("- [x] 组件文件职责正确"),
            "color-thumb check2 should mark component file responsibility item as completed.",
        );
        assert!(
            source.contains("`mod.rs` 仅保留模块边界与 `pub use`"),
            "color-thumb check2 should include explicit rationale for mod.rs boundary responsibility.",
        );
        assert!(
            source.contains("`logic.rs` 仅装配 `ui-state-primitives` 输入归一与来源标记"),
            "color-thumb check2 should include explicit rationale for logic.rs responsibility.",
        );
        assert!(
            source.contains("`view.rs` 仅做 Leptos 结构与 `ui-headless` attrs/handlers 挂载"),
            "color-thumb check2 should include explicit rationale for view.rs responsibility.",
        );
        assert!(
            source.contains("`motion.rs` 仅做语义到 CSS 变量 contract 映射与 attach"),
            "color-thumb check2 should include explicit rationale for motion.rs responsibility.",
        );
        assert!(
            source.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
            "color-thumb check2 should mark spec.rs governance item as completed.",
        );
        assert!(
            source.contains("`components/color-thumb/src/` 未新增 `spec.rs`"),
            "color-thumb check2 should include explicit rationale proving no spec.rs was added.",
        );
        assert!(
            source.contains("`mod.rs/lib.rs` 也未引入 `mod spec` 或 `pub mod spec`"),
            "color-thumb check2 should include explicit rationale proving no spec module export exists.",
        );
        assert!(
            source.contains("- [x] 组件层遵循 token-first 静态样式契约"),
            "color-thumb check2 should mark token-first static style contract item as completed.",
        );
        assert!(
            source.contains("`crates/ui/src/css.rs` 以 `component-color_thumb` feature gate 聚合"),
            "color-thumb check2 should include explicit rationale for css.rs feature-gated aggregation.",
        );
        assert!(
            source.contains(
                "`crates/ui/src/root.rs` 在 `UiRoot` 的 `inject_components_css` 路径统一注入"
            ),
            "color-thumb check2 should include explicit rationale for UiRoot css injection path.",
        );
        assert!(
            source.contains("仅通过 `style=style_vars` 注入 `--ui-color-thumb-*` 动效变量"),
            "color-thumb check2 should include explicit rationale for runtime style-vars-only contract.",
        );
        assert!(
            source.contains("- [x] 默认主题美学质量达标（Visual Desire）"),
            "color-thumb check2 should mark visual desire item as completed.",
        );
        assert!(
            source.contains("跨组件默认主题治理门禁"),
            "color-thumb check2 should include explicit N/A rationale for cross-component visual baseline governance.",
        );
        assert!(
            source.contains("不属于 `ColorThumb` 单组件可独立闭环范围"),
            "color-thumb check2 should explicitly scope visual desire gate as non-component-local.",
        );
        assert!(
            source.contains("升级到统一的 docs-app/视觉基线任务执行"),
            "color-thumb check2 should include explicit escalation path for repo-level visual baseline work.",
        );
        assert!(
            source.contains("- [x] Tree Shaking 是一等能力"),
            "color-thumb check2 should mark tree-shaking item as completed.",
        );
        assert!(
            source.contains("- [x] 静态片段常量化"),
            "color-thumb check2 should mark static-fragment constantization item as completed.",
        );
        assert!(
            source.contains("- [x] `inner_html` 使用约束"),
            "color-thumb check2 should mark inner_html safety item as completed.",
        );
        assert!(
            source.contains("- [x] WASM 调试要求：关键状态可追踪"),
            "color-thumb check2 should mark wasm-debug governance item as completed.",
        );
        assert!(
            source.contains("- [x] DX 要求：样式热重载优先无需重编 wasm"),
            "color-thumb check2 should mark DX governance item as completed.",
        );
        assert!(
            source.contains("- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化"),
            "color-thumb check2 should mark engineering-governance item as completed.",
        );
        assert!(
            source.contains("- [x] 样式孤岛防御（Defensive Variables）"),
            "color-thumb check2 should mark defensive-variable item as completed.",
        );
        assert!(
            source.contains("- [x] 级联层覆盖（`@layer ui`）"),
            "color-thumb check2 should mark cascade-layer item as completed.",
        );
        assert!(
            source.contains("- [x] Motion 合同化"),
            "color-thumb check2 should mark motion-contract item as completed.",
        );
        assert!(
            source.contains("- [x] `ui` 固定入口文件落点正确。"),
            "color-thumb check2 should mark fixed-entrypoint item as completed.",
        );
        assert!(
            source.contains("- [x] 组件目录标准文件落点正确。"),
            "color-thumb check2 should mark component-directory standard files item as completed.",
        );
        assert!(
            source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
            "color-thumb check2 should mark file-placement discipline item as completed.",
        );
        assert!(
            source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
            "color-thumb check2 should mark hyper-structure-builder item as completed.",
        );
        assert!(
            source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
            "color-thumb check2 should mark context-compression item as completed.",
        );
        assert!(
            source.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。"),
            "color-thumb check2 should mark Agent Contract item as completed.",
        );
        assert!(
            source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
            "color-thumb check2 should mark streaming-definition item as completed.",
        );
        assert!(
            source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
            "color-thumb check2 should mark snapshot-baseline item as completed.",
        );
        assert!(
            source.contains("- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`"),
            "color-thumb check2 should mark rust-hygiene item as completed in section 7.",
        );
        assert!(
            source.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
            "color-thumb check2 should mark streaming required/optional classification item as completed.",
        );
        assert!(
            source.contains(
                "color_thumb_static_fragments_are_constantized_or_absent_for_simple_layout"
            ),
            "color-thumb check2 should include explicit static-fragment regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_inner_html_usage_is_forbidden_in_component_and_docs_examples"
            ),
            "color-thumb check2 should include explicit inner_html regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_wasm_debug_contract_reuses_shared_trace_and_stays_feature_isolated"
            ),
            "color-thumb check2 should include explicit wasm-debug regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_dx_playground_supports_css_hot_reload_and_context_with_optional_persist_na"
            ),
            "color-thumb check2 should include explicit DX regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries"
            ),
            "color-thumb check2 should include explicit engineering regression test evidence.",
        );
        assert!(
            source.contains("color_thumb_styles_use_defensive_variable_fallback_chain"),
            "color-thumb check2 should include explicit defensive-variable regression test evidence.",
        );
        assert!(
            source.contains("color_thumb_cascade_layer_and_runtime_style_contract_is_enforced"),
            "color-thumb check2 should include explicit cascade-layer regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop"
            ),
            "color-thumb check2 should include explicit motion-contract regression test evidence.",
        );
        assert!(
            source
                .contains("color_thumb_ui_components_fixed_entry_files_follow_layered_boundaries"),
            "color-thumb check2 should include explicit fixed-entrypoint regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_component_directory_standard_files_follow_contract_and_na_paths"
            ),
            "color-thumb check2 should include explicit component-directory regression test evidence.",
        );
        assert!(
            source.contains("color_thumb_component_files_script_covers_standard_layout_contract"),
            "color-thumb check2 should include explicit component-files script regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_file_placement_discipline_contract_is_explicit_for_interactive_component_scope"
            ),
            "color-thumb check2 should include explicit file-placement discipline regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_component_files_script_covers_file_placement_discipline_contract"
            ),
            "color-thumb check2 should include explicit file-placement script regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_hyper_structure_builder_spec_is_not_applicable_for_simple_component"
            ),
            "color-thumb check2 should include explicit hyper-structure-builder regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_component_files_script_covers_hyper_structure_builder_na_contract"
            ),
            "color-thumb check2 should include explicit hyper-structure script regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_context_compression_manifest_and_rbi_projection_are_present_and_current"
            ),
            "color-thumb check2 should include explicit context-compression regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_component_files_script_covers_context_compression_manifest_and_rbi_contract"
            ),
            "color-thumb check2 should include explicit context-compression script regression test evidence.",
        );
        assert!(
            source.contains("color_thumb_agent_contract_is_schema_typed_and_machine_readable"),
            "color-thumb check2 should include explicit Agent Contract schema regression test evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_agent_contract_render_path_is_whitelist_safe_and_script_injection_free"
            ),
            "color-thumb check2 should include explicit Agent Contract whitelist regression test evidence.",
        );
        assert!(
            source.contains("白名单能力边界"),
            "color-thumb check2 should include explicit Agent Contract whitelist rationale.",
        );
        assert!(
            source.contains("color_thumb_check2_documents_streaming_definition_is_llm_output_only_with_two_modes"),
            "color-thumb check2 should include explicit LLM-only streaming-definition regression evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_snapshot_baseline_consumes_complete_result_and_renders_stably"
            ),
            "color-thumb check2 should include explicit snapshot-baseline regression evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources"
            ),
            "color-thumb check2 should include explicit rust-hygiene no-unwrap/no-let_underscore regression evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent"
            ),
            "color-thumb check2 should include explicit rust-hygiene Cow hotspot regression evidence.",
        );
        assert!(
            source.contains("color_thumb_rust_hygiene_script_enforces_repo_level_hygiene_guards"),
            "color-thumb check2 should include explicit rust-hygiene script coverage regression evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_check2_documents_streaming_required_optional_classification_rules"
            ),
            "color-thumb check2 should include explicit streaming-classification regression evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous"
            ),
            "color-thumb check2 should include explicit optional-streaming semantics continuity regression evidence.",
        );
        assert!(
            source.contains(
                "color_thumb_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer"
            ),
            "color-thumb check2 should include explicit upstream-responsibility regression evidence.",
        );
        assert!(
            source.contains("`ColorThumb` 归类为 `Streaming Optional`"),
            "color-thumb check2 should include explicit streaming-optional classification rationale.",
        );
        assert!(
            source.contains("data-ui-stream-fallback=\\\"snapshot\\\""),
            "color-thumb check2 should include explicit snapshot-fallback marker rationale.",
        );
        assert!(
            source.contains("token-by-token streaming 协议"),
            "color-thumb check2 should include explicit N/A rationale for component-local streaming protocol support.",
        );
        assert!(
            source.contains("`scripts/check-ui-entrypoints.sh`"),
            "color-thumb check2 should include explicit fixed-entrypoint gate script evidence.",
        );
        assert!(
            source.contains("`scripts/check-ui-component-files.sh`"),
            "color-thumb check2 should include explicit component-files gate script evidence.",
        );
        assert!(
            source.contains("`scripts/check-ui-streaming.sh`"),
            "color-thumb check2 should include explicit streaming gate script evidence.",
        );
        assert!(
            source.contains("`scripts/check-ui-engineering.sh`"),
            "color-thumb check2 should include explicit engineering gate script evidence for rust-hygiene regressions.",
        );
        assert!(
            source.contains("`./scripts/check-rust-hygiene.sh`"),
            "color-thumb check2 should include explicit repo rust-hygiene command evidence.",
        );
        assert!(
            source.contains("`SLOT_COLOR_THUMB*`、`CLASS_COLOR_THUMB*`、`BOOL_TRUE`"),
            "color-thumb check2 should include explicit static token centralization evidence.",
        );
        assert!(
            source.contains(
                "`crates/ui/src/lib.rs` 通过 `#[cfg(feature = \"component-color_thumb\")]` 门控 `pub mod color_thumb`"
            ),
            "color-thumb check2 should include explicit rationale for lib.rs feature-gated color_thumb export.",
        );
        assert!(
            source.contains(
                "`cargo tree -e features -i ui -p ui --no-default-features --features component-color_thumb,inject-css`"
            ),
            "color-thumb check2 should include explicit feature-tree verification command for minimal color-thumb chain.",
        );
        assert!(
            source
                .contains("`cargo tree -e features -i ui -p web-demo` 输出未出现 `all-components`"),
            "color-thumb check2 should include explicit reverse-dependency verification proving all-components is not implicitly enabled.",
        );
        assert!(
            source.contains("`Invalid cross-device link (os error 18)`"),
            "color-thumb check2 should document current local wasm compile blocker for CI follow-up.",
        );
        assert!(
            source.contains("- [x] 类型系统 + 语义标记共同提供机器可读状态"),
            "color-thumb check2 should mark typed machine-readable state contract item as completed.",
        );
        assert!(
            source.contains("`ColorThumbInteractionState` / `ColorThumbInputSource` / `ColorThumbAriaValueTextSource`"),
            "color-thumb check2 should include explicit enum-backed state-axis rationale.",
        );
        assert!(
            source.contains("`interaction_state_from_flags + resolve_component_state`"),
            "color-thumb check2 should include explicit logic normalization rationale.",
        );
        assert!(
            source.contains(
                "`data-state`、`data-*-source`、`data-x-bucket`、`data-y-bucket`、`aria-*`"
            ),
            "color-thumb check2 should include explicit machine-readable semantic marker rationale.",
        );
        assert!(
            source.contains("`components/color-thumb/test/logic.rs`、`components/color-thumb/test/semantics.rs` 与 `components/color-thumb/test/color_thumb_semantics.rs`"),
            "color-thumb check2 should include explicit contract-regression coverage rationale.",
        );
    }

    assert!(
        check2.contains("- [x] Tree Shaking & 特性剪裁"),
        "color-thumb check2 should mark tree-shaking feature-pruning item as completed in section 7.",
    );
    assert!(
        check2.contains("`component-color_thumb` 已注册在 `crates/ui/Cargo.toml` 特性树"),
        "color-thumb check2 should include explicit rationale for feature-tree registration.",
    );
}
