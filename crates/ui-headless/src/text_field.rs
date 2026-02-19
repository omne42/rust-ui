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
mod tests {
    use super::*;

    #[test]
    fn text_field_contract_exposes_locale_and_derived_state() {
        let (value, set_value) = signal("  ".to_string());
        let (is_invalid, set_invalid) = signal(false);
        let (is_required, set_required) = signal(false);

        let contract = use_text_field_contract(TextFieldContractOptions {
            is_disabled: false,
            is_read_only: false,
            value: value.into(),
            on_value_change: Callback::new(move |next| set_value.set(next)),
            is_invalid: is_invalid.into(),
            is_required: is_required.into(),
            lang: Some("  en-US ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(contract.attrs.lang.as_deref(), Some("en-US"));
        assert_eq!(contract.attrs.dir, Some("rtl"));
        let state = contract.state.resolved.get_untracked();
        assert_eq!(state.state_attr, "ready");
        assert_eq!(state.value_attr, "empty");
        assert_eq!(state.requirement_attr, "optional");

        contract.handlers.on_input.run("hello".to_string());
        set_invalid.set(true);
        set_required.set(true);

        let state = contract.state.resolved.get_untracked();
        assert_eq!(state.state_attr, "invalid");
        assert_eq!(state.value_attr, "filled");
        assert_eq!(state.requirement_attr, "required");
    }

    #[test]
    fn text_field_contract_focus_handlers_respect_disabled() {
        let (value, _) = signal("value".to_string());
        let (is_invalid, _) = signal(false);
        let (is_required, _) = signal(false);

        let enabled = use_text_field_contract(TextFieldContractOptions {
            is_disabled: false,
            is_read_only: false,
            value: value.into(),
            on_value_change: Callback::new(move |_| {}),
            is_invalid: is_invalid.into(),
            is_required: is_required.into(),
            lang: None,
            dir: None,
        });

        enabled.handlers.focus_ring.on_focus.run(());
        assert!(enabled.state.is_focused.get_untracked());
        assert_eq!(enabled.state.resolved.get_untracked().state_attr, "ready");

        let (value, _) = signal("value".to_string());
        let (is_invalid, _) = signal(false);
        let (is_required, _) = signal(false);
        let disabled = use_text_field_contract(TextFieldContractOptions {
            is_disabled: true,
            is_read_only: false,
            value: value.into(),
            on_value_change: Callback::new(move |_| {}),
            is_invalid: is_invalid.into(),
            is_required: is_required.into(),
            lang: None,
            dir: None,
        });

        disabled.handlers.focus_ring.on_focus.run(());
        assert!(!disabled.state.is_focused.get_untracked());
        assert_eq!(
            disabled.state.resolved.get_untracked().state_attr,
            "disabled"
        );
    }

    #[test]
    fn text_field_contract_input_handler_respects_disabled_and_read_only() {
        let (value, set_value) = signal("before".to_string());
        let (is_invalid, _) = signal(false);
        let (is_required, _) = signal(false);
        let enabled = use_text_field_contract(TextFieldContractOptions {
            is_disabled: false,
            is_read_only: false,
            value: value.into(),
            on_value_change: Callback::new(move |next| set_value.set(next)),
            is_invalid: is_invalid.into(),
            is_required: is_required.into(),
            lang: None,
            dir: None,
        });

        enabled.handlers.on_input.run("after".to_string());
        assert_eq!(enabled.state.resolved.get_untracked().value_attr, "filled");

        let (disabled_value, set_disabled_value) = signal("locked".to_string());
        let (is_invalid, _) = signal(false);
        let (is_required, _) = signal(false);
        let disabled = use_text_field_contract(TextFieldContractOptions {
            is_disabled: true,
            is_read_only: false,
            value: disabled_value.into(),
            on_value_change: Callback::new(move |next| set_disabled_value.set(next)),
            is_invalid: is_invalid.into(),
            is_required: is_required.into(),
            lang: None,
            dir: None,
        });
        disabled.handlers.on_input.run("ignored".to_string());
        assert_eq!(disabled.state.resolved.get_untracked().value_attr, "filled");

        let (readonly_value, set_readonly_value) = signal("readonly".to_string());
        let (is_invalid, _) = signal(false);
        let (is_required, _) = signal(false);
        let read_only = use_text_field_contract(TextFieldContractOptions {
            is_disabled: false,
            is_read_only: true,
            value: readonly_value.into(),
            on_value_change: Callback::new(move |next| set_readonly_value.set(next)),
            is_invalid: is_invalid.into(),
            is_required: is_required.into(),
            lang: None,
            dir: None,
        });
        read_only.handlers.on_input.run("ignored".to_string());
        assert_eq!(
            read_only.state.resolved.get_untracked().value_attr,
            "filled"
        );
    }
}
