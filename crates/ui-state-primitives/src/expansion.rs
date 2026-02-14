use std::collections::BTreeSet;

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
mod tests {
    use super::*;

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
}
