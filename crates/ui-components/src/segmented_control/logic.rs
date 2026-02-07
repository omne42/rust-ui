use std::collections::HashSet;
use ui_headless::RovingOrientation;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SegmentedControlOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SegmentedControlOrientation {
    pub fn roving_orientation(self) -> RovingOrientation {
        match self {
            SegmentedControlOrientation::Horizontal => RovingOrientation::Horizontal,
            SegmentedControlOrientation::Vertical => RovingOrientation::Vertical,
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            SegmentedControlOrientation::Horizontal => "ui-segmented-control--horizontal",
            SegmentedControlOrientation::Vertical => "ui-segmented-control--vertical",
        }
    }

    pub fn aria_orientation(self) -> &'static str {
        match self {
            SegmentedControlOrientation::Horizontal => "horizontal",
            SegmentedControlOrientation::Vertical => "vertical",
        }
    }

    pub fn data_orientation(self) -> &'static str {
        self.aria_orientation()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentedControlState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub has_disabled_options: bool,
    pub disabled_option_count: usize,
    pub selected_index: Option<usize>,
    pub has_selection: bool,
    pub selection_empty: bool,
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub has_label: bool,
}

pub fn resolve_state(
    item_count: usize,
    is_disabled: bool,
    disabled_indices: &HashSet<usize>,
    selected_index: Option<usize>,
    orientation: SegmentedControlOrientation,
    has_label: bool,
) -> SegmentedControlState {
    let has_items = item_count > 0;
    let selected_index = selected_index.filter(|index| *index < item_count);
    let has_selection = selected_index.is_some();
    let disabled_option_count = disabled_indices
        .iter()
        .filter(|index| **index < item_count)
        .count();

    SegmentedControlState {
        item_count,
        is_empty: !has_items,
        has_items,
        is_disabled,
        has_disabled_options: disabled_option_count > 0,
        disabled_option_count,
        selected_index,
        has_selection,
        selection_empty: !has_selection,
        is_horizontal: matches!(orientation, SegmentedControlOrientation::Horizontal),
        is_vertical: matches!(orientation, SegmentedControlOrientation::Vertical),
        has_label,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SegmentedControlSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl SegmentedControlSize {
    pub fn class_name(self) -> &'static str {
        match self {
            SegmentedControlSize::Default => "ui-segmented-control--size-default",
            SegmentedControlSize::Sm => "ui-segmented-control--size-sm",
            SegmentedControlSize::Lg => "ui-segmented-control--size-lg",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_class_names_are_stable() {
        assert_eq!(
            SegmentedControlOrientation::Horizontal.class_name(),
            "ui-segmented-control--horizontal"
        );
        assert_eq!(
            SegmentedControlOrientation::Vertical.class_name(),
            "ui-segmented-control--vertical"
        );
    }

    #[test]
    fn roving_orientation_matches_headless_contract() {
        assert_eq!(
            SegmentedControlOrientation::Horizontal.roving_orientation(),
            RovingOrientation::Horizontal
        );
        assert_eq!(
            SegmentedControlOrientation::Vertical.roving_orientation(),
            RovingOrientation::Vertical
        );
    }

    #[test]
    fn aria_and_data_orientation_values_are_stable() {
        assert_eq!(
            SegmentedControlOrientation::Horizontal.aria_orientation(),
            "horizontal"
        );
        assert_eq!(
            SegmentedControlOrientation::Vertical.aria_orientation(),
            "vertical"
        );
        assert_eq!(
            SegmentedControlOrientation::Horizontal.data_orientation(),
            "horizontal"
        );
        assert_eq!(
            SegmentedControlOrientation::Vertical.data_orientation(),
            "vertical"
        );
    }

    #[test]
    fn resolve_state_tracks_empty_disabled_group() {
        let disabled = HashSet::new();
        let state = resolve_state(
            0,
            true,
            &disabled,
            Some(0),
            SegmentedControlOrientation::Horizontal,
            false,
        );

        assert_eq!(state.item_count, 0);
        assert!(state.is_empty);
        assert!(!state.has_items);
        assert!(state.is_disabled);
        assert!(!state.has_disabled_options);
        assert_eq!(state.disabled_option_count, 0);
        assert_eq!(state.selected_index, None);
        assert!(!state.has_selection);
        assert!(state.selection_empty);
        assert!(state.is_horizontal);
        assert!(!state.is_vertical);
        assert!(!state.has_label);
    }

    #[test]
    fn resolve_state_tracks_selection_orientation_and_disabled_options() {
        let disabled = HashSet::from([1_usize, 8_usize]);
        let state = resolve_state(
            3,
            false,
            &disabled,
            Some(2),
            SegmentedControlOrientation::Vertical,
            true,
        );

        assert_eq!(state.item_count, 3);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(!state.is_disabled);
        assert!(state.has_disabled_options);
        assert_eq!(state.disabled_option_count, 1);
        assert_eq!(state.selected_index, Some(2));
        assert!(state.has_selection);
        assert!(!state.selection_empty);
        assert!(!state.is_horizontal);
        assert!(state.is_vertical);
        assert!(state.has_label);
    }

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(
            SegmentedControlSize::Default.class_name(),
            "ui-segmented-control--size-default"
        );
        assert_eq!(
            SegmentedControlSize::Sm.class_name(),
            "ui-segmented-control--size-sm"
        );
        assert_eq!(
            SegmentedControlSize::Lg.class_name(),
            "ui-segmented-control--size-lg"
        );
    }
}
