use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn flip_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button_flip/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FlipButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn flip_button_uses_logic_state_model() {
    let view_source = load_source("src/button_flip/view.rs");
    let logic_source = load_source("src/button_flip/logic.rs");

    for needle in [
        "pub struct FlipButtonStateInput",
        "pub struct FlipButtonState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub direction_class: &'static str",
        "pub state_attr: &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "FlipButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let class_name = logic::normalize_optional_text(class_name);",
        "logic::resolve_state(FlipButtonStateInput {",
        "class=move || logic::compose_class_name(class_name_source.clone(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "FlipButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn flip_button_uses_headless_hover_and_focus_within_hooks() {
    let source = load_source("src/button_flip/view.rs");

    for needle in ["use_hover", "use_focus_within"] {
        assert!(
            source.contains(needle),
            "FlipButton should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn flip_button_emits_spectrum_style_data_attributes() {
    let source = load_source("src/button_flip/view.rs");

    for needle in [
        "data-slot=\"flip-button\"",
        "data-from=move || state.get().direction_attr",
        "data-state=move || state.get().state_attr",
        "data-hover=move || state.get().hover_attr",
        "data-focus-within-state=move || state.get().focus_within_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-active=move || state.get().is_active.then_some(\"true\")",
        "data-inactive=move || state.get().is_inactive.then_some(\"true\")",
        "data-hovered=move || state.get().is_hovered.then_some(\"true\")",
        "data-focus-within=move || state.get().is_focus_within.then_some(\"true\")",
        "data-motion-source=if motion == FlipButtonMotion::default()",
        "data-custom-motion=(motion != FlipButtonMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "FlipButton should expose `{needle}` for Spectrum-style state inspection."
        );
    }
}

#[test]
fn flip_button_styles_include_state_marker_contracts() {
    let source = load_source("src/button_flip/styles.rs");

    for selector in [
        ".ui-flip-button--custom-class",
        ".ui-flip-button[data-custom-class=\"true\"]",
        ".ui-flip-button[data-motion-source=\"custom\"]",
        ".ui-flip-button[data-custom-motion=\"true\"]",
        ".ui-flip-button--state-active .ui-flip-button__front",
        ".ui-flip-button[data-state=\"active\"] .ui-flip-button__back",
        ".ui-flip-button--from-top .ui-flip-button__front",
        ".ui-flip-button[data-from=\"left\"] .ui-flip-button__back",
        ".ui-flip-button--from-right .ui-flip-button__back",
    ] {
        assert!(
            source.contains(selector),
            "FlipButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn flip_button_attaches_motion_driver() {
    let view_source = load_source("src/button_flip/view.rs");
    let motion_source = load_source("src/button_flip/motion.rs");

    assert!(
        view_source.contains("motion::attach_motion(node_ref, is_active, from, motion);"),
        "FlipButton view should attach the motion driver to synchronize spring progress."
    );

    for needle in [
        "set_property(\"--ui-flip-progress\"",
        "SpringAnimator::new",
        "spring.set_target(if active { 1.0 } else { 0.0 });",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipButton motion should include `{needle}` for spring-based flip transitions."
        );
    }
}

#[test]
fn flip_button_exposes_front_and_back_face_slots() {
    let source = load_source("src/button_flip/view.rs");

    for needle in [
        "data-slot=\"flip-button-front\"",
        "data-slot=\"flip-button-back\"",
    ] {
        assert!(
            source.contains(needle),
            "FlipButton should include `{needle}` to make face composition contract explicit."
        );
    }
}

#[test]
fn flip_button_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/button_flip/motion.rs");

    for needle in [
        "pub struct FlipButtonMotion",
        "fn default_motion_matches_flip_button_spring_contract()",
        "fn supports_custom_flip_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "FlipButton motion module should include `{needle}` for HeroUI-level motion contract coverage."
        );
    }
}

#[test]
fn flip_button_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/button_flip/motion.rs");
    let view_source = load_source("src/button_flip/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: FlipButtonMotion) -> FlipButtonMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipButton motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::button_flip::motion::sanitize_motion(motion);"),
        "FlipButton view should sanitize motion before attaching spring driver.",
    );
}

#[test]
fn flip_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "description=\"HeroUI-level spring flip surface with centralized direction/interaction/class-source state attrs.\"",
        "<Playground title=\"Top flip\" code=code>",
        "<Playground title=\"Direction matrix\" code=states_code>",
        "<Playground title=\"Custom Class\" code=custom_code>",
        "<FlipButton",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for flip_button primary playground coverage.",
        );
    }
}

#[test]
fn flip_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Top flip\"",
        "from=FlipDirection::Top",
        "variant=ButtonVariant::Secondary",
        "variant=ButtonVariant::Accent",
        "title=\"Direction matrix\"",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
        "title=\"Custom Class\"",
        "class_name=\"docs-flip-button-custom\".to_string()",
        "variant=ButtonVariant::Outline",
        "\"Inspect\"",
        "\"Inspecting\"",
    ] {
        assert!(
            source.contains(needle),
            "flip_button docs playgrounds should contain `{needle}`.",
        );
    }
}
