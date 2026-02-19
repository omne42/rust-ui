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
mod tests {
    use super::*;
    use crate::sonner::{SonnerPartStateInput, SonnerPosition, SonnerStoreSource};

    #[test]
    fn normalize_helpers_trim_and_guard_limits() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-sonner ".to_string())),
            Some("docs-sonner".to_string())
        );

        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
        assert_eq!(
            normalize_aria_label(Some(" Status host ".to_string())),
            ("Status host".to_string(), true)
        );

        assert_eq!(normalize_max_toasts(0), 1);
        assert_eq!(normalize_max_toasts(2), 2);
    }

    #[test]
    fn normalize_props_centralizes_defaults_and_custom_source_flags() {
        let normalized = normalize_props(SonnerNormalizeInput {
            position: SonnerPosition::TopCenter,
            portal: false,
            max_toasts: 0,
            aria_label: Some(" Status host ".to_string()),
            class_name: Some(" docs-sonner ".to_string()),
            motion: ToastMotion {
                initial_y_px: 22.0,
                initial_scale: 0.94,
                ..ToastMotion::default()
            },
        });

        assert_eq!(normalized.position, SonnerPosition::TopCenter);
        assert!(!normalized.portal);
        assert_eq!(normalized.max_toasts, 1);
        assert_eq!(normalized.aria_label, "Status host");
        assert_eq!(normalized.class_name, Some("docs-sonner".to_string()));
        assert!(normalized.has_custom_position);
        assert!(normalized.has_custom_portal);
        assert!(normalized.has_custom_max_toasts);
        assert!(normalized.has_custom_aria_label);
        assert!(normalized.has_custom_class_name);
        assert!(normalized.has_custom_motion);
    }

    #[test]
    fn queue_and_state_markers_follow_contract() {
        assert_eq!(sonner_state::state_attr(true), "portal");
        assert_eq!(sonner_state::state_attr(false), "inline");

        assert_eq!(sonner_state::queue_attr(1), "single");
        assert_eq!(sonner_state::queue_attr(3), "bounded");
        assert_eq!(sonner_state::queue_attr(6), "extended");
    }

    #[test]
    fn resolve_state_tracks_state_sources_and_store_origin() {
        let state = resolve_state(SonnerPartStateInput {
            slot: SonnerSlot::Root,
            position: SonnerPosition::TopCenter,
            portal: false,
            max_toasts: 0,
            has_custom_position: true,
            has_custom_portal: true,
            has_custom_max_toasts: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            store_source: SonnerStoreSource::Provided,
        });

        assert_eq!(state.slot_attr, "sonner");
        assert_eq!(state.base_class, "ui-sonner");
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
        let root_state = resolve_state(SonnerPartStateInput {
            slot: SonnerSlot::Root,
            position: SonnerPosition::BottomLeft,
            portal: true,
            max_toasts: 5,
            has_custom_position: true,
            has_custom_portal: false,
            has_custom_max_toasts: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            store_source: SonnerStoreSource::Local,
        });

        let class_name = compose_class_name(Some("docs-sonner".to_string()), root_state);
        assert!(class_name.contains("ui-sonner"));
        assert!(class_name.contains("ui-sonner--bottom-left"));
        assert!(class_name.contains("ui-sonner--portal"));
        assert!(class_name.contains("ui-sonner--custom-position"));
        assert!(class_name.contains("ui-sonner--custom-max-toasts"));
        assert!(class_name.contains("ui-sonner--custom-motion"));
        assert!(class_name.contains("ui-sonner--custom-class"));
        assert!(class_name.contains("ui-sonner--custom-aria"));
        assert!(class_name.contains("docs-sonner"));

        let viewport_state = resolve_state(SonnerPartStateInput {
            slot: SonnerSlot::Viewport,
            position: SonnerPosition::TopRight,
            portal: false,
            max_toasts: 2,
            has_custom_position: false,
            has_custom_portal: false,
            has_custom_max_toasts: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            has_custom_motion: false,
            store_source: SonnerStoreSource::Context,
        });

        let viewport_class = compose_class_name(None, viewport_state);
        assert_eq!(
            viewport_class,
            "ui-sonner__viewport ui-sonner__viewport--top-right ui-sonner__viewport--inline"
        );
    }

    #[test]
    fn agent_contract_is_stable() {
        let contract = agent_contract();

        assert_eq!(contract.schema_attr, "ui.sonner.v1");
        assert_eq!(contract.intent_attr, "notification-host");
        assert_eq!(contract.action_model_attr, "push|clear|dismiss");
        assert_eq!(contract.stream_support_attr, "optional");
        assert_eq!(contract.stream_fallback_attr, "snapshot");
        assert_eq!(contract.output_status_attr, "verified");
        assert_eq!(
            contract.state_axis_attr,
            "state|queue|position|portal|max-toasts"
        );
        assert_eq!(
            contract.source_axis_attr,
            "position|portal|max-toasts|aria|class|motion|store"
        );
    }
}
