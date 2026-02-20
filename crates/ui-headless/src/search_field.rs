use crate::a11y::{A11yDirection, locale_attrs};
use crate::clearable_text_field::{ClearableTextFieldOptions, use_clearable_text_field};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFieldKeyDownResult {
    Ignored,
    Submitted,
    Cleared,
}

#[derive(Clone)]
pub struct SearchFieldOptions {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub value: Signal<String>,
    pub on_value_change: Callback<String>,
    pub on_submit: Option<Callback<String>>,
    pub on_clear: Option<Callback<()>>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct SearchFieldAttrs {
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub aria_keyshortcuts: Memo<Option<&'static str>>,
}

#[derive(Clone)]
pub struct SearchFieldHandlers {
    pub on_input: Callback<String>,
    pub on_key_down: Callback<String, SearchFieldKeyDownResult>,
    pub on_clear: Callback<()>,
}

#[derive(Clone)]
pub struct SearchFieldState {
    pub is_empty: Memo<bool>,
    pub can_clear: Memo<bool>,
    pub can_submit: Memo<bool>,
}

#[derive(Clone)]
pub struct SearchFieldContract {
    pub attrs: SearchFieldAttrs,
    pub handlers: SearchFieldHandlers,
    pub state: SearchFieldState,
}

pub fn use_search_field(options: SearchFieldOptions) -> SearchFieldContract {
    let SearchFieldOptions {
        is_disabled,
        is_read_only,
        value,
        on_value_change,
        on_submit,
        on_clear,
        lang,
        dir,
    } = options;

    let locale = locale_attrs(lang, dir);
    let is_empty = Memo::new(move |_| value.get().is_empty());

    let user_on_clear = StoredValue::new(on_clear);
    let on_value_change_for_clear = on_value_change;
    let request_clear = Callback::new(move |_| {
        if is_disabled || is_read_only {
            return;
        }
        on_value_change_for_clear.run(String::new());
        if let Some(on_clear) = user_on_clear.get_value() {
            on_clear.run(());
        }
    });

    let clearable = use_clearable_text_field(ClearableTextFieldOptions {
        is_disabled,
        is_read_only,
        is_clearable: true,
        is_empty: is_empty.into(),
        on_clear: Some(request_clear),
    });

    let can_submit = Memo::new(move |_| !is_disabled && !is_read_only);
    let on_value_change_for_input = on_value_change;
    let on_input = Callback::new(move |next: String| {
        if is_disabled || is_read_only {
            return;
        }
        on_value_change_for_input.run(next);
    });

    let on_submit = StoredValue::new(on_submit);
    let value_for_submit = value;
    let on_key_down = Callback::new(move |key: String| -> SearchFieldKeyDownResult {
        if key == "Enter" {
            if is_disabled || is_read_only {
                return SearchFieldKeyDownResult::Ignored;
            }
            if let Some(on_submit) = on_submit.get_value() {
                on_submit.run(value_for_submit.get_untracked());
                return SearchFieldKeyDownResult::Submitted;
            }
            return SearchFieldKeyDownResult::Ignored;
        }

        if clearable.handlers.on_key_down.run(key) {
            return SearchFieldKeyDownResult::Cleared;
        }

        SearchFieldKeyDownResult::Ignored
    });

    SearchFieldContract {
        attrs: SearchFieldAttrs {
            lang: locale.lang,
            dir: locale.dir,
            aria_keyshortcuts: clearable.attrs.aria_keyshortcuts,
        },
        handlers: SearchFieldHandlers {
            on_input,
            on_key_down,
            on_clear: request_clear,
        },
        state: SearchFieldState {
            is_empty,
            can_clear: clearable.state.can_clear_on_escape,
            can_submit,
        },
    }
}

#[cfg(test)]
#[path = "test/search_field.rs"]
mod tests;
