use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn meter_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/meter/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Meter internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn meter_uses_logic_state_model() {
    let view_source = load_source("src/meter/view.rs");
    let logic_source = load_source("src/meter/logic.rs");

    for needle in [
        "pub struct MeterStateInput",
        "pub struct MeterState",
        "pub enum MeterPhase",
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
            "Meter logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_optional_text(label)",
        "logic::resolve_aria_label(aria_label, label.clone())",
        "logic::resolve_value_label(value_label)",
        "logic::resolve_state(logic::MeterStateInput {",
        "logic::compose_class_name(class_name, state)",
        "logic::resolve_phase(is_indeterminate.get())",
        "motion::attach_motion",
    ] {
        assert!(
            view_source.contains(needle),
            "Meter view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn meter_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/meter/view.rs");

    for attr in [
        "data-slot=\"meter\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=move || phase.get().as_str()",
        "data-phase-class=move || phase.get().class_name()",
        "data-indeterminate=move || {",
        "data-determinate=move || {",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-aria-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-value-label=state.has_custom_value_label.then_some(\"true\")",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "role=\"meter\"",
        "aria-valuetext=move || value_label_text.get()",
    ] {
        assert!(
            source.contains(attr),
            "Meter should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn meter_styles_include_state_source_contracts() {
    let source = load_source("src/meter/styles.rs");

    for selector in [
        ".ui-meter--variant-default",
        ".ui-meter[data-variant=\"danger\"]",
        ".ui-meter--size-lg .ui-meter__track",
        ".ui-meter[data-size=\"sm\"] .ui-meter__track",
        ".ui-meter--label-custom .ui-meter__label",
        ".ui-meter[data-label-source=\"custom\"] .ui-meter__label",
        ".ui-meter--value-label-custom .ui-meter__value-label",
        ".ui-meter[data-value-label-source=\"custom\"] .ui-meter__value-label",
        ".ui-meter--motion-custom",
        ".ui-meter[data-motion-source=\"custom\"]",
        ".ui-meter--custom-class",
        ".ui-meter[data-custom-class=\"true\"]",
        ".ui-meter--state-indeterminate .ui-meter__indicator",
        ".ui-meter[data-state=\"indeterminate\"] .ui-meter__indicator",
        ".ui-meter--state-determinate .ui-meter__indicator",
        ".ui-meter[data-state=\"determinate\"] .ui-meter__indicator",
    ] {
        assert!(
            source.contains(selector),
            "Meter styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn meter_motion_uses_spring_animator() {
    let source = load_source("src/meter/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Meter motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn meter_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/meter/motion.rs");
    let view_source = load_source("src/meter/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: MeterMotion) -> MeterMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Meter motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::meter::motion::sanitize_motion(motion);"),
        "Meter view should sanitize motion before attaching spring driver.",
    );
}
