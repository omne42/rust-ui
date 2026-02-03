#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Accent,
    Danger,
    Outline,
}

impl BadgeVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            BadgeVariant::Default => "ui-badge--variant-default",
            BadgeVariant::Accent => "ui-badge--variant-accent",
            BadgeVariant::Danger => "ui-badge--variant-danger",
            BadgeVariant::Outline => "ui-badge--variant-outline",
        }
    }
}
