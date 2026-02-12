use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn status_light_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/status_light/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "StatusLight internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn status_light_uses_logic_state_model() {
    let view_source = load_source("src/status_light/view.rs");
    let logic_source = load_source("src/status_light/logic.rs");

    for needle in [
        "pub struct StatusLightStateInput",
        "pub struct StatusLightState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "state_class",
        "role_source_class",
        "class_source_attr",
        "ui-status-light--custom-class",
    ] {
        assert!(
            logic_source.contains(needle),
            "StatusLight logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(StatusLightStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "StatusLight view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn status_light_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/status_light/view.rs");

    for attr in [
        "data-slot=\"status-light\"",
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-live=state.is_live.then_some(\"true\")",
        "data-static=(!state.is_live).then_some(\"true\")",
        "data-role=state.role_attr",
        "data-role-source=state.role_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-slot=\"status-light-indicator\"",
        "data-slot=\"status-light-label\"",
    ] {
        assert!(
            source.contains(attr),
            "StatusLight should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn status_light_styles_include_variant_state_and_source_markers() {
    let source = load_source("src/status_light/styles.rs");

    for selector in [
        ".ui-status-light--variant-default",
        ".ui-status-light[data-variant=\"accent\"]",
        ".ui-status-light--variant-danger",
        ".ui-status-light--live",
        ".ui-status-light[data-state=\"live\"]",
        ".ui-status-light--static",
        ".ui-status-light[data-static=\"true\"]",
        ".ui-status-light[data-state=\"static\"] .ui-status-light__dot",
        ".ui-status-light--role-custom",
        ".ui-status-light[data-role-source=\"custom\"]",
        ".ui-status-light--custom-class",
        ".ui-status-light[data-custom-class=\"true\"]",
        ".ui-status-light[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "StatusLight styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn status_light_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn status_light() -> AnyView",
        "title=\"StatusLight\"",
        "slug=\"status-light\"",
        "Playground title=\"Variants\"",
        "Playground title=\"Live Region Role\"",
        "Playground title=\"Custom Class + Static\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for StatusLight.",
        );
    }
}

#[test]
fn status_light_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Variants\"",
        "<StatusLight variant=StatusLightVariant::Default>\"Idle\"</StatusLight>",
        "<StatusLight variant=StatusLightVariant::Accent>\"Deploying\"</StatusLight>",
        "<StatusLight variant=StatusLightVariant::Danger>\"Failed\"</StatusLight>",
        "title=\"Live Region Role\"",
        "<StatusLight role=StatusLightRole::Status>\"Background sync complete\"</StatusLight>",
        "title=\"Custom Class + Static\"",
        "class_name=\"docs-status-light-custom\".to_string()",
        "role=StatusLightRole::Status",
    ] {
        assert!(
            source.contains(needle),
            "status-light docs playgrounds should contain `{needle}`.",
        );
    }
}
