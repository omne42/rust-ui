use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn separator_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/separator/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Separator internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn separator_uses_logic_state_model() {
    let view_source = load_source("src/separator/view.rs");
    let logic_source = load_source("src/separator/logic.rs");

    for needle in [
        "pub struct SeparatorStateInput",
        "pub struct SeparatorState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Separator logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(SeparatorStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Separator view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn separator_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/separator/view.rs");

    for attr in [
        "data-slot=\"separator\"",
        "data-state=state.state_attr",
        "data-orientation=state.orientation_attr",
        "data-element=state.element_attr",
        "data-decorative=state.is_decorative.then_some(\"true\")",
        "data-semantic=state.is_semantic.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "role=role",
        "aria-orientation=aria_orientation",
    ] {
        assert!(
            source.contains(attr),
            "Separator should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn separator_styles_include_state_marker_contracts() {
    let source = load_source("src/separator/styles.rs");

    for selector in [
        ".ui-separator--horizontal",
        ".ui-separator[data-orientation=\"vertical\"]",
        ".ui-separator--element-hr",
        ".ui-separator[data-element=\"div\"]",
        ".ui-separator--semantic",
        ".ui-separator[data-state=\"semantic\"]",
        ".ui-separator--decorative",
        ".ui-separator[data-state=\"decorative\"]",
        ".ui-separator[data-decorative=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Separator styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn separator_does_not_ignore_motion_contract() {
    let source = load_source("src/separator/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "Separator should honor `SeparatorMotion` rather than ignoring it."
    );
}

#[test]
fn separator_attaches_motion_driver() {
    let source = load_source("src/separator/view.rs");

    assert!(
        source.contains("attach_motion"),
        "Separator should attach its motion driver when `SeparatorMotion` requests animation."
    );
}

#[test]
fn separator_styles_use_only_css_variables_for_motion() {
    let source = load_source("src/separator/styles.rs");

    for name in [
        "--ui-separator-scale-x",
        "--ui-separator-scale-y",
        "--ui-separator-opacity",
    ] {
        assert!(
            source.contains(name),
            "Separator styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn separator_motion_uses_spring_animator() {
    let source = load_source("src/separator/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Separator motion should animate via a spring to match the repo's motion spec."
    );
}
