use std::borrow::Cow;

use leptos::prelude::*;

pub use ui_state_primitives::autocomplete::{
    AutocompleteInputEvent, AutocompleteInputState, AutocompleteState, AutocompleteStateInput,
    filter_indices, map_filtered_to_original, map_selected_to_filtered, normalize_disabled_indices,
    normalize_id_base, normalize_label, normalize_optional_text, reduce_input_state,
    resolve_empty_message, resolve_placeholder, resolve_state,
};

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_required: Option<Signal<bool>>,
    pub required: Option<Signal<bool>>,
    pub is_invalid: Option<Signal<bool>>,
    pub invalid: Option<Signal<bool>>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub required: Signal<bool>,
    pub invalid: Signal<bool>,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    let required = input
        .is_required
        .or(input.required)
        .unwrap_or_else(|| Signal::derive(|| false));
    let invalid = input
        .is_invalid
        .or(input.invalid)
        .unwrap_or_else(|| Signal::derive(|| false));

    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
        required,
        invalid,
    }
}

pub struct OpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

pub struct OpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_controlled: bool,
}

pub fn normalize_open_state(input: OpenStateInput) -> OpenState {
    let open = input.is_open.or(input.open);
    OpenState {
        is_controlled: open.is_some(),
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
    }
}

pub struct SelectionChangeInput {
    pub selected_index: Option<Signal<Option<usize>>>,
    pub default_selected_index: Option<usize>,
    pub on_selected_index_change: Option<Callback<Option<usize>>>,
    pub set_selected_index: Option<WriteSignal<Option<usize>>>,
    pub item_count: usize,
}

pub struct SelectionChange {
    pub selected_index: Option<Signal<Option<usize>>>,
    pub default_selected_index: Option<usize>,
    pub on_selected_index_change: Option<Callback<Option<usize>>>,
    pub is_controlled: bool,
    pub selected_source: SelectedSource,
    pub change_source: SelectedChangeSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectedSource {
    SelectedIndex,
    DefaultSelectedIndex,
}

impl SelectedSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::SelectedIndex => "selected_index",
            Self::DefaultSelectedIndex => "default_selected_index",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectedChangeSource {
    OnSelectedIndexChange,
    SetSelectedIndex,
    None,
}

impl SelectedChangeSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::OnSelectedIndexChange => "on_selected_index_change",
            Self::SetSelectedIndex => "set_selected_index",
            Self::None => "none",
        }
    }
}

pub fn normalize_selection_change(input: SelectionChangeInput) -> SelectionChange {
    let default_selected_index = input
        .default_selected_index
        .filter(|&index| index < input.item_count);

    let (on_selected_index_change, change_source) =
        if let Some(on_selected_index_change) = input.on_selected_index_change {
            (
                Some(on_selected_index_change),
                SelectedChangeSource::OnSelectedIndexChange,
            )
        } else if let Some(set_selected_index) = input.set_selected_index {
            (
                Some(Callback::new(move |next| set_selected_index.set(next))),
                SelectedChangeSource::SetSelectedIndex,
            )
        } else {
            (None, SelectedChangeSource::None)
        };

    let is_controlled = input.selected_index.is_some();
    let selected_source = if is_controlled {
        SelectedSource::SelectedIndex
    } else {
        SelectedSource::DefaultSelectedIndex
    };

    SelectionChange {
        selected_index: input.selected_index,
        default_selected_index,
        on_selected_index_change,
        is_controlled,
        selected_source,
        change_source,
    }
}

pub struct InputStateSource {
    pub query: String,
    pub has_typed: bool,
}

impl InputStateSource {
    fn into_input_state(self) -> AutocompleteInputState {
        AutocompleteInputState {
            query: self.query,
            has_typed: self.has_typed,
        }
    }
}

pub fn reduce_sync_from_selection(
    source: InputStateSource,
    selected_label: Option<String>,
) -> AutocompleteInputState {
    reduce_input_state(
        source.into_input_state(),
        AutocompleteInputEvent::SyncFromSelection { selected_label },
    )
}

pub fn reduce_after_option_commit(
    source: InputStateSource,
    selected_label: String,
) -> AutocompleteInputState {
    reduce_input_state(
        source.into_input_state(),
        AutocompleteInputEvent::OptionCommitted { selected_label },
    )
}

pub fn reduce_after_input_blur(source: InputStateSource) -> AutocompleteInputState {
    reduce_input_state(
        source.into_input_state(),
        AutocompleteInputEvent::InputBlurred,
    )
}

pub fn reduce_after_input_change(
    source: InputStateSource,
    query: String,
) -> AutocompleteInputState {
    reduce_input_state(
        source.into_input_state(),
        AutocompleteInputEvent::InputChanged { query },
    )
}

pub fn resolve_id_base(id_base: String, generated_id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or(generated_id_base)
}

pub struct RootStateInput {
    pub id_base: String,
    pub has_custom_id_base: bool,
    pub label: String,
    pub placeholder: Option<String>,
    pub empty_message: Option<String>,
    pub i18n_empty_message: Option<String>,
    pub description: Option<String>,
    pub error: Option<String>,
    pub class_name: Option<String>,
    pub item_count: usize,
    pub disabled_indices: Vec<usize>,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub has_custom_motion: bool,
}

pub struct RootState {
    pub id_base: String,
    pub label: String,
    pub placeholder: String,
    pub empty_message: String,
    pub description: Option<String>,
    pub error: Option<String>,
    pub class_name: String,
    pub disabled_indices: Vec<usize>,
    pub state: AutocompleteState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RootDataState {
    Open,
    Disabled,
    Closed,
}

impl RootDataState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Disabled => "disabled",
            Self::Closed => "closed",
        }
    }
}

pub fn resolve_root_data_state(is_open: bool, is_disabled: bool) -> RootDataState {
    if is_open {
        RootDataState::Open
    } else if is_disabled {
        RootDataState::Disabled
    } else {
        RootDataState::Closed
    }
}

pub const AUTOCOMPLETE_AGENT_SCHEMA: &str = "ui.autocomplete.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentSchemaVersion {
    V1,
}

impl AutocompleteAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentIntent {
    SuggestAndSelect,
}

impl AutocompleteAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuggestAndSelect => "autocomplete.suggest-and-select",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentAction {
    Idle,
    Query,
    CommitSelection,
}

impl AutocompleteAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Query => "query",
            Self::CommitSelection => "commit-selection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentState {
    Open,
    Closed,
    Disabled,
}

impl AutocompleteAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentSource {
    StatePrimitives,
}

impl AutocompleteAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentOutputStatus {
    Verified,
}

impl AutocompleteAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentStreamSupport {
    Unsupported,
}

impl AutocompleteAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentStreamFallback {
    Snapshot,
}

impl AutocompleteAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentStreamMode {
    Streaming,
    Snapshot,
}

impl AutocompleteAgentStreamMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::Snapshot => "snapshot",
        }
    }
}
const _: [AutocompleteAgentStreamMode; 2] = [
    AutocompleteAgentStreamMode::Streaming,
    AutocompleteAgentStreamMode::Snapshot,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocompleteAgentOpenValueSource {
    Controlled,
    Uncontrolled,
}

impl AutocompleteAgentOpenValueSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutocompleteAgentContract {
    pub schema_name: &'static str,
    pub schema_version: AutocompleteAgentSchemaVersion,
    pub intent: AutocompleteAgentIntent,
    pub action: AutocompleteAgentAction,
    pub state: AutocompleteAgentState,
    pub source: AutocompleteAgentSource,
    pub output_status: AutocompleteAgentOutputStatus,
    pub stream_support: AutocompleteAgentStreamSupport,
    pub stream_fallback: AutocompleteAgentStreamFallback,
    pub stream_mode: AutocompleteAgentStreamMode,
    pub state_source: &'static str,
    pub motion_source: &'static str,
    pub selected_source: &'static str,
    pub selected_change_source: &'static str,
    pub open_value_source: &'static str,
    pub config_policy: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutocompleteAgentContractInput {
    pub is_open: bool,
    pub is_disabled: bool,
    pub has_typed: bool,
    pub has_selection: bool,
    pub is_open_controlled: bool,
    pub selected_source: SelectedSource,
    pub selected_change_source: SelectedChangeSource,
    pub render_state: AutocompleteState,
}

fn resolve_agent_action(input: AutocompleteAgentContractInput) -> AutocompleteAgentAction {
    if input.has_typed {
        AutocompleteAgentAction::Query
    } else if input.has_selection {
        AutocompleteAgentAction::CommitSelection
    } else {
        AutocompleteAgentAction::Idle
    }
}

fn resolve_agent_state(input: AutocompleteAgentContractInput) -> AutocompleteAgentState {
    match resolve_root_data_state(input.is_open, input.is_disabled) {
        RootDataState::Open => AutocompleteAgentState::Open,
        RootDataState::Disabled => AutocompleteAgentState::Disabled,
        RootDataState::Closed => AutocompleteAgentState::Closed,
    }
}

fn resolve_open_value_source(is_open_controlled: bool) -> AutocompleteAgentOpenValueSource {
    if is_open_controlled {
        AutocompleteAgentOpenValueSource::Controlled
    } else {
        AutocompleteAgentOpenValueSource::Uncontrolled
    }
}

pub fn resolve_agent_contract(input: AutocompleteAgentContractInput) -> AutocompleteAgentContract {
    AutocompleteAgentContract {
        schema_name: AUTOCOMPLETE_AGENT_SCHEMA,
        schema_version: AutocompleteAgentSchemaVersion::V1,
        intent: AutocompleteAgentIntent::SuggestAndSelect,
        action: resolve_agent_action(input),
        state: resolve_agent_state(input),
        source: AutocompleteAgentSource::StatePrimitives,
        output_status: AutocompleteAgentOutputStatus::Verified,
        stream_support: AutocompleteAgentStreamSupport::Unsupported,
        stream_fallback: AutocompleteAgentStreamFallback::Snapshot,
        stream_mode: AutocompleteAgentStreamMode::Snapshot,
        state_source: resolve_open_value_source(input.is_open_controlled).as_str(),
        motion_source: input.render_state.motion_source_attr,
        selected_source: input.selected_source.as_attr(),
        selected_change_source: input.selected_change_source.as_attr(),
        open_value_source: resolve_open_value_source(input.is_open_controlled).as_str(),
        config_policy: "whitelist",
    }
}

pub fn normalize_root_state(input: RootStateInput) -> RootState {
    let id_base = normalize_id_base(input.id_base);

    let has_custom_label = !input.label.trim().is_empty();
    let label = normalize_label(input.label);

    let has_custom_placeholder = normalize_optional_text(input.placeholder.clone()).is_some();
    let placeholder = resolve_placeholder(input.placeholder);
    let empty_message = resolve_empty_message(input.empty_message.or(input.i18n_empty_message));

    let description = normalize_optional_text(input.description);
    let error = normalize_optional_text(input.error);
    let has_custom_description = description.is_some();
    let has_custom_error = error.is_some();

    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let disabled_indices = normalize_disabled_indices(input.disabled_indices, input.item_count);
    let disabled_option_count = disabled_indices.len();

    let state = resolve_state(AutocompleteStateInput {
        item_count: input.item_count,
        disabled_option_count,
        is_disabled: input.is_disabled,
        has_custom_label,
        has_custom_description,
        has_custom_error,
        has_custom_placeholder,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        is_controlled: input.is_controlled,
    });

    let class_name = compose_class_name(class_name, state);

    RootState {
        id_base,
        label,
        placeholder,
        empty_message,
        description,
        error,
        class_name,
        disabled_indices,
        state,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AutocompleteState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed("ui-autocomplete")];

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-autocomplete--disabled"));
    }
    if state.is_empty {
        classes.push(Cow::Borrowed("ui-autocomplete--empty"));
    }
    if state.has_description {
        classes.push(Cow::Borrowed("ui-autocomplete--has-description"));
    }
    if state.has_error {
        classes.push(Cow::Borrowed("ui-autocomplete--has-error"));
    }
    if state.has_disabled_options {
        classes.push(Cow::Borrowed("ui-autocomplete--has-disabled-options"));
    }
    if state.is_controlled {
        classes.push(Cow::Borrowed("ui-autocomplete--controlled"));
    }
    if state.has_custom_label {
        classes.push(Cow::Borrowed("ui-autocomplete--custom-label"));
    }
    if state.has_custom_description {
        classes.push(Cow::Borrowed("ui-autocomplete--custom-description"));
    }
    if state.has_custom_error {
        classes.push(Cow::Borrowed("ui-autocomplete--custom-error"));
    }
    if state.has_custom_placeholder {
        classes.push(Cow::Borrowed("ui-autocomplete--custom-placeholder"));
    }
    if state.has_custom_id_base {
        classes.push(Cow::Borrowed("ui-autocomplete--custom-id"));
    }
    if state.has_custom_motion {
        classes.push(Cow::Borrowed("ui-autocomplete--custom-motion"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-autocomplete--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    let mut iter = classes.into_iter();
    let Some(first) = iter.next() else {
        return String::new();
    };
    let mut composed = first.into_owned();
    for class in iter {
        composed.push(' ');
        composed.push_str(class.as_ref());
    }
    composed
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
