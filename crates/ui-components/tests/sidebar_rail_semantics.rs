use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_rail_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar_rail/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarRail internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_rail_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar_rail/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarRail;"),
        "sidebar_rail module should export `SidebarRail`.",
    );
    assert!(
        crate_source.contains("pub use sidebar_rail::SidebarRail;"),
        "crate root should re-export SidebarRail contract.",
    );
}

#[test]
fn sidebar_rail_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar_rail/logic.rs");
    let view_source = load_source("src/sidebar_rail/view.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_label(",
        "pub fn normalize_default_open(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SidebarRail logic should include `{needle}`.",
        );
    }

    for needle in [
        "overlay_open::use_controllable_open_state(",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_label(label)",
        "logic::resolve_state(SidebarRailStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarRail view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_rail_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/sidebar_rail/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let is_controlled = open.is_some()",
        "overlay_open::use_controllable_open_state(",
    ] {
        assert!(
            source.contains(needle),
            "SidebarRail should support `{needle}` for controllable open-state flow.",
        );
    }
}

#[test]
fn sidebar_rail_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/sidebar_rail/view.rs");

    for needle in [
        "data-slot=\"sidebar-rail\"",
        "data-state=move || state.get().state_attr",
        "data-side=move || state.get().side_attr",
        "data-open=move || state.get().open.then_some(\"true\")",
        "data-closed=move || state.get().closed.then_some(\"true\")",
        "data-control-mode=move || state.get().control_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "SidebarRail should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_rail_styles_include_state_markers() {
    let source = load_source("src/sidebar_rail/styles.rs");

    for needle in [
        ".ui-sidebar-rail {",
        ".ui-sidebar-rail--right",
        ".ui-sidebar-rail[data-side=\"right\"]",
        ".ui-sidebar-rail--closed",
        ".ui-sidebar-rail[data-closed=\"true\"]",
        ".ui-sidebar-rail--disabled",
        ".ui-sidebar-rail[data-disabled=\"true\"]",
        ".ui-sidebar-rail--custom-class",
        ".ui-sidebar-rail[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "SidebarRail styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_rail_docs_page_exists_in_layout_extra_modules() {
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_rail.rs");

    assert!(
        layout_extra.contains("pub(super) fn sidebar_rail() -> AnyView"),
        "layout_extra should expose sidebar_rail route entry.",
    );

    for needle in [
        "pub(super) fn sidebar_rail() -> AnyView",
        "title=\"SidebarRail\"",
        "slug=\"sidebar-rail\"",
        "<SidebarRail",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarRail docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn sidebar_rail_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_rail.rs");

    for needle in [
        "pub(super) fn sidebar_rail() -> AnyView",
        r#"title="SidebarRail""#,
        r#"slug="sidebar-rail""#,
        r#"description="Shadcn-compatible sidebar rail primitive with controlled/uncontrolled open state, side-aware contracts, and Spectrum-style data markers.""#,
        r#"<Playground title="Default Rail" code_signal=default_code>"#,
        r#"<Playground title="Controlled Right Rail" code_signal=controlled_code>"#,
        "<SidebarRail",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_rail docs should include `{needle}` for sidebar_rail primary playground coverage.",
        );
    }
}

#[test]
fn sidebar_rail_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_rail.rs");

    for needle in [
        r#"title="Default Rail""#,
        "<SidebarRail on_open_change=on_open_change />",
        "open=open",
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Sidebar",
        "collapsible=SidebarCollapsible::Offcanvas",
        "show_trigger=false",
        r#"aria_label="Sidebar rail playground".to_string()"#,
        r#"<SidebarContent aria_label="Workspace content".to_string()>"#,
        r#""Dashboard""#,
        r#""Projects""#,
        r#""Billing""#,
        r#"title="Controlled Right Rail""#,
        "open=right_open",
        "on_open_change=on_right_open_change",
        "side=SidebarSide::Right",
        r#"aria_label="Toggle right rail".to_string()"#,
        r#"label="toggle inspector".to_string()"#,
        r#"class_name="docs-sidebar-rail-custom".to_string()"#,
        "variant=SidebarVariant::Inset",
        "collapsible=SidebarCollapsible::Icon",
        r#"aria_label="Right inspector sidebar".to_string()"#,
        r#"<SidebarContent aria_label="Inspector content".to_string()>"#,
        r#""Tokens""#,
        r#""Layers""#,
        r#""Motion""#,
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_rail docs playgrounds should contain `{needle}` for sidebar_rail contracts.",
        );
    }
}
