use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_area_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color_area/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorArea internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_area_uses_logic_state_model() {
    let logic_source = load_source("src/color_area/logic.rs");
    let view_source = load_source("src/color_area/view.rs");

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn sanitize_step(",
        "pub fn sanitize_grid_size(",
        "pub fn clamp_value(",
        "pub fn sanitize_preview_color(",
        "pub fn value_from_cell(",
        "pub fn move_value_by_delta(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorArea logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "logic::resolve_state(ColorAreaStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "logic::value_from_cell(col, row, state.grid_size)",
        "logic::move_value_by_delta(",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorArea view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_area_exposes_spectrum_style_data_markers() {
    let source = load_source("src/color_area/view.rs");

    for attr in [
        "data-slot=\"color-area\"",
        "data-state=move || state.get().data_state_attr",
        "data-grid-size=move || state.get().grid_size.to_string()",
        "data-value-x=move || state.get().value_x_percent.to_string()",
        "data-value-y=move || state.get().value_y_percent.to_string()",
        "data-selected-col=move || state.get().selected_col.to_string()",
        "data-selected-row=move || state.get().selected_row.to_string()",
        "data-slot=\"color-area-grid\"",
        "data-slot=\"color-area-cell\"",
        "data-slot=\"color-area-thumb\"",
        "data-slot=\"color-area-axis-x\"",
        "data-slot=\"color-area-axis-y\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorArea should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn color_area_styles_include_grid_selection_and_disabled_contracts() {
    let source = load_source("src/color_area/styles.rs");

    for selector in [
        ".ui-color-area",
        ".ui-color-area__grid",
        ".ui-color-area__cell",
        ".ui-color-area__cell[data-selected=\"true\"] .ui-color-area__thumb",
        ".ui-color-area__axes",
        ".ui-color-area--with-preview",
        ".ui-color-area[data-has-preview=\"true\"]",
        ".ui-color-area--disabled",
        ".ui-color-area[data-disabled=\"true\"]",
        ".ui-color-area--custom-class",
        ".ui-color-area[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorArea styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
