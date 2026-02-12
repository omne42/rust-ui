use std::collections::BTreeSet;

use crate::context_menu::{
    ContextMenuIds, ContextMenuPartState, ContextMenuPartStateInput, ContextMenuSlot,
};
use ui_headless::PopoverPlacement;

pub const DEFAULT_ID_BASE: &str = "context-menu";
pub const DEFAULT_ARIA_LABEL: &str = "Open context menu";
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_CLOSE_ON_ACTION: bool = true;
pub const DEFAULT_PLACEMENT: PopoverPlacement = PopoverPlacement::BottomStart;

pub fn state_attr(is_open: bool, trigger_disabled: bool) -> &'static str {
    if is_open {
        "open"
    } else if trigger_disabled {
        "disabled"
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

pub fn disabled_attr(trigger_disabled: bool) -> &'static str {
    if trigger_disabled { "true" } else { "false" }
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

pub fn resolve_ids(id_base: &str) -> ContextMenuIds {
    ContextMenuIds {
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

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: ContextMenuPartStateInput) -> ContextMenuPartState {
    let is_empty = input.item_count == 0;
    let has_items = input.item_count > 0;
    let is_enabled = !input.trigger_disabled;
    let keep_open_on_action = !input.close_on_action;
    let is_uncontrolled = !input.is_controlled;

    ContextMenuPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.is_open, input.trigger_disabled),
        item_attr: item_attr(input.item_count),
        disabled_attr: disabled_attr(input.trigger_disabled),
        action_attr: action_attr(input.close_on_action),
        open_mode_attr: open_mode_attr(input.is_controlled),
        placement: input.placement,
        placement_attr: input.placement.as_str(),
        open_attr: input.is_open.then_some("true"),
        closed_attr: (!input.is_open).then_some("true"),
        is_open: input.is_open,
        item_count: input.item_count,
        is_empty,
        has_items,
        is_trigger_disabled: input.trigger_disabled,
        is_enabled,
        close_on_action: input.close_on_action,
        keep_open_on_action,
        is_controlled: input.is_controlled,
        is_uncontrolled,
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

pub fn compose_class_name(base_class_name: Option<String>, state: ContextMenuPartState) -> String {
    let mut classes = vec![state.base_class.to_string()];

    if matches!(state.slot, ContextMenuSlot::Root) {
        classes.push(format!(
            "ui-context-menu--placement-{}",
            state.placement_attr
        ));

        if state.is_open {
            classes.push("ui-context-menu--open".to_string());
        } else {
            classes.push("ui-context-menu--closed".to_string());
        }

        if state.is_trigger_disabled {
            classes.push("ui-context-menu--disabled".to_string());
        } else {
            classes.push("ui-context-menu--enabled".to_string());
        }

        if state.has_items {
            classes.push("ui-context-menu--has-items".to_string());
        } else {
            classes.push("ui-context-menu--empty".to_string());
        }

        if state.keep_open_on_action {
            classes.push("ui-context-menu--persistent".to_string());
        } else {
            classes.push("ui-context-menu--close-on-action".to_string());
        }

        if state.is_controlled {
            classes.push("ui-context-menu--controlled".to_string());
        } else {
            classes.push("ui-context-menu--uncontrolled".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-context-menu--custom-id".to_string());
        }

        if state.has_custom_aria_label {
            classes.push("ui-context-menu--custom-aria-label".to_string());
        }

        if state.has_custom_disabled {
            classes.push("ui-context-menu--custom-disabled".to_string());
        }

        if state.has_custom_disabled_indices {
            classes.push("ui-context-menu--custom-disabled-indices".to_string());
        }

        if state.has_custom_item_kinds {
            classes.push("ui-context-menu--custom-item-kinds".to_string());
        }

        if state.has_custom_close_on_action {
            classes.push("ui-context-menu--custom-close-on-action".to_string());
        }

        if state.has_custom_placement {
            classes.push("ui-context-menu--custom-placement".to_string());
        }

        if state.has_custom_open {
            classes.push("ui-context-menu--custom-open".to_string());
        }

        if state.has_custom_default_open {
            classes.push("ui-context-menu--custom-default-open".to_string());
        }

        if state.has_custom_on_open_change {
            classes.push("ui-context-menu--custom-open-change".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-context-menu--custom-motion".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-context-menu--custom-class".to_string());
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
    use crate::context_menu::{ContextMenuPartStateInput, ContextMenuSlot, MenuOpenFocusStrategy};

    #[test]
    fn menu_id_derives_from_base() {
        let ids = resolve_ids("demo");
        assert_eq!(ids.trigger_id, "demo-trigger");
        assert_eq!(ids.menu_id, "demo-menu");
    }

    #[test]
    fn normalize_id_base_falls_back_when_blank() {
        assert_eq!(
            normalize_id_base("  demo-context-menu  ".to_string()),
            "demo-context-menu"
        );
        assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn disabled_indices_are_deduped_and_clamped_to_item_count() {
        assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
        assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
    }

    #[test]
    fn aria_label_defaults_and_trims() {
        assert_eq!(
            resolve_trigger_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_trigger_aria_label(Some("  Context actions  ".to_string())),
            ("Context actions".to_string(), true)
        );
    }

    #[test]
    fn focus_strategy_for_open_key_maps_context_shortcuts() {
        assert_eq!(
            crate::context_menu::focus_strategy_for_open_key("ContextMenu", false),
            Some(MenuOpenFocusStrategy::First)
        );
        assert_eq!(
            crate::context_menu::focus_strategy_for_open_key("F10", true),
            Some(MenuOpenFocusStrategy::First)
        );
        assert_eq!(
            crate::context_menu::focus_strategy_for_open_key("ArrowUp", false),
            Some(MenuOpenFocusStrategy::Last)
        );
        assert_eq!(
            crate::context_menu::focus_strategy_for_open_key("F10", false),
            None
        );
    }

    #[test]
    fn focus_strategy_default_index() {
        assert_eq!(MenuOpenFocusStrategy::First.default_index(4), 0);
        assert_eq!(MenuOpenFocusStrategy::Last.default_index(4), 3);
        assert_eq!(MenuOpenFocusStrategy::Last.default_index(0), 0);
    }

    #[test]
    fn trigger_disabled_when_component_or_items_disabled() {
        assert!(resolve_trigger_disabled(true, 3));
        assert!(resolve_trigger_disabled(false, 0));
        assert!(!resolve_trigger_disabled(false, 2));
    }

    #[test]
    fn resolve_state_tracks_source_and_open_contracts() {
        let state = resolve_state(ContextMenuPartStateInput {
            slot: ContextMenuSlot::Root,
            is_open: true,
            item_count: 3,
            trigger_disabled: false,
            close_on_action: false,
            placement: PopoverPlacement::TopEnd,
            is_controlled: true,
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

        assert_eq!(state.slot_attr, "context-menu");
        assert_eq!(state.state_attr, "open");
        assert_eq!(state.item_attr, "populated");
        assert_eq!(state.disabled_attr, "false");
        assert_eq!(state.action_attr, "keep-open");
        assert_eq!(state.open_mode_attr, "controlled");
        assert_eq!(state.placement_attr, "top-end");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.aria_label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.disabled_source_attr, "custom");
        assert_eq!(state.disabled_indices_source_attr, "custom");
        assert_eq!(state.item_kinds_source_attr, "custom");
        assert_eq!(state.close_on_action_source_attr, "custom");
        assert_eq!(state.placement_source_attr, "custom");
        assert_eq!(state.open_source_attr, "custom");
        assert_eq!(state.default_open_source_attr, "custom");
        assert_eq!(state.open_change_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ContextMenuPartStateInput {
                slot: ContextMenuSlot::Root,
                is_open: false,
                item_count: 0,
                trigger_disabled: true,
                close_on_action: false,
                placement: PopoverPlacement::BottomStart,
                is_controlled: true,
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
            }),
        );

        for token in [
            "ui-context-menu",
            "ui-context-menu--placement-bottom-start",
            "ui-context-menu--closed",
            "ui-context-menu--disabled",
            "ui-context-menu--empty",
            "ui-context-menu--persistent",
            "ui-context-menu--controlled",
            "ui-context-menu--custom-id",
            "ui-context-menu--custom-aria-label",
            "ui-context-menu--custom-disabled",
            "ui-context-menu--custom-disabled-indices",
            "ui-context-menu--custom-item-kinds",
            "ui-context-menu--custom-close-on-action",
            "ui-context-menu--custom-placement",
            "ui-context-menu--custom-open",
            "ui-context-menu--custom-default-open",
            "ui-context-menu--custom-open-change",
            "ui-context-menu--custom-motion",
            "ui-context-menu--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
