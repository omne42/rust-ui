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

    pub fn as_attr(self) -> &'static str {
        match self {
            BadgeVariant::Default => "default",
            BadgeVariant::Accent => "accent",
            BadgeVariant::Danger => "danger",
            BadgeVariant::Outline => "outline",
        }
    }

    pub fn fill_class(self) -> &'static str {
        match self {
            BadgeVariant::Outline => "ui-badge--fill-outline",
            BadgeVariant::Default | BadgeVariant::Accent | BadgeVariant::Danger => {
                "ui-badge--fill-solid"
            }
        }
    }

    pub fn fill_attr(self) -> &'static str {
        match self {
            BadgeVariant::Outline => "outline",
            BadgeVariant::Default | BadgeVariant::Accent | BadgeVariant::Danger => "solid",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BadgeStateInput {
    pub variant: BadgeVariant,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BadgeState {
    pub variant: BadgeVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub fill_class: &'static str,
    pub fill_attr: &'static str,
    pub is_solid: bool,
    pub is_outline: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_state(input: BadgeStateInput) -> BadgeState {
    let is_outline = matches!(input.variant, BadgeVariant::Outline);

    BadgeState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        fill_class: input.variant.fill_class(),
        fill_attr: input.variant.fill_attr(),
        is_solid: !is_outline,
        is_outline,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/badge.rs"]
mod tests;
