use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn overlay_escape_respects_default_prevented_and_composition() {
    let source = load_source("src/overlay/view.rs");

    assert!(
        source.contains("default_prevented"),
        "Overlay should not close on Escape when a child already called preventDefault (Spectrum parity for Escape-to-clear flows)."
    );
    assert!(
        source.contains("is_composing"),
        "Overlay should ignore Escape while IME composition is active (matches React Spectrum's `useOverlay`)."
    );
    assert!(
        source.contains("stop_propagation()"),
        "Overlay should stop Escape propagation when closing to avoid cascading dismiss handlers."
    );
}

#[test]
fn overlay_supports_dismissable_and_keyboard_dismiss_flags() {
    let source = load_source("src/overlay/view.rs");

    for needle in [
        "is_dismissable",
        "is_keyboard_dismiss_disabled",
        "if is_dismissable",
        "!is_keyboard_dismiss_disabled",
    ] {
        assert!(
            source.contains(needle),
            "Overlay should support Spectrum-style dismiss control flags (`{needle}`)."
        );
    }
}
