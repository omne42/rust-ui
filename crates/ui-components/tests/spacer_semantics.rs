use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn spacer_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/spacer/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Spacer internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn spacer_uses_logic_state_model() {
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");

    for needle in [
        "pub struct SpacerStateInput",
        "pub struct SpacerState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spacer logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(SpacerStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn spacer_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/spacer/view.rs");

    for attr in [
        "data-slot=\"spacer\"",
        "data-axis=state.axis_attr",
        "data-size=state.size_attr",
        "data-state=state.axis_attr",
        "data-vertical=state.is_vertical.then_some(\"true\")",
        "data-horizontal=state.is_horizontal.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            source.contains(attr),
            "Spacer should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn spacer_styles_include_axis_and_size_state_markers() {
    let source = load_source("src/spacer/styles.rs");

    for selector in [
        ".ui-spacer--size-xs",
        ".ui-spacer[data-size=\"md\"]",
        ".ui-spacer--size-xl",
        ".ui-spacer--axis-vertical",
        ".ui-spacer[data-axis=\"horizontal\"]",
        ".ui-spacer[data-state=\"vertical\"]",
        ".ui-spacer[data-horizontal=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Spacer styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
