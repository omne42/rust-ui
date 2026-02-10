use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_picker_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color_picker/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorPicker internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_picker_uses_logic_state_model() {
    let logic_source = load_source("src/color_picker/logic.rs");
    let view_source = load_source("src/color_picker/view.rs");

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub const DEFAULT_ARIA_LABEL",
        "pub struct ColorPickerIds",
        "pub fn sanitize_selected_color(",
        "pub fn resolve_ids(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorPicker logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "overlay_open::use_controllable_open_state(",
        "use_presence(open)",
        "logic::resolve_state(ColorPickerStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<Popover",
        "motion=motion.popover",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorPicker view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_picker_exposes_spectrum_style_data_markers() {
    let source = load_source("src/color_picker/view.rs");

    for attr in [
        "data-slot=\"color-picker\"",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-motion-source=if motion == ColorPickerMotion::default()",
        "data-custom-motion=move || (motion != ColorPickerMotion::default()).then_some(\"true\")",
        "data-slot=\"color-picker-trigger\"",
        "data-slot=\"color-picker-swatch\"",
        "data-slot=\"color-picker-label\"",
        "data-slot=\"color-picker-value\"",
        "data-slot=\"color-picker-panel\"",
        "data-slot=\"color-picker-content\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorPicker should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn color_picker_styles_include_open_disabled_and_custom_contracts() {
    let source = load_source("src/color_picker/styles.rs");

    for selector in [
        ".ui-color-picker",
        ".ui-color-picker__trigger",
        ".ui-color-picker__panel",
        ".ui-color-picker__content",
        ".ui-color-picker--open .ui-color-picker__trigger",
        ".ui-color-picker[data-open=\"true\"] .ui-color-picker__trigger",
        ".ui-color-picker--disabled",
        ".ui-color-picker[data-disabled=\"true\"]",
        ".ui-color-picker--custom-class",
        ".ui-color-picker[data-motion-source=\"custom\"]",
        ".ui-color-picker[data-custom-motion=\"true\"]",
        ".ui-color-picker[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorPicker styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_picker_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/color_picker/mod.rs");
    let motion_source = load_source("src/color_picker/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::ColorPickerMotion;",
        "pub struct ColorPickerMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "ColorPicker motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}
