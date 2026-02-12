use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_header_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar_header/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarHeader internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_header_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar_header/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarHeader;"),
        "sidebar_header module should export `SidebarHeader`.",
    );
    assert!(
        crate_source.contains("pub use sidebar_header::SidebarHeader;"),
        "crate root should re-export SidebarHeader contract.",
    );
}

#[test]
fn sidebar_header_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar_header/logic.rs");
    let view_source = load_source("src/sidebar_header/view.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SidebarHeader logic should include `{needle}`.",
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(SidebarHeaderStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarHeader view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_header_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/sidebar_header/view.rs");

    for needle in [
        "data-slot=\"sidebar-header\"",
        "data-state=move || state.get().state_attr",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-enabled=move || state.get().enabled.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "SidebarHeader should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_header_styles_include_state_markers() {
    let source = load_source("src/sidebar_header/styles.rs");

    for needle in [
        ".ui-sidebar-header {",
        ".ui-sidebar-header--disabled",
        ".ui-sidebar-header[data-disabled=\"true\"]",
        ".ui-sidebar-header--custom-class",
        ".ui-sidebar-header[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "SidebarHeader styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_header_docs_page_exists_in_layout_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn sidebar_header() -> AnyView",
        "title=\"SidebarHeader\"",
        "slug=\"sidebar-header\"",
        "<SidebarHeader",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarHeader docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn sidebar_header_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn sidebar_header() -> AnyView",
        "title=\"SidebarHeader\"",
        "slug=\"sidebar-header\"",
        "description=\"Shadcn-compatible sidebar header region primitive with centralized disabled/source-state contracts and Spectrum-style data markers.\"",
        "<Playground title=\"Default Header Region\" code=basic_code>",
        "<Playground title=\"Disabled + Custom Class\" code=disabled_code>",
        "<SidebarHeader",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs should include `{needle}` for sidebar_header primary playground coverage.",
        );
    }
}

#[test]
fn sidebar_header_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"Default Header Region\"",
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Sidebar",
        "collapsible=SidebarCollapsible::Offcanvas",
        "show_trigger=false",
        "aria_label=\"Sidebar header playground\".to_string()",
        "<SidebarHeader aria_label=\"Workspace header\".to_string()>",
        "\"Workspace\"",
        "\"5 active projects\"",
        "title=\"Disabled + Custom Class\"",
        "variant=SidebarVariant::Inset",
        "collapsible=SidebarCollapsible::Icon",
        "aria_label=\"Disabled header sidebar\".to_string()",
        "disabled=true",
        "aria_label=\"Disabled inspector header\".to_string()",
        "class_name=\"docs-sidebar-header-custom\".to_string()",
        "\"Inspector\"",
        "\"Read-only mode\"",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs playgrounds should contain `{needle}` for sidebar_header contracts.",
        );
    }
}
