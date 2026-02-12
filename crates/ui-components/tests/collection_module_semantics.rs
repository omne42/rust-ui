use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn collection_module_reexports_item_contracts() {
    let source = load_source("src/collection/mod.rs");

    for needle in [
        "pub use crate::item::Item as Collection;",
        "pub use crate::item::ItemGroup as CollectionSection;",
        "pub use crate::item::ItemSeparator as CollectionSeparator;",
    ] {
        assert!(
            source.contains(needle),
            "collection module should expose `{needle}` for react-aria-components Collection compatibility.",
        );
    }
}

#[test]
fn crate_root_registers_collection_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod collection;",
        "pub use collection::{Collection, CollectionSection, CollectionSeparator};",
        "pub use collection::{",
        "Collection, CollectionSection, CollectionSeparator",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for collection compatibility.",
        );
    }
}

#[test]
fn collection_compatibility_reuses_item_docs_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_item_shadcn.rs");

    for needle in [
        "title=\"Item\"",
        "slug=\"item\"",
        "<ItemGroup",
        "<ItemSeparator",
    ] {
        assert!(
            source.contains(needle),
            "item docs should contain `{needle}` for collection compatibility coverage.",
        );
    }
}
