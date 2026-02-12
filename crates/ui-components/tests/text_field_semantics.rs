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

#[test]
fn text_field_styles_respect_prefers_reduced_motion() {
    let source = load_source("src/text_field/styles.rs");

    assert!(
        source.contains("prefers-reduced-motion: reduce"),
        "TextField styles should respect prefers-reduced-motion to avoid forced transitions."
    );
    assert!(
        source.contains("transition: none;"),
        "TextField styles should disable transitions under prefers-reduced-motion."
    );
}

#[test]
fn text_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn text_field() -> AnyView",
        "title=\"TextField\"",
        "slug=\"text-field\"",
        "description=\"A compact field wrapper built on headless text field semantics.\"",
        "<Playground title=\"Label + placeholder\" code=code>",
        "<TextField",
    ] {
        assert!(
            source.contains(needle),
            "forms text_field docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn text_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Label + placeholder\"",
        "id=\"docs-text-field\".to_string()",
        "label=\"Name\".to_string()",
        "value=value",
        "set_value=set_value",
        "placeholder=\"Jane\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "forms text_field docs playground should contain `{needle}` for state-matrix contracts.",
        );
    }
}
