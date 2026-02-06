use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn text_area_uses_headless_text_field_and_focus_ring() {
    let source = load_source("src/text_area/view.rs");

    for needle in ["use_focus_ring", "use_text_field"] {
        assert!(
            source.contains(needle),
            "TextArea should use headless `{needle}` hooks."
        );
    }
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

#[test]
fn text_area_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/text_area/view.rs");

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
            "TextArea should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn text_area_styles_respect_prefers_reduced_motion() {
    let source = load_source("src/text_area/styles.rs");

    assert!(
        source.contains("prefers-reduced-motion: reduce"),
        "TextArea styles should respect prefers-reduced-motion to avoid forced transitions."
    );
    assert!(
        source.contains("transition: none;"),
        "TextArea styles should disable transitions under prefers-reduced-motion."
    );
}
