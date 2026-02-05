use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn text_field_uses_headless_text_field_and_focus_ring() {
    let source = load_source("src/text_field/view.rs");

    for needle in ["use_focus_ring", "use_text_field"] {
        assert!(
            source.contains(needle),
            "TextField should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn text_field_supports_read_only() {
    let source = load_source("src/text_field/view.rs");

    assert!(
        source.contains("read_only: bool"),
        "TextField should accept a `read_only` prop to match Spectrum-style text field contracts."
    );

    assert!(
        source.contains("readonly=read_only"),
        "TextField should forward `read_only` to the underlying <input readonly> attribute."
    );
}

#[test]
fn text_field_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/text_field/view.rs");

    for attr in [
        "data-focused",
        "data-focus-visible",
        "data-invalid",
        "data-disabled",
        "data-read-only",
        "data-required",
    ] {
        assert!(
            source.contains(attr),
            "TextField should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}
