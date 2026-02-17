use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/action/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ActionGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn action_group_uses_logic_state_model() {
    let logic_source = load_source("src/button/action/logic.rs");
    let view_source = load_source("src/button/action/view.rs");

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
        "use_controllable_state(selected_ids, Some(default_selected_ids), on_selected_change)",
        "action_logic::action_group_logic::normalize_items(items)",
        "action_logic::action_group_logic::collect_item_ids(&items)",
        "action_logic::action_group_logic::sanitize_selected_ids(",
        "action_logic::action_group_logic::toggle_selected_id(",
        "action_logic::action_group_logic::resolve_state(ActionGroupStateInput {",
        "action_logic::action_group_logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn action_group_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/button/action/view.rs");

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
            "ActionGroup should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn action_group_styles_include_tone_mode_selection_and_markers() {
    let source = load_source("src/button/action/styles.rs");

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

#[test]
fn action_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn action_group() -> AnyView",
        "title=\"ActionGroup\"",
        "slug=\"action-group\"",
        "Playground title=\"Single Selection + Action Callback\"",
        "Playground title=\"Multiple + Strong Tone\"",
    ] {
        assert!(
            source.contains(needle),
            "actions-extra docs page should contain `{needle}` for ActionGroup.",
        );
    }
}

#[test]
fn action_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Single Selection + Action Callback\"",
        "id_base=\"docs-action-group-single\".to_string()",
        "selected_ids=selected_ids",
        "on_selected_change=on_selected_change",
        "on_action=on_action",
        "selected: ",
        "last action:",
        "title=\"Multiple + Strong Tone\"",
        "id_base=\"docs-action-group-multiple\".to_string()",
        "selection_mode=ActionGroupSelectionMode::Multiple",
        "default_selected_ids=BTreeSet::from([",
        "tone=ActionGroupTone::Strong",
        "class_name=\"docs-action-group-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "action-group docs playgrounds should contain `{needle}`.",
        );
    }
}
