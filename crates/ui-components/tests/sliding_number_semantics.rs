use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn number_module_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/number/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Number internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn sliding_number_uses_logic_state_model() {
    let view_source = load_source("src/number/view.rs");
    let logic_source = load_source("src/number/logic.rs");

    for needle in [
        "pub struct SlidingNumberStateInput",
        "pub struct SlidingNumberState",
        "pub enum SlidingNumberPhase",
        "pub fn resolve_sliding_phase(",
        "pub fn resolve_sliding_number_state(",
        "pub fn compose_sliding_number_class_name(",
        "decimal_separator_source_attr",
        "decimal_places_source_attr",
        "thousand_separator_source_attr",
        "motion_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Number logic should include `{needle}` for centralized sliding-number state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_decimal_separator(decimal_separator)",
        "logic::sanitize_decimal_places(decimal_places)",
        "logic::resolve_thousand_separator(thousand_separator)",
        "logic::resolve_sliding_number_state(logic::SlidingNumberStateInput {",
        "logic::compose_sliding_number_class_name(class_name.get_value(), state.get())",
        "motion::attach_motion",
    ] {
        assert!(
            view_source.contains(needle),
            "SlidingNumber view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn sliding_number_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/number/view.rs");

    for attr in [
        "data-slot=\"sliding-number\"",
        "data-state=move || state.get().phase_attr",
        "data-phase-class=move || state.get().phase_class",
        "data-sign=move || state.get().sign_attr",
        "data-animated=move || state.get().is_animated.then_some(\"true\")",
        "data-static=move || state.get().is_static.then_some(\"true\")",
        "data-decimal-separator-source=move || state.get().decimal_separator_source_attr",
        "data-decimal-places-source=move || state.get().decimal_places_source_attr",
        "data-thousand-separator-source=move || state.get().thousand_separator_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-decimal-separator=move || {",
        "data-custom-decimal-places=move || {",
        "data-custom-thousand-separator=move || {",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            source.contains(attr),
            "SlidingNumber should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn sliding_number_styles_include_sign_phase_and_source_contracts() {
    let source = load_source("src/number/styles.rs");

    for selector in [
        ".ui-sliding-number--sign-negative",
        ".ui-sliding-number[data-sign=\"zero\"]",
        ".ui-sliding-number--state-animated",
        ".ui-sliding-number[data-state=\"static\"]",
        ".ui-sliding-number--decimal-separator-custom .ui-sliding-number__separator",
        ".ui-sliding-number[data-decimal-separator-source=\"custom\"] .ui-sliding-number__separator",
        ".ui-sliding-number--decimal-places-custom .ui-sliding-number__digit",
        ".ui-sliding-number[data-decimal-places-source=\"custom\"] .ui-sliding-number__digit",
        ".ui-sliding-number--thousand-separator-custom .ui-sliding-number__separator",
        ".ui-sliding-number[data-thousand-separator-source=\"custom\"] .ui-sliding-number__separator",
        ".ui-sliding-number--motion-custom",
        ".ui-sliding-number[data-motion-source=\"custom\"]",
        ".ui-sliding-number--custom-class",
        ".ui-sliding-number[data-custom-class=\"true\"]",
        "--ui-sliding-number-offset",
    ] {
        assert!(
            source.contains(selector),
            "SlidingNumber styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn sliding_number_motion_uses_spring_animator() {
    let source = load_source("src/number/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "SlidingNumber motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn sliding_number_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/number/motion.rs");
    let view_source = load_source("src/number/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SlidingNumberMotion) -> SlidingNumberMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let motion = sanitize_motion(motion);",
        "let motion = StoredValue::new(motion);",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "SlidingNumber motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    for needle in [
        "let motion = crate::number::motion::sanitize_motion(motion);",
        "motion::attach_motion(roller_ref, digit, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "SlidingNumber view should include `{needle}` to sanitize motion at composition boundaries.",
        );
    }
}

#[test]
fn sliding_number_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn sliding_number() -> AnyView",
        "title=\"SlidingNumber\"",
        "slug=\"sliding-number\"",
        "Playground title=\"Animated Matrix\"",
        "Playground title=\"Custom Separators + Motion + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for SlidingNumber.",
        );
    }
}

#[test]
fn sliding_number_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let (value, set_value) = signal(12345.67_f64);",
        "number=number_signal",
        "thousand_separator=\",\".to_string()",
        "on_press=Callback::new(move |_| set_value.update(|v| *v += 250.0))",
        "on_press=Callback::new(move |_| set_value.update(|v| *v -= 100.0))",
        "title=\"Custom Separators + Motion + Class\"",
        "number=Signal::derive(|| 42123.456)",
        "decimal_separator=\",\".to_string()",
        "decimal_places=30",
        "thousand_separator=\" \".to_string()",
        "number=Signal::derive(|| f64::NAN)",
        "animate: false",
        "class_name=\"docs-sliding-number-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "sliding-number docs playgrounds should contain `{needle}`.",
        );
    }
}
