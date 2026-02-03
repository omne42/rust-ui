#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToggleButtonGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            ToggleButtonGroupOrientation::Horizontal => "ui-toggle-button-group--horizontal",
            ToggleButtonGroupOrientation::Vertical => "ui-toggle-button-group--vertical",
        }
    }
}
