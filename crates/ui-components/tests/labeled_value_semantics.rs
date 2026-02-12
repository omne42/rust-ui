use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn labeled_value_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/labeled_value/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "LabeledValue internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn labeled_value_uses_logic_state_model() {
    let logic_source = load_source("src/labeled_value/logic.rs");
    let view_source = load_source("src/labeled_value/view.rs");

    for needle in [
        "pub enum LabeledValueTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label_text(",
        "pub fn normalize_value_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "value_source_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "LabeledValue logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_label_text(label)",
        "logic::normalize_value_text(value)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(LabeledValueStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "LabeledValue view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn labeled_value_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/labeled_value/view.rs");

    for attr in [
        "data-slot=\"labeled-value\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || if state.get().has_description { \"with-description\" } else { \"default\" }",
        "data-has-description=move || state.get().has_description.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-value-source=move || state.get().value_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "role=\"group\"",
        "data-slot=\"labeled-value-label\"",
        "data-slot=\"labeled-value-value\"",
        "data-slot=\"labeled-value-description\"",
    ] {
        assert!(
            source.contains(attr),
            "LabeledValue should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn labeled_value_styles_include_orientation_tone_and_source_markers() {
    let source = load_source("src/labeled_value/styles.rs");

    for selector in [
        ".ui-labeled-value--orientation-stacked",
        ".ui-labeled-value[data-orientation=\"stacked\"]",
        ".ui-labeled-value--orientation-inline",
        ".ui-labeled-value[data-orientation=\"inline\"]",
        ".ui-labeled-value--tone-default",
        ".ui-labeled-value--tone-subtle",
        ".ui-labeled-value--tone-strong",
        ".ui-labeled-value--with-description",
        ".ui-labeled-value[data-has-description=\"true\"]",
        ".ui-labeled-value--label-custom",
        ".ui-labeled-value[data-label-source=\"custom\"]",
        ".ui-labeled-value--value-custom",
        ".ui-labeled-value[data-value-source=\"custom\"]",
        ".ui-labeled-value--aria-custom",
        ".ui-labeled-value[data-aria-source=\"custom\"]",
        ".ui-labeled-value--custom-class",
        ".ui-labeled-value[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "LabeledValue styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn labeled_value_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn labeled_value() -> AnyView",
        "title=\"LabeledValue\"",
        "slug=\"labeled-value\"",
        "description=\"Label-value pair primitive with centralized orientation/tone/source state contracts and Spectrum-style data markers.\"",
        "<Playground title=\"Orientation + Tone\" code=orientation_code>",
        "<Playground title=\"Description + Custom Aria/Class\" code=custom_code>",
        "<LabeledValue",
    ] {
        assert!(
            source.contains(needle),
            "display_extra docs should include `{needle}` for labeled_value primary playground coverage.",
        );
    }
}

#[test]
fn labeled_value_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Orientation + Tone\"",
        "label=\"Project\".to_string()",
        "value=\"Omne\".to_string()",
        "orientation=LabeledValueOrientation::Inline",
        "tone=LabeledValueTone::Subtle",
        "title=\"Description + Custom Aria/Class\"",
        "label=\"Build\".to_string()",
        "description=\"Updated 2 minutes ago\".to_string()",
        "aria_label=\"Build status\".to_string()",
        "class_name=\"docs-labeled-value-custom\".to_string()",
        "tone=LabeledValueTone::Strong",
    ] {
        assert!(
            source.contains(needle),
            "labeled_value docs playgrounds should contain `{needle}`.",
        );
    }
}
