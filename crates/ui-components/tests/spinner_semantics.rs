use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn spinner_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/spinner/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Spinner internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn spinner_uses_logic_state_model() {
    let view_source = load_source("src/spinner/view.rs");
    let logic_source = load_source("src/spinner/logic.rs");

    for needle in [
        "pub const DEFAULT_ARIA_LABEL",
        "pub struct SpinnerStateInput",
        "pub struct SpinnerState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spinner logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_state(SpinnerStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Spinner view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn spinner_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/spinner/view.rs");

    for attr in [
        "data-slot=\"spinner\"",
        "data-size=state.size_attr",
        "data-state=\"indeterminate\"",
        "data-indeterminate=\"true\"",
        "data-label-source=state.label_source_attr",
        "data-custom-aria-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "class_name=\"ui-spinner__progress\"",
    ] {
        assert!(
            source.contains(attr),
            "Spinner should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn spinner_styles_include_size_and_source_markers() {
    let source = load_source("src/spinner/styles.rs");

    for selector in [
        ".ui-spinner__progress",
        ".ui-spinner--size-sm",
        ".ui-spinner[data-size=\"md\"]",
        ".ui-spinner--size-lg",
        ".ui-spinner--label-custom .ui-spinner__progress",
        ".ui-spinner[data-label-source=\"custom\"] .ui-spinner__progress",
        ".ui-spinner--custom-class",
        ".ui-spinner[data-custom-class=\"true\"]",
        ".ui-spinner[data-class-source=\"custom\"] .ui-spinner__progress",
        ".ui-spinner[data-state=\"indeterminate\"] .ui-spinner__progress",
    ] {
        assert!(
            source.contains(selector),
            "Spinner styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn spinner_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn spinner() -> AnyView",
        "title=\"Spinner\"",
        "slug=\"spinner\"",
        "Playground title=\"Size Matrix\"",
        "Playground title=\"Custom Label + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Spinner.",
        );
    }
}

#[test]
fn spinner_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Size Matrix\"",
        "<Spinner size=SpinnerSize::Sm />",
        "<Spinner size=SpinnerSize::Md />",
        "<Spinner size=SpinnerSize::Lg />",
        "title=\"Custom Label + Class\"",
        "<Spinner aria_label=\"Fetching notifications\".to_string() />",
        "aria_label=\"   \".to_string()",
        "aria_label=\"Syncing inbox\".to_string()",
        "class_name=\"docs-spinner-custom\".to_string()",
        "size=SpinnerSize::Lg",
    ] {
        assert!(
            source.contains(needle),
            "spinner docs playgrounds should contain `{needle}`.",
        );
    }
}
