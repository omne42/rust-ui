use crate::swatch::sanitize_color_value;

pub use crate::button::normalize_optional_text;

pub const DEFAULT_ARIA_LABEL: &str = "Color slider";
pub const MIN_RANGE: f64 = 0.000_001;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorSliderChannel {
    #[default]
    Hue,
    Saturation,
    Lightness,
    Alpha,
    Red,
    Green,
    Blue,
}

impl ColorSliderChannel {
    pub fn class_name(self) -> &'static str {
        match self {
            ColorSliderChannel::Hue => "ui-color-slider--channel-hue",
            ColorSliderChannel::Saturation => "ui-color-slider--channel-saturation",
            ColorSliderChannel::Lightness => "ui-color-slider--channel-lightness",
            ColorSliderChannel::Alpha => "ui-color-slider--channel-alpha",
            ColorSliderChannel::Red => "ui-color-slider--channel-red",
            ColorSliderChannel::Green => "ui-color-slider--channel-green",
            ColorSliderChannel::Blue => "ui-color-slider--channel-blue",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ColorSliderChannel::Hue => "hue",
            ColorSliderChannel::Saturation => "saturation",
            ColorSliderChannel::Lightness => "lightness",
            ColorSliderChannel::Alpha => "alpha",
            ColorSliderChannel::Red => "red",
            ColorSliderChannel::Green => "green",
            ColorSliderChannel::Blue => "blue",
        }
    }

    pub fn default_label(self) -> &'static str {
        match self {
            ColorSliderChannel::Hue => "Hue",
            ColorSliderChannel::Saturation => "Saturation",
            ColorSliderChannel::Lightness => "Lightness",
            ColorSliderChannel::Alpha => "Alpha",
            ColorSliderChannel::Red => "Red",
            ColorSliderChannel::Green => "Green",
            ColorSliderChannel::Blue => "Blue",
        }
    }

    pub fn default_aria_label(self) -> &'static str {
        match self {
            ColorSliderChannel::Hue => "Hue slider",
            ColorSliderChannel::Saturation => "Saturation slider",
            ColorSliderChannel::Lightness => "Lightness slider",
            ColorSliderChannel::Alpha => "Alpha slider",
            ColorSliderChannel::Red => "Red slider",
            ColorSliderChannel::Green => "Green slider",
            ColorSliderChannel::Blue => "Blue slider",
        }
    }

    pub fn default_bounds(self) -> (f64, f64) {
        match self {
            ColorSliderChannel::Hue => (0.0, 360.0),
            ColorSliderChannel::Saturation
            | ColorSliderChannel::Lightness
            | ColorSliderChannel::Alpha => (0.0, 100.0),
            ColorSliderChannel::Red | ColorSliderChannel::Green | ColorSliderChannel::Blue => {
                (0.0, 255.0)
            }
        }
    }

    pub fn default_step(self) -> f64 {
        1.0
    }

    pub fn default_value(self) -> f64 {
        match self {
            ColorSliderChannel::Hue => 0.0,
            ColorSliderChannel::Saturation => 100.0,
            ColorSliderChannel::Lightness => 50.0,
            ColorSliderChannel::Alpha => 100.0,
            ColorSliderChannel::Red | ColorSliderChannel::Green | ColorSliderChannel::Blue => 255.0,
        }
    }

    pub fn uses_percent_value(self) -> bool {
        matches!(
            self,
            ColorSliderChannel::Saturation
                | ColorSliderChannel::Lightness
                | ColorSliderChannel::Alpha
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSliderStateInput {
    pub disabled: bool,
    pub channel: ColorSliderChannel,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub show_value_label: bool,
    pub has_custom_motion: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_track: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSliderState {
    pub is_disabled: bool,
    pub channel: ColorSliderChannel,
    pub channel_class: &'static str,
    pub channel_attr: &'static str,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
    pub value_percent: f64,
    pub show_value_label: bool,
    pub data_state_attr: &'static str,
    pub motion_source_class: &'static str,
    pub motion_source_attr: &'static str,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub track_source_class: &'static str,
    pub track_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_track: bool,
}

pub fn source_attr_from_presence(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn sanitize_bounds(channel: ColorSliderChannel, min: f64, max: f64) -> (f64, f64) {
    let (default_min, default_max) = channel.default_bounds();

    let mut lower = if min.is_finite() { min } else { default_min };
    let mut upper = if max.is_finite() { max } else { default_max };

    if lower > upper {
        std::mem::swap(&mut lower, &mut upper);
    }

    if (upper - lower).abs() < MIN_RANGE {
        (default_min, default_max)
    } else {
        (lower, upper)
    }
}

pub fn sanitize_step(channel: ColorSliderChannel, step: f64, min: f64, max: f64) -> f64 {
    let range = (max - min).abs().max(channel.default_step());

    if step.is_finite() && step > 0.0 {
        step.min(range)
    } else {
        channel.default_step().min(range)
    }
}

fn round_to_precision(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

pub fn sanitize_value(
    channel: ColorSliderChannel,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
) -> f64 {
    let fallback = channel.default_value().clamp(min, max);
    let value = if value.is_finite() { value } else { fallback };
    let clamped = value.clamp(min, max);

    let step = sanitize_step(channel, step, min, max);
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

pub fn sanitize_track_color(value: Option<String>) -> Option<String> {
    sanitize_color_value(normalize_optional_text(value))
}

pub fn format_channel_value(channel: ColorSliderChannel, value: f64) -> String {
    let rounded = if value.is_finite() {
        value.round()
    } else {
        0.0
    } as i64;

    if channel == ColorSliderChannel::Hue {
        return format!("{rounded}°");
    }

    if channel.uses_percent_value() {
        return format!("{rounded}%");
    }

    rounded.to_string()
}

pub fn resolve_state(input: ColorSliderStateInput) -> ColorSliderState {
    let (min, max) = sanitize_bounds(input.channel, input.min, input.max);
    let step = sanitize_step(input.channel, input.step, min, max);
    let value = sanitize_value(input.channel, input.value, min, max, step);
    let value_percent = resolve_percent(value, min, max);

    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-color-slider--motion-custom", "custom")
    } else {
        ("ui-color-slider--motion-default", "default")
    };

    let (label_source_class, label_source_attr) = if input.has_custom_label {
        ("ui-color-slider--label-custom", "custom")
    } else {
        ("ui-color-slider--label-default", "default")
    };

    let (track_source_class, track_source_attr) = if input.has_custom_track {
        ("ui-color-slider--track-custom", "custom")
    } else {
        ("ui-color-slider--track-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };

    ColorSliderState {
        is_disabled: input.disabled,
        channel: input.channel,
        channel_class: input.channel.class_name(),
        channel_attr: input.channel.as_attr(),
        min,
        max,
        step,
        value,
        value_percent,
        show_value_label: input.show_value_label,
        data_state_attr: if input.disabled { "disabled" } else { "active" },
        motion_source_class,
        motion_source_attr,
        label_source_class,
        label_source_attr,
        aria_source_attr,
        class_source_attr,
        track_source_class,
        track_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_track: input.has_custom_track,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorSliderState) -> String {
    let mut classes = vec![
        "ui-color-slider".to_string(),
        state.channel_class.into(),
        state.motion_source_class.into(),
        state.label_source_class.into(),
        state.track_source_class.into(),
    ];

    if state.is_disabled {
        classes.push("ui-color-slider--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-slider--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/color_slider.rs"]
mod tests;
