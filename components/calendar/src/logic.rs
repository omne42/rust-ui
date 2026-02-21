pub use ui_state_primitives::calendar::{
    CalendarFirstWeekday, CalendarGridCell, CalendarSelectedDayMode, CalendarSelectedDaySource,
    CalendarState, CalendarStateInput, CalendarTone, DEFAULT_ARIA_LABEL, build_month_grid,
    month_title, normalize_aria_label, normalize_is_show_outside_days, normalize_month,
    normalize_optional_text, normalize_selected_day_axis, resolve_effective_selected_day,
    resolve_selected_day_press_update, resolve_state, weekday_labels,
};

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
