mod logic;
pub mod styles;
mod view;

pub use logic::{CalendarFirstWeekday, CalendarGridCell, CalendarTone, DEFAULT_ARIA_LABEL};
pub use view::Calendar;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarStateInput {
    pub year: i32,
    pub month: u8,
    pub tone: CalendarTone,
    pub first_weekday: CalendarFirstWeekday,
    pub show_outside_days: bool,
    pub selected_day: Option<u8>,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarState {
    pub year: i32,
    pub month: u8,
    pub tone: CalendarTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub first_weekday: CalendarFirstWeekday,
    pub first_weekday_class: &'static str,
    pub first_weekday_attr: &'static str,
    pub show_outside_days: bool,
    pub has_selected_day: bool,
    pub selected_day: Option<u8>,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
