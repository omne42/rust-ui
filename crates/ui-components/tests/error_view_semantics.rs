use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn error_view_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/error_view/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ErrorView internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn error_view_uses_logic_state_model() {
    let logic_source = load_source("src/error_view/logic.rs");
    let view_source = load_source("src/error_view/view.rs");

    for needle in [
        "pub enum ErrorViewTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_message(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "message_source_attr",
        "aria_source_attr",
        "class_source_attr",
        "motion_source_attr",
        "content_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ErrorView logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_message(message)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(ErrorViewStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "motion::attach_motion(root_ref, visible, motion)",
    ] {
        assert!(
            view_source.contains(needle),
            "ErrorView view should derive state via logic/motion helpers; missing `{needle}`."
        );
    }
}

#[test]
fn error_view_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/error_view/view.rs");

    for attr in [
        "data-slot=\"error-view\"",
        "data-slot=\"error-view-icon\"",
        "data-slot=\"error-view-content\"",
        "data-slot=\"error-view-text\"",
        "data-slot=\"error-view-actions\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().state_attr",
        "data-invalid=move || state.get().is_visible.then_some(\"true\")",
        "data-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-compact=move || state.get().is_compact.then_some(\"true\")",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-icon=move || state.get().has_icon.then_some(\"true\")",
        "data-actions=move || state.get().has_actions.then_some(\"true\")",
        "data-content=move || state.get().content_attr",
        "data-message-source=move || state.get().message_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "ErrorView should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn error_view_styles_include_visibility_tone_and_markers() {
    let source = load_source("src/error_view/styles.rs");

    for selector in [
        ".ui-error-view--tone-negative",
        ".ui-error-view[data-tone=\"negative\"]",
        ".ui-error-view--tone-neutral",
        ".ui-error-view[data-tone=\"neutral\"]",
        ".ui-error-view--visible",
        ".ui-error-view[data-state=\"visible\"]",
        ".ui-error-view--hidden",
        ".ui-error-view[data-state=\"hidden\"]",
        ".ui-error-view--compact",
        ".ui-error-view[data-compact=\"true\"]",
        ".ui-error-view--bordered",
        ".ui-error-view[data-bordered=\"true\"]",
        ".ui-error-view--with-actions",
        ".ui-error-view[data-actions=\"true\"]",
        ".ui-error-view--custom-class",
        ".ui-error-view[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ErrorView styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn error_view_motion_contract_is_present() {
    let source = load_source("src/error_view/motion.rs");

    for needle in [
        "pub struct ErrorViewMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "--ui-error-view-translate-y",
        "--ui-error-view-opacity",
        "--ui-error-view-scale",
    ] {
        assert!(
            source.contains(needle),
            "ErrorView motion should expose `{needle}` for spring-driven visibility transitions."
        );
    }
}

#[test]
fn error_view_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn error_view() -> AnyView",
        "title=\"ErrorView\"",
        "slug=\"error-view\"",
        "description=\"Spectrum/HeroUI-style validation error container with centralized visibility/content/source state contracts and spring-driven motion markers.\"",
        "<Playground title=\"Invalid Visibility\" code_signal=basic_code>",
        "<Playground title=\"Custom Content + Motion + Actions\" code_signal=state_code>",
        "<ErrorView",
    ] {
        assert!(
            source.contains(needle),
            "display_extra docs page should include `{needle}` for error_view primary playground coverage.",
        );
    }
}

#[test]
fn error_view_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Invalid Visibility\"",
        "is_invalid=true",
        "message=\"Please enter a valid email address\".to_string()",
        "is_invalid=false",
        "message=\"This error stays hidden until the field becomes invalid.\".to_string()",
        "title=\"Custom Content + Motion + Actions\"",
        "tone=ErrorViewTone::Neutral",
        "compact=true",
        "bordered=true",
        "class_name=\"docs-error-view-custom\".to_string()",
        "motion=ErrorViewMotion {",
        "hidden_translate_px: 12.0",
        "hidden_opacity: 0.0",
        "hidden_scale: 0.95",
        "variant=ui_components::ButtonVariant::Secondary",
        "\"Retry\"",
        "\"Validation failed. Check highlighted fields and retry.\"",
    ] {
        assert!(
            source.contains(needle),
            "error_view docs playgrounds should contain `{needle}`.",
        );
    }
}
