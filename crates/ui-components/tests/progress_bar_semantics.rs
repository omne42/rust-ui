use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn progress_bar_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/progress_bar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ProgressBar internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn progress_bar_uses_logic_state_model() {
    let view_source = load_source("src/progress_bar/view.rs");
    let logic_source = load_source("src/progress_bar/logic.rs");

    for needle in [
        "pub struct ProgressBarStateInput",
        "pub struct ProgressBarState",
        "pub enum ProgressBarPhase",
        "pub fn normalize_optional_text(",
        "pub fn resolve_aria_label(",
        "pub fn sanitize_max(",
        "pub fn sanitize_value(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ProgressBar logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_state(logic::ProgressBarStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "ProgressBar view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn progress_bar_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/progress_bar/view.rs");

    for attr in [
        "data-slot=\"progress-bar\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=state.phase_attr",
        "data-indeterminate=state.is_indeterminate.then_some(\"true\")",
        "data-determinate=state.is_determinate.then_some(\"true\")",
        "data-has-value=state.has_value.then_some(\"true\")",
        "data-label-source=state.label_source_attr",
        "data-custom-aria-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "aria-label=aria_label",
        "max=state.max.to_string()",
        "value=state.value.map(|value| value.to_string())",
    ] {
        assert!(
            source.contains(attr),
            "ProgressBar should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn progress_bar_styles_include_variant_size_and_phase_contracts() {
    let source = load_source("src/progress_bar/styles.rs");

    for selector in [
        ".ui-progress-bar--variant-default",
        ".ui-progress-bar[data-variant=\"accent\"]",
        ".ui-progress-bar--size-lg",
        ".ui-progress-bar[data-size=\"sm\"]",
        ".ui-progress-bar--state-indeterminate::-webkit-progress-value",
        ".ui-progress-bar[data-state=\"indeterminate\"]::-moz-progress-bar",
        ".ui-progress-bar--state-determinate::-webkit-progress-value",
        ".ui-progress-bar[data-state=\"determinate\"]::-moz-progress-bar",
        ".ui-progress-bar--label-custom",
        ".ui-progress-bar[data-label-source=\"custom\"]",
        ".ui-progress-bar--custom-class",
        ".ui-progress-bar[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ProgressBar styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
