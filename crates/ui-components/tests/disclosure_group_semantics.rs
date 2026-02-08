use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn disclosure_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/disclosure_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "DisclosureGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn disclosure_group_uses_logic_state_model() {
    let logic_source = load_source("src/disclosure_group/logic.rs");
    let view_source = load_source("src/disclosure_group/view.rs");

    for needle in [
        "pub enum DisclosureGroupSelectionMode",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_expanded_indices(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "data_state_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "DisclosureGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state",
        "logic::normalize_expanded_indices(",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(DisclosureGroupStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "DisclosureGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn disclosure_group_composes_accordion_with_motion() {
    let mod_source = load_source("src/disclosure_group/mod.rs");
    let view_source = load_source("src/disclosure_group/view.rs");

    assert!(
        mod_source.contains("AccordionMotion as DisclosureGroupMotion"),
        "DisclosureGroup should expose `DisclosureGroupMotion` as the motion contract alias."
    );

    for needle in [
        "motion: DisclosureGroupMotion",
        "<Accordion",
        "open_indices=expanded_signal",
        "on_open_change=request_expanded_change",
        "selection_mode=accordion_selection_mode",
        "motion=motion",
    ] {
        assert!(
            view_source.contains(needle),
            "DisclosureGroup should compose Accordion with `{needle}` to preserve spring-driven behavior."
        );
    }
}

#[test]
fn disclosure_group_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/disclosure_group/view.rs");

    for attr in [
        "data-slot=\"disclosure-group\"",
        "data-slot=\"disclosure-group-list\"",
        "data-selection-mode=move || state.get().selection_mode_attr",
        "data-state=move || state.get().data_state_attr",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-expanded-count=move || state.get().expanded_count.to_string()",
        "data-all-collapsed=move || (!state.get().has_expanded_items).then_some(\"true\")",
        "data-multiple-expanded=move || state.get().has_multiple_expanded.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-disabled-items=move || state.get().has_disabled_items.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "DisclosureGroup should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn disclosure_group_styles_include_state_markers() {
    let source = load_source("src/disclosure_group/styles.rs");

    for selector in [
        ".ui-disclosure-group--selection-single",
        ".ui-disclosure-group[data-selection-mode=\"multiple\"]",
        ".ui-disclosure-group--empty",
        ".ui-disclosure-group[data-empty=\"true\"]",
        ".ui-disclosure-group--disabled",
        ".ui-disclosure-group[data-disabled=\"true\"]",
        ".ui-disclosure-group--multiple-expanded .ui-disclosure-group__accordion",
        ".ui-disclosure-group[data-all-collapsed=\"true\"] .ui-disclosure-group__accordion",
        ".ui-disclosure-group--custom-class",
        ".ui-disclosure-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "DisclosureGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
