use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidebar_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/sidebar/src/group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SidebarGroup internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_group_is_exported_from_module_and_crate_root() {
    let module_source = load_source("../../components/sidebar/src/group/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SidebarGroup;"),
        "sidebar_group module should export `SidebarGroup`.",
    );
    assert!(
        crate_source.contains("pub use sidebar::group::SidebarGroup;"),
        "crate root should re-export SidebarGroup contract.",
    );
}

#[test]
fn sidebar_group_uses_logic_state_model() {
    let logic_source = load_source("../../components/sidebar/src/group/logic.rs");
    let view_source = load_source("../../components/sidebar/src/group/view.rs");

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
        "overlay_open::use_controllable_open_state_traced(",
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
    let source = load_source("../../components/sidebar/src/group/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let is_controlled = open.is_some()",
        "overlay_open::use_controllable_open_state_traced(",
    ] {
        assert!(
            source.contains(needle),
            "SidebarGroup should support `{needle}` for controllable open-state flow.",
        );
    }
}

#[test]
fn sidebar_group_emits_baseline_root_state_data_attributes() {
    let source = load_source("../../components/sidebar/src/group/view.rs");

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
    let source = load_source("../../components/sidebar/src/group/styles.rs");

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
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_group.rs");

    assert!(
        layout_extra.contains("pub(super) fn sidebar_group() -> AnyView"),
        "layout_extra should expose sidebar_group route entry.",
    );

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

#[test]
fn sidebar_group_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_group.rs");

    for needle in [
        "pub(super) fn sidebar_group() -> AnyView",
        "title=\"SidebarGroup\"",
        "slug=\"sidebar-group\"",
        "description=\"baseline-compatible sidebar group primitive with label/action header regions, controlled/uncontrolled collapsible state, baseline-style data contracts, and motion-ready collapse behavior.\"",
        "<Playground title=\"Label + Group Action\" code_signal=base_code>",
        "<Playground title=\"Controlled + Collapsible Group\" code_signal=controlled_code>",
        "<SidebarGroup",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_group docs should include `{needle}` for sidebar_group primary playground coverage.",
        );
    }
}

#[test]
fn sidebar_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidebar_group.rs");

    for needle in [
        "title=\"Label + Group Action\"",
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Sidebar",
        "collapsible=SidebarCollapsible::Offcanvas",
        "show_trigger=false",
        "aria_label=\"Sidebar group playground\".to_string()",
        "label=\"Help\".to_string()",
        "action_label=\"Add\".to_string()",
        "on_action=on_group_action",
        "collapsible=false",
        "aria_label=\"Help group\".to_string()",
        "id_base=\"docs-sidebar-group-basic\".to_string()",
        "show_actions=false",
        "aria_label=\"Help menu\".to_string()",
        "title=\"Controlled + Collapsible Group\"",
        "open=group_open",
        "on_open_change=on_group_open_change",
        "collapsible=true",
        "show_action=false",
        "label=\"Architecture\".to_string()",
        "aria_label=\"Architecture group\".to_string()",
        "class_name=\"docs-sidebar-group-custom\".to_string()",
        "id_base=\"docs-sidebar-group-collapsible\".to_string()",
        "allow_submenu_collapse=true",
        "show_actions=true",
        "show_badges=false",
        "aria_label=\"Architecture menu\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidebar_group docs playgrounds should contain `{needle}` for sidebar_group contracts.",
        );
    }
}
