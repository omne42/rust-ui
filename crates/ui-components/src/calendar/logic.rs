pub use ui_state_primitives::calendar::{
    CalendarFirstWeekday, CalendarGridCell, CalendarState, CalendarStateInput, CalendarTone,
    DEFAULT_ARIA_LABEL, build_month_grid, month_title, normalize_aria_label, normalize_month,
    normalize_optional_text, normalize_selected_day, resolve_state, weekday_labels,
};

pub fn compose_class_name(base_class_name: Option<String>, state: CalendarState) -> String {
    let mut classes = vec![
        "ui-calendar".to_string(),
        state.tone_class.to_string(),
        state.first_weekday_class.to_string(),
    ];

    if state.show_outside_days {
        classes.push("ui-calendar--outside-days".to_string());
    }
    if state.has_selected_day {
        classes.push("ui-calendar--has-selection".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-calendar--custom-class".to_string());
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
    fn state_primitives_are_reexported_from_ui_state_primitives() {
        assert_eq!(normalize_month(0), 1);
        assert_eq!(
            normalize_aria_label(Some("  Calendar picker  ".to_string())),
            ("Calendar picker".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (
                ui_state_primitives::calendar::DEFAULT_ARIA_LABEL.to_string(),
                false
            )
        );
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-calendar".to_string()),
            resolve_state(CalendarStateInput {
                year: 2026,
                month: 1,
                tone: CalendarTone::Quiet,
                first_weekday: CalendarFirstWeekday::Sunday,
                show_outside_days: true,
                selected_day: None,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-calendar",
            "ui-calendar--tone-quiet",
            "ui-calendar--weekday-sunday",
            "ui-calendar--outside-days",
            "ui-calendar--custom-class",
            "docs-calendar",
        ] {
            assert!(class_name.contains(token), "class should include `{token}`");
        }
    }
}
