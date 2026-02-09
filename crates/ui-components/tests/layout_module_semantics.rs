use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn layout_module_reexports_flex_and_grid_contracts() {
    let source = load_source("src/layout/mod.rs");

    for needle in [
        "pub use crate::flex::{Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexWrap};",
        "pub use crate::grid::{Grid, GridAlign, GridColumns, GridGap, GridJustify, GridRows};",
    ] {
        assert!(
            source.contains(needle),
            "layout module should expose `{needle}` for react-spectrum layout compatibility."
        );
    }
}

#[test]
fn crate_root_registers_layout_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod layout;"),
        "crate root should include `pub mod layout;` for @react-spectrum/layout compatibility."
    );
}

#[test]
fn layout_compatibility_reuses_flex_and_grid_docs_playgrounds() {
    let layout_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let layout_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in ["title=\"Flex\"", "slug=\"flex\"", "<Flex"] {
        assert!(
            layout_source.contains(needle),
            "layout docs should contain `{needle}` for Flex compatibility coverage."
        );
    }

    for needle in ["title=\"Grid\"", "slug=\"grid\"", "<Grid"] {
        assert!(
            layout_extra_source.contains(needle),
            "layout_extra docs should contain `{needle}` for Grid compatibility coverage."
        );
    }
}
