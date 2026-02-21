use crate::a11y::{A11yDirection, locale_attrs};
use crate::text_field::{TextFieldOptions, use_text_field};
use leptos::prelude::*;
use ui_state_primitives::checkbox_group::{
    CheckboxGroupState as PrimitiveCheckboxGroupState, resolve_checkbox_group_state,
};

#[derive(Clone)]
pub struct CheckboxGroupOptions {
    pub id: String,
    pub is_disabled: bool,
    pub has_description: bool,
    pub has_error: bool,
    pub aria_describedby: Signal<Option<String>>,
    pub is_invalid: Signal<bool>,
    pub is_required: Signal<bool>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct CheckboxGroupFieldsetAttrs {
    pub aria_describedby: Memo<Option<String>>,
    pub aria_invalid: Memo<Option<&'static str>>,
    pub aria_required: Memo<Option<&'static str>>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct CheckboxGroupMessageAttrs {
    pub id: String,
}

#[derive(Clone)]
pub struct CheckboxGroupAttrs {
    pub fieldset: CheckboxGroupFieldsetAttrs,
    pub description: CheckboxGroupMessageAttrs,
    pub error: CheckboxGroupMessageAttrs,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CheckboxGroupHandlers;

#[derive(Clone)]
pub struct CheckboxGroupSemanticState {
    pub resolved: Memo<PrimitiveCheckboxGroupState>,
}

#[derive(Clone)]
pub struct CheckboxGroupA11y {
    pub attrs: CheckboxGroupAttrs,
    pub handlers: CheckboxGroupHandlers,
    pub state: CheckboxGroupSemanticState,
}

pub fn use_checkbox_group(options: CheckboxGroupOptions) -> CheckboxGroupA11y {
    let CheckboxGroupOptions {
        id,
        is_disabled,
        has_description,
        has_error,
        aria_describedby,
        is_invalid,
        is_required,
        lang,
        dir,
    } = options;
    let locale = locale_attrs(lang, dir);

    let text_field = use_text_field(TextFieldOptions {
        id,
        has_description,
        has_error,
        aria_describedby,
        is_invalid,
        is_required,
    });
    let resolved = Memo::new(move |_| {
        resolve_checkbox_group_state(
            is_disabled,
            is_invalid.get(),
            is_required.get(),
            has_description,
            has_error,
        )
    });

    CheckboxGroupA11y {
        attrs: CheckboxGroupAttrs {
            fieldset: CheckboxGroupFieldsetAttrs {
                aria_describedby: text_field.input.aria_describedby,
                aria_invalid: text_field.input.aria_invalid,
                aria_required: text_field.input.aria_required,
                lang: locale.lang,
                dir: locale.dir,
            },
            description: CheckboxGroupMessageAttrs {
                id: text_field.description.id,
            },
            error: CheckboxGroupMessageAttrs {
                id: text_field.error.id,
            },
        },
        handlers: CheckboxGroupHandlers,
        state: CheckboxGroupSemanticState { resolved },
    }
}

#[cfg(test)]
#[path = "test/checkbox_group.rs"]
mod tests;
