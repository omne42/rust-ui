use leptos::prelude::*;
use std::borrow::Cow;

pub use ui_state_primitives::combo_box::{
    ComboBoxState, ComboBoxStateInput, filter_indices, map_filtered_to_original,
    map_selected_to_filtered, normalize_disabled_indices, normalize_id_base, normalize_label,
    normalize_optional_text, resolve_empty_message, resolve_placeholder, resolve_state,
    resolve_toggle_aria_label,
};

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub is_required: Option<Signal<bool>>,
    pub is_invalid: Option<Signal<bool>>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub required: Signal<bool>,
    pub invalid: Signal<bool>,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    let required = input
        .is_required
        .unwrap_or_else(|| Signal::derive(|| false));
    let invalid = input.is_invalid.unwrap_or_else(|| Signal::derive(|| false));

    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(false),
        required,
        invalid,
    }
}

pub struct OpenStateInput {
    pub is_open: Option<Signal<bool>>,
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
    let open = input.is_open;
    OpenState {
        is_controlled: open.is_some(),
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
    }
}

pub struct RootStateInput {
    pub id_base: String,
    pub has_custom_id_base: bool,
    pub label: String,
    pub placeholder: Option<String>,
    pub empty_message: Option<String>,
    pub toggle_button_aria_label: Option<String>,
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
    pub toggle_button_aria_label: String,
    pub description: Option<String>,
    pub error: Option<String>,
    pub class_name: String,
    pub disabled_indices: Vec<usize>,
    pub state: ComboBoxState,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboBoxAgentSchemaVersion {
    V1,
}

impl ComboBoxAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboBoxAgentIntent {
    SelectionDiscovery,
}

impl ComboBoxAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectionDiscovery => "selection-discovery",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboBoxAgentAction {
    Inert,
    ToggleOpen,
    NavigateOptions,
    FilterQuery,
}

impl ComboBoxAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inert => "inert",
            Self::ToggleOpen => "toggle-open",
            Self::NavigateOptions => "navigate-options",
            Self::FilterQuery => "filter-query",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboBoxAgentStateAxis {
    Open,
    Closed,
    Disabled,
}

impl ComboBoxAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboBoxAgentSourceAxis {
    ControlledExternal,
    UncontrolledInternal,
}

impl ComboBoxAgentSourceAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlledExternal => "controlled-external",
            Self::UncontrolledInternal => "uncontrolled-internal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboBoxAgentStreamSupport {
    Unsupported,
}

impl ComboBoxAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboBoxAgentStreamFallback {
    Snapshot,
}

impl ComboBoxAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboBoxAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl ComboBoxAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}
const _: [ComboBoxAgentOutputStatus; 3] = [
    ComboBoxAgentOutputStatus::Draft,
    ComboBoxAgentOutputStatus::Verified,
    ComboBoxAgentOutputStatus::Submittable,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboBoxAgentCapabilities {
    pub can_filter: bool,
    pub can_select: bool,
    pub can_open: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboBoxAgentContractInput {
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub has_typed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboBoxAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ComboBoxAgentSchemaVersion,
    pub intent: ComboBoxAgentIntent,
    pub action: ComboBoxAgentAction,
    pub state: ComboBoxAgentStateAxis,
    pub source: ComboBoxAgentSourceAxis,
    pub stream_support: ComboBoxAgentStreamSupport,
    pub stream_fallback: ComboBoxAgentStreamFallback,
    pub output_status: ComboBoxAgentOutputStatus,
    pub capabilities: ComboBoxAgentCapabilities,
}

pub fn resolve_agent_contract(input: ComboBoxAgentContractInput) -> ComboBoxAgentContract {
    let state = match resolve_root_data_state(input.is_open, input.is_disabled) {
        RootDataState::Open => ComboBoxAgentStateAxis::Open,
        RootDataState::Disabled => ComboBoxAgentStateAxis::Disabled,
        RootDataState::Closed => ComboBoxAgentStateAxis::Closed,
    };

    let action = if input.is_disabled {
        ComboBoxAgentAction::Inert
    } else if input.has_typed {
        ComboBoxAgentAction::FilterQuery
    } else if input.is_open {
        ComboBoxAgentAction::NavigateOptions
    } else {
        ComboBoxAgentAction::ToggleOpen
    };

    let source = if input.is_controlled {
        ComboBoxAgentSourceAxis::ControlledExternal
    } else {
        ComboBoxAgentSourceAxis::UncontrolledInternal
    };

    ComboBoxAgentContract {
        schema_name: "ui.combo-box.agent-contract",
        schema_version: ComboBoxAgentSchemaVersion::V1,
        intent: ComboBoxAgentIntent::SelectionDiscovery,
        action,
        state,
        source,
        stream_support: ComboBoxAgentStreamSupport::Unsupported,
        stream_fallback: ComboBoxAgentStreamFallback::Snapshot,
        output_status: ComboBoxAgentOutputStatus::Verified,
        capabilities: ComboBoxAgentCapabilities {
            can_filter: !input.is_disabled,
            can_select: !input.is_disabled && input.is_open,
            can_open: !input.is_disabled,
        },
    }
}

pub fn resolve_id_base(id_base: String, generated_id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or(generated_id_base)
}

pub fn normalize_root_state(input: RootStateInput) -> RootState {
    let id_base = normalize_id_base(input.id_base);

    let has_custom_label = !input.label.trim().is_empty();
    let label = normalize_label(input.label);

    let has_custom_placeholder = normalize_optional_text(input.placeholder.clone()).is_some();
    let placeholder = resolve_placeholder(input.placeholder);
    let empty_message = resolve_empty_message(input.empty_message);
    let toggle_button_aria_label = resolve_toggle_aria_label(input.toggle_button_aria_label);

    let description = normalize_optional_text(input.description);
    let error = normalize_optional_text(input.error);
    let has_custom_description = description.is_some();
    let has_custom_error = error.is_some();

    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let disabled_indices = normalize_disabled_indices(input.disabled_indices, input.item_count);
    let disabled_option_count = disabled_indices.len();

    let state = resolve_state(ComboBoxStateInput {
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
        toggle_button_aria_label,
        description,
        error,
        class_name,
        disabled_indices,
        state,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ComboBoxState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed("ui-combo-box")];

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-combo-box--disabled"));
    }
    if state.is_empty {
        classes.push(Cow::Borrowed("ui-combo-box--empty"));
    }
    if state.has_description {
        classes.push(Cow::Borrowed("ui-combo-box--has-description"));
    }
    if state.has_error {
        classes.push(Cow::Borrowed("ui-combo-box--has-error"));
    }
    if state.has_disabled_options {
        classes.push(Cow::Borrowed("ui-combo-box--has-disabled-options"));
    }
    if state.is_controlled {
        classes.push(Cow::Borrowed("ui-combo-box--controlled"));
    }
    if state.has_custom_label {
        classes.push(Cow::Borrowed("ui-combo-box--custom-label"));
    }
    if state.has_custom_description {
        classes.push(Cow::Borrowed("ui-combo-box--custom-description"));
    }
    if state.has_custom_error {
        classes.push(Cow::Borrowed("ui-combo-box--custom-error"));
    }
    if state.has_custom_placeholder {
        classes.push(Cow::Borrowed("ui-combo-box--custom-placeholder"));
    }
    if state.has_custom_id_base {
        classes.push(Cow::Borrowed("ui-combo-box--custom-id"));
    }
    if state.has_custom_motion {
        classes.push(Cow::Borrowed("ui-combo-box--custom-motion"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-combo-box--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .into_iter()
        .map(Cow::into_owned)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
