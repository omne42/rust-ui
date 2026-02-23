use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn pressable_feedback_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/pressable-feedback/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "PressableFeedback internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn pressable_feedback_uses_logic_state_model() {
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");

    for needle in [
        "pub enum PressableFeedbackTone",
        "pub enum PressableFeedbackEffect",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "highlight_attr",
        "ripple_attr",
        "motion_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "PressableFeedback logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(PressableFeedbackStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "use_press(PressOptions {",
        "motion::attach_motion(root_ref, pressed, motion, has_highlight)",
        "trigger_ripple(ripple_ref, ripple_motion)",
    ] {
        assert!(
            view_source.contains(needle),
            "PressableFeedback view should derive state via logic/motion helpers; missing `{needle}`."
        );
    }
}

#[test]
fn pressable_feedback_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/pressable-feedback/src/view.rs");

    for attr in [
        "data-slot=\"pressable-feedback\"",
        "data-slot=\"pressable-feedback-highlight\"",
        "data-slot=\"pressable-feedback-content\"",
        "data-tone=move || state.get().tone_attr",
        "data-effect=move || state.get().effect_attr",
        "data-state=move || state.get().state_attr",
        "data-boundary=move || state.get().boundary_attr",
        "data-bounded=move || state.get().is_bounded.then_some(\"true\")",
        "data-unbounded=move || state.get().is_unbounded.then_some(\"true\")",
        "data-pressed=move || state.get().is_pressed.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-highlight=move || state.get().highlight_attr",
        "data-ripple=move || state.get().ripple_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "PressableFeedback should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn pressable_feedback_styles_include_effect_boundary_and_markers() {
    let source = load_source("../../components/pressable-feedback/src/styles.rs");

    for selector in [
        ".ui-pressable-feedback--tone-default",
        ".ui-pressable-feedback[data-tone=\"default\"]",
        ".ui-pressable-feedback--state-pressed",
        ".ui-pressable-feedback[data-state=\"pressed\"]",
        ".ui-pressable-feedback--effect-highlight-ripple",
        ".ui-pressable-feedback[data-effect=\"highlight-ripple\"]",
        ".ui-pressable-feedback--boundary-bounded",
        ".ui-pressable-feedback[data-boundary=\"bounded\"]",
        ".ui-pressable-feedback--highlight-enabled",
        ".ui-pressable-feedback[data-highlight=\"enabled\"]",
        ".ui-pressable-feedback--ripple-enabled",
        ".ui-pressable-feedback[data-ripple=\"enabled\"]",
        ".ui-pressable-feedback--custom-class",
        ".ui-pressable-feedback[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "PressableFeedback styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn pressable_feedback_motion_contract_is_present() {
    let source = load_source("../../components/pressable-feedback/src/motion.rs");

    for needle in [
        "pub struct PressableFeedbackMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "--ui-pressable-feedback-scale",
        "--ui-pressable-feedback-highlight-opacity",
    ] {
        assert!(
            source.contains(needle),
            "PressableFeedback motion should expose `{needle}` for spring-driven press feedback transitions."
        );
    }
}

#[test]
fn pressable_feedback_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn pressable_feedback() -> AnyView",
        "title=\"PressableFeedback\"",
        "slug=\"pressable-feedback\"",
        "description=\"baseline-style press feedback container with centralized effect/tone/boundary/source contracts, spring-driven scale/highlight motion, and optional ripple composition.\"",
        "<Playground title=\"Scale + Highlight\" code_signal=basic_code>",
        "<Playground title=\"Highlight + Ripple + Custom Motion\" code_signal=custom_code>",
        "<PressableFeedback",
    ] {
        assert!(
            source.contains(needle),
            "display_extra docs should include `{needle}` for pressable-feedback primary playground coverage.",
        );
    }
}

#[test]
fn pressable_feedback_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Scale + Highlight\"",
        "effect=PressableFeedbackEffect::Highlight",
        "tone=PressableFeedbackTone::Accent",
        "on_press=on_press_count",
        "\"Press me\"",
        "format!(\"Press count: {}\", press_count.get())",
        "title=\"Highlight + Ripple + Custom Motion\"",
        "effect=PressableFeedbackEffect::HighlightRipple",
        "tone=PressableFeedbackTone::Neutral",
        "bounded=false",
        "motion=PressableFeedbackMotion {",
        "pressed_scale: 0.94",
        "highlight_opacity: 0.2",
        "duration_ms: 720",
        "class_name=\"docs-pressable-feedback-custom\".to_string()",
        "\"Custom feedback\"",
        "is_disabled=true",
        "\"Disabled\"",
    ] {
        assert!(
            source.contains(needle),
            "display_extra docs playgrounds should contain `{needle}` for pressable-feedback contracts.",
        );
    }
}
