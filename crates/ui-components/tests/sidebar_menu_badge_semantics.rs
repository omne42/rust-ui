use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_menu_badge_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar_menu_badge/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarMenuBadge internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_menu_badge_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar_menu_badge/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarMenuBadge;"),
        "sidebar_menu_badge module should export `SidebarMenuBadge`.",
    );
    assert!(
        crate_source.contains("pub use sidebar_menu_badge::SidebarMenuBadge;"),
        "crate root should re-export SidebarMenuBadge contract.",
    );
}

#[test]
fn sidebar_menu_badge_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar_menu_badge/logic.rs");
    let view_source = load_source("src/sidebar_menu_badge/view.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SidebarMenuBadge logic should include `{needle}`.",
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(SidebarMenuBadgeStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarMenuBadge view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_menu_badge_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/sidebar_menu_badge/view.rs");

    for needle in [
        "data-slot=\"sidebar-menu-badge\"",
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-muted=move || state.get().muted.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-enabled=move || state.get().enabled.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "SidebarMenuBadge should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_menu_badge_styles_include_state_markers() {
    let source = load_source("src/sidebar_menu_badge/styles.rs");

    for needle in [
        ".ui-sidebar-menu-badge {",
        ".ui-sidebar-menu-badge--muted",
        ".ui-sidebar-menu-badge[data-muted=\"true\"]",
        ".ui-sidebar-menu-badge--disabled",
        ".ui-sidebar-menu-badge[data-disabled=\"true\"]",
        ".ui-sidebar-menu-badge--custom-class",
        ".ui-sidebar-menu-badge[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "SidebarMenuBadge styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_menu_badge_docs_page_exists_in_layout_extra_modules() {
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_menu_badge.rs",
    );

    assert!(
        layout_extra.contains("pub(super) fn sidebar_menu_badge() -> AnyView"),
        "layout_extra should expose sidebar_menu_badge route entry.",
    );

    for needle in [
        "pub(super) fn sidebar_menu_badge() -> AnyView",
        "title=\"SidebarMenuBadge\"",
        "slug=\"sidebar-menu-badge\"",
        "<SidebarMenuBadge",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarMenuBadge docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn sidebar_menu_badge_docs_page_covers_primary_playgrounds() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_menu_badge.rs",
    );

    for needle in [
        "pub(super) fn sidebar_menu_badge() -> AnyView",
        "title=\"SidebarMenuBadge\"",
        "slug=\"sidebar-menu-badge\"",
        "description=\"Shadcn-compatible sidebar menu badge primitive with centralized tone/disabled/source-state normalization and stable data-marker contracts.\"",
        "<Playground title=\"Default Numeric Badge\" code_signal=default_code>",
        "<Playground title=\"Muted + Disabled + Custom\" code_signal=muted_code>",
        "<SidebarMenuBadge",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_menu_badge docs should include `{needle}` for sidebar_menu_badge primary playground coverage.",
        );
    }
}

#[test]
fn sidebar_menu_badge_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_menu_badge.rs",
    );

    for needle in [
        "title=\"Default Numeric Badge\"",
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Sidebar",
        "collapsible=SidebarCollapsible::Icon",
        "show_trigger=false",
        "aria_label=\"Sidebar menu badge playground\".to_string()",
        "<SidebarContent aria_label=\"Sidebar badge rows\".to_string()>",
        "aria_label=\"Open reviews\".to_string()",
        "\"Open reviews\"",
        "\"7\"",
        "aria_label=\"Deploy requests\".to_string()",
        "\"Deploy requests\"",
        "\"2\"",
        "title=\"Muted + Disabled + Custom\"",
        "side=SidebarSide::Right",
        "variant=SidebarVariant::Inset",
        "collapsible=SidebarCollapsible::Offcanvas",
        "aria_label=\"Muted badge sidebar\".to_string()",
        "<SidebarContent aria_label=\"Muted badge rows\".to_string()>",
        "muted=true",
        "disabled=true",
        "aria_label=\"Muted archived items\".to_string()",
        "class_name=\"docs-sidebar-menu-badge-custom\".to_string()",
        "\"Archived items\"",
        "\"archived\"",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_menu_badge docs playgrounds should contain `{needle}` for sidebar_menu_badge contracts.",
        );
    }
}
