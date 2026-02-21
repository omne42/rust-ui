pub(crate) mod action_button_logic {
    use super::super::{ActionButtonSize, ActionButtonType};
    use crate::button::ButtonVariant;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActionButtonInputResolutionInput {
        pub is_disabled: Option<bool>,
        pub inherited_disabled: Option<bool>,
        pub size: Option<ActionButtonSize>,
        pub inherited_size: Option<ActionButtonSize>,
        pub is_quiet: Option<bool>,
        pub inherited_quiet: Option<bool>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActionButtonResolvedInput {
        pub is_disabled: bool,
        pub size: ActionButtonSize,
        pub is_quiet: bool,
        pub variant: ButtonVariant,
    }

    pub fn resolve_input(input: ActionButtonInputResolutionInput) -> ActionButtonResolvedInput {
        let is_disabled = input
            .is_disabled
            .or(input.inherited_disabled)
            .unwrap_or(false);
        let size = input.size.or(input.inherited_size).unwrap_or_default();
        let is_quiet = input.is_quiet.or(input.inherited_quiet).unwrap_or(false);
        let variant = if is_quiet {
            ButtonVariant::Ghost
        } else {
            ButtonVariant::Default
        };

        ActionButtonResolvedInput {
            is_disabled,
            size,
            is_quiet,
            variant,
        }
    }

    pub fn resolve_button_type(button_type: Option<ActionButtonType>) -> ActionButtonType {
        button_type.unwrap_or_default()
    }

    #[cfg(test)]
    #[path = "tests.rs"]
    mod tests;
}

#[cfg(feature = "component-action_button_group")]
pub(crate) mod action_button_group_logic {
    use crate::button::action::ActionButtonSize;
    use leptos::prelude::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum ActionButtonGroupOrientation {
        #[default]
        Horizontal,
        Vertical,
    }

    impl ActionButtonGroupOrientation {
        pub fn class_name(self) -> &'static str {
            match self {
                ActionButtonGroupOrientation::Horizontal => "ui-action-button-group--horizontal",
                ActionButtonGroupOrientation::Vertical => "ui-action-button-group--vertical",
            }
        }

        pub fn as_attr(self) -> &'static str {
            match self {
                ActionButtonGroupOrientation::Horizontal => "horizontal",
                ActionButtonGroupOrientation::Vertical => "vertical",
            }
        }

        pub fn aria_orientation(self) -> &'static str {
            self.as_attr()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum ActionButtonGroupDensity {
        #[default]
        Regular,
        Compact,
    }

    impl ActionButtonGroupDensity {
        pub fn class_name(self) -> &'static str {
            match self {
                ActionButtonGroupDensity::Regular => "ui-action-button-group--density-regular",
                ActionButtonGroupDensity::Compact => "ui-action-button-group--density-compact",
            }
        }

        pub fn as_attr(self) -> &'static str {
            match self {
                ActionButtonGroupDensity::Regular => "regular",
                ActionButtonGroupDensity::Compact => "compact",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActionButtonGroupContextValue {
        pub size: ActionButtonSize,
        pub density: ActionButtonGroupDensity,
        pub orientation: ActionButtonGroupOrientation,
        pub is_justified: bool,
        pub is_quiet: bool,
        pub is_disabled: bool,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActionButtonGroupState {
        pub orientation: ActionButtonGroupOrientation,
        pub orientation_attr: &'static str,
        pub density: ActionButtonGroupDensity,
        pub density_attr: &'static str,
        pub is_horizontal: bool,
        pub is_vertical: bool,
        pub is_regular: bool,
        pub is_compact: bool,
        pub is_justified: bool,
        pub is_not_justified: bool,
        pub is_quiet: bool,
        pub is_filled: bool,
        pub is_disabled: bool,
        pub is_enabled: bool,
        pub has_explicit_label: bool,
        pub has_fallback_label: bool,
        pub has_custom_class_name: bool,
    }

    pub(crate) fn use_action_button_group_context() -> Option<ActionButtonGroupContextValue> {
        use_context::<ActionButtonGroupContextValue>()
    }

    pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
        ui_state_primitives::button::normalize_optional_text(value)
    }

    pub fn normalize_aria_label(aria_label: Option<String>) -> (String, bool) {
        if let Some(label) = normalize_optional_text(aria_label) {
            return (label, true);
        }

        ("Action button group".to_string(), false)
    }

    pub fn resolve_state(
        orientation: ActionButtonGroupOrientation,
        density: ActionButtonGroupDensity,
        is_justified: bool,
        is_quiet: bool,
        is_disabled: bool,
        has_explicit_label: bool,
        has_custom_class_name: bool,
    ) -> ActionButtonGroupState {
        ActionButtonGroupState {
            orientation,
            orientation_attr: orientation.as_attr(),
            density,
            density_attr: density.as_attr(),
            is_horizontal: matches!(orientation, ActionButtonGroupOrientation::Horizontal),
            is_vertical: matches!(orientation, ActionButtonGroupOrientation::Vertical),
            is_regular: matches!(density, ActionButtonGroupDensity::Regular),
            is_compact: matches!(density, ActionButtonGroupDensity::Compact),
            is_justified,
            is_not_justified: !is_justified,
            is_quiet,
            is_filled: !is_quiet,
            is_disabled,
            is_enabled: !is_disabled,
            has_explicit_label,
            has_fallback_label: !has_explicit_label,
            has_custom_class_name,
        }
    }

    pub fn compose_class_name(
        base_class_name: Option<String>,
        state: ActionButtonGroupState,
    ) -> String {
        let mut classes = vec![
            "ui-action-button-group".to_string(),
            state.orientation.class_name().into(),
            state.density.class_name().into(),
        ];

        if state.is_justified {
            classes.push("ui-action-button-group--justified".to_string());
        }
        if state.is_quiet {
            classes.push("ui-action-button-group--quiet".to_string());
        }
        if state.is_disabled {
            classes.push("ui-action-button-group--disabled".to_string());
        }

        if state.has_custom_class_name
            && let Some(base_class_name) = base_class_name
        {
            classes.push(base_class_name);
        }

        classes.join(" ")
    }

    #[cfg(test)]
    #[path = "tests.rs"]
    mod tests;
}

#[cfg(feature = "component-action_group")]
pub(crate) mod action_group_logic {
    use super::super::{ActionGroupItem, ActionGroupState, ActionGroupStateInput};
    use std::collections::BTreeSet;
    use ui_state_primitives::action_group as action_group_state;

    pub const DEFAULT_ARIA_LABEL: &str = "Action group";

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum ActionGroupTone {
        #[default]
        Default,
        Quiet,
        Strong,
    }

    impl ActionGroupTone {
        pub fn class_name(self) -> &'static str {
            match self {
                ActionGroupTone::Default => "ui-action-group--tone-default",
                ActionGroupTone::Quiet => "ui-action-group--tone-quiet",
                ActionGroupTone::Strong => "ui-action-group--tone-strong",
            }
        }

        pub fn as_attr(self) -> &'static str {
            match self {
                ActionGroupTone::Default => "default",
                ActionGroupTone::Quiet => "quiet",
                ActionGroupTone::Strong => "strong",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum ActionGroupSelectionMode {
        #[default]
        Single,
        Multiple,
        None,
    }

    impl ActionGroupSelectionMode {
        pub fn class_name(self) -> &'static str {
            match self {
                ActionGroupSelectionMode::Single => "ui-action-group--mode-single",
                ActionGroupSelectionMode::Multiple => "ui-action-group--mode-multiple",
                ActionGroupSelectionMode::None => "ui-action-group--mode-none",
            }
        }

        pub fn as_attr(self) -> &'static str {
            match self {
                ActionGroupSelectionMode::Single => "single",
                ActionGroupSelectionMode::Multiple => "multiple",
                ActionGroupSelectionMode::None => "none",
            }
        }

        fn as_state_primitive(self) -> action_group_state::ActionGroupSelectionMode {
            match self {
                ActionGroupSelectionMode::Single => {
                    action_group_state::ActionGroupSelectionMode::Single
                }
                ActionGroupSelectionMode::Multiple => {
                    action_group_state::ActionGroupSelectionMode::Multiple
                }
                ActionGroupSelectionMode::None => {
                    action_group_state::ActionGroupSelectionMode::None
                }
            }
        }
    }

    pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
        action_group_state::normalize_optional_text(value)
    }

    pub fn normalize_aria_label(
        value: Option<String>,
        fallback_aria_label: &str,
    ) -> (String, bool) {
        if let Some(label) = normalize_optional_text(value) {
            return (label, true);
        }

        // (fallback_aria_label.to_string(), false)
        (fallback_aria_label.into(), false)
    }

    pub fn normalize_items(items: Vec<ActionGroupItem>) -> Vec<ActionGroupItem> {
        action_group_state::normalize_items(items)
    }

    pub fn collect_item_ids(items: &[ActionGroupItem]) -> BTreeSet<String> {
        action_group_state::collect_item_ids(items.iter().map(|item| item.id.as_str()))
    }

    pub fn sanitize_selected_ids(
        selected_ids: BTreeSet<String>,
        item_ids: &BTreeSet<String>,
        selection_mode: ActionGroupSelectionMode,
    ) -> BTreeSet<String> {
        action_group_state::sanitize_selected_ids(
            selected_ids,
            item_ids,
            selection_mode.as_state_primitive(),
        )
    }

    pub fn resolve_selected_ids(
        selected_ids: BTreeSet<String>,
        item_ids: &BTreeSet<String>,
        selection_mode: ActionGroupSelectionMode,
    ) -> BTreeSet<String> {
        sanitize_selected_ids(selected_ids, item_ids, selection_mode)
    }

    pub fn normalize_default_selected_ids(
        default_selected_ids: Option<BTreeSet<String>>,
        item_ids: &BTreeSet<String>,
        selection_mode: ActionGroupSelectionMode,
    ) -> BTreeSet<String> {
        resolve_selected_ids(
            default_selected_ids.unwrap_or_default(),
            item_ids,
            selection_mode,
        )
    }

    pub fn toggle_selected_id(
        selected_ids: BTreeSet<String>,
        id: &str,
        item_ids: &BTreeSet<String>,
        selection_mode: ActionGroupSelectionMode,
    ) -> BTreeSet<String> {
        action_group_state::toggle_selected_id(
            selected_ids,
            id,
            item_ids,
            selection_mode.as_state_primitive(),
        )
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ActionGroupItemRenderState {
        pub is_disabled: bool,
        pub is_selected: bool,
        pub class_name: String,
    }

    pub fn resolve_item_render_state(
        is_group_disabled: bool,
        is_item_disabled: bool,
        is_selected: bool,
    ) -> ActionGroupItemRenderState {
        let is_disabled = is_group_disabled || is_item_disabled;
        let mut class_name = String::from("ui-action-group__item");

        if is_selected {
            class_name.push_str(" ui-action-group__item--selected");
        }
        if is_disabled {
            class_name.push_str(" ui-action-group__item--disabled");
        }

        ActionGroupItemRenderState {
            is_disabled,
            is_selected,
            class_name,
        }
    }

    pub fn resolve_next_selected_ids(
        current_selected_ids: BTreeSet<String>,
        id: &str,
        item_ids: &BTreeSet<String>,
        selection_mode: ActionGroupSelectionMode,
        is_item_disabled: bool,
    ) -> Option<BTreeSet<String>> {
        if is_item_disabled {
            return None;
        }

        let selected_ids = resolve_selected_ids(current_selected_ids, item_ids, selection_mode);
        Some(toggle_selected_id(
            selected_ids,
            id,
            item_ids,
            selection_mode,
        ))
    }

    pub fn resolve_state(input: ActionGroupStateInput) -> ActionGroupState {
        let selection_source_attr = if input.is_selection_controlled {
            "controlled"
        } else {
            "uncontrolled"
        };
        let aria_source_attr = if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        };
        let class_source_attr = if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        };

        let is_empty = input.item_count == 0;
        let has_selection = input.selected_count > 0;

        let data_state_attr = if input.is_disabled {
            "disabled"
        } else if is_empty {
            "empty"
        } else if has_selection {
            "selected"
        } else {
            "default"
        };

        ActionGroupState {
            tone: input.tone,
            tone_class: input.tone.class_name(),
            tone_attr: input.tone.as_attr(),
            selection_mode: input.selection_mode,
            selection_mode_class: input.selection_mode.class_name(),
            selection_mode_attr: input.selection_mode.as_attr(),
            is_disabled: input.is_disabled,
            item_count: input.item_count,
            selected_count: input.selected_count,
            has_selection,
            is_empty,
            data_state_attr,
            selection_source_attr,
            aria_source_attr,
            class_source_attr,
            has_custom_class_name: input.has_custom_class_name,
        }
    }

    pub fn compose_class_name(base_class_name: Option<String>, state: ActionGroupState) -> String {
        let mut classes = vec![
            "ui-action-group".to_string(),
            state.tone_class.into(),
            state.selection_mode_class.into(),
        ];

        if state.is_disabled {
            classes.push("ui-action-group--disabled".to_string());
        }
        if state.has_selection {
            classes.push("ui-action-group--has-selection".to_string());
        }
        if state.is_empty {
            classes.push("ui-action-group--empty".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-action-group--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }

        classes.join(" ")
    }

    #[cfg(test)]
    #[path = "tests.rs"]
    mod tests;
}

#[cfg(feature = "component-action_button_group")]
pub use action_button_group_logic::{ActionButtonGroupDensity, ActionButtonGroupOrientation};

#[cfg(feature = "component-action_group")]
pub use action_group_logic::{ActionGroupSelectionMode, ActionGroupTone, DEFAULT_ARIA_LABEL};
