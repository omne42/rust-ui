pub use crate::button::normalize_optional_text;

pub const DEFAULT_LABEL: &str = "Color";
pub const DEFAULT_PLACEHOLDER: &str = "#RRGGBB";
pub const DEFAULT_ARIA_LABEL: &str = "Color value";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorFieldStateInput {
    pub disabled: bool,
    pub has_value: bool,
    pub has_valid_value: bool,
    pub has_preview: bool,
    pub has_custom_label: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorFieldVisualState {
    Disabled,
    Empty,
    Valid,
    Invalid,
}

impl ColorFieldVisualState {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Empty => "empty",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorFieldState {
    pub is_disabled: bool,
    pub has_value: bool,
    pub has_valid_value: bool,
    pub has_preview: bool,
    pub visual_state: ColorFieldVisualState,
    pub label_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorFieldDerivedStateInput {
    pub is_disabled: bool,
    pub is_preview_visible: bool,
    pub value: Option<String>,
    pub preview_color: Option<String>,
    pub has_custom_label: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.into(), false)
}

pub fn normalize_placeholder(value: Option<String>) -> (String, bool) {
    if let Some(placeholder) = normalize_optional_text(value) {
        return (placeholder, true);
    }

    (DEFAULT_PLACEHOLDER.into(), false)
}

pub fn normalize_aria_label(value: Option<String>, label: &str) -> (String, bool) {
    if let Some(aria_label) = normalize_optional_text(value) {
        return (aria_label, true);
    }

    if !label.trim().is_empty() {
        return (format!("{label} value"), false);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_color_value(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
}

pub fn sanitize_preview_color(value: Option<String>) -> Option<String> {
    crate::swatch::sanitize_color_value(normalize_color_value(value))
}

pub fn resolve_visual_state(input: ColorFieldStateInput) -> ColorFieldVisualState {
    if input.disabled {
        ColorFieldVisualState::Disabled
    } else if !input.has_value {
        ColorFieldVisualState::Empty
    } else if input.has_valid_value {
        ColorFieldVisualState::Valid
    } else {
        ColorFieldVisualState::Invalid
    }
}

pub fn resolve_state(input: ColorFieldStateInput) -> ColorFieldState {
    let visual_state = resolve_visual_state(input);

    ColorFieldState {
        is_disabled: input.disabled,
        has_value: input.has_value,
        has_valid_value: input.has_valid_value,
        has_preview: input.has_preview,
        visual_state,
        label_source_attr: if input.has_custom_label {
            "custom"
        } else {
            "default"
        },
        placeholder_source_attr: if input.has_custom_placeholder {
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

pub fn resolve_derived_state(input: ColorFieldDerivedStateInput) -> ColorFieldState {
    let has_value = input.value.is_some();
    let has_valid_value = input.preview_color.is_some();

    resolve_state(ColorFieldStateInput {
        disabled: input.is_disabled,
        has_value,
        has_valid_value,
        has_preview: input.is_preview_visible && has_valid_value,
        has_custom_label: input.has_custom_label,
        has_custom_placeholder: input.has_custom_placeholder,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    })
}

pub fn is_invalid_state(state: ColorFieldState) -> bool {
    matches!(state.visual_state, ColorFieldVisualState::Invalid)
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorFieldState) -> String {
    let mut classes = vec!["ui-color-field".to_string()];

    if state.is_disabled {
        classes.push("ui-color-field--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-field--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/color_field.rs"]
mod tests;
