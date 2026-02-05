use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sheet_escape_respects_default_prevented_and_composition() {
    let source = load_source("src/sheet/view.rs");

    assert!(
        source.contains("default_prevented"),
        "Sheet should not close on Escape when a child already called preventDefault (Spectrum parity for Escape-to-clear flows)."
    );
    assert!(
        source.contains("is_composing"),
        "Sheet should ignore Escape while IME composition is active (matches React Spectrum's `useOverlay`)."
    );
    assert!(
        source.contains("stop_propagation()"),
        "Sheet should stop Escape propagation when closing to avoid cascading dismiss handlers."
    );
}
