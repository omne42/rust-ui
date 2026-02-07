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
fn checkbox_group_uses_logic_state_model() {
    let view_source = load_source("src/checkbox_group/view.rs");
    let logic_source = load_source("src/checkbox_group/logic.rs");

    for needle in [
        "pub struct CheckboxGroupState",
        "pub fn resolve_state(",
        "pub is_disabled: bool",
        "pub is_invalid: bool",
        "pub shows_error: bool",
        "pub has_messages: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "resolve_state(",
        "state.get().shows_error",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup view should derive root state via logic::resolve_state; missing `{needle}`."
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
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-valid=move || state.get().is_valid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-optional=move || state.get().is_optional.then_some(\"true\")",
        "data-has-description=move || state.get().has_description.then_some(\"true\")",
        "data-has-error=move || state.get().has_error.then_some(\"true\")",
        "data-shows-error=move || state.get().shows_error.then_some(\"true\")",
        "data-has-messages=move || state.get().has_messages.then_some(\"true\")",
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
        "<Show when=move || state.get().shows_error>",
        "data-slot=\"checkbox-group-error\"",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxGroup should guard error rendering via `{needle}`."
        );
    }
}
