pub use crate::button::normalize_optional_text;

pub const DEFAULT_LABEL: &str = "Hue";
pub const DEFAULT_ARIA_LABEL: &str = "Hue wheel";
pub const MIN_VALUE: f64 = 0.0;
pub const MAX_VALUE: f64 = 359.0;
pub const DEFAULT_STEP: f64 = 1.0;
pub const DEFAULT_PAGE_STEP: f64 = 15.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelStatus {
    Active,
    Disabled,
}

impl ColorWheelStatus {
    pub const fn from_disabled(disabled: bool) -> Self {
        if disabled {
            Self::Disabled
        } else {
            Self::Active
        }
    }

    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    pub const fn data_state_attr(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelValueLabelMode {
    Visible,
    Hidden,
}

impl ColorWheelValueLabelMode {
    pub const fn from_visible(visible: bool) -> Self {
        if visible { Self::Visible } else { Self::Hidden }
    }

    pub const fn is_visible(self) -> bool {
        matches!(self, Self::Visible)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelSource {
    Default,
    Custom,
}

impl ColorWheelSource {
    pub const fn from_custom(custom: bool) -> Self {
        if custom { Self::Custom } else { Self::Default }
    }

    pub const fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }

    pub const fn attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorWheelStateInput {
    pub status: ColorWheelStatus,
    pub value: f64,
    pub step: f64,
    pub value_label_mode: ColorWheelValueLabelMode,
    pub motion_source: ColorWheelSource,
    pub label_source: ColorWheelSource,
    pub aria_source: ColorWheelSource,
    pub class_source: ColorWheelSource,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorWheelState {
    pub is_disabled: bool,
    pub value: f64,
    pub step: f64,
    pub value_percent: f64,
    pub show_value_label: bool,
    pub data_state_attr: &'static str,
    pub motion_source_class: &'static str,
    pub motion_source_attr: &'static str,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.into(), false)
}

pub fn normalize_aria_label(value: Option<String>, label: &str) -> (String, bool) {
    if let Some(aria_label) = normalize_optional_text(value) {
        return (aria_label, true);
    }

    let label = label.trim();
    if !label.is_empty() {
        return (format!("{label} wheel"), false);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn sanitize_step(step: f64) -> f64 {
    if step.is_finite() && step > 0.0 {
        step.min(90.0)
    } else {
        DEFAULT_STEP
    }
}

pub fn normalize_angle(value: f64) -> f64 {
    if !value.is_finite() {
        return MIN_VALUE;
    }

    value.rem_euclid(360.0)
}

fn round_to_precision(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

pub fn sanitize_value(value: f64, step: f64) -> f64 {
    let step = sanitize_step(step);
    let normalized = normalize_angle(value);
    let snapped = (normalized / step).round() * step;
    let snapped = normalize_angle(snapped);

    round_to_precision(snapped).clamp(MIN_VALUE, MAX_VALUE)
}

pub fn resolve_default_value(default_value: Option<f64>, step: f64) -> f64 {
    default_value.map_or_else(
        || sanitize_value(MIN_VALUE, step),
        |value| sanitize_value(value, step),
    )
}

pub fn parse_value(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    (!trimmed.is_empty())
        .then(|| trimmed.parse::<f64>().ok())
        .flatten()
}

pub fn page_step(step: f64) -> f64 {
    sanitize_step(step).max(DEFAULT_PAGE_STEP)
}

pub fn move_value_by_delta(current: f64, delta: f64, step: f64) -> f64 {
    sanitize_value(current + delta, step)
}

pub fn resolve_percent(value: f64) -> f64 {
    let value = sanitize_value(value, DEFAULT_STEP);
    (value / 360.0 * 100.0).clamp(0.0, 100.0)
}

pub fn format_value_text(value: f64) -> String {
    let value = sanitize_value(value, DEFAULT_STEP).round() as i64;
    format!("{value}°")
}

pub fn resolve_state(input: ColorWheelStateInput) -> ColorWheelState {
    let step = sanitize_step(input.step);
    let value = sanitize_value(input.value, step);

    let (motion_source_class, motion_source_attr) = match input.motion_source {
        ColorWheelSource::Custom => ("ui-color-wheel--motion-custom", "custom"),
        ColorWheelSource::Default => ("ui-color-wheel--motion-default", "default"),
    };

    let (label_source_class, label_source_attr) = match input.label_source {
        ColorWheelSource::Custom => ("ui-color-wheel--label-custom", "custom"),
        ColorWheelSource::Default => ("ui-color-wheel--label-default", "default"),
    };

    ColorWheelState {
        is_disabled: input.status.is_disabled(),
        value,
        step,
        value_percent: resolve_percent(value),
        show_value_label: input.value_label_mode.is_visible(),
        data_state_attr: input.status.data_state_attr(),
        motion_source_class,
        motion_source_attr,
        label_source_class,
        label_source_attr,
        aria_source_attr: input.aria_source.attr(),
        class_source_attr: input.class_source.attr(),
        has_custom_class_name: input.class_source.is_custom(),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorWheelState) -> String {
    let mut classes = vec![
        "ui-color-wheel".to_string(),
        state.motion_source_class.into(),
        state.label_source_class.into(),
    ];

    if state.is_disabled {
        classes.push("ui-color-wheel--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-wheel--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn pointer_to_hue_angle(
    client_x: f64,
    client_y: f64,
    rect_left: f64,
    rect_top: f64,
    rect_width: f64,
    rect_height: f64,
) -> f64 {
    let center_x = rect_left + rect_width / 2.0;
    let center_y = rect_top + rect_height / 2.0;
    let dx = client_x - center_x;
    let dy = client_y - center_y;

    let radians = dy.atan2(dx);
    let degrees = radians.to_degrees();

    normalize_angle(degrees + 90.0)
}

#[cfg(test)]
#[path = "test/color_wheel.rs"]
mod tests;
