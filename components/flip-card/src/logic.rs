use crate::{FlipCardPartState, FlipCardPartStateInput, FlipCardSlot};

pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_FLIPPED: bool = false;
pub const DEFAULT_HOVER_FLIP: bool = false;

pub fn state_attr(is_flipped: bool) -> &'static str {
    if is_flipped { "flipped" } else { "default" }
}

pub fn flip_mode_attr(flip_on_hover: bool) -> &'static str {
    if flip_on_hover { "hover" } else { "toggle" }
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

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_part_state(input: FlipCardPartStateInput) -> FlipCardPartState {
    FlipCardPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            FlipCardSlot::Root => state_attr(input.is_flipped),
            FlipCardSlot::Front => "front",
            FlipCardSlot::Back => "back",
        },
        visibility_attr: match input.slot {
            FlipCardSlot::Root => state_attr(input.is_flipped),
            FlipCardSlot::Front => {
                if input.is_flipped {
                    "hidden"
                } else {
                    "visible"
                }
            }
            FlipCardSlot::Back => {
                if input.is_flipped {
                    "visible"
                } else {
                    "hidden"
                }
            }
        },
        is_disabled: input.disabled,
        is_flipped: input.is_flipped,
        flip_mode_attr: flip_mode_attr(input.flip_on_hover),
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_id: input.has_custom_id,
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        id_source_attr: source_attr(input.has_custom_id),
        flip_mode_source_attr: source_attr(input.flip_on_hover),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FlipCardPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    match state.slot {
        FlipCardSlot::Root => {
            if state.is_disabled {
                classes.push("ui-flip-card--disabled".to_string());
            } else {
                classes.push("ui-flip-card--enabled".to_string());
            }

            if state.is_flipped {
                classes.push("ui-flip-card--flipped".to_string());
            } else {
                classes.push("ui-flip-card--default".to_string());
            }

            if state.flip_mode_attr == "hover" {
                classes.push("ui-flip-card--hover".to_string());
            } else {
                classes.push("ui-flip-card--toggle".to_string());
            }

            if state.has_custom_class_name {
                classes.push("ui-flip-card--custom-class".to_string());
            }

            if state.has_custom_motion {
                classes.push("ui-flip-card--custom-motion".to_string());
            }

            if state.has_custom_id {
                classes.push("ui-flip-card--custom-id".to_string());
            }

            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
        FlipCardSlot::Front | FlipCardSlot::Back => {
            if state.visibility_attr == "visible" {
                classes.push("ui-flip-card__face--visible".to_string());
            } else {
                classes.push("ui-flip-card__face--hidden".to_string());
            }
        }
    }

    classes.join(" ")
}

pub fn should_toggle_key(key: &str, is_composing: bool) -> bool {
    if is_composing {
        return false;
    }

    matches!(key, "Enter" | " ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
