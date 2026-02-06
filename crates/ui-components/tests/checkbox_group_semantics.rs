use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn checkbox_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/checkbox_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CheckboxGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_resolves_ids_and_normalizes_text_inputs() {
    let view_source = load_source("src/checkbox_group/view.rs");
    let logic_source = load_source("src/checkbox_group/logic.rs");

    for needle in [
        "resolve_ids",
        "normalize_label",
        "normalize_optional_text",
        "aria-labelledby=legend_id.get_value()",
        "id=legend_id.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup should wire `{needle}` for stable labeling and normalized text content."
        );
    }

    assert!(
        logic_source.contains("\"Options\".to_string()"),
        "CheckboxGroup label normalization should default empty labels to a stable fallback."
    );
}

#[test]
fn checkbox_group_uses_headless_text_field_contract() {
    let source = load_source("src/checkbox_group/logic.rs");

    for needle in [
        "use_text_field",
        "CheckboxGroupFieldsetAttrs",
        "aria_describedby",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxGroup should delegate describedby/invalid/required modeling via `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_emits_spectrum_state_data_attributes() {
    let source = load_source("src/checkbox_group/view.rs");

    for needle in [
        "data-slot=\"checkbox-group\"",
        "data-disabled=disabled.then_some(\"true\")",
        "data-invalid=move || invalid.get().then_some(\"true\")",
        "data-required=move || required.get().then_some(\"true\")",
        "data-has-description=has_description.then_some(\"true\")",
        "data-has-error=has_error.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxGroup should expose `{needle}` for Spectrum-style state styling and inspection."
        );
    }
}

#[test]
fn checkbox_group_only_renders_error_slot_when_invalid() {
    let source = load_source("src/checkbox_group/view.rs");

    for needle in [
        "<Show when=move || invalid.get()>",
        "data-slot=\"checkbox-group-error\"",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxGroup should guard error rendering via `{needle}`."
        );
    }
}
