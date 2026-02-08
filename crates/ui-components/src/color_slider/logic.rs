use crate::color_slider::{ColorSliderState, ColorSliderStateInput};

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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_label(value: Option<String>, channel: ColorSliderChannel) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (channel.default_label().to_string(), false)
}

pub fn normalize_aria_label(
    value: Option<String>,
    label: &str,
    channel: ColorSliderChannel,
) -> (String, bool) {
    if let Some(aria_label) = normalize_optional_text(value) {
        return (aria_label, true);
    }

    let label = label.trim();
    if !label.is_empty() {
        return (format!("{label} slider"), false);
    }

    (channel.default_aria_label().to_string(), false)
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

pub fn parse_value(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    (!trimmed.is_empty())
        .then(|| trimmed.parse::<f64>().ok())
        .flatten()
}

pub fn sanitize_track_color(value: Option<String>) -> Option<String> {
    crate::color_swatch::sanitize_color_value(normalize_optional_text(value))
}

pub fn compose_inline_style(track_start: Option<&str>, track_end: Option<&str>) -> Option<String> {
    let mut declarations = Vec::new();

    if let Some(track_start) = track_start {
        declarations.push(format!("--ui-color-slider-track-start: {track_start};"));
    }

    if let Some(track_end) = track_end {
        declarations.push(format!("--ui-color-slider-track-end: {track_end};"));
    }

    if declarations.is_empty() {
        None
    } else {
        Some(declarations.join(" "))
    }
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
        state.channel_class.to_string(),
        state.motion_source_class.to_string(),
        state.label_source_class.to_string(),
        state.track_source_class.to_string(),
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
mod tests {
    use super::*;

    #[test]
    fn channel_contracts_are_stable() {
        assert_eq!(
            ColorSliderChannel::Hue.class_name(),
            "ui-color-slider--channel-hue"
        );
        assert_eq!(ColorSliderChannel::Hue.as_attr(), "hue");
        assert_eq!(ColorSliderChannel::Hue.default_bounds(), (0.0, 360.0));
        assert_eq!(ColorSliderChannel::Hue.default_label(), "Hue");

        assert_eq!(ColorSliderChannel::Alpha.default_bounds(), (0.0, 100.0));
        assert_eq!(ColorSliderChannel::Red.default_bounds(), (0.0, 255.0));
    }

    #[test]
    fn normalize_helpers_keep_defaults_and_trim_values() {
        assert_eq!(
            normalize_label(None, ColorSliderChannel::Hue),
            ("Hue".to_string(), false)
        );
        assert_eq!(
            normalize_label(Some("  Tint  ".to_string()), ColorSliderChannel::Hue),
            ("Tint".to_string(), true)
        );

        assert_eq!(
            normalize_aria_label(None, "Hue", ColorSliderChannel::Hue),
            ("Hue slider".to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(Some("  Tone  ".to_string()), "Hue", ColorSliderChannel::Hue),
            ("Tone".to_string(), true)
        );

        assert_eq!(
            sanitize_track_color(Some(" #09f ".to_string())),
            Some("#09f".to_string())
        );
        assert_eq!(
            sanitize_track_color(Some("javascript:alert(1)".to_string())),
            None
        );
    }

    #[test]
    fn sanitizers_handle_invalid_bounds_step_and_value() {
        let channel = ColorSliderChannel::Hue;
        let (min, max) = sanitize_bounds(channel, 360.0, 0.0);
        assert_eq!((min, max), (0.0, 360.0));

        let step = sanitize_step(channel, f64::NAN, min, max);
        assert_eq!(step, 1.0);

        let value = sanitize_value(channel, 482.5, min, max, step);
        assert_eq!(value, 360.0);

        assert_eq!(resolve_percent(180.0, min, max), 50.0);
        assert_eq!(parse_value(" 42.5 "), Some(42.5));
        assert_eq!(parse_value(""), None);
    }

    #[test]
    fn inline_style_and_formatting_are_stable() {
        assert_eq!(
            compose_inline_style(Some("#000"), Some("#fff")),
            Some(
                "--ui-color-slider-track-start: #000; --ui-color-slider-track-end: #fff;"
                    .to_string()
            )
        );
        assert_eq!(compose_inline_style(None, None), None);

        assert_eq!(format_channel_value(ColorSliderChannel::Hue, 120.4), "120°");
        assert_eq!(format_channel_value(ColorSliderChannel::Alpha, 57.6), "58%");
        assert_eq!(format_channel_value(ColorSliderChannel::Red, 200.2), "200");
    }

    #[test]
    fn resolve_state_and_class_name_track_markers() {
        let state = resolve_state(ColorSliderStateInput {
            disabled: false,
            channel: ColorSliderChannel::Alpha,
            value: 45.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            show_value_label: true,
            has_custom_motion: true,
            has_custom_label: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_track: true,
        });

        assert_eq!(state.channel_attr, "alpha");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.track_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");

        let class = compose_class_name(Some("docs-custom".to_string()), state);
        assert!(class.contains("ui-color-slider"));
        assert!(class.contains("ui-color-slider--channel-alpha"));
        assert!(class.contains("ui-color-slider--motion-custom"));
        assert!(class.contains("ui-color-slider--track-custom"));
        assert!(class.contains("docs-custom"));
    }
}
