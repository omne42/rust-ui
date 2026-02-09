use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn coachmark_does_not_expose_view_module() {
    let source = load_source("src/coachmark/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Coachmark internals should stay private; found `pub mod view`."
    );
}

#[test]
fn coachmark_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/coachmark/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Coachmark;"),
        "coachmark module should export `Coachmark`."
    );
    assert!(
        crate_source.contains("pub use coachmark::{"),
        "crate root should re-export `Coachmark` contracts."
    );
}

#[test]
fn coachmark_wraps_contextual_help_contract() {
    let source = load_source("src/coachmark/view.rs");

    for needle in [
        "pub fn Coachmark(",
        "<ContextualHelp",
        "primary_cta: Option<String>",
        "asset_variant: Option<CoachmarkAssetVariant>",
    ] {
        assert!(
            source.contains(needle),
            "Coachmark wrapper should preserve ContextualHelp contract marker `{needle}`."
        );
    }
}

#[test]
fn coachmark_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs");

    for needle in [
        "pub(super) fn coachmark() -> AnyView",
        "title=\"Coachmark\"",
        "slug=\"coachmark\"",
        "<Coachmark",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs page should contain `{needle}` for Coachmark."
        );
    }
}
