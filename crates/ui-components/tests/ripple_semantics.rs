use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ripple_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/ripple/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Ripple internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn ripple_uses_logic_state_model() {
    let logic_source = load_source("src/ripple/logic.rs");
    let view_source = load_source("src/ripple/view.rs");

    for needle in [
        "pub enum RipplePhase",
        "pub enum RippleBoundary",
        "pub struct RippleStateInput",
        "pub struct RippleState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_phase(",
        "pub fn resolve_boundary(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "motion_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Ripple logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "motion::sanitize_motion(motion)",
        "logic::resolve_phase(motion.enabled)",
        "logic::resolve_boundary(bounded)",
        "logic::resolve_state(RippleStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Ripple view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn ripple_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/ripple/view.rs");

    for attr in [
        "data-slot=\"ripple\"",
        "data-state=state.phase_attr",
        "data-phase-class=state.phase_class",
        "data-boundary=state.boundary_attr",
        "data-bounded=state.is_bounded.then_some(\"true\")",
        "data-unbounded=state.is_unbounded.then_some(\"true\")",
        "data-motion-source=state.motion_source_attr",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-duration-ms=duration_ms_attr",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            source.contains(attr),
            "Ripple should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn ripple_styles_include_state_and_source_contracts() {
    let source = load_source("src/ripple/styles.rs");

    for selector in [
        ".ui-ripple--state-animated",
        ".ui-ripple[data-state=\"static\"]",
        ".ui-ripple--boundary-bounded",
        ".ui-ripple[data-boundary=\"unbounded\"]",
        ".ui-ripple[data-bounded=\"true\"]",
        ".ui-ripple[data-unbounded=\"true\"]",
        ".ui-ripple--motion-custom",
        ".ui-ripple[data-motion-source=\"custom\"]",
        ".ui-ripple--custom-class",
        ".ui-ripple[data-custom-class=\"true\"]",
        ".ui-ripple[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "Ripple styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn ripple_motion_sanitizes_and_supports_origin_triggering() {
    let source = load_source("src/ripple/motion.rs");

    for needle in [
        "pub fn sanitize_duration_ms(",
        "pub fn sanitize_motion(",
        "trigger_ripple_with_origin_internal",
        "pub fn trigger_ripple_at(",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-ripple-origin-x",
        "--ui-ripple-origin-y",
        "duration_ms: motion.duration_ms",
    ] {
        assert!(
            source.contains(needle),
            "Ripple motion should include `{needle}` for stable duration/origin contracts."
        );
    }
}

#[test]
fn ripple_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn motion_ripple() -> AnyView",
        "title=\"MotionRipple\"",
        "slug=\"motion-ripple\"",
        "title=\"Animation Matrix\"",
        "title=\"Custom Boundary + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for MotionRipple.",
        );
    }
}

#[test]
fn ripple_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let matrix_code = Signal::derive(move || {",
        "duration_ms: 880",
        "let custom_code = Signal::derive(move || {",
        "bounded=false",
        "duration_ms: 620",
        "duration_ms: 520",
        "ui_components::ripple::motion::trigger_ripple_at(",
        "18.0,",
        "48.0,",
        "class_name=\"docs-ripple-custom\".to_string()",
        "\"Unbounded + Origin\"",
    ] {
        assert!(
            source.contains(needle),
            "motion-ripple docs playgrounds should contain `{needle}`.",
        );
    }
}
