use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn listbox_item_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/listbox_item/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ListBoxItem internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn listbox_item_uses_logic_state_model() {
    let logic_source = load_source("src/listbox_item/logic.rs");
    let view_source = load_source("src/listbox_item/view.rs");

    for needle in [
        "pub enum ListBoxItemSelectionIndicator",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_selection_indicator(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "data_state_attr",
        "selection_indicator_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ListBoxItem logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(ListBoxItemStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ListBoxItem view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn listbox_item_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/listbox_item/view.rs");

    for attr in [
        "data-slot=\"listbox-item\"",
        "data-slot=\"listbox-item-indicator\"",
        "data-slot=\"listbox-item-label\"",
        "data-slot=\"listbox-item-divider\"",
        "data-index=index_text",
        "data-state=move || state.get().data_state_attr",
        "data-selected=move || state.get().is_selected.then_some(\"true\")",
        "data-unselected=move || (!state.get().is_selected).then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-divider=move || state.get().has_divider.then_some(\"true\")",
        "data-show-selection-indicator=move ||",
        "data-selection-indicator=move || state.get().selection_indicator_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "ListBoxItem should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn listbox_item_styles_include_state_markers() {
    let source = load_source("src/listbox_item/styles.rs");

    for selector in [
        ".ui-listbox-item--selected",
        ".ui-listbox-item[data-selected=\"true\"]",
        ".ui-listbox-item--focused",
        ".ui-listbox-item[data-focused=\"true\"]",
        ".ui-listbox-item--disabled",
        ".ui-listbox-item[data-disabled=\"true\"]",
        ".ui-listbox-item--selection-indicator",
        ".ui-listbox-item[data-show-selection-indicator=\"true\"]",
        ".ui-listbox-item--divider",
        ".ui-listbox-item[data-has-divider=\"true\"]",
        ".ui-listbox-item--custom-class",
        ".ui-listbox-item[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ListBoxItem styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn listbox_item_supports_option_accessibility_contract() {
    let source = load_source("src/listbox_item/view.rs");

    for needle in [
        "role=\"option\"",
        "aria-selected=selected.then_some(\"true\")",
        "aria-disabled=disabled.then_some(\"true\")",
        "on:pointermove=move |_|",
        "on:click=move |_|",
    ] {
        assert!(
            source.contains(needle),
            "ListBoxItem should include `{needle}` for option accessibility and interactions."
        );
    }
}
