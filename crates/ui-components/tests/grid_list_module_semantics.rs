use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn grid_list_module_reexports_gridlist_contracts() {
    let source = load_source("src/grid_list/mod.rs");

    for needle in [
        "pub use crate::gridlist::GridList;",
        "pub use crate::gridlist::GridListItem;",
        "pub use crate::gridlist::GridListItemSelectionIndicator;",
        "pub use crate::gridlist::GridListSection;",
        "pub use crate::gridlist::GridListSectionHeadingTone;",
    ] {
        assert!(
            source.contains(needle),
            "grid_list module should expose `{needle}` for react-aria-components GridList naming compatibility."
        );
    }
}

#[test]
fn crate_root_registers_grid_list_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod grid_list;"),
        "crate root should include `pub mod grid_list;` for grid-list compatibility.",
    );
}

#[test]
fn grid_list_compatibility_reuses_listbox_docs_playgrounds() {
    let listbox_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let item_section_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in ["title=\"ListBox\"", "slug=\"listbox\"", "<ListBox"] {
        assert!(
            listbox_source.contains(needle),
            "collections docs should contain `{needle}` for grid-list compatibility coverage.",
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
            "collections-extra docs should contain `{needle}` for grid-list compatibility coverage.",
        );
    }
}

#[test]
fn grid_list_module_docs_page_covers_primary_playgrounds() {
    let listbox_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let item_section_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "title=\"ListBox\"",
        "slug=\"listbox\"",
        "<Playground title=\"Selection + Typeahead\" code=code>",
        "<Playground title=\"Disabled + Empty\" code=states_code>",
        "<ListBox",
    ] {
        assert!(
            listbox_source.contains(needle),
            "collections listbox docs should include `{needle}` for grid_list_module primary playground coverage.",
        );
    }

    for needle in [
        "title=\"ListBoxItem\"",
        "slug=\"listbox-item\"",
        "title=\"ListBoxSection\"",
        "slug=\"listbox-section\"",
        "<ListBoxItem",
        "<ListBoxSection",
    ] {
        assert!(
            item_section_source.contains(needle),
            "collections_extra listbox docs should include `{needle}` for grid_list_module primary playground coverage.",
        );
    }
}

#[test]
fn grid_list_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let listbox_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let item_section_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

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
            listbox_source.contains(needle),
            "grid_list module listbox docs should contain `{needle}`.",
        );
    }

    for needle in [
        "title=\"Selectable Option\"",
        "title=\"Focused + Divider + Disabled\"",
        "id=\"docs-listbox-item-focused\".to_string()",
        "class_name=\"docs-listbox-item-custom\".to_string()",
        "title=\"Default Section\"",
        "title=\"Quiet + Sticky + Divider + Empty\"",
        "heading_tone=ListBoxSectionHeadingTone::Quiet",
        "class_name=\"docs-listbox-section-custom\".to_string()",
    ] {
        assert!(
            item_section_source.contains(needle),
            "grid_list module listbox-item/section docs should contain `{needle}`.",
        );
    }

    for needle in [
        "\"gridlist\" => &[\"listbox\", \"listbox-item\", \"listbox-section\"]",
        "\"grid-list\" => &[\"listbox\", \"listbox-item\", \"listbox-section\"]",
    ] {
        assert!(
            mod_source.contains(needle),
            "docs component module mapping should keep `{needle}` for grid-list compatibility contracts.",
        );
    }
}
