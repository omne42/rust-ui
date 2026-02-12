use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn selection_indicator_module_reexports_listbox_and_menu_indicator_contracts() {
    let source = load_source("src/selection_indicator/mod.rs");

    for needle in [
        "pub use crate::listbox_item::ListBoxItemSelectionIndicator as SelectionIndicator;",
        "pub use crate::menu_item::MenuItemSelectionIndicator as MenuSelectionIndicator;",
    ] {
        assert!(
            source.contains(needle),
            "selection_indicator module should expose `{needle}` for react-aria-components SelectionIndicator compatibility."
        );
    }
}

#[test]
fn crate_root_registers_selection_indicator_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::SelectionIndicator;",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for selection-indicator compatibility."
        );
    }
}

#[test]
fn selection_indicator_compatibility_reuses_listbox_item_and_menu_item_docs_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "title=\"ListBoxItem\"",
        "slug=\"listbox-item\"",
        "<ListBoxItem",
        "title=\"MenuItem\"",
        "slug=\"menu-item\"",
        "<MenuItem",
    ] {
        assert!(
            source.contains(needle),
            "collections-extra docs should contain `{needle}` for selection-indicator compatibility coverage.",
        );
    }
}
