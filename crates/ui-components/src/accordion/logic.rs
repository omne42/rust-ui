use std::collections::BTreeSet;

use ui_state_primitives::expansion::{
    ExpansionMode, ExpansionSummary, normalize_open_indices, summarize, toggle_open_indices,
};

pub type AccordionSelectionMode = ExpansionMode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccordionState {
    pub is_empty: bool,
    pub has_items: bool,
    pub open_count: usize,
    pub has_open_items: bool,
    pub has_multiple_open: bool,
    pub has_disabled_items: bool,
}

pub fn resolve_state(
    mode: AccordionSelectionMode,
    item_count: usize,
    open_count: usize,
    has_disabled_items: bool,
) -> AccordionState {
    let ExpansionSummary {
        is_empty,
        has_items,
        open_count,
        has_open_items,
        has_multiple_open,
    } = summarize(mode, item_count, open_count);

    AccordionState {
        is_empty,
        has_items,
        open_count,
        has_open_items,
        has_multiple_open,
        has_disabled_items,
    }
}

pub fn normalize_open_indices_for_items(
    mode: AccordionSelectionMode,
    open: &BTreeSet<usize>,
    item_count: usize,
) -> BTreeSet<usize> {
    normalize_open_indices(mode, open, item_count)
}

pub fn toggle_open_indices_for_items(
    mode: AccordionSelectionMode,
    open: &BTreeSet<usize>,
    index: usize,
) -> BTreeSet<usize> {
    toggle_open_indices(mode, open, index)
}
