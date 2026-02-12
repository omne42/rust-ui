use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toggle_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/toggle/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Toggle internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn toggle_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/toggle/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Toggle;"),
        "toggle module should export `Toggle`."
    );
    assert!(
        module_source.contains("pub struct ToggleStateInput"),
        "toggle module should expose `ToggleStateInput` contract."
    );
    assert!(
        crate_source.contains("pub use toggle::{Toggle, ToggleMotion, ToggleSize, ToggleVariant};"),
        "crate root should re-export toggle types."
    );
}

#[test]
fn toggle_logic_exposes_state_helpers() {
    let source = load_source("src/toggle/logic.rs");

    for needle in [
        "pub fn state_attr_for_selected(selected: bool)",
        "pub fn interaction_attr(",
        "pub fn variant_attr(variant: ToggleVariant)",
        "pub fn size_attr(size: ToggleSize)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_state(input: ToggleStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ToggleState)",
    ] {
        assert!(
            source.contains(needle),
            "Toggle logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn toggle_view_contains_press_and_state_contracts() {
    let source = load_source("src/toggle/view.rs");

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_optional_text(aria_label)",
        "let has_custom_class_name = class_name.is_some();",
        "let has_custom_motion = motion != ToggleMotion::default();",
        "let has_custom_aria_label = aria_label.is_some();",
        "let has_on_pressed_change = on_pressed_change.is_some();",
        "let state = Memo::new(move |_| {",
        "logic::resolve_state(ToggleStateInput {",
        "let class = logic::compose_class_name(class_name, state.get_untracked());",
        "data-slot=\"toggle\"",
        "data-slot=\"toggle-label\"",
        "data-state=move || state.get().state_attr",
        "data-interaction=move || state.get().interaction_attr",
        "data-variant=move || state.get().variant_attr",
        "data-size=move || state.get().size_attr",
        "data-variant-source=move || state.get().variant_source_attr",
        "data-size-source=move || state.get().size_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-handler-source=move || state.get().handler_source_attr",
        "data-custom-aria-label=move || state.get().has_custom_aria_label.then_some(\"true\")",
        "data-custom-aria=move || state.get().has_custom_aria_label.then_some(\"true\")",
        "data-custom-handler=move || state.get().has_on_pressed_change.then_some(\"true\")",
        "aria-pressed=move || if state.get().is_selected { \"true\" } else { \"false\" }",
        "motion::attach_motion",
    ] {
        assert!(
            source.contains(needle),
            "Toggle view should include `{needle}` for stable behavior contracts."
        );
    }
}

#[test]
fn toggle_css_contains_expected_state_and_source_selectors() {
    let css = load_source("src/toggle/styles.rs");

    for needle in [
        ".ui-toggle {",
        ".ui-toggle[data-state=\"selected\"]",
        ".ui-toggle[data-state=\"unselected\"]",
        ".ui-toggle[data-interaction=\"pressed\"]",
        ".ui-toggle[data-variant=\"outline\"]",
        ".ui-toggle[data-size=\"sm\"]",
        ".ui-toggle[data-variant-source=\"custom\"]",
        ".ui-toggle[data-size-source=\"custom\"]",
        ".ui-toggle[data-class-source=\"custom\"]",
        ".ui-toggle[data-motion-source=\"custom\"]",
        ".ui-toggle[data-custom-motion=\"true\"]",
        ".ui-toggle[data-aria-source=\"custom\"]",
        ".ui-toggle[data-custom-aria-label=\"true\"]",
        ".ui-toggle[data-handler-source=\"custom\"]",
    ] {
        assert!(
            css.contains(needle),
            "Toggle CSS should include `{needle}` selector."
        );
    }
}

#[test]
fn toggle_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::toggle::styles::CSS);"),
        "ui-components css aggregator should include toggle styles."
    );
}

#[test]
fn toggle_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn toggle() -> AnyView",
        "title=\"Toggle\"",
        "slug=\"toggle\"",
        "State + Source Markers",
        "data-handler-source",
        "<Toggle",
    ] {
        assert!(
            source.contains(needle),
            "toggle docs page should contain `{needle}`."
        );
    }
}

#[test]
fn toggle_docs_outline_disabled_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Outline + Ghost + Disabled\"",
        "variant=ToggleVariant::Outline",
        "size=ToggleSize::Sm",
        "\"Italic\"",
        "variant=ToggleVariant::Ghost",
        "disabled=true",
        "\"Disabled\"",
    ] {
        assert!(
            source.contains(needle),
            "Toggle docs outline/disabled playground should contain `{needle}`.",
        );
    }
}

#[test]
fn toggle_docs_state_source_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "variant=ToggleVariant::Outline",
        "size=ToggleSize::Sm",
        "motion=ToggleMotion {",
        "tap_scale: 0.92",
        "..ToggleMotion::default()",
        "class_name=\"docs-toggle-state\".to_string()",
        "aria_label=\"Toggle formatting\".to_string()",
        "on_pressed_change=on_pressed_change",
        "Inspect `data-state`, `data-interaction`, `data-variant-source`, `data-motion-source`, `data-aria-source`, and `data-handler-source` contracts.",
    ] {
        assert!(
            source.contains(needle),
            "Toggle docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn toggle_docs_page_covers_primary_playgrounds() {
    toggle_docs_page_contains_state_source_playground();
}

#[test]
fn toggle_docs_playgrounds_lock_state_matrix_contract_values() {
    toggle_docs_outline_disabled_playground_locks_contract_values();
    toggle_docs_state_source_playground_locks_contract_values();
}
