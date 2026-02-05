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
