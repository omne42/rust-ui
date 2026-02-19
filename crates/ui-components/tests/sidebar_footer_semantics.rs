use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_footer_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar/footer/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarFooter internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_footer_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar/footer/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarFooter;"),
        "sidebar_footer module should export `SidebarFooter`.",
    );
    assert!(
        crate_source.contains("pub use sidebar_footer::SidebarFooter;"),
        "crate root should re-export SidebarFooter contract.",
    );
}

#[test]
fn sidebar_footer_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar/footer/logic.rs");
    let view_source = load_source("src/sidebar/footer/view.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SidebarFooter logic should include `{needle}`.",
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(SidebarFooterStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarFooter view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_footer_emits_baseline_root_state_data_attributes() {
    let source = load_source("src/sidebar/footer/view.rs");

    for needle in [
        "data-slot=\"sidebar-footer\"",
        "data-state=move || state.get().state_attr",
        "data-border=move || state.get().border_attr",
        "data-bordered=move || state.get().bordered.then_some(\"true\")",
        "data-unbordered=move || state.get().unbordered.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-enabled=move || state.get().enabled.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "SidebarFooter should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_footer_styles_include_state_markers() {
    let source = load_source("src/sidebar/footer/styles.rs");

    for needle in [
        ".ui-sidebar-footer {",
        ".ui-sidebar-footer--bordered",
        ".ui-sidebar-footer[data-bordered=\"true\"]",
        ".ui-sidebar-footer--disabled",
        ".ui-sidebar-footer[data-disabled=\"true\"]",
        ".ui-sidebar-footer--custom-class",
        ".ui-sidebar-footer[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "SidebarFooter styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_footer_docs_page_exists_in_layout_extra_modules() {
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_footer.rs",
    );

    assert!(
        layout_extra.contains("pub(super) fn sidebar_footer() -> AnyView"),
        "layout_extra should expose sidebar_footer route entry.",
    );

    for needle in [
        "pub(super) fn sidebar_footer() -> AnyView",
        "title=\"SidebarFooter\"",
        "slug=\"sidebar-footer\"",
        "<SidebarFooter",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarFooter docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn sidebar_footer_docs_page_covers_primary_playgrounds() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_footer.rs",
    );

    for needle in [
        "pub(super) fn sidebar_footer() -> AnyView",
        "title=\"SidebarFooter\"",
        "slug=\"sidebar-footer\"",
        "description=\"baseline-compatible sidebar footer region primitive with centralized border/disabled/source-state contracts and baseline-style data markers.\"",
        "<Playground title=\"Default Footer Region\" code_signal=default_code>",
        "<Playground title=\"Disabled + Custom Class\" code_signal=disabled_code>",
        "<SidebarFooter",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_footer docs should include `{needle}` for sidebar_footer primary playground coverage.",
        );
    }
}

#[test]
fn sidebar_footer_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_footer.rs",
    );

    for needle in [
        "title=\"Default Footer Region\"",
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Sidebar",
        "collapsible=SidebarCollapsible::Offcanvas",
        "show_trigger=false",
        "aria_label=\"Sidebar footer playground\".to_string()",
        "<SidebarFooter bordered=true aria_label=\"Workspace footer\".to_string()>",
        "\"Free plan\"",
        "\"2 seats remaining\"",
        "title=\"Disabled + Custom Class\"",
        "variant=SidebarVariant::Inset",
        "collapsible=SidebarCollapsible::Icon",
        "aria_label=\"Disabled footer sidebar\".to_string()",
        "disabled=true",
        "bordered=true",
        "aria_label=\"Disabled usage footer\".to_string()",
        "class_name=\"docs-sidebar-footer-custom\".to_string()",
        "\"Read-only quota\"",
        "\"Upgrade required\"",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_footer docs playgrounds should contain `{needle}` for sidebar_footer contracts.",
        );
    }
}
