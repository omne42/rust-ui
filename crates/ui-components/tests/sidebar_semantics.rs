use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Sidebar internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Sidebar;"),
        "sidebar module should export `Sidebar`.",
    );
    assert!(
        module_source.contains("pub use logic::{SidebarCollapsible, SidebarSide, SidebarVariant};"),
        "sidebar module should export sidebar state enums.",
    );
    assert!(
        crate_source.contains("pub use sidebar::{"),
        "crate root should re-export sidebar contracts.",
    );
}

#[test]
fn sidebar_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar/logic.rs");
    let view_source = load_source("src/sidebar/view.rs");

    for needle in [
        "pub enum SidebarSide",
        "pub enum SidebarVariant",
        "pub enum SidebarCollapsible",
        "pub struct SidebarStateInput",
        "pub struct SidebarState",
        "pub fn normalize_shortcut_key(",
        "pub fn should_toggle_for_shortcut(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Sidebar logic should include `{needle}`.",
        );
    }

    for needle in [
        "overlay_open::use_controllable_open_state(",
        "logic::normalize_shortcut_key(shortcut_key, enable_shortcut)",
        "logic::resolve_state(SidebarStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "logic::should_toggle_for_shortcut(",
    ] {
        assert!(
            view_source.contains(needle),
            "Sidebar view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/sidebar/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let is_controlled = open.is_some()",
        "overlay_open::use_controllable_open_state(",
    ] {
        assert!(
            source.contains(needle),
            "Sidebar should support `{needle}` for controllable open-state flow.",
        );
    }
}

#[test]
fn sidebar_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/sidebar/view.rs");

    for needle in [
        "data-slot=\"sidebar\"",
        "data-side=move || state.get().side_attr",
        "data-variant=move || state.get().variant_attr",
        "data-collapsible=move || state.get().collapsible_attr",
        "data-state=move || state.get().state_attr",
        "data-open=move || state.get().open.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-controls=move || state.get().control_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Sidebar should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_styles_include_panel_and_state_markers() {
    let source = load_source("src/sidebar/styles.rs");

    for needle in [
        ".ui-sidebar {",
        ".ui-sidebar__panel",
        ".ui-sidebar__trigger",
        ".ui-sidebar__rail",
        ".ui-sidebar[data-state=\"closed\"][data-collapsible=\"offcanvas\"] .ui-sidebar__panel",
        ".ui-sidebar[data-state=\"closed\"][data-collapsible=\"icon\"] .ui-sidebar__panel",
        ".ui-sidebar--disabled",
        ".ui-sidebar--custom-class",
    ] {
        assert!(
            source.contains(needle),
            "Sidebar styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_docs_page_exists_in_layout_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn sidebar() -> AnyView",
        "title=\"Sidebar\"",
        "slug=\"sidebar\"",
        "<Sidebar",
    ] {
        assert!(
            docs.contains(needle),
            "Sidebar docs page should contain `{needle}`.",
        );
    }
}
