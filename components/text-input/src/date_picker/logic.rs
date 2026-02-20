pub use ui_logic_calendar::date_picker::{
    DEFAULT_ARIA_LABEL, DEFAULT_PLACEHOLDER, DatePickerIds, DatePickerState, DatePickerStateInput,
    DatePickerTone, normalize_aria_label, normalize_month, normalize_optional_text,
    normalize_placeholder, normalize_selected_day, resolve_ids, resolve_state,
    resolve_trigger_label,
};

pub fn compose_class_name(base_class_name: Option<String>, state: DatePickerState) -> String {
    let mut classes = vec!["ui-date-picker".to_string(), state.tone_class.into()];

    if state.is_open {
        classes.push("ui-date-picker--open".to_string());
    }
    if state.is_closed {
        classes.push("ui-date-picker--closed".to_string());
    }
    if state.is_disabled {
        classes.push("ui-date-picker--disabled".to_string());
    }
    if state.has_value {
        classes.push("ui-date-picker--has-value".to_string());
    }
    if state.is_empty {
        classes.push("ui-date-picker--empty".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-date-picker--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    if state.has_custom_motion {
        classes.push("ui-date-picker--custom-motion".to_string());
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/date_picker/logic.rs"]
mod tests;
