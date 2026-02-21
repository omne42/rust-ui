use super::{OverlayPartState, OverlayPartStateInput, OverlaySlot};
use leptos::prelude::Callback;
use std::borrow::Cow;

pub const DEFAULT_ROLE: &str = "dialog";
pub const DEFAULT_DISMISSABLE: bool = true;
pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayDismissMode {
    Dismissable,
    Locked,
}

impl OverlayDismissMode {
    pub fn from_is_dismissable(is_dismissable: bool) -> Self {
        if is_dismissable {
            Self::Dismissable
        } else {
            Self::Locked
        }
    }

    pub fn is_dismissable(self) -> bool {
        matches!(self, Self::Dismissable)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayKeyboardDismissMode {
    Enabled,
    Disabled,
}

impl OverlayKeyboardDismissMode {
    pub fn from_is_disabled(is_keyboard_dismiss_disabled: bool) -> Self {
        if is_keyboard_dismiss_disabled {
            Self::Disabled
        } else {
            Self::Enabled
        }
    }

    pub fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }
}

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn dismiss_attr(is_dismissable: bool) -> &'static str {
    if is_dismissable {
        "dismissable"
    } else {
        "locked"
    }
}

pub fn keyboard_dismiss_attr(is_keyboard_dismiss_disabled: bool) -> &'static str {
    if is_keyboard_dismiss_disabled {
        "disabled"
    } else {
        "enabled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_on_exit_complete(callback: Option<Callback<()>>) -> Callback<()> {
    callback.unwrap_or_else(|| Callback::new(|_| {}))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlayStateInputs {
    pub open: bool,
    pub dismiss_mode: OverlayDismissMode,
    pub keyboard_dismiss_mode: OverlayKeyboardDismissMode,
    pub has_custom_role: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_aria_describedby: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlayResolvedStates {
    pub root_state: OverlayPartState,
    pub backdrop_state: OverlayPartState,
    pub panel_state: OverlayPartState,
}

pub fn resolve_states(input: OverlayStateInputs) -> OverlayResolvedStates {
    let state_for_slot = |slot, open, has_custom_class_name| {
        resolve_state(OverlayPartStateInput {
            slot,
            open,
            is_dismissable: input.dismiss_mode.is_dismissable(),
            is_keyboard_dismiss_disabled: input.keyboard_dismiss_mode.is_disabled(),
            has_custom_role: input.has_custom_role,
            has_custom_aria_labelledby: input.has_custom_aria_labelledby,
            has_custom_aria_describedby: input.has_custom_aria_describedby,
            has_custom_class_name,
            has_custom_motion: input.has_custom_motion,
            has_on_exit_complete: input.has_on_exit_complete,
        })
    };

    OverlayResolvedStates {
        root_state: state_for_slot(OverlaySlot::Root, input.open, input.has_custom_class_name),
        backdrop_state: state_for_slot(OverlaySlot::Backdrop, false, false),
        panel_state: state_for_slot(OverlaySlot::Panel, false, false),
    }
}

pub fn resolve_state(input: OverlayPartStateInput) -> OverlayPartState {
    OverlayPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            OverlaySlot::Root => state_attr_for_open(input.open),
            OverlaySlot::Backdrop => "backdrop",
            OverlaySlot::Panel => "panel",
        },
        is_open: input.open,
        is_dismissable: input.is_dismissable,
        is_keyboard_dismiss_disabled: input.is_keyboard_dismiss_disabled,
        has_custom_role: input.has_custom_role,
        has_custom_aria_labelledby: input.has_custom_aria_labelledby,
        has_custom_aria_describedby: input.has_custom_aria_describedby,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
        dismiss_attr: dismiss_attr(input.is_dismissable),
        keyboard_dismiss_attr: keyboard_dismiss_attr(input.is_keyboard_dismiss_disabled),
        role_source_attr: if input.has_custom_role {
            "custom"
        } else {
            "default"
        },
        aria_labelledby_source_attr: if input.has_custom_aria_labelledby {
            "custom"
        } else {
            "default"
        },
        aria_describedby_source_attr: if input.has_custom_aria_describedby {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        dismiss_source_attr: if input.is_dismissable == DEFAULT_DISMISSABLE {
            "default"
        } else {
            "custom"
        },
        keyboard_dismiss_source_attr: if input.is_keyboard_dismiss_disabled
            == DEFAULT_KEYBOARD_DISMISS_DISABLED
        {
            "default"
        } else {
            "custom"
        },
        exit_source_attr: if input.has_on_exit_complete {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: OverlayPartState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if state.slot == OverlaySlot::Root {
        if state.has_custom_motion {
            classes.push(Cow::Borrowed("ui-overlay--custom-motion"));
        }

        if state.has_custom_role {
            classes.push(Cow::Borrowed("ui-overlay--custom-role"));
        }

        if state.has_custom_aria_labelledby {
            classes.push(Cow::Borrowed("ui-overlay--custom-aria-labelledby"));
        }

        if state.has_custom_aria_describedby {
            classes.push(Cow::Borrowed("ui-overlay--custom-aria-describedby"));
        }

        if state.is_dismissable != DEFAULT_DISMISSABLE {
            classes.push(Cow::Borrowed("ui-overlay--custom-dismiss"));
        }

        if state.is_keyboard_dismiss_disabled != DEFAULT_KEYBOARD_DISMISS_DISABLED {
            classes.push(Cow::Borrowed("ui-overlay--custom-keyboard-dismiss"));
        }

        if state.has_on_exit_complete {
            classes.push(Cow::Borrowed("ui-overlay--custom-exit"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-overlay--custom-class"));
            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }
    }

    classes.join(" ")
}

pub fn should_close_on_escape(
    key: &str,
    is_topmost: bool,
    is_composing: bool,
    default_prevented: bool,
    is_keyboard_dismiss_disabled: bool,
) -> bool {
    key == "Escape"
        && is_topmost
        && !is_composing
        && !default_prevented
        && !is_keyboard_dismiss_disabled
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
