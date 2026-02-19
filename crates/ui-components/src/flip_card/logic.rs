use crate::flip_card::{FlipCardPartState, FlipCardPartStateInput, FlipCardSlot};

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
mod tests {
    use super::*;

    #[test]
    fn helpers_keep_state_and_mode_contracts_stable() {
        assert_eq!(state_attr(true), "flipped");
        assert_eq!(state_attr(false), "default");
        assert_eq!(flip_mode_attr(true), "hover");
        assert_eq!(flip_mode_attr(false), "toggle");

        assert_eq!(
            normalize_optional_text(Some("  flip  ".to_string())),
            Some("flip".to_string())
        );
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);

        assert_eq!(
            resolve_id(Some(" docs-flip ".to_string()), "fallback".to_string()),
            ("docs-flip".to_string(), true)
        );
        assert_eq!(
            resolve_id(None, "generated".to_string()),
            ("generated".to_string(), false)
        );
    }

    #[test]
    fn resolve_part_state_tracks_sources_and_visibility() {
        let root = resolve_part_state(FlipCardPartStateInput {
            slot: FlipCardSlot::Root,
            disabled: false,
            is_flipped: true,
            flip_on_hover: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_id: true,
        });

        assert_eq!(root.state_attr, "flipped");
        assert_eq!(root.flip_mode_attr, "hover");
        assert_eq!(root.class_source_attr, "custom");
        assert_eq!(root.motion_source_attr, "custom");
        assert_eq!(root.id_source_attr, "custom");
        assert_eq!(root.flip_mode_source_attr, "custom");

        let front = resolve_part_state(FlipCardPartStateInput {
            slot: FlipCardSlot::Front,
            disabled: false,
            is_flipped: true,
            flip_on_hover: false,
            has_custom_class_name: false,
            has_custom_motion: false,
            has_custom_id: false,
        });
        assert_eq!(front.visibility_attr, "hidden");

        let back = resolve_part_state(FlipCardPartStateInput {
            slot: FlipCardSlot::Back,
            disabled: false,
            is_flipped: true,
            flip_on_hover: false,
            has_custom_class_name: false,
            has_custom_motion: false,
            has_custom_id: false,
        });
        assert_eq!(back.visibility_attr, "visible");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-flip-card".to_string()),
            resolve_part_state(FlipCardPartStateInput {
                slot: FlipCardSlot::Root,
                disabled: false,
                is_flipped: true,
                flip_on_hover: true,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_custom_id: true,
            }),
        );

        for token in [
            "ui-flip-card",
            "ui-flip-card--enabled",
            "ui-flip-card--flipped",
            "ui-flip-card--hover",
            "ui-flip-card--custom-class",
            "ui-flip-card--custom-motion",
            "ui-flip-card--custom-id",
            "docs-flip-card",
        ] {
            assert!(
                class_name.contains(token),
                "flip card class name should include `{token}`"
            );
        }
    }

    #[test]
    fn toggle_key_handler_accepts_enter_and_space() {
        assert!(should_toggle_key("Enter", false));
        assert!(should_toggle_key(" ", false));
        assert!(!should_toggle_key("Escape", false));
        assert!(!should_toggle_key("Enter", true));
    }
}
