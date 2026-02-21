use std::borrow::Cow;

pub use ui_headless::A11yDirection;
use ui_headless::{A11yLocaleAttrs, locale_attrs};
pub use ui_state_primitives::description::{
    DEFAULT_ARIA_LABEL, DEFAULT_TEXT, DescriptionState, DescriptionStateInput, DescriptionTone,
    normalize_aria_label, normalize_content, normalize_optional_text, resolve_state,
};

pub const DESCRIPTION_AGENT_SCHEMA: &str = "ui.description.agent-contract.v1";
pub const DESCRIPTION_AGENT_SCHEMA_VERSION: &str = "v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DescriptionElement {
    Span,
    #[default]
    Paragraph,
    Div,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescriptionAgentIntent {
    TextAssistance,
}

impl DescriptionAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::TextAssistance => "text-assistance",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescriptionAgentAction {
    RenderSnapshot,
}

impl DescriptionAgentAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::RenderSnapshot => "render-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescriptionAgentSource {
    Default,
    Custom,
}

impl DescriptionAgentSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescriptionAgentStreamSupport {
    Optional,
}

impl DescriptionAgentStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescriptionAgentStreamFallback {
    Snapshot,
}

impl DescriptionAgentStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescriptionAgentOutputStatus {
    Verified,
}

impl DescriptionAgentOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DescriptionAgentContractAttrs {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
}

pub fn resolve_agent_contract_attrs(state: DescriptionState) -> DescriptionAgentContractAttrs {
    let source = if state.aria_source_attr == "custom" || state.class_source_attr == "custom" {
        DescriptionAgentSource::Custom
    } else {
        DescriptionAgentSource::Default
    };

    DescriptionAgentContractAttrs {
        schema_attr: DESCRIPTION_AGENT_SCHEMA,
        schema_version_attr: DESCRIPTION_AGENT_SCHEMA_VERSION,
        intent_attr: DescriptionAgentIntent::TextAssistance.as_attr(),
        action_attr: DescriptionAgentAction::RenderSnapshot.as_attr(),
        state_attr: state.data_state_attr,
        source_attr: source.as_attr(),
        stream_support_attr: DescriptionAgentStreamSupport::Optional.as_attr(),
        stream_fallback_attr: DescriptionAgentStreamFallback::Snapshot.as_attr(),
        output_status_attr: DescriptionAgentOutputStatus::Verified.as_attr(),
    }
}

pub fn resolve_locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs {
    locale_attrs(lang, dir)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptionViewModelInput {
    pub text: String,
    pub tone: DescriptionTone,
    pub is_disabled: bool,
    pub is_truncated: bool,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptionViewModel {
    pub text: String,
    pub aria_label: String,
    pub class_name: Option<String>,
    pub state: DescriptionState,
    pub agent_contract: DescriptionAgentContractAttrs,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

pub fn resolve_view_model(input: DescriptionViewModelInput) -> DescriptionViewModel {
    let text = normalize_content(Some(input.text));
    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();
    let locale = resolve_locale_attrs(input.lang, input.dir);

    let state = resolve_state(DescriptionStateInput {
        tone: input.tone,
        disabled: input.is_disabled,
        truncate: input.is_truncated,
        has_custom_aria_label,
        has_custom_class_name,
    });

    DescriptionViewModel {
        text,
        aria_label,
        class_name,
        agent_contract: resolve_agent_contract_attrs(state),
        state,
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DescriptionState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-description"),
        Cow::Borrowed(state.tone_class),
    ];

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-description--disabled"));
    }

    if state.is_truncated {
        classes.push(Cow::Borrowed("ui-description--truncate"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-description--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .into_iter()
        .map(|class_name| class_name.into_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
