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
        "data-state=move || data_state.get()",
        "data-value=move || data_value.get()",
        "data-requirement=move || data_requirement.get()",
        "data-label-source=label_source_attr",
        "data-description-source=description_source_attr",
        "data-error-source=error_source_attr",
        "data-placeholder-source=placeholder_source_attr",
        "data-type-source=type_source_attr",
        "data-class-source=class_source_attr",
        "data-custom-class=has_custom_class_name.then_some(\"true\")",
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
fn text_field_styles_include_state_and_source_markers() {
    let source = load_source("src/text_field/styles.rs");

    for selector in [
        "prefers-reduced-motion: reduce",
        "transition: none;",
        ".ui-text-field[data-state=\"disabled\"] .ui-text-field__input",
        ".ui-text-field[data-state=\"invalid\"] .ui-text-field__input",
        ".ui-text-field[data-state=\"readonly\"] .ui-text-field__input",
        ".ui-text-field[data-value=\"filled\"]",
        ".ui-text-field[data-requirement=\"required\"]",
        ".ui-text-field[data-description-source=\"custom\"]",
        ".ui-text-field[data-error-source=\"custom\"]",
        ".ui-text-field[data-placeholder-source=\"custom\"]",
        ".ui-text-field[data-type-source=\"custom\"]",
        ".ui-text-field[data-class-source=\"custom\"]",
        ".ui-text-field[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "TextField styles should include `{selector}` as stable state/source selectors."
        );
    }
}

#[test]
fn text_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");

    for needle in [
        "pub(super) fn text_field() -> AnyView",
        "title=\"TextField\"",
        "slug=\"text-field\"",
        "description=\"A compact field wrapper built on headless text field semantics with explicit state/source marker contracts.\"",
        "<Playground title=\"Label + placeholder\" code_signal=code>",
        "title=\"State + Source Markers\"",
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
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");

    for needle in [
        "title=\"Label + placeholder\"",
        "id=\"docs-text-field\".to_string()",
        "label=\"Name\".to_string()",
        "value=value",
        "set_value=set_value",
        "placeholder=\"Jane\".to_string()",
        "title=\"State + Source Markers\"",
        "id=\"docs-text-field-markers\".to_string()",
        "label=\"Email\".to_string()",
        "required=true",
        "invalid=Signal::derive(move || marker_invalid.get())",
        "read_only=marker_read_only.get()",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Email is required\".to_string()",
        "placeholder=\"release@omne.rs\".to_string()",
        "input_type=\"email\"",
        "class_name=\"docs-text-field-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "forms text_field docs playground should contain `{needle}` for state-matrix contracts.",
        );
    }
}
