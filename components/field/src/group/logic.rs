use std::borrow::Cow;

pub use ui_state_primitives::field_group::*;

pub const FIELD_GROUP_AGENT_SCHEMA: &str = "ui.field.agent-contract/v1";
pub const FIELD_GROUP_AGENT_SCHEMA_VERSION: &str = "1";
pub const FIELD_GROUP_LLM_RENDER_MODES: [&str; 2] = ["streaming", "snapshot"];
pub const FIELD_GROUP_DEFAULT_RENDER_MODE: &str = "snapshot";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldGroupBoolPropSource {
    IsProp,
    LegacyProp,
    DefaultValue,
}

impl FieldGroupBoolPropSource {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldGroupBoolPropSource::IsProp => "is-prop",
            FieldGroupBoolPropSource::LegacyProp => "legacy-prop",
            FieldGroupBoolPropSource::DefaultValue => "default",
        }
    }
}

fn resolve_bool_value(primary: Option<bool>, legacy: Option<bool>) -> bool {
    primary.or(legacy).unwrap_or_default()
}

fn resolve_bool_source(primary: Option<bool>, legacy: Option<bool>) -> FieldGroupBoolPropSource {
    if primary.is_some() {
        FieldGroupBoolPropSource::IsProp
    } else if legacy.is_some() {
        FieldGroupBoolPropSource::LegacyProp
    } else {
        FieldGroupBoolPropSource::DefaultValue
    }
}

pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {
    resolve_bool_value(is_disabled, disabled)
}

pub fn resolve_disabled_source(
    is_disabled: Option<bool>,
    disabled: Option<bool>,
) -> FieldGroupBoolPropSource {
    resolve_bool_source(is_disabled, disabled)
}

pub fn resolve_is_invalid(is_invalid: Option<bool>, invalid: Option<bool>) -> bool {
    resolve_bool_value(is_invalid, invalid)
}

pub fn resolve_invalid_source(
    is_invalid: Option<bool>,
    invalid: Option<bool>,
) -> FieldGroupBoolPropSource {
    resolve_bool_source(is_invalid, invalid)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldGroupAgentIntent {
    FormField,
}

impl FieldGroupAgentIntent {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldGroupAgentIntent::FormField => "form-field",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldGroupAgentAction {
    SnapshotRender,
}

impl FieldGroupAgentAction {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldGroupAgentAction::SnapshotRender => "snapshot_render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldGroupAgentState {
    Default,
    Disabled,
    Invalid,
    InvalidDisabled,
}

impl FieldGroupAgentState {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldGroupAgentState::Default => "default",
            FieldGroupAgentState::Disabled => "disabled",
            FieldGroupAgentState::Invalid => "invalid",
            FieldGroupAgentState::InvalidDisabled => "invalid-disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldGroupAgentSource {
    Default,
    IsProp,
    LegacyProp,
    Mixed,
}

impl FieldGroupAgentSource {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldGroupAgentSource::Default => "default",
            FieldGroupAgentSource::IsProp => "is-prop",
            FieldGroupAgentSource::LegacyProp => "legacy-prop",
            FieldGroupAgentSource::Mixed => "mixed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldGroupAgentStreamSupport {
    Optional,
}

impl FieldGroupAgentStreamSupport {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldGroupAgentStreamSupport::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldGroupAgentOutputStatus {
    Verified,
}

impl FieldGroupAgentOutputStatus {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            FieldGroupAgentOutputStatus::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldGroupAgentContract {
    pub schema: &'static str,
    pub schema_version: &'static str,
    pub intent: &'static str,
    pub action: &'static str,
    pub state: &'static str,
    pub source: &'static str,
    pub source_disabled: &'static str,
    pub source_invalid: &'static str,
    pub source_aria: &'static str,
    pub source_class: &'static str,
    pub stream_mode: &'static str,
    pub stream_support: &'static str,
    pub stream_fallback: &'static str,
    pub output_mode: &'static str,
    pub output_status: &'static str,
}

fn resolve_agent_state(state: FieldGroupState) -> FieldGroupAgentState {
    if state.is_invalid && state.is_disabled {
        FieldGroupAgentState::InvalidDisabled
    } else if state.is_invalid {
        FieldGroupAgentState::Invalid
    } else if state.is_disabled {
        FieldGroupAgentState::Disabled
    } else {
        FieldGroupAgentState::Default
    }
}

fn source_has_legacy_prop(
    disabled_source: FieldGroupBoolPropSource,
    invalid_source: FieldGroupBoolPropSource,
) -> bool {
    disabled_source == FieldGroupBoolPropSource::LegacyProp
        || invalid_source == FieldGroupBoolPropSource::LegacyProp
}

fn source_has_is_prop(
    disabled_source: FieldGroupBoolPropSource,
    invalid_source: FieldGroupBoolPropSource,
) -> bool {
    disabled_source == FieldGroupBoolPropSource::IsProp
        || invalid_source == FieldGroupBoolPropSource::IsProp
}

fn resolve_agent_source(
    state: FieldGroupState,
    disabled_source: FieldGroupBoolPropSource,
    invalid_source: FieldGroupBoolPropSource,
) -> FieldGroupAgentSource {
    let has_legacy = source_has_legacy_prop(disabled_source, invalid_source);
    let has_is_prop = source_has_is_prop(disabled_source, invalid_source);
    let has_custom_side_channel =
        state.aria_source_attr == "custom" || state.class_source_attr == "custom";

    if !has_legacy && !has_is_prop && !has_custom_side_channel {
        FieldGroupAgentSource::Default
    } else if has_legacy && !has_is_prop && !has_custom_side_channel {
        FieldGroupAgentSource::LegacyProp
    } else if has_is_prop && !has_legacy && !has_custom_side_channel {
        FieldGroupAgentSource::IsProp
    } else {
        FieldGroupAgentSource::Mixed
    }
}

pub fn resolve_agent_contract(
    state: FieldGroupState,
    disabled_source: FieldGroupBoolPropSource,
    invalid_source: FieldGroupBoolPropSource,
) -> FieldGroupAgentContract {
    debug_assert!(FIELD_GROUP_LLM_RENDER_MODES.contains(&FIELD_GROUP_DEFAULT_RENDER_MODE));

    let intent = FieldGroupAgentIntent::FormField;
    let action = FieldGroupAgentAction::SnapshotRender;
    let state_axis = resolve_agent_state(state);
    let source_axis = resolve_agent_source(state, disabled_source, invalid_source);
    let stream_support = FieldGroupAgentStreamSupport::Optional;
    let output_status = FieldGroupAgentOutputStatus::Verified;

    FieldGroupAgentContract {
        schema: FIELD_GROUP_AGENT_SCHEMA,
        schema_version: FIELD_GROUP_AGENT_SCHEMA_VERSION,
        intent: intent.as_data_attr(),
        action: action.as_data_attr(),
        state: state_axis.as_data_attr(),
        source: source_axis.as_data_attr(),
        source_disabled: disabled_source.as_data_attr(),
        source_invalid: invalid_source.as_data_attr(),
        source_aria: state.aria_source_attr,
        source_class: state.class_source_attr,
        stream_mode: FIELD_GROUP_DEFAULT_RENDER_MODE,
        stream_support: stream_support.as_data_attr(),
        stream_fallback: FIELD_GROUP_DEFAULT_RENDER_MODE,
        output_mode: FIELD_GROUP_DEFAULT_RENDER_MODE,
        output_status: output_status.as_data_attr(),
    }
}

pub struct FieldGroupContentInput {
    pub id_base: Option<String>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub aria_label: Option<String>,
    pub lang: Option<String>,
    pub class_name: Option<String>,
}

type FieldGroupCowStr = Cow<'static, str>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldGroupContent {
    pub id_base: FieldGroupCowStr,
    pub label_text: FieldGroupCowStr,
    pub description_text: FieldGroupCowStr,
    pub has_label: bool,
    pub has_description: bool,
    pub aria_label: FieldGroupCowStr,
    pub has_custom_aria_label: bool,
    pub lang: Option<FieldGroupCowStr>,
    pub class_name: Option<FieldGroupCowStr>,
    pub has_custom_class_name: bool,
}

fn normalize_optional_cow(value: Option<String>) -> Option<FieldGroupCowStr> {
    normalize_optional_text(value).map(Cow::Owned)
}

fn normalize_id_base_cow(value: Option<String>) -> FieldGroupCowStr {
    match normalize_optional_text(value) {
        Some(id_base) => Cow::Owned(id_base),
        None => Cow::Borrowed(DEFAULT_ID_BASE),
    }
}

fn normalize_aria_label_cow(value: Option<String>) -> (FieldGroupCowStr, bool) {
    match normalize_optional_text(value) {
        Some(label) => (Cow::Owned(label), true),
        None => (Cow::Borrowed(DEFAULT_ARIA_LABEL), false),
    }
}

pub fn resolve_content(input: FieldGroupContentInput) -> FieldGroupContent {
    let id_base = normalize_id_base_cow(input.id_base);
    let label = normalize_optional_cow(input.label);
    let description = normalize_optional_cow(input.description);
    let (aria_label, has_custom_aria_label) = normalize_aria_label_cow(input.aria_label);
    let lang = normalize_optional_cow(input.lang);
    let class_name = normalize_optional_cow(input.class_name);

    let has_label = label.is_some();
    let has_description = description.is_some();
    let has_custom_class_name = class_name.is_some();

    let label_text = match label {
        Some(text) => text,
        None => Cow::Borrowed(""),
    };
    let description_text = match description {
        Some(text) => text,
        None => Cow::Borrowed(""),
    };

    FieldGroupContent {
        id_base,
        label_text,
        description_text,
        has_label,
        has_description,
        aria_label,
        has_custom_aria_label,
        lang,
        class_name,
        has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "../../test/group/logic.rs"]
mod tests;
