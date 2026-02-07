use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn chip_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/chip/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Chip internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn chip_uses_logic_state_model() {
    let view_source = load_source("src/chip/view.rs");
    let logic_source = load_source("src/chip/logic.rs");

    for needle in [
        "pub struct ChipStateInput",
        "pub struct ChipState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_dismiss_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Chip logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_dismiss_aria_label(dismiss_aria_label)",
        "logic::resolve_state(ChipStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Chip view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn chip_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/chip/view.rs");

    for attr in [
        "data-slot=\"chip\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=if state.is_disabled",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-removable=state.has_dismiss_action.then_some(\"true\")",
        "data-static=state.is_static.then_some(\"true\")",
        "data-dismiss-label=dismiss_label_source",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-slot=\"chip-content\"",
        "data-slot=\"chip-dismiss\"",
        "data-label-source=dismiss_label_source",
    ] {
        assert!(
            source.contains(attr),
            "Chip should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn chip_styles_include_variant_size_and_state_markers() {
    let source = load_source("src/chip/styles.rs");

    for selector in [
        ".ui-chip--size-sm",
        ".ui-chip[data-size=\"md\"]",
        ".ui-chip--variant-danger",
        ".ui-chip[data-variant=\"outline\"]",
        ".ui-chip--disabled",
        ".ui-chip[data-static=\"true\"]",
        ".ui-chip[data-removable=\"true\"]",
        ".ui-chip__dismiss[data-disabled=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Chip styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
