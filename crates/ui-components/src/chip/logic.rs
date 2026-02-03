#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChipVariant {
    #[default]
    Default,
    Accent,
    Danger,
    Outline,
}

impl ChipVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ChipVariant::Default => "ui-chip--variant-default",
            ChipVariant::Accent => "ui-chip--variant-accent",
            ChipVariant::Danger => "ui-chip--variant-danger",
            ChipVariant::Outline => "ui-chip--variant-outline",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChipSize {
    #[default]
    Md,
    Sm,
    Lg,
}

impl ChipSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ChipSize::Md => "ui-chip--size-md",
            ChipSize::Sm => "ui-chip--size-sm",
            ChipSize::Lg => "ui-chip--size-lg",
        }
    }
}
