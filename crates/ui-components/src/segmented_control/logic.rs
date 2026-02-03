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
