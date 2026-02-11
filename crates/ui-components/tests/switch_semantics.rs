use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn switch_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/switch/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Switch internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn switch_uses_headless_hooks() {
    let source = load_source("src/switch/view.rs");

    for needle in ["use_switch", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Switch should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn switch_uses_logic_state_model() {
    let view_source = load_source("src/switch/view.rs");
    let logic_source = load_source("src/switch/logic.rs");

    for needle in [
        "pub struct SwitchState",
        "pub fn resolve_state(",
        "pub is_checked: bool",
        "pub is_enabled: bool",
        "pub is_pressed: bool",
        "pub is_focus_visible: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Switch logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "logic::resolve_state(",
        "checked.get()",
        "state.get().data_state()",
    ] {
        assert!(
            view_source.contains(needle),
            "Switch view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }
}

#[test]
fn switch_attaches_thumb_motion_driver() {
    let source = load_source("src/switch/view.rs");

    assert!(
        source.contains("attach_thumb_motion"),
        "Switch should attach a motion driver for thumb micro-interactions."
    );
}

#[test]
fn switch_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/switch/view.rs");

    for attr in [
        "data-slot=\"switch\"",
        "data-state=move || state.get().data_state()",
        "data-checked=move || state.get().is_checked.then_some(\"true\")",
        "data-unchecked=move || state.get().is_unchecked.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-pressed=move || state.get().is_pressed.then_some(\"true\")",
        "data-hovered=move || state.get().is_hovered.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-focus-visible=move || state.get().is_focus_visible.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Switch should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn switch_styles_include_motion_marker_contracts() {
    let source = load_source("src/switch/styles.rs");

    for selector in [
        ".ui-switch[data-motion-source=\"custom\"]",
        ".ui-switch[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Switch styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn switch_motion_uses_spring_animator() {
    let source = load_source("src/switch/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Switch motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn switch_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/switch/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SwitchMotion) -> SwitchMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "fn sanitize_pressed_width_px(value: f64) -> f64",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let pressed_width_px = sanitize_pressed_width_px(pressed_width_px);",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_pressed_width_clamps_and_uses_fallback()",
    ] {
        assert!(
            source.contains(needle),
            "Switch motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}
