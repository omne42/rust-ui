pub const DEFAULT_OPEN: bool = false;
pub const DEFAULT_TRANSPARENT: bool = false;
pub const DEFAULT_DISABLED: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlayOpenMode {
    Controlled,
    Uncontrolled,
}

impl UnderlayOpenMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayOpenStateInput {
    pub has_is_open_prop: bool,
    pub has_open_prop: bool,
    pub default_open: Option<bool>,
    pub has_on_open_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayOpenState {
    pub default_open: bool,
    pub mode: UnderlayOpenMode,
    pub has_default_open: bool,
    pub has_open_change_handler: bool,
    pub open_prop_source_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
}

pub fn resolve_open_state(input: UnderlayOpenStateInput) -> UnderlayOpenState {
    let mode = if input.has_is_open_prop || input.has_open_prop {
        UnderlayOpenMode::Controlled
    } else {
        UnderlayOpenMode::Uncontrolled
    };
    let has_default_open = input.default_open.is_some();
    let open_source_attr = if matches!(mode, UnderlayOpenMode::Controlled) {
        "external"
    } else if has_default_open {
        "default"
    } else {
        "implicit-default"
    };

    UnderlayOpenState {
        default_open: input.default_open.unwrap_or(DEFAULT_OPEN),
        mode,
        has_default_open,
        has_open_change_handler: input.has_on_open_change,
        open_prop_source_attr: if input.has_is_open_prop {
            "is_open"
        } else if input.has_open_prop {
            "open"
        } else {
            "none"
        },
        open_mode_attr: mode.as_attr(),
        open_source_attr,
        open_change_source_attr: if input.has_on_open_change {
            "provided"
        } else {
            "none"
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

pub fn resolve_flags(input: UnderlayFlagsInput) -> UnderlayFlags {
    UnderlayFlags {
        transparent: input
            .is_transparent
            .or(input.transparent)
            .unwrap_or(DEFAULT_TRANSPARENT),
        disabled: input
            .is_disabled
            .or(input.disabled)
            .unwrap_or(DEFAULT_DISABLED),
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
pub enum UnderlaySlot {
    Root,
}

impl UnderlaySlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            UnderlaySlot::Root => "underlay",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            UnderlaySlot::Root => "ui-underlay",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayPartStateInput {
    pub slot: UnderlaySlot,
    pub open: bool,
    pub transparent: bool,
    pub disabled: bool,
    pub has_on_close: bool,
    pub has_custom_transparent: bool,
    pub has_custom_disabled: bool,
    pub has_custom_close_handler: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayPartState {
    pub slot: UnderlaySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub tone_attr: &'static str,
    pub close_mode_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub transparent_attr: Option<&'static str>,
    pub disabled_attr: Option<&'static str>,
    pub interactive_attr: Option<&'static str>,
    pub is_open: bool,
    pub is_transparent: bool,
    pub is_disabled: bool,
    pub is_interactive: bool,
    pub has_custom_transparent: bool,
    pub has_custom_disabled: bool,
    pub has_custom_close_handler: bool,
    pub has_custom_class_name: bool,
    pub transparent_source_attr: &'static str,
    pub disabled_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayViewStateInput {
    pub slot: UnderlaySlot,
    pub open: bool,
    pub has_on_close: bool,
    pub has_custom_class_name: bool,
    pub open_state: UnderlayOpenState,
    pub flags: UnderlayFlags,
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

pub fn state_attr(is_open: bool, is_disabled: bool) -> &'static str {
    if is_disabled {
        "disabled"
    } else if is_open {
        "open"
    } else {
        "closed"
    }
}

pub fn tone_attr(is_transparent: bool) -> &'static str {
    if is_transparent {
        "transparent"
    } else {
        "scrim"
    }
}

pub fn close_mode_attr(is_interactive: bool) -> &'static str {
    if is_interactive {
        "interactive"
    } else {
        "static"
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: UnderlayPartStateInput) -> UnderlayPartState {
    let is_open = input.open && !input.disabled;
    let is_interactive = is_open && input.has_on_close;

    UnderlayPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(is_open, input.disabled),
        tone_attr: tone_attr(input.transparent),
        close_mode_attr: close_mode_attr(is_interactive),
        open_attr: is_open.then_some("true"),
        transparent_attr: input.transparent.then_some("true"),
        disabled_attr: input.disabled.then_some("true"),
        interactive_attr: is_interactive.then_some("true"),
        is_open,
        is_transparent: input.transparent,
        is_disabled: input.disabled,
        is_interactive,
        has_custom_transparent: input.has_custom_transparent,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_close_handler: input.has_custom_close_handler,
        has_custom_class_name: input.has_custom_class_name,
        transparent_source_attr: source_attr(input.has_custom_transparent),
        disabled_source_attr: source_attr(input.has_custom_disabled),
        close_source_attr: source_attr(input.has_custom_close_handler),
        class_source_attr: source_attr(input.has_custom_class_name),
    }
}

pub fn resolve_view_state(input: UnderlayViewStateInput) -> UnderlayViewState {
    let part = resolve_state(UnderlayPartStateInput {
        slot: input.slot,
        open: input.open,
        transparent: input.flags.transparent,
        disabled: input.flags.disabled,
        has_on_close: input.has_on_close,
        has_custom_transparent: input.flags.transparent != DEFAULT_TRANSPARENT,
        has_custom_disabled: input.flags.disabled != DEFAULT_DISABLED,
        has_custom_close_handler: input.has_on_close,
        has_custom_class_name: input.has_custom_class_name,
    });

    UnderlayViewState {
        part,
        open_mode: input.open_state.mode,
        open_mode_attr: input.open_state.open_mode_attr,
        open_source_attr: input.open_state.open_source_attr,
        open_change_source_attr: input.open_state.open_change_source_attr,
        open_prop_source_attr: input.open_state.open_prop_source_attr,
        transparent_prop_source_attr: input.flags.transparent_prop_source_attr,
        disabled_prop_source_attr: input.flags.disabled_prop_source_attr,
    }
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

#[cfg(test)]
#[path = "test/underlay.rs"]
mod tests;
