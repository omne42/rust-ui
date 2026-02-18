pub use ui_state_primitives::header::{
    DEFAULT_ARIA_LABEL, HeaderState, HeaderStateInput, HeaderTone, normalize_aria_label,
    normalize_optional_text, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: HeaderState) -> String {
    let mut classes = vec!["ui-header".to_string(), state.tone_class.to_string()];

    if state.is_bordered {
        classes.push("ui-header--bordered".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-header--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderAgentAction {
    StaticHeader,
}

impl HeaderAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::StaticHeader => "static-header",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderAgentState {
    Default,
    Strong,
    Bordered,
    StrongBordered,
}

impl HeaderAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Strong => "strong",
            Self::Bordered => "bordered",
            Self::StrongBordered => "strong-bordered",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderAgentSource {
    ImplicitDefault,
    PropsStrong,
}

impl HeaderAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::ImplicitDefault => "implicit-default",
            Self::PropsStrong => "props-strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderAgentStreamSupport {
    Unsupported,
}

impl HeaderAgentStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderAgentStreamFallback {
    Snapshot,
}

impl HeaderAgentStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderAgentOutputStatus {
    Verified,
}

impl HeaderAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderAgentContract {
    pub schema_attr: &'static str,
    pub intent_attr: &'static str,
    pub action: HeaderAgentAction,
    pub state: HeaderAgentState,
    pub source: HeaderAgentSource,
    pub stream_support: HeaderAgentStreamSupport,
    pub stream_fallback: HeaderAgentStreamFallback,
    pub output_status: HeaderAgentOutputStatus,
}

pub fn resolve_agent_contract(state: HeaderState) -> HeaderAgentContract {
    let tone_source = if state.tone == HeaderTone::Strong {
        HeaderAgentSource::PropsStrong
    } else {
        HeaderAgentSource::ImplicitDefault
    };

    let state_axis = if state.is_bordered && state.tone == HeaderTone::Strong {
        HeaderAgentState::StrongBordered
    } else if state.is_bordered {
        HeaderAgentState::Bordered
    } else if state.tone == HeaderTone::Strong {
        HeaderAgentState::Strong
    } else {
        HeaderAgentState::Default
    };

    HeaderAgentContract {
        schema_attr: "ui.header",
        intent_attr: "section-heading",
        action: HeaderAgentAction::StaticHeader,
        state: state_axis,
        source: tone_source,
        stream_support: HeaderAgentStreamSupport::Unsupported,
        stream_fallback: HeaderAgentStreamFallback::Snapshot,
        output_status: HeaderAgentOutputStatus::Verified,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_primitives_are_reexported_from_ui_state_primitives() {
        let (label, custom) = normalize_aria_label(Some("  Header area  ".to_string()));
        assert_eq!(label, "Header area");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some(" ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn compose_class_name_includes_custom_marker_and_user_class() {
        let state = resolve_state(HeaderStateInput {
            tone: HeaderTone::Default,
            bordered: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-header-custom".to_string()), state);
        for token in [
            "ui-header",
            "ui-header--tone-default",
            "ui-header--bordered",
            "ui-header--custom-class",
            "docs-header-custom",
        ] {
            assert!(class_name.contains(token), "class should include `{token}`");
        }
    }

    #[test]
    fn resolve_agent_contract_uses_tone_and_bordered_axes() {
        let state = resolve_state(HeaderStateInput {
            tone: HeaderTone::Strong,
            bordered: true,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });
        let contract = resolve_agent_contract(state);

        assert_eq!(contract.schema_attr, "ui.header");
        assert_eq!(contract.intent_attr, "section-heading");
        assert_eq!(contract.action.as_attr(), "static-header");
        assert_eq!(contract.state.as_attr(), "strong-bordered");
        assert_eq!(contract.source.as_attr(), "props-strong");
        assert_eq!(contract.stream_support.as_attr(), "unsupported");
        assert_eq!(contract.stream_fallback.as_attr(), "snapshot");
        assert_eq!(contract.output_status.as_attr(), "verified");
    }
}
