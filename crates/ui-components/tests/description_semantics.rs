use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn description_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/description/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Description internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn description_uses_logic_state_model() {
    let logic_source = load_source("src/description/logic.rs");
    let view_source = load_source("src/description/view.rs");

    for needle in [
        "pub enum DescriptionTone",
        "pub enum DescriptionElement",
        "pub fn normalize_optional_text(",
        "pub fn normalize_content(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Description logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_content(Some(text))",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(DescriptionStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Description view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn description_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/description/view.rs");

    for attr in [
        "data-slot=\"description\"",
        "slot=\"description\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-truncate=move || state.get().is_truncated.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Description should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn description_styles_include_tone_state_and_markers() {
    let source = load_source("src/description/styles.rs");

    for selector in [
        ".ui-description--tone-default",
        ".ui-description[data-tone=\"default\"]",
        ".ui-description--tone-muted",
        ".ui-description[data-tone=\"muted\"]",
        ".ui-description--tone-negative",
        ".ui-description[data-tone=\"negative\"]",
        ".ui-description--disabled",
        ".ui-description[data-disabled=\"true\"]",
        ".ui-description--truncate",
        ".ui-description[data-truncate=\"true\"]",
        ".ui-description--custom-class",
        ".ui-description[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Description styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn description_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn description() -> AnyView",
        "title=\"Description\"",
        "slug=\"description\"",
        "description=\"baseline-style form description primitive with centralized tone/state/source contracts and stable slot semantics.\"",
        "<Playground title=\"Tone Variants\" code_signal=tone_code>",
        "<Playground title=\"Truncate + Element + Disabled\" code_signal=truncate_code>",
        "<Description",
        "DescriptionTone::Negative",
        "DescriptionElement::Span",
        "truncate=true",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for description primary coverage.",
        );
    }
}

#[test]
fn description_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "text=\"This appears below the field as guidance.\".to_string()",
        "tone=DescriptionTone::Default",
        "aria_label=\"Name helper\".to_string()",
        "text=\"Optional details are only visible to admins.\".to_string()",
        "tone=DescriptionTone::Muted",
        "text=\"Two-factor code expired. Request a new one.\".to_string()",
        "tone=DescriptionTone::Negative",
        "text=\"A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.\".to_string()",
        "element=DescriptionElement::Span",
        "truncate=true",
        "class_name=\"docs-description-custom\".to_string()",
        "text=\"Disabled helper text\".to_string()",
        "disabled=true",
        "class=\"docs-stack docs-description-limit\"",
    ] {
        assert!(
            source.contains(needle),
            "description docs playgrounds should contain `{needle}`.",
        );
    }
}
