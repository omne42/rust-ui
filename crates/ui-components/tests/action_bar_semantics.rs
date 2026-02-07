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
