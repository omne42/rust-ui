use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icons_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/icons/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Icons internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn icons_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/icons/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Icons;"),
        "icons module should export `Icons`."
    );
    assert!(
        module_source.contains("pub enum IconsSet"),
        "icons module should expose `IconsSet` enum contract."
    );
    assert!(
        module_source.contains("pub enum IconsScale"),
        "icons module should expose `IconsScale` enum contract."
    );
    assert!(
        crate_source
            .contains("pub use icons::{Icons, IconsGlyph, IconsScale, IconsSet, IconsTone};"),
        "crate root should re-export `Icons` contracts."
    );
}

#[test]
fn icons_logic_exposes_state_helpers() {
    let source = load_source("src/icons/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn parse_set_from_name(name: &str)",
        "pub fn resolve_set(name: &str, set: IconsSet)",
        "pub fn normalize_name(name: String, set: IconsSet)",
        "pub fn resolve_state(input: IconsStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: IconsState)",
    ] {
        assert!(
            source.contains(needle),
            "Icons logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn icons_view_uses_logic_state_contracts() {
    let source = load_source("src/icons/view.rs");

    for needle in [
        "pub fn Icons(",
        "logic::resolve_set(&name, set)",
        "logic::normalize_name(name, resolved_set)",
        "logic::normalize_optional_text(aria_label)",
        "logic::resolve_state(IconsStateInput {",
        "logic::compose_class_name(class_name_for_wrapper, state)",
        "<IconsUi",
        "<IconsWorkflow",
        "data-slot=\"icons\"",
        "data-state=state.state_attr",
        "data-set=state.set_attr",
        "data-scale=state.scale_attr",
        "data-set-source=state.set_source_attr",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-glyph-source=state.glyph_source_attr",
        "data-tone-source=state.tone_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Icons view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn icons_styles_include_state_and_source_markers() {
    let source = load_source("src/icons/styles.rs");

    for selector in [
        ".ui-icons {",
        ".ui-icons[data-state=\"disabled\"]",
        ".ui-icons[data-state=\"decorative\"]",
        ".ui-icons[data-set=\"ui\"]",
        ".ui-icons[data-set=\"workflow\"]",
        ".ui-icons[data-scale=\"medium\"]",
        ".ui-icons[data-scale=\"large\"]",
        ".ui-icons[data-set-source=\"name\"]",
        ".ui-icons[data-set-source=\"prop\"]",
        ".ui-icons[data-set-source=\"default\"]",
        ".ui-icons[data-aria-source=\"custom\"]",
        ".ui-icons[data-class-source=\"custom\"]",
        ".ui-icons[data-glyph-source=\"custom\"]",
        ".ui-icons[data-tone-source=\"custom\"]",
        ".ui-icons--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Icons styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn icons_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::icons::styles::CSS);"),
        "ui-components css aggregator should include icons styles."
    );
}

#[test]
fn icons_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons.rs");

    for needle in [
        "pub(super) fn icons() -> AnyView",
        "title=\"Icons\"",
        "slug=\"icons\"",
        "State + Source Markers",
        "data-tone-source",
        "<Icons",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_icons docs page should contain `{needle}`."
        );
    }
}

#[test]
fn icons_docs_default_and_custom_playgrounds_lock_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons.rs");

    for needle in [
        "<Playground title=\"Medium + Large Set Selection\" code=default_code>",
        "name=\"check\".to_string()",
        "set=IconsSet::Ui",
        "scale=IconsScale::Medium",
        "tone=IconsTone::Accent",
        "name=\"workflow:warning\".to_string()",
        "scale=IconsScale::Large",
        "tone=IconsTone::Danger",
        "<Playground title=\"Custom Workflow Glyph Extension\" code=custom_code>",
        "name=\"workflow:deploy\".to_string()",
        "set=IconsSet::Workflow",
        "tone=IconsTone::Default",
        "IconsGlyph::new(\"workflow:deploy\", \"🚀\")",
        ".with_aria_label(\"Workflow Deploy\")",
        "class_name=\"docs-icons-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons docs default/custom playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn icons_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "name=\"check\".to_string()",
        "set=IconsSet::Workflow",
        "scale=IconsScale::Large",
        "tone=IconsTone::Muted",
        "IconsGlyph::new(\"workflow:check\", \"✓\")",
        ".with_aria_label(\"Workflow Check\")",
        "decorative=false",
        "aria_label=\"Explicit icon label\".to_string()",
        "class_name=\"docs-icons-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons docs marker playground should contain `{needle}`.",
        );
    }
}

#[test]
fn icons_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons.rs");

    for needle in [
        "pub(super) fn icons() -> AnyView",
        "title=\"Icons\"",
        "slug=\"icons\"",
        "description=\"Spectrum-compatible `icons` package wrapper that maps medium/large scale and ui/workflow set selection onto IconsUi/IconsWorkflow with stable source-state contracts.\"",
        "<Playground title=\"Medium + Large Set Selection\" code=default_code>",
        "<Playground title=\"Custom Workflow Glyph Extension\" code=custom_code>",
        "title=\"State + Source Markers\"",
        "<Icons",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_icons docs should include `{needle}` for icons primary playground coverage.",
        );
    }
}

#[test]
fn icons_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons.rs");

    for needle in [
        "title=\"Medium + Large Set Selection\"",
        "name=\"check\".to_string()",
        "set=IconsSet::Ui",
        "scale=IconsScale::Medium",
        "tone=IconsTone::Accent",
        "name=\"workflow:warning\".to_string()",
        "scale=IconsScale::Large",
        "tone=IconsTone::Danger",
        "title=\"Custom Workflow Glyph Extension\"",
        "name=\"workflow:deploy\".to_string()",
        "set=IconsSet::Workflow",
        "IconsGlyph::new(\"workflow:deploy\", \"🚀\")",
        ".with_aria_label(\"Workflow Deploy\")",
        "class_name=\"docs-icons-custom\".to_string()",
        "title=\"State + Source Markers\"",
        "name=\"check\".to_string()",
        "set=IconsSet::Workflow",
        "tone=IconsTone::Muted",
        "aria_label=\"Explicit icon label\".to_string()",
        "class_name=\"docs-icons-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons docs playgrounds should contain `{needle}`.",
        );
    }
}
