use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn select_does_not_expose_logic_module() {
    let source = load_source("src/select/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Select's `logic` module should stay private to avoid leaking internal behavior helpers into the public API."
    );
}

#[test]
fn select_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "Select should accept `{needle}` to support controlled/uncontrolled open state."
        );
    }
}

#[test]
fn select_trigger_is_labeled_and_owns_a_listbox() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "aria_haspopup=\"listbox\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_labelledby=trigger_id.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "Select should wire `{needle}` for Spectrum-style listbox trigger semantics."
        );
    }
}

#[test]
fn select_uses_presence_to_allow_exit_motion() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "use_presence(open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Select should use presence for motion-friendly unmounting via `{needle}`."
        );
    }
}
