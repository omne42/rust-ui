use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toggle_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/toggle_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ToggleButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn toggle_button_uses_headless_hooks() {
    let source = load_source("src/toggle_button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "ToggleButton should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn toggle_button_uses_logic_state_model() {
    let view_source = load_source("src/toggle_button/view.rs");
    let logic_source = load_source("src/toggle_button/logic.rs");

    for needle in [
        "pub struct ToggleButtonState",
        "pub fn resolve_state(",
        "pub is_selected: bool",
        "pub is_enabled: bool",
        "pub is_pressed: bool",
        "pub is_focus_visible: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "ToggleButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "logic::resolve_state(",
        "selected.get()",
        "state.get().data_state()",
    ] {
        assert!(
            view_source.contains(needle),
            "ToggleButton view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }
}

#[test]
fn toggle_button_attaches_motion_driver() {
    let source = load_source("src/toggle_button/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "ToggleButton should attach a motion driver to match the repo's motion spec."
    );
}

#[test]
fn toggle_button_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/toggle_button/view.rs");

    for attr in [
        "data-slot=\"toggle-button\"",
        "data-state=move || state.get().data_state()",
        "data-selected=move || state.get().is_selected.then_some(\"true\")",
        "data-unselected=move || state.get().is_unselected.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-hovered=move || state.get().is_hovered.then_some(\"true\")",
        "data-pressed=move || state.get().is_pressed.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-focus-visible=move || state.get().is_focus_visible.then_some(\"true\")",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            source.contains(attr),
            "ToggleButton should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn toggle_button_styles_define_scale_css_var() {
    let source = load_source("src/toggle_button/styles.rs");

    assert!(
        source.contains("--ui-toggle-button-scale"),
        "ToggleButton styles should define `--ui-toggle-button-scale` so motion can update scale without re-rendering."
    );
}

#[test]
fn toggle_button_styles_include_motion_marker_contracts() {
    let source = load_source("src/toggle_button/styles.rs");

    for selector in [
        ".ui-toggle-button[data-motion-source=\"custom\"]",
        ".ui-toggle-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ToggleButton styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn toggle_button_motion_uses_spring_animator() {
    let source = load_source("src/toggle_button/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "ToggleButton motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn toggle_button_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/toggle_button/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ToggleButtonMotion) -> ToggleButtonMotion",
        "fn sanitize_spring(",
        "hover_scale:",
        "tap_scale:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "ToggleButton motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn toggle_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn toggle_button() -> AnyView",
        "title=\"ToggleButton\"",
        "slug=\"toggle-button\"",
        "description=\"Pressable toggle state with HeroUI-level spring motion and Spectrum-style root state attrs.\"",
        "<Playground title=\"Controlled + on_change\" code=code>",
        "<Playground title=\"Variant + Disabled matrix\" code=states_code>",
        "<ToggleButton",
        "on_change=on_toggle_change",
        "variant=ToggleButtonVariant::Accent",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for toggle-button coverage.",
        );
    }
}

#[test]
fn toggle_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "let on_change = Callback::new(move |next: bool| {",
        "<ToggleButton selected=selected set_selected=set_selected on_change=Some(on_change)>",
        "\"selected: \"",
        "\"last on_change: \" {move || last_change.get()}",
        "selected=notifications",
        "set_selected=set_notifications",
        "size=ToggleButtonSize::Lg",
        "\"notifications: \"",
        "selected=disabled_selected",
        "selected=disabled_unselected",
        "\"Disabled on\"",
        "\"Disabled off\"",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "toggle-button docs playgrounds should contain `{needle}`.",
        );
    }
}
