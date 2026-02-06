use leptos::prelude::*;
use ui_headless::{TextFieldOptions, use_text_field};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckboxGroupIds {
    pub legend_id: String,
}

pub fn resolve_ids(id: &str) -> CheckboxGroupIds {
    CheckboxGroupIds {
        legend_id: format!("{id}-label"),
    }
}

pub fn normalize_label(label: String) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        "Options".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_ids_builds_legend_id() {
        assert_eq!(
            resolve_ids("prefs"),
            CheckboxGroupIds {
                legend_id: "prefs-label".to_string(),
            }
        );
    }

    #[test]
    fn normalize_label_trims_and_defaults() {
        assert_eq!(
            normalize_label("  Fruits  ".to_string()),
            "Fruits".to_string()
        );
        assert_eq!(normalize_label("   ".to_string()), "Options".to_string());
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Pick at least one  ".to_string())),
            Some("Pick at least one".to_string())
        );
    }
}
