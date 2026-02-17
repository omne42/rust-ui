use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn list_module_reexports_canonical_list_contracts() {
    let source = load_source("src/list/mod.rs");

    for needle in [
        "pub use logic::{ListItemSelectionIndicator, ListSectionHeadingTone, ListState};",
        "pub use motion::ListMotion;",
        "pub use motion::ListSectionMotion;",
        "pub use view::{List, ListItem, ListSection};",
    ] {
        assert!(
            source.contains(needle),
            "list module should expose canonical `{needle}`."
        );
    }

    for removed in [
        "pub use crate::listbox::ListBox as ListView;",
        "pub use crate::item::Item;",
    ] {
        assert!(
            !source.contains(removed),
            "list module should not keep removed alias `{removed}`."
        );
    }
}

#[test]
fn crate_root_registers_list_and_hides_listbox_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod list;"),
        "crate root should include `pub mod list;`."
    );
    assert!(
        !source.contains("mod listbox;"),
        "crate root should not keep legacy listbox module."
    );
}

#[test]
fn list_docs_use_list_family_slugs_and_components() {
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let collections_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "component_doc!(\"List\", \"list\", \"Collections\", collections::list)",
        "\"ListItem\"",
        "\"list-item\"",
        "\"ListSection\"",
        "\"list-section\"",
        "collections_extra::list_item",
        "collections_extra::list_section",
    ] {
        assert!(
            pages_source.contains(needle),
            "components catalog should include `{needle}` for list family docs."
        );
    }

    for needle in [
        "pub(super) fn list() -> AnyView",
        "title=\"List\"",
        "slug=\"list\"",
        "<List",
    ] {
        assert!(
            collections_source.contains(needle),
            "collections docs should include `{needle}` for the canonical List page."
        );
    }

    for needle in [
        "pub(super) fn list_item() -> AnyView",
        "title=\"ListItem\"",
        "slug=\"list-item\"",
        "<ListItem",
        "pub(super) fn list_section() -> AnyView",
        "title=\"ListSection\"",
        "slug=\"list-section\"",
        "<ListSection",
    ] {
        assert!(
            collections_extra_source.contains(needle),
            "collections-extra docs should include `{needle}` for list item/section pages."
        );
    }

    assert!(
        mod_source.contains("\"list\" => &[\"list\", \"list-item\", \"list-section\"]"),
        "components mapping should point `list` to list/list-item/list-section."
    );
    assert!(
        !mod_source.contains("\"list-box\" =>"),
        "components mapping should not contain the removed `list-box` alias."
    );
}
