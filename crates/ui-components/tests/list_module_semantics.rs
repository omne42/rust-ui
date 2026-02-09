use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn list_module_reexports_listview_and_item_contracts() {
    let source = load_source("src/list/mod.rs");

    for needle in [
        "pub use crate::listbox::ListBox as ListView;",
        "pub use crate::item::Item;",
    ] {
        assert!(
            source.contains(needle),
            "list module should expose `{needle}` for @react-spectrum/list compatibility."
        );
    }
}

#[test]
fn crate_root_registers_list_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod list;"),
        "crate root should include `pub mod list;` for @react-spectrum/list compatibility."
    );
}

#[test]
fn list_compatibility_reuses_listbox_and_item_docs_playgrounds() {
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let item_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_item_shadcn.rs");

    for needle in ["title=\"ListBox\"", "slug=\"listbox\"", "<ListBox"] {
        assert!(
            collections_source.contains(needle),
            "collections docs should contain `{needle}` for ListView compatibility coverage."
        );
    }

    for needle in ["title=\"Item\"", "slug=\"item\"", "<Item"] {
        assert!(
            item_source.contains(needle),
            "item docs should contain `{needle}` for Item compatibility coverage."
        );
    }
}
