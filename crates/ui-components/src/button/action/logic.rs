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
        value.and_then(|value| {
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        })
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
        disabled: bool,
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
            is_disabled: disabled,
            is_enabled: !disabled,
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
            state.orientation.class_name().to_string(),
            state.density.class_name().to_string(),
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
    mod tests {
        use super::*;

        #[test]
        fn density_class_names_are_stable() {
            assert_eq!(
                ActionButtonGroupDensity::Regular.class_name(),
                "ui-action-button-group--density-regular"
            );
            assert_eq!(ActionButtonGroupDensity::Regular.as_attr(), "regular");
            assert_eq!(
                ActionButtonGroupDensity::Compact.class_name(),
                "ui-action-button-group--density-compact"
            );
            assert_eq!(ActionButtonGroupDensity::Compact.as_attr(), "compact");
        }

        #[test]
        fn orientation_attributes_match_variants() {
            assert_eq!(
                ActionButtonGroupOrientation::Horizontal.aria_orientation(),
                "horizontal"
            );
            assert_eq!(
                ActionButtonGroupOrientation::Horizontal.as_attr(),
                "horizontal"
            );
            assert_eq!(
                ActionButtonGroupOrientation::Vertical.aria_orientation(),
                "vertical"
            );
            assert_eq!(ActionButtonGroupOrientation::Vertical.as_attr(), "vertical");
        }

        #[test]
        fn normalize_optional_text_trims_and_filters_blank_values() {
            assert_eq!(
                normalize_optional_text(Some("  Group  ".to_string())),
                Some("Group".to_string())
            );
            assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
            assert_eq!(normalize_optional_text(None), None);
        }

        #[test]
        fn normalize_aria_label_uses_trimmed_label_or_fallback() {
            let (label, explicit) = normalize_aria_label(Some("  Actions  ".to_string()));
            assert_eq!(label, "Actions");
            assert!(explicit);

            let (label, explicit) = normalize_aria_label(Some("   ".to_string()));
            assert_eq!(label, "Action button group");
            assert!(!explicit);

            let (label, explicit) = normalize_aria_label(None);
            assert_eq!(label, "Action button group");
            assert!(!explicit);
        }

        #[test]
        fn resolve_state_tracks_orientation_density_and_flags() {
            let state = resolve_state(
                ActionButtonGroupOrientation::Vertical,
                ActionButtonGroupDensity::Compact,
                true,
                true,
                true,
                false,
                true,
            );

            assert_eq!(state.orientation_attr, "vertical");
            assert_eq!(state.density_attr, "compact");
            assert!(!state.is_horizontal);
            assert!(state.is_vertical);
            assert!(!state.is_regular);
            assert!(state.is_compact);
            assert!(state.is_justified);
            assert!(!state.is_not_justified);
            assert!(state.is_quiet);
            assert!(!state.is_filled);
            assert!(state.is_disabled);
            assert!(!state.is_enabled);
            assert!(!state.has_explicit_label);
            assert!(state.has_fallback_label);
            assert!(state.has_custom_class_name);
        }

        #[test]
        fn compose_class_name_includes_state_markers() {
            let class_name = compose_class_name(
                Some("custom".to_string()),
                resolve_state(
                    ActionButtonGroupOrientation::Horizontal,
                    ActionButtonGroupDensity::Compact,
                    true,
                    true,
                    true,
                    true,
                    true,
                ),
            );

            for token in [
                "ui-action-button-group",
                "ui-action-button-group--horizontal",
                "ui-action-button-group--density-compact",
                "ui-action-button-group--justified",
                "ui-action-button-group--quiet",
                "ui-action-button-group--disabled",
                "custom",
            ] {
                assert!(
                    class_name.contains(token),
                    "composed class name should include `{token}`"
                );
            }
        }
    }
}

#[cfg(feature = "component-action_group")]
pub(crate) mod action_group_logic {
    use super::super::{ActionGroupItem, ActionGroupState, ActionGroupStateInput};
    use std::collections::BTreeSet;

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
    }

    pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
        value.and_then(|value| {
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        })
    }

    pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
        if let Some(label) = normalize_optional_text(value) {
            return (label, true);
        }

        (DEFAULT_ARIA_LABEL.to_string(), false)
    }

    fn normalize_item(mut item: ActionGroupItem, index: usize) -> ActionGroupItem {
        let fallback_id = format!("action-{}", index + 1);
        item.id = normalize_optional_text(Some(item.id)).unwrap_or(fallback_id);
        item.label = normalize_optional_text(Some(item.label)).unwrap_or_else(|| item.id.clone());
        item
    }

    pub fn normalize_items(items: Vec<ActionGroupItem>) -> Vec<ActionGroupItem> {
        items
            .into_iter()
            .enumerate()
            .map(|(index, item)| normalize_item(item, index))
            .collect()
    }

    pub fn collect_item_ids(items: &[ActionGroupItem]) -> BTreeSet<String> {
        items.iter().map(|item| item.id.clone()).collect()
    }

    pub fn sanitize_selected_ids(
        selected_ids: BTreeSet<String>,
        item_ids: &BTreeSet<String>,
        selection_mode: ActionGroupSelectionMode,
    ) -> BTreeSet<String> {
        let mut selected_ids: BTreeSet<String> = selected_ids
            .into_iter()
            .filter(|id| item_ids.contains(id))
            .collect();

        match selection_mode {
            ActionGroupSelectionMode::None => BTreeSet::new(),
            ActionGroupSelectionMode::Single => {
                if selected_ids.len() <= 1 {
                    return selected_ids;
                }

                let first = selected_ids.iter().next().cloned();
                selected_ids.clear();
                if let Some(first) = first {
                    selected_ids.insert(first);
                }
                selected_ids
            }
            ActionGroupSelectionMode::Multiple => selected_ids,
        }
    }

    pub fn toggle_selected_id(
        selected_ids: BTreeSet<String>,
        id: &str,
        item_ids: &BTreeSet<String>,
        selection_mode: ActionGroupSelectionMode,
    ) -> BTreeSet<String> {
        if !item_ids.contains(id) {
            return selected_ids;
        }

        match selection_mode {
            ActionGroupSelectionMode::None => BTreeSet::new(),
            ActionGroupSelectionMode::Single => {
                let mut next = BTreeSet::new();
                if !selected_ids.contains(id) {
                    next.insert(id.to_string());
                }
                next
            }
            ActionGroupSelectionMode::Multiple => {
                let mut next = selected_ids;
                if !next.insert(id.to_string()) {
                    next.remove(id);
                }
                next
            }
        }
    }

    pub fn resolve_state(input: ActionGroupStateInput) -> ActionGroupState {
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

        let data_state_attr = if input.disabled {
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
            is_disabled: input.disabled,
            item_count: input.item_count,
            selected_count: input.selected_count,
            has_selection,
            is_empty,
            data_state_attr,
            aria_source_attr,
            class_source_attr,
            has_custom_class_name: input.has_custom_class_name,
        }
    }

    pub fn compose_class_name(base_class_name: Option<String>, state: ActionGroupState) -> String {
        let mut classes = vec![
            "ui-action-group".to_string(),
            state.tone_class.to_string(),
            state.selection_mode_class.to_string(),
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
    mod tests {
        use super::super::super::{ActionGroupItem, ActionGroupStateInput};
        use super::*;

        #[test]
        fn tone_and_selection_mode_contracts_are_stable() {
            assert_eq!(
                ActionGroupTone::Default.class_name(),
                "ui-action-group--tone-default"
            );
            assert_eq!(
                ActionGroupTone::Quiet.class_name(),
                "ui-action-group--tone-quiet"
            );
            assert_eq!(
                ActionGroupTone::Strong.class_name(),
                "ui-action-group--tone-strong"
            );

            assert_eq!(
                ActionGroupSelectionMode::Single.class_name(),
                "ui-action-group--mode-single"
            );
            assert_eq!(
                ActionGroupSelectionMode::Multiple.class_name(),
                "ui-action-group--mode-multiple"
            );
            assert_eq!(
                ActionGroupSelectionMode::None.class_name(),
                "ui-action-group--mode-none"
            );
        }

        #[test]
        fn items_and_selection_are_normalized() {
            let items = normalize_items(vec![
                ActionGroupItem::new(" ", " Edit "),
                ActionGroupItem::new("share", " "),
            ]);

            assert_eq!(items[0].id, "action-1");
            assert_eq!(items[0].label, "Edit");
            assert_eq!(items[1].label, "share");

            let item_ids = collect_item_ids(&items);

            let selected = sanitize_selected_ids(
                BTreeSet::from(["action-1".to_string(), "missing".to_string()]),
                &item_ids,
                ActionGroupSelectionMode::Single,
            );
            assert_eq!(selected, BTreeSet::from(["action-1".to_string()]));
        }

        #[test]
        fn toggle_selected_id_respects_selection_mode() {
            let item_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);

            let next = toggle_selected_id(
                BTreeSet::new(),
                "a",
                &item_ids,
                ActionGroupSelectionMode::Single,
            );
            assert_eq!(next, BTreeSet::from(["a".to_string()]));

            let next = toggle_selected_id(next, "a", &item_ids, ActionGroupSelectionMode::Single);
            assert!(next.is_empty());

            let next = toggle_selected_id(
                BTreeSet::new(),
                "a",
                &item_ids,
                ActionGroupSelectionMode::Multiple,
            );
            let next = toggle_selected_id(next, "b", &item_ids, ActionGroupSelectionMode::Multiple);
            assert_eq!(next, BTreeSet::from(["a".to_string(), "b".to_string()]));
        }

        #[test]
        fn resolve_state_and_class_name_track_markers() {
            let state = resolve_state(ActionGroupStateInput {
                tone: ActionGroupTone::Strong,
                selection_mode: ActionGroupSelectionMode::Multiple,
                disabled: false,
                item_count: 3,
                selected_count: 2,
                has_custom_aria_label: true,
                has_custom_class_name: true,
            });

            assert_eq!(state.tone_attr, "strong");
            assert_eq!(state.selection_mode_attr, "multiple");
            assert_eq!(state.data_state_attr, "selected");
            assert_eq!(state.aria_source_attr, "custom");
            assert_eq!(state.class_source_attr, "custom");

            let class_name = compose_class_name(Some("docs-action-group".to_string()), state);
            for token in [
                "ui-action-group",
                "ui-action-group--tone-strong",
                "ui-action-group--mode-multiple",
                "ui-action-group--has-selection",
                "ui-action-group--custom-class",
                "docs-action-group",
            ] {
                assert!(class_name.contains(token), "class should include `{token}`");
            }
        }
    }
}

#[cfg(feature = "component-action_button_group")]
pub use action_button_group_logic::{ActionButtonGroupDensity, ActionButtonGroupOrientation};

#[cfg(feature = "component-action_group")]
pub use action_group_logic::{ActionGroupSelectionMode, ActionGroupTone, DEFAULT_ARIA_LABEL};
