use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn legend_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/legend/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Legend internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn legend_uses_logic_state_model() {
    let mod_source = load_source("src/legend/mod.rs");
    let view_source = load_source("src/legend/view.rs");
    let logic_source = load_source("src/legend/logic.rs");

    for needle in ["pub struct LegendStateInput", "pub struct LegendState"] {
        assert!(
            mod_source.contains(needle),
            "Legend module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_text(",
        "pub fn normalize_required_indicator(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_text(text)",
        "logic::normalize_required_indicator(required_indicator)",
        "logic::resolve_state(LegendStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn legend_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/legend/view.rs");

    for attr in [
        "data-slot=\"legend\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || if state.get().is_required { \"required\" } else { \"optional\" }",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-text-source=move || state.get().text_source_attr",
        "data-indicator-source=move || state.get().indicator_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-slot=\"legend-text\"",
        "data-slot=\"legend-required\"",
    ] {
        assert!(
            source.contains(attr),
            "Legend should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn legend_styles_include_state_marker_contracts() {
    let source = load_source("src/legend/styles.rs");

    for selector in [
        ".ui-legend--tone-default",
        ".ui-legend[data-tone=\"strong\"]",
        ".ui-legend--required",
        ".ui-legend[data-disabled=\"true\"]",
        ".ui-legend--text-custom",
        ".ui-legend[data-indicator-source=\"custom\"]",
        ".ui-legend--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Legend styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn legend_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn fieldset() -> AnyView",
        "title=\"Fieldset\"",
        "slug=\"fieldset\"",
        "<Playground title=\"Legend + Description\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Invalid + Actions\" code_signal=invalid_code>",
        "legend=\"Notification channels\".to_string()",
        "<Fieldset",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs should include `{needle}` for legend primary playground coverage.",
        );
    }
}

#[test]
fn legend_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Legend + Description\"",
        "legend=\"Notification channels\".to_string()",
        "description=\"Pick every channel you want to receive release updates from.\".to_string()",
        "required=true",
        "aria_label=\"Notification channel group\".to_string()",
        "title=\"Horizontal + Invalid + Actions\"",
        "orientation=FieldsetOrientation::Horizontal",
        "tone=FieldsetTone::Muted",
        "invalid=true",
        "error_message=\"Pick at least one channel\".to_string()",
        "class_name=\"docs-fieldset-custom\".to_string()",
        "\"Manage channels\"",
    ] {
        assert!(
            source.contains(needle),
            "legend docs playgrounds should contain `{needle}`.",
        );
    }
}
