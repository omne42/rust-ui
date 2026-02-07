use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn code_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/code/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Code internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn code_uses_logic_state_model() {
    let view_source = load_source("src/code/view.rs");
    let logic_source = load_source("src/code/logic.rs");

    for needle in [
        "pub struct CodeStateInput",
        "pub struct CodeState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: CodeStateInput)",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Code logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(CodeStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Code view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn code_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/code/view.rs");

    for attr in [
        "data-slot=\"code\"",
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-inline=state.is_inline.then_some(\"true\")",
        "data-block=state.is_block.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Code should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn code_styles_include_variant_and_state_markers() {
    let source = load_source("src/code/styles.rs");

    for selector in [
        ".ui-code--variant-inline",
        ".ui-code[data-variant=\"inline\"]",
        ".ui-code--variant-block",
        ".ui-code[data-variant=\"block\"]",
        ".ui-code--state-inline",
        ".ui-code[data-state=\"block\"]",
        ".ui-code[data-block=\"true\"]",
        ".ui-code--custom-class",
        ".ui-code[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Code styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn code_keeps_code_element_slot_contract() {
    let source = load_source("src/code/view.rs");

    for needle in ["<code", "data-slot=\"code\""] {
        assert!(
            source.contains(needle),
            "Code view should retain `{needle}` markup contract."
        );
    }
}
