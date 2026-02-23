use crate::a11y::{A11yDirection, labeled_group_attrs};
use leptos::prelude::*;

#[derive(Clone)]
pub struct DateFieldOptions {
    pub is_disabled: bool,
    pub value: Signal<Option<String>>,
    pub resolve_parts: Callback<Option<String>, (i32, u8, u8, bool)>,
    pub on_year_input: Callback<String>,
    pub on_month_input: Callback<String>,
    pub on_day_input: Callback<String>,
    pub on_clear: Callback<()>,
    pub aria_label: String,
    pub aria_labelledby: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
    pub year_aria_label: String,
    pub month_aria_label: String,
    pub day_aria_label: String,
    pub clear_aria_label: String,
}

#[derive(Clone)]
pub struct DateFieldHandlers {
    pub on_year_input: Callback<String>,
    pub on_month_input: Callback<String>,
    pub on_day_input: Callback<String>,
    pub on_clear: Callback<()>,
}

#[derive(Clone)]
pub struct DateFieldAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub aria_labelledby: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub year_aria_label: String,
    pub month_aria_label: String,
    pub day_aria_label: String,
    pub clear_aria_label: String,
}

#[derive(Clone)]
pub struct DateFieldState {
    pub parts: Memo<(i32, u8, u8, bool)>,
    pub has_value: Memo<bool>,
}

#[derive(Clone)]
pub struct DateFieldContract {
    pub attrs: DateFieldAttrs,
    pub handlers: DateFieldHandlers,
    pub state: DateFieldState,
}

pub fn use_date_field(options: DateFieldOptions) -> DateFieldContract {
    let DateFieldOptions {
        is_disabled,
        value,
        resolve_parts,
        on_year_input,
        on_month_input,
        on_day_input,
        on_clear,
        aria_label,
        aria_labelledby,
        lang,
        dir,
        year_aria_label,
        month_aria_label,
        day_aria_label,
        clear_aria_label,
    } = options;

    let group = labeled_group_attrs(aria_label, lang, dir);
    let parts = Memo::new(move |_| resolve_parts.run(value.get()));
    let parts_for_has_value = parts;
    let has_value = Memo::new(move |_| parts_for_has_value.get().3);

    let on_year_input = Callback::new(move |year_input: String| {
        if is_disabled {
            return;
        }
        on_year_input.run(year_input);
    });
    let on_month_input = Callback::new(move |month_input: String| {
        if is_disabled {
            return;
        }
        on_month_input.run(month_input);
    });
    let on_day_input = Callback::new(move |day_input: String| {
        if is_disabled {
            return;
        }
        on_day_input.run(day_input);
    });
    let on_clear = Callback::new(move |_| {
        if is_disabled {
            return;
        }
        on_clear.run(());
    });

    DateFieldContract {
        attrs: DateFieldAttrs {
            role: group.role,
            aria_label: group.aria_label,
            aria_labelledby,
            lang: group.lang,
            dir: group.dir,
            year_aria_label,
            month_aria_label,
            day_aria_label,
            clear_aria_label,
        },
        handlers: DateFieldHandlers {
            on_year_input,
            on_month_input,
            on_day_input,
            on_clear,
        },
        state: DateFieldState { parts, has_value },
    }
}

#[cfg(test)]
#[path = "test/date_field.rs"]
mod tests;
