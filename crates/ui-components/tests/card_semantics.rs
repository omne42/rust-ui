use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn card_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/card/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Card internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn card_uses_logic_state_model() {
    let view_source = load_source("src/card/view.rs");
    let logic_source = load_source("src/card/logic.rs");

    for needle in [
        "pub struct CardStateInput",
        "pub struct CardState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Card logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(CardStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Card view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn card_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/card/view.rs");

    for attr in [
        "data-slot=\"card\"",
        "data-variant=state.variant_attr",
        "data-state=if state.is_padded { \"padded\" } else { \"flush\" }",
        "data-padded=state.is_padded.then_some(\"true\")",
        "data-flush=state.is_flush.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Card should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn card_styles_include_variant_and_padding_markers() {
    let source = load_source("src/card/styles.rs");

    for selector in [
        ".ui-card--padded",
        ".ui-card[data-padded=\"true\"]",
        ".ui-card--no-padding",
        ".ui-card[data-flush=\"true\"]",
        ".ui-card--variant-default",
        ".ui-card[data-variant=\"muted\"]",
        ".ui-card--variant-outline",
    ] {
        assert!(
            source.contains(selector),
            "Card styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn card_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn card() -> AnyView",
        "title=\"Card\"",
        "slug=\"card\"",
        "Playground title=\"Variants\"",
        "Playground title=\"Padding States\"",
        "Playground title=\"Custom Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Card.",
        );
    }
}

#[test]
fn card_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Variants\"",
        "<Card variant=CardVariant::Default>",
        "<Card variant=CardVariant::Muted>",
        "<Card variant=CardVariant::Outline>",
        "title=\"Padding States\"",
        "<Card padded=true>",
        "<Card padded=false>",
        "title=\"Custom Class\"",
        "<Card class_name=\"docs-card-custom\".to_string()>",
        "Verifies `data-custom-class` + class merge.",
    ] {
        assert!(
            source.contains(needle),
            "card docs playgrounds should contain `{needle}`.",
        );
    }
}
