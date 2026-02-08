use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/action_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ActionGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn action_group_uses_logic_state_model() {
    let logic_source = load_source("src/action_group/logic.rs");
    let view_source = load_source("src/action_group/view.rs");

    for needle in [
        "pub enum ActionGroupTone",
        "pub enum ActionGroupSelectionMode",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_items(",
        "pub fn collect_item_ids(",
        "pub fn sanitize_selected_ids(",
        "pub fn toggle_selected_id(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "logic::normalize_items(items)",
        "logic::collect_item_ids(&items)",
        "logic::sanitize_selected_ids(",
        "logic::toggle_selected_id(",
        "logic::resolve_state(ActionGroupStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn action_group_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/action_group/view.rs");

    for attr in [
        "data-slot=\"action-group\"",
        "data-tone=move || state.get().tone_attr",
        "data-selection-mode=move || state.get().selection_mode_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-item-count=move || state.get().item_count.to_string()",
        "data-selected-count=move || state.get().selected_count.to_string()",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"action-group-list\"",
        "data-slot=\"action-group-node\"",
        "data-slot=\"action-group-item\"",
        "role=\"toolbar\"",
    ] {
        assert!(
            source.contains(attr),
            "ActionGroup should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn action_group_styles_include_tone_mode_selection_and_markers() {
    let source = load_source("src/action_group/styles.rs");

    for selector in [
        ".ui-action-group--tone-default",
        ".ui-action-group[data-tone=\"default\"]",
        ".ui-action-group--tone-quiet",
        ".ui-action-group--tone-strong",
        ".ui-action-group--disabled",
        ".ui-action-group[data-disabled=\"true\"]",
        ".ui-action-group--has-selection",
        ".ui-action-group[data-has-selection=\"true\"]",
        ".ui-action-group--custom-class",
        ".ui-action-group[data-custom-class=\"true\"]",
        ".ui-action-group[data-selection-mode=\"single\"] .ui-action-group__item",
        ".ui-action-group[data-selection-mode=\"multiple\"] .ui-action-group__item",
        ".ui-action-group__item--selected",
        ".ui-action-group__item[data-selected=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ActionGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
