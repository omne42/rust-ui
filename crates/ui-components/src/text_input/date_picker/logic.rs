pub use ui_state_primitives::date_picker::{
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
mod tests {
    use super::*;

    #[test]
    fn state_primitives_are_reexported_from_ui_state_primitives() {
        assert_eq!(normalize_month(0), 1);
        assert_eq!(normalize_month(17), 12);
        assert_eq!(ui_state_primitives::date_picker::days_in_month(2024, 2), 29);
        assert_eq!(ui_state_primitives::date_picker::days_in_month(2023, 2), 28);
        assert_eq!(normalize_selected_day(Some(31), 2026, 4), None);
        assert_eq!(normalize_selected_day(Some(30), 2026, 4), Some(30));
        assert_eq!(
            normalize_aria_label(Some("  Ship date ".to_string())),
            ("Ship date".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (
                ui_state_primitives::date_picker::DEFAULT_ARIA_LABEL.into(),
                false
            )
        );
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-date-picker".to_string()),
            resolve_state(DatePickerStateInput {
                year: 2026,
                month: 1,
                selected_day: None,
                tone: DatePickerTone::Quiet,
                disabled: true,
                open: false,
                has_custom_placeholder: false,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_motion: true,
            }),
        );

        for token in [
            "ui-date-picker",
            "ui-date-picker--tone-quiet",
            "ui-date-picker--closed",
            "ui-date-picker--disabled",
            "ui-date-picker--empty",
            "ui-date-picker--custom-class",
            "ui-date-picker--custom-motion",
            "docs-date-picker",
        ] {
            assert!(class_name.contains(token), "class should include `{token}`");
        }
    }
}
