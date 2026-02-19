#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxVariant {
    #[default]
    Default,
    Accent,
}

impl CheckboxVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-checkbox--variant-default",
            Self::Accent => "ui-checkbox--variant-accent",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl CheckboxSize {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-checkbox--size-default",
            Self::Sm => "ui-checkbox--size-sm",
            Self::Lg => "ui-checkbox--size-lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxState {
    pub is_checked: bool,
    pub is_unchecked: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
}

impl CheckboxState {
    pub fn data_state(self) -> &'static str {
        if self.is_checked {
            "checked"
        } else {
            "unchecked"
        }
    }
}

pub fn resolve_state(
    is_checked: bool,
    is_disabled: bool,
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
    is_focus_visible: bool,
) -> CheckboxState {
    let is_enabled = !is_disabled;

    CheckboxState {
        is_checked,
        is_unchecked: !is_checked,
        is_disabled,
        is_enabled,
        is_pressed: is_pressed && is_enabled,
        is_hovered: is_hovered && is_enabled,
        is_focused: is_focused && is_enabled,
        is_focus_visible: is_focus_visible && is_enabled,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_class_names_are_stable() {
        assert_eq!(
            CheckboxVariant::Default.class_name(),
            "ui-checkbox--variant-default"
        );
        assert_eq!(
            CheckboxVariant::Accent.class_name(),
            "ui-checkbox--variant-accent"
        );
    }

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(
            CheckboxSize::Default.class_name(),
            "ui-checkbox--size-default"
        );
        assert_eq!(CheckboxSize::Sm.class_name(), "ui-checkbox--size-sm");
        assert_eq!(CheckboxSize::Lg.class_name(), "ui-checkbox--size-lg");
    }

    #[test]
    fn resolve_state_tracks_checked_enabled_interactions() {
        let state = resolve_state(true, false, true, true, true, true);

        assert!(state.is_checked);
        assert!(!state.is_unchecked);
        assert!(!state.is_disabled);
        assert!(state.is_enabled);
        assert!(state.is_pressed);
        assert!(state.is_hovered);
        assert!(state.is_focused);
        assert!(state.is_focus_visible);
        assert_eq!(state.data_state(), "checked");
    }

    #[test]
    fn resolve_state_clears_interaction_flags_when_disabled() {
        let state = resolve_state(false, true, true, true, true, true);

        assert!(!state.is_checked);
        assert!(state.is_unchecked);
        assert!(state.is_disabled);
        assert!(!state.is_enabled);
        assert!(!state.is_pressed);
        assert!(!state.is_hovered);
        assert!(!state.is_focused);
        assert!(!state.is_focus_visible);
        assert_eq!(state.data_state(), "unchecked");
    }
}

#[cfg(feature = "component-checkbox_group")]
use leptos::prelude::{Memo, Signal};
#[cfg(feature = "component-checkbox_group")]
use ui_headless::{TextFieldOptions, use_text_field};

#[cfg(feature = "component-checkbox_group")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckboxGroupIds {
    pub legend_id: String,
}

#[cfg(feature = "component-checkbox_group")]
pub fn resolve_checkbox_group_ids(id: &str) -> CheckboxGroupIds {
    CheckboxGroupIds {
        legend_id: format!("{id}-label"),
    }
}

#[cfg(feature = "component-checkbox_group")]
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

#[cfg(feature = "component-checkbox_group")]
pub fn resolve_checkbox_group_state(
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

#[cfg(feature = "component-checkbox_group")]
pub fn normalize_checkbox_group_label(label: String) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        "Options".to_string()
    } else {
        trimmed.into()
    }
}

#[cfg(feature = "component-checkbox_group")]
pub fn normalize_checkbox_group_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

#[cfg(feature = "component-checkbox_group")]
#[derive(Clone)]
pub struct CheckboxGroupOptions {
    pub id: String,
    pub has_description: bool,
    pub has_error: bool,
    pub aria_describedby: Signal<Option<String>>,
    pub is_invalid: Signal<bool>,
    pub is_required: Signal<bool>,
}

#[cfg(feature = "component-checkbox_group")]
#[derive(Clone)]
pub struct CheckboxGroupFieldsetAttrs {
    pub aria_describedby: Memo<Option<String>>,
    pub aria_invalid: Memo<Option<&'static str>>,
    pub aria_required: Memo<Option<&'static str>>,
}

#[cfg(feature = "component-checkbox_group")]
#[derive(Clone)]
pub struct CheckboxGroupMessageAttrs {
    pub id: String,
}

#[cfg(feature = "component-checkbox_group")]
#[derive(Clone)]
pub struct CheckboxGroupAria {
    pub fieldset: CheckboxGroupFieldsetAttrs,
    pub description: CheckboxGroupMessageAttrs,
    pub error: CheckboxGroupMessageAttrs,
}

#[cfg(feature = "component-checkbox_group")]
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

#[cfg(all(test, feature = "component-checkbox_group"))]
mod checkbox_group_tests {
    use super::*;

    #[test]
    fn resolve_checkbox_group_ids_builds_legend_id() {
        assert_eq!(
            resolve_checkbox_group_ids("prefs"),
            CheckboxGroupIds {
                legend_id: "prefs-label".to_string(),
            }
        );
    }

    #[test]
    fn resolve_checkbox_group_state_tracks_optional_without_messages() {
        let state = resolve_checkbox_group_state(false, false, false, false, false);

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
    fn resolve_checkbox_group_state_tracks_invalid_required_and_messages() {
        let state = resolve_checkbox_group_state(true, true, true, true, true);

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
    fn normalize_checkbox_group_label_trims_and_defaults() {
        assert_eq!(
            normalize_checkbox_group_label("  Fruits  ".to_string()),
            "Fruits".to_string()
        );
        assert_eq!(
            normalize_checkbox_group_label("   ".to_string()),
            "Options".to_string()
        );
    }

    #[test]
    fn normalize_checkbox_group_optional_text_filters_blank_values() {
        assert_eq!(normalize_checkbox_group_optional_text(None), None);
        assert_eq!(
            normalize_checkbox_group_optional_text(Some("  ".to_string())),
            None
        );
        assert_eq!(
            normalize_checkbox_group_optional_text(Some("  Pick at least one  ".to_string())),
            Some("Pick at least one".to_string())
        );
    }
}
