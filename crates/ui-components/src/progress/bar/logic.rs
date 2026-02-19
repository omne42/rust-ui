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
mod tests {
    use super::*;

    #[test]
    fn variant_and_size_mappings_are_stable() {
        assert_eq!(
            ProgressBarVariant::Default.class_name(),
            "ui-progress-bar--variant-default"
        );
        assert_eq!(
            ProgressBarVariant::Accent.class_name(),
            "ui-progress-bar--variant-accent"
        );
        assert_eq!(
            ProgressBarVariant::Danger.class_name(),
            "ui-progress-bar--variant-danger"
        );

        assert_eq!(ProgressBarVariant::Default.as_str(), "default");
        assert_eq!(ProgressBarVariant::Accent.as_str(), "accent");
        assert_eq!(ProgressBarVariant::Danger.as_str(), "danger");

        assert_eq!(ProgressBarSize::Sm.class_name(), "ui-progress-bar--size-sm");
        assert_eq!(ProgressBarSize::Md.class_name(), "ui-progress-bar--size-md");
        assert_eq!(ProgressBarSize::Lg.class_name(), "ui-progress-bar--size-lg");

        assert_eq!(ProgressBarSize::Sm.as_str(), "sm");
        assert_eq!(ProgressBarSize::Md.as_str(), "md");
        assert_eq!(ProgressBarSize::Lg.as_str(), "lg");
    }

    #[test]
    fn phase_mappings_are_stable() {
        assert_eq!(
            ProgressBarPhase::Determinate.class_name(),
            "ui-progress-bar--state-determinate"
        );
        assert_eq!(
            ProgressBarPhase::Indeterminate.class_name(),
            "ui-progress-bar--state-indeterminate"
        );
        assert_eq!(ProgressBarPhase::Determinate.as_str(), "determinate");
        assert_eq!(ProgressBarPhase::Indeterminate.as_str(), "indeterminate");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-progress-bar  ".to_string())),
            Some("docs-progress-bar".to_string())
        );
    }

    #[test]
    fn resolve_aria_label_defaults_and_detects_custom_source() {
        assert_eq!(
            resolve_aria_label("\n\t".to_string()),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
        assert_eq!(
            resolve_aria_label("Progress".to_string()),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
        assert_eq!(
            resolve_aria_label("  Upload progress ".to_string()),
            ("Upload progress".to_string(), true)
        );
    }

    #[test]
    fn sanitize_numeric_inputs() {
        assert_eq!(sanitize_max(DEFAULT_MAX), DEFAULT_MAX);
        assert_eq!(sanitize_max(0.0), MIN_MAX);
        assert_eq!(sanitize_max(f64::NAN), MIN_MAX);

        assert_eq!(sanitize_value(Some(12.0), 100.0), Some(12.0));
        assert_eq!(sanitize_value(Some(-5.0), 100.0), Some(0.0));
        assert_eq!(sanitize_value(Some(120.0), 100.0), Some(100.0));
        assert_eq!(sanitize_value(Some(f64::NAN), 100.0), None);
    }

    #[test]
    fn resolve_state_tracks_phase_and_source_contracts() {
        let state = resolve_state(ProgressBarStateInput {
            variant: ProgressBarVariant::Accent,
            size: ProgressBarSize::Lg,
            value: Some(42.0),
            max: 100.0,
            indeterminate: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.variant_attr, "accent");
        assert_eq!(state.size_attr, "lg");
        assert_eq!(state.phase_attr, "determinate");
        assert_eq!(state.value, Some(42.0));
        assert_eq!(state.max, 100.0);
        assert!(state.is_determinate);
        assert!(!state.is_indeterminate);
        assert_eq!(state.label_source_class, "ui-progress-bar--label-custom");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn resolve_state_marks_indeterminate_when_value_missing() {
        let state = resolve_state(ProgressBarStateInput {
            variant: ProgressBarVariant::Default,
            size: ProgressBarSize::Md,
            value: None,
            max: 100.0,
            indeterminate: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });

        assert_eq!(state.phase, ProgressBarPhase::Indeterminate);
        assert!(!state.has_value);
        assert!(state.is_indeterminate);
        assert!(!state.is_determinate);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ProgressBarStateInput {
                variant: ProgressBarVariant::Danger,
                size: ProgressBarSize::Sm,
                value: Some(5.0),
                max: 10.0,
                indeterminate: false,
                has_custom_aria_label: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-progress-bar",
            "ui-progress-bar--variant-danger",
            "ui-progress-bar--size-sm",
            "ui-progress-bar--state-determinate",
            "ui-progress-bar--label-custom",
            "ui-progress-bar--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
