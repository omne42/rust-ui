use std::borrow::Cow;

use crate::dialog::{DialogPartState, DialogPartStateInput, DialogSlot};
use leptos::prelude::*;
use ui_state_primitives::dialog as dialog_state;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DialogSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl DialogSize {
    pub fn class_name(self) -> &'static str {
        to_primitive_size(self).class_name()
    }

    pub fn as_attr(self) -> &'static str {
        to_primitive_size(self).as_attr()
    }
}

fn to_primitive_size(size: DialogSize) -> dialog_state::DialogSize {
    match size {
        DialogSize::Sm => dialog_state::DialogSize::Sm,
        DialogSize::Md => dialog_state::DialogSize::Md,
        DialogSize::Lg => dialog_state::DialogSize::Lg,
    }
}

pub const DEFAULT_ID_BASE: &str = dialog_state::DEFAULT_ID_BASE;
pub const DEFAULT_TITLE: &str = dialog_state::DEFAULT_TITLE;
pub const DEFAULT_CLOSE_LABEL: &str = dialog_state::DEFAULT_CLOSE_LABEL;
pub const DEFAULT_SHOW_CLOSE_BUTTON: bool = dialog_state::DEFAULT_SHOW_CLOSE_BUTTON;
pub const DEFAULT_OPEN: bool = false;
pub const DEFAULT_SIZE: DialogSize = match dialog_state::DEFAULT_SIZE {
    dialog_state::DialogSize::Sm => DialogSize::Sm,
    dialog_state::DialogSize::Md => DialogSize::Md,
    dialog_state::DialogSize::Lg => DialogSize::Lg,
};

pub type DialogOpenMode = dialog_state::DialogOpenMode;

pub struct DialogOpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct DialogOpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: bool,
    pub on_open_change: Option<Callback<bool>>,
    pub mode: DialogOpenMode,
    pub has_default_open: bool,
    pub has_open_change_handler: bool,
    pub open_prop_source_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
}

pub fn normalize_open_state(input: DialogOpenStateInput) -> DialogOpenState {
    let contract =
        dialog_state::resolve_open_state_contract(dialog_state::DialogOpenStateContractInput {
            has_is_open_prop: input.is_open.is_some(),
            has_open_prop: input.open.is_some(),
            has_default_open: input.default_open.is_some(),
            has_open_change_handler: input.on_open_change.is_some(),
        });
    let open = input.is_open.or(input.open);

    DialogOpenState {
        open,
        default_open: input.default_open.unwrap_or(DEFAULT_OPEN),
        on_open_change: input.on_open_change,
        mode: contract.mode,
        has_default_open: input.default_open.is_some(),
        has_open_change_handler: input.on_open_change.is_some(),
        open_prop_source_attr: contract.open_prop_source_attr,
        open_mode_attr: contract.open_mode_attr,
        open_source_attr: contract.open_source_attr,
        open_change_source_attr: contract.open_change_source_attr,
    }
}

pub fn can_request_close(mode: DialogOpenMode, has_open_change_handler: bool) -> bool {
    dialog_state::can_request_close(mode, has_open_change_handler)
}

pub struct DialogCloseConfigInput {
    pub is_close_button_visible: bool,
    pub show_close_button: Option<bool>,
    pub close_label: &'static str,
}

pub type DialogCloseButtonVisibility = dialog_state::DialogCloseButtonVisibility;
pub type DialogCloseButtonPropSource = dialog_state::DialogCloseButtonPropSource;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogCloseConfig {
    pub close_button_visibility: DialogCloseButtonVisibility,
    pub close_label: &'static str,
    pub has_custom_close_label: bool,
    pub close_button_prop_source: DialogCloseButtonPropSource,
}

impl DialogCloseConfig {
    #[cfg(test)]
    pub fn show_close_button(self) -> bool {
        self.close_button_visibility.is_visible()
    }

    #[cfg(test)]
    pub fn close_button_prop_source_attr(self) -> &'static str {
        self.close_button_prop_source.as_attr()
    }
}

pub fn normalize_close_config(input: DialogCloseConfigInput) -> DialogCloseConfig {
    let close_button =
        dialog_state::resolve_close_button_contract(dialog_state::DialogCloseButtonContractInput {
            is_close_button_visible: input.is_close_button_visible,
            show_close_button: input.show_close_button,
        });
    let close_label = if input.close_label.trim().is_empty() {
        DEFAULT_CLOSE_LABEL
    } else {
        input.close_label
    };

    DialogCloseConfig {
        close_button_visibility: close_button.visibility,
        close_label,
        has_custom_close_label: close_label != DEFAULT_CLOSE_LABEL,
        close_button_prop_source: close_button.prop_source,
    }
}

#[derive(Clone)]
pub struct DialogExitConfig {
    pub on_exit_complete: Callback<()>,
    pub has_custom_on_exit_complete: bool,
}

pub fn normalize_exit_config(on_exit_complete: Option<Callback<()>>) -> DialogExitConfig {
    DialogExitConfig {
        has_custom_on_exit_complete: on_exit_complete.is_some(),
        on_exit_complete: on_exit_complete.unwrap_or_else(|| Callback::new(|_| {})),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogPartStatesInput {
    pub size: DialogSize,
    pub has_description: bool,
    pub has_footer: bool,
    pub close_button_visibility: DialogCloseButtonVisibility,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_close_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogPartStates {
    pub root: DialogPartState,
    pub header: DialogPartState,
    pub title: DialogPartState,
    pub description: DialogPartState,
    pub body: DialogPartState,
    pub footer: DialogPartState,
    pub close: DialogPartState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogPartClasses {
    pub root: String,
    pub header: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub footer: String,
    pub close: String,
}

fn resolve_part_state(
    slot: DialogSlot,
    input: DialogPartStatesInput,
    has_custom_class_name: bool,
    has_custom_motion: bool,
) -> DialogPartState {
    resolve_state(DialogPartStateInput {
        slot,
        size: input.size,
        has_description: input.has_description,
        has_footer: input.has_footer,
        show_close_button: input.close_button_visibility.is_visible(),
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_close_label: input.has_custom_close_label,
        has_custom_class_name,
        has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
    })
}

pub fn resolve_part_states(input: DialogPartStatesInput) -> DialogPartStates {
    DialogPartStates {
        root: resolve_part_state(
            DialogSlot::Root,
            input,
            input.has_custom_class_name,
            input.has_custom_motion,
        ),
        header: resolve_part_state(DialogSlot::Header, input, false, false),
        title: resolve_part_state(DialogSlot::Title, input, false, false),
        description: resolve_part_state(DialogSlot::Description, input, false, false),
        body: resolve_part_state(DialogSlot::Body, input, false, false),
        footer: resolve_part_state(DialogSlot::Footer, input, false, false),
        close: resolve_part_state(DialogSlot::Close, input, false, false),
    }
}

pub fn resolve_part_classes(
    base_class_name: Option<String>,
    states: DialogPartStates,
) -> DialogPartClasses {
    DialogPartClasses {
        root: compose_class_name(base_class_name, states.root),
        header: compose_class_name(None, states.header),
        title: compose_class_name(None, states.title),
        description: compose_class_name(None, states.description),
        body: compose_class_name(None, states.body),
        footer: compose_class_name(None, states.footer),
        close: compose_class_name(None, states.close),
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    dialog_state::normalize_optional_text(value)
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    dialog_state::normalize_required_text(value, fallback)
}

pub fn normalize_id_base(value: String) -> String {
    dialog_state::normalize_id_base(value)
}

pub fn resolve_state(input: DialogPartStateInput) -> DialogPartState {
    let core = dialog_state::resolve_state_core(dialog_state::DialogStateCoreInput {
        size: to_primitive_size(input.size),
        has_description: input.has_description,
        has_footer: input.has_footer,
        show_close_button: input.show_close_button,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_close_label: input.has_custom_close_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
    });

    DialogPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        size: input.size,
        size_attr: core.size_attr,
        size_class: core.size_class,
        state_attr: core.state_attr,
        description_attr: core.description_attr,
        footer_attr: core.footer_attr,
        close_button_attr: core.close_button_attr,
        show_description: core.show_description,
        show_footer: core.show_footer,
        show_close_button: core.show_close_button,
        has_custom_size: core.has_custom_size,
        has_custom_id_base: core.has_custom_id_base,
        has_custom_title: core.has_custom_title,
        has_custom_description: core.has_custom_description,
        has_custom_close_label: core.has_custom_close_label,
        has_custom_class_name: core.has_custom_class_name,
        has_custom_motion: core.has_custom_motion,
        has_on_exit_complete: core.has_on_exit_complete,
        size_source_attr: core.size_source_attr,
        description_source_attr: core.description_source_attr,
        footer_source_attr: core.footer_source_attr,
        close_source_attr: core.close_source_attr,
        id_source_attr: core.id_source_attr,
        title_source_attr: core.title_source_attr,
        class_source_attr: core.class_source_attr,
        motion_source_attr: core.motion_source_attr,
        exit_source_attr: core.exit_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DialogPartState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if matches!(state.slot, DialogSlot::Root) {
        classes.push(Cow::Borrowed(state.size_class));

        if state.show_description {
            classes.push(Cow::Borrowed("ui-dialog--with-description"));
        } else {
            classes.push(Cow::Borrowed("ui-dialog--title-only"));
        }

        if state.show_footer {
            classes.push(Cow::Borrowed("ui-dialog--with-footer"));
        } else {
            classes.push(Cow::Borrowed("ui-dialog--footer-absent"));
        }

        if state.show_close_button {
            classes.push(Cow::Borrowed("ui-dialog--close-shown"));
        } else {
            classes.push(Cow::Borrowed("ui-dialog--close-hidden"));
        }

        if state.has_custom_size {
            classes.push(Cow::Borrowed("ui-dialog--custom-size"));
        }

        if state.has_custom_id_base {
            classes.push(Cow::Borrowed("ui-dialog--custom-id"));
        }

        if state.has_custom_title {
            classes.push(Cow::Borrowed("ui-dialog--custom-title"));
        }

        if state.has_custom_description {
            classes.push(Cow::Borrowed("ui-dialog--custom-description"));
        }

        if state.close_source_attr == "custom" {
            classes.push(Cow::Borrowed("ui-dialog--custom-close"));
        }

        if state.has_custom_motion {
            classes.push(Cow::Borrowed("ui-dialog--custom-motion"));
        }

        if state.has_on_exit_complete {
            classes.push(Cow::Borrowed("ui-dialog--custom-exit"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-dialog--custom-class"));
            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }
    } else if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(Cow::Owned(base_class_name));
    }

    classes
        .iter()
        .map(|class_name| class_name.as_ref())
        .collect::<Vec<_>>()
        .join(" ")
}

pub const DIALOG_AGENT_SCHEMA: &str = "ui.dialog.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAgentSchemaVersion {
    V1,
}

impl DialogAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAgentIntent {
    OverlayInteraction,
}

impl DialogAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OverlayInteraction => "overlay.interaction",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAgentAction {
    OpenClose,
}

impl DialogAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenClose => "open-close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAgentState {
    Open,
    Closed,
}

impl DialogAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAgentSource {
    Controlled,
    Uncontrolled,
}

impl DialogAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAgentConfigPolicy {
    Whitelist,
}

impl DialogAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAgentStreamMode {
    Streaming,
    Snapshot,
}

impl DialogAgentStreamMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::Snapshot => "snapshot",
        }
    }
}
const _: [DialogAgentStreamMode; 2] = [
    DialogAgentStreamMode::Streaming,
    DialogAgentStreamMode::Snapshot,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAgentStreamSupport {
    Required,
    Optional,
}

impl DialogAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}
const _: [DialogAgentStreamSupport; 2] = [
    DialogAgentStreamSupport::Required,
    DialogAgentStreamSupport::Optional,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAgentOutputStatus {
    Draft,
    Verified,
    CommitReady,
}

impl DialogAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::CommitReady => "commit-ready",
        }
    }
}
const _: [DialogAgentOutputStatus; 3] = [
    DialogAgentOutputStatus::Draft,
    DialogAgentOutputStatus::Verified,
    DialogAgentOutputStatus::CommitReady,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogAgentContract {
    pub schema_name: &'static str,
    pub schema_version: DialogAgentSchemaVersion,
    pub intent: DialogAgentIntent,
    pub action: DialogAgentAction,
    pub state: DialogAgentState,
    pub source: DialogAgentSource,
    pub config_policy: DialogAgentConfigPolicy,
    pub open_change_source: &'static str,
    pub stream_support: DialogAgentStreamSupport,
    pub stream_mode: DialogAgentStreamMode,
    pub stream_fallback: DialogAgentStreamMode,
    pub output_status: DialogAgentOutputStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogAgentContractInput {
    pub is_open: bool,
    pub source: DialogAgentSource,
    pub open_change_source: &'static str,
}

pub fn dialog_agent_source_from_open_mode(mode: DialogOpenMode) -> DialogAgentSource {
    match mode {
        DialogOpenMode::Controlled => DialogAgentSource::Controlled,
        DialogOpenMode::Uncontrolled => DialogAgentSource::Uncontrolled,
    }
}

pub fn resolve_agent_contract(input: DialogAgentContractInput) -> DialogAgentContract {
    DialogAgentContract {
        schema_name: DIALOG_AGENT_SCHEMA,
        schema_version: DialogAgentSchemaVersion::V1,
        intent: DialogAgentIntent::OverlayInteraction,
        action: DialogAgentAction::OpenClose,
        state: if input.is_open {
            DialogAgentState::Open
        } else {
            DialogAgentState::Closed
        },
        source: input.source,
        config_policy: DialogAgentConfigPolicy::Whitelist,
        open_change_source: input.open_change_source,
        stream_support: DialogAgentStreamSupport::Optional,
        stream_mode: DialogAgentStreamMode::Snapshot,
        stream_fallback: DialogAgentStreamMode::Snapshot,
        output_status: DialogAgentOutputStatus::Verified,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
