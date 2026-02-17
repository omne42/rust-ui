use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn number_module_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/number/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Number internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn static_number_uses_logic_state_model() {
    let view_source = load_source("src/number/view.rs");
    let logic_source = load_source("src/number/logic.rs");

    for needle in [
        "pub struct StaticNumberStateInput",
        "pub struct StaticNumberState",
        "pub enum NumberSign",
        "pub fn normalize_optional_text(",
        "pub fn resolve_decimal_separator(",
        "pub fn resolve_thousand_separator(",
        "pub fn sanitize_decimal_places(",
        "pub fn sanitize_number(",
        "pub fn resolve_sign(",
        "pub fn resolve_static_number_state(",
        "pub fn compose_static_number_class_name(",
        "decimal_separator_source_attr",
        "decimal_places_source_attr",
        "thousand_separator_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Number logic should include `{needle}` for centralized static-number state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_decimal_separator(decimal_separator)",
        "logic::sanitize_decimal_places(decimal_places)",
        "logic::resolve_thousand_separator(thousand_separator)",
        "logic::sanitize_number(number)",
        "logic::resolve_static_number_state(logic::StaticNumberStateInput {",
        "logic::compose_static_number_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "StaticNumber view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn static_number_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/number/view.rs");

    for attr in [
        "data-slot=\"static-number\"",
        "data-state=state.sign_attr",
        "data-sign=state.sign_attr",
        "data-decimal-separator-source=state.decimal_separator_source_attr",
        "data-decimal-places-source=state.decimal_places_source_attr",
        "data-thousand-separator-source=state.thousand_separator_source_attr",
        "data-custom-decimal-separator=state.has_custom_decimal_separator.then_some(\"true\")",
        "data-custom-decimal-places=state.has_custom_decimal_places.then_some(\"true\")",
        "data-custom-thousand-separator=state.has_custom_thousand_separator.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "StaticNumber should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn static_number_styles_include_sign_and_source_contracts() {
    let source = load_source("src/number/styles.rs");

    for selector in [
        ".ui-static-number--sign-negative",
        ".ui-static-number[data-sign=\"zero\"]",
        ".ui-static-number--decimal-separator-custom",
        ".ui-static-number[data-decimal-separator-source=\"custom\"]",
        ".ui-static-number--decimal-places-custom",
        ".ui-static-number[data-decimal-places-source=\"custom\"]",
        ".ui-static-number--thousand-separator-custom",
        ".ui-static-number[data-thousand-separator-source=\"custom\"]",
        ".ui-static-number--custom-class",
        ".ui-static-number[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "StaticNumber styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn static_number_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn static_number() -> AnyView",
        "title=\"StaticNumber\"",
        "slug=\"static-number\"",
        "Playground title=\"Formatting Matrix\"",
        "Playground title=\"Custom Separators + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for StaticNumber.",
        );
    }
}

#[test]
fn static_number_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Formatting Matrix\"",
        "number=12345.67",
        "number=-9876.5",
        "number=1000.0 decimal_places=0",
        "title=\"Custom Separators + Class\"",
        "number=42.123456789",
        "decimal_separator=\",\".to_string()",
        "decimal_places=30",
        "thousand_separator=\" \".to_string()",
        "number=f64::NAN",
        "class_name=\"docs-static-number-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "static-number docs playgrounds should contain `{needle}`.",
        );
    }
}
