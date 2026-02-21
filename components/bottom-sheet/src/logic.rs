use leptos::prelude::Callback;
pub use ui_state_primitives::bottom_sheet::{
    BottomSheetState, BottomSheetStateInput, compose_class_name, normalize_bottom_inset_px,
    normalize_id_base, normalize_optional_text, normalize_required_text, resolve_state,
};

pub const DEFAULT_TITLE: &str = "Bottom sheet";
pub const DEFAULT_CLOSE_LABEL: &str = "Close bottom sheet";
pub const DEFAULT_HANDLE_VISIBILITY: BottomSheetVisibility = BottomSheetVisibility::Visible;
pub const DEFAULT_CLOSE_BUTTON_VISIBILITY: BottomSheetVisibility = BottomSheetVisibility::Visible;
pub const DEFAULT_ATTACHMENT: BottomSheetAttachment = BottomSheetAttachment::Attached;
pub const DEFAULT_BOTTOM_INSET_PX: f64 = 0.0;
pub const DEFAULT_DISMISSABLE: bool = true;
pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;
pub const DEFAULT_MOTION_SOURCE_ATTR: &str = "default";
pub const CUSTOM_MOTION_SOURCE_ATTR: &str = "custom";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetVisibility {
    Visible,
    Hidden,
}

impl BottomSheetVisibility {
    pub fn is_visible(self) -> bool {
        matches!(self, Self::Visible)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAttachment {
    Attached,
    Detached,
}

impl BottomSheetAttachment {
    pub fn is_detached(self) -> bool {
        matches!(self, Self::Detached)
    }
}

pub fn resolve_title(value: String) -> String {
    normalize_required_text(value, DEFAULT_TITLE)
}

pub fn resolve_close_label(value: Option<&'static str>) -> &'static str {
    value
        .filter(|label| !label.trim().is_empty())
        .unwrap_or(DEFAULT_CLOSE_LABEL)
}

pub fn resolve_description_text(value: Option<String>) -> String {
    value.unwrap_or_default()
}

pub fn resolve_handle_visibility(
    is_handle_visible: Option<bool>,
    show_handle: Option<bool>,
) -> BottomSheetVisibility {
    if is_handle_visible
        .or(show_handle)
        .unwrap_or(DEFAULT_HANDLE_VISIBILITY.is_visible())
    {
        BottomSheetVisibility::Visible
    } else {
        BottomSheetVisibility::Hidden
    }
}

pub fn resolve_close_button_visibility(
    is_close_button_visible: Option<bool>,
    show_close_button: Option<bool>,
) -> BottomSheetVisibility {
    if is_close_button_visible
        .or(show_close_button)
        .unwrap_or(DEFAULT_CLOSE_BUTTON_VISIBILITY.is_visible())
    {
        BottomSheetVisibility::Visible
    } else {
        BottomSheetVisibility::Hidden
    }
}

pub fn resolve_attachment(
    is_detached: Option<bool>,
    detached: Option<bool>,
) -> BottomSheetAttachment {
    if is_detached
        .or(detached)
        .unwrap_or(DEFAULT_ATTACHMENT.is_detached())
    {
        BottomSheetAttachment::Detached
    } else {
        BottomSheetAttachment::Attached
    }
}

#[cfg(test)]
pub fn resolve_detached(
    is_detached: Option<bool>,
    detached: Option<bool>,
) -> BottomSheetAttachment {
    resolve_attachment(is_detached, detached)
}

pub fn resolve_bottom_inset_px(value: Option<f64>) -> f64 {
    normalize_bottom_inset_px(value.unwrap_or(DEFAULT_BOTTOM_INSET_PX))
}

pub fn resolve_dismissable(value: Option<bool>) -> bool {
    value.unwrap_or(DEFAULT_DISMISSABLE)
}

pub fn resolve_keyboard_dismiss_disabled(value: Option<bool>) -> bool {
    value.unwrap_or(DEFAULT_KEYBOARD_DISMISS_DISABLED)
}

pub fn resolve_on_exit_complete(on_exit_complete: Option<Callback<()>>) -> Callback<()> {
    on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}))
}

pub fn has_slot<T>(slot: &Option<T>) -> bool {
    slot.is_some()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BottomSheetDeriveInput {
    pub has_description: bool,
    pub has_footer: bool,
    pub handle_visibility: BottomSheetVisibility,
    pub close_button_visibility: BottomSheetVisibility,
    pub attachment: BottomSheetAttachment,
    pub bottom_inset_px: f64,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BottomSheetDerivedState {
    pub state: BottomSheetState,
    pub motion_source_attr: &'static str,
    pub has_custom_motion: bool,
}

pub fn derive_view_state(input: BottomSheetDeriveInput) -> BottomSheetDerivedState {
    let state = resolve_state(BottomSheetStateInput {
        has_description: input.has_description,
        has_footer: input.has_footer,
        show_handle: input.handle_visibility.is_visible(),
        show_close_button: input.close_button_visibility.is_visible(),
        detached: input.attachment.is_detached(),
        bottom_inset_px: input.bottom_inset_px,
        has_custom_class_name: input.has_custom_class_name,
    });

    BottomSheetDerivedState {
        state,
        motion_source_attr: if input.has_custom_motion {
            CUSTOM_MOTION_SOURCE_ATTR
        } else {
            DEFAULT_MOTION_SOURCE_ATTR
        },
        has_custom_motion: input.has_custom_motion,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentSchemaVersion {
    V1,
}

impl BottomSheetAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentIntent {
    OverlayBottomSheet,
}

impl BottomSheetAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OverlayBottomSheet => "overlay.bottom-sheet",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentAction {
    DismissAnyInput,
    DismissPointerOnly,
    NonDismissable,
}

impl BottomSheetAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DismissAnyInput => "dismiss-any-input",
            Self::DismissPointerOnly => "dismiss-pointer-only",
            Self::NonDismissable => "non-dismissible",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentStateAxis {
    Closed,
    OpenBasic,
    OpenWithDescription,
    OpenWithFooter,
    OpenWithDescriptionFooter,
    OpenDetached,
}

impl BottomSheetAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::OpenBasic => "open-basic",
            Self::OpenWithDescription => "open-with-description",
            Self::OpenWithFooter => "open-with-footer",
            Self::OpenWithDescriptionFooter => "open-with-description-footer",
            Self::OpenDetached => "open-detached",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentSourceAxis {
    StatePrimitivesDefaultMotion,
    StatePrimitivesCustomMotion,
}

impl BottomSheetAgentSourceAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatePrimitivesDefaultMotion => "state-primitives/default-motion",
            Self::StatePrimitivesCustomMotion => "state-primitives/custom-motion",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentOutputStatus {
    Verified,
}

impl BottomSheetAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentStreamSupport {
    Optional,
}

impl BottomSheetAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentStreamMode {
    Streaming,
    Snapshot,
}

impl BottomSheetAgentStreamMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::Snapshot => "snapshot",
        }
    }
}
const _: [BottomSheetAgentStreamMode; 2] = [
    BottomSheetAgentStreamMode::Streaming,
    BottomSheetAgentStreamMode::Snapshot,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentStreamFallback {
    Snapshot,
}

impl BottomSheetAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetAgentRenderPolicy {
    TypedOnly,
}

impl BottomSheetAgentRenderPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TypedOnly => "typed-only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BottomSheetAgentContract {
    pub schema_name: &'static str,
    pub schema_version: BottomSheetAgentSchemaVersion,
    pub intent: BottomSheetAgentIntent,
    pub action: BottomSheetAgentAction,
    pub state: BottomSheetAgentStateAxis,
    pub source: BottomSheetAgentSourceAxis,
    pub output_status: BottomSheetAgentOutputStatus,
    pub stream_support: BottomSheetAgentStreamSupport,
    pub stream_mode: BottomSheetAgentStreamMode,
    pub stream_fallback: BottomSheetAgentStreamFallback,
    pub render_policy: BottomSheetAgentRenderPolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BottomSheetAgentContractInput {
    pub is_open: bool,
    pub show_description: bool,
    pub show_footer: bool,
    pub detached: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub motion_source_attr: &'static str,
}

fn resolve_agent_action(input: BottomSheetAgentContractInput) -> BottomSheetAgentAction {
    if !input.is_dismissable {
        BottomSheetAgentAction::NonDismissable
    } else if input.is_keyboard_dismiss_disabled {
        BottomSheetAgentAction::DismissPointerOnly
    } else {
        BottomSheetAgentAction::DismissAnyInput
    }
}

fn resolve_agent_state(input: BottomSheetAgentContractInput) -> BottomSheetAgentStateAxis {
    if !input.is_open {
        BottomSheetAgentStateAxis::Closed
    } else if input.detached {
        BottomSheetAgentStateAxis::OpenDetached
    } else if input.show_description && input.show_footer {
        BottomSheetAgentStateAxis::OpenWithDescriptionFooter
    } else if input.show_description {
        BottomSheetAgentStateAxis::OpenWithDescription
    } else if input.show_footer {
        BottomSheetAgentStateAxis::OpenWithFooter
    } else {
        BottomSheetAgentStateAxis::OpenBasic
    }
}

fn resolve_agent_source(input: BottomSheetAgentContractInput) -> BottomSheetAgentSourceAxis {
    if input.motion_source_attr == CUSTOM_MOTION_SOURCE_ATTR {
        BottomSheetAgentSourceAxis::StatePrimitivesCustomMotion
    } else {
        BottomSheetAgentSourceAxis::StatePrimitivesDefaultMotion
    }
}

pub fn resolve_agent_contract(input: BottomSheetAgentContractInput) -> BottomSheetAgentContract {
    BottomSheetAgentContract {
        schema_name: "ui.bottom-sheet.agent-contract",
        schema_version: BottomSheetAgentSchemaVersion::V1,
        intent: BottomSheetAgentIntent::OverlayBottomSheet,
        action: resolve_agent_action(input),
        state: resolve_agent_state(input),
        source: resolve_agent_source(input),
        output_status: BottomSheetAgentOutputStatus::Verified,
        stream_support: BottomSheetAgentStreamSupport::Optional,
        stream_mode: BottomSheetAgentStreamMode::Snapshot,
        stream_fallback: BottomSheetAgentStreamFallback::Snapshot,
        render_policy: BottomSheetAgentRenderPolicy::TypedOnly,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
