use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_menu_action_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar_menu_action/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarMenuAction internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_menu_action_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar_menu_action/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarMenuAction;"),
        "sidebar_menu_action module should export `SidebarMenuAction`.",
    );
    assert!(
        crate_source.contains("pub use sidebar_menu_action::SidebarMenuAction;"),
        "crate root should re-export SidebarMenuAction contract.",
    );
}

#[test]
fn sidebar_menu_action_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar_menu_action/logic.rs");
    let view_source = load_source("src/sidebar_menu_action/view.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SidebarMenuAction logic should include `{needle}`.",
        );
    }

    for needle in [
        "logic::normalize_label(label)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(SidebarMenuActionStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarMenuAction view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_menu_action_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/sidebar_menu_action/view.rs");

    for needle in [
        "data-slot=\"sidebar-menu-action\"",
        "data-state=move || state.get().state_attr",
        "data-visibility=move || state.get().visibility_attr",
        "data-hover-only=move || state.get().hover_only.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-enabled=move || state.get().enabled.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "SidebarMenuAction should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_menu_action_styles_include_state_markers() {
    let source = load_source("src/sidebar_menu_action/styles.rs");

    for needle in [
        ".ui-sidebar-menu-action {",
        ".ui-sidebar-menu-action--hover-only",
        ".ui-sidebar-menu-action[data-hover-only=\"true\"]",
        ".ui-sidebar-menu-action--disabled",
        ".ui-sidebar-menu-action[data-disabled=\"true\"]",
        ".ui-sidebar-menu-action--custom-class",
        ".ui-sidebar-menu-action[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "SidebarMenuAction styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_menu_action_docs_page_exists_in_layout_extra_modules() {
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_menu_action.rs",
    );

    assert!(
        layout_extra.contains("pub(super) fn sidebar_menu_action() -> AnyView"),
        "layout_extra should expose sidebar_menu_action route entry.",
    );

    for needle in [
        "pub(super) fn sidebar_menu_action() -> AnyView",
        "title=\"SidebarMenuAction\"",
        "slug=\"sidebar-menu-action\"",
        "<SidebarMenuAction",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarMenuAction docs page should contain `{needle}`.",
        );
    }
}
