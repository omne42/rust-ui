use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn badge_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/badge/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Badge internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn badge_uses_logic_state_model() {
    let view_source = load_source("src/badge/view.rs");
    let logic_source = load_source("src/badge/logic.rs");

    for needle in [
        "pub struct BadgeStateInput",
        "pub struct BadgeState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: BadgeStateInput)",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Badge logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(BadgeStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Badge view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn badge_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/badge/view.rs");

    for attr in [
        "data-slot=\"badge\"",
        "data-variant=state.variant_attr",
        "data-fill=state.fill_attr",
        "data-state=state.fill_attr",
        "data-solid=state.is_solid.then_some(\"true\")",
        "data-outline=state.is_outline.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Badge should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn badge_styles_include_variant_fill_and_custom_class_markers() {
    let source = load_source("src/badge/styles.rs");

    for selector in [
        ".ui-badge--variant-default",
        ".ui-badge[data-variant=\"accent\"]",
        ".ui-badge--variant-danger",
        ".ui-badge[data-variant=\"outline\"]",
        ".ui-badge--fill-solid",
        ".ui-badge[data-fill=\"solid\"]",
        ".ui-badge[data-state=\"outline\"]",
        ".ui-badge--custom-class",
        ".ui-badge[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Badge styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn badge_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn badge() -> AnyView",
        "title=\"Badge\"",
        "slug=\"badge\"",
        "Playground title=\"Variant Matrix\"",
        "Playground title=\"Custom Class + Outline\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Badge.",
        );
    }
}

#[test]
fn badge_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Variant Matrix\"",
        "<Badge variant=BadgeVariant::Default>\"Default\"</Badge>",
        "<Badge variant=BadgeVariant::Accent>\"Accent\"</Badge>",
        "<Badge variant=BadgeVariant::Danger>\"Danger\"</Badge>",
        "<Badge variant=BadgeVariant::Outline>\"Outline\"</Badge>",
        "title=\"Custom Class + Outline\"",
        "variant=BadgeVariant::Accent class_name=\"docs-badge-custom\".to_string()",
        "variant=BadgeVariant::Outline class_name=\"docs-badge-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "badge docs playgrounds should contain `{needle}`.",
        );
    }
}
