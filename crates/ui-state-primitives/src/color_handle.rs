pub const DEFAULT_ARIA_LABEL: &str = "Color handle";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorHandleStateInput {
    pub is_disabled: bool,
    pub is_focused: bool,
    pub is_dragging: bool,
    pub is_loupe_visible: bool,
    pub has_color: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorHandleState {
    pub is_disabled: bool,
    pub is_focused: bool,
    pub is_dragging: bool,
    pub loupe_visible: bool,
    pub has_color: bool,
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

pub fn sanitize_color(value: Option<String>) -> Option<String> {
    crate::swatch::sanitize_color_value(normalize_optional_text(value))
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(aria_label) = normalize_optional_text(value) {
        return (aria_label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: ColorHandleStateInput) -> ColorHandleState {
    let data_state_attr = if input.is_disabled {
        "disabled"
    } else if input.is_dragging {
        "dragging"
    } else if input.is_focused {
        "focused"
    } else if input.has_color {
        "color"
    } else {
        "idle"
    };

    ColorHandleState {
        is_disabled: input.is_disabled,
        is_focused: input.is_focused,
        is_dragging: input.is_dragging,
        loupe_visible: !input.is_disabled && input.is_loupe_visible && input.is_dragging,
        has_color: input.has_color,
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
#[path = "test/color_handle.rs"]
mod tests;
