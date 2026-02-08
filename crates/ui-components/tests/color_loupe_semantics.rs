use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_loupe_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color_loupe/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorLoupe internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_loupe_uses_logic_state_model() {
    let logic_source = load_source("src/color_loupe/logic.rs");
    let view_source = load_source("src/color_loupe/view.rs");

    for needle in [
        "pub const DEFAULT_COLOR",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn sanitize_percent(",
        "pub fn sanitize_color(",
        "pub fn normalize_aria_label(",
        "pub fn position_bucket(",
        "pub fn vertical_bucket(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorLoupe logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::resolve_state(ColorLoupeStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<ColorSwatch",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorLoupe view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_loupe_exposes_spectrum_style_data_markers() {
    let source = load_source("src/color_loupe/view.rs");

    for attr in [
        "data-slot=\"color-loupe\"",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-x-bucket=move || state.get().x_bucket_attr",
        "data-y-bucket=move || state.get().y_bucket_attr",
        "data-slot=\"color-loupe-bubble\"",
        "data-slot=\"color-loupe-checker\"",
        "data-slot=\"color-loupe-fill\"",
        "data-slot=\"color-loupe-tail\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorLoupe should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn color_loupe_styles_include_open_disabled_position_and_custom_contracts() {
    let source = load_source("src/color_loupe/styles.rs");

    for selector in [
        ".ui-color-loupe",
        ".ui-color-loupe__bubble",
        ".ui-color-loupe__fill",
        ".ui-color-loupe__tail",
        ".ui-color-loupe--x-start",
        ".ui-color-loupe--x-center",
        ".ui-color-loupe--x-end",
        ".ui-color-loupe--y-start",
        ".ui-color-loupe--y-center",
        ".ui-color-loupe--y-end",
        ".ui-color-loupe--open",
        ".ui-color-loupe[data-open=\"true\"]",
        ".ui-color-loupe[data-state=\"open\"]",
        ".ui-color-loupe--disabled",
        ".ui-color-loupe[data-disabled=\"true\"]",
        ".ui-color-loupe--custom-class",
        ".ui-color-loupe[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorLoupe styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
