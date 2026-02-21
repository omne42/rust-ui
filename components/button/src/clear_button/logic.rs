use crate::clear_button::{ClearButtonState, ClearButtonStateInput};
use leptos::prelude::Signal;

pub const DEFAULT_ARIA_LABEL: &str = "Clear";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ClearButtonVariant {
    #[default]
    Default,
    OverBackground,
}

impl ClearButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ClearButtonVariant::Default => "ui-clear-button--variant-default",
            ClearButtonVariant::OverBackground => "ui-clear-button--variant-over-background",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ClearButtonVariant::Default => "default",
            ClearButtonVariant::OverBackground => "over-background",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ClearButtonFocusMode {
    #[default]
    Default,
    Prevent,
    ExcludeTab,
}

impl ClearButtonFocusMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            ClearButtonFocusMode::Default => "default",
            ClearButtonFocusMode::Prevent => "prevent",
            ClearButtonFocusMode::ExcludeTab => "exclude-tab",
        }
    }

    pub fn prevents_focus(self) -> bool {
        matches!(self, ClearButtonFocusMode::Prevent)
    }

    pub fn excludes_from_tab_order(self) -> bool {
        matches!(self, ClearButtonFocusMode::ExcludeTab)
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::button::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>, default: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (default.into(), false)
}

pub fn resolve_state(input: ClearButtonStateInput) -> ClearButtonState {
    let prevent_focus = input.focus_mode.prevents_focus();
    let exclude_from_tab_order = input.focus_mode.excludes_from_tab_order();

    let data_state_attr = if input.disabled && input.inset {
        "disabled-inset"
    } else if input.disabled {
        "disabled"
    } else if prevent_focus {
        "prevent-focus"
    } else if exclude_from_tab_order {
        "exclude-tab"
    } else if input.inset {
        "inset"
    } else {
        "ready"
    };

    ClearButtonState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        is_inset: input.inset,
        is_disabled: input.disabled,
        focus_mode: input.focus_mode,
        prevent_focus,
        exclude_from_tab_order,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
        data_state_attr,
        focus_mode_attr: input.focus_mode.as_attr(),
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ClearButtonState) -> String {
    let mut classes = vec!["ui-clear-button".to_string(), state.variant_class.into()];

    if state.is_inset {
        classes.push("ui-clear-button--inset".to_string());
    }

    if state.is_disabled {
        classes.push("ui-clear-button--disabled".to_string());
    }

    if state.prevent_focus {
        classes.push("ui-clear-button--prevent-focus".to_string());
    }

    if state.exclude_from_tab_order {
        classes.push("ui-clear-button--exclude-tab".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-clear-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-clear-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-clear-button--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn resolve_visibility_signals(
    is_visible: Option<Signal<bool>>,
    is_disabled_signal: Option<Signal<bool>>,
) -> (Signal<bool>, Signal<bool>) {
    (
        is_visible.unwrap_or_else(|| Signal::derive(|| true)),
        is_disabled_signal.unwrap_or_else(|| Signal::derive(|| false)),
    )
}

#[cfg(test)]
#[path = "../../test/clear_button/logic.rs"]
mod tests;
