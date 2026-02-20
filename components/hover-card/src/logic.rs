use crate::{HoverCardPartState, HoverCardPartStateInput, HoverCardSlot};

pub const DEFAULT_OPEN_DELAY_MS: u64 = 140;
pub const DEFAULT_CLOSE_DELAY_MS: u64 = 180;

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_id(custom_id: Option<String>, fallback_id: String) -> (String, bool) {
    if let Some(custom_id) = normalize_optional_text(custom_id) {
        return (custom_id, true);
    }

    (fallback_id, false)
}

pub fn has_custom_delays(open_delay_ms: u64, close_delay_ms: u64) -> bool {
    open_delay_ms != DEFAULT_OPEN_DELAY_MS || close_delay_ms != DEFAULT_CLOSE_DELAY_MS
}

pub fn resolve_part_state(input: HoverCardPartStateInput) -> HoverCardPartState {
    HoverCardPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            HoverCardSlot::Root => state_attr_for_open(input.open),
            HoverCardSlot::Trigger => "trigger",
            HoverCardSlot::Panel => "panel",
        },
        is_open: input.open,
        is_disabled: input.disabled,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_delays: input.has_custom_delays,
        has_custom_id: input.has_custom_id,
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
        delay_source_attr: if input.has_custom_delays {
            "custom"
        } else {
            "default"
        },
        id_source_attr: if input.has_custom_id {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: HoverCardPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if state.slot == HoverCardSlot::Root {
        if state.has_custom_motion {
            classes.push("ui-hover-card--custom-motion".to_string());
        }

        if state.has_custom_delays {
            classes.push("ui-hover-card--custom-delay".to_string());
        }

        if state.has_custom_id {
            classes.push("ui-hover-card--custom-id".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-hover-card--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
    }

    classes.join(" ")
}

pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String {
    format!(
        "--ui-hover-card-top: {top_px}px; --ui-hover-card-left: {left_px}px; --ui-hover-card-anchor-width: {anchor_width_px}px;"
    )
}

pub fn should_handle_escape(key: &str, is_open: bool, is_composing: bool) -> bool {
    key == "Escape" && is_open && !is_composing
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
