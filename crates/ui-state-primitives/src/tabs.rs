#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TabsKeyboardActivation {
    #[default]
    Automatic,
    Manual,
}

impl TabsKeyboardActivation {
    pub const fn selects_on_focus(self) -> bool {
        matches!(self, TabsKeyboardActivation::Automatic)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsSelectionTrigger {
    Focus,
    Press,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabsState {
    pub is_empty: bool,
    pub has_items: bool,
    pub selected_index: Option<usize>,
    pub has_disabled_tabs: bool,
}

pub fn normalize_index_skipping_disabled(
    index: usize,
    item_count: usize,
    is_disabled: impl Fn(usize) -> bool,
) -> usize {
    if item_count == 0 {
        return 0;
    }

    let index = index.min(item_count.saturating_sub(1));
    if !is_disabled(index) {
        return index;
    }

    (0..item_count)
        .find(|&idx| !is_disabled(idx))
        .unwrap_or(index)
}

pub fn resolve_next_selected_index(
    current: usize,
    candidate: usize,
    item_count: usize,
    is_disabled: impl Fn(usize) -> bool,
    keyboard_activation: TabsKeyboardActivation,
    trigger: TabsSelectionTrigger,
) -> usize {
    let candidate = normalize_index_skipping_disabled(candidate, item_count, &is_disabled);
    if is_disabled(candidate) {
        return current;
    }

    match trigger {
        TabsSelectionTrigger::Focus => {
            if keyboard_activation.selects_on_focus() {
                candidate
            } else {
                current
            }
        }
        TabsSelectionTrigger::Press => candidate,
    }
}

pub fn resolve_tabs_state(
    item_count: usize,
    selected_index: usize,
    has_disabled_tabs: bool,
) -> TabsState {
    let has_items = item_count > 0;

    TabsState {
        is_empty: !has_items,
        has_items,
        selected_index: has_items.then_some(selected_index.min(item_count.saturating_sub(1))),
        has_disabled_tabs,
    }
}

#[cfg(test)]
#[path = "test/tabs.rs"]
mod tests;
