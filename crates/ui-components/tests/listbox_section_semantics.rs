use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn listbox_section_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/listbox_section/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ListBoxSection internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn listbox_section_uses_logic_state_model() {
    let logic_source = load_source("src/listbox_section/logic.rs");
    let view_source = load_source("src/listbox_section/view.rs");

    for needle in [
        "pub enum ListBoxSectionHeadingTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "data_state_attr",
        "aria_source_attr",
        "class_source_attr",
        "title_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ListBoxSection logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(ListBoxSectionStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ListBoxSection view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn listbox_section_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/listbox_section/view.rs");

    for attr in [
        "data-slot=\"listbox-section\"",
        "data-slot=\"listbox-section-header\"",
        "data-slot=\"listbox-section-items\"",
        "data-slot=\"listbox-section-divider\"",
        "data-tone=move || state.get().heading_tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-item-count=move || state.get().item_count.to_string()",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-has-title=move || state.get().has_title.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-sticky-heading=move || state.get().is_sticky_heading.then_some(\"true\")",
        "data-divided=move || state.get().has_divider.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-title-source=move || state.get().title_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "ListBoxSection should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn listbox_section_styles_include_tone_and_state_markers() {
    let source = load_source("src/listbox_section/styles.rs");

    for selector in [
        ".ui-listbox-section--tone-default",
        ".ui-listbox-section[data-tone=\"quiet\"]",
        ".ui-listbox-section--empty",
        ".ui-listbox-section[data-empty=\"true\"]",
        ".ui-listbox-section--disabled",
        ".ui-listbox-section[data-disabled=\"true\"]",
        ".ui-listbox-section--sticky-heading",
        ".ui-listbox-section[data-sticky-heading=\"true\"]",
        ".ui-listbox-section--divided",
        ".ui-listbox-section[data-divided=\"true\"]",
        ".ui-listbox-section--custom-class",
        ".ui-listbox-section[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ListBoxSection styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn listbox_section_supports_group_accessibility_and_items_layout() {
    let source = load_source("src/listbox_section/view.rs");

    for needle in [
        "<section",
        "role=\"group\"",
        "aria-label=aria_label",
        "aria-disabled=disabled.then_some(\"true\")",
        "<header",
        "{children()}",
    ] {
        assert!(
            source.contains(needle),
            "ListBoxSection should include `{needle}` for accessibility and content composition."
        );
    }
}
