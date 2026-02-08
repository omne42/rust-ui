use crate::color_area::{ColorAreaState, ColorAreaStateInput};

pub const DEFAULT_LABEL: &str = "Color area";
pub const DEFAULT_ARIA_LABEL: &str = "Color area";
pub const DEFAULT_STEP: f32 = 0.1;
pub const DEFAULT_GRID_SIZE: usize = 11;

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.to_string(), false)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn sanitize_step(step: f32) -> f32 {
    if !step.is_finite() {
        return DEFAULT_STEP;
    }

    step.clamp(0.01, 1.0)
}

pub fn sanitize_grid_size(size: usize) -> usize {
    size.clamp(3, 31)
}

pub fn clamp_value(value: (f32, f32)) -> (f32, f32) {
    let x = if value.0.is_finite() {
        value.0.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let y = if value.1.is_finite() {
        value.1.clamp(0.0, 1.0)
    } else {
        0.0
    };

    (x, y)
}

pub fn sanitize_preview_color(value: Option<String>) -> Option<String> {
    crate::color_swatch::sanitize_color_value(normalize_optional_text(value))
}

pub fn value_from_cell(col: usize, row: usize, grid_size: usize) -> (f32, f32) {
    let grid_size = sanitize_grid_size(grid_size);
    let max = (grid_size - 1) as f32;
    let col = col.min(grid_size - 1) as f32;
    let row = row.min(grid_size - 1) as f32;

    (col / max, 1.0 - (row / max))
}

pub fn move_value_by_delta(value: (f32, f32), dx: f32, dy: f32, step: f32) -> (f32, f32) {
    let step = sanitize_step(step);
    clamp_value((value.0 + dx * step, value.1 + dy * step))
}

pub fn parse_axis_percent(value: &str) -> Option<f32> {
    let parsed = value.trim().parse::<f32>().ok()?;
    Some(parsed.clamp(0.0, 100.0) / 100.0)
}

pub fn resolve_state(input: ColorAreaStateInput) -> ColorAreaState {
    let step = sanitize_step(input.step);
    let grid_size = sanitize_grid_size(input.grid_size);
    let value = clamp_value(input.value);

    let value_x_percent = (value.0 * 100.0).round() as u8;
    let value_y_percent = (value.1 * 100.0).round() as u8;

    let max_index = grid_size.saturating_sub(1) as f32;
    let selected_col = (value.0 * max_index).round() as usize;
    let selected_row = ((1.0 - value.1) * max_index).round() as usize;

    ColorAreaState {
        is_disabled: input.disabled,
        step,
        value_x: value.0,
        value_y: value.1,
        value_x_percent,
        value_y_percent,
        grid_size,
        selected_col,
        selected_row,
        data_state_attr: if input.disabled { "disabled" } else { "active" },
        has_preview_color: input.has_preview_color,
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

pub fn compose_class_name(base_class_name: Option<String>, state: ColorAreaState) -> String {
    let mut classes = vec!["ui-color-area".to_string()];

    if state.is_disabled {
        classes.push("ui-color-area--disabled".to_string());
    }

    if state.has_preview_color {
        classes.push("ui-color-area--with-preview".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-area--custom-class".to_string());
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
    fn normalize_and_sanitize_helpers_work() {
        assert_eq!(normalize_label(None), (DEFAULT_LABEL.to_string(), false));
        assert_eq!(
            normalize_label(Some("  Saturation/Lightness  ".to_string())),
            ("Saturation/Lightness".to_string(), true)
        );

        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(Some("  Fill area  ".to_string())),
            ("Fill area".to_string(), true)
        );

        assert_eq!(sanitize_step(0.0), 0.01);
        assert_eq!(sanitize_step(9.0), 1.0);
        assert_eq!(sanitize_grid_size(1), 3);
        assert_eq!(sanitize_grid_size(99), 31);
        assert_eq!(clamp_value((1.2, -0.2)), (1.0, 0.0));

        assert_eq!(
            sanitize_preview_color(Some("#09f".to_string())),
            Some("#09f".to_string())
        );
        assert_eq!(
            sanitize_preview_color(Some("javascript:alert(1)".to_string())),
            None
        );
    }

    #[test]
    fn cell_mapping_and_axis_parse_are_stable() {
        let (x, y) = value_from_cell(5, 5, 11);
        assert!((x - 0.5).abs() < 0.0001);
        assert!((y - 0.5).abs() < 0.0001);

        assert_eq!(parse_axis_percent("75"), Some(0.75));
        assert_eq!(parse_axis_percent("-5"), Some(0.0));
        assert_eq!(parse_axis_percent("foo"), None);

        let moved = move_value_by_delta((0.5, 0.5), 1.0, -1.0, 0.1);
        assert!((moved.0 - 0.6).abs() < 0.0001);
        assert!((moved.1 - 0.4).abs() < 0.0001);
    }

    #[test]
    fn resolve_state_and_class_name_track_markers() {
        let state = resolve_state(ColorAreaStateInput {
            disabled: false,
            step: 0.1,
            value: (0.35, 0.8),
            grid_size: 11,
            has_preview_color: true,
            has_custom_label: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.data_state_attr, "active");
        assert_eq!(state.value_x_percent, 35);
        assert_eq!(state.value_y_percent, 80);
        assert_eq!(state.selected_col, 4);
        assert_eq!(state.selected_row, 2);
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");

        let class_name = compose_class_name(Some("docs-color-area".to_string()), state);
        assert!(class_name.contains("ui-color-area"));
        assert!(class_name.contains("ui-color-area--with-preview"));
        assert!(class_name.contains("ui-color-area--custom-class"));
        assert!(class_name.contains("docs-color-area"));
    }
}
