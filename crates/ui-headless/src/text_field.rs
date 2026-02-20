use crate::a11y::{A11yDirection, locale_attrs};
use crate::focus_ring::{FocusRingHandlers, FocusRingOptions, use_focus_ring};
use leptos::prelude::*;
use std::collections::HashSet;
use ui_state_primitives::text_field::{
    TextFieldState as PrimitiveTextFieldState, TextFieldStateInput,
    resolve_state as resolve_text_field_state,
};

fn push_token(tokens: &mut Vec<String>, seen: &mut HashSet<String>, token: &str) {
    let token = token.trim();
    if token.is_empty() {
        return;
    }
    if seen.insert(token.into()) {
        tokens.push(token.into());
    }
}

fn push_whitespace_separated(tokens: &mut Vec<String>, seen: &mut HashSet<String>, value: &str) {
    for token in value.split_whitespace() {
        push_token(tokens, seen, token);
    }
}

#[derive(Clone)]
pub struct TextFieldOptions {
    pub id: String,
    pub has_description: bool,
    pub has_error: bool,
    pub aria_describedby: Signal<Option<String>>,
    pub is_invalid: Signal<bool>,
    pub is_required: Signal<bool>,
}

#[derive(Clone)]
pub struct TextFieldInputAttrs {
    pub id: String,
    pub aria_describedby: Memo<Option<String>>,
    pub aria_invalid: Memo<Option<&'static str>>,
    pub aria_required: Memo<Option<&'static str>>,
}

#[derive(Clone)]
pub struct TextFieldLabelAttrs {
    pub for_attr: String,
}

#[derive(Clone)]
pub struct TextFieldMessageAttrs {
    pub id: String,
}

#[derive(Clone)]
pub struct TextFieldAria {
    pub input: TextFieldInputAttrs,
    pub label: TextFieldLabelAttrs,
    pub description: TextFieldMessageAttrs,
    pub error: TextFieldMessageAttrs,
}

#[derive(Clone)]
pub struct TextFieldContractOptions {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub value: Signal<String>,
    pub on_value_change: Callback<String>,
    pub is_invalid: Signal<bool>,
    pub is_required: Signal<bool>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct TextFieldContractAttrs {
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct TextFieldContractHandlers {
    pub focus_ring: FocusRingHandlers,
    pub on_input: Callback<String>,
}

#[derive(Clone)]
pub struct TextFieldContractState {
    pub is_focused: ReadSignal<bool>,
    pub is_focus_visible: Memo<bool>,
    pub resolved: Memo<PrimitiveTextFieldState>,
}

#[derive(Clone)]
pub struct TextFieldContract {
    pub attrs: TextFieldContractAttrs,
    pub handlers: TextFieldContractHandlers,
    pub state: TextFieldContractState,
}

pub fn use_text_field(options: TextFieldOptions) -> TextFieldAria {
    let id = options.id;
    let description_id = format!("{id}-description");
    let error_id = format!("{id}-error");

    let aria_describedby = Memo::new({
        let has_description = options.has_description;
        let has_error = options.has_error;
        let aria_describedby = options.aria_describedby;
        let is_invalid = options.is_invalid;
        let description_id = description_id.clone();
        let error_id = error_id.clone();
        move |_| {
            let mut tokens = Vec::new();
            let mut seen = HashSet::new();

            if has_description {
                push_token(&mut tokens, &mut seen, &description_id);
            }

            if has_error && is_invalid.get() {
                push_token(&mut tokens, &mut seen, &error_id);
            }

            if let Some(value) = aria_describedby.get() {
                push_whitespace_separated(&mut tokens, &mut seen, &value);
            }

            (!tokens.is_empty()).then(|| tokens.join(" "))
        }
    });

    let aria_invalid = Memo::new({
        let is_invalid = options.is_invalid;
        move |_| is_invalid.get().then_some("true")
    });

    let aria_required = Memo::new({
        let is_required = options.is_required;
        move |_| is_required.get().then_some("true")
    });

    TextFieldAria {
        input: TextFieldInputAttrs {
            id: id.clone(),
            aria_describedby,
            aria_invalid,
            aria_required,
        },
        label: TextFieldLabelAttrs {
            for_attr: id.clone(),
        },
        description: TextFieldMessageAttrs { id: description_id },
        error: TextFieldMessageAttrs { id: error_id },
    }
}

pub fn use_text_field_contract(options: TextFieldContractOptions) -> TextFieldContract {
    let TextFieldContractOptions {
        is_disabled,
        is_read_only,
        value,
        on_value_change,
        is_invalid,
        is_required,
        lang,
        dir,
    } = options;

    let locale = locale_attrs(lang, dir);
    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
    let resolved = Memo::new(move |_| {
        resolve_text_field_state(TextFieldStateInput {
            is_disabled,
            is_invalid: is_invalid.get(),
            is_read_only,
            value: value.get().as_str(),
            is_required: is_required.get(),
        })
    });

    let on_input = Callback::new(move |next: String| {
        if is_disabled || is_read_only {
            return;
        }
        on_value_change.run(next);
    });

    TextFieldContract {
        attrs: TextFieldContractAttrs {
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: TextFieldContractHandlers {
            focus_ring: focus_ring.handlers,
            on_input,
        },
        state: TextFieldContractState {
            is_focused: focus_ring.is_focused,
            is_focus_visible: focus_ring.is_focus_visible,
            resolved,
        },
    }
}

#[cfg(test)]
#[path = "test/text_field.rs"]
mod tests;
