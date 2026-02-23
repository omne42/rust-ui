pub const DEFAULT_ARIA_LABEL: &str = "Progress";
pub const DEFAULT_MAX: f64 = 100.0;
pub const MIN_MAX: f64 = 1.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressBarVariant {
    #[default]
    Default,
    Accent,
    Danger,
}

impl ProgressBarVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ProgressBarVariant::Default => "ui-progress-bar--variant-default",
            ProgressBarVariant::Accent => "ui-progress-bar--variant-accent",
            ProgressBarVariant::Danger => "ui-progress-bar--variant-danger",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ProgressBarVariant::Default => "default",
            ProgressBarVariant::Accent => "accent",
            ProgressBarVariant::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressBarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ProgressBarSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ProgressBarSize::Sm => "ui-progress-bar--size-sm",
            ProgressBarSize::Md => "ui-progress-bar--size-md",
            ProgressBarSize::Lg => "ui-progress-bar--size-lg",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ProgressBarSize::Sm => "sm",
            ProgressBarSize::Md => "md",
            ProgressBarSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressBarPhase {
    Determinate,
    Indeterminate,
}

impl ProgressBarPhase {
    pub fn class_name(self) -> &'static str {
        match self {
            ProgressBarPhase::Determinate => "ui-progress-bar--state-determinate",
            ProgressBarPhase::Indeterminate => "ui-progress-bar--state-indeterminate",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ProgressBarPhase::Determinate => "determinate",
            ProgressBarPhase::Indeterminate => "indeterminate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressBarMode {
    #[default]
    Auto,
    Indeterminate,
}

impl ProgressBarMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ProgressBarMode::Auto => "auto",
            ProgressBarMode::Indeterminate => "indeterminate",
        }
    }

    pub fn is_indeterminate(self) -> bool {
        matches!(self, ProgressBarMode::Indeterminate)
    }
}

pub fn normalize_mode(is_indeterminate: bool) -> ProgressBarMode {
    if is_indeterminate {
        ProgressBarMode::Indeterminate
    } else {
        ProgressBarMode::Auto
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgressBarValueAxisInput {
    pub is_controlled: bool,
    pub has_default_value: bool,
    pub has_on_value_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgressBarValueAxisState {
    pub is_controlled: bool,
    pub has_default_value: bool,
    pub has_on_value_change: bool,
    pub mode_attr: &'static str,
    pub value_source_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
}

pub fn resolve_value_axis(input: ProgressBarValueAxisInput) -> ProgressBarValueAxisState {
    ProgressBarValueAxisState {
        is_controlled: input.is_controlled,
        has_default_value: input.has_default_value,
        has_on_value_change: input.has_on_value_change,
        mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        value_source_attr: if input.is_controlled {
            "external"
        } else {
            "default_value"
        },
        default_value_source_attr: if input.has_default_value {
            "provided"
        } else {
            "default"
        },
        value_change_source_attr: if input.has_on_value_change {
            "provided"
        } else {
            "none"
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressBarStateInput {
    pub variant: ProgressBarVariant,
    pub size: ProgressBarSize,
    pub value: Option<f64>,
    pub max: f64,
    pub indeterminate: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressBarState {
    pub variant: ProgressBarVariant,
    pub size: ProgressBarSize,
    pub phase: ProgressBarPhase,
    pub variant_class: &'static str,
    pub size_class: &'static str,
    pub phase_class: &'static str,
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub phase_attr: &'static str,
    pub value: Option<f64>,
    pub max: f64,
    pub has_value: bool,
    pub is_determinate: bool,
    pub is_indeterminate: bool,
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

pub fn resolve_aria_label(value: String) -> (String, bool) {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return (DEFAULT_ARIA_LABEL.into(), false);
    }

    let normalized = trimmed.into();
    let has_custom_aria_label = normalized != DEFAULT_ARIA_LABEL;

    (normalized, has_custom_aria_label)
}

pub fn sanitize_max(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        MIN_MAX
    }
}

pub fn sanitize_value(value: Option<f64>, max: f64) -> Option<f64> {
    value
        .filter(|value| value.is_finite())
        .map(|value| value.clamp(0.0, max))
}

pub fn resolve_state(input: ProgressBarStateInput) -> ProgressBarState {
    let max = sanitize_max(input.max);
    let value = sanitize_value(input.value, max);
    let is_indeterminate = input.indeterminate || value.is_none();
    let value = (!is_indeterminate).then_some(value).flatten();
    let phase = if is_indeterminate {
        ProgressBarPhase::Indeterminate
    } else {
        ProgressBarPhase::Determinate
    };

    let (label_source_class, label_source_attr) = if input.has_custom_aria_label {
        ("ui-progress-bar--label-custom", "custom")
    } else {
        ("ui-progress-bar--label-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    ProgressBarState {
        variant: input.variant,
        size: input.size,
        phase,
        variant_class: input.variant.class_name(),
        size_class: input.size.class_name(),
        phase_class: phase.class_name(),
        variant_attr: input.variant.as_str(),
        size_attr: input.size.as_str(),
        phase_attr: phase.as_str(),
        value,
        max,
        has_value: value.is_some(),
        is_determinate: !is_indeterminate,
        is_indeterminate,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        label_source_class,
        label_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ProgressBarState) -> String {
    let mut classes = vec![
        "ui-progress-bar".to_string(),
        state.variant_class.into(),
        state.size_class.into(),
        state.phase_class.into(),
        state.label_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-progress-bar--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/progress_bar.rs"]
mod tests;
