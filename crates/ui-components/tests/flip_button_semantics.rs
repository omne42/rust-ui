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
        "pub struct FlipButtonState",
        "pub fn normalize_class_name(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub direction_attr: &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "FlipButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let class_name = logic::normalize_class_name(class_name);",
        "let state = Memo::new(move |_| {",
        "logic::resolve_state(",
        "let class = logic::compose_class_name(",
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
        "data-state=move || if state.get().is_active { \"active\" } else { \"inactive\" }",
        "data-active=move || state.get().is_active.then_some(\"true\")",
        "data-inactive=move || state.get().is_inactive.then_some(\"true\")",
        "data-hovered=move || state.get().is_hovered.then_some(\"true\")",
        "data-focus-within=move || state.get().is_focus_within.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "FlipButton should expose `{needle}` for Spectrum-style state inspection."
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
