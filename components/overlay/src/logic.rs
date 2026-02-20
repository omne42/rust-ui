use super::{OverlayPartState, OverlayPartStateInput, OverlaySlot};

pub const DEFAULT_ROLE: &str = "dialog";
pub const DEFAULT_DISMISSABLE: bool = true;
pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;

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
    let mut classes = vec![state.base_class.into()];

    if state.slot == OverlaySlot::Root {
        if state.has_custom_motion {
            classes.push("ui-overlay--custom-motion".to_string());
        }

        if state.has_custom_role {
            classes.push("ui-overlay--custom-role".to_string());
        }

        if state.has_custom_aria_labelledby {
            classes.push("ui-overlay--custom-aria-labelledby".to_string());
        }

        if state.has_custom_aria_describedby {
            classes.push("ui-overlay--custom-aria-describedby".to_string());
        }

        if state.is_dismissable != DEFAULT_DISMISSABLE {
            classes.push("ui-overlay--custom-dismiss".to_string());
        }

        if state.is_keyboard_dismiss_disabled != DEFAULT_KEYBOARD_DISMISS_DISABLED {
            classes.push("ui-overlay--custom-keyboard-dismiss".to_string());
        }

        if state.has_on_exit_complete {
            classes.push("ui-overlay--custom-exit".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-overlay--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
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
