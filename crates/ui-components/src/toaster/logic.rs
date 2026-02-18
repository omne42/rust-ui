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
    let _ = [
        ToasterAgentOutputStatus::Draft.as_attr(),
        ToasterAgentOutputStatus::Verified.as_attr(),
        ToasterAgentOutputStatus::Submittable.as_attr(),
    ];

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
        state.base_class.to_string(),
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
mod tests {
    use super::*;
    use crate::toaster::ToasterStoreSource;

    #[test]
    fn normalize_helpers_trim_and_guard_limits() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-toaster ".to_string())),
            Some("docs-toaster".to_string())
        );

        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(Some(" Alerts host ".to_string())),
            ("Alerts host".to_string(), true)
        );

        assert_eq!(normalize_max_toasts(0), 1);
        assert_eq!(normalize_max_toasts(2), 2);
    }

    #[test]
    fn normalize_props_centralizes_defaults_and_custom_source_flags() {
        let normalized = normalize_props(ToasterNormalizeInput {
            position: ToasterPosition::TopLeft,
            portal: DEFAULT_PORTAL,
            max_toasts: 0,
            aria_label: Some("  Alerts host ".to_string()),
            class_name: Some(" docs-toaster ".to_string()),
            motion: crate::toast::ToastMotion::default(),
        });

        assert_eq!(normalized.position, ToasterPosition::TopLeft);
        assert_eq!(normalized.portal, DEFAULT_PORTAL);
        assert_eq!(normalized.max_toasts, 1);
        assert_eq!(normalized.aria_label, "Alerts host");
        assert_eq!(normalized.class_name, Some("docs-toaster".to_string()));
        assert!(normalized.has_custom_position);
        assert!(!normalized.has_custom_portal);
        assert!(normalized.has_custom_max_toasts);
        assert!(normalized.has_custom_aria_label);
        assert!(normalized.has_custom_class_name);
        assert!(!normalized.has_custom_motion);
    }

    #[test]
    fn queue_and_state_markers_follow_contract() {
        assert_eq!(state_attr(true), "portal");
        assert_eq!(state_attr(false), "inline");

        assert_eq!(queue_attr(1), "single");
        assert_eq!(queue_attr(3), "bounded");
        assert_eq!(queue_attr(6), "extended");
    }

    #[test]
    fn agent_contract_is_typed_and_stable() {
        let contract = agent_contract();
        assert_eq!(contract.schema_attr, "ui.toaster.v1");
        assert_eq!(
            contract.intent_attr,
            ToasterAgentIntent::NotificationHost.as_attr()
        );
        assert_eq!(
            contract.action_model_attr,
            ToasterAgentActionModel::PushClearDismiss.as_attr()
        );
        assert_eq!(
            contract.stream_support_attr,
            ToasterAgentStreamSupport::Optional.as_attr()
        );
        assert_eq!(
            contract.stream_fallback_attr,
            ToasterAgentStreamFallback::Snapshot.as_attr()
        );
        assert_eq!(
            contract.output_status_attr,
            ToasterAgentOutputStatus::Verified.as_attr()
        );
        assert_eq!(
            contract.state_axis_attr,
            "state|queue|position|portal|max-toasts"
        );
        assert_eq!(
            contract.source_axis_attr,
            "position|portal|max-toasts|aria|class|motion|store"
        );
        assert_eq!(ToasterAgentOutputStatus::Draft.as_attr(), "draft");
        assert_eq!(
            ToasterAgentOutputStatus::Submittable.as_attr(),
            "submittable"
        );
    }

    #[test]
    fn resolve_state_tracks_state_sources_and_store_origin() {
        let state = resolve_state(ToasterPartStateInput {
            slot: ToasterSlot::Root,
            position: ToasterPosition::TopCenter,
            portal: false,
            max_toasts: 0,
            has_custom_position: true,
            has_custom_portal: true,
            has_custom_max_toasts: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            store_source: ToasterStoreSource::Provided,
        });

        assert_eq!(state.slot_attr, "toaster");
        assert_eq!(state.base_class, "ui-toaster");
        assert_eq!(state.position_attr, "top-center");
        assert_eq!(state.state_attr, "inline");
        assert_eq!(state.portal_attr, "false");
        assert_eq!(state.max_toasts, 1);
        assert_eq!(state.queue_attr, "single");
        assert_eq!(state.position_source_attr, "custom");
        assert_eq!(state.portal_source_attr, "custom");
        assert_eq!(state.max_toasts_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.store_source_attr, "provided");
    }

    #[test]
    fn compose_class_name_includes_custom_markers() {
        let root_state = resolve_state(ToasterPartStateInput {
            slot: ToasterSlot::Root,
            position: ToasterPosition::BottomLeft,
            portal: true,
            max_toasts: 5,
            has_custom_position: true,
            has_custom_portal: false,
            has_custom_max_toasts: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            store_source: ToasterStoreSource::Local,
        });

        let class_name = compose_class_name(Some("docs-toaster".to_string()), root_state);
        assert!(class_name.contains("ui-toaster"));
        assert!(class_name.contains("ui-toaster--bottom-left"));
        assert!(class_name.contains("ui-toaster--portal"));
        assert!(class_name.contains("ui-toaster--custom-position"));
        assert!(class_name.contains("ui-toaster--custom-max-toasts"));
        assert!(class_name.contains("ui-toaster--custom-motion"));
        assert!(class_name.contains("ui-toaster--custom-class"));
        assert!(class_name.contains("ui-toaster--custom-aria"));
        assert!(class_name.contains("docs-toaster"));

        let sonner_state = resolve_state(ToasterPartStateInput {
            slot: ToasterSlot::Sonner,
            position: ToasterPosition::TopRight,
            portal: false,
            max_toasts: 2,
            has_custom_position: false,
            has_custom_portal: false,
            has_custom_max_toasts: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            has_custom_motion: false,
            store_source: ToasterStoreSource::Context,
        });

        let sonner_class = compose_class_name(None, sonner_state);
        assert_eq!(
            sonner_class,
            "ui-toaster__sonner ui-toaster__sonner--top-right ui-toaster__sonner--inline"
        );
    }

    #[test]
    fn map_to_sonner_position_matches_all_variants() {
        assert_eq!(
            map_to_sonner_position(ToasterPosition::TopLeft),
            crate::sonner::SonnerPosition::TopLeft
        );
        assert_eq!(
            map_to_sonner_position(ToasterPosition::TopCenter),
            crate::sonner::SonnerPosition::TopCenter
        );
        assert_eq!(
            map_to_sonner_position(ToasterPosition::TopRight),
            crate::sonner::SonnerPosition::TopRight
        );
        assert_eq!(
            map_to_sonner_position(ToasterPosition::BottomLeft),
            crate::sonner::SonnerPosition::BottomLeft
        );
        assert_eq!(
            map_to_sonner_position(ToasterPosition::BottomCenter),
            crate::sonner::SonnerPosition::BottomCenter
        );
        assert_eq!(
            map_to_sonner_position(ToasterPosition::BottomRight),
            crate::sonner::SonnerPosition::BottomRight
        );
    }
}
