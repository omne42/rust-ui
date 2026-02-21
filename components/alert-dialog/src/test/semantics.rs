#[test]
fn view_contract_exposes_overlay_and_semantic_markers() {
    let source = include_str!("../view.rs");

    for needle in [
        "role=\"alertdialog\"",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-variant=move || root_state.get().variant_attr",
        "data-description=move || root_state.get().description_attr",
        "data-cancel=move || root_state.get().cancel_attr",
        "data-secondary=move || root_state.get().secondary_attr",
        "data-confirm-source=move || root_state.get().confirm_source_attr",
        "data-auto-focus-source=move || root_state.get().auto_focus_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "alert-dialog view semantics contract should include `{needle}`."
        );
    }
}

#[test]
fn module_contract_keeps_layered_public_surface() {
    let source = include_str!("../mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod protocol;",
        "pub mod styles;",
        "mod view;",
        "pub use view::AlertDialog;",
    ] {
        assert!(
            source.contains(needle),
            "alert-dialog module contract should include `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "mod spec;",
        "pub mod spec;",
    ] {
        assert!(
            !source.contains(forbidden),
            "alert-dialog module should not expose `{forbidden}`."
        );
    }
}

#[test]
fn docs_page_covers_primary_playgrounds() {
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );

    for needle in [
        "pub(super) fn alert_dialog() -> AnyView",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "alert-dialog docs page should include `{needle}`."
        );
    }
}

#[test]
fn docs_playgrounds_lock_state_matrix_contract_values() {
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );

    for needle in [
        "data-slot=\"alert-dialog-state-matrix\"",
        "data-slot=\"alert-dialog-controlled-uncontrolled\"",
        "data-slot=\"alert-dialog-streaming-contract\"",
        "id_base=\"docs-alert-matrix\".to_string()",
        "id_base=\"docs-alert-controlled\".to_string()",
        "id_base=\"docs-alert-stream\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "alert-dialog docs state-matrix contract should include `{needle}`."
        );
    }
}

#[test]
fn files_keep_component_layer_responsibilities() {
    let mod_source = include_str!("../mod.rs");
    let logic_source = include_str!("../logic.rs");
    let styles_source = include_str!("../styles.rs");
    let view_source = include_str!("../view.rs");
    let motion_source = include_str!("../motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod protocol;",
        "pub mod styles;",
        "mod view;",
        "pub use view::AlertDialog;",
    ] {
        assert!(
            mod_source.contains(needle),
            "alert-dialog mod boundary should include `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !mod_source.contains(forbidden),
            "alert-dialog mod boundary should not expose `{forbidden}`."
        );
    }

    for needle in [
        "normalize_required_text",
        "normalize_optional_text",
        "resolve_disabled_flag",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation contract `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "NodeRef<", "view!", "<Overlay"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include DOM/view concerns `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-)", ".ui-alert-dialog"] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should keep token-first static css contract `{needle}`."
        );
    }

    for forbidden in ["role=\"alertdialog\"", "Delete workspace?", "Save draft"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not include behavior/document text `{forbidden}`."
        );
    }

    for needle in [
        "logic::resolve_state(AlertDialogPartStateInput {",
        "logic::compose_class_name(None, root_state.get())",
        "<Overlay",
        "locale_attrs(lang, dir)",
        "data-slot=move || root_state.get().slot_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep structure + semantic mount contract `{needle}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: AlertDialogMotion) -> AlertDialogMotion",
        "crate::overlay::motion::sanitize_motion(motion.overlay)",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep motion-contract mapping `{needle}`."
        );
    }

    for forbidden in ["request_animation_frame", "set_timeout_with_callback"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not implement runtime driver details `{forbidden}`."
        );
    }
}

#[test]
fn styles_follow_token_first_and_avoid_component_level_utility_patterns() {
    let styles_source = include_str!("../styles.rs");
    let view_source = include_str!("../view.rs");

    for needle in ["pub const CSS: &str", "var(--ui-)", ".ui-alert-dialog"] {
        assert!(
            styles_source.contains(needle),
            "alert-dialog styles should keep token-first contract `{needle}`."
        );
    }

    for forbidden in ["rgb(", "hsl(", "oklch(", "color-mix("] {
        assert!(
            !styles_source.contains(forbidden),
            "alert-dialog styles should avoid hard-coded color literal pattern `{forbidden}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "docs-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "alert-dialog component view should avoid app-layer utility class pattern `{forbidden}`."
        );
    }

    for forbidden in ["style=", "css!(", "styled(", "styled_components"] {
        assert!(
            !view_source.contains(forbidden),
            "alert-dialog component view should avoid css-in-rust or inline style pattern `{forbidden}`."
        );
    }
}

#[test]
fn styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let styles_source = include_str!("../styles.rs");
    let theme_css_source = include_str!("../../../../crates/ui-theme/src/css.rs");

    for required in [
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-icon-size-200, var(--ui-fallback-icon-size-200))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fallback-heading-h5-font-size)",
        "var(--ui-fallback-heading-h5-line-height)",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-fg, var(--ui-fallback-fg))",
    ] {
        assert!(
            styles_source.contains(required),
            "alert-dialog styles should keep defensive fallback chain marker `{required}`."
        );
    }

    for required in [
        "--ui-fallback-overlay-panel-min-width:",
        "--ui-fallback-icon-size-200:",
        "--ui-fallback-heading-h5-font-size:",
        "--ui-fallback-heading-h5-line-height:",
        "--ui-fallback-space-2xs:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-md:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-accent:",
        "--ui-fallback-danger:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css should provide fallback terminal `{required}`."
        );
    }

    for forbidden in [
        "var(--ui-space-md);",
        "var(--ui-overlay-panel-min-width);",
        "var(--ui-icon-size-200);",
        "var(--ui-heading-h5-font-size);",
        "var(--ui-heading-h5-line-height);",
        "var(--ui-font-size-150);",
        "var(--ui-line-height-150);",
        "#",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "alert-dialog styles should avoid raw terminal token `{forbidden}`."
        );
    }
}

#[test]
fn defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = include_str!("../../../../crates/ui-components/src/css.rs");
    let root_source = include_str!("../../../../crates/ui-components/src/root.rs");
    let view_source = include_str!("../view.rs");
    let styles_source = include_str!("../styles.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-alert_dialog\")]",
        "out.push_str(crate::alert_dialog::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui-components css entry should enforce cascade-layer contract `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`."
        );
    }

    assert!(
        !view_source.contains(" style="),
        "alert-dialog view should not embed plain inline style assignments."
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "alert-dialog view should not include fragile inline style token `{forbidden}`."
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
                "alert-dialog runtime style should only set css custom properties; found `style:{key}` at line {}.",
                line_index + 1
            );
        }
    }

    for needle in ["pub const CSS: &str", ".ui-alert-dialog", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "alert-dialog styles should remain static token css contract `{needle}`."
        );
    }
}

#[test]
fn cascade_layer_check_script_covers_alert_dialog_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn focus_restore_contract_is_delegated_to_overlay_and_not_local_noderef_state() {
    let source = include_str!("../view.rs");

    for needle in [
        "let cancel_ref: NodeRef<html::Button> = NodeRef::new();",
        "let secondary_ref: NodeRef<html::Button> = NodeRef::new();",
        "let confirm_ref: NodeRef<html::Button> = NodeRef::new();",
        "focus_button_soon(target);",
        "<Overlay",
    ] {
        assert!(
            source.contains(needle),
            "alert-dialog view should include `{needle}`."
        );
    }

    for forbidden in ["restore_focus_ref", "previous_focus_ref", "focus_stack"] {
        assert!(
            !source.contains(forbidden),
            "alert-dialog should not store local restore target `{forbidden}`."
        );
    }
}

#[test]
fn escape_hatch_foreign_zone_contract_is_na_for_alert_dialog() {
    let mod_source = include_str!("../mod.rs");
    let logic_source = include_str!("../logic.rs");
    let view_source = include_str!("../view.rs");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "alert-dialog should not integrate imperative third-party zone token `{forbidden}`."
        );
    }

    for forbidden in ["pub struct Echarts", "pub struct Map", "pub type Foreign"] {
        assert!(
            !mod_source.contains(forbidden),
            "alert-dialog public API should not expose foreign imperative instance `{forbidden}`."
        );
    }
}

#[test]
fn hydration_discontinuity_contract_is_na_without_entropy_init() {
    let mod_source = include_str!("../mod.rs");
    let logic_source = include_str!("../logic.rs");
    let view_source = include_str!("../view.rs");
    let motion_source = include_str!("../motion.rs");

    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "js_sys::Date::now",
        "Date::now",
        "Uuid::new_v4",
        "uuid::Uuid",
        "thread_rng",
        "rand::random",
        "nanoid",
        "random::<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "alert-dialog should remain deterministic across SSR/hydration; found entropy source `{forbidden}`."
        );
    }

    for required in [
        "id_base: String,",
        "let id_base = logic::normalize_id_base(id_base);",
        "let title_id = format!(\"{id_base}-title\");",
        "let description_id = format!(\"{id_base}-description\");",
    ] {
        assert!(
            view_source.contains(required),
            "alert-dialog should keep deterministic caller-owned id contract `{required}`."
        );
    }
}

#[test]
fn platform_cfg_guards_browser_apis_for_wasm_and_non_wasm_paths() {
    let view_source = include_str!("../view.rs");
    let logic_source = include_str!("../logic.rs");
    let styles_source = include_str!("../styles.rs");
    let motion_source = include_str!("../motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "fn focus_button_soon(node_ref: NodeRef<html::Button>)",
        "fn focus_button_soon(_node_ref: NodeRef<html::Button>)",
        "let Some(window) = web_sys::window() else {",
    ] {
        assert!(
            view_source.contains(needle),
            "alert-dialog view should keep explicit platform branch marker `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "js_sys::", "wasm_bindgen::"] {
        assert!(
            !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "non-view files should remain browser-object free: `{forbidden}`."
        );
    }
}

#[test]
fn ui_headless_feature_mutex_contract_is_present_for_alert_dialog_dependency() {
    let view_source = include_str!("../view.rs");
    let headless_lib_source = include_str!("../../../../crates/ui-headless/src/lib.rs");

    assert!(
        view_source.contains("use ui_headless::{A11yDirection, locale_attrs};"),
        "alert-dialog should consume ui-headless a11y contracts from a single dependency surface."
    );

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless mutex contract should include `{needle}`."
        );
    }

    for forbidden in ["feature = \"web\"", "feature = \"ssr\""] {
        assert!(
            !view_source.contains(forbidden),
            "alert-dialog should not hardcode ui-headless feature selection `{forbidden}`."
        );
    }
}

#[test]
fn ui_motion_non_wasm_stub_contract_keeps_alert_dialog_safe() {
    let ui_motion_lib_source = include_str!("../../../../crates/ui-motion/src/lib.rs");
    let ui_motion_stub_test_source =
        include_str!("../../../../crates/ui-motion/tests/non_wasm_stub.rs");
    let overlay_motion_source = include_str!("../../../overlay/src/motion.rs");
    let view_source = include_str!("../view.rs");
    let motion_source = include_str!("../motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }

    for needle in [
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
    ] {
        assert!(
            ui_motion_stub_test_source.contains(needle),
            "ui-motion should keep non-wasm stub regression `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !is_open.get() {",
        "finish_exit.run(());",
    ] {
        assert!(
            overlay_motion_source.contains(needle),
            "overlay motion should keep non-wasm safe degrade path `{needle}`."
        );
    }

    for needle in [
        "let motion = crate::alert_dialog::motion::sanitize_motion(motion);",
        "motion=motion.overlay",
    ] {
        assert!(
            view_source.contains(needle),
            "alert-dialog view should map component motion contract via `{needle}`."
        );
    }

    for forbidden in ["SpringAnimator", "request_animation_frame", "web_sys::"] {
        assert!(
            !motion_source.contains(forbidden),
            "alert-dialog motion layer should not depend on runtime driver details `{forbidden}`."
        );
    }
}

#[test]
fn reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let view_source = include_str!("../view.rs");
    let overlay_motion_source = include_str!("../../../overlay/src/motion.rs");
    let ui_motion_spring_source = include_str!("../../../../crates/ui-motion/src/spring.rs");
    let ui_motion_spring_tests = include_str!("../../../../crates/ui-motion/tests/spring.rs");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring reduced-motion path should keep `{needle}`."
        );
    }

    for needle in [
        "fn reduced_motion_set_target_applies_immediately()",
        "fn reduced_motion_set_target_triggers_on_rest_synchronously()",
        "fn reduced_motion_clear_on_rest_stops_triggering()",
    ] {
        assert!(
            ui_motion_spring_tests.contains(needle),
            "ui-motion reduced-motion regression should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "finish_exit.run(());",
    ] {
        assert!(
            overlay_motion_source.contains(needle),
            "overlay motion should keep SSR/wasm branch contract `{needle}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn AlertDialog(",
        "let motion = crate::alert_dialog::motion::sanitize_motion(motion);",
        "motion=motion.overlay",
        "role=\"alertdialog\"",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            view_source.contains(needle),
            "alert-dialog should keep reduced-motion/SSR/wasm semantic contract `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("role=\"alertdialog\"").count(),
        2,
        "alert-dialog should keep identical alertdialog role mapping in both description/no-description branches."
    );
}

#[test]
fn motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = include_str!("../motion.rs");
    let motion_unit_test_source = include_str!("motion.rs");
    let overlay_motion_source = include_str!("../../../overlay/src/motion.rs");
    let ui_motion_source = include_str!("../../../../crates/ui-motion/src/lib.rs");
    let view_source = include_str!("../view.rs");

    for needle in [
        "pub struct AlertDialogMotion {",
        "pub overlay: crate::overlay::OverlayMotion,",
        "pub fn sanitize_motion(motion: AlertDialogMotion) -> AlertDialogMotion",
        "crate::overlay::motion::sanitize_motion(motion.overlay)",
    ] {
        assert!(
            motion_source.contains(needle),
            "alert-dialog motion contract should include `{needle}`."
        );
    }

    for needle in [
        "fn default_motion_uses_default_overlay_motion_contract()",
        "fn supports_custom_overlay_motion_contract()",
        "stiffness: 240.0",
        "damping: 22.0",
        "fn sanitize_motion_delegates_to_overlay_contract()",
    ] {
        assert!(
            motion_unit_test_source.contains(needle),
            "alert-dialog component-local motion regression should include `{needle}`."
        );
    }

    for needle in [
        "pub struct OverlayMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub fn sanitize_motion(motion: OverlayMotion) -> OverlayMotion",
        "stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {",
        "damping: if value.damping.is_finite() && value.damping > 0.0 {",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "finish_exit.run(());",
    ] {
        assert!(
            overlay_motion_source.contains(needle),
            "overlay motion delegation contract should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion non-wasm/reduced-motion base contract should include `{needle}`."
        );
    }

    for needle in [
        "let motion = crate::alert_dialog::motion::sanitize_motion(motion);",
        "motion=motion.overlay",
    ] {
        assert!(
            view_source.contains(needle),
            "alert-dialog view should mount motion contract via overlay attach path `{needle}`."
        );
    }
}

#[test]
fn motion_contract_platform_script_covers_alert_dialog_guard() {
    let script_source = include_str!("../../../../scripts/check-ui-components-platforms.sh");

    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        script_source.contains(needle),
        "platform check script should enforce `{needle}`."
    );
}

#[test]
fn performance_governance_budget_is_defined_and_blocking() {
    let shell_source = include_str!("../../../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = include_str!("../../../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = include_str!("../../../../apps/docs-app/src/perf_probe.rs");
    let coverage_source =
        include_str!("../../../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = include_str!("../../../../docs/plan/TODO.md");
    let script_source = include_str!("../../../../scripts/check-ui-components-performance.sh");
    let view_source = include_str!("../view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"alert-dialog\" => UiPerfBudget {",
        "max_mount_ms: 36.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget contract token `{needle}`."
        );
    }

    for needle in [
        "\"AlertDialog\",",
        "\"alert-dialog\",",
        "overlays::alert_dialog",
    ] {
        assert!(
            pages_source.contains(needle),
            "AlertDialog docs page should remain in coverage traversal via `{needle}`."
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
            coverage_source.contains(needle),
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || root_state.get().state_attr",
        "data-open=move || open.get().then_some(\"true\")",
        "data-variant=move || root_state.get().variant_attr",
        "data-confirm-disabled=move || root_state.get().confirm_disabled_attr",
        "data-secondary-disabled=move || root_state.get().secondary_disabled_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "AlertDialog view should expose attribution marker `{needle}` for perf triage."
        );
    }

    let script_needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`."
    );
}

#[test]
fn view_macro_complexity_is_split_into_semantic_subrenders() {
    let source = include_str!("../view.rs");

    for needle in [
        "struct DialogHeaderViewCtx {",
        "fn render_dialog_header(ctx: DialogHeaderViewCtx) -> AnyView",
        "struct DialogFooterViewCtx {",
        "fn render_dialog_footer(ctx: DialogFooterViewCtx) -> AnyView",
        "struct DialogContentViewCtx {",
        "fn render_dialog_content(ctx: DialogContentViewCtx) -> AnyView",
        "let header_view = render_dialog_header(DialogHeaderViewCtx {",
        "let footer_view = render_dialog_footer(DialogFooterViewCtx {",
        "render_dialog_content(DialogContentViewCtx {",
    ] {
        assert!(
            source.contains(needle),
            "alert-dialog view should keep macro complexity split marker `{needle}`."
        );
    }

    assert_eq!(
        source.matches("#[component]").count(),
        1,
        "alert-dialog should keep a single public component boundary."
    );
}

#[test]
fn view_functional_split_prefers_plain_functions_over_local_components() {
    let source = include_str!("../view.rs");

    for needle in [
        "fn render_variant_type_icon(variant: AlertDialogVariant) -> AnyView",
        "fn render_dialog_header(ctx: DialogHeaderViewCtx) -> AnyView",
        "fn render_dialog_footer(ctx: DialogFooterViewCtx) -> AnyView",
        "fn render_dialog_content(ctx: DialogContentViewCtx) -> AnyView",
        "let header_view = render_dialog_header(DialogHeaderViewCtx {",
        "let footer_view = render_dialog_footer(DialogFooterViewCtx {",
        "render_dialog_content(DialogContentViewCtx {",
        "data-title-source=ctx.title_state.title_source_attr",
        "data-cancel-source=ctx.cancel_state.cancel_source_attr",
        "data-confirm-source=ctx.confirm_state.confirm_source_attr",
        "data-slot=move || ctx.root_state.get().slot_attr",
        "data-motion-source=move || ctx.root_state.get().motion_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "alert-dialog function-first split should include `{needle}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_variant_type_icon",
        "#[component]\nfn render_dialog_header",
        "#[component]\nfn render_dialog_footer",
        "#[component]\nfn render_dialog_content",
    ] {
        assert!(
            !source.contains(forbidden),
            "alert-dialog local split helper should stay plain fn, not `{forbidden}`."
        );
    }

    assert_eq!(
        source.matches("#[component]").count(),
        1,
        "alert-dialog should keep one component boundary and leave local fragments as plain functions."
    );
    assert!(
        source.contains("pub fn AlertDialog("),
        "alert-dialog public component boundary should remain explicit."
    );
}

#[test]
fn static_fragments_are_constantized_with_templated_type_icons() {
    let source = include_str!("../view.rs");

    for needle in [
        "const ALERT_DIALOG_TYPE_ICON_VIEWBOX: &str = \"0 0 20 20\";",
        "const ALERT_DIALOG_TYPE_ICON_STROKE: &str = \"currentColor\";",
        "const ALERT_DIALOG_WARNING_ICON_OUTLINE_D: &str =",
        "const ALERT_DIALOG_WARNING_ICON_VERTICAL_D: &str = \"M10 7.2v5.8\";",
        "const ALERT_DIALOG_WARNING_ICON_DOT_D: &str = \"M10 15.8h.01\";",
        "const ALERT_DIALOG_ERROR_ICON_RING_D: &str = \"M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16z\";",
        "const ALERT_DIALOG_ERROR_ICON_VERTICAL_D: &str = \"M10 6.2v5.2\";",
        "const ALERT_DIALOG_ERROR_ICON_DOT_D: &str = \"M10 14.2h.01\";",
        "struct AlertDialogTypeIconPath {",
        "const ALERT_DIALOG_WARNING_ICON_PATHS: [AlertDialogTypeIconPath; 3] = [",
        "const ALERT_DIALOG_ERROR_ICON_PATHS: [AlertDialogTypeIconPath; 3] = [",
        "fn render_type_icon_path(path: AlertDialogTypeIconPath) -> AnyView",
        "fn render_static_type_icon(paths: &'static [AlertDialogTypeIconPath]) -> AnyView",
        "render_static_type_icon(&ALERT_DIALOG_WARNING_ICON_PATHS)",
        "render_static_type_icon(&ALERT_DIALOG_ERROR_ICON_PATHS)",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            source.contains(needle),
            "alert-dialog static fragment constantization should include `{needle}`."
        );
    }

    for literal in [
        "M10 2.8l8.2 14.4c.6 1-.1 2.3-1.3 2.3H3.1c-1.2 0-1.9-1.3-1.3-2.3L10 2.8z",
        "M10 7.2v5.8",
        "M10 15.8h.01",
        "M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16z",
        "M10 6.2v5.2",
        "M10 14.2h.01",
    ] {
        assert_eq!(
            source.matches(literal).count(),
            1,
            "alert-dialog type-icon geometry literal `{literal}` should be centralized to one constant."
        );
    }
}

#[test]
fn inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for (rel_path, source) in [
        ("mod.rs", include_str!("../mod.rs")),
        ("logic.rs", include_str!("../logic.rs")),
        ("styles.rs", include_str!("../styles.rs")),
        ("view.rs", include_str!("../view.rs")),
        ("motion.rs", include_str!("../motion.rs")),
        ("protocol.rs", include_str!("../protocol.rs")),
    ] {
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "alert-dialog source `{rel_path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "alert-dialog docs examples must not demonstrate raw html injection token `{forbidden}`."
        );
    }
}

#[test]
fn inner_html_check_script_covers_alert_dialog_security_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-inner-html.sh");

    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce alert-dialog contract marker `{needle}`."
    );
}

#[test]
fn wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = include_str!("../../../../crates/ui-components/Cargo.toml");
    let crate_root_source = include_str!("../../../../crates/ui-components/src/lib.rs");
    let docs_app_source = include_str!("../../../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = include_str!("../../../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = include_str!("../../../../crates/ui-headless/src/trace.rs");
    let button_view_source = include_str!("../../../button/src/view.rs");
    let alert_dialog_view_source = include_str!("../view.rs");
    let alert_dialog_logic_source = include_str!("../logic.rs");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "wasm debug capability should stay feature-gated via `button-wasm-debug`."
    );
    assert!(
        !cargo_source.contains("alert-dialog-wasm-debug")
            && !cargo_source.contains("alert_dialog-wasm-debug")
            && !cargo_source.contains("alert_dialog_wasm_debug"),
        "alert-dialog should not define component-local wasm-debug feature aliases."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components root should keep wasm-debug isolation marker `{needle}`."
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
            "docs-app should expose dev-only wasm debug entry via `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::Inspect",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep visual/temporal trace marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace should keep typed timestamp/source event marker `{needle}`."
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
            button_view_source.contains(needle),
            "shared button wasm debug path should keep trace/replay marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || ctx.root_state.get().state_attr",
        "data-open=move || ctx.open.get().then_some(\"true\")",
        "data-variant=move || ctx.root_state.get().variant_attr",
        "data-confirm-source=move || ctx.root_state.get().confirm_source_attr",
        "data-motion-source=move || ctx.root_state.get().motion_source_attr",
    ] {
        assert!(
            alert_dialog_view_source.contains(needle),
            "alert-dialog should expose machine-readable state/source marker `{needle}` for debug attribution."
        );
    }

    let docs_alert_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );
    for needle in [
        "Playground title=\"State + Source Markers\"",
        "Open marker alert",
        "Inspect data-id-source / data-title-source / data-description-source / data-cancel-source / data-secondary-source / data-motion-source in DevTools.",
        "on_confirm=on_confirm",
        "on_secondary=on_secondary",
    ] {
        assert!(
            docs_alert_source.contains(needle),
            "alert-dialog docs playground should keep minimal replay path marker `{needle}`."
        );
    }

    let combined = format!("{alert_dialog_view_source}\n{alert_dialog_logic_source}");
    for forbidden in [
        "button-wasm-debug",
        "alert-dialog-wasm-debug",
        "alert_dialog_wasm_debug",
        "wasm_debug_proxy!",
        "wasm_debug::",
        "render_debug_panel(",
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
            "alert-dialog component contract should not leak wasm-debug internals `{forbidden}`."
        );
    }
}

#[test]
fn wasm_debug_check_script_covers_alert_dialog_shared_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce `{needle}`."
    );
}

#[test]
fn dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = include_str!("../../../../apps/docs-app/src/playground.rs");
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn alert_dialog() -> AnyView",
        "<Playground",
        "title=\"Hello World (Minimal Path)\"",
        "code_signal=hello_world_code",
        "code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()",
        "title=\"State + Source Markers\"",
        "code_signal=marker_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "AlertDialog docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
fn dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na() {
    let playground_source = include_str!("../../../../apps/docs-app/src/playground.rs");
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );
    let check2_source = include_str!("../check2.md");

    for needle in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "data-slot=\"playground-controls\"",
        "class_name=\"playground__panel playground__controls\".to_string()",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for needle in [
        "let (hello_open_raw, set_hello_open_raw) = signal(false);",
        "let (hello_present, set_hello_present) = signal(hello_open.get_untracked());",
        "let (marker_open_raw, set_marker_open_raw) = signal(false);",
        "let (marker_present, set_marker_present) = signal(marker_open.get_untracked());",
        "let (confirmed, set_confirmed) = signal(0u32);",
        "\"confirmed: \" {move || confirmed.get()}",
        "<Show when=move || hello_present.get()>",
        "<Show when=move || marker_present.get()>",
        "Open marker alert",
    ] {
        assert!(
            docs_source.contains(needle),
            "AlertDialog docs should keep context-preserving interactive marker `{needle}`."
        );
    }

    for forbidden in [
        "ALERT_DIALOG_WORKBENCH_STORAGE_KEY",
        "load_alert_dialog_workbench_state(",
        "save_alert_dialog_workbench_state(",
        "clear_alert_dialog_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "AlertDialog keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2_source.contains(required),
            "AlertDialog checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );

    for needle in [
        "pub(super) fn alert_dialog() -> AnyView",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()",
        "data-slot=\"alert-dialog-state-matrix\"",
        "data-slot=\"alert-dialog-controlled-uncontrolled\"",
        "data-slot=\"alert-dialog-streaming-contract\"",
        "data-slot=\"alert-dialog-streaming-policy\"",
        "Streaming Optional; fallback=snapshot.",
    ] {
        assert!(
            docs_source.contains(needle),
            "alert-dialog docs copy-paste-ready matrix should include `{needle}`."
        );
    }
}

#[test]
fn docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );
    let playground_source = include_str!("../../../../apps/docs-app/src/playground.rs");
    let code_block_view = include_str!("../../../code-block/src/view.rs");

    for needle in [
        "data-slot=\"alert-dialog-source-first\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy alert-dialog starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-alert-dialog-source-copy\".to_string()",
        "use leptos::prelude::*;\\nuse ui_components::{AlertDialog, AlertDialogVariant, OnPress};",
        "ALERT_DIALOG_DOC_IMPORTS",
        "component-alert_dialog",
        "inject-css",
        "data-slot=\"alert-dialog-source-paths\"",
        "components/alert-dialog/src/mod.rs",
        "components/alert-dialog/src/logic.rs",
        "components/alert-dialog/src/view.rs",
        "components/alert-dialog/src/styles.rs",
        "components/alert-dialog/src/motion.rs",
        "components/alert-dialog/src/protocol.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "alert-dialog source-first copy contract should include `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should include `{needle}`."
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view.contains(needle),
            "CodeBlock one-click copy affordance should include `{needle}`."
        );
    }
}

#[test]
fn check2_documents_docs_product_copy_paste_ready_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
            "docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
            "docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
            "dx_check_script_covers_docs_product_copy_paste_ready_contract",
            "scripts/check-ui-components-dx.sh",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 docs-product rule should include `{needle}`."
            );
        }
    }
}

#[test]
fn dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_docs_product_copy_paste_ready_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_dx_check_script_covers_docs_product_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for required in [
            "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
            "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
            "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
            "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
        ] {
            assert!(
                source.contains(required),
                "alert-dialog check2 should keep docs-sync/state-matrix rule `{required}`."
            );
        }
    }
}

#[test]
fn docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );
    let view_source = include_str!("../view.rs");
    let logic_source = include_str!("../logic.rs");
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    docs_page_covers_primary_playgrounds();
    docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "pub(super) fn alert_dialog() -> AnyView",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"alert-dialog-state-matrix\"",
        "data-slot=\"alert-dialog-controlled-uncontrolled\"",
        "open=hello_open",
        "open=matrix_open",
        "id_base=\"docs-alert\".to_string()",
        "id_base=\"docs-alert-matrix\".to_string()",
        "secondary_disabled=state_matrix_is_warning.get()",
        "confirm_disabled=state_matrix_is_error.get()",
        "description=\"`open: Signal<bool>` is the single source of truth at component boundary.\".to_string()",
        "`AlertDialog` requires `open: Signal<bool>` + `on_close`; uncontrolled behavior should be adapted upstream via primitives.",
    ] {
        assert!(
            docs_source.contains(needle),
            "alert-dialog docs should keep docs/matrix/api marker `{needle}`."
        );
    }

    for needle in [
        "open: Signal<bool>",
        "#[prop(optional)] is_confirm_disabled: Option<bool>",
        "#[prop(optional)] confirm_disabled: Option<bool>",
        "#[prop(optional)] is_secondary_disabled: Option<bool>",
        "#[prop(optional)] secondary_disabled: Option<bool>",
        "logic::normalize_required_text(title, logic::DEFAULT_TITLE)",
        "logic::normalize_required_text(confirm_label, logic::DEFAULT_CONFIRM_LABEL)",
        "logic::normalize_cancel_label(cancel_label)",
        "logic::normalize_secondary_label(secondary_label)",
        "logic::resolve_disabled_flag(",
    ] {
        assert!(
            view_source.contains(needle),
            "alert-dialog view API/default marker `{needle}` should remain synced with docs."
        );
    }

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = alert_dialog_state::DEFAULT_ID_BASE;",
        "pub const DEFAULT_TITLE: &str = alert_dialog_state::DEFAULT_TITLE;",
        "pub const DEFAULT_CONFIRM_LABEL: &str = alert_dialog_state::DEFAULT_CONFIRM_LABEL;",
        "pub const DEFAULT_CANCEL_LABEL: &str = alert_dialog_state::DEFAULT_CANCEL_LABEL;",
        "pub const DEFAULT_AUTO_FOCUS_BUTTON: AlertDialogAutoFocusButton = AlertDialogAutoFocusButton::None;",
        "pub const DEFAULT_CONFIRM_DISABLED: bool = alert_dialog_state::DEFAULT_CONFIRM_DISABLED;",
        "pub const DEFAULT_SECONDARY_DISABLED: bool = alert_dialog_state::DEFAULT_SECONDARY_DISABLED;",
    ] {
        assert!(
            logic_source.contains(needle),
            "alert-dialog logic default marker `{needle}` should stay stable for docs sync."
        );
    }

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
            "apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs::alert_dialog",
            "check2_documents_docs_sync_and_state_matrix_rules",
            "docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
            "scripts/check-ui-components-dx.sh",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 docs-sync evidence should include `{needle}`."
            );
        }
    }
}

#[test]
fn dx_check_script_covers_docs_sync_state_matrix_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: alert-dialog docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce docs-sync/state-matrix marker `{needle}`."
        );
    }
}

#[test]
fn check2_marks_docs_sync_and_state_matrix_contract_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        assert!(
            source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
            "alert-dialog check2 should mark docs-sync/state-matrix item complete."
        );

        for needle in [
            "apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs::alert_dialog",
            "title=\"State Matrix\"",
            "title=\"Controlled vs Uncontrolled\"",
            "open: Signal<bool>",
            "DEFAULT_ID_BASE",
            "DEFAULT_TITLE",
            "DEFAULT_CONFIRM_LABEL",
            "DEFAULT_CANCEL_LABEL",
            "check2_documents_docs_sync_and_state_matrix_rules",
            "docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
            "dx_check_script_covers_docs_sync_state_matrix_contract",
            "scripts/check-ui-components-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 docs-sync section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn check2_documents_documentation_as_product_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
            "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
            "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
            "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 documentation-as-product section should include `{needle}`."
            );
        }
    }
}

#[test]
fn documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = include_str!("../README.md");
    let pages_source = include_str!("../../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );

    for needle in [
        "# AlertDialog",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "默认路径：先把确认流程跑起来，不需要先理解底层 primitives/headless 分层。",
        "默认路径：先用 `open + on_close + title + confirm_label`，直接完成基础确认交互。",
        "进阶控制：再启用 `secondary_label + on_secondary`、禁用态（`is_confirm_disabled/is_secondary_disabled`）、`auto_focus_button`、`motion`。",
        "### Controlled Example（高级入口）",
    ] {
        assert!(
            readme_source.contains(needle),
            "alert-dialog README should include beginner-first marker `{needle}`."
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("alert-dialog README should include hello-world section");
    let readme_beginner = readme_source
        .find("## 先用起来，再进阶")
        .expect("alert-dialog README should include beginner-first section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("alert-dialog README should include common-usage section");
    let readme_advanced = readme_source
        .find("### Controlled Example（高级入口）")
        .expect("alert-dialog README should include advanced controlled section");
    assert!(
        readme_hello < readme_beginner
            && readme_beginner < readme_common
            && readme_common < readme_advanced,
        "alert-dialog README should keep beginner-first progression order (hello -> beginner -> common -> advanced).",
    );

    for needle in [
        "\"AlertDialog\",",
        "\"alert-dialog\",",
        "overlays::alert_dialog",
    ] {
        assert!(
            pages_source.contains(needle),
            "alert-dialog docs catalog entry should include `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn alert_dialog() -> AnyView",
        "title=\"AlertDialog\"",
        "slug=\"alert-dialog\"",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "alert-dialog docs page should include `{needle}`."
        );
    }

    let docs_hello = docs_source
        .find("title=\"Hello World (Minimal Path)\"")
        .expect("alert-dialog docs should include hello-world playground");
    let docs_controlled = docs_source
        .find("title=\"Controlled vs Uncontrolled\"")
        .expect("alert-dialog docs should include controlled/uncontrolled playground");
    let docs_streaming = docs_source
        .find("title=\"Streaming / Snapshot Contract\"")
        .expect("alert-dialog docs should include streaming/snapshot playground");
    assert!(
        docs_hello < docs_controlled && docs_controlled < docs_streaming,
        "alert-dialog docs should keep beginner-first progression order (hello -> controlled -> streaming).",
    );
}

#[test]
fn dx_check_script_covers_documentation_as_product_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: alert-dialog documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include documentation-as-product marker `{needle}`."
        );
    }
}

#[test]
fn check2_marks_documentation_as_product_contract_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        assert!(
            source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
            "alert-dialog check2 should mark documentation-as-product item complete.",
        );

        for needle in [
            "components/alert-dialog/src/README.md",
            "apps/docs-app/src/pages/components/pages.rs",
            "apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs",
            "## Hello World（最小可用）",
            "## 先用起来，再进阶",
            "## 常见用法",
            "check2_documents_documentation_as_product_rules",
            "documentation_entry_exists_with_beginner_first_progression",
            "dx_check_script_covers_documentation_as_product_contract",
            "scripts/check-ui-components-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 documentation-as-product section should include `{needle}`."
            );
        }
    }
}

#[test]
fn check2_documents_interactive_playground_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
            "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
            "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
            "Playground 作为验收面，需可重复复现关键交互路径。",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 interactive-playground section should include `{needle}`."
            );
        }
    }
}

#[test]
fn docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );

    docs_page_covers_primary_playgrounds();
    docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "data-slot=\"alert-dialog-state-matrix\"",
        "data-slot=\"alert-dialog-controlled-uncontrolled\"",
        "data-slot=\"alert-dialog-streaming-contract\"",
        "state_matrix_options.clone()",
        "set_state_matrix_index",
        "stream_mode_options.clone()",
        "set_stream_mode_index",
        "Button on_press=open_matrix_alert",
        "Button on_press=open_controlled_alert",
        "Button on_press=open_stream_alert",
        "\"requested mode: \" {move || stream_requested_mode.get()}",
        "\"requested output status: \" {move || stream_requested_output_status.get()}",
    ] {
        assert!(
            docs_source.contains(needle),
            "alert-dialog docs interactive playground surface should include `{needle}`."
        );
    }
}

#[test]
fn interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = include_str!("../../../../e2e/tests/docs_app_alert_dialog_contract.spec.mjs");
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );

    for needle in [
        "docs-app alert-dialog key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "[data-slot=\"alert-dialog-e2e-open-marker\"]",
        "[data-slot=\"alert-dialog-e2e-open-destructive\"]",
        "await page.keyboard.press(\"Enter\");",
        "await page.reload();",
        "await expectAlertDialogSettledClosed(overlayPanel, alertDialog, overlayRoot);",
    ] {
        assert!(
            e2e_source.contains(needle),
            "alert-dialog e2e interactive replay should include `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"alert-dialog-e2e-open-marker\"",
        "data-slot=\"alert-dialog-e2e-open-destructive\"",
        "data-slot=\"alert-dialog-state-matrix\"",
        "data-slot=\"alert-dialog-controlled-uncontrolled\"",
        "data-slot=\"alert-dialog-streaming-contract\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "alert-dialog docs should expose stable interactive anchor `{needle}` for repeatable e2e replay.",
        );
    }
}

#[test]
fn dx_check_script_covers_interactive_playground_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: alert-dialog interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include interactive-playground marker `{needle}`."
        );
    }
}

#[test]
fn check2_marks_interactive_playground_contract_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        assert!(
            source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
            "alert-dialog check2 should mark interactive-playground item complete.",
        );

        for needle in [
            "apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs::alert_dialog",
            "State Matrix",
            "Controlled vs Uncontrolled",
            "Streaming / Snapshot Contract",
            "AI Spec 输入联动：N/A（`AlertDialog` 非 Spec 输入组件，当前无 `spec.rs` 且无 Spec schema 输入面）。",
            "check2_documents_interactive_playground_rules",
            "docs_app_provides_interactive_playground_for_props_state_and_preview",
            "interactive_playground_reuses_repeatable_semantic_e2e_flow",
            "dx_check_script_covers_interactive_playground_contract",
            "scripts/check-ui-components-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 interactive-playground section should include `{needle}`."
            );
        }
    }
}

#[test]
fn check2_documents_source_first_copy_paste_ready_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
            "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
            "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
            "文档代码与当前实现必须同步，防止示例漂移。",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 source-first section should include `{needle}`."
            );
        }
    }
}

#[test]
fn docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );
    let playground_source = include_str!("../../../../apps/docs-app/src/playground.rs");
    let code_block_view = include_str!("../../../code-block/src/view.rs");

    for needle in [
        "data-slot=\"alert-dialog-source-first\"",
        "label=\"Copy alert-dialog starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-alert-dialog-source-copy\".to_string()",
        "ALERT_DIALOG_DOC_IMPORTS",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "component-alert_dialog",
        "inject-css",
        "data-slot=\"alert-dialog-source-paths\"",
        "components/alert-dialog/src/mod.rs",
        "components/alert-dialog/src/logic.rs",
        "components/alert-dialog/src/view.rs",
        "components/alert-dialog/src/styles.rs",
        "components/alert-dialog/src/motion.rs",
        "components/alert-dialog/src/protocol.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "alert-dialog source-first docs should include `{needle}`."
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
            "playground copy pipeline should include `{needle}`."
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view.contains(needle),
            "code-block copy affordance should include `{needle}`."
        );
    }
}

#[test]
fn dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: alert-dialog source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include source-first marker `{needle}`."
        );
    }
}

#[test]
fn check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        assert!(
            source.contains("- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"),
            "alert-dialog check2 should mark source-first item complete.",
        );

        for needle in [
            "alert-dialog-source-first",
            "alert-dialog-source-paths",
            "Copy alert-dialog starter",
            "component-alert_dialog",
            "inject-css",
            "check2_documents_source_first_copy_paste_ready_rules",
            "docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
            "dx_check_script_covers_source_first_copy_paste_ready_contract",
            "scripts/check-ui-components-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 source-first section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn engineering_contract_uses_serde_protocol_and_structured_schema_defaults() {
    let mod_source = include_str!("../mod.rs");
    let protocol_source = include_str!("../protocol.rs");

    assert!(
        mod_source.contains("pub mod protocol;"),
        "alert-dialog module should expose `protocol` for schema migration contract discoverability."
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum AlertDialogComponentSchemaVersion",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct AlertDialogComponentSpec",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(default)]",
        "pub schema_version: AlertDialogComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "alert-dialog protocol should keep serde/schema contract marker `{needle}`."
        );
    }

    for forbidden in [
        "serde_json::",
        "from_json(",
        "to_json_result(",
        "SchemaError",
    ] {
        assert!(
            !protocol_source.contains(forbidden),
            "alert-dialog protocol should avoid ad-hoc serde drift token `{forbidden}`."
        );
    }
}

#[test]
fn engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = include_str!("../../../../crates/ui-components/Cargo.toml");
    let button_view_source = include_str!("../../../button/src/view.rs");
    let combined = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        include_str!("../mod.rs"),
        include_str!("../logic.rs"),
        include_str!("../view.rs"),
        include_str!("../styles.rs"),
        include_str!("../motion.rs"),
        include_str!("../protocol.rs"),
    );

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("alert-dialog-wasm-debug")
            && !cargo_source.contains("alert_dialog-wasm-debug")
            && !cargo_source.contains("alert_dialog_wasm_debug"),
        "alert-dialog should not define component-local tracing feature aliases."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::alert_dialog::",
        "const ALERT_DIALOG_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "alert-dialog should avoid tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = include_str!("../mod.rs");
    let logic_source = include_str!("../logic.rs");
    let view_source = include_str!("../view.rs");
    let styles_source = include_str!("../styles.rs");
    let motion_source = include_str!("../motion.rs");
    let protocol_source = include_str!("../protocol.rs");

    for source in [
        mod_source,
        logic_source,
        view_source,
        styles_source,
        motion_source,
        protocol_source,
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
                "alert-dialog engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "alert-dialog public module boundary should not leak web_sys types."
    );
}

#[test]
fn version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade() {
    let manifest_source = include_str!("../Component.toml");
    let rbi_source = include_str!("../alert_dialog.rbi");
    let protocol_source = include_str!("../protocol.rs");
    let combined = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        include_str!("../mod.rs"),
        include_str!("../logic.rs"),
        include_str!("../view.rs"),
        include_str!("../styles.rs"),
        include_str!("../motion.rs"),
        protocol_source,
    );

    for needle in [
        "schema_version = \"1\"",
        "name = \"AlertDialog\"",
        "crate = \"ui-alert-dialog\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "alert-dialog manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub enum AlertDialogComponentSchemaVersion {",
        "V1,",
        "pub struct AlertDialogComponentSpec {",
        "pub schema_version: AlertDialogComponentSchemaVersion,",
        "pub enum AlertDialogAgentSchemaVersion {",
        "pub fn as_str(self) -> &'static str;",
    ] {
        assert!(
            rbi_source.contains(needle) || protocol_source.contains(needle),
            "alert-dialog schema contracts should keep v1 marker `{needle}`."
        );
    }

    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
        "codemod",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "alert-dialog should not introduce migration/registry marker `{forbidden}` without a major-breaking upgrade."
        );
    }
}

#[test]
fn version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = include_str!("../../../../scripts/check-ui-components-engineering.sh");

    let marker = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = include_str!("../../../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn check2_marks_version_deprecation_migration_registry_item_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
            "N/A：本次 `AlertDialog` 改动未引入跨大版本 API 破坏升级",
            "schema_version = \"1\"",
            "version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
            "version_deprecation_migration_script_covers_engineering_gate",
            "scripts/check-ui-components-engineering.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 version-migration section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = include_str!("../../../../crates/ui-components/src/lib.rs");
    let css_source = include_str!("../../../../crates/ui-components/src/css.rs");
    let root_source = include_str!("../../../../crates/ui-components/src/root.rs");
    let active_highlight_source =
        include_str!("../../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source =
        include_str!("../../../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = include_str!("../../../../crates/ui-headless/src/presence.rs");
    let a11y_source = include_str!("../../../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-alert_dialog\")]",
        "pub mod alert_dialog;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep marker `{needle}`."
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
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-alert_dialog\")]",
        "out.push_str(crate::alert_dialog::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "AlertDialog",
        "Accordion",
        "Button",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }

    let workspace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for forbidden in [
        "../../crates/ui-components/src/overlay_open.rs",
        "../../crates/ui-components/src/presence.rs",
        "../../crates/ui-components/src/a11y.rs",
    ] {
        assert!(
            !workspace_dir.join(forbidden).exists(),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            workspace_dir.join(required).exists(),
            "ui-headless canonical primitive file should exist: `{required}`."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`."
        );
    }
}

#[test]
fn entrypoints_check_script_covers_alert_dialog_fixed_entrypoint_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-entrypoints.sh");
    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_ui_components_fixed_entry_files_follow_layered_boundaries";

    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn component_directory_has_standard_file_layout() {
    let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            crate_dir.join(required).exists(),
            "alert-dialog should keep required component file `{required}`."
        );
    }

    for forbidden in ["src/render.rs", "src/spec.rs"] {
        assert!(
            !crate_dir.join(forbidden).exists(),
            "alert-dialog should not introduce `{forbidden}` for this component scope."
        );
    }
}

#[test]
fn component_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = include_str!("../mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod protocol;",
        "pub mod styles;",
        "mod view;",
        "pub use view::AlertDialog;",
    ] {
        assert!(
            mod_source.contains(needle),
            "alert-dialog mod.rs should keep stable export boundary marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "mod render;",
        "pub mod render;",
        "mod spec;",
        "pub mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "alert-dialog mod.rs should not over-export internal module `{forbidden}`."
        );
    }
}

#[test]
fn component_file_responsibilities_remain_scoped() {
    let logic_source = include_str!("../logic.rs");
    let styles_source = include_str!("../styles.rs");
    let view_source = include_str!("../view.rs");
    let motion_source = include_str!("../motion.rs");

    for needle in [
        "normalize_required_text",
        "normalize_optional_text",
        "resolve_disabled_flag",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation contract marker `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "NodeRef<", "view!", "<Overlay"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include DOM/view concerns `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-)", ".ui-alert-dialog"] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should keep token-first static css contract marker `{needle}`."
        );
    }

    for forbidden in ["rgb(", "hsl(", "oklch(", "color-mix("] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not carry hard-coded theme literal token `{forbidden}`."
        );
    }

    for needle in [
        "logic::resolve_state(AlertDialogPartStateInput {",
        "<Overlay",
        "locale_attrs(lang, dir)",
        "data-slot=move || root_state.get().slot_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep structure + semantic mount marker `{needle}`."
        );
    }

    assert!(
        !view_source.contains("render.rs"),
        "view.rs should not depend on render.rs split path."
    );

    for needle in [
        "pub struct AlertDialogMotion",
        "pub fn sanitize_motion(motion: AlertDialogMotion) -> AlertDialogMotion",
        "crate::overlay::motion::sanitize_motion(motion.overlay)",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep contract mapping marker `{needle}`."
        );
    }

    for forbidden in ["request_animation_frame", "SpringAnimator", "web_sys::"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not implement runtime driver detail `{forbidden}`."
        );
    }
}

#[test]
fn spec_file_is_not_introduced_for_simple_component() {
    let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(
        !crate_dir.join("src/spec.rs").exists(),
        "alert-dialog should keep spec.rs absent for current simple component scope."
    );
}

#[test]
fn component_files_check_script_covers_alert_dialog_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_component_directory_standard_files_follow_contract_and_no_spec";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn file_placement_discipline_is_strict_for_component_scope() {
    let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = include_str!("../mod.rs");
    let logic_source = include_str!("../logic.rs");
    let styles_source = include_str!("../styles.rs");
    let view_source = include_str!("../view.rs");
    let motion_source = include_str!("../motion.rs");
    let protocol_source = include_str!("../protocol.rs");

    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            crate_dir.join(required).exists(),
            "alert-dialog should keep required file-placement target `{required}`."
        );
    }

    for forbidden in ["src/render.rs", "src/spec.rs"] {
        assert!(
            !crate_dir.join(forbidden).exists(),
            "alert-dialog should not introduce forbidden file-placement target `{forbidden}`."
        );
    }

    assert!(
        crate_dir.join("src/protocol.rs").exists(),
        "alert-dialog keeps protocol.rs as schema/projection sidecar."
    );
    for needle in [
        "pub enum AlertDialogComponentSchemaVersion",
        "pub struct AlertDialogComponentSpec",
        "#[serde(default)]",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should keep schema-sidecar marker `{needle}`."
        );
    }

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::AlertDialog;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep export-boundary marker `{needle}`."
        );
    }
    for forbidden in [
        "mod render;",
        "pub mod render;",
        "mod spec;",
        "pub mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should reject forbidden split marker `{forbidden}`."
        );
    }

    for needle in [
        "resolve_state(",
        "compose_class_name(",
        "normalize_required_text(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep derivation marker `{needle}`."
        );
    }
    for forbidden in ["web_sys::", "view!"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include rendering/browser marker `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-", ".ui-alert-dialog"] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should keep token-style marker `{needle}`."
        );
    }
    for forbidden in ["role=\"alertdialog\"", "<Overlay"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid structure/semantics marker `{forbidden}`."
        );
    }

    for needle in ["view! {", "<Overlay", "locale_attrs(lang, dir)"] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep render/semantic-mount marker `{needle}`."
        );
    }
    assert!(
        !view_source.contains("render.rs"),
        "view.rs should not drift to render.rs split."
    );

    for needle in [
        "pub struct AlertDialogMotion",
        "pub fn sanitize_motion(motion: AlertDialogMotion) -> AlertDialogMotion",
        "crate::overlay::motion::sanitize_motion(motion.overlay)",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep contract-mapping marker `{needle}`."
        );
    }
}

#[test]
fn file_placement_check_script_covers_alert_dialog_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_file_placement_discipline_is_strict_for_component_scope";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = include_str!("../mod.rs");
    let logic_source = include_str!("../logic.rs");
    let view_source = include_str!("../view.rs");
    let protocol_source = include_str!("../protocol.rs");
    let check2_source = include_str!("../check2.md");

    assert!(
        !crate_dir.join("src/spec.rs").exists(),
        "alert-dialog is not a complex spec-first component; src/spec.rs should remain absent."
    );
    assert!(
        crate_dir.join("src/protocol.rs").exists(),
        "alert-dialog keeps schema contract in protocol.rs instead of introducing spec.rs."
    );

    for needle in [
        "pub enum AlertDialogComponentSchemaVersion",
        "pub struct AlertDialogComponentSpec",
        "#[serde(default)]",
    ] {
        assert!(
            protocol_source.contains(needle),
            "alert-dialog protocol should keep schema marker `{needle}`."
        );
    }

    let combined = format!("{mod_source}\n{logic_source}\n{view_source}");
    for forbidden in [
        "Spec::new()",
        ".render()",
        "pub struct AlertDialogSpec",
        "impl AlertDialogSpec",
    ] {
        assert!(
            !combined.contains(forbidden),
            "alert-dialog should not expose complex builder token `{forbidden}`."
        );
    }

    for needle in [
        "Hyper-Structure Builder（`spec.rs`）",
        "N/A：`AlertDialog` 作为标准 overlay 组合组件",
    ] {
        assert!(
            check2_source.contains(needle),
            "alert-dialog check2 should document hyper-structure applicability via `{needle}`."
        );
    }
}

#[test]
fn hyper_structure_builder_check_script_covers_alert_dialog_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_source = include_str!("../Component.toml");
    let rbi_source = include_str!("../alert_dialog.rbi");

    for required in ["src/Component.toml", "src/alert_dialog.rbi"] {
        assert!(
            crate_dir.join(required).exists(),
            "alert-dialog context-compression artifact should exist: `{required}`."
        );
    }

    for needle in [
        "schema_version = \"1\"",
        "name = \"AlertDialog\"",
        "crate = \"ui-alert-dialog\"",
        "name = \"open\"",
        "name = \"id_base\"",
        "name = \"title\"",
        "name = \"confirm_label\"",
        "name = \"cancel_label\"",
        "name = \"secondary_label\"",
        "name = \"is_confirm_disabled\"",
        "name = \"is_secondary_disabled\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"on_close\"",
        "name = \"on_confirm\"",
        "name = \"on_cancel\"",
        "name = \"on_secondary\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Component.toml should include context-compression marker `{needle}`."
        );
    }

    for needle in [
        "pub enum AlertDialogVariant",
        "pub enum AlertDialogAutoFocusButton",
        "pub struct AlertDialogMotion",
        "pub enum AlertDialogComponentSchemaVersion",
        "pub struct AlertDialogComponentSpec",
        "pub fn AlertDialog(",
        "is_confirm_disabled: Option<bool>",
        "confirm_disabled: Option<bool>",
        "is_secondary_disabled: Option<bool>",
        "secondary_disabled: Option<bool>",
        "lang: Option<String>",
        "dir: Option<ui_headless::a11y::A11yDirection>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "alert_dialog.rbi should include context-compression marker `{needle}`."
        );
    }
}

#[test]
fn context_compression_check_script_covers_alert_dialog_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_context_compression_manifest_and_rbi_projection_are_present_and_current";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn check2_documents_agent_contract_schema_governance_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for required in [
            "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
            "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
            "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
            "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
            "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
            "agent_contract_is_schema_typed_and_machine_readable",
            "agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
            "agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
            "scripts/check-ui-components-contract-hygiene.sh",
        ] {
            assert!(
                source.contains(required),
                "alert-dialog check2 should keep Agent Contract governance rule `{required}`."
            );
        }
    }
}

#[test]
fn agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = include_str!("../logic.rs");
    let view_source = include_str!("../view.rs");
    let manifest_source = include_str!("../Component.toml");
    let rbi_source = include_str!("../alert_dialog.rbi");

    for needle in [
        "pub const ALERT_DIALOG_AGENT_SCHEMA: &str = \"ui.alert-dialog.agent-contract\";",
        "pub enum AlertDialogAgentSchemaVersion",
        "pub enum AlertDialogAgentIntent",
        "pub enum AlertDialogAgentAction",
        "pub enum AlertDialogAgentState",
        "pub enum AlertDialogAgentSource",
        "pub enum AlertDialogAgentConfigPolicy",
        "pub enum AlertDialogAgentOutputStatus",
        "pub struct AlertDialogAgentCapabilities",
        "pub struct AlertDialogAgentContractInput",
        "pub struct AlertDialogAgentContract",
        "pub fn resolve_agent_contract(input: AlertDialogAgentContractInput) -> AlertDialogAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "alert-dialog logic should keep typed agent contract marker `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = Signal::derive(move || {",
        "logic::resolve_agent_contract(logic::AlertDialogAgentContractInput {",
        "data-ui-schema=move || ctx.agent_contract.get().schema_name",
        "data-ui-schema-version=move || ctx.agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || ctx.agent_contract.get().intent.as_str()",
        "data-ui-action=move || ctx.agent_contract.get().action.as_str()",
        "data-ui-state=move || ctx.agent_contract.get().state.as_str()",
        "data-ui-source=move || ctx.agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || ctx.agent_contract.get().config_policy.as_str()",
        "data-ui-output-status=move || ctx.agent_contract.get().output_status.as_str()",
        "data-ui-capability-description=move || {",
        "data-ui-capability-cancel=move || {",
        "data-ui-capability-secondary=move || {",
        "data-ui-capability-confirm=move || {",
        "data-ui-capability-dismiss=move || {",
        "data-ui-source-variant=move || ctx.agent_contract.get().variant_source",
        "data-ui-source-title=move || ctx.agent_contract.get().title_source",
        "data-ui-source-description=move || ctx.agent_contract.get().description_source",
        "data-ui-source-cancel=move || ctx.agent_contract.get().cancel_source",
        "data-ui-source-secondary=move || ctx.agent_contract.get().secondary_source",
        "data-ui-source-confirm=move || ctx.agent_contract.get().confirm_source",
        "data-ui-source-auto-focus=move || ctx.agent_contract.get().auto_focus_source",
        "data-ui-source-motion=move || ctx.agent_contract.get().motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "alert-dialog view should mount schemaized agent marker `{needle}`."
        );
    }

    for needle in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"agent_contract_whitelist_render_policy\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "alert-dialog Component.toml should include agent contract projection marker `{needle}`."
        );
    }

    for needle in [
        "pub const ALERT_DIALOG_AGENT_SCHEMA: &str;",
        "pub enum AlertDialogAgentSchemaVersion",
        "pub enum AlertDialogAgentIntent",
        "pub enum AlertDialogAgentAction",
        "pub enum AlertDialogAgentState",
        "pub enum AlertDialogAgentSource",
        "pub enum AlertDialogAgentConfigPolicy",
        "pub enum AlertDialogAgentOutputStatus",
        "pub struct AlertDialogAgentCapabilities",
        "pub struct AlertDialogAgentContractInput",
        "pub struct AlertDialogAgentContract",
        "pub fn resolve_agent_contract(input: AlertDialogAgentContractInput) -> AlertDialogAgentContract;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "alert-dialog RBI should project typed agent contract surface `{needle}`."
        );
    }
}

#[test]
fn agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = include_str!("../logic.rs");
    let view_source = include_str!("../view.rs");

    for typed_source in [
        "schema_name: ALERT_DIALOG_AGENT_SCHEMA,",
        "schema_version: AlertDialogAgentSchemaVersion::V1,",
        "intent: AlertDialogAgentIntent::ConfirmationDialog,",
        "AlertDialogAgentAction::ConfirmOnly",
        "AlertDialogAgentAction::ConfirmCancel",
        "AlertDialogAgentAction::ConfirmSecondary",
        "AlertDialogAgentAction::ConfirmCancelSecondary",
        "AlertDialogAgentState::Open",
        "AlertDialogAgentState::Closed",
        "AlertDialogAgentSource::Customized",
        "AlertDialogAgentSource::Default",
        "config_policy: AlertDialogAgentConfigPolicy::Whitelist,",
        "output_status: AlertDialogAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "alert-dialog agent fields should stay type-derived via `{typed_source}`."
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
            "alert-dialog agent contract should avoid free-form schema splicing `{forbidden}`."
        );
    }
}

#[test]
fn agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = include_str!("../view.rs");
    let logic_source = include_str!("../logic.rs");
    let styles_source = include_str!("../styles.rs");
    let mod_source = include_str!("../mod.rs");
    let motion_source = include_str!("../motion.rs");
    let protocol_source = include_str!("../protocol.rs");
    let combined = format!(
        "{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}\n{protocol_source}"
    );

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
            "alert-dialog render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = include_str!("../../../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn check2_marks_agent_contract_schema_governance_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
            "check2_documents_agent_contract_schema_governance_rules",
            "agent_contract_is_schema_typed_and_machine_readable",
            "agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
            "agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
            "contract_hygiene_script_covers_agent_contract_schema_guards",
            "scripts/check-ui-components-contract-hygiene.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 should keep Agent Contract governance marker `{needle}`."
            );
        }
    }
}

#[test]
fn check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");
    let view_source = include_str!("../view.rs");
    let logic_source = include_str!("../logic.rs");
    let mod_source = include_str!("../mod.rs");
    let motion_source = include_str!("../motion.rs");
    let script_source = include_str!("../../../../scripts/check-ui-components-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for source in [check2_src, check2_root] {
        for required in [
            "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
            "`Streaming`：LLM 还在生成，界面边生成边显示。",
            "`Snapshot`：LLM 全部生成完成后，一次性显示。",
            "N/A：`AlertDialog` 不是 LLM 正文渲染组件",
        ] {
            assert!(
                source.contains(required),
                "alert-dialog check2 should keep streaming-definition marker `{required}`."
            );
        }
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
            "alert-dialog runtime path should not embed LLM streaming protocol marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`."
    );
}

#[test]
fn streaming_script_covers_two_mode_definition_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-streaming.sh");

    let needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`."
    );
}

#[test]
fn check2_marks_streaming_two_mode_definition_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        assert!(
            source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
            "alert-dialog check2 should mark streaming two-mode definition gate complete."
        );

        for needle in [
            "check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
            "streaming_script_covers_two_mode_definition_contract",
            "scripts/check-ui-components-streaming.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 streaming section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn check2_documents_snapshot_as_default_baseline_capability() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
            "所有组件都应能消费“完整生成结果”并稳定渲染。",
            "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
            "N/A：`AlertDialog` 不直接渲染 LLM 正文",
            "check2_documents_snapshot_as_default_baseline_capability",
            "scripts/check-ui-components-streaming.sh",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 should keep snapshot-baseline marker `{needle}`."
            );
        }
    }
}

#[test]
fn snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = include_str!("../view.rs");
    let logic_source = include_str!("../logic.rs");
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");
    let script_source = include_str!("../../../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let title = logic::normalize_required_text(title, logic::DEFAULT_TITLE);",
        "let description = logic::normalize_optional_text(description);",
        "let confirm_label = logic::normalize_required_text(confirm_label, logic::DEFAULT_CONFIRM_LABEL);",
        "let cancel_label = logic::normalize_cancel_label(cancel_label);",
        "let secondary_label = logic::normalize_secondary_label(secondary_label);",
        "let confirm_disabled = logic::resolve_disabled_flag(",
        "let secondary_disabled = logic::resolve_disabled_flag(",
        "logic::resolve_state(AlertDialogPartStateInput {",
        "data-state=move || ctx.root_state.get().state_attr",
        "data-open=move || ctx.open.get().then_some(\"true\")",
        "data-closed=move || (!ctx.open.get()).then_some(\"true\")",
        "data-description=move || ctx.root_state.get().description_attr",
        "data-cancel=move || ctx.root_state.get().cancel_attr",
        "data-secondary=move || ctx.root_state.get().secondary_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "alert-dialog snapshot baseline should keep stable complete-result render marker `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn normalize_required_text(value: String, fallback: &'static str) -> String",
        "pub fn normalize_id_base(value: String) -> String",
        "pub fn normalize_cancel_label(value: Option<String>) -> String",
        "pub fn normalize_secondary_label(value: Option<String>) -> Option<String>",
        "pub fn resolve_disabled_flag(",
        "pub fn resolve_state(input: AlertDialogPartStateInput) -> AlertDialogPartState",
    ] {
        assert!(
            logic_source.contains(needle),
            "alert-dialog logic should keep snapshot-baseline normalization marker `{needle}`."
        );
    }

    for forbidden in [
        "streaming_chunk",
        "token_delta",
        "partial token",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-stream-mode",
        "data-stream-fallback",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "alert-dialog snapshot baseline should avoid incremental streaming marker `{forbidden}`."
        );
    }

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
            "snapshot_baseline_consumes_complete_result_and_renders_stably",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 snapshot section should reference `{needle}`."
            );
        }
    }

    let script_needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`."
    );
}

#[test]
fn streaming_script_covers_snapshot_baseline_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn check2_marks_snapshot_baseline_capability_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
            "check2_documents_snapshot_as_default_baseline_capability",
            "snapshot_baseline_consumes_complete_result_and_renders_stably",
            "streaming_script_covers_snapshot_baseline_contract",
            "scripts/check-ui-components-streaming.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 snapshot-baseline section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn check2_documents_streaming_required_optional_classification_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");
    let script_source = include_str!("../../../../scripts/check-ui-components-streaming.sh");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
            "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
            "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
            "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
            "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
            "`AlertDialog` 归类为 `Streaming Optional`",
            "fallback=snapshot",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 should keep streaming required/optional rule `{needle}`."
            );
        }
    }

    for script_needle in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`."
        );
    }
}

#[test]
fn streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = include_str!("../view.rs");
    let logic_source = include_str!("../logic.rs");
    let overlay_view_source = include_str!("../../../overlay/src/view.rs");

    for needle in [
        "<Overlay",
        "role=\"alertdialog\"",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
        "data-ui-state=move || ctx.agent_contract.get().state.as_str()",
        "data-ui-source=move || ctx.agent_contract.get().source.as_str()",
        "data-ui-output-status=move || ctx.agent_contract.get().output_status.as_str()",
        "data-output-status=move || ctx.agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "alert-dialog optional-streaming scope should keep semantic continuity marker `{needle}`."
        );
    }

    for needle in [
        "pub enum AlertDialogAgentOutputStatus",
        "AlertDialogAgentOutputStatus::Draft",
        "AlertDialogAgentOutputStatus::Verified",
        "AlertDialogAgentOutputStatus::CommitReady",
    ] {
        assert!(
            logic_source.contains(needle),
            "alert-dialog optional-streaming scope should expose explicit output-status domain marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || root_state.get().state_attr",
        "data-role=role",
        "role=role",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
    ] {
        assert!(
            overlay_view_source.contains(needle),
            "overlay should keep role/aria/data continuity marker `{needle}` for alert-dialog optional-streaming path."
        );
    }
}

#[test]
fn streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = include_str!("../view.rs");
    let logic_source = include_str!("../logic.rs");
    let mod_source = include_str!("../mod.rs");
    let motion_source = include_str!("../motion.rs");
    let protocol_source = include_str!("../protocol.rs");
    let combined =
        format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}\n{protocol_source}");

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
            "alert-dialog should keep validation/retry/resilience policy outside component layer; found `{forbidden}`."
        );
    }
}

#[test]
fn streaming_script_covers_required_optional_classification_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn check2_marks_streaming_required_optional_classification_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
            "check2_documents_streaming_required_optional_classification_rules",
            "streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
            "streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
            "streaming_script_covers_required_optional_classification_contract",
            "scripts/check-ui-components-streaming.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 required/optional classification section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = include_str!("../mod.rs");
    let logic_source = include_str!("../logic.rs");
    let styles_source = include_str!("../styles.rs");
    let view_source = include_str!("../view.rs");
    let motion_source = include_str!("../motion.rs");
    let protocol_source = include_str!("../protocol.rs");
    let combined = format!(
        "{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}\n{protocol_source}"
    );

    for forbidden in [".unwrap(", ".expect(", "let _ =", "_ = "] {
        assert!(
            !combined.contains(forbidden),
            "alert-dialog non-test source should forbid rust-hygiene anti-pattern `{forbidden}`."
        );
    }
}

#[test]
fn rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = include_str!("../logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(state.base_class)",
        "Cow::Borrowed(state.variant_class)",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(required),
            "alert-dialog logic should keep Cow-based string hotspot mitigation marker `{required}`."
        );
    }

    for forbidden in [
        "\"ui-alert-dialog--open\".to_string()",
        "\"ui-alert-dialog--closed\".to_string()",
        "\"ui-alert-dialog--with-description\".to_string()",
        "\"ui-alert-dialog--title-only\".to_string()",
        "\"ui-alert-dialog--cancel-shown\".to_string()",
        "\"ui-alert-dialog--cancel-hidden\".to_string()",
        "\"ui-alert-dialog--secondary-shown\".to_string()",
        "\"ui-alert-dialog--secondary-hidden\".to_string()",
        "\"ui-alert-dialog--confirm-disabled\".to_string()",
        "\"ui-alert-dialog--secondary-disabled\".to_string()",
        "\"ui-alert-dialog--with-type-icon\".to_string()",
        "\"ui-alert-dialog--custom-variant\".to_string()",
        "\"ui-alert-dialog--custom-id\".to_string()",
        "\"ui-alert-dialog--custom-title\".to_string()",
        "\"ui-alert-dialog--custom-description\".to_string()",
        "\"ui-alert-dialog--custom-cancel\".to_string()",
        "\"ui-alert-dialog--custom-secondary\".to_string()",
        "\"ui-alert-dialog--custom-confirm\".to_string()",
        "\"ui-alert-dialog--custom-auto-focus\".to_string()",
        "\"ui-alert-dialog--custom-motion\".to_string()",
        "\"ui-alert-dialog--custom-exit\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "alert-dialog logic should avoid string clone hotspot `{forbidden}`."
        );
    }
}

#[test]
fn rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = include_str!("../../../../scripts/check-rust-hygiene.sh");
    let engineering_script = include_str!("../../../../scripts/check-ui-components-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`."
        );
    }

    for needle in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn check2_marks_rust_hygiene_contract_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
            "rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
            "rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
            "rust_hygiene_script_enforces_repo_level_hygiene_guards",
            "./scripts/check-rust-hygiene.sh",
            "Cow<'static, str>",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 rust-hygiene section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn tree_shaking_feature_pruning_contract_is_registered_and_feature_gated() {
    let cargo_source = include_str!("../../../../crates/ui-components/Cargo.toml");
    let lib_source = include_str!("../../../../crates/ui-components/src/lib.rs");
    let css_source = include_str!("../../../../crates/ui-components/src/css.rs");

    assert!(
        cargo_source
            .contains("component-alert_dialog = [\"component-overlay\", \"component-button\"]"),
        "ui-components feature tree should register component-alert_dialog with required dependencies."
    );
    assert!(
        lib_source.contains("#[cfg(feature = \"component-alert_dialog\")]")
            && lib_source.contains("pub mod alert_dialog;"),
        "ui-components lib.rs should feature-gate alert-dialog module export."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-alert_dialog\")]")
            && css_source.contains("out.push_str(crate::alert_dialog::styles::CSS);"),
        "ui-components css.rs should gate alert-dialog CSS aggregation behind component-alert_dialog."
    );
}

#[test]
fn tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let script_source = include_str!("../../../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = include_str!("../../../../scripts/tree_shaking_budget.env");

    for needle in [
        "ALERT_DIALOG_MIN_FEATURES=\"component-alert_dialog,inject-css\"",
        "echo \"[tree-shaking] alert-dialog feature registration + gated aggregation contract\"",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_tree_shaking_keeps_component_feature_and_css_boundaries",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_tree_shaking_check_script_covers_feature_tree_wasm_and_budget",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "ALERT_DIALOG_TREE_OUTPUT=\"$(cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$ALERT_DIALOG_MIN_FEATURES\")\"",
        "feature \"component-alert_dialog\" (command-line)",
        "feature \"inject-css\" (command-line)",
        "alert-dialog minimal feature tree should not pull all-components",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$ALERT_DIALOG_MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should enforce `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for needle in [
            "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
            "tree_shaking_feature_pruning_contract_is_registered_and_feature_gated",
            "tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
            "alert_dialog_tree_shaking_keeps_component_feature_and_css_boundaries",
            "alert_dialog_tree_shaking_check_script_covers_feature_tree_wasm_and_budget",
            "alert_dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
            "alert_dialog_check2_marks_tree_shaking_feature_pruning_contract_complete",
            "component-alert_dialog = [\"component-overlay\", \"component-button\"]",
            "bash ./scripts/check-ui-components-tree-shaking.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(needle),
                "alert-dialog check2 tree-shaking feature-pruning section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement() {
    let view_source = include_str!("../view.rs");
    let overlay_view_source = include_str!("../../../overlay/src/view.rs");
    let focus_trap_source = include_str!("../../../../crates/ui-headless/src/focus_trap.rs");
    let e2e_source = include_str!("../../../../e2e/tests/docs_app_alert_dialog_contract.spec.mjs");
    let todo_source = include_str!("../../../../docs/plan/TODO.md");
    let local_semantics = include_str!("semantics.rs");

    for marker in [
        "role=\"alertdialog\"",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
        "data-state=move || ctx.root_state.get().state_attr",
        "data-open=move || ctx.open.get().then_some(\"true\")",
        "data-id-source=move || ctx.root_state.get().id_source_attr",
        "data-title-source=move || ctx.root_state.get().title_source_attr",
        "data-description-source=move || ctx.root_state.get().description_source_attr",
        "data-cancel-source=move || ctx.root_state.get().cancel_source_attr",
        "data-secondary-source=move || ctx.root_state.get().secondary_source_attr",
        "data-confirm-source=move || ctx.root_state.get().confirm_source_attr",
        "data-motion-source=move || ctx.root_state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "alert-dialog view should expose aria/data semantic marker `{marker}`."
        );
    }

    for marker in [
        "on:keydown=on_key_down",
        "role=role",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
    ] {
        assert!(
            overlay_view_source.contains(marker),
            "overlay should preserve keyboard/aria mapping marker `{marker}`."
        );
    }

    for marker in [
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
        "restore_focus_chain(",
    ] {
        assert!(
            focus_trap_source.contains(marker),
            "focus stack contract should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"overlay-panel\"",
        "role=\"alertdialog\"",
        "data-state",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-cancel-source",
        "data-secondary-source",
        "data-motion-source",
        "keyboard.press(\"Escape\")",
        ".click()",
    ] {
        assert!(
            e2e_source.contains(marker),
            "alert-dialog e2e flow should include semantic/focus regression marker `{marker}`."
        );
    }

    let snapshot_macro = ["assert", "_snapshot!"].concat();
    let insta_snapshot = ["insta::assert", "_snapshot"].concat();
    assert!(
        !local_semantics.contains(&snapshot_macro) && !local_semantics.contains(&insta_snapshot),
        "semantic/performance contract should not degrade to snapshot-only checks.",
    );

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count governance follow-up should include `{marker}`."
        );
    }
}

#[test]
fn semantics_and_performance_script_covers_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-performance.sh");

    for marker in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`."
        );
    }
}

#[test]
fn check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for marker in [
            "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
            "semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
            "alert_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
            "performance_governance_budget_is_defined_and_blocking",
            "alert_dialog_performance_governance_budget_is_defined_and_blocking",
            "`render_count` 自动化回归仍在仓库统一 follow-up",
            "scripts/check-ui-components-performance.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(marker),
                "alert-dialog check2 semantic/performance section should include `{marker}`."
            );
        }
    }
}

#[test]
fn semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks() {
    let view_source = include_str!("../view.rs");
    let overlay_view_source = include_str!("../../../overlay/src/view.rs");
    let local_semantics_source = include_str!("semantics.rs");
    let workspace_semantics_source =
        include_str!("../../../../components/alert-dialog/src/test/alert_dialog_semantics.rs");
    let perf_script_source = include_str!("../../../../scripts/check-ui-components-performance.sh");

    for marker in [
        "data-state=move || ctx.root_state.get().state_attr",
        "data-open=move || ctx.open.get().then_some(\"true\")",
        "data-id-source=move || ctx.root_state.get().id_source_attr",
        "data-title-source=move || ctx.root_state.get().title_source_attr",
        "data-description-source=move || ctx.root_state.get().description_source_attr",
        "data-cancel-source=move || ctx.root_state.get().cancel_source_attr",
        "data-secondary-source=move || ctx.root_state.get().secondary_source_attr",
        "data-confirm-source=move || ctx.root_state.get().confirm_source_attr",
        "data-motion-source=move || ctx.root_state.get().motion_source_attr",
        "role=\"alertdialog\"",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
    ] {
        assert!(
            view_source.contains(marker),
            "alert-dialog semantic-test-priority contract should keep marker `{marker}`."
        );
    }

    for marker in [
        "role=role",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "on:keydown=on_key_down",
    ] {
        assert!(
            overlay_view_source.contains(marker),
            "overlay semantic contract should keep marker `{marker}` for alert-dialog semantic priority."
        );
    }

    for marker in [
        "fn semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
        "forbidden in [",
        "\"assert_snapshot\"",
        "\"insta::\"",
        "\"snapshot!\"",
    ] {
        assert!(
            local_semantics_source.contains(marker),
            "alert-dialog local semantics suite should keep semantic-priority marker `{marker}`."
        );
    }

    for marker in [
        "fn alert_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn alert_dialog_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
    ] {
        assert!(
            workspace_semantics_source.contains(marker),
            "workspace alert-dialog semantics suite should keep semantic-priority marker `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include alert-dialog semantic-priority gate `{script_needle}`."
    );
}

#[test]
fn performance_script_covers_semantic_test_priority_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-performance.sh");

    for marker in [
        "echo \"[perf] contract: alert-dialog semantic test priority\"",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should include alert-dialog semantic-priority marker `{marker}`."
        );
    }
}

#[test]
fn check2_marks_semantic_test_priority_item_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        assert!(
            source.contains(
                "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
            ),
            "alert-dialog check2 should mark semantic-test-priority item complete."
        );

        for marker in [
            "components/alert-dialog/src/test/semantics.rs::semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
            "components/alert-dialog/src/test/semantics.rs::semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
            "components/alert-dialog/src/test/alert_dialog_semantics.rs::alert_dialog_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
            "scripts/check-ui-components-performance.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(marker),
                "alert-dialog check2 semantic-test-priority section should include `{marker}`."
            );
        }
    }
}

#[test]
fn check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for marker in [
            "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
            "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
            "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
            "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        ] {
            assert!(
                source.contains(marker),
                "alert-dialog check2 should keep e2e selector/stable-wait rule `{marker}`."
            );
        }
    }
}

#[test]
fn e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = include_str!("../../../../e2e/tests/docs_app_alert_dialog_contract.spec.mjs");
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );

    for marker in [
        "const WASM_READY_SELECTOR = \"body:not(:has(#boot))\";",
        "await waitForWasmReady(page);",
        "data-slot=\"alert-dialog-e2e-open-marker\"",
        "data-slot=\"alert-dialog-e2e-open-destructive\"",
        "[data-slot=\"overlay-panel\"][role=\"alertdialog\"][aria-labelledby=\"docs-alert-marker-title\"]",
        "[data-slot=\"overlay-panel\"][role=\"alertdialog\"][aria-labelledby=\"docs-alert-title\"]",
        "toHaveAttribute(\"data-id-source\", \"custom\")",
        "toHaveAttribute(\"data-title-source\", \"custom\")",
        "toHaveAttribute(\"data-description-source\", \"custom\")",
        "toHaveAttribute(\"data-cancel-source\", \"custom\")",
        "toHaveAttribute(\"data-secondary-source\", \"custom\")",
        "toHaveAttribute(\"data-secondary-disabled\", \"true\")",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "toHaveAttribute(\"data-auto-focus\", \"secondary\")",
    ] {
        assert!(
            e2e_source.contains(marker),
            "alert-dialog e2e selector contract should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"alert-dialog-e2e-hello-controls\"",
        "data-slot=\"alert-dialog-e2e-open-destructive\"",
        "data-slot=\"alert-dialog-e2e-marker-controls\"",
        "data-slot=\"alert-dialog-e2e-open-marker\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "alert-dialog docs should expose stable e2e selector marker `{marker}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "alert-dialog e2e selector contract should avoid brittle selector/wait token `{forbidden}`."
        );
    }
}

#[test]
fn e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths() {
    let e2e_source = include_str!("../../../../e2e/tests/docs_app_alert_dialog_contract.spec.mjs");

    for marker in [
        "function overlayForPanel(page, overlayPanel)",
        "async function expectAlertDialogReady(page, overlayPanel, alertDialogRoot)",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"aria-modal\", \"true\")",
        "toHaveAttribute(\"data-output-status\", \"verified\")",
        "async function expectAlertDialogSettledClosed(overlayPanel, alertDialogRoot, overlayRoot)",
        "await expect(overlayPanel).toHaveCount(0);",
        "await expect(alertDialogRoot).toHaveCount(0);",
        "await expect(overlayRoot).toHaveCount(0);",
        "await overlayPanel.press(\"Escape\");",
        "await confirmButton.click();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "alert-dialog e2e ready/settled contract should include `{marker}`."
        );
    }
}

#[test]
fn e2e_check_script_covers_selector_and_ready_settled_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-e2e-alert-dialog.sh");

    for marker in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
    ] {
        assert!(
            script_source.contains(marker),
            "alert-dialog e2e script should include `{marker}`."
        );
    }
}

#[test]
fn check2_marks_e2e_selector_stability_item_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        assert!(
            source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
            "alert-dialog check2 should mark e2e selector stability item complete."
        );

        for marker in [
            "e2e/tests/docs_app_alert_dialog_contract.spec.mjs",
            "body:not(:has(#boot))",
            "data-slot=\"alert-dialog-e2e-open-marker\"",
            "data-slot=\"alert-dialog-e2e-open-destructive\"",
            "ready/settled",
            "components/alert-dialog/src/test/semantics.rs::e2e_selector_contract_uses_semantic_markers_and_settled_waits",
            "components/alert-dialog/src/test/alert_dialog_semantics.rs::alert_dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
            "scripts/check-ui-components-e2e-alert-dialog.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(marker),
                "alert-dialog check2 e2e selector section should include `{marker}`."
            );
        }
    }
}

#[test]
fn check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for marker in [
            "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
            "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
            "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
            "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
        ] {
            assert!(
                source.contains(marker),
                "alert-dialog check2 should keep repeatable e2e key-flow rule `{marker}`."
            );
        }
    }
}

#[test]
fn e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = include_str!("../../../../e2e/tests/docs_app_alert_dialog_contract.spec.mjs");

    for marker in [
        "docs-app alert-dialog key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "data-slot=\"alert-dialog-e2e-open-marker\"",
        "await page.keyboard.press(\"Enter\");",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-secondary-disabled\", \"true\")",
        "toHaveAttribute(\"data-output-status\", \"verified\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "await confirmButton.focus();",
        "await expect(confirmButton).toBeFocused();",
        "await expectAlertDialogSettledClosed(overlayPanel, alertDialog, overlayRoot);",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "alert-dialog repeatable e2e key-flow contract should include `{marker}`."
        );
    }
}

#[test]
fn e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = include_str!("../../../../e2e/tests/docs_app_alert_dialog_contract.spec.mjs");

    for marker in [
        "docs-app alert-dialog high-risk paths cover overlay focus keyboard and settled semantic breakpoints",
        "data-slot=\"alert-dialog-e2e-open-destructive\"",
        "role=\"alertdialog\"",
        "await expect(overlayPanel.locator('[aria-busy=\"true\"]')).toHaveCount(0);",
        "await expect(overlayPanel.locator('[data-loading=\"true\"]')).toHaveCount(0);",
        "await expect(overlayPanel.locator('[data-state=\"loading\"]')).toHaveCount(0);",
        "const actionButtons = overlayPanel.locator('[data-slot=\"alert-dialog\"] [data-slot=\"button\"]');",
        "await expect(actionButtons).toHaveCount(2);",
        "await page.keyboard.press(\"Shift+Tab\");",
        "await expect(cancelButton).toBeFocused();",
        "await page.keyboard.press(\"Tab\");",
        "await expect(confirmButton).toBeFocused();",
        "await overlayPanel.press(\"Escape\");",
        "await expectAlertDialogSettledClosed(overlayPanel, alertDialog, overlayRoot);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "alert-dialog high-risk e2e contract should include `{marker}`."
        );
    }
}

#[test]
fn e2e_check_script_covers_repeatable_key_flow_contracts() {
    let script_source = include_str!("../../../../scripts/check-ui-components-e2e-alert-dialog.sh");

    for marker in [
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(marker),
            "alert-dialog e2e script should include repeatable key-flow marker `{marker}`."
        );
    }
}

#[test]
fn check2_marks_e2e_repeatable_key_flow_item_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        assert!(
            source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
            "alert-dialog check2 should mark repeatable e2e key-flow item complete."
        );

        for marker in [
            "e2e/tests/docs_app_alert_dialog_contract.spec.mjs",
            "for (const cycle of [1, 2])",
            "overlay/focus/keyboard",
            "async",
            "N/A",
            "components/alert-dialog/src/test/semantics.rs::e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
            "components/alert-dialog/src/test/semantics.rs::e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
            "components/alert-dialog/src/test/alert_dialog_semantics.rs::alert_dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
            "scripts/check-ui-components-e2e-alert-dialog.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(marker),
                "alert-dialog check2 repeatable e2e key-flow section should include `{marker}`."
            );
        }
    }
}

#[test]
fn check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for marker in [
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
            "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
            "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
            "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        ] {
            assert!(
                source.contains(marker),
                "alert-dialog check2 heroui-benchmark docs-sync section should include `{marker}`.",
            );
        }
    }
}

#[test]
fn heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = include_str!("../../../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = include_str!("../../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = include_str!(
        "../../../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs"
    );
    let readme_source = include_str!("../README.md");

    for marker in [
        "### AlertDialog 同步记录（2026-02-20）",
        "参数模型同步：`AlertDialog` 参数主轴保持 `open + on_close`",
        "component_doc!(\"AlertDialog\", \"alert-dialog\", \"Overlays\", overlays::alert_dialog)",
        "`apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs::alert_dialog()`",
        "`components/alert-dialog/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(marker),
            "heroui strategy doc should include alert-dialog synchronization marker `{marker}`.",
        );
    }

    for marker in [
        "component_doc!(",
        "\"AlertDialog\"",
        "\"alert-dialog\"",
        "overlays::alert_dialog",
    ] {
        assert!(
            pages_source.contains(marker),
            "component docs index should expose alert-dialog entry marker `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn alert_dialog() -> AnyView {",
        "title=\"AlertDialog\"",
        "slug=\"alert-dialog\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "docs-app alert-dialog page should stay indexable via marker `{marker}`.",
        );
    }

    for marker in ["# AlertDialog", "## Hello World（最小可用）"] {
        assert!(
            readme_source.contains(marker),
            "alert-dialog README should remain an equivalent component doc entry via `{marker}`.",
        );
    }
}

#[test]
fn dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = include_str!("../../../../scripts/check-ui-components-dx.sh");

    for marker in [
        "echo \"[dx] contract: alert-dialog heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should enforce heroui-benchmark docs-sync contract `{marker}`.",
        );
    }
}

#[test]
fn check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_src = include_str!("../check2.md");
    let check2_root = include_str!("../../check2.md");

    for source in [check2_src, check2_root] {
        for marker in [
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
            "check2_documents_heroui_benchmark_docs_sync_rules",
            "heroui_strategy_and_component_docs_are_synchronized_and_indexable",
            "dx_check_script_covers_heroui_benchmark_docs_sync_contract",
            "docs/spec/heroui-parameter-design-strategy.md",
            "scripts/check-ui-components-dx.sh",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                source.contains(marker),
                "alert-dialog check2 heroui-benchmark docs-sync section should include `{marker}`.",
            );
        }
    }
}
