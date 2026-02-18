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
mod tests {
    use super::*;

    #[test]
    fn contract_maps_locale_and_escape_shortcut() {
        let (value_raw, _set_value_raw) = signal("query".to_string());
        let contract = use_search_field(SearchFieldOptions {
            is_disabled: false,
            is_read_only: false,
            value: value_raw.into(),
            on_value_change: Callback::new(move |_| {}),
            on_submit: None,
            on_clear: None,
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.attrs.dir, Some("rtl"));
        assert_eq!(
            contract.attrs.aria_keyshortcuts.get_untracked(),
            Some("Escape")
        );
        assert!(contract.state.can_clear.get_untracked());
    }

    #[test]
    fn enter_submits_when_editable() {
        let (value_raw, _set_value_raw) = signal("needle".to_string());
        let (submitted, set_submitted) = signal(String::new());
        let on_submit = Callback::new(move |next: String| set_submitted.set(next));

        let contract = use_search_field(SearchFieldOptions {
            is_disabled: false,
            is_read_only: false,
            value: value_raw.into(),
            on_value_change: Callback::new(move |_| {}),
            on_submit: Some(on_submit),
            on_clear: None,
            lang: None,
            dir: None,
        });

        assert_eq!(
            contract.handlers.on_key_down.run("Enter".to_string()),
            SearchFieldKeyDownResult::Submitted
        );
        assert_eq!(submitted.get_untracked(), "needle");
    }

    #[test]
    fn escape_clears_through_normalized_handler() {
        let (value_raw, set_value_raw) = signal("query".to_string());
        let on_value_change = Callback::new(move |next: String| set_value_raw.set(next));
        let (cleared, set_cleared) = signal(0usize);
        let on_clear = Callback::new(move |_| set_cleared.update(|count| *count += 1));

        let contract = use_search_field(SearchFieldOptions {
            is_disabled: false,
            is_read_only: false,
            value: value_raw.into(),
            on_value_change,
            on_submit: None,
            on_clear: Some(on_clear),
            lang: None,
            dir: None,
        });

        assert_eq!(
            contract.handlers.on_key_down.run("Escape".to_string()),
            SearchFieldKeyDownResult::Cleared
        );
        assert_eq!(value_raw.get_untracked(), String::new());
        assert_eq!(cleared.get_untracked(), 1);
    }

    #[test]
    fn disabled_state_blocks_submit_and_clear() {
        let (value_raw, set_value_raw) = signal("query".to_string());
        let on_value_change = Callback::new(move |next: String| set_value_raw.set(next));
        let (submitted, set_submitted) = signal(0usize);
        let (cleared, set_cleared) = signal(0usize);

        let contract = use_search_field(SearchFieldOptions {
            is_disabled: true,
            is_read_only: false,
            value: value_raw.into(),
            on_value_change,
            on_submit: Some(Callback::new(move |_| {
                set_submitted.update(|count| *count += 1)
            })),
            on_clear: Some(Callback::new(move |_| {
                set_cleared.update(|count| *count += 1)
            })),
            lang: None,
            dir: None,
        });

        assert!(!contract.state.can_submit.get_untracked());
        assert_eq!(
            contract.handlers.on_key_down.run("Enter".to_string()),
            SearchFieldKeyDownResult::Ignored
        );
        assert_eq!(
            contract.handlers.on_key_down.run("Escape".to_string()),
            SearchFieldKeyDownResult::Ignored
        );
        assert_eq!(submitted.get_untracked(), 0);
        assert_eq!(cleared.get_untracked(), 0);
        assert_eq!(value_raw.get_untracked(), "query");
    }
}
