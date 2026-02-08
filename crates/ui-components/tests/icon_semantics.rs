use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icon_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/icon/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Icon internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn icon_uses_logic_state_model() {
    let logic_source = load_source("src/icon/logic.rs");
    let view_source = load_source("src/icon/view.rs");

    for needle in [
        "pub enum IconSize",
        "pub enum IconTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Icon logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(IconStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Icon view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn icon_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/icon/view.rs");

    for attr in [
        "data-slot=\"icon\"",
        "data-slot=\"icon-glyph\"",
        "data-size=state.size_attr",
        "data-tone=state.tone_attr",
        "data-state=state.data_state_attr",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-decorative=state.is_decorative.then_some(\"true\")",
        "data-has-label=state.has_accessible_name.then_some(\"true\")",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Icon should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn icon_styles_include_tone_size_and_state_markers() {
    let source = load_source("src/icon/styles.rs");

    for selector in [
        ".ui-icon--size-sm",
        ".ui-icon[data-size=\"md\"]",
        ".ui-icon--tone-default",
        ".ui-icon[data-tone=\"accent\"]",
        ".ui-icon--disabled",
        ".ui-icon[data-disabled=\"true\"]",
        ".ui-icon--decorative",
        ".ui-icon[data-decorative=\"true\"]",
        ".ui-icon--custom-class",
        ".ui-icon[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Icon styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn icon_supports_accessibility_role_and_label_contract() {
    let source = load_source("src/icon/view.rs");

    for needle in [
        "role=(!state.is_decorative).then_some(\"img\")",
        "aria-label=state.has_accessible_name.then_some(aria_label)",
        "aria-hidden=state.is_decorative.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Icon should include `{needle}` for Spectrum-style accessibility contracts."
        );
    }
}
