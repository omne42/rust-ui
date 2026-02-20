pub const DEFAULT_ARIA_LABEL: &str = "Meter";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MeterVariant {
    #[default]
    Default,
    Danger,
}

impl MeterVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            MeterVariant::Default => "ui-meter--variant-default",
            MeterVariant::Danger => "ui-meter--variant-danger",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            MeterVariant::Default => "default",
            MeterVariant::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MeterSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl MeterSize {
    pub fn class_name(self) -> &'static str {
        match self {
            MeterSize::Sm => "ui-meter--size-sm",
            MeterSize::Default => "ui-meter--size-default",
            MeterSize::Lg => "ui-meter--size-lg",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            MeterSize::Sm => "sm",
            MeterSize::Default => "default",
            MeterSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MeterRange {
    pub min: f64,
    pub max: f64,
}

impl MeterRange {
    pub fn sanitized(min: f64, max: f64) -> Self {
        let mut min = if min.is_finite() { min } else { 0.0 };
        let mut max = if max.is_finite() { max } else { 100.0 };

        if max <= min {
            (min, max) = (0.0, 100.0);
        }

        Self { min, max }
    }

    pub fn span(self) -> f64 {
        (self.max - self.min).max(f64::EPSILON)
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_aria_label(aria_label: Option<String>, label: Option<String>) -> (String, bool) {
    if let Some(label) =
        normalize_optional_text(aria_label).or_else(|| normalize_optional_text(label))
    {
        let is_custom = label != DEFAULT_ARIA_LABEL;
        return (label, is_custom);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_value_label(value: Option<String>) -> (Option<String>, bool) {
    let value = normalize_optional_text(value);
    let has_custom_value_label = value.is_some();
    (value, has_custom_value_label)
}

pub fn clamp_to_range(value: f64, range: MeterRange) -> f64 {
    if !value.is_finite() {
        return range.min;
    }

    value.clamp(range.min, range.max)
}

pub fn normalize_progress(value: f64, range: MeterRange) -> f64 {
    ((value - range.min) / range.span()).clamp(0.0, 1.0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeterPhase {
    Determinate,
    Indeterminate,
}

impl MeterPhase {
    pub fn class_name(self) -> &'static str {
        match self {
            MeterPhase::Determinate => "ui-meter--state-determinate",
            MeterPhase::Indeterminate => "ui-meter--state-indeterminate",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            MeterPhase::Determinate => "determinate",
            MeterPhase::Indeterminate => "indeterminate",
        }
    }
}

pub fn resolve_phase(is_indeterminate: bool) -> MeterPhase {
    if is_indeterminate {
        MeterPhase::Indeterminate
    } else {
        MeterPhase::Determinate
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeterStateInput {
    pub variant: MeterVariant,
    pub size: MeterSize,
    pub has_custom_aria_label: bool,
    pub has_custom_value_label: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeterState {
    pub variant: MeterVariant,
    pub size: MeterSize,
    pub has_custom_aria_label: bool,
    pub has_custom_value_label: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
    pub variant_class: &'static str,
    pub size_class: &'static str,
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub label_source_class: &'static str,
    pub value_label_source_class: &'static str,
    pub motion_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub value_label_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn resolve_state(input: MeterStateInput) -> MeterState {
    let (label_source_class, label_source_attr) = if input.has_custom_aria_label {
        ("ui-meter--label-custom", "custom")
    } else {
        ("ui-meter--label-default", "default")
    };

    let (value_label_source_class, value_label_source_attr) = if input.has_custom_value_label {
        ("ui-meter--value-label-custom", "custom")
    } else {
        ("ui-meter--value-label-auto", "auto")
    };

    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-meter--motion-custom", "custom")
    } else {
        ("ui-meter--motion-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    MeterState {
        variant: input.variant,
        size: input.size,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_value_label: input.has_custom_value_label,
        has_custom_motion: input.has_custom_motion,
        has_custom_class_name: input.has_custom_class_name,
        variant_class: input.variant.class_name(),
        size_class: input.size.class_name(),
        variant_attr: input.variant.as_str(),
        size_attr: input.size.as_str(),
        label_source_class,
        value_label_source_class,
        motion_source_class,
        label_source_attr,
        value_label_source_attr,
        motion_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: MeterState) -> String {
    let mut classes = vec![
        "ui-meter".to_string(),
        state.variant_class.into(),
        state.size_class.into(),
        state.label_source_class.into(),
        state.value_label_source_class.into(),
        state.motion_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-meter--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/meter.rs"]
mod tests;
