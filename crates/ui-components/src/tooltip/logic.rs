use crate::tooltip::{TooltipPartState, TooltipPartStateInput, TooltipSlot};
use ui_headless::TooltipTriggerMode;

pub const DEFAULT_DELAY_MS: u64 = 1500;
pub const DEFAULT_CLOSE_DELAY_MS: u64 = 500;
pub const DEFAULT_SHOULD_CLOSE_ON_PRESS: bool = true;

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn trigger_attr(trigger: TooltipTriggerMode) -> &'static str {
    match trigger {
        TooltipTriggerMode::Hover => "hover",
        TooltipTriggerMode::Focus => "focus",
    }
}

pub fn press_behavior_attr(should_close_on_press: bool) -> &'static str {
    if should_close_on_press {
        "close"
    } else {
        "persist"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_id(custom_id: Option<String>, fallback_id: String) -> (String, bool) {
    if let Some(custom_id) = normalize_optional_text(custom_id) {
        return (custom_id, true);
    }

    (fallback_id, false)
}

pub fn has_custom_delays(delay_ms: u64, close_delay_ms: u64) -> bool {
    delay_ms != DEFAULT_DELAY_MS || close_delay_ms != DEFAULT_CLOSE_DELAY_MS
}

pub fn resolve_state(input: TooltipPartStateInput) -> TooltipPartState {
    TooltipPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            TooltipSlot::Root => state_attr_for_open(input.open),
            TooltipSlot::Panel => "panel",
        },
        is_open: input.open,
        is_disabled: input.disabled,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_delays: input.has_custom_delays,
        has_custom_trigger_mode: input.has_custom_trigger_mode,
        has_custom_press_behavior: input.has_custom_press_behavior,
        has_custom_id: input.has_custom_id,
        trigger_attr: input.trigger_attr,
        press_behavior_attr: input.press_behavior_attr,
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
        trigger_source_attr: if input.has_custom_trigger_mode {
            "custom"
        } else {
            "default"
        },
        press_source_attr: if input.has_custom_press_behavior {
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

pub fn compose_class_name(base_class_name: Option<String>, state: TooltipPartState) -> String {
    let mut classes = vec![state.base_class.to_string()];

    if state.slot == TooltipSlot::Root {
        if state.has_custom_motion {
            classes.push("ui-tooltip--custom-motion".to_string());
        }

        if state.has_custom_delays {
            classes.push("ui-tooltip--custom-delay".to_string());
        }

        if state.has_custom_trigger_mode {
            classes.push("ui-tooltip--custom-trigger".to_string());
        }

        if state.has_custom_press_behavior {
            classes.push("ui-tooltip--custom-press".to_string());
        }

        if state.has_custom_id {
            classes.push("ui-tooltip--custom-id".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-tooltip--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
    }

    classes.join(" ")
}

pub fn compose_panel_vars(top_px: f64, left_px: f64) -> String {
    format!("--ui-tooltip-top: {top_px}px; --ui-tooltip-left: {left_px}px;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" tooltip ".to_string())),
            Some("tooltip".to_string())
        );
    }

    #[test]
    fn resolve_id_uses_custom_or_generated_paths() {
        assert_eq!(
            resolve_id(Some("docs-tooltip".to_string()), "ui-tooltip-1".to_string()),
            ("docs-tooltip".to_string(), true)
        );
        assert_eq!(
            resolve_id(Some("   ".to_string()), "ui-tooltip-2".to_string()),
            ("ui-tooltip-2".to_string(), false)
        );
        assert_eq!(
            resolve_id(None, "ui-tooltip-3".to_string()),
            ("ui-tooltip-3".to_string(), false)
        );
    }

    #[test]
    fn trigger_and_press_behavior_attrs_match_contract() {
        assert_eq!(trigger_attr(TooltipTriggerMode::Hover), "hover");
        assert_eq!(trigger_attr(TooltipTriggerMode::Focus), "focus");
        assert_eq!(press_behavior_attr(true), "close");
        assert_eq!(press_behavior_attr(false), "persist");
    }

    #[test]
    fn has_custom_delays_detects_non_default_values() {
        assert!(!has_custom_delays(DEFAULT_DELAY_MS, DEFAULT_CLOSE_DELAY_MS));
        assert!(has_custom_delays(
            DEFAULT_DELAY_MS + 1,
            DEFAULT_CLOSE_DELAY_MS
        ));
        assert!(has_custom_delays(
            DEFAULT_DELAY_MS,
            DEFAULT_CLOSE_DELAY_MS + 1
        ));
    }

    #[test]
    fn resolve_state_tracks_source_markers_and_slot_attrs() {
        let state = resolve_state(TooltipPartStateInput {
            slot: TooltipSlot::Root,
            open: true,
            disabled: false,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_delays: true,
            has_custom_trigger_mode: true,
            has_custom_press_behavior: true,
            has_custom_id: true,
            trigger_attr: "focus",
            press_behavior_attr: "persist",
        });

        assert_eq!(state.slot_attr, "tooltip");
        assert_eq!(state.base_class, "ui-tooltip");
        assert_eq!(state.state_attr, "open");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.delay_source_attr, "custom");
        assert_eq!(state.trigger_source_attr, "custom");
        assert_eq!(state.press_source_attr, "custom");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.trigger_attr, "focus");
        assert_eq!(state.press_behavior_attr, "persist");
    }

    #[test]
    fn compose_class_name_includes_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-tooltip".to_string()),
            resolve_state(TooltipPartStateInput {
                slot: TooltipSlot::Root,
                open: false,
                disabled: false,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_custom_delays: true,
                has_custom_trigger_mode: true,
                has_custom_press_behavior: true,
                has_custom_id: true,
                trigger_attr: "focus",
                press_behavior_attr: "persist",
            }),
        );

        for token in [
            "ui-tooltip",
            "ui-tooltip--custom-motion",
            "ui-tooltip--custom-delay",
            "ui-tooltip--custom-trigger",
            "ui-tooltip--custom-press",
            "ui-tooltip--custom-id",
            "ui-tooltip--custom-class",
            "docs-tooltip",
        ] {
            assert!(
                class_name.contains(token),
                "tooltip class name should include `{token}`"
            );
        }
    }

    #[test]
    fn compose_panel_vars_formats_css_custom_properties() {
        assert_eq!(
            compose_panel_vars(18.5, 42.0),
            "--ui-tooltip-top: 18.5px; --ui-tooltip-left: 42px;"
        );
    }
}
