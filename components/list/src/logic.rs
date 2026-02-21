use leptos::prelude::*;
use std::{collections::HashSet, sync::Arc};
use ui_state_primitives::list as primitives;

pub type ListAccessibleName = primitives::ListAccessibleName;
pub type ListState = primitives::ListViewState;
pub const DEFAULT_LIST_CLASS_NAME: &str = "ui-listbox";
pub const DEFAULT_ID_BASE: &str = "ui-list";
pub const LIST_AGENT_SCHEMA: &str = "ui.list.agent-contract";

pub use primitives::{ListItemSelectionIndicator, ListSectionHeadingTone};

#[derive(Clone)]
pub struct ListSelectionAxisInput {
    pub selected_index: Option<Signal<Option<usize>>>,
    pub default_selected_index: Option<usize>,
    pub on_selected_index_change: Option<Callback<Option<usize>>>,
    pub item_count: usize,
}

#[derive(Clone)]
pub struct ListSelectionAxis {
    pub selected_index: Option<Signal<Option<usize>>>,
    pub default_selected_index: Option<usize>,
    pub on_selected_index_change: Option<Callback<Option<usize>>>,
}

#[derive(Clone)]
pub struct ListOptionsAxisInput {
    pub is_disabled: bool,
    pub disabled_indices: Vec<usize>,
}

#[derive(Clone)]
pub struct ListOptionsAxis {
    pub disabled_indices: Arc<HashSet<usize>>,
    pub has_disabled_options: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListOptionStateInput {
    pub index: usize,
    pub active_index: usize,
    pub selected_index: Option<usize>,
    pub is_disabled_root: bool,
    pub is_disabled_item: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListOptionState {
    pub is_selected: bool,
    pub is_focused: bool,
    pub is_disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListSelectionSourceStateInput {
    pub is_controlled: bool,
    pub has_default_selected_index: bool,
    pub has_on_selected_index_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListSelectionSourceState {
    pub selection_mode_attr: &'static str,
    pub selection_value_source_attr: &'static str,
    pub default_selection_source_attr: &'static str,
    pub selection_change_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ListInteractionSource {
    #[default]
    None,
    Keyboard,
    Pointer,
}

impl ListInteractionSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            ListInteractionSource::None => "none",
            ListInteractionSource::Keyboard => "keyboard",
            ListInteractionSource::Pointer => "pointer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAgentSchemaVersion {
    V1,
}

impl ListAgentSchemaVersion {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ListAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAgentIntent {
    CollectionSelection,
}

impl ListAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ListAgentIntent::CollectionSelection => "collection.selection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAgentAction {
    NavigateSelect,
}

impl ListAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ListAgentAction::NavigateSelect => "navigate.select",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAgentState {
    Empty,
    SelectionEmpty,
    HasSelection,
    Disabled,
}

impl ListAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ListAgentState::Empty => "empty",
            ListAgentState::SelectionEmpty => "selection-empty",
            ListAgentState::HasSelection => "has-selection",
            ListAgentState::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAgentSource {
    Controlled,
    Uncontrolled,
}

impl ListAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ListAgentSource::Controlled => "controlled",
            ListAgentSource::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAgentStreamSupport {
    Optional,
}

impl ListAgentStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ListAgentStreamSupport::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAgentStreamFallback {
    Snapshot,
}

impl ListAgentStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ListAgentStreamFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAgentOutputStatus {
    Verified,
}

impl ListAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ListAgentOutputStatus::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAgentConfigPolicy {
    Whitelist,
}

impl ListAgentConfigPolicy {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ListAgentConfigPolicy::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListAgentContractInput {
    pub state: ListState,
    pub is_disabled: bool,
    pub is_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ListAgentSchemaVersion,
    pub intent: ListAgentIntent,
    pub action: ListAgentAction,
    pub state: ListAgentState,
    pub source: ListAgentSource,
    pub stream_support: ListAgentStreamSupport,
    pub stream_fallback: ListAgentStreamFallback,
    pub output_status: ListAgentOutputStatus,
    pub config_policy: ListAgentConfigPolicy,
}

pub fn resolve_agent_contract(input: ListAgentContractInput) -> ListAgentContract {
    let state = if input.is_disabled {
        ListAgentState::Disabled
    } else if input.state.is_empty {
        ListAgentState::Empty
    } else if input.state.has_selection {
        ListAgentState::HasSelection
    } else {
        ListAgentState::SelectionEmpty
    };

    let source = if input.is_controlled {
        ListAgentSource::Controlled
    } else {
        ListAgentSource::Uncontrolled
    };

    ListAgentContract {
        schema_name: LIST_AGENT_SCHEMA,
        schema_version: ListAgentSchemaVersion::V1,
        intent: ListAgentIntent::CollectionSelection,
        action: ListAgentAction::NavigateSelect,
        state,
        source,
        stream_support: ListAgentStreamSupport::Optional,
        stream_fallback: ListAgentStreamFallback::Snapshot,
        output_status: ListAgentOutputStatus::Verified,
        config_policy: ListAgentConfigPolicy::Whitelist,
    }
}

pub fn resolve_accessible_name(
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
) -> ListAccessibleName {
    primitives::resolve_accessible_name(aria_label, aria_labelledby)
}

pub fn resolve_state(
    item_count: usize,
    selected_index: Option<usize>,
    has_disabled_options: bool,
) -> ListState {
    primitives::resolve_view_state(primitives::ListViewStateInput {
        item_count,
        selected_index,
        has_disabled_options,
    })
}

pub fn normalize_selection_axis(input: ListSelectionAxisInput) -> ListSelectionAxis {
    ListSelectionAxis {
        selected_index: input.selected_index,
        default_selected_index: input
            .default_selected_index
            .filter(|index| *index < input.item_count),
        on_selected_index_change: input.on_selected_index_change,
    }
}

pub fn normalize_id_base(value: Option<String>) -> String {
    primitives::normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

pub fn normalize_list_class_name(value: Option<String>) -> String {
    match primitives::normalize_optional_text(value) {
        Some(custom) => format!("{DEFAULT_LIST_CLASS_NAME} {custom}"),
        None => DEFAULT_LIST_CLASS_NAME.into(),
    }
}

pub fn normalize_options_axis(input: ListOptionsAxisInput) -> ListOptionsAxis {
    let disabled_indices: HashSet<usize> = input.disabled_indices.into_iter().collect();
    let has_disabled_options = input.is_disabled || !disabled_indices.is_empty();

    ListOptionsAxis {
        disabled_indices: Arc::new(disabled_indices),
        has_disabled_options,
    }
}

pub fn resolve_option_state(input: ListOptionStateInput) -> ListOptionState {
    ListOptionState {
        is_selected: input.selected_index == Some(input.index),
        is_focused: input.active_index == input.index,
        is_disabled: input.is_disabled_root || input.is_disabled_item,
    }
}

pub fn resolve_selection_source_state(
    input: ListSelectionSourceStateInput,
) -> ListSelectionSourceState {
    ListSelectionSourceState {
        selection_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        selection_value_source_attr: if input.is_controlled {
            "external"
        } else {
            "internal"
        },
        default_selection_source_attr: if input.has_default_selected_index {
            "provided"
        } else {
            "none"
        },
        selection_change_source_attr: if input.has_on_selected_index_change {
            "provided"
        } else {
            "none"
        },
    }
}

pub fn is_disabled_index(disabled_indices: &HashSet<usize>, index: usize) -> bool {
    disabled_indices.contains(&index)
}

pub(crate) mod item {
    use super::primitives;
    use leptos::prelude::*;
    use std::borrow::Cow;

    #[cfg(test)]
    pub(crate) const DEFAULT_ARIA_LABEL: &str = primitives::DEFAULT_ITEM_ARIA_LABEL;
    pub(crate) const DEFAULT_SELECTED_TEXT: &str = "selected";
    pub(crate) const DEFAULT_UNSELECTED_TEXT: &str = "not selected";

    pub use primitives::{ListItemSelectionIndicator, ListItemState, ListItemStateInput};

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ListItemSelectionStatusText {
        pub selected: String,
        pub unselected: String,
    }

    pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
        primitives::normalize_item_aria_label(value)
    }

    pub fn normalize_class_name(value: Option<String>) -> Option<String> {
        primitives::normalize_optional_text(value)
    }

    pub fn normalize_selection_status_text(
        selected: Option<String>,
        unselected: Option<String>,
    ) -> ListItemSelectionStatusText {
        let selected = primitives::normalize_optional_text(selected)
            .unwrap_or_else(|| DEFAULT_SELECTED_TEXT.into());
        let unselected = primitives::normalize_optional_text(unselected)
            .unwrap_or_else(|| DEFAULT_UNSELECTED_TEXT.into());
        ListItemSelectionStatusText {
            selected,
            unselected,
        }
    }

    pub fn resolve_selection_indicator(
        show_selection_indicator: bool,
    ) -> ListItemSelectionIndicator {
        primitives::resolve_item_selection_indicator(show_selection_indicator)
    }

    pub fn resolve_state(input: ListItemStateInput) -> ListItemState {
        primitives::resolve_item_state(input)
    }

    pub fn normalize_callbacks(
        on_press: Option<Callback<()>>,
        on_pointer_move: Option<Callback<()>>,
    ) -> (Callback<()>, Callback<()>) {
        let on_press = on_press.unwrap_or_else(|| Callback::new(|()| {}));
        let on_pointer_move = on_pointer_move.unwrap_or_else(|| Callback::new(|()| {}));
        (on_press, on_pointer_move)
    }

    pub fn is_interaction_blocked(is_disabled: bool) -> bool {
        is_disabled
    }

    pub fn compose_class_name(base_class_name: Option<String>, state: ListItemState) -> String {
        let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed("ui-listbox-item")];

        if state.is_selected {
            classes.push(Cow::Borrowed("ui-listbox-item--selected"));
        }

        if state.is_focused {
            classes.push(Cow::Borrowed("ui-listbox-item--focused"));
        }

        if state.is_disabled {
            classes.push(Cow::Borrowed("ui-listbox-item--disabled"));
        }

        if state.show_selection_indicator {
            classes.push(Cow::Borrowed("ui-listbox-item--selection-indicator"));
        }

        if state.has_divider {
            classes.push(Cow::Borrowed("ui-listbox-item--divider"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-listbox-item--custom-class"));
            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }

        classes
            .iter()
            .map(|class_name| class_name.as_ref())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[cfg(test)]
    #[path = "tests.rs"]
    mod tests;
}

pub(crate) mod section {
    use super::primitives;
    use std::borrow::Cow;

    #[cfg(test)]
    pub(crate) const DEFAULT_ARIA_LABEL: &str = primitives::DEFAULT_SECTION_ARIA_LABEL;
    pub(crate) const DEFAULT_ITEM_COUNT: usize = 1;

    #[cfg(test)]
    pub type ListSectionHeadingTone = primitives::ListSectionHeadingTone;
    pub use primitives::{ListSectionState, ListSectionStateInput};

    pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
        primitives::normalize_section_aria_label(value)
    }

    pub fn normalize_title(value: Option<String>) -> Option<String> {
        primitives::normalize_section_title(value)
    }

    pub fn normalize_class_name(value: Option<String>) -> Option<String> {
        primitives::normalize_optional_text(value)
    }

    pub fn normalize_item_count(value: Option<usize>) -> usize {
        value.unwrap_or(DEFAULT_ITEM_COUNT)
    }

    pub fn resolve_title_text(value: Option<String>) -> String {
        value.unwrap_or_default()
    }

    pub fn resolve_state(input: ListSectionStateInput) -> ListSectionState {
        primitives::resolve_section_state(input)
    }

    pub fn compose_class_name(base_class_name: Option<String>, state: ListSectionState) -> String {
        let mut classes: Vec<Cow<'static, str>> = vec![
            Cow::Borrowed("ui-listbox-section"),
            Cow::Borrowed(state.heading_tone_class),
        ];

        if state.has_title {
            classes.push(Cow::Borrowed("ui-listbox-section--has-title"));
        }

        if state.is_empty {
            classes.push(Cow::Borrowed("ui-listbox-section--empty"));
        }

        if state.is_disabled {
            classes.push(Cow::Borrowed("ui-listbox-section--disabled"));
        }

        if state.is_sticky_heading {
            classes.push(Cow::Borrowed("ui-listbox-section--sticky-heading"));
        }

        if state.has_divider {
            classes.push(Cow::Borrowed("ui-listbox-section--divided"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-listbox-section--custom-class"));
            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }

        classes
            .iter()
            .map(|class_name| class_name.as_ref())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[cfg(test)]
    #[path = "tests.rs"]
    mod tests;
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
