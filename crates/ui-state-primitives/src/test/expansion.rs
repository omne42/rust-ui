use super::*;
use std::collections::BTreeMap;

#[test]
fn single_mode_only_keeps_one_open() {
    let open = BTreeSet::from([1, 2]);
    let open = toggle_open_indices(ExpansionMode::Single, &open, 3);
    assert_eq!(open.iter().copied().collect::<Vec<_>>(), vec![3]);
}

#[test]
fn single_mode_toggles_closed_when_open() {
    let open = BTreeSet::from([1]);
    let open = toggle_open_indices(ExpansionMode::Single, &open, 1);
    assert!(open.is_empty());
}

#[test]
fn multiple_mode_toggles_membership() {
    let open = BTreeSet::new();
    let open = toggle_open_indices(ExpansionMode::Multiple, &open, 1);
    assert!(open.contains(&1));

    let open = toggle_open_indices(ExpansionMode::Multiple, &open, 1);
    assert!(!open.contains(&1));
}

#[test]
fn normalize_clamps_indices_to_item_count() {
    let open = BTreeSet::from([0, 2, 99]);
    let open = normalize_open_indices(ExpansionMode::Multiple, &open, 3);
    assert_eq!(open.iter().copied().collect::<Vec<_>>(), vec![0, 2]);
}

#[test]
fn normalize_single_keeps_only_first_open() {
    let open = BTreeSet::from([2, 1, 3]);
    let open = normalize_open_indices(ExpansionMode::Single, &open, 10);
    assert_eq!(open.iter().copied().collect::<Vec<_>>(), vec![1]);
}

#[test]
fn summarize_tracks_open_and_empty_flags() {
    let summary = summarize(ExpansionMode::Multiple, 4, 2);
    assert!(!summary.is_empty);
    assert!(summary.has_items);
    assert_eq!(summary.open_count, 2);
    assert!(summary.has_open_items);
    assert!(summary.has_multiple_open);
}

#[test]
fn summarize_single_mode_limits_open_count() {
    let summary = summarize(ExpansionMode::Single, 4, 3);
    assert_eq!(summary.open_count, 1);
    assert!(summary.has_open_items);
    assert!(!summary.has_multiple_open);
}

#[test]
fn summarize_handles_empty_collection() {
    let summary = summarize(ExpansionMode::Single, 0, 5);
    assert!(summary.is_empty);
    assert!(!summary.has_items);
    assert_eq!(summary.open_count, 0);
    assert!(!summary.has_open_items);
    assert!(!summary.has_multiple_open);
}

#[test]
fn normalize_open_keys_respects_item_key_order_and_mode() {
    let open = BTreeSet::from([30, 10]);
    let normalized = normalize_open_keys(ExpansionMode::Single, &open, &[10, 20, 30], false);
    assert_eq!(normalized, BTreeSet::from([10]));
}

#[test]
fn normalize_default_open_keys_falls_back_to_empty_input() {
    let normalized =
        normalize_default_open_keys::<usize>(ExpansionMode::Multiple, None, &[2, 4, 6], false);
    assert!(normalized.is_empty());
}

#[test]
fn toggle_open_key_toggles_by_key_and_handles_disallow_empty_selection() {
    let next = toggle_open_key(
        ExpansionMode::Single,
        &BTreeSet::from([6]),
        6,
        &[5, 6],
        true,
    );
    assert_eq!(next, BTreeSet::from([6]));

    let next = toggle_open_key(
        ExpansionMode::Multiple,
        &BTreeSet::from([5]),
        6,
        &[5, 6, 7],
        false,
    );
    assert_eq!(next, BTreeSet::from([5, 6]));
}

#[test]
fn apply_external_open_key_sync_updates_membership_for_target_key() {
    let current = BTreeSet::from([2, 4]);
    let next = apply_external_open_key_sync(&current, 6, true);
    assert_eq!(next, BTreeSet::from([2, 4, 6]));

    let next = apply_external_open_key_sync(&next, 4, false);
    assert_eq!(next, BTreeSet::from([2, 6]));
}

#[test]
fn plan_open_commit_normalizes_and_reports_changed_callback_keys() {
    let before = BTreeSet::from([1, 2]);
    let requested_next = BTreeSet::from([2, 3]);
    let plan = match plan_open_commit(
        ExpansionMode::Multiple,
        &before,
        &requested_next,
        &[1, 2, 3],
        &[1, 2, 3],
        false,
    ) {
        Some(plan) => plan,
        None => panic!("changed plan expected"),
    };

    assert_eq!(plan.next, BTreeSet::from([2, 3]));
    assert_eq!(plan.changed_by_key, BTreeMap::from([(1, false), (3, true)]));
}

#[test]
fn plan_open_commit_returns_none_when_normalized_state_is_unchanged() {
    let before = BTreeSet::from([2]);
    let requested_next = BTreeSet::from([2]);
    let plan = plan_open_commit(
        ExpansionMode::Single,
        &before,
        &requested_next,
        &[1, 2, 3],
        &[1, 2, 3],
        false,
    );
    assert!(plan.is_none());
}
