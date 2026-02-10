use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_copy_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button_copy/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ButtonCopy internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn button_copy_uses_logic_state_model() {
    let view_source = load_source("src/button_copy/view.rs");
    let logic_source = load_source("src/button_copy/logic.rs");

    for needle in [
        "pub struct ButtonCopyViewState",
        "pub is_copyable: bool",
        "pub has_custom_label: bool",
        "pub fn normalize_optional_text(",
        "pub fn resolve_view_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ButtonCopy logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let label = logic::normalize_optional_text(label);",
        "let copied_label = logic::normalize_optional_text(copied_label);",
        "let view_state = logic::resolve_view_state(",
        "let class = logic::compose_class_name(class_name, view_state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ButtonCopy view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn button_copy_uses_snippet_logic_for_copy_behavior() {
    let source = load_source("src/button_copy/view.rs");

    for needle in [
        "crate::snippet::logic::use_snippet_logic(text.clone())",
        "on_press=logic.copy",
        "data-copied=move || logic.copied.get().then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should delegate copy behavior via `{needle}`."
        );
    }
}

#[test]
fn button_copy_forwards_button_contract_and_disabled_semantics() {
    let source = load_source("src/button_copy/view.rs");

    for needle in [
        "<Button",
        "variant=variant",
        "size=size",
        "motion=motion.button",
        "aria_label=aria_label",
        "disabled=!view_state.is_copyable",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should forward `{needle}` to the underlying Button."
        );
    }
}

#[test]
fn button_copy_emits_spectrum_style_data_attributes() {
    let source = load_source("src/button_copy/view.rs");

    for needle in [
        "data-slot=\"button-copy\"",
        "data-state=if view_state.is_copyable {",
        "data-copyable=view_state.is_copyable.then_some(\"true\")",
        "data-disabled=view_state.is_disabled.then_some(\"true\")",
        "data-empty=(!view_state.has_text).then_some(\"true\")",
        "data-label=if view_state.has_custom_label {",
        "data-copied-label=if view_state.has_custom_copied_label {",
        "data-motion-source=if motion == ButtonCopyMotion::default()",
        "data-custom-motion=(motion != ButtonCopyMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should expose `{needle}` for Spectrum-style state inspection."
        );
    }
}

#[test]
fn button_copy_announces_copy_result_for_assistive_tech() {
    let source = load_source("src/button_copy/view.rs");

    for needle in [
        "data-slot=\"button-copy-status\"",
        "aria-live=\"polite\"",
        "aria-atomic=\"true\"",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy a11y status element should include `{needle}`."
        );
    }
}

#[test]
fn button_copy_styles_include_motion_marker_contracts() {
    let source = load_source("src/button_copy/styles.rs");

    for selector in [
        ".ui-button-copy[data-motion-source=\"custom\"]",
        ".ui-button-copy[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ButtonCopy styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn button_copy_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/button_copy/motion.rs");

    for needle in [
        "pub struct ButtonCopyMotion",
        "fn default_motion_matches_button_contract_defaults()",
        "fn supports_custom_button_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy motion module should include `{needle}` for HeroUI-level motion contract coverage."
        );
    }
}
