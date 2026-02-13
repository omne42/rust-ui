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

#[test]
fn selection_indicator_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn listbox_item() -> AnyView",
        "title=\"ListBoxItem\"",
        "slug=\"listbox-item\"",
        "description=\"Spectrum/HeroUI-style listbox option primitive with centralized selection/focus/divider/source normalization and stable `slot` + `data-*` state contracts.\"",
        "<Playground title=\"Selectable Option\" code_signal=code>",
        "<Playground title=\"Focused + Divider + Disabled\" code_signal=states_code>",
        "<ListBoxItem",
        "pub(super) fn menu_item() -> AnyView",
        "title=\"MenuItem\"",
        "slug=\"menu-item\"",
        "description=\"Spectrum/HeroUI-style menu row primitive with centralized kind/checked/focus/source normalization and stable `slot` + `data-*` contracts.\"",
        "<Playground title=\"Action + Checkbox\" code_signal=code>",
        "<Playground title=\"Radio + Submenu + Disabled\" code_signal=states_code>",
        "<MenuItem",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra docs should include `{needle}` for selection_indicator_module primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"selection-indicator\" => &[\"listbox-item\", \"menu-item\"]"),
        "components mod mapping should keep `selection-indicator` mapped to `listbox-item` and `menu-item` slugs.",
    );
}

#[test]
fn selection_indicator_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "title=\"Selectable Option\"",
        "index=0",
        "selected=selected_default.get()",
        "show_selection_indicator=true",
        "on_press=toggle_default",
        "\"San Francisco\"",
        "title=\"Focused + Divider + Disabled\"",
        "id=\"docs-listbox-item-focused\".to_string()",
        "index=1",
        "selected=selected_states.get()",
        "focused=true",
        "has_divider=true",
        "class_name=\"docs-listbox-item-custom\".to_string()",
        "on_press=toggle_states",
        "index=2 disabled=true",
        "\"Disabled option\"",
        "title=\"Action + Checkbox\"",
        "kind=MenuItemKind::Action",
        "aria_label=\"Open profile\".to_string()",
        "kind=checkbox_kind",
        "on_press=toggle_checkbox",
        "\"Pin to favorites\"",
        "title=\"Radio + Submenu + Disabled\"",
        "id=\"docs-menu-item-radio\".to_string()",
        "index=2",
        "kind=radio_kind",
        "focused=true",
        "has_submenu=true",
        "on_press=toggle_radio",
        "class_name=\"docs-menu-item-custom\".to_string()",
        "index=3 disabled=true",
        "\"Disabled destructive action\"",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra docs playgrounds should contain `{needle}` for selection_indicator_module contracts.",
        );
    }
}
