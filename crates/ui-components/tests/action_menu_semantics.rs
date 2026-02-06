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
fn action_menu_emits_spectrum_root_slot_and_keyboard_open_handler() {
    let source = load_source("src/action_menu/view.rs");

    for needle in ["data-slot=\"action-menu\"", "on:keydown=on_key_down"] {
        assert!(
            source.contains(needle),
            "ActionMenu should set `{needle}` so it can be styled/tested as a Spectrum-like composite trigger."
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
        "use_presence",
        "<Popover",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should compose menu/popover/presence via `{needle}` to avoid popover unmounting before exit motion completes."
        );
    }
}
