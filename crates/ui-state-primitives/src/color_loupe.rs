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
        (!trimmed.is_empty()).then(|| trimmed.into())
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

    (DEFAULT_ARIA_LABEL.into(), false)
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
#[path = "test/color_loupe.rs"]
mod tests;
