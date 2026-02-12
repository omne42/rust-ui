use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn slider_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/slider/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Slider internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn slider_uses_logic_state_model() {
    let logic_source = load_source("src/slider/logic.rs");
    let view_source = load_source("src/slider/view.rs");

    for needle in [
        "pub enum SliderPhase",
        "pub struct SliderStateInput",
        "pub struct SliderState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_label(",
        "pub fn sanitize_bounds(",
        "pub fn sanitize_step(",
        "pub fn parse_value(",
        "pub fn sanitize_value(",
        "pub fn resolve_percent(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "motion_source_attr",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Slider logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::resolve_label(label)",
        "logic::normalize_optional_text(class_name)",
        "logic::sanitize_bounds(min, max)",
        "logic::sanitize_step(step, min, max)",
        "motion::sanitize_motion(motion)",
        "logic::resolve_state(SliderStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "motion::attach_motion(root_ref, visual_percent, motion)",
    ] {
        assert!(
            view_source.contains(needle),
            "Slider view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn slider_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/slider/view.rs");

    for attr in [
        "data-slot=\"slider\"",
        "data-state=move || state.get().phase_attr",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-custom-label=move || state.get().has_custom_label.then_some(\"true\")",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-value-percent=move || state.get().value_percent.to_string()",
        "aria-valuetext=move || format!(\"{:.0}%\", state.get().value_percent)",
    ] {
        assert!(
            source.contains(attr),
            "Slider should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn slider_styles_include_state_and_source_contracts() {
    let source = load_source("src/slider/styles.rs");

    for selector in [
        ".ui-slider--state-disabled",
        ".ui-slider[data-state=\"disabled\"]",
        ".ui-slider[data-disabled=\"true\"]",
        ".ui-slider--motion-custom",
        ".ui-slider[data-motion-source=\"custom\"]",
        ".ui-slider--label-custom",
        ".ui-slider[data-label-source=\"custom\"]",
        ".ui-slider--custom-class",
        ".ui-slider[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Slider styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn slider_motion_uses_spring_driver() {
    let source = load_source("src/slider/motion.rs");

    for needle in [
        "pub fn sanitize_percent(",
        "pub fn sanitize_motion(",
        "ui_motion::spring::SpringAnimator::new",
        "spring.set_target(target)",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-slider-visual-percent",
    ] {
        assert!(
            source.contains(needle),
            "Slider motion should include `{needle}` for spring-driven track updates."
        );
    }
}

#[test]
fn slider_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn slider() -> AnyView",
        "title=\"Slider\"",
        "slug=\"slider\"",
        "description=\"Range slider with spring-driven fill/thumb motion and Spectrum-style state data contracts.\"",
        "<Playground title=\"Controlled + on_change\" code=code>",
        "<Playground title=\"Disabled + Fine Step\" code=states_code>",
        "<Slider",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra slider docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn slider_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Controlled + on_change\"",
        "id=\"docs-slider-volume\".to_string()",
        "label=\"Volume\".to_string()",
        "min=0.0",
        "max=100.0",
        "step=1.0",
        "on_change=on_change",
        "value: ",
        " · last on_change: ",
        "title=\"Disabled + Fine Step\"",
        "id=\"docs-slider-disabled\".to_string()",
        "label=\"Disabled\".to_string()",
        "disabled=true",
        "id=\"docs-slider-fine\".to_string()",
        "label=\"Fine Step\".to_string()",
        "max=1.0",
        "step=0.05",
        "motion=fine_motion",
        "class_name=\"docs-slider--fine\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra slider playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}
