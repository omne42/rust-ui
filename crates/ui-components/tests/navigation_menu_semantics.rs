use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn navigation_menu_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/navigation_menu/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "NavigationMenu internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn navigation_menu_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/navigation_menu/mod.rs");

    for needle in [
        "pub struct NavigationMenuItem",
        "pub struct NavigationMenuItemResolved",
        "pub enum NavigationMenuSlot",
        "pub struct NavigationMenuPartStateInput",
        "pub struct NavigationMenuPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_ACTIVATE_ON_FOCUS",
        "pub use crate::active_highlight::ActiveHighlightMotion as NavigationMenuMotion;",
    ] {
        assert!(
            source.contains(needle),
            "navigation_menu::mod should include `{needle}` contracts."
        );
    }
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
        crate_source.contains(
            "pub use navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuMotion};"
        ),
        "crate root should re-export navigation_menu contracts."
    );
}

#[test]
fn navigation_menu_logic_exposes_state_helpers() {
    let source = load_source("src/navigation_menu/logic.rs");

    for needle in [
        "pub fn state_attr(item_count: usize, has_selection: bool, has_focus: bool)",
        "pub fn item_attr(item_count: usize)",
        "pub fn selected_attr(has_selection: bool)",
        "pub fn focus_attr(has_focus: bool)",
        "pub fn focus_activation_attr(activate_on_focus: bool)",
        "pub fn selection_mode_attr(is_controlled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_id_base(id_base: String)",
        "pub fn resolve_aria_label(value: Option<String>)",
        "pub fn resolve_items(",
        "pub fn sanitize_selected_id(",
        "pub fn sanitize_focused_index(",
        "pub fn resolve_state(input: NavigationMenuPartStateInput) -> NavigationMenuPartState",
        "pub fn compose_class_name(",
        "pub fn next_enabled_index(",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn navigation_menu_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/navigation_menu/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_items(&id_base.get_value(), items)",
        "logic::resolve_state(NavigationMenuPartStateInput {",
        "slot: NavigationMenuSlot::Root",
        "logic::compose_class_name(class_name.get_value(), root_state_for_class.get())",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-items=move || root_state.get().item_attr",
        "data-selection=move || root_state.get().selected_attr",
        "data-focus=move || root_state.get().focus_attr",
        "data-focus-activation=move || root_state.get().focus_activation_attr",
        "data-selection-mode=move || root_state.get().selection_mode_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-activate-on-focus-source=move || root_state.get().activate_on_focus_source_attr",
        "data-selected-id-source=move || root_state.get().selected_id_source_attr",
        "data-default-selected-id-source=move || root_state.get().default_selected_id_source_attr",
        "data-selected-id-change-source=move || root_state.get().selected_id_change_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-custom-id=move || root_state.get().has_custom_id_base.then_some(\"true\")",
        "data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some(\"true\")",
        "data-custom-selected-id=move || root_state.get().has_custom_selected_id.then_some(\"true\")",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu view should include `{needle}` for stable state/source marker contracts."
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
        "let has_custom_selected_id = selected_id.is_some()",
        "let has_custom_default_selected_id = default_selected_id.is_some()",
        "overlay_open::use_controllable_state(",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu should support `{needle}` for controlled/uncontrolled selection behavior."
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
fn navigation_menu_uses_active_highlight_motion_contract() {
    let source = load_source("src/navigation_menu/view.rs");

    for needle in [
        "use crate::active_highlight::{",
        "attach_active_highlight_motion",
        "ActiveHighlightMotion",
        "let list_ref: NodeRef<html::Div> = NodeRef::new();",
        "let highlight_ref: NodeRef<html::Div> = NodeRef::new();",
        "let (active_index, set_active_index) = signal(",
        "attach_active_highlight_motion(list_ref, highlight_ref, active_index, option_id, motion);",
        "data-slot=highlight_slot.as_attr()",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu should compose active highlight motion via `{needle}` for HeroUI-level feedback continuity."
        );
    }
}

#[test]
fn navigation_menu_styles_include_state_and_source_markers() {
    let source = load_source("src/navigation_menu/styles.rs");

    for needle in [
        ".ui-navigation-menu {",
        ".ui-navigation-menu--selected",
        ".ui-navigation-menu[data-state=\"selected\"]",
        ".ui-navigation-menu--manual-activation",
        ".ui-navigation-menu[data-focus-activation=\"manual\"]",
        ".ui-navigation-menu[data-selection-mode=\"controlled\"]",
        ".ui-navigation-menu[data-id-source=\"custom\"]",
        ".ui-navigation-menu[data-aria-label-source=\"custom\"]",
        ".ui-navigation-menu[data-selected-id-source=\"custom\"]",
        ".ui-navigation-menu[data-selected-id-change-source=\"custom\"]",
        ".ui-navigation-menu[data-motion-source=\"custom\"]",
        ".ui-navigation-menu[data-custom-motion=\"true\"]",
        ".ui-navigation-menu__item[data-state=\"selected\"]",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu styles should include `{needle}` for stable state/source contracts."
        );
    }
}

#[test]
fn navigation_menu_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn navigation_menu() -> AnyView",
        "title=\"NavigationMenu\"",
        "slug=\"navigation-menu\"",
        "State + Source Markers",
        "data-id-source",
        "data-aria-label-source",
        "data-activate-on-focus-source",
        "data-selected-id-source",
        "data-selected-id-change-source",
        "data-motion-source",
        "<NavigationMenu",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu docs page should contain `{needle}`."
        );
    }
}
