use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_wheel_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color_wheel/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorWheel internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_wheel_uses_logic_state_model() {
    let logic_source = load_source("src/color_wheel/logic.rs");
    let view_source = load_source("src/color_wheel/view.rs");

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn sanitize_value(",
        "pub fn move_value_by_delta(",
        "pub fn pointer_to_hue_angle(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorWheel logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "slider_motion::attach_motion(root_ref, visual_percent, motion)",
        "logic::hue_from_pointer_event(&track, &ev)",
        "logic::resolve_state(ColorWheelStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorWheel view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_wheel_exposes_spectrum_style_data_markers() {
    let source = load_source("src/color_wheel/view.rs");

    for attr in [
        "data-slot=\"color-wheel\"",
        "data-state=move || state.get().data_state_attr",
        "data-value=move || state.get().value.to_string()",
        "data-value-percent=move || state.get().value_percent.to_string()",
        "data-slot=\"color-wheel-track\"",
        "data-slot=\"color-wheel-ring\"",
        "data-slot=\"color-wheel-orbit\"",
        "data-slot=\"color-wheel-thumb\"",
        "data-slot=\"color-wheel-input\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorWheel should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn color_wheel_styles_include_ring_orbit_and_state_contracts() {
    let source = load_source("src/color_wheel/styles.rs");

    for selector in [
        ".ui-color-wheel",
        ".ui-color-wheel__track",
        ".ui-color-wheel__ring",
        ".ui-color-wheel__orbit",
        ".ui-color-wheel__thumb",
        ".ui-color-wheel__input",
        ".ui-color-wheel--disabled",
        ".ui-color-wheel[data-disabled=\"true\"]",
        ".ui-color-wheel--motion-custom",
        ".ui-color-wheel[data-motion-source=\"custom\"]",
        ".ui-color-wheel--custom-class",
        ".ui-color-wheel[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorWheel styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
