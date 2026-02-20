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
#[path = "../test/logic.rs"]
mod tests;
