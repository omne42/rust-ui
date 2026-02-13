use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_bar_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/action_bar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ActionBar internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn action_bar_uses_logic_state_model() {
    let logic_source = load_source("src/action_bar/logic.rs");
    let view_source = load_source("src/action_bar/view.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_clear_label(",
        "pub fn normalize_selection_text(",
        "pub fn resolve_selection_kind(",
        "pub fn resolve_selection_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "selection_source_attr",
        "clear_label_source_attr",
        "motion_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionBar logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_clear_label(clear_label)",
        "logic::normalize_selection_text(selection_text)",
        "logic::resolve_state(ActionBarStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "resolve_selection_text(state.get().selected_count, selection_text.clone())",
        "motion::attach_motion(root_ref, visible, motion)",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn action_bar_emits_toolbar_semantics_and_state_attributes() {
    let source = load_source("src/action_bar/view.rs");

    for needle in [
        "data-slot=\"action-bar\"",
        "data-state=move || state.get().phase_attr",
        "data-position=move || state.get().position_attr",
        "data-selection=move || state.get().selection_attr",
        "data-selected-count=move || state.get().selected_count.to_string()",
        "data-visible=move || state.get().is_visible.then_some(\"true\")",
        "data-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-has-clear=move || state.get().has_clear_action.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-selection-source=move || state.get().selection_source_attr",
        "data-clear-label-source=move || state.get().clear_label_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "role=\"toolbar\"",
        "aria-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-slot=\"action-bar-clear\"",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar should emit `{needle}` for Spectrum-style contract and tooling."
        );
    }
}

#[test]
fn action_bar_styles_include_position_state_and_source_contracts() {
    let source = load_source("src/action_bar/styles.rs");

    for selector in [
        ".ui-action-bar--position-bottom",
        ".ui-action-bar[data-position=\"bottom\"]",
        ".ui-action-bar--position-top",
        ".ui-action-bar[data-position=\"top\"]",
        ".ui-action-bar--state-hidden",
        ".ui-action-bar[data-state=\"hidden\"]",
        ".ui-action-bar[data-hidden=\"true\"]",
        ".ui-action-bar--selection-custom",
        ".ui-action-bar[data-selection-source=\"custom\"]",
        ".ui-action-bar--clear-label-custom",
        ".ui-action-bar[data-clear-label-source=\"custom\"]",
        ".ui-action-bar--motion-custom",
        ".ui-action-bar[data-motion-source=\"custom\"]",
        ".ui-action-bar--custom-class",
        ".ui-action-bar[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ActionBar styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn action_bar_motion_contract_defaults_and_disabled_path_are_locked() {
    let source = load_source("src/action_bar/motion.rs");

    for needle in [
        "pub struct ActionBarMotion",
        "enabled: true",
        "hidden_translate_px: 28.0",
        "hidden_opacity: 0.0",
        "pub fn disabled() -> Self",
        "enabled: false",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar motion contract should include `{needle}` for HeroUI-level defaults/disabled stability."
        );
    }
}

#[test]
fn action_bar_motion_sanitization_and_reduced_motion_paths_are_locked() {
    let source = load_source("src/action_bar/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ActionBarMotion) -> ActionBarMotion",
        ".clamp(-400.0, 400.0)",
        ".clamp(0.0, 1.0)",
        "!motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn disabled_constructor_turns_motion_off()",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar motion implementation should include `{needle}` to avoid regressions."
        );
    }
}

#[test]
fn action_bar_motion_uses_spring_driver() {
    let source = load_source("src/action_bar/motion.rs");

    for needle in [
        "pub fn sanitize_motion(",
        "ui_motion::spring::SpringAnimator::new",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-action-bar-translate-y",
        "--ui-action-bar-opacity",
        "pub fn attach_motion(",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar motion should include `{needle}` for spring-driven visibility animation."
        );
    }
}

#[test]
fn action_bar_docs_page_includes_custom_motion_contract_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "let mut custom_motion = ActionBarMotion::default();",
        "custom_motion.hidden_translate_px = 44.0;",
        "custom_motion.hidden_opacity = 0.22;",
        "motion=custom_motion",
        "motion=ActionBarMotion::disabled()",
    ] {
        assert!(
            source.contains(needle),
            "action_bar docs page should include `{needle}` for custom motion demos."
        );
    }
}

#[test]
fn action_bar_docs_default_and_state_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "<Playground title=\"Selection + clear action\" code_signal=code>",
        "selected_count=selected_count_signal",
        "on_clear_selection=clear_selection",
        "aria_label=\"Bulk actions\".to_string()",
        "class_name=\"docs-action-bar\".to_string()",
        "<ActionButton>\"Delete\"</ActionButton>",
        "<ActionButton is_quiet=true>\"Archive\"</ActionButton>",
        "<Playground title=\"Top placement + custom text + reduced motion\" code_signal=state_code>",
        "position=ActionBarPosition::Top",
        "force_visible=true",
        "selection_text=\"Rows selected\".to_string()",
        "clear_label=\"Clear all\".to_string()",
        "motion=ActionBarMotion::disabled()",
        "Top placement + custom labels + motion disabled.",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar docs default/state playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn action_bar_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "<Playground title=\"Custom Motion Contract\" code_signal=motion_code>",
        "let mut custom_motion = ActionBarMotion::default();",
        "custom_motion.spring.stiffness = 280.0;",
        "custom_motion.spring.damping = 24.0;",
        "custom_motion.spring.mass = 1.0;",
        "custom_motion.spring.precision = 0.002;",
        "custom_motion.hidden_translate_px = 44.0;",
        "custom_motion.hidden_opacity = 0.22;",
        "motion=custom_motion",
        "motion=ActionBarMotion::disabled()",
        "<ActionButton is_quiet=true>\"Sync\"</ActionButton>",
        "<ActionButton is_quiet=true>\"Share\"</ActionButton>",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar docs custom-motion playground should contain `{needle}`.",
        );
    }
}

#[test]
fn action_bar_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn action_bar() -> AnyView",
        "title=\"ActionBar\"",
        "slug=\"action-bar\"",
        "Playground title=\"Selection + clear action\"",
        "Playground title=\"Top placement + custom text + reduced motion\"",
        "Playground title=\"Custom Motion Contract\"",
    ] {
        assert!(
            source.contains(needle),
            "actions-extra docs page should contain `{needle}` for ActionBar.",
        );
    }
}

#[test]
fn action_bar_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Selection + clear action\"",
        "selected_count=selected_count_signal",
        "on_clear_selection=clear_selection",
        "aria_label=\"Bulk actions\".to_string()",
        "class_name=\"docs-action-bar\".to_string()",
        "title=\"Top placement + custom text + reduced motion\"",
        "position=ActionBarPosition::Top",
        "force_visible=true",
        "selection_text=\"Rows selected\".to_string()",
        "clear_label=\"Clear all\".to_string()",
        "motion=ActionBarMotion::disabled()",
        "title=\"Custom Motion Contract\"",
        "custom_motion.hidden_translate_px = 44.0;",
        "custom_motion.hidden_opacity = 0.22;",
    ] {
        assert!(
            source.contains(needle),
            "action-bar docs playgrounds should contain `{needle}`.",
        );
    }
}
