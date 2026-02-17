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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxGroupState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_invalid: bool,
    pub is_valid: bool,
    pub is_required: bool,
    pub is_optional: bool,
    pub has_description: bool,
    pub has_error: bool,
    pub shows_error: bool,
    pub has_messages: bool,
}

pub fn resolve_state(
    is_disabled: bool,
    is_invalid: bool,
    is_required: bool,
    has_description: bool,
    has_error: bool,
) -> CheckboxGroupState {
    let shows_error = has_error && is_invalid;
    let has_messages = has_description || shows_error;

    CheckboxGroupState {
        is_disabled,
        is_enabled: !is_disabled,
        is_invalid,
        is_valid: !is_invalid,
        is_required,
        is_optional: !is_required,
        has_description,
        has_error,
        shows_error,
        has_messages,
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
    fn resolve_state_tracks_optional_without_messages() {
        let state = resolve_state(false, false, false, false, false);

        assert!(!state.is_disabled);
        assert!(state.is_enabled);
        assert!(!state.is_invalid);
        assert!(state.is_valid);
        assert!(!state.is_required);
        assert!(state.is_optional);
        assert!(!state.has_description);
        assert!(!state.has_error);
        assert!(!state.shows_error);
        assert!(!state.has_messages);
    }

    #[test]
    fn resolve_state_tracks_invalid_required_and_messages() {
        let state = resolve_state(true, true, true, true, true);

        assert!(state.is_disabled);
        assert!(!state.is_enabled);
        assert!(state.is_invalid);
        assert!(!state.is_valid);
        assert!(state.is_required);
        assert!(!state.is_optional);
        assert!(state.has_description);
        assert!(state.has_error);
        assert!(state.shows_error);
        assert!(state.has_messages);
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
