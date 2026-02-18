use crate::a11y::{A11yDirection, labeled_group_attrs};
use leptos::prelude::*;
use ui_state_primitives::time_field::{
    normalize_minute_step, normalize_time_value, resolve_time_parts, update_hour_from_input,
    update_minute_from_input,
};

#[derive(Clone)]
pub struct TimeFieldOptions {
    pub is_disabled: bool,
    pub value: Signal<Option<String>>,
    pub on_value_change: Callback<Option<String>>,
    pub minute_step: u8,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
    pub hour_aria_label: String,
    pub minute_aria_label: String,
    pub clear_aria_label: String,
}

#[derive(Clone)]
pub struct TimeFieldHandlers {
    pub on_hour_input: Callback<String>,
    pub on_minute_input: Callback<String>,
    pub on_clear: Callback<()>,
}

#[derive(Clone)]
pub struct TimeFieldAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub hour_aria_label: String,
    pub minute_aria_label: String,
    pub clear_aria_label: String,
}

#[derive(Clone)]
pub struct TimeFieldState {
    pub normalized_value: Memo<Option<String>>,
    pub parts: Memo<(u8, u8, bool)>,
    pub has_value: Memo<bool>,
    pub minute_step: u8,
}

#[derive(Clone)]
pub struct TimeFieldAria {
    pub attrs: TimeFieldAttrs,
    pub handlers: TimeFieldHandlers,
    pub state: TimeFieldState,
}

pub fn use_time_field(options: TimeFieldOptions) -> TimeFieldAria {
    let TimeFieldOptions {
        is_disabled,
        value,
        on_value_change,
        minute_step,
        aria_label,
        lang,
        dir,
        hour_aria_label,
        minute_aria_label,
        clear_aria_label,
    } = options;
    let minute_step = normalize_minute_step(minute_step);
    let group = labeled_group_attrs(aria_label, lang, dir);

    let normalized_value = Memo::new(move |_| normalize_time_value(value.get(), minute_step));
    let normalized_value_for_parts = normalized_value;
    let parts =
        Memo::new(move |_| resolve_time_parts(normalized_value_for_parts.get(), minute_step));
    let has_value = Memo::new(move |_| parts.get().2);

    let value_for_hour = value;
    let on_hour_value_change = on_value_change;
    let on_hour_input = Callback::new(move |hour_input: String| {
        if is_disabled {
            return;
        }

        let next = update_hour_from_input(value_for_hour.get_untracked(), &hour_input, minute_step);
        on_hour_value_change.run(next);
    });

    let value_for_minute = value;
    let on_minute_value_change = on_value_change;
    let on_minute_input = Callback::new(move |minute_input: String| {
        if is_disabled {
            return;
        }

        let next =
            update_minute_from_input(value_for_minute.get_untracked(), &minute_input, minute_step);
        on_minute_value_change.run(next);
    });

    let on_clear_value_change = on_value_change;
    let on_clear = Callback::new(move |_| {
        if is_disabled {
            return;
        }

        on_clear_value_change.run(None);
    });

    TimeFieldAria {
        attrs: TimeFieldAttrs {
            role: group.role,
            aria_label: group.aria_label,
            lang: group.lang,
            dir: group.dir,
            hour_aria_label,
            minute_aria_label,
            clear_aria_label,
        },
        handlers: TimeFieldHandlers {
            on_hour_input,
            on_minute_input,
            on_clear,
        },
        state: TimeFieldState {
            normalized_value,
            parts,
            has_value,
            minute_step,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_field_handlers_update_value_and_clear() {
        let (value, set_value) = signal(Some("09:30".to_string()));
        let on_value_change = Callback::new(move |next: Option<String>| {
            set_value.set(next);
        });

        let time_field = use_time_field(TimeFieldOptions {
            is_disabled: false,
            value: value.into(),
            on_value_change,
            minute_step: 15,
            aria_label: "Meeting time".to_string(),
            lang: None,
            dir: None,
            hour_aria_label: "Hour".to_string(),
            minute_aria_label: "Minute".to_string(),
            clear_aria_label: "Clear time".to_string(),
        });

        assert_eq!(
            time_field.state.normalized_value.get_untracked(),
            Some("09:30".to_string())
        );
        time_field.handlers.on_hour_input.run("18".to_string());
        assert_eq!(value.get_untracked(), Some("18:30".to_string()));
        time_field.handlers.on_minute_input.run("44".to_string());
        assert_eq!(value.get_untracked(), Some("18:30".to_string()));
        time_field.handlers.on_clear.run(());
        assert_eq!(value.get_untracked(), None);
    }

    #[test]
    fn time_field_handlers_are_noop_when_disabled() {
        let (value, set_value) = signal(Some("09:30".to_string()));
        let on_value_change = Callback::new(move |next: Option<String>| {
            set_value.set(next);
        });

        let time_field = use_time_field(TimeFieldOptions {
            is_disabled: true,
            value: value.into(),
            on_value_change,
            minute_step: 10,
            aria_label: "Meeting time".to_string(),
            lang: None,
            dir: None,
            hour_aria_label: "Hour".to_string(),
            minute_aria_label: "Minute".to_string(),
            clear_aria_label: "Clear time".to_string(),
        });

        time_field.handlers.on_hour_input.run("10".to_string());
        time_field.handlers.on_minute_input.run("50".to_string());
        time_field.handlers.on_clear.run(());
        assert_eq!(value.get_untracked(), Some("09:30".to_string()));
    }

    #[test]
    fn time_field_attrs_expose_locale_and_segment_labels() {
        let (value, _) = signal(None::<String>);
        let time_field = use_time_field(TimeFieldOptions {
            is_disabled: false,
            value: value.into(),
            on_value_change: Callback::new(move |_| {}),
            minute_step: 5,
            aria_label: "Meeting time".to_string(),
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
            hour_aria_label: "Hour".to_string(),
            minute_aria_label: "Minute".to_string(),
            clear_aria_label: "Clear time".to_string(),
        });

        assert_eq!(time_field.attrs.role, "group");
        assert_eq!(time_field.attrs.aria_label, "Meeting time");
        assert_eq!(time_field.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(time_field.attrs.dir, Some("rtl"));
        assert_eq!(time_field.attrs.hour_aria_label, "Hour");
        assert_eq!(time_field.attrs.minute_aria_label, "Minute");
        assert_eq!(time_field.attrs.clear_aria_label, "Clear time");
    }
}
