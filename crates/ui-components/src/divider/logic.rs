#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl DividerOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            DividerOrientation::Horizontal => "ui-divider--horizontal",
            DividerOrientation::Vertical => "ui-divider--vertical",
        }
    }
}
