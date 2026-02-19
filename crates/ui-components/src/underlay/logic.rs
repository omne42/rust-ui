use crate::underlay::UnderlayPartState;
use leptos::prelude::*;

pub use ui_state_primitives::underlay::{
    DEFAULT_DISABLED, DEFAULT_OPEN, DEFAULT_TRANSPARENT, UnderlayPartStateInput, UnderlaySlot,
    resolve_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlayOpenMode {
    Controlled,
    Uncontrolled,
}

impl UnderlayOpenMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

pub struct UnderlayOpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct UnderlayOpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: bool,
    pub on_open_change: Option<Callback<bool>>,
    pub mode: UnderlayOpenMode,
    pub has_default_open: bool,
    pub has_open_change_handler: bool,
    pub open_prop_source_attr: &'static str,
}

pub fn normalize_open_state(input: UnderlayOpenStateInput) -> UnderlayOpenState {
    let open = input.is_open.or(input.open);
    let mode = if open.is_some() {
        UnderlayOpenMode::Controlled
    } else {
        UnderlayOpenMode::Uncontrolled
    };

    UnderlayOpenState {
        open,
        default_open: input.default_open.unwrap_or(DEFAULT_OPEN),
        on_open_change: input.on_open_change,
        mode,
        has_default_open: input.default_open.is_some(),
        has_open_change_handler: input.on_open_change.is_some(),
        open_prop_source_attr: if input.is_open.is_some() {
            "is_open"
        } else if input.open.is_some() {
            "open"
        } else {
            "none"
        },
    }
}

pub struct UnderlayFlagsInput {
    pub is_transparent: Option<bool>,
    pub transparent: Option<bool>,
    pub is_disabled: Option<bool>,
    pub disabled: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayFlags {
    pub transparent: bool,
    pub disabled: bool,
    pub transparent_prop_source_attr: &'static str,
    pub disabled_prop_source_attr: &'static str,
}

pub fn normalize_flags(input: UnderlayFlagsInput) -> UnderlayFlags {
    let transparent = input
        .is_transparent
        .or(input.transparent)
        .unwrap_or(DEFAULT_TRANSPARENT);
    let disabled = input
        .is_disabled
        .or(input.disabled)
        .unwrap_or(DEFAULT_DISABLED);

    UnderlayFlags {
        transparent,
        disabled,
        transparent_prop_source_attr: if input.is_transparent.is_some() {
            "is_transparent"
        } else if input.transparent.is_some() {
            "transparent"
        } else {
            "none"
        },
        disabled_prop_source_attr: if input.is_disabled.is_some() {
            "is_disabled"
        } else if input.disabled.is_some() {
            "disabled"
        } else {
            "none"
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayViewState {
    pub part: UnderlayPartState,
    pub open_mode: UnderlayOpenMode,
    pub open_mode_attr: &'static str,
    pub open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
    pub open_prop_source_attr: &'static str,
    pub transparent_prop_source_attr: &'static str,
    pub disabled_prop_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlayAgentSchemaVersion {
    V1,
}

impl UnderlayAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlayAgentIntent {
    OverlayDismiss,
}

impl UnderlayAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OverlayDismiss => "overlay-dismiss",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlayAgentAction {
    RequestClose,
    StaticBarrier,
}

impl UnderlayAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestClose => "request-close",
            Self::StaticBarrier => "static-barrier",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlayAgentStateAxis {
    Open,
    Closed,
    Disabled,
}

impl UnderlayAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlayAgentSource {
    ControlledExternal,
    UncontrolledDefault,
    UncontrolledImplicitDefault,
}

impl UnderlayAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlledExternal => "controlled-external",
            Self::UncontrolledDefault => "uncontrolled-default",
            Self::UncontrolledImplicitDefault => "uncontrolled-implicit-default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlayAgentStreamSupport {
    Optional,
}

impl UnderlayAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlayAgentStreamFallback {
    Snapshot,
}

impl UnderlayAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayAgentCapabilities {
    pub can_dismiss: bool,
    pub can_external_sync: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayAgentContract {
    pub schema_name: &'static str,
    pub schema_version: UnderlayAgentSchemaVersion,
    pub intent: UnderlayAgentIntent,
    pub action: UnderlayAgentAction,
    pub state: UnderlayAgentStateAxis,
    pub source: UnderlayAgentSource,
    pub stream_support: UnderlayAgentStreamSupport,
    pub stream_fallback: UnderlayAgentStreamFallback,
    pub capabilities: UnderlayAgentCapabilities,
}

pub fn resolve_agent_state_axis(part: UnderlayPartState) -> UnderlayAgentStateAxis {
    if part.is_disabled {
        UnderlayAgentStateAxis::Disabled
    } else if part.is_open {
        UnderlayAgentStateAxis::Open
    } else {
        UnderlayAgentStateAxis::Closed
    }
}

pub fn resolve_agent_action(part: UnderlayPartState) -> UnderlayAgentAction {
    if part.is_interactive {
        UnderlayAgentAction::RequestClose
    } else {
        UnderlayAgentAction::StaticBarrier
    }
}

pub fn resolve_agent_source(
    open_mode: UnderlayOpenMode,
    open_source_attr: &'static str,
) -> UnderlayAgentSource {
    if matches!(open_mode, UnderlayOpenMode::Controlled) {
        UnderlayAgentSource::ControlledExternal
    } else if open_source_attr == "default" {
        UnderlayAgentSource::UncontrolledDefault
    } else {
        UnderlayAgentSource::UncontrolledImplicitDefault
    }
}

pub fn resolve_agent_contract(state: UnderlayViewState) -> UnderlayAgentContract {
    let source = resolve_agent_source(state.open_mode, state.open_source_attr);
    let agent_state = resolve_agent_state_axis(state.part);
    let action = resolve_agent_action(state.part);

    UnderlayAgentContract {
        schema_name: "ui.underlay.agent-contract",
        schema_version: UnderlayAgentSchemaVersion::V1,
        intent: UnderlayAgentIntent::OverlayDismiss,
        action,
        state: agent_state,
        source,
        stream_support: UnderlayAgentStreamSupport::Optional,
        stream_fallback: UnderlayAgentStreamFallback::Snapshot,
        capabilities: UnderlayAgentCapabilities {
            can_dismiss: matches!(action, UnderlayAgentAction::RequestClose),
            can_external_sync: matches!(source, UnderlayAgentSource::ControlledExternal),
        },
    }
}

#[derive(Clone)]
pub struct UnderlayViewStateInput {
    pub slot: UnderlaySlot,
    pub open: bool,
    pub transparent: bool,
    pub disabled: bool,
    pub has_on_close: bool,
    pub has_custom_class_name: bool,
    pub open_state: UnderlayOpenState,
    pub flags: UnderlayFlags,
}

pub fn resolve_view_state(input: UnderlayViewStateInput) -> UnderlayViewState {
    let part = resolve_state(UnderlayPartStateInput {
        slot: input.slot,
        open: input.open,
        transparent: input.transparent,
        disabled: input.disabled,
        has_on_close: input.has_on_close,
        has_custom_transparent: input.transparent != DEFAULT_TRANSPARENT,
        has_custom_disabled: input.disabled != DEFAULT_DISABLED,
        has_custom_close_handler: input.has_on_close,
        has_custom_class_name: input.has_custom_class_name,
    });

    let open_mode_attr = input.open_state.mode.as_attr();
    let open_source_attr = if matches!(input.open_state.mode, UnderlayOpenMode::Controlled) {
        "external"
    } else if input.open_state.has_default_open {
        "default"
    } else {
        "implicit-default"
    };
    let open_change_source_attr = if input.open_state.has_open_change_handler {
        "provided"
    } else {
        "none"
    };

    UnderlayViewState {
        part,
        open_mode: input.open_state.mode,
        open_mode_attr,
        open_source_attr,
        open_change_source_attr,
        open_prop_source_attr: input.open_state.open_prop_source_attr,
        transparent_prop_source_attr: input.flags.transparent_prop_source_attr,
        disabled_prop_source_attr: input.flags.disabled_prop_source_attr,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn compose_class_name(base_class_name: Option<String>, state: UnderlayPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if state.is_open {
        classes.push("ui-underlay--open".to_string());
    }

    if state.is_transparent {
        classes.push("ui-underlay--transparent".to_string());
    }

    if state.is_disabled {
        classes.push("ui-underlay--disabled".to_string());
    }

    if state.is_interactive {
        classes.push("ui-underlay--interactive".to_string());
    }

    if state.has_custom_transparent {
        classes.push("ui-underlay--custom-transparent".to_string());
    }

    if state.has_custom_disabled {
        classes.push("ui-underlay--custom-disabled".to_string());
    }

    if state.has_custom_close_handler {
        classes.push("ui-underlay--custom-close".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-underlay--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_helpers_trim_text_and_filter_blanks() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-underlay ".to_string())),
            Some("docs-underlay".to_string())
        );
    }

    #[test]
    fn normalize_open_state_supports_is_open_open_and_default_open() {
        let controlled = normalize_open_state(UnderlayOpenStateInput {
            is_open: Some(Signal::derive(|| true)),
            open: None,
            default_open: Some(false),
            on_open_change: None,
        });
        assert!(matches!(controlled.mode, UnderlayOpenMode::Controlled));
        assert!(controlled.open.is_some());
        assert_eq!(controlled.open_prop_source_attr, "is_open");

        let legacy_controlled = normalize_open_state(UnderlayOpenStateInput {
            is_open: None,
            open: Some(Signal::derive(|| false)),
            default_open: Some(true),
            on_open_change: None,
        });
        assert!(matches!(
            legacy_controlled.mode,
            UnderlayOpenMode::Controlled
        ));
        assert_eq!(legacy_controlled.open_prop_source_attr, "open");

        let uncontrolled = normalize_open_state(UnderlayOpenStateInput {
            is_open: None,
            open: None,
            default_open: Some(true),
            on_open_change: Some(Callback::new(|_: bool| {})),
        });
        assert!(matches!(uncontrolled.mode, UnderlayOpenMode::Uncontrolled));
        assert!(uncontrolled.default_open);
        assert!(uncontrolled.has_default_open);
        assert!(uncontrolled.has_open_change_handler);
    }

    #[test]
    fn normalize_flags_prefers_is_prefixed_inputs() {
        let flags = normalize_flags(UnderlayFlagsInput {
            is_transparent: Some(true),
            transparent: Some(false),
            is_disabled: Some(true),
            disabled: Some(false),
        });

        assert!(flags.transparent);
        assert!(flags.disabled);
        assert_eq!(flags.transparent_prop_source_attr, "is_transparent");
        assert_eq!(flags.disabled_prop_source_attr, "is_disabled");
    }

    #[test]
    fn resolve_view_state_annotates_open_and_prop_sources() {
        let open_state = normalize_open_state(UnderlayOpenStateInput {
            is_open: None,
            open: None,
            default_open: Some(true),
            on_open_change: Some(Callback::new(|_: bool| {})),
        });
        let flags = normalize_flags(UnderlayFlagsInput {
            is_transparent: None,
            transparent: Some(true),
            is_disabled: None,
            disabled: Some(false),
        });

        let state = resolve_view_state(UnderlayViewStateInput {
            slot: UnderlaySlot::Root,
            open: true,
            transparent: flags.transparent,
            disabled: flags.disabled,
            has_on_close: true,
            has_custom_class_name: true,
            open_state,
            flags,
        });

        assert_eq!(state.part.state_attr, "open");
        assert_eq!(state.part.close_mode_attr, "interactive");
        assert_eq!(state.open_mode_attr, "uncontrolled");
        assert_eq!(state.open_source_attr, "default");
        assert_eq!(state.open_change_source_attr, "provided");
        assert_eq!(state.transparent_prop_source_attr, "transparent");
        assert_eq!(state.disabled_prop_source_attr, "disabled");
    }

    #[test]
    fn resolve_agent_contract_is_schema_typed_and_traceable() {
        let controlled_state = resolve_view_state(UnderlayViewStateInput {
            slot: UnderlaySlot::Root,
            open: true,
            transparent: false,
            disabled: false,
            has_on_close: true,
            has_custom_class_name: false,
            open_state: normalize_open_state(UnderlayOpenStateInput {
                is_open: Some(Signal::derive(|| true)),
                open: None,
                default_open: None,
                on_open_change: Some(Callback::new(|_: bool| {})),
            }),
            flags: normalize_flags(UnderlayFlagsInput {
                is_transparent: None,
                transparent: None,
                is_disabled: None,
                disabled: None,
            }),
        });
        let controlled_contract = resolve_agent_contract(controlled_state);
        assert_eq!(
            controlled_contract.schema_name,
            "ui.underlay.agent-contract"
        );
        assert_eq!(controlled_contract.schema_version.as_str(), "1");
        assert_eq!(controlled_contract.intent.as_str(), "overlay-dismiss");
        assert_eq!(controlled_contract.action.as_str(), "request-close");
        assert_eq!(controlled_contract.state.as_str(), "open");
        assert_eq!(controlled_contract.source.as_str(), "controlled-external");
        assert_eq!(controlled_contract.stream_support.as_str(), "optional");
        assert_eq!(controlled_contract.stream_fallback.as_str(), "snapshot");
        assert!(controlled_contract.capabilities.can_dismiss);
        assert!(controlled_contract.capabilities.can_external_sync);

        let disabled_state = resolve_view_state(UnderlayViewStateInput {
            slot: UnderlaySlot::Root,
            open: false,
            transparent: false,
            disabled: true,
            has_on_close: false,
            has_custom_class_name: false,
            open_state: normalize_open_state(UnderlayOpenStateInput {
                is_open: None,
                open: None,
                default_open: None,
                on_open_change: None,
            }),
            flags: normalize_flags(UnderlayFlagsInput {
                is_transparent: None,
                transparent: None,
                is_disabled: Some(true),
                disabled: None,
            }),
        });
        let disabled_contract = resolve_agent_contract(disabled_state);
        assert_eq!(disabled_contract.action.as_str(), "static-barrier");
        assert_eq!(disabled_contract.state.as_str(), "disabled");
        assert_eq!(
            disabled_contract.source.as_str(),
            "uncontrolled-implicit-default"
        );
        assert_eq!(disabled_contract.stream_support.as_str(), "optional");
        assert_eq!(disabled_contract.stream_fallback.as_str(), "snapshot");
        assert!(!disabled_contract.capabilities.can_dismiss);
        assert!(!disabled_contract.capabilities.can_external_sync);
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let part = resolve_state(UnderlayPartStateInput {
            slot: UnderlaySlot::Root,
            open: true,
            transparent: false,
            disabled: true,
            has_on_close: true,
            has_custom_transparent: false,
            has_custom_disabled: true,
            has_custom_close_handler: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-underlay".to_string()), part);
        assert!(class_name.contains("ui-underlay"));
        assert!(class_name.contains("ui-underlay--disabled"));
        assert!(class_name.contains("ui-underlay--custom-disabled"));
        assert!(class_name.contains("ui-underlay--custom-close"));
        assert!(class_name.contains("ui-underlay--custom-class"));
        assert!(class_name.contains("docs-underlay"));
    }
}
