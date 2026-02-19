pub const DEFAULT_ARIA_LABEL: &str = "Calendar";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CalendarTone {
    #[default]
    Default,
    Quiet,
    Strong,
}

impl CalendarTone {
    pub fn class_name(self) -> &'static str {
        match self {
            CalendarTone::Default => "ui-calendar--tone-default",
            CalendarTone::Quiet => "ui-calendar--tone-quiet",
            CalendarTone::Strong => "ui-calendar--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CalendarTone::Default => "default",
            CalendarTone::Quiet => "quiet",
            CalendarTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CalendarFirstWeekday {
    #[default]
    Sunday,
    Monday,
}

impl CalendarFirstWeekday {
    pub fn class_name(self) -> &'static str {
        match self {
            CalendarFirstWeekday::Sunday => "ui-calendar--weekday-sunday",
            CalendarFirstWeekday::Monday => "ui-calendar--weekday-monday",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CalendarFirstWeekday::Sunday => "sunday",
            CalendarFirstWeekday::Monday => "monday",
        }
    }

    pub fn offset(self) -> usize {
        match self {
            CalendarFirstWeekday::Sunday => 0,
            CalendarFirstWeekday::Monday => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarGridCell {
    pub year: i32,
    pub month: u8,
    pub day: Option<u8>,
    pub in_current_month: bool,
    pub is_selected: bool,
}

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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_month(month: u8) -> u8 {
    month.clamp(1, 12)
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
    match normalize_month(month) {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

fn weekday_sunday0(year: i32, month: u8, day: u8) -> usize {
    let month_offsets = [0_i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let normalized_month = normalize_month(month);
    let month_index = usize::from(normalized_month.saturating_sub(1));

    let mut y = year;
    if normalized_month < 3 {
        y -= 1;
    }

    let weekday = y + y / 4 - y / 100 + y / 400 + month_offsets[month_index] + i32::from(day);
    let mut value = weekday % 7;
    if value < 0 {
        value += 7;
    }
    usize::try_from(value).unwrap_or(0)
}

pub fn weekday_index(year: i32, month: u8, day: u8, first_weekday: CalendarFirstWeekday) -> usize {
    let sunday_index = weekday_sunday0(year, month, day);
    (sunday_index + 7 - first_weekday.offset()) % 7
}

pub fn weekday_labels(first_weekday: CalendarFirstWeekday) -> [&'static str; 7] {
    match first_weekday {
        CalendarFirstWeekday::Sunday => ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
        CalendarFirstWeekday::Monday => ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
    }
}

pub fn month_name(month: u8) -> &'static str {
    match normalize_month(month) {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "January",
    }
}

pub fn month_title(year: i32, month: u8) -> String {
    format!("{} {}", month_name(month), year)
}

pub fn normalize_selected_day(selected_day: Option<u8>, year: i32, month: u8) -> Option<u8> {
    selected_day.and_then(|day| {
        let max_day = days_in_month(year, month);
        (1..=max_day).contains(&day).then_some(day)
    })
}

fn previous_month(year: i32, month: u8) -> (i32, u8) {
    let normalized_month = normalize_month(month);
    if normalized_month == 1 {
        (year - 1, 12)
    } else {
        (year, normalized_month - 1)
    }
}

fn next_month(year: i32, month: u8) -> (i32, u8) {
    let normalized_month = normalize_month(month);
    if normalized_month == 12 {
        (year + 1, 1)
    } else {
        (year, normalized_month + 1)
    }
}

pub fn build_month_grid(
    year: i32,
    month: u8,
    first_weekday: CalendarFirstWeekday,
    show_outside_days: bool,
    selected_day: Option<u8>,
) -> Vec<CalendarGridCell> {
    let normalized_month = normalize_month(month);
    let normalized_selected_day = normalize_selected_day(selected_day, year, normalized_month);

    let first_offset = weekday_index(year, normalized_month, 1, first_weekday);
    let current_month_days = days_in_month(year, normalized_month);

    let (prev_year, prev_month) = previous_month(year, normalized_month);
    let prev_month_days = days_in_month(prev_year, prev_month);
    let (next_year, next_month) = next_month(year, normalized_month);

    (0..42)
        .map(|index| {
            if index < first_offset {
                if show_outside_days {
                    let offset = first_offset - index;
                    let day = prev_month_days.saturating_sub(u8::try_from(offset).unwrap_or(0));
                    CalendarGridCell {
                        year: prev_year,
                        month: prev_month,
                        day: Some(day.saturating_add(1)),
                        in_current_month: false,
                        is_selected: false,
                    }
                } else {
                    CalendarGridCell {
                        year: prev_year,
                        month: prev_month,
                        day: None,
                        in_current_month: false,
                        is_selected: false,
                    }
                }
            } else {
                let day_index = index - first_offset;
                if day_index < usize::from(current_month_days) {
                    let day = u8::try_from(day_index + 1).unwrap_or(1);
                    CalendarGridCell {
                        year,
                        month: normalized_month,
                        day: Some(day),
                        in_current_month: true,
                        is_selected: normalized_selected_day == Some(day),
                    }
                } else if show_outside_days {
                    let next_day =
                        u8::try_from(day_index + 1 - usize::from(current_month_days)).unwrap_or(1);
                    CalendarGridCell {
                        year: next_year,
                        month: next_month,
                        day: Some(next_day),
                        in_current_month: false,
                        is_selected: false,
                    }
                } else {
                    CalendarGridCell {
                        year: next_year,
                        month: next_month,
                        day: None,
                        in_current_month: false,
                        is_selected: false,
                    }
                }
            }
        })
        .collect()
}

pub fn resolve_state(input: CalendarStateInput) -> CalendarState {
    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let selected_day = normalize_selected_day(input.selected_day, input.year, input.month);

    CalendarState {
        year: input.year,
        month: normalize_month(input.month),
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        first_weekday: input.first_weekday,
        first_weekday_class: input.first_weekday.class_name(),
        first_weekday_attr: input.first_weekday.as_attr(),
        show_outside_days: input.show_outside_days,
        has_selected_day: selected_day.is_some(),
        selected_day,
        data_state_attr: if selected_day.is_some() {
            "selected"
        } else {
            "default"
        },
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_class_names_and_attrs_are_stable() {
        assert_eq!(
            CalendarTone::Default.class_name(),
            "ui-calendar--tone-default"
        );
        assert_eq!(CalendarTone::Quiet.class_name(), "ui-calendar--tone-quiet");
        assert_eq!(
            CalendarTone::Strong.class_name(),
            "ui-calendar--tone-strong"
        );

        assert_eq!(CalendarTone::Default.as_attr(), "default");
        assert_eq!(CalendarTone::Quiet.as_attr(), "quiet");
        assert_eq!(CalendarTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn date_helpers_cover_leap_year_and_month_bounds() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2023, 2), 28);
        assert_eq!(normalize_month(0), 1);
        assert_eq!(normalize_month(15), 12);
    }

    #[test]
    fn weekday_index_matches_known_calendar_day() {
        assert_eq!(weekday_index(2026, 1, 1, CalendarFirstWeekday::Sunday), 4);
        assert_eq!(weekday_index(2026, 1, 1, CalendarFirstWeekday::Monday), 3);
    }

    #[test]
    fn month_grid_has_fixed_cell_count_and_selection_marker() {
        let grid = build_month_grid(2026, 1, CalendarFirstWeekday::Sunday, true, Some(6));
        assert_eq!(grid.len(), 42);
        assert!(
            grid.iter()
                .any(|cell| cell.is_selected && cell.day == Some(6))
        );
        assert!(
            grid.iter()
                .any(|cell| !cell.in_current_month && cell.day.is_some())
        );
    }

    #[test]
    fn resolve_state_tracks_source_and_selection_state() {
        let state = resolve_state(CalendarStateInput {
            year: 2026,
            month: 1,
            tone: CalendarTone::Strong,
            first_weekday: CalendarFirstWeekday::Monday,
            show_outside_days: true,
            selected_day: Some(4),
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.tone_attr, "strong");
        assert_eq!(state.first_weekday_attr, "monday");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.data_state_attr, "selected");
        assert_eq!(state.selected_day, Some(4));
    }
}
