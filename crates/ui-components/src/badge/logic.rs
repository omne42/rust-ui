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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_class_names_are_stable() {
        assert_eq!(
            BadgeVariant::Default.class_name(),
            "ui-badge--variant-default"
        );
        assert_eq!(
            BadgeVariant::Accent.class_name(),
            "ui-badge--variant-accent"
        );
        assert_eq!(
            BadgeVariant::Danger.class_name(),
            "ui-badge--variant-danger"
        );
        assert_eq!(
            BadgeVariant::Outline.class_name(),
            "ui-badge--variant-outline"
        );
    }
}
