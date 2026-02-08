use crate::color_loupe::{ColorLoupeState, ColorLoupeStateInput};

pub const DEFAULT_COLOR: &str = "#3b82f6";
pub const DEFAULT_ARIA_LABEL: &str = "Color loupe";
pub const DEFAULT_POSITION_PERCENT: f32 = 50.0;

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
    crate::color_swatch::sanitize_color_value(normalize_optional_text(color))
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

pub fn compose_class_name(base_class_name: Option<String>, state: ColorLoupeState) -> String {
    let mut classes = vec![
        "ui-color-loupe".to_string(),
        state.x_bucket_class.to_string(),
        state.y_bucket_class.to_string(),
    ];

    if state.is_open {
        classes.push("ui-color-loupe--open".to_string());
    }

    if state.is_disabled {
        classes.push("ui-color-loupe--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-loupe--custom-class".to_string());
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
    fn resolve_state_and_class_name_track_flags_and_sources() {
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

        let class_name = compose_class_name(Some("docs-color-loupe".to_string()), state);
        assert!(class_name.contains("ui-color-loupe"));
        assert!(class_name.contains("ui-color-loupe--open"));
        assert!(class_name.contains("ui-color-loupe--custom-class"));
        assert!(class_name.contains("docs-color-loupe"));
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
