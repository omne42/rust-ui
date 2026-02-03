#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Muted,
    Outline,
}

impl CardVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            CardVariant::Default => "ui-card--variant-default",
            CardVariant::Muted => "ui-card--variant-muted",
            CardVariant::Outline => "ui-card--variant-outline",
        }
    }
}
