use crate::{NativeSelectOption, NativeSelectOptionResolved, NativeSelectStateInput};
use std::borrow::Cow;

pub const DEFAULT_ARIA_LABEL: &str = ui_state_primitives::native_select::DEFAULT_ARIA_LABEL;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NativeSelectSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectState {
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub is_disabled: bool,
    pub control_disabled: bool,
    pub is_invalid: bool,
    pub is_required: bool,
    pub has_placeholder: bool,
    pub is_empty: bool,
    pub has_options: bool,
    pub option_count: usize,
    pub selected_index: Option<usize>,
    pub selected_value: Option<String>,
    pub has_selection: bool,
    pub has_disabled_options: bool,
    pub has_enabled_options: bool,
    pub disabled_option_count: usize,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectResolvedStates {
    pub component: NativeSelectState,
    pub primitive: ui_state_primitives::native_select::NativeSelectState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeSelectStateParams<'a> {
    pub size: NativeSelectSize,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_required: bool,
    pub has_placeholder: bool,
    pub selected_index: Option<usize>,
    pub options: &'a [NativeSelectOptionResolved],
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub const NATIVE_SELECT_AGENT_SCHEMA_NAME: &str = "ui.native_select.agent-contract";
pub const NATIVE_SELECT_AGENT_SCHEMA_VERSION: &str = "v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeSelectAgentIntent {
    SelectionChoose,
}

impl NativeSelectAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::SelectionChoose => "selection.choose",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeSelectAgentAction {
    Idle,
    UserSelect,
    ExternalSync,
    InternalSync,
    SyncEffect,
    Disabled,
}

impl NativeSelectAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::UserSelect => "user-select",
            Self::ExternalSync => "external-sync",
            Self::InternalSync => "internal-sync",
            Self::SyncEffect => "sync-effect",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeSelectAgentState {
    Empty,
    Selected,
    Disabled,
}

impl NativeSelectAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Selected => "selected",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeSelectAgentSource {
    External,
    Default,
    Internal,
}

impl NativeSelectAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::External => "external",
            Self::Default => "default",
            Self::Internal => "internal",
        }
    }

    pub fn from_attr(attr: &str) -> Self {
        match attr {
            "external" => Self::External,
            "default" => Self::Default,
            _ => Self::Internal,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeSelectChangeSource {
    Initial,
    User,
    External,
    Internal,
    SyncEffect,
}

impl NativeSelectChangeSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::User => "user",
            Self::External => "external",
            Self::Internal => "internal",
            Self::SyncEffect => "sync-effect",
        }
    }

    pub fn from_attr(attr: &str) -> Self {
        match attr {
            "user" => Self::User,
            "external" => Self::External,
            "internal" => Self::Internal,
            "sync-effect" => Self::SyncEffect,
            _ => Self::Initial,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeSelectAgentConfigPolicy {
    Whitelist,
}

impl NativeSelectAgentConfigPolicy {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeSelectOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl NativeSelectOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeSelectAgentContract {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent: NativeSelectAgentIntent,
    pub action: NativeSelectAgentAction,
    pub state: NativeSelectAgentState,
    pub source: NativeSelectAgentSource,
    pub config_policy: NativeSelectAgentConfigPolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeSelectAgentContractInput<'a> {
    pub state: &'a NativeSelectState,
    pub selection_source_attr: &'a str,
    pub change_source_attr: &'a str,
}

impl NativeSelectSize {
    pub fn class_name(self) -> &'static str {
        match self {
            NativeSelectSize::Sm => "ui-native-select--size-sm",
            NativeSelectSize::Md => "ui-native-select--size-md",
            NativeSelectSize::Lg => "ui-native-select--size-lg",
        }
    }

    pub fn data_size(self) -> &'static str {
        match self {
            NativeSelectSize::Sm => "sm",
            NativeSelectSize::Md => "md",
            NativeSelectSize::Lg => "lg",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::native_select::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    ui_state_primitives::native_select::normalize_aria_label(value)
}

pub fn normalize_options(options: Vec<NativeSelectOption>) -> Vec<NativeSelectOption> {
    ui_state_primitives::native_select::normalize_options(options)
}

pub fn normalize_placeholder(placeholder: Option<String>) -> Option<String> {
    normalize_optional_text(placeholder)
}

pub fn normalize_default_selected_index(
    default_selected_index: Option<usize>,
) -> Option<Option<usize>> {
    default_selected_index.map(Some)
}

pub fn resolve_options(
    id_base: &str,
    options: &[NativeSelectOption],
) -> Vec<NativeSelectOptionResolved> {
    ui_state_primitives::native_select::resolve_options(id_base, options)
}

pub fn find_index_by_value(value: &str, options: &[NativeSelectOptionResolved]) -> Option<usize> {
    ui_state_primitives::native_select::find_index_by_value(value, options)
}

pub fn sanitize_selected_index(
    selected_index: Option<usize>,
    options: &[NativeSelectOptionResolved],
) -> Option<usize> {
    ui_state_primitives::native_select::sanitize_selected_index(selected_index, options)
}

pub fn resolve_selected_index_correction(
    selected_index: Option<usize>,
    options: &[NativeSelectOptionResolved],
) -> Option<Option<usize>> {
    let sanitized = sanitize_selected_index(selected_index, options);
    (sanitized != selected_index).then_some(sanitized)
}

pub fn resolve_states_for_render(
    params: NativeSelectStateParams<'_>,
) -> NativeSelectResolvedStates {
    resolve_states(
        NativeSelectStateInput {
            disabled: params.is_disabled,
            invalid: params.is_invalid,
            required: params.is_required,
            has_placeholder: params.has_placeholder,
            selected_index: params.selected_index,
            options: params.options,
            has_custom_aria_label: params.has_custom_aria_label,
            has_custom_class_name: params.has_custom_class_name,
        },
        params.size,
    )
}

pub fn resolve_state(
    input: NativeSelectStateInput<'_>,
    size: NativeSelectSize,
) -> NativeSelectState {
    resolve_states(input, size).component
}

pub fn resolve_states(
    input: NativeSelectStateInput<'_>,
    size: NativeSelectSize,
) -> NativeSelectResolvedStates {
    let primitive = ui_state_primitives::native_select::resolve_state(input);
    let component = resolve_state_from_primitive(&primitive, size);
    NativeSelectResolvedStates {
        component,
        primitive,
    }
}

pub fn resolve_state_from_primitive(
    primitive_state: &ui_state_primitives::native_select::NativeSelectState,
    size: NativeSelectSize,
) -> NativeSelectState {
    NativeSelectState {
        size_class: size.class_name(),
        size_attr: size.data_size(),
        is_disabled: primitive_state.is_disabled,
        control_disabled: primitive_state.control_disabled,
        is_invalid: primitive_state.is_invalid,
        is_required: primitive_state.is_required,
        has_placeholder: primitive_state.has_placeholder,
        is_empty: primitive_state.is_empty,
        has_options: primitive_state.has_options,
        option_count: primitive_state.option_count,
        selected_index: primitive_state.selected_index,
        selected_value: primitive_state.selected_value.clone(),
        has_selection: primitive_state.has_selection,
        has_disabled_options: primitive_state.has_disabled_options,
        has_enabled_options: primitive_state.has_enabled_options,
        disabled_option_count: primitive_state.disabled_option_count,
        data_state_attr: primitive_state.data_state_attr,
        aria_source_attr: primitive_state.aria_source_attr,
        class_source_attr: primitive_state.class_source_attr,
        has_custom_class_name: primitive_state.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: &NativeSelectState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-native-select"),
        Cow::Borrowed(state.size_class),
    ];

    if state.control_disabled {
        classes.push(Cow::Borrowed("ui-native-select--disabled"));
    }
    if state.is_invalid {
        classes.push(Cow::Borrowed("ui-native-select--invalid"));
    }
    if state.is_empty {
        classes.push(Cow::Borrowed("ui-native-select--empty"));
    }
    if state.has_selection {
        classes.push(Cow::Borrowed("ui-native-select--selected"));
    }
    if state.has_placeholder {
        classes.push(Cow::Borrowed("ui-native-select--has-placeholder"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-native-select--custom-class"));
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

pub fn resolve_control_value(selected_value: Option<&str>) -> String {
    selected_value.unwrap_or_default().to_string()
}

pub fn resolve_agent_contract(
    input: NativeSelectAgentContractInput<'_>,
) -> NativeSelectAgentContract {
    let source = NativeSelectAgentSource::from_attr(input.selection_source_attr);
    let change_source = NativeSelectChangeSource::from_attr(input.change_source_attr);
    let state = if input.state.control_disabled {
        NativeSelectAgentState::Disabled
    } else if input.state.has_selection {
        NativeSelectAgentState::Selected
    } else {
        NativeSelectAgentState::Empty
    };

    let action = if input.state.control_disabled {
        NativeSelectAgentAction::Disabled
    } else {
        match change_source {
            NativeSelectChangeSource::Initial => NativeSelectAgentAction::Idle,
            NativeSelectChangeSource::User => NativeSelectAgentAction::UserSelect,
            NativeSelectChangeSource::External => NativeSelectAgentAction::ExternalSync,
            NativeSelectChangeSource::Internal => NativeSelectAgentAction::InternalSync,
            NativeSelectChangeSource::SyncEffect => NativeSelectAgentAction::SyncEffect,
        }
    };

    NativeSelectAgentContract {
        schema_name: NATIVE_SELECT_AGENT_SCHEMA_NAME,
        schema_version: NATIVE_SELECT_AGENT_SCHEMA_VERSION,
        intent: NativeSelectAgentIntent::SelectionChoose,
        action,
        state,
        source,
        config_policy: NativeSelectAgentConfigPolicy::Whitelist,
    }
}

pub fn resolve_output_status(state: &NativeSelectState) -> NativeSelectOutputStatus {
    if state.control_disabled {
        NativeSelectOutputStatus::Verified
    } else if state.is_invalid || state.is_empty {
        NativeSelectOutputStatus::Draft
    } else {
        NativeSelectOutputStatus::Submittable
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
