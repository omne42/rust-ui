use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn dropdown_menu_does_not_expose_logic_module() {
    let source = load_source("src/dropdown_menu/mod.rs");

    for needle in ["pub mod logic", "pub use logic"] {
        assert!(
            !source.contains(needle),
            "DropdownMenu internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn dropdown_menu_uses_logic_state_model() {
    let view_source = load_source("src/dropdown_menu/view.rs");
    let logic_source = load_source("src/dropdown_menu/logic.rs");

    for needle in [
        "pub struct DropdownMenuStateInput",
        "pub struct DropdownMenuState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "DropdownMenu logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let disabled_indices = logic::normalize_disabled_indices(disabled_indices, item_count);",
        "let state = logic::resolve_state(logic::DropdownMenuStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "DropdownMenu view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn dropdown_menu_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn dropdown_menu_trigger_wires_overlay_aria_contract() {
    let source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_labelledby=trigger_id.get_value()",
        "disabled=state.is_trigger_disabled",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should wire `{needle}` to match Spectrum overlay trigger semantics."
        );
    }
}

#[test]
fn dropdown_menu_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "data-slot=\"dropdown-menu\"",
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
        "data-has-disabled-items=state.has_disabled_items.then_some(\"true\")",
        "data-has-item-kinds=state.has_item_kinds.then_some(\"true\")",
        "on:keydown=on_key_down",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should set `{needle}` so it can be styled/tested with Spectrum-compatible root state selectors."
        );
    }
}

#[test]
fn dropdown_menu_renders_menu_inside_popover_with_presence() {
    let source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "use_presence(open)",
        "<Popover",
        "placement=state.placement",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should compose popover/presence/menu via `{needle}` for motion-safe unmounting."
        );
    }
}

#[test]
fn dropdown_menu_uses_logic_for_disabled_trigger_and_open_keys() {
    let view_source = load_source("src/dropdown_menu/view.rs");
    let logic_source = load_source("src/dropdown_menu/logic.rs");

    for needle in [
        "resolve_trigger_disabled",
        "focus_strategy_for_open_key",
        "MenuOpenFocusStrategy",
    ] {
        assert!(
            logic_source.contains(needle),
            "DropdownMenu logic should centralize `{needle}` semantics."
        );
    }

    for needle in [
        "if trigger_disabled.get_value()",
        "if let Some(strategy) = logic::focus_strategy_for_open_key(&key)",
        "set_open_focus.set(strategy);",
    ] {
        assert!(
            view_source.contains(needle),
            "DropdownMenu view should consume `{needle}` to keep trigger behavior and keyboard-open semantics consistent."
        );
    }
}

#[test]
fn dropdown_menu_styles_include_disabled_and_persistent_markers() {
    let source = load_source("src/dropdown_menu/styles.rs");

    for needle in [
        ".ui-dropdown-menu--persistent",
        ".ui-dropdown-menu--disabled",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu styles should include `{needle}` for stable visual state contracts."
        );
    }
}
