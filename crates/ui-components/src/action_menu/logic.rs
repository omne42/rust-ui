use std::collections::BTreeSet;

use crate::action_menu::{
    ActionMenuIds, ActionMenuPartState, ActionMenuPartStateInput, ActionMenuSlot,
};
use ui_headless::PopoverPlacement;

pub const DEFAULT_ID_BASE: &str = "action-menu";
pub const DEFAULT_TRIGGER_ARIA_LABEL: &str = "More actions";
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_CLOSE_ON_ACTION: bool = true;
pub const DEFAULT_PLACEMENT: PopoverPlacement = PopoverPlacement::BottomStart;

pub fn state_attr(is_open: bool, trigger_disabled: bool, item_count: usize) -> &'static str {
    if is_open {
        "open"
    } else if trigger_disabled {
        "disabled"
    } else if item_count == 0 {
        "empty"
    } else {
        "closed"
    }
}

pub fn item_attr(item_count: usize) -> &'static str {
    if item_count == 0 {
        "empty"
    } else {
        "populated"
    }
}

pub fn action_attr(close_on_action: bool) -> &'static str {
    if close_on_action {
        "close"
    } else {
        "keep-open"
    }
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
}

pub fn resolve_ids(id_base: &str) -> ActionMenuIds {
    ActionMenuIds {
        trigger_id: format!("{id_base}-trigger"),
        menu_id: format!("{id_base}-menu"),
    }
}

pub fn normalize_disabled_indices(disabled_indices: Vec<usize>, item_count: usize) -> Vec<usize> {
    let mut unique = BTreeSet::new();
    for index in disabled_indices {
        if index < item_count {
            unique.insert(index);
        }
    }
    unique.into_iter().collect()
}

pub fn resolve_trigger_disabled(disabled: bool, item_count: usize) -> bool {
    disabled || item_count == 0
}

pub fn resolve_trigger_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_TRIGGER_ARIA_LABEL.to_string(), false)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: ActionMenuPartStateInput) -> ActionMenuPartState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;

    ActionMenuPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.is_open, input.trigger_disabled, input.item_count),
        item_attr: item_attr(input.item_count),
        action_attr: action_attr(input.close_on_action),
        open_mode_attr: open_mode_attr(input.is_controlled),
        placement: input.placement,
        placement_attr: input.placement.as_str(),
        open_attr: input.is_open.then_some("true"),
        closed_attr: (!input.is_open).then_some("true"),
        item_count: input.item_count,
        is_empty,
        has_items,
        is_open: input.is_open,
        is_trigger_disabled: input.trigger_disabled,
        is_enabled: !input.trigger_disabled,
        close_on_action: input.close_on_action,
        keep_open_on_action: !input.close_on_action,
        has_disabled_items: input.has_disabled_items,
        has_item_kinds: input.has_item_kinds,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_disabled_indices: input.has_custom_disabled_indices,
        has_custom_item_kinds: input.has_custom_item_kinds,
        has_custom_close_on_action: input.has_custom_close_on_action,
        has_custom_placement: input.has_custom_placement,
        has_custom_open: input.has_custom_open,
        has_custom_default_open: input.has_custom_default_open,
        has_custom_on_open_change: input.has_custom_on_open_change,
        has_custom_motion: input.has_custom_motion,
        id_source_attr: source_attr(input.has_custom_id_base),
        aria_label_source_attr: source_attr(input.has_custom_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name),
        disabled_source_attr: source_attr(input.has_custom_disabled),
        disabled_indices_source_attr: source_attr(input.has_custom_disabled_indices),
        item_kinds_source_attr: source_attr(input.has_custom_item_kinds),
        close_on_action_source_attr: source_attr(input.has_custom_close_on_action),
        placement_source_attr: source_attr(input.has_custom_placement),
        open_source_attr: source_attr(input.has_custom_open),
        default_open_source_attr: source_attr(input.has_custom_default_open),
        open_change_source_attr: source_attr(input.has_custom_on_open_change),
        motion_source_attr: source_attr(input.has_custom_motion),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ActionMenuPartState) -> String {
    let mut classes = vec![
        state.base_class.to_string(),
        format!("ui-action-menu--placement-{}", state.placement_attr),
    ];

    if matches!(state.slot, ActionMenuSlot::Root) {
        if state.is_open {
            classes.push("ui-action-menu--open".to_string());
        } else {
            classes.push("ui-action-menu--closed".to_string());
        }

        if state.is_trigger_disabled {
            classes.push("ui-action-menu--disabled".to_string());
        } else {
            classes.push("ui-action-menu--enabled".to_string());
        }

        if state.is_empty {
            classes.push("ui-action-menu--empty".to_string());
        } else {
            classes.push("ui-action-menu--has-items".to_string());
        }

        if state.keep_open_on_action {
            classes.push("ui-action-menu--persistent".to_string());
        } else {
            classes.push("ui-action-menu--close-on-action".to_string());
        }

        if state.is_controlled {
            classes.push("ui-action-menu--controlled".to_string());
        } else {
            classes.push("ui-action-menu--uncontrolled".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-action-menu--custom-id".to_string());
        }

        if state.has_custom_aria_label {
            classes.push("ui-action-menu--custom-aria-label".to_string());
        }

        if state.has_custom_disabled {
            classes.push("ui-action-menu--custom-disabled".to_string());
        }

        if state.has_custom_disabled_indices {
            classes.push("ui-action-menu--custom-disabled-indices".to_string());
        }

        if state.has_custom_item_kinds {
            classes.push("ui-action-menu--custom-item-kinds".to_string());
        }

        if state.has_custom_close_on_action {
            classes.push("ui-action-menu--custom-close-on-action".to_string());
        }

        if state.has_custom_placement {
            classes.push("ui-action-menu--custom-placement".to_string());
        }

        if state.has_custom_open {
            classes.push("ui-action-menu--custom-open".to_string());
        }

        if state.has_custom_default_open {
            classes.push("ui-action-menu--custom-default-open".to_string());
        }

        if state.has_custom_on_open_change {
            classes.push("ui-action-menu--custom-open-change".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-action-menu--custom-motion".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-action-menu--custom-class".to_string());
            if let Some(base_class_name) = normalize_optional_text(base_class_name) {
                classes.push(base_class_name);
            }
        }
    } else if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_menu::{ActionMenuPartStateInput, ActionMenuSlot};

    #[test]
    fn ids_include_menu_suffix() {
        let ids = resolve_ids("demo");
        assert_eq!(ids.trigger_id, "demo-trigger");
        assert_eq!(ids.menu_id, "demo-menu");
    }

    #[test]
    fn normalize_id_base_falls_back_when_blank() {
        assert_eq!(normalize_id_base("  demo-menu  ".to_string()), "demo-menu");
        assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn aria_label_defaults_and_trims() {
        assert_eq!(
            resolve_trigger_aria_label(None),
            (DEFAULT_TRIGGER_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_trigger_aria_label(Some("  More  ".to_string())),
            ("More".to_string(), true)
        );
    }

    #[test]
    fn disabled_indices_are_deduped_and_clamped_to_item_count() {
        assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
        assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
    }

    #[test]
    fn state_helpers_remain_stable() {
        assert_eq!(state_attr(true, false, 3), "open");
        assert_eq!(state_attr(false, true, 3), "disabled");
        assert_eq!(state_attr(false, false, 0), "empty");
        assert_eq!(state_attr(false, false, 3), "closed");

        assert_eq!(item_attr(0), "empty");
        assert_eq!(item_attr(2), "populated");
        assert_eq!(action_attr(true), "close");
        assert_eq!(action_attr(false), "keep-open");
        assert_eq!(open_mode_attr(true), "controlled");
        assert_eq!(open_mode_attr(false), "uncontrolled");
    }

    #[test]
    fn resolve_state_sets_source_markers_and_flags() {
        let state = resolve_state(ActionMenuPartStateInput {
            slot: ActionMenuSlot::Root,
            is_open: true,
            item_count: 3,
            trigger_disabled: false,
            close_on_action: false,
            has_disabled_items: true,
            has_item_kinds: true,
            is_controlled: true,
            placement: PopoverPlacement::BottomStart,
            has_custom_id_base: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_disabled: true,
            has_custom_disabled_indices: true,
            has_custom_item_kinds: true,
            has_custom_close_on_action: true,
            has_custom_placement: true,
            has_custom_open: true,
            has_custom_default_open: false,
            has_custom_on_open_change: true,
            has_custom_motion: true,
        });

        assert_eq!(state.state_attr, "open");
        assert_eq!(state.action_attr, "keep-open");
        assert_eq!(state.open_mode_attr, "controlled");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.default_open_source_attr, "default");
        assert_eq!(state.motion_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_contains_stable_tokens() {
        let state = resolve_state(ActionMenuPartStateInput {
            slot: ActionMenuSlot::Root,
            is_open: false,
            item_count: 3,
            trigger_disabled: false,
            close_on_action: false,
            has_disabled_items: true,
            has_item_kinds: true,
            is_controlled: true,
            placement: PopoverPlacement::BottomEnd,
            has_custom_id_base: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_disabled: true,
            has_custom_disabled_indices: true,
            has_custom_item_kinds: true,
            has_custom_close_on_action: true,
            has_custom_placement: true,
            has_custom_open: true,
            has_custom_default_open: true,
            has_custom_on_open_change: true,
            has_custom_motion: true,
        });
        let class_name = compose_class_name(Some("docs-action-menu".to_string()), state);

        for token in [
            "ui-action-menu",
            "ui-action-menu--placement-bottom-end",
            "ui-action-menu--closed",
            "ui-action-menu--has-items",
            "ui-action-menu--persistent",
            "ui-action-menu--controlled",
            "ui-action-menu--custom-id",
            "ui-action-menu--custom-aria-label",
            "ui-action-menu--custom-disabled",
            "ui-action-menu--custom-disabled-indices",
            "ui-action-menu--custom-item-kinds",
            "ui-action-menu--custom-close-on-action",
            "ui-action-menu--custom-placement",
            "ui-action-menu--custom-open",
            "ui-action-menu--custom-default-open",
            "ui-action-menu--custom-open-change",
            "ui-action-menu--custom-motion",
            "docs-action-menu",
        ] {
            assert!(
                class_name.contains(token),
                "composed class list should include `{token}`"
            );
        }
    }
}
