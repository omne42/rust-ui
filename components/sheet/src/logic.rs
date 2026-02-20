#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SheetPlacement {
    #[default]
    Bottom,
    Left,
    Right,
}

impl SheetPlacement {
    pub fn class_name(self) -> &'static str {
        match self {
            SheetPlacement::Bottom => "ui-sheet--placement-bottom",
            SheetPlacement::Left => "ui-sheet--placement-left",
            SheetPlacement::Right => "ui-sheet--placement-right",
        }
    }

    pub fn data_attr(self) -> &'static str {
        match self {
            SheetPlacement::Bottom => "bottom",
            SheetPlacement::Left => "left",
            SheetPlacement::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SheetSlot {
    Root,
    Backdrop,
    Panel,
}

impl SheetSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            SheetSlot::Root => "sheet",
            SheetSlot::Backdrop => "sheet-backdrop",
            SheetSlot::Panel => "sheet-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            SheetSlot::Root => "ui-sheet",
            SheetSlot::Backdrop => "ui-sheet__backdrop",
            SheetSlot::Panel => "ui-sheet__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SheetPartStateInput {
    pub slot: SheetSlot,
    pub open: bool,
    pub placement: SheetPlacement,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_motion: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_aria_describedby: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SheetPartState {
    pub slot: SheetSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub placement_attr: &'static str,
    pub placement_class: &'static str,
    pub is_open: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_aria_describedby: bool,
    pub has_on_exit_complete: bool,
    pub dismiss_attr: &'static str,
    pub keyboard_dismiss_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub placement_source_attr: &'static str,
    pub dismiss_source_attr: &'static str,
    pub keyboard_dismiss_source_attr: &'static str,
    pub aria_labelledby_source_attr: &'static str,
    pub aria_describedby_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

pub const DEFAULT_DISMISSABLE: bool = true;
pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SheetAgentContract {
    pub schema_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_axis_attr: &'static str,
    pub source_axis_attr: &'static str,
    pub render_mode_attr: &'static str,
    pub streaming_attr: &'static str,
    pub fallback_attr: &'static str,
    pub output_status_attr: &'static str,
}

pub fn agent_contract() -> SheetAgentContract {
    SheetAgentContract {
        schema_attr: "sheet.v1",
        intent_attr: "overlay",
        action_attr: "dismiss",
        state_axis_attr: "open",
        source_axis_attr: "default|custom",
        render_mode_attr: "snapshot",
        streaming_attr: "optional",
        fallback_attr: "snapshot",
        output_status_attr: "verified",
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

pub fn resolve_state(input: SheetPartStateInput) -> SheetPartState {
    let has_custom_placement = input.placement != SheetPlacement::default();

    SheetPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            SheetSlot::Root => state_attr_for_open(input.open),
            SheetSlot::Backdrop => "backdrop",
            SheetSlot::Panel => "panel",
        },
        placement_attr: input.placement.data_attr(),
        placement_class: input.placement.class_name(),
        is_open: input.open,
        is_dismissable: input.is_dismissable,
        is_keyboard_dismiss_disabled: input.is_keyboard_dismiss_disabled,
        has_custom_motion: input.has_custom_motion,
        has_custom_placement,
        has_custom_aria_labelledby: input.has_custom_aria_labelledby,
        has_custom_aria_describedby: input.has_custom_aria_describedby,
        has_on_exit_complete: input.has_on_exit_complete,
        dismiss_attr: dismiss_attr(input.is_dismissable),
        keyboard_dismiss_attr: keyboard_dismiss_attr(input.is_keyboard_dismiss_disabled),
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        placement_source_attr: if has_custom_placement {
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
        exit_source_attr: if input.has_on_exit_complete {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(state: SheetPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if state.slot == SheetSlot::Root {
        classes.push(state.placement_class.into());

        if state.has_custom_motion {
            classes.push("ui-sheet--custom-motion".to_string());
        }

        if state.has_custom_placement {
            classes.push("ui-sheet--custom-placement".to_string());
        }

        if state.is_dismissable != DEFAULT_DISMISSABLE {
            classes.push("ui-sheet--custom-dismiss".to_string());
        }

        if state.is_keyboard_dismiss_disabled != DEFAULT_KEYBOARD_DISMISS_DISABLED {
            classes.push("ui-sheet--custom-keyboard-dismiss".to_string());
        }

        if state.has_custom_aria_labelledby {
            classes.push("ui-sheet--custom-aria-labelledby".to_string());
        }

        if state.has_custom_aria_describedby {
            classes.push("ui-sheet--custom-aria-describedby".to_string());
        }

        if state.has_on_exit_complete {
            classes.push("ui-sheet--custom-exit".to_string());
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
