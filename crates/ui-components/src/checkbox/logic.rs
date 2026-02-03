#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxVariant {
    #[default]
    Default,
    Accent,
}

impl CheckboxVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-checkbox--variant-default",
            Self::Accent => "ui-checkbox--variant-accent",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl CheckboxSize {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-checkbox--size-default",
            Self::Sm => "ui-checkbox--size-sm",
            Self::Lg => "ui-checkbox--size-lg",
        }
    }
}
