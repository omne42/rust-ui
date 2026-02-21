pub const DEFAULT_COLOR: &str = "#3b82f6";
pub const DEFAULT_ARIA_LABEL: &str = "Color thumb";
pub const DEFAULT_ARIA_VALUE_TEXT: &str = "No color selected";
pub const DEFAULT_POSITION_PERCENT: f32 = 50.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbInputSource {
    Default,
    External,
}

impl ColorThumbInputSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::External => "external",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbAriaValueTextSource {
    Default,
    Color,
    Custom,
}

impl ColorThumbAriaValueTextSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Color => "color",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbInteractionState {
    Idle,
    Focused,
    Dragging,
    Disabled,
}

impl ColorThumbInteractionState {
    pub fn from_flags(is_disabled: bool, is_focused: bool, is_dragging: bool) -> Self {
        if is_disabled {
            Self::Disabled
        } else if is_dragging {
            Self::Dragging
        } else if is_focused {
            Self::Focused
        } else {
            Self::Idle
        }
    }

    pub const fn data_state_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Focused => "focused",
            Self::Dragging => "dragging",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorThumbStateInput {
    pub interaction_state: ColorThumbInteractionState,
    pub show_loupe: bool,
    pub loupe_source: ColorThumbInputSource,
    pub has_color: bool,
    pub x_percent: f32,
    pub y_percent: f32,
    pub x_source: ColorThumbInputSource,
    pub y_source: ColorThumbInputSource,
    pub has_custom_aria_label: bool,
    pub aria_value_text_source: ColorThumbAriaValueTextSource,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorThumbState {
    pub is_disabled: bool,
    pub is_focused: bool,
    pub is_dragging: bool,
    pub loupe_visible: bool,
    pub has_color: bool,
    pub x_percent: f32,
    pub y_percent: f32,
    pub x_bucket_class: &'static str,
    pub y_bucket_class: &'static str,
    pub x_bucket_attr: &'static str,
    pub y_bucket_attr: &'static str,
    pub data_state_attr: &'static str,
    pub interaction_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub aria_value_text_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub loupe_source_attr: &'static str,
    pub x_source_attr: &'static str,
    pub y_source_attr: &'static str,
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

pub fn normalize_aria_value_text(
    value: Option<String>,
    fallback_color: Option<String>,
) -> (String, ColorThumbAriaValueTextSource) {
    if let Some(value_text) = normalize_optional_text(value) {
        return (value_text, ColorThumbAriaValueTextSource::Custom);
    }

    if let Some(color_text) = normalize_optional_text(fallback_color) {
        return (color_text, ColorThumbAriaValueTextSource::Color);
    }

    (
        DEFAULT_ARIA_VALUE_TEXT.into(),
        ColorThumbAriaValueTextSource::Default,
    )
}

pub fn position_bucket(value: f32) -> (&'static str, &'static str) {
    if value <= 33.333 {
        ("start", "ui-color-thumb--x-start")
    } else if value >= 66.667 {
        ("end", "ui-color-thumb--x-end")
    } else {
        ("center", "ui-color-thumb--x-center")
    }
}

pub fn vertical_bucket(value: f32) -> (&'static str, &'static str) {
    if value <= 33.333 {
        ("start", "ui-color-thumb--y-start")
    } else if value >= 66.667 {
        ("end", "ui-color-thumb--y-end")
    } else {
        ("center", "ui-color-thumb--y-center")
    }
}

pub fn resolve_state(input: ColorThumbStateInput) -> ColorThumbState {
    let is_disabled = matches!(
        input.interaction_state,
        ColorThumbInteractionState::Disabled
    );
    let is_focused = matches!(input.interaction_state, ColorThumbInteractionState::Focused);
    let is_dragging = matches!(
        input.interaction_state,
        ColorThumbInteractionState::Dragging
    );

    let x_percent = sanitize_percent(input.x_percent);
    let y_percent = sanitize_percent(input.y_percent);
    let (x_bucket_attr, x_bucket_class) = position_bucket(x_percent);
    let (y_bucket_attr, y_bucket_class) = vertical_bucket(y_percent);

    ColorThumbState {
        is_disabled,
        is_focused,
        is_dragging,
        loupe_visible: !is_disabled && input.show_loupe && is_dragging,
        has_color: input.has_color,
        x_percent,
        y_percent,
        x_bucket_class,
        y_bucket_class,
        x_bucket_attr,
        y_bucket_attr,
        data_state_attr: input.interaction_state.data_state_attr(),
        interaction_source_attr: "external",
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        aria_value_text_source_attr: input.aria_value_text_source.as_attr(),
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        loupe_source_attr: input.loupe_source.as_attr(),
        x_source_attr: input.x_source.as_attr(),
        y_source_attr: input.y_source.as_attr(),
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/color_thumb.rs"]
mod tests;
