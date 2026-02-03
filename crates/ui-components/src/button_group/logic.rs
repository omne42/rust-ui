#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ButtonGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonGroupOrientation::Horizontal => "ui-button-group--horizontal",
            ButtonGroupOrientation::Vertical => "ui-button-group--vertical",
        }
    }
}
