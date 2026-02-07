pub const DEFAULT_LABEL: &str = "Slider";
pub const DEFAULT_MIN: f64 = 0.0;
pub const DEFAULT_MAX: f64 = 100.0;
pub const DEFAULT_STEP: f64 = 1.0;
pub const MIN_RANGE: f64 = 0.000_001;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderPhase {
    Enabled,
    Disabled,
}

impl SliderPhase {
    pub fn class_name(self) -> &'static str {
        match self {
            SliderPhase::Enabled => "ui-slider--state-enabled",
            SliderPhase::Disabled => "ui-slider--state-disabled",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SliderPhase::Enabled => "enabled",
            SliderPhase::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderStateInput {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
    pub has_custom_label: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderState {
    pub phase: SliderPhase,
    pub phase_class: &'static str,
    pub phase_attr: &'static str,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
    pub value_percent: f64,
    pub is_enabled: bool,
    pub is_disabled: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
    pub has_custom_label: bool,
    pub motion_source_class: &'static str,
    pub motion_source_attr: &'static str,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_label(value: String) -> (String, bool) {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return (DEFAULT_LABEL.to_string(), false);
    }

    let label = trimmed.to_string();
    let has_custom_label = label != DEFAULT_LABEL;

    (label, has_custom_label)
}

pub fn sanitize_bounds(min: f64, max: f64) -> (f64, f64) {
    let mut lower = if min.is_finite() { min } else { DEFAULT_MIN };
    let mut upper = if max.is_finite() { max } else { DEFAULT_MAX };

    if lower > upper {
        std::mem::swap(&mut lower, &mut upper);
    }

    if (upper - lower).abs() < MIN_RANGE {
        (DEFAULT_MIN, DEFAULT_MAX)
    } else {
        (lower, upper)
    }
}

pub fn sanitize_step(step: f64, min: f64, max: f64) -> f64 {
    let range = (max - min).abs().max(DEFAULT_STEP);

    if step.is_finite() && step > 0.0 {
        step.min(range)
    } else {
        DEFAULT_STEP.min(range)
    }
}

pub fn parse_value(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    (!trimmed.is_empty())
        .then(|| trimmed.parse::<f64>().ok())
        .flatten()
}

fn round_to_precision(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

pub fn sanitize_value(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let fallback = min;
    let value = if value.is_finite() { value } else { fallback };
    let clamped = value.clamp(min, max);

    let step = sanitize_step(step, min, max);
    let snapped = min + ((clamped - min) / step).round() * step;

    round_to_precision(snapped).clamp(min, max)
}

pub fn resolve_percent(value: f64, min: f64, max: f64) -> f64 {
    let range = (max - min).abs().max(MIN_RANGE);
    let percent = ((value - min) / range) * 100.0;

    if percent.is_finite() {
        percent.clamp(0.0, 100.0)
    } else {
        0.0
    }
}

pub fn resolve_state(input: SliderStateInput) -> SliderState {
    let (min, max) = sanitize_bounds(input.min, input.max);
    let step = sanitize_step(input.step, min, max);
    let value = sanitize_value(input.value, min, max, step);
    let value_percent = resolve_percent(value, min, max);

    let phase = if input.disabled {
        SliderPhase::Disabled
    } else {
        SliderPhase::Enabled
    };

    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-slider--motion-custom", "custom")
    } else {
        ("ui-slider--motion-default", "default")
    };

    let (label_source_class, label_source_attr) = if input.has_custom_label {
        ("ui-slider--label-custom", "custom")
    } else {
        ("ui-slider--label-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    SliderState {
        phase,
        phase_class: phase.class_name(),
        phase_attr: phase.as_str(),
        min,
        max,
        step,
        value,
        value_percent,
        is_enabled: !input.disabled,
        is_disabled: input.disabled,
        has_custom_motion: input.has_custom_motion,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_label: input.has_custom_label,
        motion_source_class,
        motion_source_attr,
        label_source_class,
        label_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SliderState) -> String {
    let mut classes = vec![
        "ui-slider".to_string(),
        state.phase_class.to_string(),
        state.motion_source_class.to_string(),
        state.label_source_class.to_string(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-slider--custom-class".to_string());
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
    fn phase_contract_is_stable() {
        assert_eq!(
            SliderPhase::Enabled.class_name(),
            "ui-slider--state-enabled"
        );
        assert_eq!(SliderPhase::Enabled.as_str(), "enabled");
        assert_eq!(
            SliderPhase::Disabled.class_name(),
            "ui-slider--state-disabled"
        );
        assert_eq!(SliderPhase::Disabled.as_str(), "disabled");
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-slider ".to_string())),
            Some("docs-slider".to_string())
        );
    }

    #[test]
    fn resolve_label_falls_back_to_default_for_empty_text() {
        assert_eq!(
            resolve_label("  ".to_string()),
            (DEFAULT_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_label(" Volume ".to_string()),
            ("Volume".to_string(), true)
        );
    }

    #[test]
    fn sanitize_bounds_swaps_or_falls_back_for_invalid_range() {
        assert_eq!(sanitize_bounds(10.0, 0.0), (0.0, 10.0));
        assert_eq!(sanitize_bounds(f64::NAN, f64::INFINITY), (0.0, 100.0));
        assert_eq!(sanitize_bounds(5.0, 5.0), (0.0, 100.0));
    }

    #[test]
    fn sanitize_step_and_value_align_to_range() {
        assert_eq!(sanitize_step(0.0, 0.0, 10.0), 1.0);
        assert_eq!(sanitize_step(200.0, 0.0, 10.0), 10.0);
        assert_eq!(sanitize_value(11.2, 0.0, 10.0, 0.5), 10.0);
        assert_eq!(sanitize_value(0.24, 0.0, 10.0, 0.5), 0.0);
        assert_eq!(sanitize_value(0.26, 0.0, 10.0, 0.5), 0.5);
        assert_eq!(sanitize_value(f64::NAN, 0.0, 10.0, 1.0), 0.0);
    }

    #[test]
    fn parse_value_reads_trimmed_numbers() {
        assert_eq!(parse_value(" 42.5 "), Some(42.5));
        assert_eq!(parse_value(""), None);
        assert_eq!(parse_value("not-a-number"), None);
    }

    #[test]
    fn resolve_percent_is_clamped() {
        assert_eq!(resolve_percent(50.0, 0.0, 100.0), 50.0);
        assert_eq!(resolve_percent(-1.0, 0.0, 100.0), 0.0);
        assert_eq!(resolve_percent(101.0, 0.0, 100.0), 100.0);
    }

    #[test]
    fn resolve_state_tracks_source_markers() {
        let state = resolve_state(SliderStateInput {
            value: 88.0,
            min: 0.0,
            max: 120.0,
            step: 2.0,
            disabled: true,
            has_custom_motion: true,
            has_custom_class_name: true,
            has_custom_label: true,
        });

        assert_eq!(state.phase_class, "ui-slider--state-disabled");
        assert_eq!(state.phase_attr, "disabled");
        assert!(state.is_disabled);
        assert!(!state.is_enabled);
        assert_eq!(state.motion_source_class, "ui-slider--motion-custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.label_source_class, "ui-slider--label-custom");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(SliderStateInput {
            value: 45.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            has_custom_motion: false,
            has_custom_class_name: true,
            has_custom_label: false,
        });
        let class_name = compose_class_name(Some("docs-slider".to_string()), state);

        for token in [
            "ui-slider",
            "ui-slider--state-enabled",
            "ui-slider--motion-default",
            "ui-slider--label-default",
            "ui-slider--custom-class",
            "docs-slider",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
