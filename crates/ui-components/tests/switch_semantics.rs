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
    let headless_source = load_source("../ui-headless/src/switch.rs");

    for needle in ["use_switch", "SwitchOptions"] {
        assert!(
            source.contains(needle),
            "Switch should use headless `{needle}` hooks."
        );
    }

    for needle in ["use_focus_ring", "use_hover"] {
        assert!(
            !source.contains(needle),
            "Switch view should not wire `{needle}` directly after semantic contract sink."
        );
    }

    for needle in [
        "use_hover(HoverOptions",
        "use_focus_ring(FocusRingOptions",
        "resolve_switch_state(SwitchStateInput",
    ] {
        assert!(
            headless_source.contains(needle),
            "Switch headless contract should include `{needle}`."
        );
    }
}

#[test]
fn switch_uses_logic_state_model() {
    let view_source = load_source("src/switch/view.rs");
    let logic_source = load_source("src/switch/logic.rs");
    let headless_source = load_source("../ui-headless/src/switch.rs");
    let primitive_source = load_source("../ui-state-primitives/src/switch.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");

    for needle in [
        "pub const TRACK_WIDTH_PX",
        "pub const TRACK_PADDING_PX",
        "pub const THUMB_WIDTH_PX",
        "pub fn checked_thumb_x_px(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Switch logic should stay focused on component-local motion geometry; missing `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_state(",
        "SwitchStateInput",
        "resolve_switch_state",
    ] {
        assert!(
            !logic_source.contains(needle),
            "Switch logic should not re-implement state derivation in component layer; found `{needle}`."
        );
    }

    for needle in [
        "pub struct SwitchState {",
        "pub resolved: Memo<PrimitiveSwitchState>",
        "pub state: SwitchState",
        "resolve_switch_state(SwitchStateInput {",
    ] {
        assert!(
            headless_source.contains(needle),
            "Switch headless should expose typed attrs/handlers/state contract; missing `{needle}`."
        );
    }

    for needle in [
        "pub struct SwitchStateInput",
        "pub struct SwitchState",
        "pub fn resolve_state(input: SwitchStateInput) -> SwitchState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Switch primitive should define `{needle}`."
        );
    }

    {
        let needle = "pub mod switch;";
        assert!(
            primitive_lib_source.contains(needle),
            "ui-state-primitives should export `{needle}`."
        );
    }

    for needle in [
        "aria.state.resolved.get().data_state()",
        "data-focus-visible=move || aria.state.resolved.get().is_focus_visible.then_some(\"true\")",
        "on:pointerenter=move |_| aria.handlers.hover.on_pointer_enter.run(())",
        "on:focus=move |_| aria.handlers.focus_ring.on_focus.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "Switch view should mount headless semantics contract; missing `{needle}`."
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
fn switch_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/switch/view.rs");

    for attr in [
        "data-slot=\"switch\"",
        "data-state=move || aria.state.resolved.get().data_state()",
        "data-checked=move || aria.state.resolved.get().is_checked.then_some(\"true\")",
        "data-unchecked=move || aria.state.resolved.get().is_unchecked.then_some(\"true\")",
        "data-disabled=move || aria.state.resolved.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || aria.state.resolved.get().is_enabled.then_some(\"true\")",
        "data-pressed=move || aria.state.resolved.get().is_pressed.then_some(\"true\")",
        "data-hovered=move || aria.state.resolved.get().is_hovered.then_some(\"true\")",
        "data-focused=move || aria.state.resolved.get().is_focused.then_some(\"true\")",
        "data-focus-visible=move || aria.state.resolved.get().is_focus_visible.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Switch should set `{attr}` to support baseline-style styling and state inspection."
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
fn switch_view_uses_token_backed_pressed_width_default() {
    let source = load_source("src/switch/view.rs");

    assert!(
        source.contains("#[prop(optional, default = motion::default_pressed_width_px())]"),
        "Switch view should source default pressed width from theme-backed motion helper.",
    );
}

#[test]
fn switch_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/switch/motion.rs");

    for needle in [
        "use ui_theme::default_switch_motion_tokens;",
        "pub fn default_pressed_width_px() -> f64",
        "ui_motion::spring::sanitize_config",
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

#[test]
fn switch_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn switch() -> AnyView",
        "title=\"Switch\"",
        "slug=\"switch\"",
        "description=\"Switch toggle with baseline-level spring thumb motion and baseline-style root state attrs.\"",
        "<Playground title=\"Controlled + on_change\" code_signal=code>",
        "<Playground title=\"State matrix\" code_signal=states_code>",
        "<Switch",
    ] {
        assert!(
            source.contains(needle),
            "forms switch docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn switch_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Controlled + on_change\"",
        "checked=checked",
        "set_checked=set_checked",
        "on_change=on_system_change",
        "title=\"State matrix\"",
        "<Switch checked=system_enabled set_checked=set_system_enabled>",
        "<Switch checked=disabled_checked set_checked=set_disabled_checked disabled=true>",
        "<Switch checked=disabled_unchecked set_checked=set_disabled_unchecked disabled=true>",
        "\"last on_change: \"",
    ] {
        assert!(
            source.contains(needle),
            "forms switch docs playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}
