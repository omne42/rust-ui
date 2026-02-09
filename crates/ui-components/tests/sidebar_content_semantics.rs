use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_content_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar_content/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarContent internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_content_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar_content/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarContent;"),
        "sidebar_content module should export `SidebarContent`.",
    );
    assert!(
        crate_source.contains("pub use sidebar_content::SidebarContent;"),
        "crate root should re-export SidebarContent contract.",
    );
}

#[test]
fn sidebar_content_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar_content/logic.rs");
    let view_source = load_source("src/sidebar_content/view.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SidebarContent logic should include `{needle}`.",
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(SidebarContentStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarContent view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_content_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/sidebar_content/view.rs");

    for needle in [
        "data-slot=\"sidebar-content\"",
        "data-state=move || state.get().state_attr",
        "data-padding=move || state.get().padding_attr",
        "data-scroll=move || state.get().scroll_attr",
        "data-padded=move || state.get().padded.then_some(\"true\")",
        "data-compact=move || state.get().compact.then_some(\"true\")",
        "data-scrollable=move || state.get().scrollable.then_some(\"true\")",
        "data-static=move || state.get().static_layout.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-enabled=move || state.get().enabled.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "SidebarContent should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_content_styles_include_state_markers() {
    let source = load_source("src/sidebar_content/styles.rs");

    for needle in [
        ".ui-sidebar-content {",
        ".ui-sidebar-content--padded",
        ".ui-sidebar-content[data-padded=\"true\"]",
        ".ui-sidebar-content--scrollable",
        ".ui-sidebar-content[data-scrollable=\"true\"]",
        ".ui-sidebar-content--disabled",
        ".ui-sidebar-content[data-disabled=\"true\"]",
        ".ui-sidebar-content--custom-class",
        ".ui-sidebar-content[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "SidebarContent styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_content_docs_page_exists_in_layout_extra_modules() {
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_content.rs",
    );

    assert!(
        layout_extra.contains("pub(super) fn sidebar_content() -> AnyView"),
        "layout_extra should expose sidebar_content route entry.",
    );

    for needle in [
        "pub(super) fn sidebar_content() -> AnyView",
        "title=\"SidebarContent\"",
        "slug=\"sidebar-content\"",
        "<SidebarContent",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarContent docs page should contain `{needle}`.",
        );
    }
}
