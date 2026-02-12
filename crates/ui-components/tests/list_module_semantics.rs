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

#[test]
fn list_module_docs_page_covers_primary_playgrounds() {
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let item_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_item_shadcn.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn list_box() -> AnyView",
        "title=\"ListBox\"",
        "slug=\"listbox\"",
        "description=\"Listbox with active highlight spring motion, typeahead, and Spectrum-style root state attrs.\"",
        "<Playground title=\"Selection + Typeahead\" code=code>",
        "<Playground title=\"Disabled + Empty\" code=states_code>",
        "<ListBox",
    ] {
        assert!(
            collections_source.contains(needle),
            "collections docs should include `{needle}` for list module ListView primary playground coverage.",
        );
    }

    for needle in [
        "pub(super) fn item_primitives() -> AnyView",
        "title=\"Item\"",
        "slug=\"item\"",
        "description=\"Shadcn-compatible item composition primitives (`Item*`) with stable slot/variant/size contracts for media-content-actions and header-footer layouts.\"",
        "<Playground title=\"Media + Content + Actions\" code=basic_code>",
        "<Playground title=\"Header + Footer Layout\" code=advanced_code>",
        "<Item",
    ] {
        assert!(
            item_source.contains(needle),
            "collections_item_shadcn docs should include `{needle}` for list module Item primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"list\" => &[\"listbox\", \"item\"]"),
        "components mod mapping should keep `list` mapped to `listbox` and `item` slugs.",
    );
}

#[test]
fn list_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let item_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_item_shadcn.rs");

    for needle in [
        "title=\"Selection + Typeahead\"",
        "id_base=\"docs-listbox\".to_string()",
        "aria_label=\"Fruit\".to_string()",
        "disabled_indices=vec![3]",
        "title=\"Disabled + Empty\"",
        "id_base=\"docs-listbox-disabled\".to_string()",
        "aria_label=\"Disabled city list\".to_string()",
        "disabled=true",
        "id_base=\"docs-listbox-empty\".to_string()",
        "aria_label=\"Empty city list\".to_string()",
    ] {
        assert!(
            collections_source.contains(needle),
            "collections docs playgrounds should contain `{needle}` for list module ListView contracts.",
        );
    }

    for needle in [
        "title=\"Media + Content + Actions\"",
        "<ItemGroup>",
        "variant=ItemVariant::Outline",
        "size=ItemSize::Default",
        "<ItemMedia variant=ItemMediaVariant::Icon>",
        "\"Build Artifact\"",
        "\"Generated from latest CI pipeline.\"",
        "<ItemActions>",
        "<ItemSeparator />",
        "title=\"Header + Footer Layout\"",
        "<ItemHeader>",
        "variant=ItemVariant::Muted",
        "size=ItemSize::Sm",
        "\"Edge deployment\"",
        "\"2 minutes ago · US-East\"",
        "<ItemFooter>",
        "\"Status: degraded\"",
    ] {
        assert!(
            item_source.contains(needle),
            "collections_item_shadcn docs playgrounds should contain `{needle}` for list module Item contracts.",
        );
    }
}
