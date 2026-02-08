use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_swatch_picker_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color_swatch_picker/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorSwatchPicker internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_uses_logic_state_model() {
    let logic_source = load_source("src/color_swatch_picker/logic.rs");
    let view_source = load_source("src/color_swatch_picker/view.rs");

    for needle in [
        "pub struct ColorSwatchPickerItem",
        "pub fn normalize_items(",
        "pub fn resolve_selected_index(",
        "pub fn resolve_selected_color(",
        "pub fn resolve_option_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorSwatchPicker logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "use_radio_group(RadioGroupOptions {",
        "logic::resolve_selected_index(&items.get(), selected_state.value.get())",
        "logic::resolve_state(ColorSwatchPickerStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSwatchPicker view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_swatch_picker_exposes_spectrum_a11y_and_state_markers() {
    let source = load_source("src/color_swatch_picker/view.rs");

    for attr in [
        "role=aria.attrs.role",
        "aria-label=aria_label",
        "data-slot=\"color-swatch-picker\"",
        "data-slot=\"color-swatch-picker-list\"",
        "data-slot=\"color-swatch-picker-option\"",
        "data-state=move || state.get().data_state_attr",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-disabled-item-count=move || state.get().disabled_item_count.to_string()",
        "aria-checked=move || if is_selected() { \"true\" } else { \"false\" }",
    ] {
        assert!(
            source.contains(attr),
            "ColorSwatchPicker should expose `{attr}` for Spectrum-style semantics."
        );
    }
}

#[test]
fn color_swatch_picker_styles_include_selected_focus_and_disabled_contracts() {
    let source = load_source("src/color_swatch_picker/styles.rs");

    for selector in [
        ".ui-color-swatch-picker",
        ".ui-color-swatch-picker__list",
        ".ui-color-swatch-picker__option",
        ".ui-color-swatch-picker__option[data-selected=\"true\"]",
        ".ui-color-swatch-picker__option:focus-visible",
        ".ui-color-swatch-picker__option[data-disabled=\"true\"]",
        ".ui-color-swatch-picker--disabled .ui-color-swatch-picker__option",
        ".ui-color-swatch-picker[data-empty=\"true\"] .ui-color-swatch-picker__list",
        ".ui-color-swatch-picker--custom-class",
        ".ui-color-swatch-picker[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorSwatchPicker styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
