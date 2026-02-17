use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn checkbox_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/checkbox_field/checkbox/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Checkbox internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn checkbox_uses_headless_hooks() {
    let source = load_source("src/checkbox_field/checkbox/view.rs");

    for needle in ["use_checkbox", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Checkbox should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn checkbox_uses_logic_state_model() {
    let view_source = load_source("src/checkbox_field/checkbox/view.rs");
    let logic_source = load_source("src/checkbox_field/checkbox/logic.rs");

    for needle in [
        "pub struct CheckboxState",
        "pub fn resolve_state(",
        "pub is_checked: bool",
        "pub is_enabled: bool",
        "pub is_pressed: bool",
        "pub is_focus_visible: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Checkbox logic should include `{needle}` for centralized state derivation."
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
            "Checkbox view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }
}

#[test]
fn checkbox_attaches_motion_drivers() {
    let source = load_source("src/checkbox_field/checkbox/view.rs");

    for needle in ["attach_root_motion", "attach_indicator_motion"] {
        assert!(
            source.contains(needle),
            "Checkbox should attach motion driver `{needle}`."
        );
    }
}

#[test]
fn checkbox_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/checkbox_field/checkbox/view.rs");

    for attr in [
        "data-slot=\"checkbox\"",
        "data-state=move || state.get().data_state()",
        "data-checked=move || state.get().is_checked.then_some(\"true\")",
        "data-unchecked=move || state.get().is_unchecked.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-hovered=move || state.get().is_hovered.then_some(\"true\")",
        "data-pressed=move || state.get().is_pressed.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-focus-visible=move || state.get().is_focus_visible.then_some(\"true\")",
        "data-motion-source=if motion == CheckboxMotion::default()",
        "data-custom-motion=(motion != CheckboxMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Checkbox should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn checkbox_motion_uses_spring_animator() {
    let source = load_source("src/checkbox_field/checkbox/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Checkbox motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn checkbox_styles_respect_prefers_reduced_motion() {
    let source = load_source("src/checkbox_field/checkbox/styles.rs");

    assert!(
        source.contains("prefers-reduced-motion: reduce"),
        "Checkbox styles should respect prefers-reduced-motion to avoid forced transitions."
    );
    assert!(
        source.contains("transition: none;"),
        "Checkbox styles should disable transitions under prefers-reduced-motion."
    );
}

#[test]
fn checkbox_styles_include_motion_marker_contracts() {
    let source = load_source("src/checkbox_field/checkbox/styles.rs");

    for selector in [
        ".ui-checkbox[data-motion-source=\"custom\"]",
        ".ui-checkbox[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Checkbox styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn checkbox_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/checkbox_field/checkbox/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: CheckboxMotion) -> CheckboxMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "fn sanitize_indicator_spring(",
        "hover_scale:",
        "tap_scale:",
        "indicator_spring:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values_and_keeps_valid_springs()",
    ] {
        assert!(
            source.contains(needle),
            "Checkbox motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn checkbox_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn checkbox() -> AnyView",
        "title=\"Checkbox\"",
        "slug=\"checkbox\"",
        "title=\"Controlled + on_change\"",
        "title=\"Variant + Disabled matrix\"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "<Playground title=\"Controlled + on_change\" code_signal=code>",
        "<Checkbox",
        "checked=checked",
        "set_checked=set_checked",
        "on_change=on_accept_change",
        "\"Accept terms\"",
        "<Playground title=\"Variant + Disabled matrix\" code_signal=states_code>",
        "variant=CheckboxVariant::Accent",
        "size=CheckboxSize::Lg",
        "disabled=true",
        "\"Disabled on\"",
        "\"Disabled off\"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox docs playground should contain `{needle}`.",
        );
    }
}
