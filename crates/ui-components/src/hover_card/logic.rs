use crate::hover_card::{HoverCardPartState, HoverCardPartStateInput, HoverCardSlot};

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
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  hover-card  ".to_string())),
            Some("hover-card".to_string())
        );
    }

    #[test]
    fn resolve_id_uses_custom_or_generated_paths() {
        assert_eq!(
            resolve_id(
                Some(" docs-hover-card ".to_string()),
                "ui-hover-card-1".to_string()
            ),
            ("docs-hover-card".to_string(), true)
        );
        assert_eq!(
            resolve_id(Some("   ".to_string()), "ui-hover-card-2".to_string()),
            ("ui-hover-card-2".to_string(), false)
        );
        assert_eq!(
            resolve_id(None, "ui-hover-card-3".to_string()),
            ("ui-hover-card-3".to_string(), false)
        );
    }

    #[test]
    fn delay_source_detection_matches_default_contract() {
        assert!(!has_custom_delays(
            DEFAULT_OPEN_DELAY_MS,
            DEFAULT_CLOSE_DELAY_MS
        ));
        assert!(has_custom_delays(
            DEFAULT_OPEN_DELAY_MS + 1,
            DEFAULT_CLOSE_DELAY_MS
        ));
        assert!(has_custom_delays(
            DEFAULT_OPEN_DELAY_MS,
            DEFAULT_CLOSE_DELAY_MS + 1
        ));
    }

    #[test]
    fn resolve_part_state_tracks_slot_and_source_markers() {
        let root = resolve_part_state(HoverCardPartStateInput {
            slot: HoverCardSlot::Root,
            open: true,
            disabled: false,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_delays: true,
            has_custom_id: true,
        });

        assert_eq!(root.slot_attr, "hover-card");
        assert_eq!(root.base_class, "ui-hover-card");
        assert_eq!(root.state_attr, "open");
        assert_eq!(root.class_source_attr, "custom");
        assert_eq!(root.motion_source_attr, "custom");
        assert_eq!(root.delay_source_attr, "custom");
        assert_eq!(root.id_source_attr, "custom");

        let trigger = resolve_part_state(HoverCardPartStateInput {
            slot: HoverCardSlot::Trigger,
            open: false,
            disabled: true,
            has_custom_class_name: false,
            has_custom_motion: false,
            has_custom_delays: false,
            has_custom_id: false,
        });
        assert_eq!(trigger.state_attr, "trigger");
        assert_eq!(trigger.motion_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-hover-card".to_string()),
            resolve_part_state(HoverCardPartStateInput {
                slot: HoverCardSlot::Root,
                open: false,
                disabled: false,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_custom_delays: true,
                has_custom_id: true,
            }),
        );

        for token in [
            "ui-hover-card",
            "ui-hover-card--custom-motion",
            "ui-hover-card--custom-delay",
            "ui-hover-card--custom-id",
            "ui-hover-card--custom-class",
            "docs-hover-card",
        ] {
            assert!(
                class_name.contains(token),
                "hover card class name should include `{token}`"
            );
        }
    }

    #[test]
    fn compose_panel_vars_generates_css_variables_only() {
        let vars = compose_panel_vars(12.5, 24.0, 220.0);
        assert_eq!(
            vars,
            "--ui-hover-card-top: 12.5px; --ui-hover-card-left: 24px; --ui-hover-card-anchor-width: 220px;"
        );
    }

    #[test]
    fn should_handle_escape_requires_open_non_composing_escape_key() {
        assert!(should_handle_escape("Escape", true, false));
        assert!(!should_handle_escape("Enter", true, false));
        assert!(!should_handle_escape("Escape", false, false));
        assert!(!should_handle_escape("Escape", true, true));
    }
}
