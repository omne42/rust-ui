use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn empty_state_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/empty_state/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "EmptyState internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn empty_state_uses_logic_state_model() {
    let logic_source = load_source("src/empty_state/logic.rs");
    let view_source = load_source("src/empty_state/view.rs");

    for needle in [
        "pub enum EmptyStateTone",
        "pub enum EmptyStateAlign",
        "pub fn normalize_optional_text(",
        "pub fn normalize_title(",
        "pub fn normalize_description(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "title_source_attr",
        "description_source_attr",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "EmptyState logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_title(title)",
        "logic::normalize_description(description)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(EmptyStateStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "EmptyState view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn empty_state_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/empty_state/view.rs");

    for attr in [
        "data-slot=\"empty-state\"",
        "data-slot=\"empty-state-icon\"",
        "data-slot=\"empty-state-title\"",
        "data-slot=\"empty-state-description\"",
        "data-slot=\"empty-state-actions\"",
        "data-tone=move || state.get().tone_attr",
        "data-align=move || state.get().align_attr",
        "data-state=move || state.get().data_state_attr",
        "data-compact=move || state.get().is_compact.then_some(\"true\")",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-icon=move || state.get().has_icon.then_some(\"true\")",
        "data-actions=move || state.get().has_actions.then_some(\"true\")",
        "data-title-source=move || state.get().title_source_attr",
        "data-description-source=move || state.get().description_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "EmptyState should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn empty_state_styles_include_tone_align_and_markers() {
    let source = load_source("src/empty_state/styles.rs");

    for selector in [
        ".ui-empty-state--tone-default",
        ".ui-empty-state[data-tone=\"default\"]",
        ".ui-empty-state--tone-muted",
        ".ui-empty-state[data-tone=\"muted\"]",
        ".ui-empty-state--tone-accent",
        ".ui-empty-state[data-tone=\"accent\"]",
        ".ui-empty-state--align-start",
        ".ui-empty-state[data-align=\"start\"]",
        ".ui-empty-state--align-center",
        ".ui-empty-state[data-align=\"center\"]",
        ".ui-empty-state--compact",
        ".ui-empty-state[data-compact=\"true\"]",
        ".ui-empty-state--bordered",
        ".ui-empty-state[data-bordered=\"true\"]",
        ".ui-empty-state--custom-class",
        ".ui-empty-state[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "EmptyState styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
