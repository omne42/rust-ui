#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Accent,
    Danger,
}

impl AlertVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            AlertVariant::Default => "ui-alert--variant-default",
            AlertVariant::Accent => "ui-alert--variant-accent",
            AlertVariant::Danger => "ui-alert--variant-danger",
        }
    }
}
