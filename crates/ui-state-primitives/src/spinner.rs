#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SpinnerSize {
    pub fn class_name(self) -> &'static str {
        match self {
            SpinnerSize::Sm => "ui-spinner--size-sm",
            SpinnerSize::Md => "ui-spinner--size-md",
            SpinnerSize::Lg => "ui-spinner--size-lg",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SpinnerSize::Sm => "sm",
            SpinnerSize::Md => "md",
            SpinnerSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpinnerStateInput {
    pub size: SpinnerSize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpinnerState {
    pub size: SpinnerSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_aria_label(value: Option<String>, default: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        let is_custom = label != default;
        return (label, is_custom);
    }

    (default.into(), false)
}

pub fn resolve_state(input: SpinnerStateInput) -> SpinnerState {
    let (label_source_class, label_source_attr) = if input.has_custom_aria_label {
        ("ui-spinner--label-custom", "custom")
    } else {
        ("ui-spinner--label-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    SpinnerState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_str(),
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        label_source_class,
        label_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SpinnerState) -> String {
    let mut classes = vec![
        "ui-spinner".to_string(),
        state.size_class.into(),
        state.label_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-spinner--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/spinner.rs"]
mod tests;
