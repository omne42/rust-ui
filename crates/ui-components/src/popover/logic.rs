use crate::popover::{PopoverPartState, PopoverPartStateInput, PopoverSlot};

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
mod tests {
    use super::*;

    #[test]
    fn state_and_modal_attrs_follow_contract() {
        assert_eq!(state_attr_for_open(true), "open");
        assert_eq!(state_attr_for_open(false), "closed");
        assert_eq!(modal_attr(true), "modal");
        assert_eq!(modal_attr(false), "non-modal");
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-popover  ".to_string())),
            Some("docs-popover".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_source_markers() {
        let state = resolve_state(PopoverPartStateInput {
            slot: PopoverSlot::Root,
            open: true,
            is_modal: false,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_placement: true,
            has_on_exit_complete: true,
        });

        assert_eq!(state.slot_attr, "popover");
        assert_eq!(state.base_class, "ui-popover");
        assert_eq!(state.state_attr, "open");
        assert_eq!(state.modal_attr, "non-modal");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.placement_source_attr, "custom");
        assert_eq!(state.modal_source_attr, "custom");
        assert_eq!(state.exit_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-popover-state".to_string()),
            resolve_state(PopoverPartStateInput {
                slot: PopoverSlot::Root,
                open: false,
                is_modal: false,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_custom_placement: true,
                has_on_exit_complete: true,
            }),
        );

        for token in [
            "ui-popover",
            "ui-popover--custom-motion",
            "ui-popover--custom-placement",
            "ui-popover--non-modal",
            "ui-popover--custom-modal",
            "ui-popover--custom-exit",
            "ui-popover--custom-class",
            "docs-popover-state",
        ] {
            assert!(
                class_name.contains(token),
                "popover class name should include `{token}`"
            );
        }
    }

    #[test]
    fn compose_panel_vars_formats_css_custom_properties() {
        assert_eq!(
            compose_panel_vars(16.5, 24.0, 320.0),
            "--ui-popover-top: 16.5px; --ui-popover-left: 24px; --ui-popover-anchor-width: 320px;"
        );
    }

    #[test]
    fn should_close_on_escape_requires_topmost_non_composing_non_prevented_escape() {
        assert!(should_close_on_escape("Escape", true, false, false));
        assert!(!should_close_on_escape("Enter", true, false, false));
        assert!(!should_close_on_escape("Escape", false, false, false));
        assert!(!should_close_on_escape("Escape", true, true, false));
        assert!(!should_close_on_escape("Escape", true, false, true));
    }
}
