use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn navigation_menu_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/navigation_menu/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::NavigationMenu;"),
        "navigation_menu module should export `NavigationMenu`."
    );
    assert!(
        module_source.contains("pub use logic::NavigationMenuItem;"),
        "navigation_menu module should export `NavigationMenuItem`."
    );
    assert!(
        module_source.contains("NavigationMenuMotion"),
        "navigation_menu module should expose a motion alias."
    );
    assert!(
        crate_source.contains(
            "pub use navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuMotion};"
        ),
        "crate root should re-export navigation_menu contracts."
    );
}

#[test]
fn navigation_menu_uses_logic_state_model() {
    let view_source = load_source("src/navigation_menu/view.rs");
    let logic_source = load_source("src/navigation_menu/logic.rs");

    for needle in [
        "pub struct NavigationMenuItem",
        "pub struct NavigationMenuItemResolved",
        "pub struct NavigationMenuStateInput",
        "pub struct NavigationMenuState",
        "pub fn resolve_items(",
        "pub fn sanitize_selected_id(",
        "pub fn sanitize_focused_index(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn next_enabled_index(",
    ] {
        assert!(
            logic_source.contains(needle),
            "NavigationMenu logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let items = logic::resolve_items(&id_base, items);",
        "let selected_state = overlay_open::use_controllable_state(",
        "attach_active_highlight_motion(list_ref, highlight_ref, active_index, option_id, motion);",
        "let state = Signal::derive(move ||",
        "logic::resolve_state(logic::NavigationMenuStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "NavigationMenu view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn navigation_menu_supports_controlled_and_uncontrolled_selection_state() {
    let source = load_source("src/navigation_menu/view.rs");

    for needle in [
        "selected_id: Option<Signal<Option<String>>>",
        "default_selected_id: Option<String>",
        "on_selected_id_change: Option<Callback<Option<String>>>",
        "activate_on_focus: bool",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu should accept `{needle}` for controlled/uncontrolled selection behavior."
        );
    }
}

#[test]
fn navigation_menu_exposes_keyboard_and_focus_contracts() {
    let source = load_source("src/navigation_menu/view.rs");

    for needle in [
        "on:keydown=on_key_down",
        "on:focus=on_focus",
        "on:pointerenter=on_pointer_enter",
        "logic::next_enabled_index(items.get_value().as_ref(), index, 1)",
        "logic::next_enabled_index(items.get_value().as_ref(), index, -1)",
        "logic::first_enabled_index(items.get_value().as_ref())",
        "logic::last_enabled_index(items.get_value().as_ref())",
        "focus_item(&item_refs, next_index);",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu should wire `{needle}` to match keyboard and roving focus semantics."
        );
    }
}

#[test]
fn navigation_menu_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/navigation_menu/view.rs");

    for needle in [
        "data-slot=\"navigation-menu\"",
        "data-state=move || state.get().data_state_attr",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-item-count=move || state.get().item_count.to_string()",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-focused-index=move || state.get().focused_index.map(|index| index.to_string())",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-has-focus=move || state.get().has_focus.then_some(\"true\")",
        "data-has-disabled-items=move || state.get().has_disabled_items.then_some(\"true\")",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "data-selected-id=move || selected_id.get()",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu should set `{needle}` so it can be styled/tested with Spectrum-compatible selectors."
        );
    }
}

#[test]
fn navigation_menu_uses_active_highlight_motion_contract() {
    let source = load_source("src/navigation_menu/view.rs");

    for needle in [
        "use crate::active_highlight::{ActiveHighlightMotion, attach_active_highlight_motion};",
        "let list_ref: NodeRef<html::Div> = NodeRef::new();",
        "let highlight_ref: NodeRef<html::Div> = NodeRef::new();",
        "let (active_index, set_active_index) = signal(",
        "attach_active_highlight_motion(list_ref, highlight_ref, active_index, option_id, motion);",
        "data-slot=\"navigation-menu-highlight\"",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu should compose active highlight motion via `{needle}` for HeroUI-level feedback continuity."
        );
    }
}

#[test]
fn navigation_menu_styles_include_selected_disabled_and_empty_markers() {
    let source = load_source("src/navigation_menu/styles.rs");

    for needle in [
        ".ui-navigation-menu {",
        ".ui-navigation-menu[data-motion-source=\"custom\"]",
        ".ui-navigation-menu[data-custom-motion=\"true\"]",
        ".ui-navigation-menu__list {",
        ".ui-navigation-menu__item[data-selected=\"true\"]",
        ".ui-navigation-menu__item[data-disabled=\"true\"]",
        ".ui-navigation-menu--empty",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu styles should include `{needle}` for stable visual state contracts."
        );
    }
}
