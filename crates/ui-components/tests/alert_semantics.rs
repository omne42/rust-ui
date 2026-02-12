use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn alert_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/alert/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Alert internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn alert_uses_logic_state_model() {
    let view_source = load_source("src/alert/view.rs");
    let logic_source = load_source("src/alert/logic.rs");

    for needle in [
        "pub struct AlertStateInput",
        "pub struct AlertState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Alert logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(title)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(AlertStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn alert_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/alert/view.rs");

    for attr in [
        "data-slot=\"alert\"",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-title=state.title_attr",
        "data-description=state.description_attr",
        "data-actions=state.actions_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "role=state.role_attr",
        "aria-live=state.live_attr",
        "data-slot=\"alert-title\"",
        "data-slot=\"alert-description\"",
        "data-slot=\"alert-actions\"",
    ] {
        assert!(
            source.contains(attr),
            "Alert should expose `{attr}` for Spectrum-style state inspection and styling."
        );
    }
}

#[test]
fn alert_styles_include_state_marker_contracts() {
    let source = load_source("src/alert/styles.rs");

    for selector in [
        ".ui-alert--detailed",
        ".ui-alert[data-state=\"compact\"]",
        ".ui-alert--with-actions",
        ".ui-alert[data-actions=\"absent\"] .ui-alert__actions",
        ".ui-alert--variant-accent",
        ".ui-alert[data-variant=\"danger\"]",
        ".ui-alert--custom-class",
        ".ui-alert[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Alert styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn alert_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn alert() -> AnyView",
        "title=\"Alert\"",
        "slug=\"alert\"",
        "Playground title=\"Variants + Live Region\"",
        "Playground title=\"Custom Class + Compact\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Alert.",
        );
    }
}

#[test]
fn alert_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Variants + Live Region\"",
        "variant=AlertVariant::Default",
        "variant=AlertVariant::Accent",
        "variant=AlertVariant::Danger",
        "title=\"Custom Class + Compact\"",
        "description=\"Custom class without title\".to_string()",
        "class_name=\"docs-alert-custom\".to_string()",
        "<Alert variant=AlertVariant::Default title=\"Heads up\".to_string()>",
    ] {
        assert!(
            source.contains(needle),
            "alert docs playgrounds should contain `{needle}`.",
        );
    }
}
