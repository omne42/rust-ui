use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_breadcrumb_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/breadcrumb").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_breadcrumb_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-breadcrumb\")]")
            && lib_source.contains("pub use ui_breadcrumb as breadcrumb;"),
        "ui-components should re-export the external ui-breadcrumb crate as `breadcrumb`.",
    );
    assert!(
        cargo_source.contains("component-breadcrumb = [\"dep:ui-breadcrumb\"]"),
        "component-breadcrumb feature should depend on dep:ui-breadcrumb after extraction.",
    );
    assert!(
        cargo_source.contains(
            "ui-breadcrumb = { path = \"../../components/breadcrumb\", optional = true }"
        ),
        "ui-components Cargo.toml should include the optional ui-breadcrumb dependency.",
    );
    assert!(
        !cargo_source.contains("component-breadcrumbs ="),
        "component-breadcrumbs should be removed after merge.",
    );
}

#[test]
fn breadcrumb_component_module_exposes_unified_api() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");

    for needle in [
        "pub use logic::BreadcrumbItem;",
        "pub use view::Breadcrumb;",
        "pub mod styles;",
    ] {
        assert!(
            module_source.contains(needle),
            "breadcrumb component module should export `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_view_accepts_items_and_optional_root_props() {
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "pub fn Breadcrumb(",
        "items: Vec<BreadcrumbItem>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should expose `{needle}` in public props."
        );
    }
}

#[test]
fn breadcrumb_logic_uses_state_primitives_and_item_mapping() {
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for needle in [
        "use ui_state_primitives::breadcrumbs as breadcrumbs_primitives;",
        "pub struct BreadcrumbItem",
        "pub fn resolve_root_state(",
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "pub fn resolve_item_href(item: &BreadcrumbItem, is_last: bool) -> Option<String>",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb logic should include `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_view_emits_unified_state_markers() {
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "data-slot=\"breadcrumb\"",
        "data-slot=\"breadcrumb-list\"",
        "data-slot=\"breadcrumb-item\"",
        "data-slot=\"breadcrumb-link\"",
        "data-slot=\"breadcrumb-page\"",
        "data-slot=\"breadcrumb-label\"",
        "data-slot=\"breadcrumb-separator\"",
        "data-aria-source=aria_source_attr",
        "data-class-source=class_source_attr",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-links=state.has_links.then_some(\"true\")",
        "data-count=state.item_count",
        "aria-current=\"page\"",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should expose `{needle}` for semantic/state inspection."
        );
    }
}

#[test]
fn breadcrumb_styles_cover_core_accessibility_and_structure_contracts() {
    let styles_source = load_breadcrumb_component_source("src/styles.rs");

    for needle in [
        ".ui-breadcrumb {",
        ".ui-breadcrumb__list",
        ".ui-breadcrumb__item",
        ".ui-breadcrumb__link",
        ".ui-breadcrumb__label",
        ".ui-breadcrumb__page",
        ".ui-breadcrumb__separator",
        "@media (forced-colors: active)",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Breadcrumb styles should include `{needle}`."
        );
    }
}

#[test]
fn docs_page_uses_unified_breadcrumb_api() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );

    for needle in [
        "pub(super) fn breadcrumb() -> AnyView",
        "title=\"Breadcrumb\"",
        "slug=\"breadcrumb\"",
        "<Breadcrumb",
        "items=items",
    ] {
        assert!(
            source.contains(needle),
            "breadcrumb docs page should include `{needle}`."
        );
    }
}

#[test]
fn docs_navigation_no_longer_lists_breadcrumbs_or_primitives() {
    let pages_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        !pages_source.contains("\"breadcrumbs\""),
        "components pages should no longer expose a separate breadcrumbs route."
    );
    assert!(
        !pages_source.contains("\"breadcrumb-list\""),
        "components pages should no longer expose breadcrumb primitive routes."
    );
}
