use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn skeleton_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/skeleton/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Skeleton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn skeleton_uses_logic_state_model() {
    let view_source = load_source("src/skeleton/view.rs");
    let logic_source = load_source("src/skeleton/logic.rs");

    for needle in [
        "pub struct SkeletonStateInput",
        "pub struct SkeletonState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Skeleton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(SkeletonStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Skeleton view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn skeleton_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/skeleton/view.rs");

    for attr in [
        "data-slot=\"skeleton\"",
        "data-variant=state.variant_attr",
        "data-state=if state.has_shimmer { \"shimmer\" } else { \"still\" }",
        "data-shimmer=state.has_shimmer.then_some(\"true\")",
        "data-still=state.is_still.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Skeleton should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn skeleton_styles_include_variant_and_shimmer_markers() {
    let source = load_source("src/skeleton/styles.rs");

    for selector in [
        ".ui-skeleton--variant-rect",
        ".ui-skeleton[data-variant=\"circle\"]",
        ".ui-skeleton--shimmer::after",
        ".ui-skeleton[data-shimmer=\"true\"]::after",
        ".ui-skeleton--still",
        ".ui-skeleton[data-still=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Skeleton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn skeleton_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn skeleton() -> AnyView",
        "title=\"Skeleton\"",
        "slug=\"skeleton\"",
        "Playground title=\"Shimmer\"",
        "Playground title=\"Still\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Skeleton.",
        );
    }
}

#[test]
fn skeleton_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Shimmer\"",
        "<Skeleton variant=SkeletonVariant::Rect class_name=\"docs-skeleton-line\".to_string() />",
        "<Skeleton variant=SkeletonVariant::Circle class_name=\"docs-skeleton-avatar\".to_string() />",
        "title=\"Still\"",
        "shimmer=false",
        "class_name=\"docs-skeleton-line docs-skeleton-line--short\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "skeleton docs playgrounds should contain `{needle}`.",
        );
    }
}
