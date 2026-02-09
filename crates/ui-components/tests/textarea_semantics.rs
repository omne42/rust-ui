use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn textarea_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/textarea/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Textarea;"),
        "textarea module should export `Textarea`."
    );
    assert!(
        crate_source.contains("pub use textarea::Textarea;"),
        "crate root should re-export `Textarea`."
    );
}

#[test]
fn textarea_view_has_textfield_a11y_contracts() {
    let source = load_source("src/textarea/view.rs");

    for needle in [
        "data-slot=\"textarea\"",
        "data-slot=\"textarea-input\"",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
    ] {
        assert!(
            source.contains(needle),
            "Textarea should include `{needle}` to preserve text-field accessibility semantics."
        );
    }
}

#[test]
fn textarea_view_exposes_state_markers() {
    let source = load_source("src/textarea/view.rs");

    for needle in [
        "data-focused=",
        "data-focus-visible=",
        "data-invalid=",
        "data-disabled=",
        "data-read-only=",
        "data-required=",
    ] {
        assert!(
            source.contains(needle),
            "Textarea should expose `{needle}` for stable Spectrum-style state contracts."
        );
    }
}

#[test]
fn textarea_css_exposes_expected_selectors() {
    let css = load_source("src/textarea/styles.rs");

    for needle in [
        ".ui-textarea {",
        ".ui-textarea__textarea {",
        ".ui-textarea--invalid .ui-textarea__textarea",
        ".ui-textarea__textarea:disabled",
    ] {
        assert!(
            css.contains(needle),
            "Textarea CSS should include `{needle}` selector."
        );
    }
}
