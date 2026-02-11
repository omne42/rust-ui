use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn progress_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/progress/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Progress internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn progress_uses_logic_state_model() {
    let view_source = load_source("src/progress/view.rs");
    let logic_source = load_source("src/progress/logic.rs");

    for needle in [
        "pub struct ProgressStateInput",
        "pub struct ProgressState",
        "pub enum ProgressPhase",
        "pub fn normalize_optional_text(",
        "pub fn resolve_aria_label(",
        "pub fn resolve_value_label(",
        "pub fn resolve_phase(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "value_label_source_attr",
        "motion_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Progress logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_value_label(value_label)",
        "logic::resolve_state(logic::ProgressStateInput {",
        "logic::compose_class_name(class_name, state)",
        "logic::resolve_phase(is_indeterminate.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Progress view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn progress_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/progress/view.rs");

    for attr in [
        "data-slot=\"progress\"",
        "data-state=move || phase.get().as_str()",
        "data-phase-class=move || phase.get().class_name()",
        "data-indeterminate=move ||",
        "data-determinate=move ||",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-aria-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-value-label=state.has_custom_value_label.then_some(\"true\")",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "role=\"progressbar\"",
        "aria-valuetext=move || value_label_text.get()",
    ] {
        assert!(
            source.contains(attr),
            "Progress should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn progress_styles_include_state_source_contracts() {
    let source = load_source("src/progress/styles.rs");

    for selector in [
        ".ui-progress--label-custom",
        ".ui-progress[data-label-source=\"custom\"]",
        ".ui-progress--value-label-custom .ui-progress__track",
        ".ui-progress[data-value-label-source=\"custom\"] .ui-progress__track",
        ".ui-progress--motion-custom",
        ".ui-progress[data-motion-source=\"custom\"]",
        ".ui-progress--custom-class",
        ".ui-progress[data-custom-class=\"true\"]",
        ".ui-progress--state-indeterminate .ui-progress__indicator",
        ".ui-progress[data-state=\"indeterminate\"] .ui-progress__indicator",
        ".ui-progress--state-determinate .ui-progress__indicator",
        ".ui-progress[data-state=\"determinate\"] .ui-progress__indicator",
    ] {
        assert!(
            source.contains(selector),
            "Progress styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn progress_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/progress/motion.rs");
    let view_source = load_source("src/progress/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ProgressMotion) -> ProgressMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Progress motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::progress::motion::sanitize_motion(motion);"),
        "Progress view should sanitize motion before attaching spring driver.",
    );
}
