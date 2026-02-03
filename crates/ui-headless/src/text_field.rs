use leptos::prelude::*;
use std::collections::HashSet;

fn push_token(tokens: &mut Vec<String>, seen: &mut HashSet<String>, token: &str) {
    let token = token.trim();
    if token.is_empty() {
        return;
    }
    if seen.insert(token.to_string()) {
        tokens.push(token.to_string());
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
