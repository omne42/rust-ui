use crate::sonner::{SonnerPartState, SonnerPartStateInput, SonnerPosition, SonnerSlot};
use crate::toast::ToastMotion;
use ui_state_primitives::sonner as sonner_state;

pub use ui_state_primitives::sonner::{DEFAULT_ARIA_LABEL, DEFAULT_MAX_TOASTS, DEFAULT_PORTAL};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SonnerAgentIntent {
    NotificationHost,
}

impl SonnerAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::NotificationHost => "notification-host",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SonnerAgentActionModel {
    PushClearDismiss,
}

impl SonnerAgentActionModel {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::PushClearDismiss => "push|clear|dismiss",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SonnerAgentStreamSupport {
    Optional,
}

impl SonnerAgentStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SonnerAgentStreamFallback {
    Snapshot,
}

impl SonnerAgentStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SonnerAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl SonnerAgentOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SonnerAgentContract {
    pub schema_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_model_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
    pub state_axis_attr: &'static str,
    pub source_axis_attr: &'static str,
}

pub fn agent_contract() -> SonnerAgentContract {
    std::hint::black_box([
        SonnerAgentOutputStatus::Draft.as_attr(),
        SonnerAgentOutputStatus::Verified.as_attr(),
        SonnerAgentOutputStatus::Submittable.as_attr(),
    ]);

    SonnerAgentContract {
        schema_attr: "ui.sonner.v1",
        intent_attr: SonnerAgentIntent::NotificationHost.as_attr(),
        action_model_attr: SonnerAgentActionModel::PushClearDismiss.as_attr(),
        stream_support_attr: SonnerAgentStreamSupport::Optional.as_attr(),
        stream_fallback_attr: SonnerAgentStreamFallback::Snapshot.as_attr(),
        output_status_attr: SonnerAgentOutputStatus::Verified.as_attr(),
        state_axis_attr: "state|queue|position|portal|max-toasts",
        source_axis_attr: "position|portal|max-toasts|aria|class|motion|store",
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SonnerNormalizeInput {
    pub position: SonnerPosition,
    pub portal: bool,
    pub max_toasts: usize,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub motion: ToastMotion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SonnerNormalizedProps {
    pub position: SonnerPosition,
    pub portal: bool,
    pub max_toasts: usize,
    pub aria_label: String,
    pub class_name: Option<String>,
    pub has_custom_position: bool,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

pub fn normalize_props(input: SonnerNormalizeInput) -> SonnerNormalizedProps {
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);

    SonnerNormalizedProps {
        position: input.position,
        portal: input.portal,
        max_toasts: normalize_max_toasts(input.max_toasts),
        aria_label,
        class_name,
        has_custom_position: input.position != SonnerPosition::default(),
        has_custom_portal: input.portal != DEFAULT_PORTAL,
        has_custom_max_toasts: input.max_toasts != DEFAULT_MAX_TOASTS,
        has_custom_aria_label,
        has_custom_class_name,
        has_custom_motion: input.motion != ToastMotion::default(),
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    sonner_state::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    sonner_state::normalize_aria_label(value)
}

pub fn normalize_max_toasts(max_toasts: usize) -> usize {
    sonner_state::normalize_max_toasts(max_toasts)
}

pub fn resolve_state(input: SonnerPartStateInput) -> SonnerPartState {
    sonner_state::resolve_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: SonnerPartState) -> String {
    let mut classes = vec![
        state.base_class.into(),
        format!("{}--{}", state.base_class, state.position.class_suffix()),
    ];

    if state.portal {
        classes.push(format!("{}--portal", state.base_class));
    } else {
        classes.push(format!("{}--inline", state.base_class));
    }

    if state.slot == SonnerSlot::Root {
        if state.has_custom_class_name {
            classes.push("ui-sonner--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }

        if state.has_custom_aria_label {
            classes.push("ui-sonner--custom-aria".to_string());
        }

        if state.has_custom_position {
            classes.push("ui-sonner--custom-position".to_string());
        }

        if state.has_custom_portal {
            classes.push("ui-sonner--custom-portal".to_string());
        }

        if state.has_custom_max_toasts {
            classes.push("ui-sonner--custom-max-toasts".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-sonner--custom-motion".to_string());
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/sonner/logic.rs"]
mod tests;
