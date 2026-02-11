use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_button_does_not_expose_logic_module() {
    let source = load_source("src/action_button/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "ActionButton's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
}

#[test]
fn action_button_uses_logic_state_model() {
    let view_source = load_source("src/action_button/view.rs");
    let logic_source = load_source("src/action_button/logic.rs");

    for needle in [
        "pub struct ActionButtonStateInput",
        "pub struct ActionButtonState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn as_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let class_name = logic::normalize_optional_text(class_name);",
        "let aria_label = logic::normalize_optional_text(aria_label);",
        "let state = logic::resolve_state(logic::ActionButtonStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn action_button_uses_headless_press_hover_and_focus_ring() {
    let source = load_source("src/action_button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "ActionButton should use headless `{needle}` hooks to align behavior with global focus-visible/modality providers."
        );
    }
}

#[test]
fn action_button_emits_spectrum_style_data_attributes() {
    let source = load_source("src/action_button/view.rs");

    for attr in [
        "data-slot=\"action-button\"",
        "data-state=if state.is_loading",
        "data-size=state.size_attr",
        "data-hovered",
        "data-pressed",
        "data-loading=state.is_loading.then_some(\"true\")",
        "data-loading-placement=state.loading_placement_attr",
        "data-quiet=state.is_quiet.then_some(\"true\")",
        "data-icon-only=state.is_icon_only.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-has-start=state.has_start_content.then_some(\"true\")",
        "data-has-end=state.has_end_content.then_some(\"true\")",
        "data-has-handler=state.has_custom_press_handler.then_some(\"true\")",
        "data-motion-source=if motion == ActionButtonMotion::default()",
        "data-custom-motion=(motion != ActionButtonMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "ActionButton should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn action_button_forwards_headless_button_semantics() {
    let source = load_source("src/action_button/view.rs");

    for attr in [
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "ActionButton should forward headless attrs via `{attr}` for correct custom-element semantics."
        );
    }
}

#[test]
fn action_button_loading_forces_disabled_and_sets_aria_busy() {
    let source = load_source("src/action_button/view.rs");

    for needle in [
        "disabled=state.is_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton should wire loading/disabled semantics via `{needle}`."
        );
    }
}

#[test]
fn action_button_has_spring_driven_scale_css_variable() {
    let styles = load_source("src/action_button/styles.rs");
    let motion = load_source("src/action_button/motion.rs");

    for needle in [
        "--ui-action-button-scale",
        "transform: scale(var(--ui-action-button-scale",
    ] {
        assert!(
            styles.contains(needle),
            "ActionButton styles should reference `{needle}` for spring-driven interaction scaling."
        );
    }

    assert!(
        motion.contains("--ui-action-button-scale"),
        "ActionButton motion should write `--ui-action-button-scale` to drive interaction feedback without triggering rerenders."
    );
}

#[test]
fn action_button_styles_include_motion_marker_contracts() {
    let source = load_source("src/action_button/styles.rs");

    for selector in [
        ".ui-action-button[data-motion-source=\"custom\"]",
        ".ui-action-button[data-custom-motion=\"true\"]",
        ".ui-action-button--focus-visible",
    ] {
        assert!(
            source.contains(selector),
            "ActionButton styles should include `{selector}` as stable state/motion contracts."
        );
    }
}

#[test]
fn action_button_spinner_respects_reduced_motion() {
    let styles = load_source("src/action_button/styles.rs");

    for needle in ["@media (prefers-reduced-motion: reduce)", "animation: none"] {
        assert!(
            styles.contains(needle),
            "ActionButton spinner should disable its CSS animation under reduced-motion via `{needle}`."
        );
    }
}

#[test]
fn action_button_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/action_button/motion.rs");

    for needle in [
        "pub struct ActionButtonMotion",
        "fn default_motion_matches_action_button_spring_contract()",
        "fn supports_custom_motion_contract_values()",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton motion module should include `{needle}` for HeroUI-level motion contract coverage."
        );
    }
}

#[test]
fn action_button_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/action_button/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ActionButtonMotion) -> ActionButtonMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "hover_scale:",
        "tap_scale:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}
