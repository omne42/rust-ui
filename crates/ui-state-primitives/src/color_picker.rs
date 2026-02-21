pub use crate::button::normalize_optional_text;

pub const DEFAULT_LABEL: &str = "Color";
pub const DEFAULT_ARIA_LABEL: &str = "Color picker";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorPickerStateInput {
    pub disabled: bool,
    pub open: bool,
    pub has_selection: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub is_open_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorPickerState {
    pub is_disabled: bool,
    pub is_open: bool,
    pub has_selection: bool,
    pub selection_empty: bool,
    pub data_state_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.into(), false)
}

pub fn normalize_aria_label(value: Option<String>, label: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    let label = label.trim();
    if !label.is_empty() {
        return (format!("{label} picker"), false);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn sanitize_selected_color(value: Option<String>) -> Option<String> {
    crate::swatch::sanitize_color_value(normalize_optional_text(value))
}

pub fn resolve_state(input: ColorPickerStateInput) -> ColorPickerState {
    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.open {
        "open"
    } else if input.has_selection {
        "selected"
    } else {
        "empty"
    };

    ColorPickerState {
        is_disabled: input.disabled,
        is_open: input.open,
        has_selection: input.has_selection,
        selection_empty: !input.has_selection,
        data_state_attr,
        open_mode_attr: if input.is_open_controlled {
            "controlled"
        } else {
            "uncontrolled"
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

pub fn compose_class_name(base_class_name: Option<String>, state: ColorPickerState) -> String {
    let mut classes = vec!["ui-color-picker".to_string()];

    if state.is_disabled {
        classes.push("ui-color-picker--disabled".to_string());
    }

    if state.is_open {
        classes.push("ui-color-picker--open".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-picker--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/color_picker.rs"]
mod tests;
