mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_CLOSE_ON_ACTION, DEFAULT_DISABLED, DEFAULT_ID_BASE, DEFAULT_PLACEMENT,
    DEFAULT_TRIGGER_ARIA_LABEL,
};
pub use motion::ActionMenuMotion;
pub use view::ActionMenu;

use crate::MenuItemKind;
use ui_headless::PopoverPlacement;

pub type MenuOpenFocusStrategy = ui_headless::MenuOpenFocusStrategy;

pub fn focus_strategy_for_open_key(key: &str) -> Option<MenuOpenFocusStrategy> {
    ui_headless::menu_trigger_open_focus_strategy_for_key(key)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActionMenuDisabledState {
    #[default]
    Enabled,
    Disabled,
}

impl ActionMenuDisabledState {
    pub fn from_bool(is_disabled: bool) -> Self {
        if is_disabled {
            Self::Disabled
        } else {
            Self::Enabled
        }
    }

    pub fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActionMenuActionMode {
    #[default]
    CloseOnAction,
    KeepOpenOnAction,
}

impl ActionMenuActionMode {
    pub fn from_bool(is_close_on_action: bool) -> Self {
        if is_close_on_action {
            Self::CloseOnAction
        } else {
            Self::KeepOpenOnAction
        }
    }

    pub fn is_close_on_action(self) -> bool {
        matches!(self, Self::CloseOnAction)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMenuItemSpec {
    pub label: String,
    pub kind: MenuItemKind,
    pub is_disabled: bool,
}

impl ActionMenuItemSpec {
    pub fn action(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            kind: MenuItemKind::Action,
            is_disabled: false,
        }
    }

    pub fn with_kind(mut self, kind: MenuItemKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMenuIds {
    pub trigger_id: String,
    pub menu_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionMenuSlot {
    Root,
}

impl ActionMenuSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            ActionMenuSlot::Root => "action-menu",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            ActionMenuSlot::Root => "ui-action-menu",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionMenuPartStateInput {
    pub slot: ActionMenuSlot,
    pub is_open: bool,
    pub item_count: usize,
    pub trigger_disabled: bool,
    pub close_on_action: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
    pub is_controlled: bool,
    pub placement: PopoverPlacement,
    pub has_custom_id_base: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_disabled: bool,
    pub has_custom_disabled_indices: bool,
    pub has_custom_item_kinds: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_placement: bool,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionMenuPartState {
    pub slot: ActionMenuSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub item_attr: &'static str,
    pub action_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub placement: PopoverPlacement,
    pub placement_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub closed_attr: Option<&'static str>,
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_open: bool,
    pub is_trigger_disabled: bool,
    pub is_enabled: bool,
    pub close_on_action: bool,
    pub keep_open_on_action: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_disabled: bool,
    pub has_custom_disabled_indices: bool,
    pub has_custom_item_kinds: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_placement: bool,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_motion: bool,
    pub id_source_attr: &'static str,
    pub aria_label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub disabled_source_attr: &'static str,
    pub disabled_indices_source_attr: &'static str,
    pub item_kinds_source_attr: &'static str,
    pub close_on_action_source_attr: &'static str,
    pub placement_source_attr: &'static str,
    pub open_source_attr: &'static str,
    pub default_open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_strategy_for_open_key_maps_arrow_keys_only() {
        assert_eq!(
            focus_strategy_for_open_key("ArrowDown"),
            Some(MenuOpenFocusStrategy::First)
        );
        assert_eq!(
            focus_strategy_for_open_key("ArrowUp"),
            Some(MenuOpenFocusStrategy::Last)
        );
        assert_eq!(focus_strategy_for_open_key("Enter"), None);
        assert_eq!(focus_strategy_for_open_key(" "), None);
    }

    #[test]
    fn menu_open_focus_strategy_default_index_handles_empty_and_populated_lists() {
        assert_eq!(MenuOpenFocusStrategy::First.default_index(0), 0);
        assert_eq!(MenuOpenFocusStrategy::First.default_index(4), 0);
        assert_eq!(MenuOpenFocusStrategy::Last.default_index(0), 0);
        assert_eq!(MenuOpenFocusStrategy::Last.default_index(4), 3);
    }

    #[test]
    fn discrete_axes_map_to_bool_consistently() {
        assert!(ActionMenuDisabledState::from_bool(true).is_disabled());
        assert!(!ActionMenuDisabledState::from_bool(false).is_disabled());
        assert!(ActionMenuActionMode::from_bool(true).is_close_on_action());
        assert!(!ActionMenuActionMode::from_bool(false).is_close_on_action());
    }

    #[test]
    fn item_spec_builders_keep_item_metadata_in_one_structure() {
        let spec = ActionMenuItemSpec::action("Profile")
            .with_kind(MenuItemKind::Action)
            .with_disabled(true);

        assert_eq!(spec.label, "Profile");
        assert_eq!(spec.kind, MenuItemKind::Action);
        assert!(spec.is_disabled);
    }
}
