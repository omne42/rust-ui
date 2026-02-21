use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ExpansionMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExpansionSummary {
    pub is_empty: bool,
    pub has_items: bool,
    pub open_count: usize,
    pub has_open_items: bool,
    pub has_multiple_open: bool,
}

pub fn toggle_open_indices(
    mode: ExpansionMode,
    open: &BTreeSet<usize>,
    index: usize,
) -> BTreeSet<usize> {
    let mut next = open.clone();
    match mode {
        ExpansionMode::Single => {
            if next.contains(&index) {
                next.remove(&index);
            } else {
                next.clear();
                next.insert(index);
            }
        }
        ExpansionMode::Multiple => {
            if next.contains(&index) {
                next.remove(&index);
            } else {
                next.insert(index);
            }
        }
    }
    next
}

pub fn normalize_open_indices(
    mode: ExpansionMode,
    open: &BTreeSet<usize>,
    item_count: usize,
) -> BTreeSet<usize> {
    let mut next = open
        .iter()
        .copied()
        .filter(|&index| index < item_count)
        .collect::<BTreeSet<_>>();

    if mode == ExpansionMode::Single && next.len() > 1 {
        let first = next.iter().copied().next();
        next.clear();
        if let Some(first) = first {
            next.insert(first);
        }
    }

    next
}

fn key_to_index_map<K: Ord + Copy>(item_keys: &[K]) -> BTreeMap<K, usize> {
    item_keys
        .iter()
        .copied()
        .enumerate()
        .map(|(index, key)| (key, index))
        .collect()
}

fn open_indices_from_keys<K: Ord + Copy>(open: &BTreeSet<K>, item_keys: &[K]) -> BTreeSet<usize> {
    let key_to_index = key_to_index_map(item_keys);
    open.iter()
        .filter_map(|key| key_to_index.get(key).copied())
        .collect()
}

fn open_keys_from_indices<K: Ord + Copy>(
    open_indices: &BTreeSet<usize>,
    item_keys: &[K],
) -> BTreeSet<K> {
    open_indices
        .iter()
        .filter_map(|index| item_keys.get(*index).copied())
        .collect()
}

pub fn normalize_open_keys<K: Ord + Copy>(
    mode: ExpansionMode,
    open: &BTreeSet<K>,
    item_keys: &[K],
    disallow_empty_selection: bool,
) -> BTreeSet<K> {
    let open_indices = open_indices_from_keys(open, item_keys);
    let mut normalized_indices = normalize_open_indices(mode, &open_indices, item_keys.len());
    if disallow_empty_selection && normalized_indices.is_empty() && !item_keys.is_empty() {
        normalized_indices.insert(0);
    }
    open_keys_from_indices(&normalized_indices, item_keys)
}

pub fn normalize_default_open_keys<K: Ord + Copy>(
    mode: ExpansionMode,
    default_open: Option<&BTreeSet<K>>,
    item_keys: &[K],
    disallow_empty_selection: bool,
) -> BTreeSet<K> {
    let default_open = default_open.cloned().unwrap_or_default();
    normalize_open_keys(mode, &default_open, item_keys, disallow_empty_selection)
}

pub fn toggle_open_key<K: Ord + Copy>(
    mode: ExpansionMode,
    open: &BTreeSet<K>,
    key: K,
    item_keys: &[K],
    disallow_empty_selection: bool,
) -> BTreeSet<K> {
    let key_to_index = key_to_index_map(item_keys);
    let Some(index) = key_to_index.get(&key).copied() else {
        return normalize_open_keys(mode, open, item_keys, disallow_empty_selection);
    };
    let open_indices = open_indices_from_keys(open, item_keys);
    let normalized_indices = normalize_open_indices(mode, &open_indices, item_keys.len());
    if disallow_empty_selection
        && normalized_indices.len() == 1
        && normalized_indices.contains(&index)
    {
        return open_keys_from_indices(&normalized_indices, item_keys);
    }
    let next_indices = toggle_open_indices(mode, &normalized_indices, index);
    let mut next = open_keys_from_indices(&next_indices, item_keys);
    if disallow_empty_selection && next.is_empty() && !item_keys.is_empty() {
        next.insert(item_keys[0]);
    }
    next
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpansionOpenCommitPlan<K: Ord + Copy> {
    pub next: BTreeSet<K>,
    pub changed_by_key: BTreeMap<K, bool>,
}

pub fn apply_external_open_key_sync<K: Ord + Copy>(
    current: &BTreeSet<K>,
    key: K,
    should_open: bool,
) -> BTreeSet<K> {
    let mut next = current.clone();
    if should_open {
        next.insert(key);
    } else {
        next.remove(&key);
    }
    next
}

pub fn plan_open_commit<K: Ord + Copy>(
    mode: ExpansionMode,
    before: &BTreeSet<K>,
    requested_next: &BTreeSet<K>,
    item_keys: &[K],
    callback_keys: &[K],
    disallow_empty_selection: bool,
) -> Option<ExpansionOpenCommitPlan<K>> {
    let next = normalize_open_keys(mode, requested_next, item_keys, disallow_empty_selection);
    if before == &next {
        return None;
    }

    let changed_by_key = callback_keys
        .iter()
        .copied()
        .filter_map(|key| {
            let before_open = before.contains(&key);
            let after_open = next.contains(&key);
            (before_open != after_open).then_some((key, after_open))
        })
        .collect::<BTreeMap<_, _>>();

    Some(ExpansionOpenCommitPlan {
        next,
        changed_by_key,
    })
}

pub fn summarize(mode: ExpansionMode, item_count: usize, open_count: usize) -> ExpansionSummary {
    let has_items = item_count > 0;
    let mut open_count = open_count.min(item_count);

    if mode == ExpansionMode::Single {
        open_count = open_count.min(1);
    }

    ExpansionSummary {
        is_empty: !has_items,
        has_items,
        open_count,
        has_open_items: open_count > 0,
        has_multiple_open: open_count > 1,
    }
}

#[cfg(test)]
#[path = "test/expansion.rs"]
mod tests;
