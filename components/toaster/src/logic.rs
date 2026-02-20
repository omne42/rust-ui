use crate::toaster::{ToasterPartState, ToasterPartStateInput, ToasterPosition, ToasterSlot};
use ui_state_primitives::toaster as toaster_state;

pub use ui_state_primitives::toaster::{DEFAULT_ARIA_LABEL, DEFAULT_MAX_TOASTS, DEFAULT_PORTAL};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToasterAgentIntent {
    NotificationHost,
}

impl ToasterAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::NotificationHost => "notification-host",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToasterAgentActionModel {
    PushClearDismiss,
}

impl ToasterAgentActionModel {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::PushClearDismiss => "push|clear|dismiss",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToasterAgentStreamSupport {
    Optional,
}

impl ToasterAgentStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToasterAgentStreamFallback {
    Snapshot,
}

impl ToasterAgentStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToasterAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl ToasterAgentOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToasterAgentContract {
    pub schema_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_model_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
    pub state_axis_attr: &'static str,
    pub source_axis_attr: &'static str,
}

pub fn agent_contract() -> ToasterAgentContract {
    std::hint::black_box([
        ToasterAgentOutputStatus::Draft.as_attr(),
        ToasterAgentOutputStatus::Verified.as_attr(),
        ToasterAgentOutputStatus::Submittable.as_attr(),
    ]);

    ToasterAgentContract {
        schema_attr: "ui.toaster.v1",
        intent_attr: ToasterAgentIntent::NotificationHost.as_attr(),
        action_model_attr: ToasterAgentActionModel::PushClearDismiss.as_attr(),
        stream_support_attr: ToasterAgentStreamSupport::Optional.as_attr(),
        stream_fallback_attr: ToasterAgentStreamFallback::Snapshot.as_attr(),
        output_status_attr: ToasterAgentOutputStatus::Verified.as_attr(),
        state_axis_attr: "state|queue|position|portal|max-toasts",
        source_axis_attr: "position|portal|max-toasts|aria|class|motion|store",
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToasterNormalizeInput {
    pub position: ToasterPosition,
    pub portal: bool,
    pub max_toasts: usize,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub motion: crate::toast::ToastMotion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToasterNormalizedProps {
    pub position: ToasterPosition,
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

pub fn normalize_props(input: ToasterNormalizeInput) -> ToasterNormalizedProps {
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);

    ToasterNormalizedProps {
        position: input.position,
        portal: input.portal,
        max_toasts: normalize_max_toasts(input.max_toasts),
        aria_label,
        class_name,
        has_custom_position: input.position != ToasterPosition::default(),
        has_custom_portal: input.portal != DEFAULT_PORTAL,
        has_custom_max_toasts: input.max_toasts != DEFAULT_MAX_TOASTS,
        has_custom_aria_label,
        has_custom_class_name,
        has_custom_motion: input.motion != crate::toast::ToastMotion::default(),
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    toaster_state::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    toaster_state::normalize_aria_label(value)
}

pub fn normalize_max_toasts(max_toasts: usize) -> usize {
    toaster_state::normalize_max_toasts(max_toasts)
}

#[cfg(test)]
pub fn state_attr(portal: bool) -> &'static str {
    toaster_state::state_attr(portal)
}

#[cfg(test)]
pub fn queue_attr(max_toasts: usize) -> &'static str {
    toaster_state::queue_attr(max_toasts)
}

pub fn resolve_state(input: ToasterPartStateInput) -> ToasterPartState /* delegated */ {
    toaster_state::resolve_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: ToasterPartState) -> String {
    let mut classes = vec![
        state.base_class.into(),
        format!("{}--{}", state.base_class, state.position.class_suffix()),
    ];

    if state.portal {
        classes.push(format!("{}--portal", state.base_class));
    } else {
        classes.push(format!("{}--inline", state.base_class));
    }

    if state.slot == ToasterSlot::Root {
        if state.has_custom_class_name {
            classes.push("ui-toaster--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }

        if state.has_custom_aria_label {
            classes.push("ui-toaster--custom-aria".to_string());
        }

        if state.has_custom_position {
            classes.push("ui-toaster--custom-position".to_string());
        }

        if state.has_custom_portal {
            classes.push("ui-toaster--custom-portal".to_string());
        }

        if state.has_custom_max_toasts {
            classes.push("ui-toaster--custom-max-toasts".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-toaster--custom-motion".to_string());
        }
    }

    classes.join(" ")
}

pub fn map_to_sonner_position(position: ToasterPosition) -> crate::sonner::SonnerPosition {
    match position {
        ToasterPosition::TopLeft => crate::sonner::SonnerPosition::TopLeft,
        ToasterPosition::TopCenter => crate::sonner::SonnerPosition::TopCenter,
        ToasterPosition::TopRight => crate::sonner::SonnerPosition::TopRight,
        ToasterPosition::BottomLeft => crate::sonner::SonnerPosition::BottomLeft,
        ToasterPosition::BottomCenter => crate::sonner::SonnerPosition::BottomCenter,
        ToasterPosition::BottomRight => crate::sonner::SonnerPosition::BottomRight,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
