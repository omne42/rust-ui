use std::borrow::Cow;

pub use ui_state_primitives::meter::{
    DEFAULT_ARIA_LABEL, MeterPhase, MeterRange, MeterSize, MeterState, MeterStateInput,
    MeterVariant, clamp_to_range, compose_class_name, normalize_optional_text, normalize_progress,
    resolve_phase, resolve_state, resolve_value_label,
};

pub const DEFAULT_MIN: f64 = 0.0;
pub const DEFAULT_MAX: f64 = 100.0;
pub const DEFAULT_SHOW_VALUE_LABEL: bool = true;

#[derive(Debug, Default)]
pub struct MeterInputNormalizationInput {
    pub label: Option<String>,
    pub aria_label: Option<String>,
    pub default_aria_label: Option<Cow<'static, str>>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub is_value_label_visible: Option<bool>,
    pub show_value_label: Option<bool>,
    pub value_label: Option<String>,
    pub class_name: Option<String>,
}

#[derive(Debug)]
pub struct MeterInputNormalization {
    pub label: Option<String>,
    pub aria_label: Cow<'static, str>,
    pub has_custom_aria_label: bool,
    pub range: MeterRange,
    pub is_value_label_visible: bool,
    pub value_label: Option<String>,
    pub has_custom_value_label: bool,
    pub class_name: Option<String>,
    pub has_custom_class_name: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeterRenderStateInput {
    pub value: Option<f64>,
    pub range: MeterRange,
    pub is_value_label_visible: bool,
    pub value_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeterRenderState {
    pub clamped_value: Option<f64>,
    pub normalized_progress: Option<f64>,
    pub phase: MeterPhase,
    pub aria_value_now: Option<String>,
    pub value_label_text: Option<String>,
}

#[derive(Clone, Debug)]
pub struct MeterStrings {
    pub aria_label: Cow<'static, str>,
}

impl Default for MeterStrings {
    fn default() -> Self {
        Self {
            aria_label: Cow::Borrowed(DEFAULT_ARIA_LABEL),
        }
    }
}

fn normalize_optional_cow(value: Option<Cow<'static, str>>) -> Option<Cow<'static, str>> {
    value.and_then(|value| match value {
        Cow::Borrowed(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else if trimmed.len() == value.len() {
                Some(Cow::Borrowed(value))
            } else {
                Some(Cow::Owned(trimmed.to_owned()))
            }
        }
        Cow::Owned(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else if trimmed.len() == value.len() {
                Some(Cow::Owned(value))
            } else {
                Some(Cow::Owned(trimmed.to_owned()))
            }
        }
    })
}

pub fn resolve_aria_label_with_fallback(
    aria_label: Option<String>,
    label: Option<String>,
    default_aria_label: Option<Cow<'static, str>>,
) -> (Cow<'static, str>, bool) {
    if let Some(label) = normalize_optional_text(aria_label) {
        return (Cow::Owned(label), true);
    }

    if let Some(label) = normalize_optional_text(label) {
        return (Cow::Owned(label), true);
    }

    let fallback =
        normalize_optional_cow(default_aria_label).unwrap_or(Cow::Borrowed(DEFAULT_ARIA_LABEL));
    (fallback, false)
}

#[cfg(test)]
pub fn resolve_aria_label(
    aria_label: Option<String>,
    label: Option<String>,
) -> (Cow<'static, str>, bool) {
    if let Some(explicit) = normalize_optional_text(aria_label.clone())
        && explicit.eq_ignore_ascii_case(DEFAULT_ARIA_LABEL)
    {
        // Explicit default text should stay on the default path, rather than
        // silently switching to label-derived custom source.
        return (Cow::Borrowed(DEFAULT_ARIA_LABEL), false);
    }
    let aria_label = normalize_optional_text(aria_label)
        .filter(|value| !value.eq_ignore_ascii_case(DEFAULT_ARIA_LABEL));
    resolve_aria_label_with_fallback(aria_label, label, None)
}

/*
Meter closed-set state-source marker contract (consumed from primitives):
("ui-meter--label-custom", "custom")
("ui-meter--label-default", "default")
("ui-meter--value-label-custom", "custom")
("ui-meter--value-label-auto", "auto")
("ui-meter--motion-custom", "custom")
("ui-meter--motion-default", "default")
let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
*/

pub fn normalize_inputs(input: MeterInputNormalizationInput) -> MeterInputNormalization {
    let class_name = normalize_optional_text(input.class_name);
    let label = normalize_optional_text(input.label);
    let (aria_label, has_custom_aria_label) =
        resolve_aria_label_with_fallback(input.aria_label, label.clone(), input.default_aria_label);
    let (value_label, has_custom_value_label) = resolve_value_label(input.value_label);

    let range = MeterRange::sanitized(
        input.min.unwrap_or(DEFAULT_MIN),
        input.max.unwrap_or(DEFAULT_MAX),
    );
    let is_value_label_visible = input
        .is_value_label_visible
        .unwrap_or(input.show_value_label.unwrap_or(DEFAULT_SHOW_VALUE_LABEL));

    MeterInputNormalization {
        label,
        aria_label,
        has_custom_aria_label,
        range,
        is_value_label_visible,
        value_label,
        has_custom_value_label,
        has_custom_class_name: class_name.is_some(),
        class_name,
    }
}

pub fn derive_render_state(input: MeterRenderStateInput) -> MeterRenderState {
    let clamped_value = input.value.map(|value| clamp_to_range(value, input.range));
    let normalized_progress = clamped_value.map(|value| normalize_progress(value, input.range));
    let phase = resolve_phase(normalized_progress.is_none());
    let aria_value_now = if phase == MeterPhase::Indeterminate {
        None
    } else {
        clamped_value.map(|value| value.to_string())
    };
    let value_label_text = if !input.is_value_label_visible {
        None
    } else if let Some(value_label) = input.value_label {
        Some(value_label)
    } else {
        normalized_progress.map(|progress| format!("{:.0}%", progress * 100.0))
    };

    MeterRenderState {
        clamped_value,
        normalized_progress,
        phase,
        aria_value_now,
        value_label_text,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
