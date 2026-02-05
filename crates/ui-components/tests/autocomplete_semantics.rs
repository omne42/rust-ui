use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn autocomplete_escape_stops_propagation_when_open() {
    let source = load_source("src/autocomplete/view.rs");

    assert!(
        source.contains("stop_propagation()"),
        "Autocomplete should stop Escape from bubbling when its popup is open (so parent overlays don't close)."
    );
    assert!(
        source.contains("key == \"Escape\""),
        "Autocomplete should conditionally stop propagation only for Escape."
    );
    assert!(
        source.contains("was_open"),
        "Autocomplete should only stop propagation when it was open (so Escape still closes parent overlays when closed)."
    );
}
