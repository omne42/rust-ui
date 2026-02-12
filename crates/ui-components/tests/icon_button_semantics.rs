use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icon_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/icon_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "IconButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn icon_button_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/icon_button/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::IconButton;"),
        "icon_button module should export `IconButton`."
    );
    assert!(
        crate_source.contains("pub use icon_button::IconButton;"),
        "crate root should re-export `IconButton`."
    );
}

#[test]
fn icon_button_logic_exposes_state_helpers() {
    let source = load_source("src/icon_button/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(aria_label: String)",
        "pub fn resolve_state(input: IconButtonStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: IconButtonState)",
        "DEFAULT_ARIA_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "IconButton logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn icon_button_view_uses_logic_state_contracts() {
    let source = load_source("src/icon_button/view.rs");

    for needle in [
        "pub fn IconButton(",
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(IconButtonStateInput {",
        "logic::compose_class_name(class_name, state)",
        "<Button",
        "on_press: Option<OnPress>",
        "data-slot=\"icon-button\"",
        "data-state=state.state_attr",
        "data-size-mode=state.size_mode_attr",
        "data-handler-source=state.handler_source_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-fallback-label=state.has_fallback_aria_label.then_some(\"true\")",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "IconButton view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn icon_button_styles_include_state_and_source_markers() {
    let source = load_source("src/icon_button/styles.rs");

    for selector in [
        ".ui-icon-button {",
        ".ui-icon-button[data-state=\"disabled\"]",
        ".ui-icon-button[data-size-mode=\"icon\"]",
        ".ui-icon-button[data-size-mode=\"custom\"]",
        ".ui-icon-button[data-handler-source=\"custom\"]",
        ".ui-icon-button[data-label-source=\"custom\"]",
        ".ui-icon-button[data-class-source=\"custom\"]",
        ".ui-icon-button[data-motion-source=\"custom\"]",
        ".ui-icon-button[data-custom-class=\"true\"]",
        ".ui-icon-button--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "IconButton styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn icon_button_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::icon_button::styles::CSS);"),
        "ui-components css aggregator should include icon_button styles."
    );
}

#[test]
fn icon_button_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra_icon_button.rs");

    for needle in [
        "pub(super) fn icon_button() -> AnyView",
        "title=\"IconButton\"",
        "slug=\"icon-button\"",
        "State + Source Markers",
        "data-motion-source",
        "<IconButton",
    ] {
        assert!(
            source.contains(needle),
            "actions_extra_icon_button docs page should contain `{needle}`."
        );
    }
}

#[test]
fn icon_button_docs_page_locks_custom_motion_marker_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra_icon_button.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "motion=ButtonMotion { hover_scale: 1.0, tap_scale: 1.0, ..ButtonMotion::default() }",
        "data-motion-source",
        "class_name=\"docs-icon-button-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icon button docs page should include `{needle}` for motion/source marker regression stability."
        );
    }
}
