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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_activation_defaults_to_automatic() {
        assert_eq!(
            TabsKeyboardActivation::default(),
            TabsKeyboardActivation::Automatic
        );
        assert!(TabsKeyboardActivation::Automatic.selects_on_focus());
        assert!(!TabsKeyboardActivation::Manual.selects_on_focus());
    }

    #[test]
    fn normalize_index_clamps_to_bounds() {
        assert_eq!(normalize_index_skipping_disabled(0, 0, |_| false), 0);
        assert_eq!(normalize_index_skipping_disabled(1, 1, |_| false), 0);
        assert_eq!(normalize_index_skipping_disabled(2, 2, |_| false), 1);
    }

    #[test]
    fn normalize_index_skips_disabled_when_possible() {
        assert_eq!(normalize_index_skipping_disabled(0, 3, |idx| idx == 0), 1);
        assert_eq!(normalize_index_skipping_disabled(2, 3, |idx| idx == 2), 0);
    }

    #[test]
    fn resolve_next_selected_respects_keyboard_activation() {
        let is_disabled = |idx: usize| idx == 2;

        let current = 0;
        let candidate = 1;
        assert_eq!(
            resolve_next_selected_index(
                current,
                candidate,
                3,
                is_disabled,
                TabsKeyboardActivation::Manual,
                TabsSelectionTrigger::Focus
            ),
            current
        );
        assert_eq!(
            resolve_next_selected_index(
                current,
                candidate,
                3,
                is_disabled,
                TabsKeyboardActivation::Automatic,
                TabsSelectionTrigger::Focus
            ),
            candidate
        );

        assert_eq!(
            resolve_next_selected_index(
                current,
                candidate,
                3,
                is_disabled,
                TabsKeyboardActivation::Manual,
                TabsSelectionTrigger::Press
            ),
            candidate
        );
    }

    #[test]
    fn resolve_next_selected_ignores_disabled_candidates() {
        let is_disabled = |idx: usize| idx == 1;
        assert_eq!(
            resolve_next_selected_index(
                0,
                1,
                3,
                is_disabled,
                TabsKeyboardActivation::Automatic,
                TabsSelectionTrigger::Press
            ),
            0
        );
    }
}
