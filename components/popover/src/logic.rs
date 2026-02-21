use std::borrow::Cow;

use crate::{PopoverPartState, PopoverPartStateInput, PopoverSlot};
use leptos::prelude::Callback;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverModalMode {
    Modal,
    NonModal,
}

impl PopoverModalMode {
    pub fn from_is_modal(is_modal: bool) -> Self {
        if is_modal {
            Self::Modal
        } else {
            Self::NonModal
        }
    }

    pub fn is_modal(self) -> bool {
        matches!(self, Self::Modal)
    }
}

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn modal_attr(is_modal: bool) -> &'static str {
    if is_modal { "modal" } else { "non-modal" }
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
pub struct PopoverStateInputs {
    pub open: bool,
    pub modal_mode: PopoverModalMode,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverResolvedStates {
    pub root_state: PopoverPartState,
    pub panel_state: PopoverPartState,
}

pub fn resolve_states(input: PopoverStateInputs) -> PopoverResolvedStates {
    let state_for_slot = |slot, open, has_custom_class_name| {
        resolve_state(PopoverPartStateInput {
            slot,
            open,
            is_modal: input.modal_mode.is_modal(),
            has_custom_class_name,
            has_custom_motion: input.has_custom_motion,
            has_custom_placement: input.has_custom_placement,
            has_on_exit_complete: input.has_on_exit_complete,
        })
    };

    PopoverResolvedStates {
        root_state: state_for_slot(PopoverSlot::Root, input.open, input.has_custom_class_name),
        panel_state: state_for_slot(PopoverSlot::Panel, false, false),
    }
}

pub fn resolve_state(input: PopoverPartStateInput) -> PopoverPartState {
    PopoverPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            PopoverSlot::Root => state_attr_for_open(input.open),
            PopoverSlot::Panel => "panel",
        },
        is_open: input.open,
        is_modal: input.is_modal,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_placement: input.has_custom_placement,
        has_on_exit_complete: input.has_on_exit_complete,
        modal_attr: modal_attr(input.is_modal),
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
        placement_source_attr: if input.has_custom_placement {
            "custom"
        } else {
            "default"
        },
        modal_source_attr: if input.is_modal { "default" } else { "custom" },
        exit_source_attr: if input.has_on_exit_complete {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: PopoverPartState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if state.slot == PopoverSlot::Root {
        if state.has_custom_motion {
            classes.push(Cow::Borrowed("ui-popover--custom-motion"));
        }

        if state.has_custom_placement {
            classes.push(Cow::Borrowed("ui-popover--custom-placement"));
        }

        if !state.is_modal {
            classes.push(Cow::Borrowed("ui-popover--non-modal"));
            classes.push(Cow::Borrowed("ui-popover--custom-modal"));
        }

        if state.has_on_exit_complete {
            classes.push(Cow::Borrowed("ui-popover--custom-exit"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-popover--custom-class"));
            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String {
    format!(
        "--ui-popover-top: {top_px}px; --ui-popover-left: {left_px}px; --ui-popover-anchor-width: {anchor_width_px}px;"
    )
}

pub fn should_close_on_escape(
    key: &str,
    is_topmost: bool,
    is_composing: bool,
    default_prevented: bool,
) -> bool {
    key == "Escape" && is_topmost && !is_composing && !default_prevented
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
