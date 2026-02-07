use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_menu_does_not_expose_logic_module() {
    let source = load_source("src/action_menu/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "ActionMenu's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
}

#[test]
fn action_menu_uses_logic_state_model() {
    let view_source = load_source("src/action_menu/view.rs");
    let logic_source = load_source("src/action_menu/logic.rs");

    for needle in [
        "pub struct ActionMenuStateInput",
        "pub struct ActionMenuState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_trigger_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionMenu logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let disabled_indices =",
        "logic::normalize_disabled_indices(disabled_indices, item_count);",
        "let (aria_label, has_custom_aria_label) = logic::resolve_trigger_aria_label(aria_label);",
        "let state = logic::resolve_state(logic::ActionMenuStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionMenu view should derive wrapper state through logic helpers; missing `{needle}`."
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
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn action_menu_emits_spectrum_root_slot_and_state_data_attributes() {
    let source = load_source("src/action_menu/view.rs");

    for needle in [
        "data-slot=\"action-menu\"",
        "data-state=move ||",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-disabled=state.is_trigger_disabled.then_some(\"true\")",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-placement=state.placement_attr",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-close-on-action=state.close_on_action.then_some(\"true\")",
        "data-keep-open-on-action=state.keep_open_on_action.then_some(\"true\")",
        "data-custom-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-has-disabled-items=state.has_disabled_items.then_some(\"true\")",
        "data-has-item-kinds=state.has_item_kinds.then_some(\"true\")",
        "on:keydown=on_key_down",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should set `{needle}` so it can be styled/tested with Spectrum-compatible root state selectors."
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
        "disabled=state.is_trigger_disabled",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should wire its trigger via `{needle}` to align with Spectrum overlay trigger semantics."
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
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should compose menu/popover/presence via `{needle}` to avoid popover unmounting before exit motion completes."
        );
    }
}

#[test]
fn action_menu_uses_logic_for_disabled_trigger_and_open_keys() {
    let logic_source = load_source("src/action_menu/logic.rs");
    let view_source = load_source("src/action_menu/view.rs");

    for needle in [
        "resolve_trigger_disabled",
        "focus_strategy_for_open_key",
        "MenuOpenFocusStrategy",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionMenu logic should centralize `{needle}` semantics."
        );
    }

    for needle in [
        "if trigger_disabled.get_value()",
        "if let Some(strategy) = logic::focus_strategy_for_open_key(&key)",
        "set_open_focus.set(strategy);",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionMenu view should consume `{needle}` to keep trigger behavior and keyboard-open semantics consistent."
        );
    }
}

#[test]
fn action_menu_styles_include_disabled_and_persistent_markers() {
    let source = load_source("src/action_menu/styles.rs");

    for needle in [".ui-action-menu--persistent", ".ui-action-menu--disabled"] {
        assert!(
            source.contains(needle),
            "ActionMenu styles should include `{needle}` for stable visual state contracts."
        );
    }
}
