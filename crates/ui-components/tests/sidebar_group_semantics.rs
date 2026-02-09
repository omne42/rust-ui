use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarGroup internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_group_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar_group/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarGroup;"),
        "sidebar_group module should export `SidebarGroup`.",
    );
    assert!(
        crate_source.contains("pub use sidebar_group::SidebarGroup;"),
        "crate root should re-export SidebarGroup contract.",
    );
}

#[test]
fn sidebar_group_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar_group/logic.rs");
    let view_source = load_source("src/sidebar_group/view.rs");

    for needle in [
        "pub struct SidebarGroupStateInput",
        "pub struct SidebarGroupState",
        "pub fn normalize_label(",
        "pub fn normalize_action_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SidebarGroup logic should include `{needle}`.",
        );
    }

    for needle in [
        "overlay_open::use_controllable_open_state(",
        "logic::resolve_state(SidebarGroupStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarGroup view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_group_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/sidebar_group/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let is_controlled = open.is_some()",
        "overlay_open::use_controllable_open_state(",
    ] {
        assert!(
            source.contains(needle),
            "SidebarGroup should support `{needle}` for controllable open-state flow.",
        );
    }
}

#[test]
fn sidebar_group_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/sidebar_group/view.rs");

    for needle in [
        "data-slot=\"sidebar-group\"",
        "data-state=move || state.get().state_attr",
        "data-open=move || state.get().open.then_some(\"true\")",
        "data-collapsible=move || state.get().collapse_attr",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-show-label=move || state.get().show_label.then_some(\"true\")",
        "data-show-action=move || state.get().show_action.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-control-mode=move || state.get().control_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "SidebarGroup should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_group_styles_include_header_content_and_state_markers() {
    let source = load_source("src/sidebar_group/styles.rs");

    for needle in [
        ".ui-sidebar-group {",
        ".ui-sidebar-group__header",
        ".ui-sidebar-group__label",
        ".ui-sidebar-group__action",
        ".ui-sidebar-group__toggle[data-open=\"true\"]",
        ".ui-sidebar-group__content",
        ".ui-sidebar-group--label-hidden",
        ".ui-sidebar-group--disabled",
        ".ui-sidebar-group--custom-class",
    ] {
        assert!(
            source.contains(needle),
            "SidebarGroup styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_group_docs_page_exists_in_layout_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn sidebar_group() -> AnyView",
        "title=\"SidebarGroup\"",
        "slug=\"sidebar-group\"",
        "<SidebarGroup",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarGroup docs page should contain `{needle}`.",
        );
    }
}
