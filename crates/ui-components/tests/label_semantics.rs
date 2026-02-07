use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn label_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/label/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Label internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn label_uses_logic_state_model() {
    let logic_source = load_source("src/label/logic.rs");
    let view_source = load_source("src/label/view.rs");

    for needle in [
        "pub enum LabelEmphasis",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label_text(",
        "pub fn normalize_required_indicator(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "indicator_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Label logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_label_text(text)",
        "logic::normalize_required_indicator(required_indicator)",
        "logic::normalize_optional_text(for_id)",
        "logic::resolve_state(LabelStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Label view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn label_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/label/view.rs");

    for attr in [
        "data-slot=\"label\"",
        "data-emphasis=move || state.get().emphasis_attr",
        "data-state=move || if state.get().is_required { \"required\" } else { \"optional\" }",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-for=move || state.get().has_for_id.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-indicator-source=move || state.get().indicator_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"label-text\"",
        "data-slot=\"label-required\"",
    ] {
        assert!(
            source.contains(attr),
            "Label should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn label_styles_include_emphasis_required_and_source_markers() {
    let source = load_source("src/label/styles.rs");

    for selector in [
        ".ui-label--emphasis-default",
        ".ui-label[data-emphasis=\"default\"]",
        ".ui-label--emphasis-subtle",
        ".ui-label--emphasis-strong",
        ".ui-label--required",
        ".ui-label[data-required=\"true\"]",
        ".ui-label--disabled",
        ".ui-label[data-disabled=\"true\"]",
        ".ui-label--for",
        ".ui-label[data-has-for=\"true\"]",
        ".ui-label--text-custom",
        ".ui-label[data-label-source=\"custom\"]",
        ".ui-label--indicator-custom",
        ".ui-label[data-indicator-source=\"custom\"]",
        ".ui-label--custom-class",
        ".ui-label[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Label styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
