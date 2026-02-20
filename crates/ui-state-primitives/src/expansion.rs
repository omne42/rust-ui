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
#[path = "test/expansion.rs"]
mod tests;
