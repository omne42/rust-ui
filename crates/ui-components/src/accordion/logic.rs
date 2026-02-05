use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AccordionSelectionMode {
    #[default]
    Single,
    Multiple,
}

pub fn toggle_open_indices(
    mode: AccordionSelectionMode,
    open: &BTreeSet<usize>,
    index: usize,
) -> BTreeSet<usize> {
    let mut next = open.clone();
    match mode {
        AccordionSelectionMode::Single => {
            if next.contains(&index) {
                next.remove(&index);
            } else {
                next.clear();
                next.insert(index);
            }
        }
        AccordionSelectionMode::Multiple => {
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
    mode: AccordionSelectionMode,
    open: &BTreeSet<usize>,
    item_count: usize,
) -> BTreeSet<usize> {
    let mut next = open
        .iter()
        .copied()
        .filter(|&index| index < item_count)
        .collect::<BTreeSet<_>>();

    if mode == AccordionSelectionMode::Single && next.len() > 1 {
        let first = next.iter().copied().next();
        next.clear();
        if let Some(first) = first {
            next.insert(first);
        }
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_mode_only_keeps_one_open() {
        let mut open = BTreeSet::new();
        open.insert(1);
        open.insert(2);

        let open = toggle_open_indices(AccordionSelectionMode::Single, &open, 3);
        assert_eq!(open.iter().copied().collect::<Vec<_>>(), vec![3]);
    }

    #[test]
    fn single_mode_toggles_closed_when_open() {
        let mut open = BTreeSet::new();
        open.insert(1);

        let open = toggle_open_indices(AccordionSelectionMode::Single, &open, 1);
        assert!(open.is_empty());
    }

    #[test]
    fn multiple_mode_toggles_membership() {
        let open = BTreeSet::new();
        let open = toggle_open_indices(AccordionSelectionMode::Multiple, &open, 1);
        assert!(open.contains(&1));

        let open = toggle_open_indices(AccordionSelectionMode::Multiple, &open, 1);
        assert!(!open.contains(&1));
    }

    #[test]
    fn normalize_clamps_indices_to_item_count() {
        let open = BTreeSet::from([0, 2, 99]);
        let open = normalize_open_indices(AccordionSelectionMode::Multiple, &open, 3);
        assert_eq!(open.iter().copied().collect::<Vec<_>>(), vec![0, 2]);
    }

    #[test]
    fn normalize_single_keeps_only_first_open() {
        let open = BTreeSet::from([2, 1, 3]);
        let open = normalize_open_indices(AccordionSelectionMode::Single, &open, 10);
        assert_eq!(open.iter().copied().collect::<Vec<_>>(), vec![1]);
    }
}
