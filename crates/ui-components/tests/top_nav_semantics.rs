use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn top_nav_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/top_nav/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "TopNav internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn top_nav_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/top_nav/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::TopNav;"),
        "top_nav module should export `TopNav`.",
    );
    assert!(
        crate_source.contains("pub use top_nav::{TopNav, TopNavItem, TopNavMotion};"),
        "crate root should re-export `TopNav`, `TopNavItem`, and `TopNavMotion`.",
    );
}

#[test]
fn top_nav_logic_exposes_state_helpers() {
    let source = load_source("src/top_nav/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_label(",
        "pub fn normalize_default_selected_id(",
        "pub fn resolve_state(input: TopNavStateInput)",
        "pub fn compose_class_name(class_name: Option<String>, state: TopNavState)",
        "DEFAULT_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "TopNav logic should include `{needle}` for centralized source/state contracts.",
        );
    }
}

#[test]
fn top_nav_view_uses_logic_state_and_motion_contracts() {
    let source = load_source("src/top_nav/view.rs");

    for needle in [
        "logic::normalize_label(label)",
        "logic::normalize_default_selected_id(default_selected_id)",
        "logic::resolve_state(TopNavStateInput {",
        "logic::compose_class_name(class_name, state)",
        "data-slot=\"top-nav\"",
        "data-state=state.state_attr",
        "data-selection-mode=state.selection_mode_attr",
        "data-default-selection=state.default_selection_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "TopNav view should include `{needle}` for stable marker contracts.",
        );
    }
}

#[test]
fn top_nav_styles_include_motion_and_source_markers() {
    let source = load_source("src/top_nav/styles.rs");

    for selector in [
        ".ui-top-nav {",
        ".ui-top-nav[data-selection-mode=\"controlled\"]",
        ".ui-top-nav[data-focus-activation=\"manual\"]",
        ".ui-top-nav[data-has-default-selection=\"true\"]",
        ".ui-top-nav[data-label-source=\"custom\"]",
        ".ui-top-nav[data-motion-source=\"custom\"]",
        ".ui-top-nav[data-custom-motion=\"true\"]",
        ".ui-top-nav--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "TopNav styles should include `{selector}` as stable selectors.",
        );
    }
}

#[test]
fn top_nav_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::top_nav::styles::CSS);"),
        "ui-components css aggregator should include top_nav styles.",
    );
}

#[test]
fn top_nav_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_top_nav.rs");

    for needle in [
        "pub(super) fn top_nav() -> AnyView",
        "title=\"TopNav\"",
        "slug=\"top-nav\"",
        "State + Source Markers",
        "data-selection-mode",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra_top_nav docs page should contain `{needle}`.",
        );
    }
}
