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
        "disabled=trigger_disabled.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should wire `{needle}` to match Spectrum overlay trigger semantics."
        );
    }
}

#[test]
fn dropdown_menu_uses_presence_and_state_data_attributes() {
    let source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "use_presence(open)",
        "on_exit_complete=presence.finish_exit",
        "data-slot=\"dropdown-menu\"",
        "data-open=move || open.get().then_some(\"true\")",
        "data-disabled=trigger_disabled.get_value().then_some(\"true\")",
        "data-empty=(item_count.get_value() == 0).then_some(\"true\")",
        "data-has-items=(item_count.get_value() > 0).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should expose `{needle}` for motion-safe unmounting and Spectrum-style state styling."
        );
    }
}

#[test]
fn dropdown_menu_supports_arrow_key_opening_and_empty_item_disable() {
    let view_source = load_source("src/dropdown_menu/view.rs");
    let logic_source = load_source("src/dropdown_menu/logic.rs");

    for needle in [
        "focus_strategy_for_open_key",
        "set_open_focus.set(strategy)",
        "request_open_change.run(true)",
    ] {
        assert!(
            view_source.contains(needle),
            "DropdownMenu should support ArrowUp/ArrowDown opening via `{needle}`."
        );
    }

    assert!(
        logic_source.contains("resolve_trigger_disabled"),
        "DropdownMenu logic should centralize trigger disabled state so empty menus are not interactive."
    );

    for needle in [
        "let trigger_disabled = StoredValue::new(logic::resolve_trigger_disabled(",
        "if trigger_disabled.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "DropdownMenu view should consume `{needle}` so trigger behavior matches the disabled model."
        );
    }
}
