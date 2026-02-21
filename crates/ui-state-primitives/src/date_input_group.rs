pub const DEFAULT_ARIA_LABEL: &str = "Date input group";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DateInputGroupVariant {
    #[default]
    Primary,
    Secondary,
}

impl DateInputGroupVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            DateInputGroupVariant::Primary => "ui-date-input-group--variant-primary",
            DateInputGroupVariant::Secondary => "ui-date-input-group--variant-secondary",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DateInputGroupVariant::Primary => "primary",
            DateInputGroupVariant::Secondary => "secondary",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DateInputGroupWidth {
    #[default]
    Fit,
    Full,
}

impl DateInputGroupWidth {
    pub fn class_name(self) -> &'static str {
        match self {
            DateInputGroupWidth::Fit => "ui-date-input-group--fit-width",
            DateInputGroupWidth::Full => "ui-date-input-group--full-width",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DateInputGroupWidth::Fit => "fit",
            DateInputGroupWidth::Full => "full",
        }
    }

    pub fn is_full(self) -> bool {
        matches!(self, DateInputGroupWidth::Full)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DateInputGroupStatus {
    #[default]
    Default,
    Invalid,
    Disabled,
    DisabledInvalid,
}

impl DateInputGroupStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            DateInputGroupStatus::Default => "default",
            DateInputGroupStatus::Invalid => "invalid",
            DateInputGroupStatus::Disabled => "disabled",
            DateInputGroupStatus::DisabledInvalid => "disabled-invalid",
        }
    }

    pub fn is_disabled(self) -> bool {
        matches!(
            self,
            DateInputGroupStatus::Disabled | DateInputGroupStatus::DisabledInvalid
        )
    }

    pub fn is_invalid(self) -> bool {
        matches!(
            self,
            DateInputGroupStatus::Invalid | DateInputGroupStatus::DisabledInvalid
        )
    }
}

pub fn resolve_width(is_full_width: bool) -> DateInputGroupWidth {
    if is_full_width {
        DateInputGroupWidth::Full
    } else {
        DateInputGroupWidth::Fit
    }
}

pub fn resolve_status(is_disabled: bool, is_invalid: bool) -> DateInputGroupStatus {
    if is_disabled && is_invalid {
        DateInputGroupStatus::DisabledInvalid
    } else if is_disabled {
        DateInputGroupStatus::Disabled
    } else if is_invalid {
        DateInputGroupStatus::Invalid
    } else {
        DateInputGroupStatus::Default
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateInputGroupStateInput {
    pub variant: DateInputGroupVariant,
    pub width: DateInputGroupWidth,
    pub status: DateInputGroupStatus,
    pub segmented: bool,
    pub has_prefix: bool,
    pub has_suffix: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateInputGroupState {
    pub variant: DateInputGroupVariant,
    pub width: DateInputGroupWidth,
    pub status: DateInputGroupStatus,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub width_class: &'static str,
    pub width_attr: &'static str,
    pub is_full_width: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_segmented: bool,
    pub has_prefix: bool,
    pub has_suffix: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: DateInputGroupStateInput) -> DateInputGroupState {
    let data_state_attr = if input.segmented && input.status == DateInputGroupStatus::Default {
        "segmented"
    } else {
        input.status.as_attr()
    };

    DateInputGroupState {
        variant: input.variant,
        width: input.width,
        status: input.status,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        width_class: input.width.class_name(),
        width_attr: input.width.as_attr(),
        is_full_width: input.width.is_full(),
        is_disabled: input.status.is_disabled(),
        is_invalid: input.status.is_invalid(),
        is_segmented: input.segmented,
        has_prefix: input.has_prefix,
        has_suffix: input.has_suffix,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

#[cfg(test)]
#[path = "test/date_input_group.rs"]
mod tests;
