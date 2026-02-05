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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_class_names_are_stable() {
        assert_eq!(
            RadioGroupOrientation::Vertical.class_name(),
            "ui-radio-group--vertical"
        );
        assert_eq!(
            RadioGroupOrientation::Horizontal.class_name(),
            "ui-radio-group--horizontal"
        );
    }

    #[test]
    fn roving_orientation_matches_headless_contract() {
        assert_eq!(
            RadioGroupOrientation::Vertical.roving_orientation(),
            RovingOrientation::Vertical
        );
        assert_eq!(
            RadioGroupOrientation::Horizontal.roving_orientation(),
            RovingOrientation::Horizontal
        );
    }
}
