use ui_headless::RovingOrientation;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RadioGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl RadioGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            RadioGroupOrientation::Vertical => "ui-radio-group--vertical",
            RadioGroupOrientation::Horizontal => "ui-radio-group--horizontal",
        }
    }

    pub fn roving_orientation(self) -> RovingOrientation {
        match self {
            RadioGroupOrientation::Vertical => RovingOrientation::Vertical,
            RadioGroupOrientation::Horizontal => RovingOrientation::Horizontal,
        }
    }
}
