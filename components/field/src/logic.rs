use std::borrow::Cow;

pub use ui_state_primitives::field::*;

pub const FIELD_AGENT_SCHEMA: &str = "ui.field.agent-contract/v1";
pub const FIELD_AGENT_SCHEMA_VERSION: &str = "1";
pub const FIELD_LLM_RENDER_MODES: [&str; 2] = ["streaming", "snapshot"];
pub const FIELD_DEFAULT_RENDER_MODE: &str = "snapshot";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldBoolPropSource {
    IsProp,
    LegacyProp,
    DefaultValue,
}

impl FieldBoolPropSource {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldBoolPropSource::IsProp => "is-prop",
            FieldBoolPropSource::LegacyProp => "legacy-prop",
            FieldBoolPropSource::DefaultValue => "default",
        }
    }
}

fn resolve_bool_value(primary: Option<bool>, legacy: Option<bool>) -> bool {
    primary.or(legacy).unwrap_or_default()
}

fn resolve_bool_source(primary: Option<bool>, legacy: Option<bool>) -> FieldBoolPropSource {
    if primary.is_some() {
        FieldBoolPropSource::IsProp
    } else if legacy.is_some() {
        FieldBoolPropSource::LegacyProp
    } else {
        FieldBoolPropSource::DefaultValue
    }
}

pub fn resolve_is_required(is_required: Option<bool>, required: Option<bool>) -> bool {
    resolve_bool_value(is_required, required)
}

pub fn resolve_required_source(
    is_required: Option<bool>,
    required: Option<bool>,
) -> FieldBoolPropSource {
    resolve_bool_source(is_required, required)
}

pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {
    resolve_bool_value(is_disabled, disabled)
}

pub fn resolve_disabled_source(
    is_disabled: Option<bool>,
    disabled: Option<bool>,
) -> FieldBoolPropSource {
    resolve_bool_source(is_disabled, disabled)
}

pub fn resolve_is_invalid(is_invalid: Option<bool>, invalid: Option<bool>) -> bool {
    resolve_bool_value(is_invalid, invalid)
}

pub fn resolve_invalid_source(
    is_invalid: Option<bool>,
    invalid: Option<bool>,
) -> FieldBoolPropSource {
    resolve_bool_source(is_invalid, invalid)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldAgentIntent {
    FormField,
}

impl FieldAgentIntent {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldAgentIntent::FormField => "form-field",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldAgentAction {
    SnapshotRender,
}

impl FieldAgentAction {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldAgentAction::SnapshotRender => "snapshot_render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldAgentState {
    Default,
    Required,
    Disabled,
    Invalid,
    InvalidDisabled,
    Horizontal,
    Muted,
}

impl FieldAgentState {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldAgentState::Default => "default",
            FieldAgentState::Required => "required",
            FieldAgentState::Disabled => "disabled",
            FieldAgentState::Invalid => "invalid",
            FieldAgentState::InvalidDisabled => "invalid-disabled",
            FieldAgentState::Horizontal => "horizontal",
            FieldAgentState::Muted => "muted",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldAgentSource {
    Default,
    IsProp,
    LegacyProp,
    Mixed,
}

impl FieldAgentSource {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldAgentSource::Default => "default",
            FieldAgentSource::IsProp => "is-prop",
            FieldAgentSource::LegacyProp => "legacy-prop",
            FieldAgentSource::Mixed => "mixed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldAgentStreamSupport {
    Optional,
}

impl FieldAgentStreamSupport {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldAgentStreamSupport::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldAgentOutputStatus {
    Verified,
}

impl FieldAgentOutputStatus {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldAgentOutputStatus::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldAgentContract {
    pub schema: &'static str,
    pub schema_version: &'static str,
    pub intent: &'static str,
    pub action: &'static str,
    pub state: &'static str,
    pub source: &'static str,
    pub source_required: &'static str,
    pub source_disabled: &'static str,
    pub source_invalid: &'static str,
    pub source_motion: &'static str,
    pub source_aria: &'static str,
    pub source_error: &'static str,
    pub source_class: &'static str,
    pub stream_mode: &'static str,
    pub stream_support: &'static str,
    pub stream_fallback: &'static str,
    pub output_mode: &'static str,
    pub output_status: &'static str,
}

fn resolve_agent_state(state: FieldState) -> FieldAgentState {
    if state.is_invalid && state.is_disabled {
        FieldAgentState::InvalidDisabled
    } else if state.is_invalid {
        FieldAgentState::Invalid
    } else if state.is_disabled {
        FieldAgentState::Disabled
    } else if state.is_required {
        FieldAgentState::Required
    } else if state.orientation == FieldOrientation::Horizontal {
        FieldAgentState::Horizontal
    } else if state.tone == FieldTone::Muted {
        FieldAgentState::Muted
    } else {
        FieldAgentState::Default
    }
}

fn source_has_legacy_prop(
    required_source: FieldBoolPropSource,
    disabled_source: FieldBoolPropSource,
    invalid_source: FieldBoolPropSource,
) -> bool {
    matches!(
        (required_source, disabled_source, invalid_source),
        (FieldBoolPropSource::LegacyProp, _, _,)
            | (_, FieldBoolPropSource::LegacyProp, _,)
            | (_, _, FieldBoolPropSource::LegacyProp,)
    )
}

fn source_has_is_prop(
    required_source: FieldBoolPropSource,
    disabled_source: FieldBoolPropSource,
    invalid_source: FieldBoolPropSource,
) -> bool {
    matches!(
        (required_source, disabled_source, invalid_source),
        (FieldBoolPropSource::IsProp, _, _,)
            | (_, FieldBoolPropSource::IsProp, _,)
            | (_, _, FieldBoolPropSource::IsProp,)
    )
}

fn resolve_agent_source(
    state: FieldState,
    required_source: FieldBoolPropSource,
    disabled_source: FieldBoolPropSource,
    invalid_source: FieldBoolPropSource,
    motion_source: &'static str,
) -> FieldAgentSource {
    let has_legacy = source_has_legacy_prop(required_source, disabled_source, invalid_source);
    let has_is_prop = source_has_is_prop(required_source, disabled_source, invalid_source);
    let has_custom_side_channel = motion_source == "custom"
        || state.aria_source_attr == "custom"
        || state.error_source_attr == "custom"
        || state.class_source_attr == "custom";

    if !has_legacy && !has_is_prop && !has_custom_side_channel {
        FieldAgentSource::Default
    } else if has_legacy && !has_is_prop && !has_custom_side_channel {
        FieldAgentSource::LegacyProp
    } else if has_is_prop && !has_legacy && !has_custom_side_channel {
        FieldAgentSource::IsProp
    } else {
        FieldAgentSource::Mixed
    }
}

pub fn resolve_agent_contract(
    state: FieldState,
    required_source: FieldBoolPropSource,
    disabled_source: FieldBoolPropSource,
    invalid_source: FieldBoolPropSource,
    motion_source: &'static str,
) -> FieldAgentContract {
    debug_assert!(FIELD_LLM_RENDER_MODES.contains(&FIELD_DEFAULT_RENDER_MODE));

    let intent = FieldAgentIntent::FormField;
    let action = FieldAgentAction::SnapshotRender;
    let state_axis = resolve_agent_state(state);
    let source_axis = resolve_agent_source(
        state,
        required_source,
        disabled_source,
        invalid_source,
        motion_source,
    );
    let stream_support = FieldAgentStreamSupport::Optional;
    let output_status = FieldAgentOutputStatus::Verified;

    FieldAgentContract {
        schema: FIELD_AGENT_SCHEMA,
        schema_version: FIELD_AGENT_SCHEMA_VERSION,
        intent: intent.as_data_attr(),
        action: action.as_data_attr(),
        state: state_axis.as_data_attr(),
        source: source_axis.as_data_attr(),
        source_required: required_source.as_data_attr(),
        source_disabled: disabled_source.as_data_attr(),
        source_invalid: invalid_source.as_data_attr(),
        source_motion: motion_source,
        source_aria: state.aria_source_attr,
        source_error: state.error_source_attr,
        source_class: state.class_source_attr,
        stream_mode: FIELD_DEFAULT_RENDER_MODE,
        stream_support: stream_support.as_data_attr(),
        stream_fallback: FIELD_DEFAULT_RENDER_MODE,
        output_mode: FIELD_DEFAULT_RENDER_MODE,
        output_status: output_status.as_data_attr(),
    }
}

pub struct FieldContentInput {
    pub label: Option<String>,
    pub description: Option<String>,
    pub error_message: Option<String>,
    pub aria_label: Option<String>,
    pub lang: Option<String>,
    pub class_name: Option<String>,
    pub is_invalid: bool,
}

type FieldCowStr = Cow<'static, str>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldContent {
    pub label_text: FieldCowStr,
    pub description_text: FieldCowStr,
    pub error_message_text: FieldCowStr,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub aria_label: FieldCowStr,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub lang: Option<FieldCowStr>,
    pub class_name: Option<FieldCowStr>,
    pub has_custom_class_name: bool,
}

fn normalize_optional_cow(value: Option<String>) -> Option<FieldCowStr> {
    normalize_optional_text(value).map(Cow::Owned)
}

fn normalize_aria_label_cow(value: Option<String>) -> (FieldCowStr, bool) {
    match normalize_optional_text(value) {
        Some(label) => (Cow::Owned(label), true),
        None => (Cow::Borrowed(DEFAULT_ARIA_LABEL), false),
    }
}

fn normalize_error_message_cow(
    value: Option<String>,
    invalid: bool,
) -> (Option<FieldCowStr>, bool) {
    if !invalid {
        return (None, false);
    }

    match normalize_optional_text(value) {
        Some(message) => (Some(Cow::Owned(message)), true),
        None => (Some(Cow::Borrowed(DEFAULT_ERROR_MESSAGE)), false),
    }
}

pub fn resolve_content(input: FieldContentInput) -> FieldContent {
    let label = normalize_optional_cow(input.label);
    let description = normalize_optional_cow(input.description);
    let (error_message, has_custom_error_message) =
        normalize_error_message_cow(input.error_message, input.is_invalid);
    let (aria_label, has_custom_aria_label) = normalize_aria_label_cow(input.aria_label);
    let lang = normalize_optional_cow(input.lang);
    let class_name = normalize_optional_cow(input.class_name);

    let has_label = label.is_some();
    let has_description = description.is_some();
    let has_error_message = error_message.is_some();
    let has_custom_class_name = class_name.is_some();

    let label_text = match label {
        Some(text) => text,
        None => Cow::Borrowed(""),
    };
    let description_text = match description {
        Some(text) => text,
        None => Cow::Borrowed(""),
    };
    let error_message_text = match error_message {
        Some(text) => text,
        None => Cow::Borrowed(""),
    };

    FieldContent {
        label_text,
        description_text,
        error_message_text,
        has_label,
        has_description,
        has_error_message,
        aria_label,
        has_custom_aria_label,
        has_custom_error_message,
        lang,
        class_name,
        has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
