use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tag_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tag/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tag internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tag_uses_logic_state_model() {
    let logic_source = load_source("src/tag/logic.rs");
    let view_source = load_source("src/tag/view.rs");

    for needle in [
        "pub enum TagVariant",
        "pub enum TagSize",
        "pub fn normalize_optional_text(",
        "pub fn normalize_remove_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "remove_label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tag logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_remove_aria_label(remove_aria_label)",
        "logic::resolve_state(TagStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn tag_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/tag/view.rs");

    for attr in [
        "data-slot=\"tag\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=state.state_attr",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-removable=state.is_removable.then_some(\"true\")",
        "data-static=state.is_static.then_some(\"true\")",
        "data-has-remove-handler=state.has_remove_handler.then_some(\"true\")",
        "data-remove-label-source=state.remove_label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-slot=\"tag-content\"",
        "data-slot=\"tag-remove-button\"",
        "data-label-source=state.remove_label_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Tag should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn tag_styles_include_variant_size_and_state_markers() {
    let source = load_source("src/tag/styles.rs");

    for selector in [
        ".ui-tag--size-sm",
        ".ui-tag[data-size=\"md\"]",
        ".ui-tag--variant-default",
        ".ui-tag[data-variant=\"surface\"]",
        ".ui-tag--enabled",
        ".ui-tag[data-state=\"disabled\"]",
        ".ui-tag[data-state=\"static\"]",
        ".ui-tag[data-state=\"removable\"]",
        ".ui-tag--custom-class",
        ".ui-tag[data-custom-class=\"true\"]",
        ".ui-tag[data-class-source=\"custom\"]",
        ".ui-tag__remove[data-disabled=\"true\"]",
        ".ui-tag__remove[data-label-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "Tag styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
