use leptos::prelude::*;
use ui_headless::{TextFieldOptions, use_text_field};

#[derive(Clone)]
pub struct CheckboxGroupOptions {
    pub id: String,
    pub has_description: bool,
    pub has_error: bool,
    pub aria_describedby: Signal<Option<String>>,
    pub is_invalid: Signal<bool>,
    pub is_required: Signal<bool>,
}

#[derive(Clone)]
pub struct CheckboxGroupFieldsetAttrs {
    pub aria_describedby: Memo<Option<String>>,
    pub aria_invalid: Memo<Option<&'static str>>,
    pub aria_required: Memo<Option<&'static str>>,
}

#[derive(Clone)]
pub struct CheckboxGroupMessageAttrs {
    pub id: String,
}

#[derive(Clone)]
pub struct CheckboxGroupAria {
    pub fieldset: CheckboxGroupFieldsetAttrs,
    pub description: CheckboxGroupMessageAttrs,
    pub error: CheckboxGroupMessageAttrs,
}

pub fn use_checkbox_group(options: CheckboxGroupOptions) -> CheckboxGroupAria {
    let aria = use_text_field(TextFieldOptions {
        id: options.id,
        has_description: options.has_description,
        has_error: options.has_error,
        aria_describedby: options.aria_describedby,
        is_invalid: options.is_invalid,
        is_required: options.is_required,
    });

    CheckboxGroupAria {
        fieldset: CheckboxGroupFieldsetAttrs {
            aria_describedby: aria.input.aria_describedby,
            aria_invalid: aria.input.aria_invalid,
            aria_required: aria.input.aria_required,
        },
        description: CheckboxGroupMessageAttrs {
            id: aria.description.id,
        },
        error: CheckboxGroupMessageAttrs { id: aria.error.id },
    }
}
