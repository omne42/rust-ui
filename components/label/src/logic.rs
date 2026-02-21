pub use ui_state_primitives::label::{
    DEFAULT_ARIA_LABEL, DEFAULT_REQUIRED_INDICATOR, LabelEmphasis, LabelState, LabelStateInput,
    compose_class_name, normalize_label_text, normalize_optional_text,
    normalize_required_indicator, resolve_state,
};

pub const LABEL_AGENT_SCHEMA: &str = "ui.label.agent-contract.v1";
pub const LABEL_AGENT_SCHEMA_VERSION: &str = "v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelAgentIntent {
    FormLabel,
}

impl LabelAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::FormLabel => "form-label",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelAgentAction {
    RenderSnapshot,
}

impl LabelAgentAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::RenderSnapshot => "render-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelAgentState {
    Required,
    Optional,
}

impl LabelAgentState {
    pub fn from_required(is_required: bool) -> Self {
        if is_required {
            Self::Required
        } else {
            Self::Optional
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelAgentSource {
    Default,
    Custom,
}

impl LabelAgentSource {
    pub fn from_state_and_motion(state: LabelState, motion_source_attr: &'static str) -> Self {
        if state.label_source_attr == "custom"
            || state.indicator_source_attr == "custom"
            || state.class_source_attr == "custom"
            || motion_source_attr == "custom"
        {
            Self::Custom
        } else {
            Self::Default
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelAgentStreamSupport {
    Optional,
}

impl LabelAgentStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelAgentStreamFallback {
    Snapshot,
}

impl LabelAgentStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelAgentOutputStatus {
    Verified,
}

impl LabelAgentOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelAgentContractAttrs {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
    pub label_source_attr: &'static str,
    pub indicator_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}

pub fn resolve_agent_contract_attrs(
    state: LabelState,
    motion_source_attr: &'static str,
) -> LabelAgentContractAttrs {
    let state_attr = LabelAgentState::from_required(state.is_required);
    let source_attr = LabelAgentSource::from_state_and_motion(state, motion_source_attr);

    LabelAgentContractAttrs {
        schema_attr: LABEL_AGENT_SCHEMA,
        schema_version_attr: LABEL_AGENT_SCHEMA_VERSION,
        intent_attr: LabelAgentIntent::FormLabel.as_attr(),
        action_attr: LabelAgentAction::RenderSnapshot.as_attr(),
        state_attr: state_attr.as_attr(),
        source_attr: source_attr.as_attr(),
        stream_support_attr: LabelAgentStreamSupport::Optional.as_attr(),
        stream_fallback_attr: LabelAgentStreamFallback::Snapshot.as_attr(),
        output_status_attr: LabelAgentOutputStatus::Verified.as_attr(),
        label_source_attr: state.label_source_attr,
        indicator_source_attr: state.indicator_source_attr,
        class_source_attr: state.class_source_attr,
        motion_source_attr,
    }
}

pub(super) struct LabelViewInput {
    pub text: Option<String>,
    pub for_id: Option<String>,
    pub required_indicator: Option<String>,
    pub class_name: Option<String>,
    pub lang: Option<String>,
}

pub(super) struct NormalizedLabelViewInput {
    pub text: String,
    pub for_id: Option<String>,
    pub required_indicator: String,
    pub class_name: Option<String>,
    pub lang: Option<String>,
    pub has_for_id: bool,
    pub has_custom_label: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
}

pub(super) struct LabelStateAxisInput {
    pub emphasis: LabelEmphasis,
    pub is_required: bool,
    pub is_disabled: bool,
}

pub(super) struct LabelRenderState {
    pub state: LabelState,
    pub class_name: String,
}

pub(super) fn normalize_view_input(input: LabelViewInput) -> NormalizedLabelViewInput {
    let (text, has_custom_label) = normalize_label_text(input.text);
    let (required_indicator, has_custom_indicator) =
        normalize_required_indicator(input.required_indicator);
    let for_id = normalize_optional_text(input.for_id);
    let class_name = normalize_optional_text(input.class_name);
    let lang = normalize_optional_text(input.lang);
    let has_for_id = for_id.is_some();
    let has_custom_class_name = class_name.is_some();

    NormalizedLabelViewInput {
        text,
        for_id,
        required_indicator,
        class_name,
        lang,
        has_for_id,
        has_custom_label,
        has_custom_indicator,
        has_custom_class_name,
    }
}

pub(super) fn derive_render_state(
    state_input: LabelStateAxisInput,
    normalized: &NormalizedLabelViewInput,
) -> LabelRenderState {
    let state = resolve_state(LabelStateInput {
        emphasis: state_input.emphasis,
        required: state_input.is_required,
        disabled: state_input.is_disabled,
        has_for_id: normalized.has_for_id,
        has_custom_label: normalized.has_custom_label,
        has_custom_indicator: normalized.has_custom_indicator,
        has_custom_class_name: normalized.has_custom_class_name,
    });
    let class_name = compose_class_name(normalized.class_name.clone(), state);

    LabelRenderState { state, class_name }
}
