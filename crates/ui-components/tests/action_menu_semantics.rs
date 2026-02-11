use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_menu_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/action_menu/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ActionMenu internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn action_menu_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/action_menu/mod.rs");

    for needle in [
        "pub enum MenuOpenFocusStrategy",
        "pub struct ActionMenuIds",
        "pub enum ActionMenuSlot",
        "pub struct ActionMenuPartStateInput",
        "pub struct ActionMenuPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_TRIGGER_ARIA_LABEL",
        "DEFAULT_DISABLED",
        "DEFAULT_CLOSE_ON_ACTION",
        "DEFAULT_PLACEMENT",
        "pub use motion::ActionMenuMotion;",
    ] {
        assert!(
            source.contains(needle),
            "action_menu::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn action_menu_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/action_menu/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::ActionMenu;"),
        "action_menu module should export `ActionMenu`."
    );
    assert!(
        crate_source.contains("pub use action_menu::{ActionMenu, ActionMenuMotion};"),
        "crate root should re-export action_menu contracts."
    );
}

#[test]
fn action_menu_logic_exposes_state_helpers() {
    let source = load_source("src/action_menu/logic.rs");

    for needle in [
        "pub fn state_attr(is_open: bool, trigger_disabled: bool, item_count: usize)",
        "pub fn item_attr(item_count: usize)",
        "pub fn action_attr(close_on_action: bool)",
        "pub fn open_mode_attr(is_controlled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_id_base(id_base: String)",
        "pub fn resolve_ids(id_base: &str)",
        "pub fn normalize_disabled_indices(disabled_indices: Vec<usize>, item_count: usize)",
        "pub fn resolve_trigger_disabled(disabled: bool, item_count: usize)",
        "pub fn resolve_trigger_aria_label(value: Option<String>)",
        "pub fn resolve_state(input: ActionMenuPartStateInput) -> ActionMenuPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ActionMenuPartState)",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn action_menu_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/action_menu/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_disabled_indices(disabled_indices, item_count)",
        "logic::resolve_trigger_aria_label(aria_label)",
        "logic::resolve_state(ActionMenuPartStateInput {",
        "slot: ActionMenuSlot::Root",
        "logic::compose_class_name(class_name.get_value(), root_state_for_class.get())",
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
        "data-custom-open=move || root_state.get().has_custom_open.then_some(\"true\")",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu view should include `{needle}` for stable state/source marker contracts."
        );
    }
}

#[test]
fn action_menu_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/action_menu/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let has_custom_open = open.is_some()",
        "let has_custom_default_open = default_open.is_some()",
        "let has_custom_on_open_change = on_open_change.is_some()",
        "overlay_open::use_controllable_open_state(open, default_open, on_open_change)",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should support `{needle}` for controllable open behavior."
        );
    }
}

#[test]
fn action_menu_trigger_uses_action_button_with_overlay_aria_contract() {
    let source = load_source("src/action_menu/view.rs");

    for needle in [
        "<ActionButton",
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_label=aria_label.get_value()",
        "disabled=trigger_disabled",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should wire its trigger via `{needle}` for Spectrum overlay semantics."
        );
    }
}

#[test]
fn action_menu_renders_menu_inside_popover_with_presence() {
    let source = load_source("src/action_menu/view.rs");

    for needle in [
        "use_presence(open)",
        "<Popover",
        "<Menu",
        "aria_labelledby=trigger_id.get_value()",
        "on_exit_complete=presence.finish_exit",
        "motion=motion.popover",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should compose menu/popover/presence via `{needle}` for motion-safe unmounting."
        );
    }
}

#[test]
fn action_menu_styles_include_state_and_source_markers() {
    let source = load_source("src/action_menu/styles.rs");

    for needle in [
        ".ui-action-menu {",
        ".ui-action-menu--open",
        ".ui-action-menu[data-state=\"open\"]",
        ".ui-action-menu--persistent",
        ".ui-action-menu[data-action-mode=\"keep-open\"]",
        ".ui-action-menu[data-open-mode=\"controlled\"]",
        ".ui-action-menu[data-id-source=\"custom\"]",
        ".ui-action-menu[data-aria-label-source=\"custom\"]",
        ".ui-action-menu[data-disabled-indices-source=\"custom\"]",
        ".ui-action-menu[data-item-kinds-source=\"custom\"]",
        ".ui-action-menu[data-open-source=\"custom\"]",
        ".ui-action-menu[data-open-change-source=\"custom\"]",
        ".ui-action-menu[data-motion-source=\"custom\"]",
        ".ui-action-menu[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu styles should include `{needle}` for stable state/source contracts."
        );
    }
}

#[test]
fn action_menu_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/action_menu/mod.rs");
    let motion_source = load_source("src/action_menu/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::ActionMenuMotion;",
        "pub struct ActionMenuMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "ActionMenu motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}

#[test]
fn action_menu_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn action_menu() -> AnyView",
        "title=\"ActionMenu\"",
        "slug=\"action-menu\"",
        "State + Source Markers",
        "data-id-source",
        "data-aria-label-source",
        "data-disabled-indices-source",
        "data-item-kinds-source",
        "data-open-source",
        "data-open-change-source",
        "data-motion-source",
        "<ActionMenu",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu docs page should contain `{needle}`."
        );
    }
}
