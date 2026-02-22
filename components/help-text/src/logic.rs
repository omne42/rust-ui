pub use ui_headless::A11yDirection;
use ui_headless::{A11yLocaleAttrs, LiveRegionPriority, live_region_attrs, locale_attrs};
pub use ui_state_primitives::help_text::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, HelpTextDataState, HelpTextErrorSourceAttr,
    HelpTextMessageKind, HelpTextSourceAttr, HelpTextState, HelpTextStateInput, HelpTextTone,
    normalize_aria_label, normalize_error_message, normalize_optional_text, resolve_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelpTextLogicInput {
    pub tone: HelpTextTone,
    pub is_invalid: bool,
    pub is_disabled: bool,
    pub is_error_icon_visible: bool,
    pub description: Option<String>,
    pub error_message: Option<String>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelpTextRenderModel {
    pub aria_label: String,
    pub description_text: String,
    pub error_message_text: String,
    pub class_name: Option<String>,
    pub state: HelpTextState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpTextErrorLiveRegionAttrs {
    pub role: &'static str,
    pub aria_live: &'static str,
}

pub const HELP_TEXT_AGENT_SCHEMA: &str = "ui.help-text.agent-contract.v1";
pub const HELP_TEXT_AGENT_SCHEMA_VERSION: &str = "v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpTextAgentIntent {
    FormAssistance,
}

impl HelpTextAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            HelpTextAgentIntent::FormAssistance => "form-assistance",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpTextAgentAction {
    AnnounceError,
    ReadOnly,
}

impl HelpTextAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            HelpTextAgentAction::AnnounceError => "announce-error",
            HelpTextAgentAction::ReadOnly => "read-only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpTextAgentOutputStatus {
    Verified,
}

impl HelpTextAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            HelpTextAgentOutputStatus::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpTextAgentStreamSupport {
    Optional,
}

impl HelpTextAgentStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            HelpTextAgentStreamSupport::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpTextAgentStreamMode {
    Snapshot,
}

impl HelpTextAgentStreamMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            HelpTextAgentStreamMode::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpTextAgentContractAttrs {
    pub data_ui_schema: &'static str,
    pub data_ui_schema_version: &'static str,
    pub data_ui_intent: &'static str,
    pub data_ui_action: &'static str,
    pub data_ui_state: &'static str,
    pub data_ui_source: &'static str,
    pub data_ui_stream_support: &'static str,
    pub data_ui_stream_mode: &'static str,
    pub data_ui_stream_fallback: &'static str,
    pub data_ui_output_status: &'static str,
}

pub fn resolve_locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs {
    locale_attrs(normalize_optional_text(lang), dir)
}

pub fn resolve_error_live_region_attrs() -> HelpTextErrorLiveRegionAttrs {
    let attrs = live_region_attrs(LiveRegionPriority::Assertive);
    HelpTextErrorLiveRegionAttrs {
        role: attrs.role,
        aria_live: attrs.aria_live,
    }
}

pub fn resolve_display_text(value: Option<String>) -> String {
    value.unwrap_or_default()
}

pub fn resolve_agent_contract_attrs(state: HelpTextState) -> HelpTextAgentContractAttrs {
    let action = if state.message_kind == HelpTextMessageKind::Error {
        HelpTextAgentAction::AnnounceError
    } else {
        HelpTextAgentAction::ReadOnly
    };
    let source = if state.message_kind == HelpTextMessageKind::Error {
        state.error_source.as_attr()
    } else {
        state.aria_source.as_attr()
    };
    let stream_mode = HelpTextAgentStreamMode::Snapshot.as_attr();

    HelpTextAgentContractAttrs {
        data_ui_schema: HELP_TEXT_AGENT_SCHEMA,
        data_ui_schema_version: HELP_TEXT_AGENT_SCHEMA_VERSION,
        data_ui_intent: HelpTextAgentIntent::FormAssistance.as_attr(),
        data_ui_action: action.as_attr(),
        data_ui_state: state.data_state.as_attr(),
        data_ui_source: source,
        data_ui_stream_support: HelpTextAgentStreamSupport::Optional.as_attr(),
        data_ui_stream_mode: stream_mode,
        data_ui_stream_fallback: stream_mode,
        data_ui_output_status: HelpTextAgentOutputStatus::Verified.as_attr(),
    }
}

pub fn resolve_render_model(input: HelpTextLogicInput) -> HelpTextRenderModel {
    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);
    let description = normalize_optional_text(input.description);
    let (error_message, has_custom_error_message) =
        normalize_error_message(input.error_message, input.is_invalid);
    let class_name = normalize_optional_text(input.class_name);

    let state = resolve_state(HelpTextStateInput {
        tone: input.tone,
        invalid: input.is_invalid,
        disabled: input.is_disabled,
        show_error_icon: input.is_error_icon_visible,
        has_description: description.is_some(),
        has_error_message: error_message.is_some(),
        has_custom_aria_label,
        has_custom_error_message,
        has_custom_class_name: class_name.is_some(),
    });

    let description_text = if state.message_kind == HelpTextMessageKind::Description {
        resolve_display_text(description)
    } else {
        String::new()
    };
    let error_message_text = if state.message_kind == HelpTextMessageKind::Error {
        resolve_display_text(error_message)
    } else {
        String::new()
    };

    HelpTextRenderModel {
        aria_label,
        description_text,
        error_message_text,
        class_name,
        state,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: HelpTextState) -> String {
    fn push_class_token(classes: &mut String, token: &str) {
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(token);
    }

    let mut classes = String::new();
    push_class_token(&mut classes, "ui-help-text");
    push_class_token(&mut classes, state.tone_class);

    if state.is_invalid {
        push_class_token(&mut classes, "ui-help-text--invalid");
    }

    if state.is_disabled {
        push_class_token(&mut classes, "ui-help-text--disabled");
    }

    if state.show_error_icon {
        push_class_token(&mut classes, "ui-help-text--with-icon");
    }

    if state.has_error_message {
        push_class_token(&mut classes, "ui-help-text--has-error");
    }

    if state.has_description {
        push_class_token(&mut classes, "ui-help-text--has-description");
    }

    if state.has_custom_class_name {
        push_class_token(&mut classes, "ui-help-text--custom-class");
        if let Some(base_class_name) = base_class_name
            && !base_class_name.is_empty()
        {
            push_class_token(&mut classes, &base_class_name);
        }
    }

    classes
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
