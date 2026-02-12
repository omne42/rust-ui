use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_menu_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar_menu/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarMenu internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_menu_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar_menu/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarMenu;"),
        "sidebar_menu module should export `SidebarMenu`.",
    );
    assert!(
        module_source.contains("SidebarMenuItem") && module_source.contains("SidebarMenuSubItem"),
        "sidebar_menu module should export menu data contracts.",
    );
    assert!(
        crate_source.contains("pub use sidebar_menu::{SidebarMenu, SidebarMenuItem, SidebarMenuMotion, SidebarMenuSubItem};"),
        "crate root should re-export SidebarMenu contracts.",
    );
}

#[test]
fn sidebar_menu_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar_menu/logic.rs");
    let view_source = load_source("src/sidebar_menu/view.rs");

    for needle in [
        "pub struct SidebarMenuItem",
        "pub struct SidebarMenuSubItem",
        "pub struct SidebarMenuState",
        "pub fn normalize_items(",
        "pub fn default_open_sub_ids(",
        "pub fn default_active_id(",
        "pub fn next_id_for_key(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SidebarMenu logic should include `{needle}`.",
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "logic::normalize_items(items)",
        "logic::default_open_sub_ids(items.as_ref())",
        "logic::default_active_id(items.as_ref(), default_active_id)",
        "logic::resolve_state(",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "logic::next_id_for_key(",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarMenu view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_menu_supports_controlled_and_uncontrolled_active_state() {
    let source = load_source("src/sidebar_menu/view.rs");

    for needle in [
        "active_id: Option<Signal<Option<String>>>",
        "default_active_id: Option<String>",
        "on_active_id_change: Option<Callback<Option<String>>>",
        "let is_controlled = active_id.is_some()",
        "overlay_open::use_controllable_state(",
    ] {
        assert!(
            source.contains(needle),
            "SidebarMenu should support `{needle}` for controllable active-state flow.",
        );
    }
}

#[test]
fn sidebar_menu_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/sidebar_menu/view.rs");

    for needle in [
        "data-slot=\"sidebar-menu\"",
        "data-state=move || state.get().state_attr",
        "data-count=item_count.to_string()",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-show-badges=move || state.get().show_badges.then_some(\"true\")",
        "data-show-actions=move || state.get().show_actions.then_some(\"true\")",
        "data-collapsible-sub=move || state.get().allow_submenu_collapse.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-active-id=move || active_id.get().unwrap_or_default()",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "SidebarMenu should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_menu_styles_include_core_and_submenu_markers() {
    let source = load_source("src/sidebar_menu/styles.rs");

    for needle in [
        ".ui-sidebar-menu {",
        ".ui-sidebar-menu__highlight",
        ".ui-sidebar-menu__item-main",
        ".ui-sidebar-menu__badge",
        ".ui-sidebar-menu__action",
        ".ui-sidebar-menu__toggle[data-open=\"true\"]",
        ".ui-sidebar-menu__sub",
        ".ui-sidebar-menu__sub-button",
        ".ui-sidebar-menu--disabled",
        ".ui-sidebar-menu--custom-class",
    ] {
        assert!(
            source.contains(needle),
            "SidebarMenu styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_menu_docs_page_exists_in_layout_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn sidebar_menu() -> AnyView",
        "title=\"SidebarMenu\"",
        "slug=\"sidebar-menu\"",
        "<SidebarMenu",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarMenu docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn sidebar_menu_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn sidebar_menu() -> AnyView",
        "title=\"SidebarMenu\"",
        "slug=\"sidebar-menu\"",
        "description=\"Shadcn-compatible sidebar menu primitive with badges/actions/sub-items, controlled active-id flow, collapsible submenu behavior, Spectrum-style data contracts, and HeroUI-level active-highlight motion.\"",
        "<Playground title=\"Badge + Item Action\" code=badge_code>",
        "<Playground title=\"Controlled + Collapsible Submenu\" code=controlled_code>",
        "<SidebarMenu",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs should include `{needle}` for sidebar_menu primary playground coverage.",
        );
    }
}

#[test]
fn sidebar_menu_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"Badge + Item Action\"",
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Sidebar",
        "collapsible=SidebarCollapsible::Offcanvas",
        "show_trigger=false",
        "aria_label=\"Menu playground sidebar\".to_string()",
        "id_base=\"docs-sidebar-menu-badge\".to_string()",
        "items=badge_items",
        "on_action=on_action",
        "on_item_action=on_item_action",
        "aria_label=\"Primary menu\".to_string()",
        "\"Action: \"",
        "\" · Item action: \"",
        "title=\"Controlled + Collapsible Submenu\"",
        "variant=SidebarVariant::Inset",
        "collapsible=SidebarCollapsible::Icon",
        "aria_label=\"Controlled menu sidebar\".to_string()",
        "id_base=\"docs-sidebar-menu-controlled\".to_string()",
        "items=collapsible_items",
        "active_id=active",
        "on_active_id_change=on_active_change",
        "allow_submenu_collapse=true",
        "show_badges=false",
        "show_actions=true",
        "keyboard_shortcut_key=\"k\".to_string()",
        "aria_label=\"Workspace menu\".to_string()",
        "class_name=\"docs-sidebar-menu-custom\".to_string()",
        "\"active: \"",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs playgrounds should contain `{needle}` for sidebar_menu contracts.",
        );
    }
}
