pub use ui_logic_calendar::calendar::{
    CalendarFirstWeekday, CalendarGridCell, CalendarState, CalendarStateInput, CalendarTone,
    DEFAULT_ARIA_LABEL, build_month_grid, month_title, normalize_aria_label, normalize_month,
    normalize_optional_text, normalize_selected_day, resolve_state, weekday_labels,
};

pub fn compose_class_name(base_class_name: Option<String>, state: CalendarState) -> String {
    let mut classes = vec![
        "ui-calendar".to_string(),
        state.tone_class.into(),
        state.first_weekday_class.into(),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarAgentAction {
    Idle,
    SelectDay,
}

impl CalendarAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::SelectDay => "select-day",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarAgentState {
    Default,
    Selected,
}

impl CalendarAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Selected => "selected",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarAgentSource {
    ImplicitDefault,
    PropsSelectedDay,
}

impl CalendarAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::ImplicitDefault => "implicit-default",
            Self::PropsSelectedDay => "props-selected-day",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarAgentStreamSupport {
    Unsupported,
}

impl CalendarAgentStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarAgentStreamFallback {
    Snapshot,
}

impl CalendarAgentStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarAgentOutputStatus {
    Verified,
}

impl CalendarAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarAgentContract {
    pub schema_attr: &'static str,
    pub intent_attr: &'static str,
    pub action: CalendarAgentAction,
    pub state: CalendarAgentState,
    pub source: CalendarAgentSource,
    pub stream_support: CalendarAgentStreamSupport,
    pub stream_fallback: CalendarAgentStreamFallback,
    pub output_status: CalendarAgentOutputStatus,
}

pub fn resolve_agent_contract(state: CalendarState) -> CalendarAgentContract {
    let has_selected_day = state.has_selected_day;

    CalendarAgentContract {
        schema_attr: "ui.calendar",
        intent_attr: "date-selection",
        action: if has_selected_day {
            CalendarAgentAction::SelectDay
        } else {
            CalendarAgentAction::Idle
        },
        state: if has_selected_day {
            CalendarAgentState::Selected
        } else {
            CalendarAgentState::Default
        },
        source: if has_selected_day {
            CalendarAgentSource::PropsSelectedDay
        } else {
            CalendarAgentSource::ImplicitDefault
        },
        stream_support: CalendarAgentStreamSupport::Unsupported,
        stream_fallback: CalendarAgentStreamFallback::Snapshot,
        output_status: CalendarAgentOutputStatus::Verified,
    }
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
                ui_logic_calendar::calendar::DEFAULT_ARIA_LABEL.into(),
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

    #[test]
    fn resolve_agent_contract_uses_selection_state_as_machine_contract() {
        let selected = resolve_agent_contract(resolve_state(CalendarStateInput {
            year: 2026,
            month: 1,
            tone: CalendarTone::Default,
            first_weekday: CalendarFirstWeekday::Sunday,
            show_outside_days: true,
            selected_day: Some(6),
            has_custom_aria_label: false,
            has_custom_class_name: false,
        }));

        assert_eq!(selected.schema_attr, "ui.calendar");
        assert_eq!(selected.intent_attr, "date-selection");
        assert_eq!(selected.action.as_attr(), "select-day");
        assert_eq!(selected.state.as_attr(), "selected");
        assert_eq!(selected.source.as_attr(), "props-selected-day");
        assert_eq!(selected.stream_support.as_attr(), "unsupported");
        assert_eq!(selected.stream_fallback.as_attr(), "snapshot");
        assert_eq!(selected.output_status.as_attr(), "verified");
    }
}
