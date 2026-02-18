pub const DEFAULT_COLOR: &str = "#3b82f6";
pub const DEFAULT_ARIA_LABEL: &str = "Color loupe";
pub const DEFAULT_POSITION_PERCENT: f32 = 50.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorLoupeStateInput {
    pub open: bool,
    pub disabled: bool,
    pub has_color: bool,
    pub x_percent: f32,
    pub y_percent: f32,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorLoupeState {
    pub is_open: bool,
    pub is_disabled: bool,
    pub has_color: bool,
    pub x_percent: f32,
    pub y_percent: f32,
    pub x_bucket_class: &'static str,
    pub y_bucket_class: &'static str,
    pub x_bucket_attr: &'static str,
    pub y_bucket_attr: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn sanitize_percent(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 100.0)
    } else {
        DEFAULT_POSITION_PERCENT
    }
}

pub fn sanitize_color(color: Option<String>) -> Option<String> {
    crate::swatch::sanitize_color_value(normalize_optional_text(color))
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(aria_label) = normalize_optional_text(value) {
        return (aria_label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn position_bucket(value: f32) -> (&'static str, &'static str) {
    if value <= 33.333 {
        ("start", "ui-color-loupe--x-start")
    } else if value >= 66.667 {
        ("end", "ui-color-loupe--x-end")
    } else {
        ("center", "ui-color-loupe--x-center")
    }
}

pub fn vertical_bucket(value: f32) -> (&'static str, &'static str) {
    if value <= 33.333 {
        ("start", "ui-color-loupe--y-start")
    } else if value >= 66.667 {
        ("end", "ui-color-loupe--y-end")
    } else {
        ("center", "ui-color-loupe--y-center")
    }
}

pub fn resolve_state(input: ColorLoupeStateInput) -> ColorLoupeState {
    let x_percent = sanitize_percent(input.x_percent);
    let y_percent = sanitize_percent(input.y_percent);
    let (x_bucket_attr, x_bucket_class) = position_bucket(x_percent);
    let (y_bucket_attr, y_bucket_class) = vertical_bucket(y_percent);

    let is_open = input.open && !input.disabled;
    let data_state_attr = if input.disabled {
        "disabled"
    } else if is_open {
        "open"
    } else if input.has_color {
        "color"
    } else {
        "idle"
    };

    ColorLoupeState {
        is_open,
        is_disabled: input.disabled,
        has_color: input.has_color,
        x_percent,
        y_percent,
        x_bucket_class,
        y_bucket_class,
        x_bucket_attr,
        y_bucket_attr,
        data_state_attr,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_percent_clamps_and_falls_back_for_invalid_numbers() {
        assert_eq!(sanitize_percent(-1.0), 0.0);
        assert_eq!(sanitize_percent(38.5), 38.5);
        assert_eq!(sanitize_percent(101.0), 100.0);
        assert_eq!(sanitize_percent(f32::NAN), DEFAULT_POSITION_PERCENT);
    }

    #[test]
    fn sanitize_color_rejects_unsafe_values() {
        assert_eq!(
            sanitize_color(Some(" #09f ".to_string())),
            Some("#09f".to_string())
        );
        assert_eq!(
            sanitize_color(Some("javascript:alert(1)".to_string())),
            None
        );
    }

    #[test]
    fn resolve_state_tracks_flags_and_state_sources() {
        let state = resolve_state(ColorLoupeStateInput {
            open: true,
            disabled: false,
            has_color: true,
            x_percent: 22.0,
            y_percent: 88.0,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert!(state.is_open);
        assert_eq!(state.data_state_attr, "open");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.x_bucket_attr, "start");
        assert_eq!(state.y_bucket_attr, "end");
    }

    #[test]
    fn normalize_aria_label_uses_default_or_custom_values() {
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(Some("  Accent loupe  ".to_string())),
            ("Accent loupe".to_string(), true)
        );
    }
}
