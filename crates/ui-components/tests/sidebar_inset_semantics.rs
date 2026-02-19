use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_inset_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar/inset/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarInset internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_inset_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar/inset/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarInset;"),
        "sidebar_inset module should export `SidebarInset`.",
    );
    assert!(
        crate_source.contains("pub use sidebar_inset::SidebarInset;"),
        "crate root should re-export SidebarInset contract.",
    );
}

#[test]
fn sidebar_inset_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar/inset/logic.rs");
    let view_source = load_source("src/sidebar/inset/view.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SidebarInset logic should include `{needle}`.",
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(SidebarInsetStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarInset view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_inset_emits_baseline_root_state_data_attributes() {
    let source = load_source("src/sidebar/inset/view.rs");

    for needle in [
        "data-slot=\"sidebar-inset\"",
        "data-side=move || state.get().side_attr",
        "data-state=move || state.get().state_attr",
        "data-padding=move || state.get().padding_attr",
        "data-surface=move || state.get().surface_attr",
        "data-recessed=move || state.get().recessed.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "SidebarInset should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_inset_styles_include_state_markers() {
    let source = load_source("src/sidebar/inset/styles.rs");

    for needle in [
        ".ui-sidebar-inset {",
        ".ui-sidebar-inset--padded",
        ".ui-sidebar-inset[data-padded=\"true\"]",
        ".ui-sidebar-inset--recessed",
        ".ui-sidebar-inset[data-recessed=\"true\"]",
        ".ui-sidebar-inset--right",
        ".ui-sidebar-inset[data-side=\"right\"]",
        ".ui-sidebar-inset--custom-class",
        ".ui-sidebar-inset[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "SidebarInset styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_inset_docs_page_exists_in_layout_extra_modules() {
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_inset.rs");

    assert!(
        layout_extra.contains("pub(super) fn sidebar_inset() -> AnyView"),
        "layout_extra should expose sidebar_inset route entry.",
    );

    for needle in [
        "pub(super) fn sidebar_inset() -> AnyView",
        "title=\"SidebarInset\"",
        "slug=\"sidebar-inset\"",
        "<SidebarInset",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarInset docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn sidebar_inset_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_inset.rs");

    for needle in [
        "pub(super) fn sidebar_inset() -> AnyView",
        "title=\"SidebarInset\"",
        "slug=\"sidebar-inset\"",
        "description=\"baseline-compatible sidebar inset primitive with side/padding/surface contracts and baseline-style root data markers.\"",
        "<Playground title=\"Default Inset Region\" code_signal=default_code>",
        "<Playground title=\"Compact + Plain + Disabled\" code_signal=compact_code>",
        "<SidebarInset",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_inset docs should include `{needle}` for sidebar_inset primary playground coverage.",
        );
    }
}

#[test]
fn sidebar_inset_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_inset.rs");

    for needle in [
        "title=\"Default Inset Region\"",
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Inset",
        "collapsible=SidebarCollapsible::Icon",
        "show_trigger=false",
        "aria_label=\"Sidebar inset playground\".to_string()",
        "<SidebarInset aria_label=\"Workspace inset region\".to_string()>",
        "\"Overview\"",
        "\"Recent activity\"",
        "\"Pinned links\"",
        "title=\"Compact + Plain + Disabled\"",
        "side=SidebarSide::Right",
        "collapsible=SidebarCollapsible::Offcanvas",
        "aria_label=\"Inspector sidebar inset\".to_string()",
        "padded=false",
        "recessed=false",
        "disabled=true",
        "aria_label=\"Inspector inset panel\".to_string()",
        "class_name=\"docs-sidebar-inset-custom\".to_string()",
        "\"Read-only\"",
        "\"3 warnings\"",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_inset docs playgrounds should contain `{needle}` for sidebar_inset contracts.",
        );
    }
}
