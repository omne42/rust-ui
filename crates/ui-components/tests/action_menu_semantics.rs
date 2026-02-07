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
        "data-open=move || open.get().then_some(\"true\")",
        "data-disabled=trigger_disabled.get_value().then_some(\"true\")",
        "data-empty=(item_count.get_value() == 0).then_some(\"true\")",
        "data-has-items=(item_count.get_value() > 0).then_some(\"true\")",
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
        "disabled=trigger_disabled.get_value()",
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
fn action_menu_uses_logic_for_empty_and_disabled_trigger_state() {
    let logic_source = load_source("src/action_menu/logic.rs");
    let view_source = load_source("src/action_menu/view.rs");

    assert!(
        logic_source.contains("resolve_trigger_disabled"),
        "ActionMenu logic should centralize trigger-disabled semantics for disabled + empty states."
    );

    for needle in [
        "let trigger_disabled = StoredValue::new(logic::resolve_trigger_disabled(",
        "if trigger_disabled.get_value()",
        "disabled=trigger_disabled.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionMenu view should consume `{needle}` to keep trigger behavior/state attrs consistent."
        );
    }
}
