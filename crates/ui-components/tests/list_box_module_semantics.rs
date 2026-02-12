use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn list_box_module_reexports_listbox_contracts() {
    let source = load_source("src/list_box/mod.rs");

    for needle in [
        "pub use crate::listbox::ListBox;",
        "pub use crate::listbox_item::ListBoxItem;",
        "pub use crate::listbox_item::ListBoxItemSelectionIndicator;",
        "pub use crate::listbox_section::ListBoxSection;",
        "pub use crate::listbox_section::ListBoxSectionHeadingTone;",
    ] {
        assert!(
            source.contains(needle),
            "list_box module should expose `{needle}` for react-aria-components ListBox compatibility.",
        );
    }
}

#[test]
fn crate_root_registers_list_box_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod list_box;"),
        "crate root should include `pub mod list_box;` for list-box compatibility.",
    );
}

#[test]
fn list_box_compatibility_reuses_listbox_docs_playgrounds() {
    let listbox_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let item_section_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in ["title=\"ListBox\"", "slug=\"listbox\"", "<ListBox"] {
        assert!(
            listbox_source.contains(needle),
            "collections docs should contain `{needle}` for list-box compatibility coverage.",
        );
    }

    for needle in [
        "title=\"ListBoxItem\"",
        "slug=\"listbox-item\"",
        "<ListBoxItem",
        "title=\"ListBoxSection\"",
        "slug=\"listbox-section\"",
        "<ListBoxSection",
    ] {
        assert!(
            item_section_source.contains(needle),
            "collections-extra docs should contain `{needle}` for list-box compatibility coverage.",
        );
    }
}
