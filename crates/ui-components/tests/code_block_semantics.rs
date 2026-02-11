use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn code_block_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/code_block/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CodeBlock internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn code_block_uses_logic_state_model() {
    let view_source = load_source("src/code_block/view.rs");
    let logic_source = load_source("src/code_block/logic.rs");

    for needle in [
        "pub struct CodeBlockStateInput",
        "pub struct CodeBlockViewState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: CodeBlockStateInput)",
        "pub fn resolve_view_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CodeBlock logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(label)",
        "logic::normalize_optional_text(language)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(CodeBlockStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn code_block_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/code_block/view.rs");

    for attr in [
        "data-slot=\"code-block\"",
        "data-state=state.state_attr",
        "data-header=state.header_attr",
        "data-multiline=state.is_multiline.then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-label=state.has_label.then_some(\"true\")",
        "data-language=state.has_language.then_some(\"true\")",
        "data-copyable=state.copyable.then_some(\"true\")",
        "data-motion-source=state.motion_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-slot=\"code-block-status\"",
    ] {
        assert!(
            source.contains(attr),
            "CodeBlock should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn code_block_styles_include_state_marker_contracts() {
    let source = load_source("src/code_block/styles.rs");

    for selector in [
        ".ui-code-block--state-multiline",
        ".ui-code-block[data-state=\"single-line\"]",
        ".ui-code-block--header-visible",
        ".ui-code-block[data-header=\"hidden\"]",
        ".ui-code-block--copyable",
        ".ui-code-block[data-motion-source=\"custom\"]",
        ".ui-code-block--custom-class",
        ".ui-code-block[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "CodeBlock styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn code_block_does_not_ignore_motion_contract() {
    let source = load_source("src/code_block/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "CodeBlock should honor `CodeBlockMotion` rather than ignoring it."
    );
}

#[test]
fn code_block_attaches_motion_driver() {
    let source = load_source("src/code_block/view.rs");

    assert!(
        source.contains("attach_motion"),
        "CodeBlock should attach its motion driver to deliver copy feedback motion."
    );
}

#[test]
fn code_block_styles_define_css_vars_for_motion() {
    let source = load_source("src/code_block/styles.rs");

    assert!(
        source.contains("--ui-code-block-copy-flash"),
        "CodeBlock styles should define `--ui-code-block-copy-flash` so motion updates only touch CSS variables."
    );
}

#[test]
fn code_block_motion_uses_spring_animator() {
    let source = load_source("src/code_block/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "CodeBlock motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn code_block_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/code_block/motion.rs");
    let view_source = load_source("src/code_block/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: CodeBlockMotion) -> CodeBlockMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "CodeBlock motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::code_block::motion::sanitize_motion(motion);"),
        "CodeBlock view should sanitize motion before attaching copy-flash driver.",
    );
}
