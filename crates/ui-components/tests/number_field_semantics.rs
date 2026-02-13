use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn number_field_uses_headless_spinbutton_semantics() {
    let source = load_source("src/number_field/view.rs");

    assert!(
        source.contains("use_number_field"),
        "NumberField should delegate keyboard/editing behavior to `ui_headless::use_number_field`."
    );
    assert!(
        source.contains("role=number_field.input.role"),
        "NumberField input should use spinbutton role from the headless hook."
    );
    assert!(
        source.contains("aria-valuenow"),
        "NumberField input should expose `aria-valuenow` for spinbutton semantics."
    );
    assert!(
        source.contains("aria-valuemin"),
        "NumberField input should expose `aria-valuemin` when min is configured."
    );
    assert!(
        source.contains("aria-valuemax"),
        "NumberField input should expose `aria-valuemax` when max is configured."
    );
    assert!(
        source.contains("on:keydown=on_key_down"),
        "NumberField should handle ArrowUp/ArrowDown/PageUp/PageDown keys for stepping."
    );
}

#[test]
fn number_field_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/number_field/view.rs");

    for attr in [
        "data-focused",
        "data-focus-visible",
        "data-invalid",
        "data-disabled",
        "data-required",
    ] {
        assert!(
            source.contains(attr),
            "NumberField should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn number_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn number_field() -> AnyView",
        "title=\"NumberField\"",
        "slug=\"number-field\"",
        "description=\"Numeric input with steppers and keyboard control.\"",
        "<Playground title=\"Stepper\" code_signal=code>",
        "<NumberField",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for number-field primary playground coverage.",
        );
    }
}

#[test]
fn number_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Stepper\"",
        "id=\"docs-number-field\".to_string()",
        "label=\"Quantity\".to_string()",
        "min=0",
        "max=100",
        "value: ",
    ] {
        assert!(
            source.contains(needle),
            "number-field docs playground should contain `{needle}`.",
        );
    }
}
