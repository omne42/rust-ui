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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_class_names_are_stable() {
        assert_eq!(
            CheckboxVariant::Default.class_name(),
            "ui-checkbox--variant-default"
        );
        assert_eq!(
            CheckboxVariant::Accent.class_name(),
            "ui-checkbox--variant-accent"
        );
    }

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(
            CheckboxSize::Default.class_name(),
            "ui-checkbox--size-default"
        );
        assert_eq!(CheckboxSize::Sm.class_name(), "ui-checkbox--size-sm");
        assert_eq!(CheckboxSize::Lg.class_name(), "ui-checkbox--size-lg");
    }
}
