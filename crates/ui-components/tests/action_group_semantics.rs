use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(suffix) = rel_path.strip_prefix("src/button/") {
        let migrated = manifest_dir
            .join("../../components/button/src")
            .join(suffix);
        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

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
        "selection_source_attr",
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
fn action_group_controlled_uncontrolled_contract_is_complete() {
    let view_source = load_source("src/button/action/view.rs");

    for needle in [
        "#[prop(optional, into)] selected_ids: Option<Signal<BTreeSet<String>>>",
        "#[prop(optional)] default_selected_ids: Option<BTreeSet<String>>",
        "#[prop(optional)] on_selected_ids_change: Option<Callback<BTreeSet<String>>>",
        "let is_selection_controlled = selected_ids.is_some();",
        "let on_selected_change = on_selected_ids_change.or(on_selected_change);",
        "use_controllable_state(selected_ids, Some(default_selected_ids), on_selected_change)",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionGroup controlled/uncontrolled contract should include `{needle}`."
        );
    }
}

#[test]
fn action_composite_apis_use_explicit_children_or_typed_item_specs() {
    let view_source = load_source("src/button/action/view.rs");
    let mod_source = load_source("src/button/action/mod.rs");
    let action_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let action_extra_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub fn ActionButtonGroup(",
        "children: Children,",
        "{children()}",
        "pub fn ActionGroup(",
        "items: Vec<ActionGroupItem>,",
    ] {
        assert!(
            view_source.contains(needle),
            "Action composite API should expose explicit composition/type contracts via `{needle}`."
        );
    }

    for needle in [
        "pub struct ActionGroupItem",
        "pub id: String,",
        "pub label: String,",
        "pub disabled: bool,",
        "pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self",
    ] {
        assert!(
            mod_source.contains(needle),
            "ActionGroup typed item spec should stay explicit via `{needle}`."
        );
    }

    for needle in [
        "<ActionButtonGroup",
        "<ActionButton on_press=on_press>\"One\"</ActionButton>",
        "<ActionButton on_press=on_press>\"Two\"</ActionButton>",
        "<ActionButton on_press=on_press>\"Three\"</ActionButton>",
    ] {
        assert!(
            action_docs_source.contains(needle),
            "ActionButtonGroup docs should prefer explicit parent-child composition via `{needle}`."
        );
    }

    for needle in [
        "items=vec![",
        "ActionGroupItem::new(\"align-left\", \"Align Left\")",
        "ActionGroupItem::new(\"align-center\", \"Align Center\")",
        "ActionGroupItem::new(\"align-right\", \"Align Right\")",
        "ActionGroupItem::new(\"align-justify\", \"Justify\").disabled(true)",
    ] {
        assert!(
            action_extra_docs_source.contains(needle),
            "ActionGroup docs should prefer typed item specs via `{needle}`."
        );
    }

    for forbidden in [
        "labels: Vec<String>",
        "titles: Vec<String>",
        "panels: Vec<",
        "labels=vec![",
        "titles=vec![",
        "panels=vec![",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !action_docs_source.contains(forbidden)
                && !action_extra_docs_source.contains(forbidden),
            "Action composite API should not regress to implicit parallel-array contract `{forbidden}`."
        );
    }
}

#[test]
fn action_group_selection_primitive_is_sourced_from_state_primitives() {
    let logic_source = load_source("src/button/action/logic.rs");

    for needle in [
        "use ui_state_primitives::action_group as action_group_state;",
        "fn as_state_primitive(self) -> action_group_state::ActionGroupSelectionMode",
        "action_group_state::collect_item_ids(",
        "action_group_state::sanitize_selected_ids(",
        "action_group_state::toggle_selected_id(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionGroup selection primitive should delegate to ui-state-primitives via `{needle}`."
        );
    }

    for forbidden in [
        "if !item_ids.contains(id) {",
        "ActionGroupSelectionMode::None => BTreeSet::new(),",
        "if selected_ids.len() <= 1 {",
        "selected_ids.insert(first);",
        "if !next.insert(id.to_string()) {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ActionGroup selection state machine should not be reimplemented in ui-components: `{forbidden}`."
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
        "data-selection-source=move || state.get().selection_source_attr",
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
fn action_button_family_styles_use_explicit_state_markers_not_fragile_structure() {
    let source = load_source("src/button/action/styles.rs");

    for selector in [
        ".ui-action-button-group--justified > .ui-button",
        ".ui-action-button-group--quiet > .ui-button",
        ".ui-action-group[data-tone=\"default\"]",
        ".ui-action-group[data-disabled=\"true\"]",
        ".ui-action-group[data-has-selection=\"true\"]",
        ".ui-action-group[data-selection-mode=\"single\"] .ui-action-group__item",
        ".ui-action-group__item[data-selected=\"true\"]",
        ".ui-action-group__item[data-disabled=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Action button-family styles should anchor on explicit class/data markers via `{selector}`."
        );
    }

    for fragile in [":nth-child(", ":nth-of-type(", ":only-child", ":has("] {
        assert!(
            !source.contains(fragile),
            "Action button-family styles should avoid fragile DOM-guessing selector `{fragile}`."
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
