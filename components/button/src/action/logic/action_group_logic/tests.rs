use super::super::super::{ActionGroupItem, ActionGroupStateInput};
use super::*;

#[test]
fn tone_and_selection_mode_contracts_are_stable() {
    assert_eq!(
        ActionGroupTone::Default.class_name(),
        "ui-action-group--tone-default"
    );
    assert_eq!(
        ActionGroupTone::Quiet.class_name(),
        "ui-action-group--tone-quiet"
    );
    assert_eq!(
        ActionGroupTone::Strong.class_name(),
        "ui-action-group--tone-strong"
    );

    assert_eq!(
        ActionGroupSelectionMode::Single.class_name(),
        "ui-action-group--mode-single"
    );
    assert_eq!(
        ActionGroupSelectionMode::Multiple.class_name(),
        "ui-action-group--mode-multiple"
    );
    assert_eq!(
        ActionGroupSelectionMode::None.class_name(),
        "ui-action-group--mode-none"
    );
}

#[test]
fn items_and_selection_are_normalized() {
    let items = normalize_items(vec![
        ActionGroupItem::new(" ", " Edit "),
        ActionGroupItem::new("share", " "),
    ]);

    assert_eq!(items[0].id, "action-1");
    assert_eq!(items[0].label, "Edit");
    assert_eq!(items[1].label, "share");

    let item_ids = collect_item_ids(&items);

    let selected = sanitize_selected_ids(
        BTreeSet::from(["action-1".to_string(), "missing".to_string()]),
        &item_ids,
        ActionGroupSelectionMode::Single,
    );
    assert_eq!(selected, BTreeSet::from(["action-1".to_string()]));
}

#[test]
fn normalize_aria_label_uses_trimmed_label_or_i18n_fallback() {
    let (label, explicit) =
        normalize_aria_label(Some("  Align controls  ".to_string()), "Action group");
    assert_eq!(label, "Align controls");
    assert!(explicit);

    let (label, explicit) = normalize_aria_label(Some("   ".to_string()), "Action group");
    assert_eq!(label, "Action group");
    assert!(!explicit);

    let (label, explicit) = normalize_aria_label(None, "Action group");
    assert_eq!(label, "Action group");
    assert!(!explicit);
}

#[test]
fn toggle_selected_id_respects_selection_mode() {
    let item_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);

    let next = toggle_selected_id(
        BTreeSet::new(),
        "a",
        &item_ids,
        ActionGroupSelectionMode::Single,
    );
    assert_eq!(next, BTreeSet::from(["a".to_string()]));

    let next = toggle_selected_id(next, "a", &item_ids, ActionGroupSelectionMode::Single);
    assert!(next.is_empty());

    let next = toggle_selected_id(
        BTreeSet::new(),
        "a",
        &item_ids,
        ActionGroupSelectionMode::Multiple,
    );
    let next = toggle_selected_id(next, "b", &item_ids, ActionGroupSelectionMode::Multiple);
    assert_eq!(next, BTreeSet::from(["a".to_string(), "b".to_string()]));
}

#[test]
fn resolve_state_and_class_name_track_markers() {
    let state = resolve_state(ActionGroupStateInput {
        tone: ActionGroupTone::Strong,
        selection_mode: ActionGroupSelectionMode::Multiple,
        is_disabled: false,
        is_selection_controlled: true,
        item_count: 3,
        selected_count: 2,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.selection_mode_attr, "multiple");
    assert_eq!(state.data_state_attr, "selected");
    assert_eq!(state.selection_source_attr, "controlled");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");

    let class_name = compose_class_name(Some("docs-action-group".to_string()), state);
    for token in [
        "ui-action-group",
        "ui-action-group--tone-strong",
        "ui-action-group--mode-multiple",
        "ui-action-group--has-selection",
        "ui-action-group--custom-class",
        "docs-action-group",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}
