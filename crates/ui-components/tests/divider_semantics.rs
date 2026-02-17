use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn divider_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/divider/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Divider internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn divider_uses_logic_state_model() {
    let view_source = load_source("src/divider/view.rs");
    let logic_source = load_source("src/divider/logic.rs");

    for needle in [
        "pub struct DividerStateInput",
        "pub struct DividerState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Divider logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(DividerStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Divider view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn divider_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/divider/view.rs");

    for attr in [
        "data-slot=\"divider\"",
        "data-orientation=state.orientation_attr",
        "data-state=state.orientation_attr",
        "data-horizontal=state.is_horizontal.then_some(\"true\")",
        "data-vertical=state.is_vertical.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "role=\"separator\"",
        "aria-orientation=state.aria_orientation",
    ] {
        assert!(
            source.contains(attr),
            "Divider should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn divider_styles_include_orientation_state_markers() {
    let source = load_source("src/divider/styles.rs");

    for selector in [
        ".ui-divider--horizontal",
        ".ui-divider[data-orientation=\"horizontal\"]",
        ".ui-divider[data-state=\"horizontal\"]",
        ".ui-divider[data-horizontal=\"true\"]",
        ".ui-divider--vertical",
        ".ui-divider[data-orientation=\"vertical\"]",
        ".ui-divider[data-state=\"vertical\"]",
        ".ui-divider[data-vertical=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Divider styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn divider_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn divider() -> AnyView",
        "title=\"Divider\"",
        "slug=\"divider\"",
        "Playground title=\"Orientation\"",
        "Playground title=\"Custom Class Marker\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Divider.",
        );
    }
}

#[test]
fn divider_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Orientation\"",
        "<Divider />",
        "orientation=DividerOrientation::Vertical",
        "class_name=\"docs-divider-rail\".to_string()",
        "title=\"Custom Class Marker\"",
        "<Divider class_name=\"docs-divider-custom\".to_string() />",
        "class_name=\"docs-divider-custom docs-divider-rail\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "divider docs playgrounds should contain `{needle}`.",
        );
    }
}
