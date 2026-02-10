use crate::overlay::{OverlayPartState, OverlayPartStateInput, OverlaySlot};

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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
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
    let mut classes = vec![state.base_class.to_string()];

    if state.slot == OverlaySlot::Root {
        if state.has_custom_motion {
            classes.push("ui-overlay--custom-motion".to_string());
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
mod tests {
    use super::*;

    #[test]
    fn state_and_dismiss_attrs_follow_contract() {
        assert_eq!(state_attr_for_open(true), "open");
        assert_eq!(state_attr_for_open(false), "closed");
        assert_eq!(dismiss_attr(true), "dismissable");
        assert_eq!(dismiss_attr(false), "locked");
        assert_eq!(keyboard_dismiss_attr(false), "enabled");
        assert_eq!(keyboard_dismiss_attr(true), "disabled");
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-overlay  ".to_string())),
            Some("docs-overlay".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_source_markers() {
        let state = resolve_state(OverlayPartStateInput {
            slot: OverlaySlot::Root,
            open: true,
            is_dismissable: false,
            is_keyboard_dismiss_disabled: true,
            has_custom_role: true,
            has_custom_aria_labelledby: true,
            has_custom_aria_describedby: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        });

        assert_eq!(state.slot_attr, "overlay");
        assert_eq!(state.base_class, "ui-overlay");
        assert_eq!(state.state_attr, "open");
        assert_eq!(state.dismiss_attr, "locked");
        assert_eq!(state.keyboard_dismiss_attr, "disabled");
        assert_eq!(state.role_source_attr, "custom");
        assert_eq!(state.aria_labelledby_source_attr, "custom");
        assert_eq!(state.aria_describedby_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.dismiss_source_attr, "custom");
        assert_eq!(state.keyboard_dismiss_source_attr, "custom");
        assert_eq!(state.exit_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-overlay-state".to_string()),
            resolve_state(OverlayPartStateInput {
                slot: OverlaySlot::Root,
                open: false,
                is_dismissable: false,
                is_keyboard_dismiss_disabled: true,
                has_custom_role: false,
                has_custom_aria_labelledby: false,
                has_custom_aria_describedby: false,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_on_exit_complete: true,
            }),
        );

        for token in [
            "ui-overlay",
            "ui-overlay--custom-motion",
            "ui-overlay--custom-dismiss",
            "ui-overlay--custom-keyboard-dismiss",
            "ui-overlay--custom-exit",
            "ui-overlay--custom-class",
            "docs-overlay-state",
        ] {
            assert!(
                class_name.contains(token),
                "overlay class name should include `{token}`"
            );
        }
    }

    #[test]
    fn should_close_on_escape_requires_topmost_and_enabled_keyboard_dismiss() {
        assert!(should_close_on_escape("Escape", true, false, false, false));
        assert!(!should_close_on_escape("Enter", true, false, false, false));
        assert!(!should_close_on_escape(
            "Escape", false, false, false, false
        ));
        assert!(!should_close_on_escape("Escape", true, true, false, false));
        assert!(!should_close_on_escape("Escape", true, false, true, false));
        assert!(!should_close_on_escape("Escape", true, false, false, true));
    }
}
