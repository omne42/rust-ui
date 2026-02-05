use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn text_area_supports_read_only() {
    let source = load_source("src/text_area/view.rs");

    assert!(
        source.contains("read_only: bool"),
        "TextArea should accept a `read_only` prop to match Spectrum-style text area contracts."
    );

    assert!(
        source.contains("readonly=read_only"),
        "TextArea should forward `read_only` to the underlying <textarea readonly> attribute."
    );
}
