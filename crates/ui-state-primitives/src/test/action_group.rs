use super::*;

#[test]
fn collect_item_ids_deduplicates_ids() {
    let ids = collect_item_ids(["b", "a", "a", "c"]);
    assert_eq!(
        ids,
        BTreeSet::from(["a".to_string(), "b".to_string(), "c".to_string()])
    );
}

#[test]
fn sanitize_selected_ids_enforces_single_selection_and_known_ids() {
    let item_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);
    let selected = sanitize_selected_ids(
        BTreeSet::from(["b".to_string(), "a".to_string(), "ghost".to_string()]),
        &item_ids,
        ActionGroupSelectionMode::Single,
    );
    assert_eq!(selected, BTreeSet::from(["a".to_string()]));
}

#[test]
fn sanitize_selected_ids_clears_when_mode_is_none() {
    let item_ids = BTreeSet::from(["a".to_string()]);
    let selected = sanitize_selected_ids(
        BTreeSet::from(["a".to_string()]),
        &item_ids,
        ActionGroupSelectionMode::None,
    );
    assert!(selected.is_empty());
}

#[test]
fn toggle_selected_id_respects_modes() {
    let item_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);

    let single = toggle_selected_id(
        BTreeSet::new(),
        "a",
        &item_ids,
        ActionGroupSelectionMode::Single,
    );
    assert_eq!(single, BTreeSet::from(["a".to_string()]));
    let single = toggle_selected_id(single, "a", &item_ids, ActionGroupSelectionMode::Single);
    assert!(single.is_empty());

    let multiple = toggle_selected_id(
        BTreeSet::new(),
        "a",
        &item_ids,
        ActionGroupSelectionMode::Multiple,
    );
    assert_eq!(multiple, BTreeSet::from(["a".to_string()]));
    let multiple = toggle_selected_id(multiple, "a", &item_ids, ActionGroupSelectionMode::Multiple);
    assert!(multiple.is_empty());

    let none = toggle_selected_id(
        BTreeSet::from(["a".to_string()]),
        "b",
        &item_ids,
        ActionGroupSelectionMode::None,
    );
    assert!(none.is_empty());
}
