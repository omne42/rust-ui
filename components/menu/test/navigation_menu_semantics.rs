use std::fs;
use std::path::Path;

fn workspace_dir() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
        .to_path_buf()
}

fn load_source(rel_path: &str) -> String {
    if let Some(component_rel_path) = rel_path.strip_prefix("src/menu/") {
        let path = workspace_dir()
            .join("components/menu/src")
            .join(component_rel_path);
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn navigation_menu_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menu/navigation_menu/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "NavigationMenu internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn navigation_menu_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/menu/navigation_menu/mod.rs");

    for needle in [
        "pub struct NavigationMenuItem",
        "pub struct NavigationMenuItemResolved",
        "pub enum NavigationMenuSlot",
        "pub struct NavigationMenuPartStateInput",
        "pub struct NavigationMenuPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_ACTIVATE_ON_FOCUS",
        "pub use ui_visual_primitive::active_highlight::ActiveHighlightMotion as NavigationMenuMotion;",
    ] {
        assert!(
            source.contains(needle),
            "navigation_menu::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn navigation_menu_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/menu/navigation_menu/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::NavigationMenu;"),
        "navigation_menu module should export `NavigationMenu`."
    );
    assert!(
        crate_source.contains(
            "pub use navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuMotion};"
        ),
        "crate root should re-export navigation_menu contracts."
    );
}

#[test]
fn navigation_menu_logic_exposes_state_helpers() {
    let source = load_source("src/menu/navigation_menu/logic.rs");

    for needle in [
        "pub fn state_attr(item_count: usize, has_selection: bool, has_focus: bool)",
        "pub fn item_attr(item_count: usize)",
        "pub fn selected_attr(has_selection: bool)",
        "pub fn focus_attr(has_focus: bool)",
        "pub fn focus_activation_attr(activate_on_focus: bool)",
        "pub fn selection_mode_attr(is_controlled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_id_base(id_base: String)",
        "pub fn resolve_aria_label(value: Option<String>)",
        "pub fn resolve_items(",
        "pub fn sanitize_selected_id(",
        "pub fn sanitize_focused_index(",
        "pub fn resolve_state(input: NavigationMenuPartStateInput) -> NavigationMenuPartState",
        "pub fn compose_class_name(",
        "pub fn next_enabled_index(",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn navigation_menu_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/menu/navigation_menu/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_items(&id_base.get_value(), items)",
        "logic::resolve_state(NavigationMenuPartStateInput {",
        "slot: NavigationMenuSlot::Root",
        "logic::compose_class_name(class_name.get_value(), root_state_for_class.get())",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-items=move || root_state.get().item_attr",
        "data-selection=move || root_state.get().selected_attr",
        "data-focus=move || root_state.get().focus_attr",
        "data-focus-activation=move || root_state.get().focus_activation_attr",
        "data-selection-mode=move || root_state.get().selection_mode_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-activate-on-focus-source=move || root_state.get().activate_on_focus_source_attr",
        "data-selected-id-source=move || root_state.get().selected_id_source_attr",
        "data-default-selected-id-source=move || root_state.get().default_selected_id_source_attr",
        "data-selected-id-change-source=move || root_state.get().selected_id_change_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-custom-id=move || root_state.get().has_custom_id_base.then_some(\"true\")",
        "data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some(\"true\")",
        "data-custom-selected-id=move || root_state.get().has_custom_selected_id.then_some(\"true\")",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu view should include `{needle}` for stable state/source marker contracts."
        );
    }
}

#[test]
fn navigation_menu_items_use_native_anchor_semantics() {
    let source = load_source("src/menu/navigation_menu/view.rs");

    assert!(
        source.contains(
            "aria-current=move || (selected_index.get() == Some(index)).then_some(\"page\")"
        ),
        "NavigationMenu items should expose aria-current on selected anchors."
    );

    assert!(
        !source.contains("role=\"link\""),
        "NavigationMenu items should rely on native anchor semantics instead of redundant role=\"link\"."
    );
}

#[test]
fn navigation_menu_supports_controlled_and_uncontrolled_selection_state() {
    let source = load_source("src/menu/navigation_menu/view.rs");

    for needle in [
        "selected_id: Option<Signal<Option<String>>>",
        "default_selected_id: Option<String>",
        "on_selected_id_change: Option<Callback<Option<String>>>",
        "let has_custom_selected_id = selected_id.is_some()",
        "let has_custom_default_selected_id = default_selected_id.is_some()",
        "overlay_open::use_controllable_state(",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu should support `{needle}` for controlled/uncontrolled selection behavior."
        );
    }
}

#[test]
fn navigation_menu_exposes_keyboard_and_focus_contracts() {
    let source = load_source("src/menu/navigation_menu/view.rs");

    for needle in [
        "on:keydown=on_key_down",
        "on:focus=on_focus",
        "on:pointerenter=on_pointer_enter",
        "logic::next_enabled_index(items.get_value().as_ref(), index, 1)",
        "logic::next_enabled_index(items.get_value().as_ref(), index, -1)",
        "logic::first_enabled_index(items.get_value().as_ref())",
        "logic::last_enabled_index(items.get_value().as_ref())",
        "focus_item(&item_refs, next_index);",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu should wire `{needle}` to match keyboard and roving focus semantics."
        );
    }
}

#[test]
fn navigation_menu_uses_active_highlight_motion_contract() {
    let source = load_source("src/menu/navigation_menu/view.rs");

    for needle in [
        "use ui_visual_primitive::active_highlight::{",
        "attach_active_highlight_motion",
        "ActiveHighlightMotion",
        "let list_ref: NodeRef<html::Div> = NodeRef::new();",
        "let highlight_ref: NodeRef<html::Div> = NodeRef::new();",
        "let (active_index, set_active_index) = signal(",
        "attach_active_highlight_motion(list_ref, highlight_ref, active_index, option_id, motion);",
        "data-slot=highlight_slot.as_attr()",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu should compose active highlight motion via `{needle}` for baseline-level feedback continuity."
        );
    }
}

#[test]
fn navigation_menu_styles_include_state_and_source_markers() {
    let source = load_source("src/menu/navigation_menu/styles.rs");

    for needle in [
        ".ui-navigation-menu {",
        ".ui-navigation-menu--selected",
        ".ui-navigation-menu[data-state=\"selected\"]",
        ".ui-navigation-menu--manual-activation",
        ".ui-navigation-menu[data-focus-activation=\"manual\"]",
        ".ui-navigation-menu[data-selection-mode=\"controlled\"]",
        ".ui-navigation-menu--custom-id",
        ".ui-navigation-menu[data-id-source=\"custom\"]",
        ".ui-navigation-menu--custom-aria-label",
        ".ui-navigation-menu[data-aria-label-source=\"custom\"]",
        ".ui-navigation-menu--custom-activate-on-focus",
        ".ui-navigation-menu--custom-selected-id",
        ".ui-navigation-menu[data-selected-id-source=\"custom\"]",
        ".ui-navigation-menu--custom-default-selected-id",
        ".ui-navigation-menu--custom-selected-id-change",
        ".ui-navigation-menu[data-selected-id-change-source=\"custom\"]",
        ".ui-navigation-menu[data-motion-source=\"custom\"]",
        ".ui-navigation-menu[data-custom-motion=\"true\"]",
        ".ui-navigation-menu__item[data-state=\"selected\"]",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu styles should include `{needle}` for stable state/source contracts."
        );
    }
}

#[test]
fn navigation_menu_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn navigation_menu() -> AnyView",
        "title=\"NavigationMenu\"",
        "slug=\"navigation-menu\"",
        "State + Source Markers",
        "data-id-source",
        "data-aria-label-source",
        "data-activate-on-focus-source",
        "data-selected-id-source",
        "data-selected-id-change-source",
        "data-motion-source",
        "<NavigationMenu",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu docs page should contain `{needle}`."
        );
    }
}

#[test]
fn navigation_menu_docs_controlled_state_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Controlled + Manual Activation\"",
        "id_base=\"docs-navigation-menu-controlled\".to_string()",
        "selected_id=controlled_selected",
        "on_selected_id_change=on_controlled_selected_change",
        "activate_on_focus=false",
        "class_name=\"docs-navigation-menu-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu docs controlled-state playground should contain `{needle}`.",
        );
    }
}

#[test]
fn navigation_menu_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id_base=\"docs-navigation-menu-markers\".to_string()",
        "selected_id=marker_selected",
        "default_selected_id=\"workspace\".to_string()",
        "on_selected_id_change=on_marker_selected_change",
        "activate_on_focus=false",
        "aria_label=\"Workspace navigation\".to_string()",
        "class_name=\"docs-navigation-menu-custom\".to_string()",
        "let mut marker_motion = ui_components::NavigationMenuMotion::default();",
        "marker_motion.spring.stiffness = 260.0",
        "marker_motion.spring.damping = 24.0",
        "motion=marker_motion",
        "Inspect data-id-source / data-aria-label-source / data-activate-on-focus-source / data-selected-id-source / data-selected-id-change-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "NavigationMenu docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn navigation_menu_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn navigation_menu() -> AnyView",
        "title=\"NavigationMenu\"",
        "slug=\"navigation-menu\"",
        "<Playground title=\"Default + Roving Focus + Selection\" code_signal=code>",
        "<Playground title=\"Controlled + Manual Activation\" code_signal=states_code>",
        "<Playground title=\"State + Source Markers\" code_signal=marker_code>",
        "<NavigationMenu",
    ] {
        assert!(
            source.contains(needle),
            "collections_command docs should include `{needle}` for navigation-menu primary playground coverage.",
        );
    }
}

#[test]
fn navigation_menu_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Default + Roving Focus + Selection\"",
        "id_base=\"docs-navigation-menu-default\".to_string()",
        "default_selected_id=\"components\".to_string()",
        "title=\"Controlled + Manual Activation\"",
        "id_base=\"docs-navigation-menu-controlled\".to_string()",
        "selected_id=controlled_selected",
        "on_selected_id_change=on_controlled_selected_change",
        "activate_on_focus=false",
        "class_name=\"docs-navigation-menu-custom\".to_string()",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-navigation-menu-markers\".to_string()",
        "selected_id=marker_selected",
        "default_selected_id=\"workspace\".to_string()",
        "on_selected_id_change=on_marker_selected_change",
        "aria_label=\"Workspace navigation\".to_string()",
        "let mut marker_motion = ui_components::NavigationMenuMotion::default();",
        "marker_motion.spring.stiffness = 260.0",
        "marker_motion.spring.damping = 24.0",
        "motion=marker_motion",
        "Inspect data-id-source / data-aria-label-source / data-activate-on-focus-source / data-selected-id-source / data-selected-id-change-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "navigation-menu docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn navigation_menu_docs_workbench_exposes_display_config_code_and_css_test_contract() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/menu/navigation_menu/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"navigation-menu-workbench-controls\"",
        "display: baseline vs configured",
        "docs-navigation-menu-workbench",
    ] {
        assert!(
            source.contains(needle),
            "navigation-menu workbench should contain `{needle}`.",
        );
    }
}

#[test]
fn navigation_menu_readme_covers_workbench_display_config_code_css_test_sections() {
    let source = load_source("src/menu/navigation_menu/README.md");

    for needle in [
        "# NavigationMenu",
        "Docs Playground（展示 / Config / Code / CSS Test）",
        "展示",
        "Config",
        "Code",
        "CSS Test",
        "对比场景",
        "Workbench (Display + Config + Code + CSS Test)",
    ] {
        assert!(
            source.contains(needle),
            "navigation-menu README should contain `{needle}`.",
        );
    }
}

#[test]
fn navigation_menu_check2_has_no_remaining_unchecked_items() {
    let source = load_source("src/menu/navigation_menu/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "navigation_menu/check2.md still contains unchecked checklist items."
    );
}
