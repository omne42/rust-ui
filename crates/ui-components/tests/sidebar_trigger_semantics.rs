use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_trigger_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidebar_trigger/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarTrigger internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_trigger_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidebar_trigger/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarTrigger;"),
        "sidebar_trigger module should export `SidebarTrigger`.",
    );
    assert!(
        crate_source.contains("pub use sidebar_trigger::SidebarTrigger;"),
        "crate root should re-export SidebarTrigger contract.",
    );
}

#[test]
fn sidebar_trigger_uses_logic_state_model() {
    let logic_source = load_source("src/sidebar_trigger/logic.rs");
    let view_source = load_source("src/sidebar_trigger/view.rs");

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
            "SidebarTrigger logic should include `{needle}`.",
        );
    }

    for needle in [
        "overlay_open::use_controllable_open_state_traced(",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_label(label)",
        "logic::resolve_state(SidebarTriggerStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SidebarTrigger view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_trigger_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/sidebar_trigger/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let is_controlled = open.is_some()",
        "overlay_open::use_controllable_open_state_traced(",
    ] {
        assert!(
            source.contains(needle),
            "SidebarTrigger should support `{needle}` for controllable open-state flow.",
        );
    }
}

#[test]
fn sidebar_trigger_emits_baseline_root_state_data_attributes() {
    let source = load_source("src/sidebar_trigger/view.rs");

    for needle in [
        "data-slot=\"sidebar-trigger\"",
        "data-state=move || state.get().state_attr",
        "data-open=move || state.get().open.then_some(\"true\")",
        "data-closed=move || state.get().closed.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-control-mode=move || state.get().control_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "SidebarTrigger should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_trigger_styles_include_state_markers() {
    let source = load_source("src/sidebar_trigger/styles.rs");

    for needle in [
        ".ui-sidebar-trigger {",
        ".ui-sidebar-trigger--open",
        ".ui-sidebar-trigger[data-open=\"true\"]",
        ".ui-sidebar-trigger--disabled",
        ".ui-sidebar-trigger[data-disabled=\"true\"]",
        ".ui-sidebar-trigger--custom-class",
        ".ui-sidebar-trigger[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "SidebarTrigger styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_trigger_docs_page_exists_in_layout_extra_modules() {
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_trigger.rs",
    );

    assert!(
        layout_extra.contains("pub(super) fn sidebar_trigger() -> AnyView"),
        "layout_extra should expose sidebar_trigger route entry.",
    );

    for needle in [
        "pub(super) fn sidebar_trigger() -> AnyView",
        "title=\"SidebarTrigger\"",
        "slug=\"sidebar-trigger\"",
        "<SidebarTrigger",
    ] {
        assert!(
            docs.contains(needle),
            "SidebarTrigger docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn sidebar_trigger_docs_page_covers_primary_playgrounds() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_trigger.rs",
    );

    for needle in [
        "pub(super) fn sidebar_trigger() -> AnyView",
        r#"title="SidebarTrigger""#,
        r#"slug="sidebar-trigger""#,
        r#"description="baseline-compatible sidebar trigger primitive with controlled/uncontrolled open state, baseline-style data contracts, and motion-ready state transitions.""#,
        r#"<Playground title="Default Trigger" code_signal=default_code>"#,
        r#"<Playground title="Controlled + Custom Label" code_signal=controlled_code>"#,
        "<SidebarTrigger",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_trigger docs should include `{needle}` for sidebar_trigger primary playground coverage.",
        );
    }
}

#[test]
fn sidebar_trigger_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_trigger.rs",
    );

    for needle in [
        r#"title="Default Trigger""#,
        "<SidebarTrigger on_open_change=on_open_change />",
        "open=open",
        "on_open_change=on_open_change",
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Sidebar",
        "collapsible=SidebarCollapsible::Offcanvas",
        "show_trigger=false",
        r#"aria_label="Sidebar trigger playground".to_string()"#,
        r#"<SidebarContent aria_label="Workspace content".to_string()>"#,
        r#""Dashboard""#,
        r#""Projects""#,
        r#""Billing""#,
        r#"title="Controlled + Custom Label""#,
        r#"aria_label="Toggle inspector sidebar".to_string()"#,
        r#"label="Inspector".to_string()"#,
        r#"class_name="docs-sidebar-trigger-custom".to_string()"#,
        r#""open: ""#,
        r#""true""#,
        r#""false""#,
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_trigger docs playgrounds should contain `{needle}` for sidebar_trigger contracts.",
        );
    }
}
