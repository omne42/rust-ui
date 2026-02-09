use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn s2_module_reexports_theme_and_ui_root_contracts() {
    let source = load_source("src/s2/mod.rs");

    assert!(
        source.contains("pub use crate::{Theme, UiRoot};"),
        "s2 module should expose Theme + UiRoot compatibility contracts."
    );
}

#[test]
fn crate_root_registers_s2_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod s2;"),
        "crate root should include `pub mod s2;` for @react-spectrum/s2 compatibility."
    );
}

#[test]
fn s2_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in ["title=\"UiRoot\"", "slug=\"ui-root\"", "<UiRoot"] {
        assert!(
            source.contains(needle),
            "layout ui_root docs should contain `{needle}` for s2 compatibility coverage."
        );
    }
}
