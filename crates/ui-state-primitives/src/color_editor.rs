use crate::swatch::sanitize_color_value;

pub const DEFAULT_LABEL: &str = "Color editor";
pub const DEFAULT_ARIA_LABEL: &str = "Color editor";
pub const DEFAULT_HUE: f64 = 220.0;
pub const DEFAULT_ALPHA: f64 = 100.0;
pub const DEFAULT_AREA: (f32, f32) = (0.75, 0.75);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorEditorFormat {
    #[default]
    Hex,
    Rgb,
    Hsl,
    Hsb,
}

impl ColorEditorFormat {
    pub fn as_attr(self) -> &'static str {
        match self {
            ColorEditorFormat::Hex => "hex",
            ColorEditorFormat::Rgb => "rgb",
            ColorEditorFormat::Hsl => "hsl",
            ColorEditorFormat::Hsb => "hsb",
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            ColorEditorFormat::Hex => "ui-color-editor--format-hex",
            ColorEditorFormat::Rgb => "ui-color-editor--format-rgb",
            ColorEditorFormat::Hsl => "ui-color-editor--format-hsl",
            ColorEditorFormat::Hsb => "ui-color-editor--format-hsb",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ColorEditorFormat::Hex => "HEX",
            ColorEditorFormat::Rgb => "RGB",
            ColorEditorFormat::Hsl => "HSL",
            ColorEditorFormat::Hsb => "HSB",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorEditorStateInput {
    pub disabled: bool,
    pub hide_alpha_channel: bool,
    pub format: ColorEditorFormat,
    pub has_selection: bool,
    pub has_custom_motion: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorEditorState {
    pub is_disabled: bool,
    pub hide_alpha_channel: bool,
    pub format: ColorEditorFormat,
    pub format_class: &'static str,
    pub data_state_attr: &'static str,
    pub format_attr: &'static str,
    pub alpha_visibility_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
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
        return (label.into(), false);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn sanitize_color(value: Option<String>) -> Option<String> {
    sanitize_color_value(normalize_optional_text(value))
}

pub fn sanitize_hue(value: f64) -> f64 {
    if value.is_finite() {
        value.rem_euclid(360.0)
    } else {
        DEFAULT_HUE
    }
}

pub fn sanitize_alpha(value: f64) -> f64 {
    if value.is_finite() {
        value.clamp(0.0, 100.0)
    } else {
        DEFAULT_ALPHA
    }
}

pub fn sanitize_area(value: (f32, f32)) -> (f32, f32) {
    let x = if value.0.is_finite() {
        value.0.clamp(0.0, 1.0)
    } else {
        DEFAULT_AREA.0
    };

    let y = if value.1.is_finite() {
        value.1.clamp(0.0, 1.0)
    } else {
        DEFAULT_AREA.1
    };

    (x, y)
}

pub fn hsb_to_rgb(hue: f64, saturation: f64, brightness: f64) -> (u8, u8, u8) {
    let hue = sanitize_hue(hue);
    let saturation = saturation.clamp(0.0, 100.0) / 100.0;
    let brightness = brightness.clamp(0.0, 100.0) / 100.0;

    if saturation <= f64::EPSILON {
        let gray = (brightness * 255.0).round() as u8;
        return (gray, gray, gray);
    }

    let sector = hue / 60.0;
    let sector_floor = sector.floor();
    let fraction = sector - sector_floor;

    let p = brightness * (1.0 - saturation);
    let q = brightness * (1.0 - saturation * fraction);
    let t = brightness * (1.0 - saturation * (1.0 - fraction));

    let (red, green, blue) = match sector_floor as i32 {
        0 => (brightness, t, p),
        1 => (q, brightness, p),
        2 => (p, brightness, t),
        3 => (p, q, brightness),
        4 => (t, p, brightness),
        _ => (brightness, p, q),
    };

    (
        (red * 255.0).round() as u8,
        (green * 255.0).round() as u8,
        (blue * 255.0).round() as u8,
    )
}

pub fn hsb_to_hsl(hue: f64, saturation: f64, brightness: f64) -> (f64, f64, f64) {
    let hue = sanitize_hue(hue);
    let saturation = saturation.clamp(0.0, 100.0) / 100.0;
    let brightness = brightness.clamp(0.0, 100.0) / 100.0;

    let lightness = brightness * (1.0 - saturation / 2.0);
    let hsl_saturation = if lightness <= f64::EPSILON || (1.0 - lightness) <= f64::EPSILON {
        0.0
    } else {
        (brightness - lightness) / lightness.min(1.0 - lightness)
    };

    (
        hue,
        (hsl_saturation.clamp(0.0, 1.0) * 100.0).clamp(0.0, 100.0),
        (lightness * 100.0).clamp(0.0, 100.0),
    )
}

pub fn compose_color_from_hsb(
    hue: f64,
    saturation: f64,
    brightness: f64,
    alpha: f64,
    hide_alpha_channel: bool,
) -> String {
    let (red, green, blue) = hsb_to_rgb(hue, saturation, brightness);
    let alpha = sanitize_alpha(alpha);

    if hide_alpha_channel || alpha >= 99.999 {
        return format!("#{red:02x}{green:02x}{blue:02x}");
    }

    let alpha_ratio = (alpha / 100.0).clamp(0.0, 1.0);
    format!("rgba({red}, {green}, {blue}, {alpha_ratio:.3})")
}

pub fn format_channel_preview(
    format: ColorEditorFormat,
    hue: f64,
    saturation: f64,
    brightness: f64,
    alpha: f64,
    hide_alpha_channel: bool,
) -> Vec<(String, String)> {
    let mut rows = match format {
        ColorEditorFormat::Hex => {
            let (red, green, blue) = hsb_to_rgb(hue, saturation, brightness);
            vec![(
                "HEX".to_string(),
                format!("#{red:02x}{green:02x}{blue:02x}"),
            )]
        }
        ColorEditorFormat::Rgb => {
            let (red, green, blue) = hsb_to_rgb(hue, saturation, brightness);
            vec![
                ("R".to_string(), red.to_string()),
                ("G".to_string(), green.to_string()),
                ("B".to_string(), blue.to_string()),
            ]
        }
        ColorEditorFormat::Hsl => {
            let (hue, saturation, lightness) = hsb_to_hsl(hue, saturation, brightness);
            vec![
                ("H".to_string(), format!("{:.0}°", hue.round())),
                ("S".to_string(), format!("{:.0}%", saturation.round())),
                ("L".to_string(), format!("{:.0}%", lightness.round())),
            ]
        }
        ColorEditorFormat::Hsb => vec![
            (
                "H".to_string(),
                format!("{:.0}°", sanitize_hue(hue).round()),
            ),
            (
                "S".to_string(),
                format!("{:.0}%", saturation.clamp(0.0, 100.0).round()),
            ),
            (
                "B".to_string(),
                format!("{:.0}%", brightness.clamp(0.0, 100.0).round()),
            ),
        ],
    };

    if !hide_alpha_channel {
        rows.push((
            "A".to_string(),
            format!("{:.0}%", sanitize_alpha(alpha).round()),
        ));
    }

    rows
}

pub fn resolve_state(input: ColorEditorStateInput) -> ColorEditorState {
    ColorEditorState {
        is_disabled: input.disabled,
        hide_alpha_channel: input.hide_alpha_channel,
        format: input.format,
        format_class: input.format.class_name(),
        data_state_attr: if input.disabled {
            "disabled"
        } else if input.has_selection {
            "ready"
        } else {
            "empty"
        },
        format_attr: input.format.as_attr(),
        alpha_visibility_attr: if input.hide_alpha_channel {
            "hidden"
        } else {
            "visible"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        label_source_attr: if input.has_custom_label {
            "custom"
        } else {
            "default"
        },
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
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorEditorState) -> String {
    let mut classes = vec!["ui-color-editor".to_string(), state.format_class.into()];

    if state.is_disabled {
        classes.push("ui-color-editor--disabled".to_string());
    }

    if state.hide_alpha_channel {
        classes.push("ui-color-editor--alpha-hidden".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-editor--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/color_editor.rs"]
mod tests;
