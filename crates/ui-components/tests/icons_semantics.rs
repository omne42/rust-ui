use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icons_does_not_expose_view_module() {
    let source = load_source("src/icons/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Icons internals should stay private; found `pub mod view`."
    );
}

#[test]
fn icons_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/icons/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{Icons, IconsScale, IconsSet};"),
        "icons module should export `Icons`, `IconsScale`, and `IconsSet`."
    );
    assert!(
        crate_source
            .contains("pub use icons::{Icons, IconsGlyph, IconsScale, IconsSet, IconsTone};"),
        "crate root should re-export `Icons` contracts."
    );
}

#[test]
fn icons_wraps_icons_ui_and_icons_workflow_contracts() {
    let source = load_source("src/icons/view.rs");

    for needle in [
        "pub fn Icons(",
        "enum IconsSet",
        "enum IconsScale",
        "<IconsUi",
        "<IconsWorkflow",
        "data-slot=\"icons\"",
    ] {
        assert!(
            source.contains(needle),
            "Icons wrapper should preserve nested icon contract marker `{needle}`."
        );
    }
}

#[test]
fn icons_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons.rs");

    for needle in [
        "pub(super) fn icons() -> AnyView",
        "title=\"Icons\"",
        "slug=\"icons\"",
        "<Icons",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_icons docs page should contain `{needle}`."
        );
    }
}
