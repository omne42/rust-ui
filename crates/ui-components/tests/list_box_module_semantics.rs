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

#[test]
fn list_box_module_docs_page_covers_primary_playgrounds() {
    let listbox_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let item_section_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
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
            listbox_source.contains(needle),
            "collections docs should include `{needle}` for list-box module primary listbox playground coverage.",
        );
    }

    for needle in [
        "pub(super) fn listbox_item() -> AnyView",
        "title=\"ListBoxItem\"",
        "slug=\"listbox-item\"",
        "<Playground title=\"Selectable Option\" code=code>",
        "<Playground title=\"Focused + Divider + Disabled\" code=states_code>",
        "<ListBoxItem",
        "pub(super) fn listbox_section() -> AnyView",
        "title=\"ListBoxSection\"",
        "slug=\"listbox-section\"",
        "<Playground title=\"Default Section\" code=code>",
        "<Playground title=\"Quiet + Sticky + Divider + Empty\" code=states_code>",
        "<ListBoxSection",
    ] {
        assert!(
            item_section_source.contains(needle),
            "collections_extra docs should include `{needle}` for list-box module primary item/section playground coverage.",
        );
    }

    assert!(
        mod_source
            .contains("\"list-box\" => &[\"listbox\", \"listbox-item\", \"listbox-section\"]"),
        "components mod mapping should keep `list-box` mapped to listbox/listbox-item/listbox-section slugs.",
    );
}

#[test]
fn list_box_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let listbox_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let item_section_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

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
            "collections docs playgrounds should contain `{needle}` for list-box module listbox contracts.",
        );
    }

    for needle in [
        "title=\"Selectable Option\"",
        "index=0",
        "show_selection_indicator=true",
        "\"San Francisco\"",
        "title=\"Focused + Divider + Disabled\"",
        "id=\"docs-listbox-item-focused\".to_string()",
        "focused=true",
        "has_divider=true",
        "class_name=\"docs-listbox-item-custom\".to_string()",
        "\"Disabled option\"",
        "title=\"Default Section\"",
        "title=\"Preferred regions\".to_string()",
        "item_count=3",
        "aria_label=\"Preferred regions section\".to_string()",
        "title=\"Quiet + Sticky + Divider + Empty\"",
        "heading_tone=ListBoxSectionHeadingTone::Quiet",
        "sticky_heading=true",
        "show_divider=true",
        "class_name=\"docs-listbox-section-custom\".to_string()",
        "title=\"Empty section\".to_string()",
        "item_count=0",
        "disabled=true",
        "\"No options available\"",
    ] {
        assert!(
            item_section_source.contains(needle),
            "collections_extra docs playgrounds should contain `{needle}` for list-box module item/section contracts.",
        );
    }
}
