pub use crate::button::normalize_optional_text;

pub const DEFAULT_REMOVE_ARIA_LABEL: &str = "Remove tag";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TagVariant {
    #[default]
    Default,
    Surface,
}

impl TagVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            TagVariant::Default => "ui-tag--variant-default",
            TagVariant::Surface => "ui-tag--variant-surface",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TagVariant::Default => "default",
            TagVariant::Surface => "surface",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TagSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl TagSize {
    pub fn class_name(self) -> &'static str {
        match self {
            TagSize::Sm => "ui-tag--size-sm",
            TagSize::Md => "ui-tag--size-md",
            TagSize::Lg => "ui-tag--size-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TagSize::Sm => "sm",
            TagSize::Md => "md",
            TagSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagStateInput {
    pub variant: TagVariant,
    pub size: TagSize,
    pub disabled: bool,
    pub removable: bool,
    pub has_remove_handler: bool,
    pub has_custom_remove_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagState {
    pub variant: TagVariant,
    pub size: TagSize,
    pub variant_class: &'static str,
    pub size_class: &'static str,
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub is_enabled: bool,
    pub is_disabled: bool,
    pub is_removable: bool,
    pub is_static: bool,
    pub has_remove_handler: bool,
    pub has_custom_remove_aria_label: bool,
    pub remove_label_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
}

pub fn normalize_remove_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_REMOVE_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: TagStateInput) -> TagState {
    let is_removable = input.removable && input.has_remove_handler;

    let (state_class, state_attr) = if input.disabled {
        ("ui-tag--disabled", "disabled")
    } else if is_removable {
        ("ui-tag--removable", "removable")
    } else {
        ("ui-tag--static", "static")
    };

    TagState {
        variant: input.variant,
        size: input.size,
        variant_class: input.variant.class_name(),
        size_class: input.size.class_name(),
        variant_attr: input.variant.as_attr(),
        size_attr: input.size.as_attr(),
        state_class,
        state_attr,
        is_enabled: !input.disabled,
        is_disabled: input.disabled,
        is_removable,
        is_static: !is_removable,
        has_remove_handler: input.has_remove_handler,
        has_custom_remove_aria_label: input.has_custom_remove_aria_label,
        remove_label_source_attr: if input.has_custom_remove_aria_label {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

#[cfg(test)]
#[path = "test/tag.rs"]
mod tests;
