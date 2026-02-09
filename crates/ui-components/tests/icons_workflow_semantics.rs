use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icons_workflow_does_not_expose_view_module() {
    let source = load_source("src/icons_workflow/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "IconsWorkflow internals should stay private; found `pub mod view`."
    );
}

#[test]
fn icons_workflow_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/icons_workflow/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::IconsWorkflow;"),
        "icons_workflow module should export `IconsWorkflow`."
    );
    assert!(
        crate_source.contains(
            "pub use icons_workflow::{IconsWorkflow, IconsWorkflowSize, IconsWorkflowTone};"
        ),
        "crate root should re-export `IconsWorkflow` contracts."
    );
}

#[test]
fn icons_workflow_wraps_iconset_contract() {
    let source = load_source("src/icons_workflow/view.rs");

    for needle in [
        "pub fn IconsWorkflow(",
        "fn default_workflow_glyphs()",
        "<Iconset",
        "iconset=\"workflow\".to_string()",
        "data-slot=\"icons-workflow\"",
    ] {
        assert!(
            source.contains(needle),
            "IconsWorkflow wrapper should preserve Iconset contract marker `{needle}`."
        );
    }
}

#[test]
fn icons_workflow_docs_page_exists() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_icons_workflow.rs",
    );

    for needle in [
        "pub(super) fn icons_workflow() -> AnyView",
        "title=\"IconsWorkflow\"",
        "slug=\"icons-workflow\"",
        "<IconsWorkflow",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_icons_workflow docs page should contain `{needle}`."
        );
    }
}
