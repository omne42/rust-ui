use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn auto_height_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/auto_height/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "AutoHeight internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn auto_height_uses_logic_state_model() {
    let logic_source = load_source("src/auto_height/logic.rs");
    let view_source = load_source("src/auto_height/view.rs");

    for needle in [
        "pub struct AutoHeightStateInput",
        "pub struct AutoHeightState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "AutoHeight logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(AutoHeightStateInput {",
        "logic::compose_class_name(class_name, state)",
        "motion != AutoHeightMotion::default()",
    ] {
        assert!(
            view_source.contains(needle),
            "AutoHeight view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn auto_height_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/auto_height/view.rs");

    for attr in [
        "data-slot=\"auto-height\"",
        "data-slot=\"auto-height-content\"",
        "data-state=if state.animate_height { \"animated\" } else { \"static\" }",
        "data-animated=state.animate_height.then_some(\"true\")",
        "data-static=state.is_static.then_some(\"true\")",
        "data-overflow-hidden=state.overflow_hidden.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-motion-source=if state.has_custom_motion { \"custom\" } else { \"default\" }",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "AutoHeight should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn auto_height_attaches_motion_driver_and_uses_motion_contract() {
    let view_source = load_source("src/auto_height/view.rs");

    assert!(
        view_source.contains("motion::attach_motion"),
        "AutoHeight should attach its motion driver rather than ignoring the motion contract."
    );
}

#[test]
fn auto_height_styles_define_state_marker_contracts() {
    let source = load_source("src/auto_height/styles.rs");

    for selector in [
        "--ui-auto-height-height",
        ".ui-auto-height--animated",
        ".ui-auto-height[data-state=\"animated\"]",
        ".ui-auto-height--static",
        ".ui-auto-height[data-state=\"static\"]",
        ".ui-auto-height[data-overflow-hidden=\"true\"]",
        ".ui-auto-height[data-motion-source=\"custom\"]",
        ".ui-auto-height--custom-motion",
        ".ui-auto-height[data-custom-motion=\"true\"]",
        ".ui-auto-height--custom-class",
        ".ui-auto-height[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "AutoHeight styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn auto_height_motion_uses_resize_observer_and_spring() {
    let source = load_source("src/auto_height/motion.rs");

    assert!(
        source.contains("ResizeObserver"),
        "AutoHeight motion should observe content size changes via ResizeObserver."
    );

    assert!(
        source.contains("SpringAnimator"),
        "AutoHeight motion should animate height changes via a spring."
    );
}

#[test]
fn auto_height_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/auto_height/motion.rs");

    for needle in [
        "pub struct AutoHeightMotion",
        "fn default_motion_matches_auto_height_contract()",
        "fn supports_custom_motion_contract_values()",
    ] {
        assert!(
            source.contains(needle),
            "AutoHeight motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
fn auto_height_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/auto_height/motion.rs");
    let view_source = load_source("src/auto_height/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: AutoHeightMotion) -> AutoHeightMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "AutoHeight motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::auto_height::motion::sanitize_motion(motion);"),
        "AutoHeight view should sanitize motion before deriving state and attaching motion driver.",
    );
}

#[test]
fn auto_height_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn auto_height() -> AnyView",
        "title=\"AutoHeight\"",
        "slug=\"auto-height\"",
        "Playground title=\"Animated Height\"",
        "Playground title=\"Static Motion + Custom Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for AutoHeight.",
        );
    }
}

#[test]
fn auto_height_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Animated Height\"",
        "set_animated_open.update(|v| *v = !*v)",
        "<AutoHeight class_name=\"docs-auto-height\".to_string()>",
        "\"AutoHeight content\"",
        "title=\"Static Motion + Custom Class\"",
        "animate_height: false",
        "..AutoHeightMotion::default()",
        "class_name=\"docs-auto-height docs-auto-height--static-demo\".to_string()",
        "\"Static mode content\"",
    ] {
        assert!(
            source.contains(needle),
            "auto-height docs playgrounds should contain `{needle}`.",
        );
    }
}
