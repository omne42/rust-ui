use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn progress_circle_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/progress_circle/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ProgressCircle internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn progress_circle_uses_logic_state_model() {
    let view_source = load_source("src/progress_circle/view.rs");
    let logic_source = load_source("src/progress_circle/logic.rs");

    for needle in [
        "pub struct ProgressCircleStateInput",
        "pub struct ProgressCircleState",
        "pub struct ProgressCircleMetricsInput",
        "pub struct ProgressCircleResolvedMetrics",
        "pub enum ProgressCirclePhase",
        "pub fn normalize_optional_text(",
        "pub fn resolve_aria_label(",
        "pub fn resolve_value_label(",
        "pub fn sanitize_dimension(",
        "pub fn resolve_metrics(",
        "pub fn resolve_phase(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "value_label_source_attr",
        "motion_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ProgressCircle logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_value_label(value_label)",
        "logic::resolve_metrics(logic::ProgressCircleMetricsInput {",
        "logic::resolve_state(logic::ProgressCircleStateInput {",
        "logic::compose_class_name(class_name, state)",
        "logic::resolve_phase(is_indeterminate.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ProgressCircle view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn progress_circle_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/progress_circle/view.rs");

    for attr in [
        "data-slot=\"progress-circle\"",
        "data-state=move || logic::resolve_phase(is_indeterminate.get()).as_str()",
        "data-phase-class=move || logic::resolve_phase(is_indeterminate.get()).class_name()",
        "data-indeterminate=move || is_indeterminate.get().then_some(\"true\")",
        "data-determinate=move || (!is_indeterminate.get()).then_some(\"true\")",
        "data-size-source=state.size_source_attr",
        "data-stroke-source=state.stroke_source_attr",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-size=state.has_custom_size.then_some(\"true\")",
        "data-custom-stroke=state.has_custom_stroke_width.then_some(\"true\")",
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
            "ProgressCircle should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn progress_circle_styles_include_state_source_contracts() {
    let source = load_source("src/progress_circle/styles.rs");

    for selector in [
        ".ui-progress-circle--label-custom",
        ".ui-progress-circle[data-label-source=\"custom\"]",
        ".ui-progress-circle--size-custom",
        ".ui-progress-circle[data-size-source=\"custom\"]",
        ".ui-progress-circle--stroke-custom .ui-progress-circle__track",
        ".ui-progress-circle[data-stroke-source=\"custom\"] .ui-progress-circle__track",
        ".ui-progress-circle--motion-custom",
        ".ui-progress-circle[data-motion-source=\"custom\"]",
        ".ui-progress-circle--custom-class",
        ".ui-progress-circle[data-custom-class=\"true\"]",
        ".ui-progress-circle--state-indeterminate .ui-progress-circle__svg",
        ".ui-progress-circle[data-state=\"indeterminate\"] .ui-progress-circle__svg",
        ".ui-progress-circle--state-determinate .ui-progress-circle__svg",
        ".ui-progress-circle[data-state=\"determinate\"] .ui-progress-circle__svg",
    ] {
        assert!(
            source.contains(selector),
            "ProgressCircle styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn progress_circle_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/progress_circle/motion.rs");
    let view_source = load_source("src/progress_circle/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ProgressCircleMotion) -> ProgressCircleMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let motion = StoredValue::new(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "ProgressCircle motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source
            .contains("let motion = crate::progress_circle::motion::sanitize_motion(motion);"),
        "ProgressCircle view should sanitize motion before attaching spring driver.",
    );
}

#[test]
fn progress_circle_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn progress_circle() -> AnyView",
        "title=\"ProgressCircle\"",
        "slug=\"progress-circle\"",
        "Playground title=\"Determinate + Indeterminate\"",
        "Playground title=\"Custom Value Label + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for ProgressCircle.",
        );
    }
}

#[test]
fn progress_circle_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Determinate + Indeterminate\"",
        "<ProgressCircle aria_label=\"Determinate\".to_string() value=progress_value min=0.0 max=100.0 />",
        "<ProgressCircle aria_label=\"Indeterminate\".to_string() value=Signal::derive(|| None) />",
        "on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 10.0).min(100.0)))",
        "title=\"Custom Value Label + Class\"",
        "aria_label=\"Sync progress\".to_string()",
        "value=Signal::derive(|| Some(64.0))",
        "size_px=40.0",
        "stroke_width_px=5.0",
        "value_label=\"64 done\".to_string()",
        "aria_label=\"   \".to_string()",
        "class_name=\"docs-progress-circle-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "progress-circle docs playgrounds should contain `{needle}`.",
        );
    }
}
