pub type CalendarFirstWeekday = ui_state_primitives::calendar::CalendarFirstWeekday;
pub type CalendarTone = ui_state_primitives::calendar::CalendarTone;
pub type CalendarGridCell = ui_state_primitives::calendar::CalendarGridCell;
pub type CalendarSelectedDayMode = ui_state_primitives::calendar::CalendarSelectedDayMode;
pub type CalendarSelectedDaySource = ui_state_primitives::calendar::CalendarSelectedDaySource;

pub struct CalendarMotion {
    pub enabled: bool,
    pub duration_ms: f64,
    pub spring: ui_motion::spring::SpringConfig,
}

pub enum CalendarAgentAction {
    Idle,
    SelectDay,
}

pub enum CalendarAgentState {
    Default,
    Selected,
}

pub enum CalendarAgentSource {
    ImplicitDefault,
    PropsSelectedDay,
}

pub enum CalendarAgentStreamSupport {
    Unsupported,
}

pub enum CalendarAgentStreamFallback {
    Snapshot,
}

pub enum CalendarAgentOutputStatus {
    Verified,
}

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

pub fn Calendar(
    year: i32,
    month: u8,
    tone: CalendarTone,
    first_weekday: CalendarFirstWeekday,
    is_show_outside_days: Option<bool>,
    show_outside_days: Option<bool>,
    selected_day: Option<u8>,
    default_selected_day: Option<u8>,
    on_selected_day_change: Option<leptos::prelude::Callback<Option<u8>>>,
    on_day_press: Option<leptos::prelude::Callback<u8>>,
    aria_label: Option<String>,
    class_name: Option<String>,
    motion: CalendarMotion,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
