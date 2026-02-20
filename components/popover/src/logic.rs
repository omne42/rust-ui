use crate::{PopoverPartState, PopoverPartStateInput, PopoverSlot};

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
    let mut classes = vec![state.base_class.into()];

    if state.slot == PopoverSlot::Root {
        if state.has_custom_motion {
            classes.push("ui-popover--custom-motion".to_string());
        }

        if state.has_custom_placement {
            classes.push("ui-popover--custom-placement".to_string());
        }

        if !state.is_modal {
            classes.push("ui-popover--non-modal".to_string());
            classes.push("ui-popover--custom-modal".to_string());
        }

        if state.has_on_exit_complete {
            classes.push("ui-popover--custom-exit".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-popover--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
    }

    classes.join(" ")
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
