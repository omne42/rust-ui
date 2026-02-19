use crate::color::wheel::{ColorWheelState, ColorWheelStateInput};

pub const DEFAULT_LABEL: &str = "Hue";
pub const DEFAULT_ARIA_LABEL: &str = "Hue wheel";
pub const MIN_VALUE: f64 = 0.0;
pub const MAX_VALUE: f64 = 359.0;
pub const DEFAULT_STEP: f64 = 1.0;
pub const DEFAULT_PAGE_STEP: f64 = 15.0;

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

    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-color-wheel--motion-custom", "custom")
    } else {
        ("ui-color-wheel--motion-default", "default")
    };

    let (label_source_class, label_source_attr) = if input.has_custom_label {
        ("ui-color-wheel--label-custom", "custom")
    } else {
        ("ui-color-wheel--label-default", "default")
    };

    ColorWheelState {
        is_disabled: input.disabled,
        value,
        step,
        value_percent: resolve_percent(value),
        show_value_label: input.show_value_label,
        data_state_attr: if input.disabled { "disabled" } else { "active" },
        motion_source_class,
        motion_source_attr,
        label_source_class,
        label_source_attr,
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

#[cfg(any(test, target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
pub fn hue_from_pointer_event(
    track: &leptos::web_sys::Element,
    ev: &leptos::ev::PointerEvent,
) -> Option<f64> {
    let rect = track.get_bounding_client_rect();

    Some(pointer_to_hue_angle(
        ev.client_x() as f64,
        ev.client_y() as f64,
        rect.left(),
        rect.top(),
        rect.width(),
        rect.height(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_helpers_use_defaults_and_trim_custom_values() {
        assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));
        assert_eq!(
            normalize_label(Some("  Accent hue  ".to_string())),
            ("Accent hue".to_string(), true)
        );

        assert_eq!(
            normalize_aria_label(None, "Hue"),
            ("Hue wheel".to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(Some("  Brand wheel  ".to_string()), "Hue"),
            ("Brand wheel".to_string(), true)
        );
    }

    #[test]
    fn sanitize_helpers_wrap_and_snap_values() {
        assert_eq!(sanitize_step(0.0), DEFAULT_STEP);
        assert_eq!(sanitize_step(120.0), 90.0);

        assert_eq!(normalize_angle(-1.0), 359.0);
        assert_eq!(normalize_angle(361.0), 1.0);

        assert_eq!(sanitize_value(370.0, 1.0), 10.0);
        assert_eq!(sanitize_value(-15.0, 1.0), 345.0);
        assert_eq!(sanitize_value(14.0, 5.0), 15.0);

        assert_eq!(parse_value(" 42.5 "), Some(42.5));
        assert_eq!(parse_value(""), None);
        assert_eq!(page_step(1.0), 15.0);
        assert_eq!(move_value_by_delta(355.0, 10.0, 1.0), 5.0);
    }

    #[test]
    fn pointer_conversion_and_percent_are_stable() {
        let top = pointer_to_hue_angle(50.0, 0.0, 0.0, 0.0, 100.0, 100.0);
        let right = pointer_to_hue_angle(100.0, 50.0, 0.0, 0.0, 100.0, 100.0);
        let bottom = pointer_to_hue_angle(50.0, 100.0, 0.0, 0.0, 100.0, 100.0);
        let left = pointer_to_hue_angle(0.0, 50.0, 0.0, 0.0, 100.0, 100.0);

        assert_eq!(top.round(), 0.0);
        assert_eq!(right.round(), 90.0);
        assert_eq!(bottom.round(), 180.0);
        assert_eq!(left.round(), 270.0);

        assert_eq!(resolve_percent(180.0), 50.0);
        assert_eq!(format_value_text(123.6), "124°");
    }

    #[test]
    fn resolve_state_and_class_name_track_markers() {
        let state = resolve_state(ColorWheelStateInput {
            disabled: false,
            value: 95.0,
            step: 1.0,
            show_value_label: true,
            has_custom_motion: true,
            has_custom_label: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.data_state_attr, "active");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");

        let class = compose_class_name(Some("docs-wheel".to_string()), state);
        assert!(class.contains("ui-color-wheel"));
        assert!(class.contains("ui-color-wheel--motion-custom"));
        assert!(class.contains("ui-color-wheel--custom-class"));
        assert!(class.contains("docs-wheel"));
    }
}
