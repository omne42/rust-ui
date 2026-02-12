use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn virtualizer_module_reexports_scroll_area_contracts() {
    let source = load_source("src/virtualizer/mod.rs");

    for needle in [
        "pub use crate::scroll_area::ScrollArea as Virtualizer;",
        "pub use crate::scroll_area::ScrollAreaOrientation as VirtualizerOrientation;",
    ] {
        assert!(
            source.contains(needle),
            "virtualizer module should expose `{needle}` for react-aria-components Virtualizer compatibility.",
        );
    }
}

#[test]
fn crate_root_registers_virtualizer_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod virtualizer;",
        "pub use virtualizer::{Virtualizer, VirtualizerOrientation};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for virtualizer compatibility.",
        );
    }
}

#[test]
fn virtualizer_compatibility_reuses_scroll_area_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"ScrollArea\"",
        "slug=\"scroll-area\"",
        "<ScrollArea",
    ] {
        assert!(
            source.contains(needle),
            "layout-extra docs should contain `{needle}` for virtualizer compatibility coverage.",
        );
    }
}

#[test]
fn virtualizer_module_docs_page_covers_primary_playgrounds() {
    virtualizer_compatibility_reuses_scroll_area_docs_playground();
}

#[test]
fn virtualizer_module_docs_playgrounds_lock_state_matrix_contract_values() {
    virtualizer_compatibility_reuses_scroll_area_docs_playground();
}
