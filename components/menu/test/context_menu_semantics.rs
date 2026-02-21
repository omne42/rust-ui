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
fn context_menu_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menu/context_menu/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ContextMenu internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn context_menu_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/menu/context_menu/mod.rs");

    for needle in [
        "pub enum ContextMenuSlot",
        "pub struct ContextMenuPartStateInput",
        "pub struct ContextMenuPartState",
        "pub enum MenuOpenFocusStrategy",
        "pub struct ContextMenuIds",
        "DEFAULT_ID_BASE",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_CLOSE_ON_ACTION",
        "DEFAULT_DISABLED",
        "DEFAULT_PLACEMENT",
        "pub use crate::dropdown_menu::DropdownMenuMotion as ContextMenuMotion;",
    ] {
        assert!(
            source.contains(needle),
            "context_menu::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn context_menu_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/menu/context_menu/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::ContextMenu;"),
        "context_menu module should export `ContextMenu`."
    );
    assert!(
        crate_source.contains("pub use context_menu::{ContextMenu, ContextMenuMotion};"),
        "crate root should re-export context_menu contracts."
    );
}

#[test]
fn context_menu_logic_exposes_state_helpers() {
    let source = load_source("src/menu/context_menu/logic.rs");

    for needle in [
        "pub fn state_attr(is_open: bool, trigger_disabled: bool)",
        "pub fn item_attr(item_count: usize)",
        "pub fn disabled_attr(trigger_disabled: bool)",
        "pub fn action_attr(close_on_action: bool)",
        "pub fn open_mode_attr(is_controlled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_id_base(id_base: String)",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_trigger_disabled(disabled: bool, item_count: usize)",
        "pub fn resolve_trigger_aria_label(value: Option<String>)",
        "pub fn resolve_state(input: ContextMenuPartStateInput) -> ContextMenuPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ContextMenuPartState)",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn context_menu_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/menu/context_menu/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_disabled_indices(disabled_indices, item_count.get_value())",
        "logic::resolve_trigger_aria_label(aria_label)",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "logic::resolve_state(ContextMenuPartStateInput {",
        "slot: ContextMenuSlot::Root",
        "logic::compose_class_name(class_name.get_value(), root_state_for_class.get())",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-items=move || root_state.get().item_attr",
        "data-action-mode=move || root_state.get().action_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-disabled-source=move || root_state.get().disabled_source_attr",
        "data-disabled-indices-source=move || root_state.get().disabled_indices_source_attr",
        "data-item-kinds-source=move || root_state.get().item_kinds_source_attr",
        "data-close-on-action-source=move || root_state.get().close_on_action_source_attr",
        "data-placement-source=move || root_state.get().placement_source_attr",
        "data-open-source=move || root_state.get().open_source_attr",
        "data-default-open-source=move || root_state.get().default_open_source_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-custom-id=move || root_state.get().has_custom_id_base.then_some(\"true\")",
        "data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=move || root_state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-disabled=move || root_state.get().has_custom_disabled.then_some(\"true\")",
        "data-custom-disabled-indices=move || {",
        "data-custom-item-kinds=move || root_state.get().has_custom_item_kinds.then_some(\"true\")",
        "data-custom-close-on-action=move || {",
        "data-custom-placement=move || root_state.get().has_custom_placement.then_some(\"true\")",
        "data-custom-open=move || root_state.get().has_custom_open.then_some(\"true\")",
        "data-custom-default-open=move || {",
        "data-custom-open-change=move || {",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
        "data-ui-schema=\"ui.context_menu.agent-contract.v1\"",
        "data-ui-stream-support=\"unsupported\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-state=move || root_state.get().state_attr",
        "data-ui-source=move || root_state.get().open_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu view should include `{needle}` for stable state/source marker contracts."
        );
    }
}

#[test]
fn context_menu_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/menu/context_menu/view.rs");

    for needle in [
        "is_disabled: Option<bool>",
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let disabled = is_disabled.unwrap_or(disabled);",
        "overlay_open::use_controllable_open_state_traced(",
        "let has_custom_open = open.is_some()",
        "let has_custom_default_open = default_open.is_some()",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu should support `{needle}` for controllable open state."
        );
    }
}

#[test]
fn context_menu_trigger_wires_context_and_keyboard_open_contract() {
    let source = load_source("src/menu/context_menu/view.rs");

    for needle in [
        "on:contextmenu=on_context_menu",
        "on:keydown=on_key_down",
        "crate::context_menu::focus_strategy_for_open_key(&key, ev.shift_key())",
        "aria-haspopup=\"menu\"",
        "aria-expanded=move || if open.get() { \"true\" } else { \"false\" }",
        "aria-controls=aria_controls",
        "aria_labelledby=trigger_id.get_value()",
        "data-slot=trigger_slot.as_attr()",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu should wire `{needle}` to match context-trigger + keyboard-open semantics."
        );
    }
}

#[test]
fn context_menu_renders_menu_inside_popover_with_presence() {
    let source = load_source("src/menu/context_menu/view.rs");

    for needle in [
        "use_presence(open)",
        "<Popover",
        "placement=placement",
        "motion=motion.popover",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu should compose popover/presence/menu via `{needle}` for motion-safe unmounting."
        );
    }
}

#[test]
fn context_menu_styles_include_state_and_source_markers() {
    let source = load_source("src/menu/context_menu/styles.rs");

    for needle in [
        ".ui-context-menu {",
        ".ui-context-menu__trigger {",
        ".ui-context-menu--open",
        ".ui-context-menu[data-state=\"open\"]",
        ".ui-context-menu--persistent",
        ".ui-context-menu[data-action-mode=\"keep-open\"]",
        ".ui-context-menu[data-open-mode=\"controlled\"]",
        ".ui-context-menu[data-id-source=\"custom\"]",
        ".ui-context-menu[data-custom-id=\"true\"]",
        ".ui-context-menu--custom-id",
        ".ui-context-menu[data-aria-label-source=\"custom\"]",
        ".ui-context-menu[data-custom-aria-label=\"true\"]",
        ".ui-context-menu--custom-aria-label",
        ".ui-context-menu[data-class-source=\"custom\"]",
        ".ui-context-menu[data-custom-class=\"true\"]",
        ".ui-context-menu--custom-class",
        ".ui-context-menu[data-disabled-source=\"custom\"]",
        ".ui-context-menu[data-custom-disabled=\"true\"]",
        ".ui-context-menu--custom-disabled",
        ".ui-context-menu[data-disabled-indices-source=\"custom\"]",
        ".ui-context-menu[data-custom-disabled-indices=\"true\"]",
        ".ui-context-menu--custom-disabled-indices",
        ".ui-context-menu[data-item-kinds-source=\"custom\"]",
        ".ui-context-menu[data-custom-item-kinds=\"true\"]",
        ".ui-context-menu--custom-item-kinds",
        ".ui-context-menu[data-close-on-action-source=\"custom\"]",
        ".ui-context-menu[data-custom-close-on-action=\"true\"]",
        ".ui-context-menu--custom-close-on-action",
        ".ui-context-menu[data-placement-source=\"custom\"]",
        ".ui-context-menu[data-custom-placement=\"true\"]",
        ".ui-context-menu--custom-placement",
        ".ui-context-menu[data-open-source=\"custom\"]",
        ".ui-context-menu[data-custom-open=\"true\"]",
        ".ui-context-menu--custom-open",
        ".ui-context-menu[data-default-open-source=\"custom\"]",
        ".ui-context-menu[data-custom-default-open=\"true\"]",
        ".ui-context-menu--custom-default-open",
        ".ui-context-menu[data-open-change-source=\"custom\"]",
        ".ui-context-menu[data-custom-open-change=\"true\"]",
        ".ui-context-menu--custom-open-change",
        ".ui-context-menu[data-motion-source=\"custom\"]",
        ".ui-context-menu--custom-motion",
        ".ui-context-menu[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "ContextMenu styles should include `{needle}` as stable state/source contracts."
        );
    }
}

#[test]
fn context_menu_uses_dropdown_menu_motion_alias_contract() {
    let mod_source = load_source("src/menu/context_menu/mod.rs");
    let dropdown_motion_source = load_source("src/menu/dropdown_menu/motion.rs");

    for needle in [
        "pub use crate::dropdown_menu::DropdownMenuMotion as ContextMenuMotion;",
        "pub struct DropdownMenuMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || dropdown_motion_source.contains(needle),
            "ContextMenu motion alias contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn context_menu_docs_page_contains_state_source_playground() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn context_menu() -> AnyView",
        "title=\"ContextMenu\"",
        "slug=\"context-menu\"",
        "State + Source Markers",
        "data-id-source",
        "data-aria-label-source",
        "data-disabled-indices-source",
        "data-close-on-action-source",
        "data-open-source",
        "data-motion-source",
        "<ContextMenu",
    ] {
        assert!(
            docs.contains(needle),
            "ContextMenu docs page should contain `{needle}`."
        );
    }
}

#[test]
fn context_menu_docs_persistent_state_playground_locks_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Persistent + Disabled + ItemKinds\"",
        "id_base=\"docs-context-menu-persistent\".to_string()",
        "close_on_action=false",
        "disabled_indices=vec![1]",
        "item_kinds=vec![",
        "aria_label=\"File actions\".to_string()",
        "class_name=\"docs-context-menu-custom\".to_string()",
        "close_on_action: false (selection keeps menu open)",
    ] {
        assert!(
            docs.contains(needle),
            "ContextMenu docs persistent-state playground should contain `{needle}`."
        );
    }
}

#[test]
fn context_menu_docs_state_source_playground_locks_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id_base=\"docs-context-menu-markers\".to_string()",
        "open=marker_open",
        "default_open=true",
        "on_open_change=on_marker_open_change",
        "disabled_indices=vec![2]",
        "aria_label=\"Workspace context actions\".to_string()",
        "class_name=\"docs-context-menu-custom\".to_string()",
        "let marker_motion = ui_components::ContextMenuMotion {",
        "initial_scale: 0.94",
        "offset_y_px: 10.0",
        "motion=marker_motion",
        "Inspect data-id-source / data-aria-label-source / data-disabled-indices-source / data-close-on-action-source / data-open-source / data-motion-source in DevTools.",
    ] {
        assert!(
            docs.contains(needle),
            "ContextMenu docs state/source playground should contain `{needle}`."
        );
    }
}

#[test]
fn context_menu_docs_page_covers_primary_playgrounds() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn context_menu() -> AnyView",
        "title=\"ContextMenu\"",
        "slug=\"context-menu\"",
        "description=\"baseline-compatible context trigger menu with right-click + keyboard open semantics, baseline state/source attrs, and baseline-level popover spring motion reuse.\"",
        "<Playground title=\"Right Click + Keyboard Open\" code_signal=code>",
        "<Playground title=\"Persistent + Disabled + ItemKinds\" code_signal=states_code>",
        "<Playground title=\"State + Source Markers\" code_signal=marker_code>",
        "data-id-source",
        "data-aria-label-source",
        "data-disabled-indices-source",
        "data-close-on-action-source",
        "data-open-source",
        "data-motion-source",
    ] {
        assert!(
            docs.contains(needle),
            "collections_command docs page should include `{needle}` for context_menu primary coverage.",
        );
    }
}

#[test]
fn context_menu_docs_playgrounds_lock_state_matrix_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "id_base=\"docs-context-menu-default\".to_string()",
        "items=default_items",
        "on_action=on_default_action",
        "\"Right click or press Shift+F10\"",
        "id_base=\"docs-context-menu-persistent\".to_string()",
        "items=keep_open_items",
        "on_action=on_keep_open_action",
        "close_on_action=false",
        "disabled_indices=vec![1]",
        "aria_label=\"File actions\".to_string()",
        "class_name=\"docs-context-menu-custom\".to_string()",
        "\"close_on_action: false (selection keeps menu open)\"",
        "id_base=\"docs-context-menu-markers\".to_string()",
        "items=marker_items",
        "on_action=on_marker_action",
        "open=marker_open",
        "default_open=true",
        "on_open_change=on_marker_open_change",
        "disabled_indices=vec![2]",
        "aria_label=\"Workspace context actions\".to_string()",
        "let marker_motion = ui_components::ContextMenuMotion {",
        "initial_scale: 0.94",
        "offset_y_px: 10.0",
        "motion=marker_motion",
        "\"open: \"",
        "\"last action: \"",
    ] {
        assert!(
            docs.contains(needle),
            "context_menu docs playgrounds should contain `{needle}`.",
        );
    }
}
