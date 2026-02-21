use crate::a11y::{A11yDirection, locale_attrs};
use crate::button::{ButtonElement, ButtonHandlers, ButtonOptions, use_button};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CalendarRootHandlers;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CalendarRootState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarRootAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarRootContract {
    pub attrs: CalendarRootAttrs,
    pub handlers: CalendarRootHandlers,
    pub state: CalendarRootState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarRootOptions {
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_calendar_root(options: CalendarRootOptions) -> CalendarRootContract {
    let locale = locale_attrs(options.lang, options.dir);

    CalendarRootContract {
        attrs: CalendarRootAttrs {
            role: "group",
            aria_label: options.aria_label,
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: CalendarRootHandlers,
        state: CalendarRootState,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarDayA11yInput {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub in_current_month: bool,
    pub is_selected: bool,
}

#[derive(Clone)]
pub struct CalendarDayHandlers {
    pub press: ButtonHandlers,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarDayAttrs {
    pub role: Option<&'static str>,
    pub tabindex: Option<i32>,
    pub aria_selected: Option<&'static str>,
    pub aria_disabled: Option<&'static str>,
    pub aria_label: String,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarDayState {
    pub is_pressable: bool,
    pub is_selected: bool,
    pub month_source: &'static str,
}

#[derive(Clone)]
pub struct CalendarDayContract {
    pub attrs: CalendarDayAttrs,
    pub handlers: CalendarDayHandlers,
    pub state: CalendarDayState,
}

#[derive(Clone, Default)]
pub struct CalendarDayOptions {
    pub on_press: Option<Callback<()>>,
}

pub fn use_calendar_day(
    input: CalendarDayA11yInput,
    options: CalendarDayOptions,
) -> CalendarDayContract {
    let is_pressable = input.in_current_month;
    let button = use_button(ButtonOptions {
        is_disabled: !is_pressable,
        on_press: options.on_press,
        element: ButtonElement::Button,
    });

    CalendarDayContract {
        attrs: CalendarDayAttrs {
            role: button.attrs.role,
            tabindex: button.attrs.tabindex,
            aria_selected: input.is_selected.then_some("true"),
            aria_disabled: button.attrs.aria_disabled,
            aria_label: format!(
                "{}-{:02}-{:02}",
                input.year,
                input.month.clamp(1, 12),
                input.day
            ),
            disabled: !is_pressable,
        },
        handlers: CalendarDayHandlers {
            press: button.handlers,
        },
        state: CalendarDayState {
            is_pressable,
            is_selected: input.is_selected,
            month_source: if input.in_current_month {
                "current"
            } else {
                "outside"
            },
        },
    }
}

#[cfg(test)]
#[path = "test/calendar.rs"]
mod tests;
