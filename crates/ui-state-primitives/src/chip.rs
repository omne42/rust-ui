pub use crate::button::normalize_optional_text;

pub const DEFAULT_DISMISS_ARIA_LABEL: &str = "Remove tag";

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

    pub fn as_str(self) -> &'static str {
        match self {
            ChipVariant::Default => "default",
            ChipVariant::Accent => "accent",
            ChipVariant::Danger => "danger",
            ChipVariant::Outline => "outline",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChipSize {
    Sm,
    #[default]
    Md,
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

    pub fn as_str(self) -> &'static str {
        match self {
            ChipSize::Sm => "sm",
            ChipSize::Md => "md",
            ChipSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChipStateInput {
    pub variant: ChipVariant,
    pub size: ChipSize,
    pub disabled: bool,
    pub has_dismiss_action: bool,
    pub has_custom_dismiss_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChipState {
    pub variant: ChipVariant,
    pub size: ChipSize,
    pub variant_class: &'static str,
    pub size_class: &'static str,
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_dismiss_action: bool,
    pub is_static: bool,
    pub has_custom_dismiss_aria_label: bool,
    pub dismiss_label_source_class: &'static str,
    pub dismiss_label_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
}

pub fn resolve_dismiss_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_DISMISS_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: ChipStateInput) -> ChipState {
    let (state_class, state_attr) = if input.disabled {
        ("ui-chip--disabled", "disabled")
    } else if input.has_dismiss_action {
        ("ui-chip--removable", "removable")
    } else {
        ("ui-chip--static", "static")
    };

    let (dismiss_label_source_class, dismiss_label_source_attr) =
        if input.has_custom_dismiss_aria_label {
            ("ui-chip--dismiss-label-custom", "custom")
        } else {
            ("ui-chip--dismiss-label-default", "default")
        };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    ChipState {
        variant: input.variant,
        size: input.size,
        variant_class: input.variant.class_name(),
        size_class: input.size.class_name(),
        variant_attr: input.variant.as_str(),
        size_attr: input.size.as_str(),
        state_class,
        state_attr,
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        has_dismiss_action: input.has_dismiss_action,
        is_static: !input.has_dismiss_action,
        has_custom_dismiss_aria_label: input.has_custom_dismiss_aria_label,
        dismiss_label_source_class,
        dismiss_label_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr,
    }
}

#[cfg(test)]
#[path = "test/chip.rs"]
mod tests;
