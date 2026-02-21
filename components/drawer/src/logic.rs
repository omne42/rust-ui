use std::borrow::Cow;

use leptos::prelude::*;
use ui_sheet::SheetPlacement;

pub use ui_state_primitives::drawer::{
    DEFAULT_ID_BASE, DEFAULT_OPEN, DEFAULT_SHOW_CLOSE_BUTTON, DEFAULT_TITLE, DrawerOpenConfigInput,
    DrawerOpenMode, DrawerPartState, DrawerPartStateInput, DrawerPlacement, DrawerSlot,
    DrawerVisibility, can_request_open_change, close_button_attr, description_attr, footer_attr,
    normalize_id_base, normalize_optional_text, normalize_required_text, placement_attr,
    placement_class, resolve_close_button_visibility, resolve_open_config, resolve_state,
    state_attr,
};

const _: &str = DEFAULT_ID_BASE;
const _: bool = DEFAULT_OPEN;
const _: bool = DEFAULT_SHOW_CLOSE_BUTTON;
const _: fn(bool) -> &'static str = close_button_attr;
const _: fn(bool) -> &'static str = description_attr;
const _: fn(bool) -> &'static str = footer_attr;
const _: fn(DrawerPlacement) -> &'static str = placement_attr;
const _: fn(DrawerPlacement) -> &'static str = placement_class;
const _: fn(bool) -> &'static str = state_attr;

pub const DEFAULT_PLACEMENT: DrawerPlacement = DrawerPlacement::Right;
pub const DEFAULT_CLOSE_LABEL: &str = "Close";
pub const DRAWER_AGENT_SCHEMA: &str = "ui.drawer.agent-contract";

pub struct DrawerOpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct DrawerOpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: bool,
    pub on_open_change: Option<Callback<bool>>,
    pub mode: DrawerOpenMode,
    pub has_default_open: bool,
    pub has_open_change_handler: bool,
}

pub fn normalize_open_state(input: DrawerOpenStateInput) -> DrawerOpenState {
    let open_config = resolve_open_config(DrawerOpenConfigInput {
        has_open: input.is_open.is_some(),
        default_open: input.default_open,
        has_on_open_change: input.on_open_change.is_some(),
    });

    DrawerOpenState {
        open: input.is_open,
        default_open: open_config.default_open,
        on_open_change: input.on_open_change,
        mode: open_config.mode,
        has_default_open: open_config.has_default_open,
        has_open_change_handler: open_config.has_open_change_handler,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerOpenValueSource {
    External,
    Default,
    PrimitiveDefault,
}

impl DrawerOpenValueSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::External => "external",
            Self::Default => "default",
            Self::PrimitiveDefault => "primitive-default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerOpenActionSource {
    Programmatic,
    Interaction,
}

impl DrawerOpenActionSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Programmatic => "programmatic",
            Self::Interaction => "interaction",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerAgentSchemaVersion {
    V1,
}

impl DrawerAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerAgentIntent {
    OverlayDrawer,
}

impl DrawerAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OverlayDrawer => "overlay.drawer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerAgentAction {
    Open,
    Close,
}

impl DrawerAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Close => "close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerAgentState {
    Open,
    Closed,
}

impl DrawerAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerAgentSource {
    Controlled,
    Uncontrolled,
}

impl DrawerAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerAgentConfigPolicy {
    Whitelist,
}

impl DrawerAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerAgentOutputStatus {
    Verified,
}

impl DrawerAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerAgentCapabilities {
    pub has_description: bool,
    pub has_footer: bool,
    pub can_open: bool,
    pub can_close: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerAgentContractInput {
    pub is_open: bool,
    pub open_mode: DrawerOpenMode,
    pub has_description: bool,
    pub has_footer: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerAgentContract {
    pub schema_name: &'static str,
    pub schema_version: DrawerAgentSchemaVersion,
    pub intent: DrawerAgentIntent,
    pub action: DrawerAgentAction,
    pub state: DrawerAgentState,
    pub source: DrawerAgentSource,
    pub config_policy: DrawerAgentConfigPolicy,
    pub output_status: DrawerAgentOutputStatus,
    pub capabilities: DrawerAgentCapabilities,
}

pub fn resolve_agent_contract(input: DrawerAgentContractInput) -> DrawerAgentContract {
    let action = if input.is_open {
        DrawerAgentAction::Close
    } else {
        DrawerAgentAction::Open
    };
    let state = if input.is_open {
        DrawerAgentState::Open
    } else {
        DrawerAgentState::Closed
    };
    let source = if input.open_mode == DrawerOpenMode::Controlled {
        DrawerAgentSource::Controlled
    } else {
        DrawerAgentSource::Uncontrolled
    };

    DrawerAgentContract {
        schema_name: DRAWER_AGENT_SCHEMA,
        schema_version: DrawerAgentSchemaVersion::V1,
        intent: DrawerAgentIntent::OverlayDrawer,
        action,
        state,
        source,
        config_policy: DrawerAgentConfigPolicy::Whitelist,
        output_status: DrawerAgentOutputStatus::Verified,
        capabilities: DrawerAgentCapabilities {
            has_description: input.has_description,
            has_footer: input.has_footer,
            can_open: !input.is_open,
            can_close: input.is_open,
        },
    }
}

pub fn open_state_attr(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn open_mode_attr(mode: DrawerOpenMode) -> &'static str {
    match mode {
        DrawerOpenMode::Controlled => "controlled",
        DrawerOpenMode::Uncontrolled => "uncontrolled",
    }
}

pub fn resolve_open_value_source(
    mode: DrawerOpenMode,
    has_default_open: bool,
) -> DrawerOpenValueSource {
    if mode == DrawerOpenMode::Controlled {
        DrawerOpenValueSource::External
    } else if has_default_open {
        DrawerOpenValueSource::Default
    } else {
        DrawerOpenValueSource::PrimitiveDefault
    }
}

pub struct DrawerViewConfigInput {
    pub placement: Option<DrawerPlacement>,
    pub is_close_button_visible: Option<bool>,
    pub close_label: Option<&'static str>,
    pub on_exit_complete: Option<Callback<()>>,
}

#[derive(Clone)]
pub struct DrawerViewConfig {
    pub placement: DrawerPlacement,
    pub close_button_visibility: DrawerVisibility,
    pub close_label: &'static str,
    pub on_exit_complete: Callback<()>,
    pub has_on_exit_complete: bool,
}

pub fn normalize_view_config(input: DrawerViewConfigInput) -> DrawerViewConfig {
    let close_label = input
        .close_label
        .filter(|label| !label.trim().is_empty())
        .unwrap_or(DEFAULT_CLOSE_LABEL);

    DrawerViewConfig {
        placement: input.placement.unwrap_or(DEFAULT_PLACEMENT),
        close_button_visibility: resolve_close_button_visibility(input.is_close_button_visible),
        close_label,
        has_on_exit_complete: input.on_exit_complete.is_some(),
        on_exit_complete: input
            .on_exit_complete
            .unwrap_or_else(|| Callback::new(|_| {})),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerPartStatesInput {
    pub placement: DrawerPlacement,
    pub has_description: bool,
    pub has_footer: bool,
    pub close_button_visibility: DrawerVisibility,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerPartStates {
    pub root: DrawerPartState,
    pub header: DrawerPartState,
    pub title: DrawerPartState,
    pub description: DrawerPartState,
    pub body: DrawerPartState,
    pub footer: DrawerPartState,
    pub close: DrawerPartState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawerPartClasses {
    pub root: String,
    pub header: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub footer: String,
    pub close: String,
}

fn resolve_part_state(
    slot: DrawerSlot,
    input: DrawerPartStatesInput,
    has_custom_class_name: bool,
) -> DrawerPartState {
    resolve_state(DrawerPartStateInput {
        slot,
        placement: input.placement,
        has_description: input.has_description,
        has_footer: input.has_footer,
        show_close_button: input.close_button_visibility.is_visible(),
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
    })
}

pub fn resolve_part_states(input: DrawerPartStatesInput) -> DrawerPartStates {
    DrawerPartStates {
        root: resolve_part_state(DrawerSlot::Root, input, input.has_custom_class_name),
        header: resolve_part_state(DrawerSlot::Header, input, false),
        title: resolve_part_state(DrawerSlot::Title, input, false),
        description: resolve_part_state(DrawerSlot::Description, input, false),
        body: resolve_part_state(DrawerSlot::Body, input, false),
        footer: resolve_part_state(DrawerSlot::Footer, input, false),
        close: resolve_part_state(DrawerSlot::Close, input, false),
    }
}

pub fn resolve_part_classes(
    base_class_name: Option<String>,
    states: DrawerPartStates,
) -> DrawerPartClasses {
    DrawerPartClasses {
        root: compose_class_name(base_class_name, states.root),
        header: compose_class_name(None, states.header),
        title: compose_class_name(None, states.title),
        description: compose_class_name(None, states.description),
        body: compose_class_name(None, states.body),
        footer: compose_class_name(None, states.footer),
        close: compose_class_name(None, states.close),
    }
}

pub fn to_sheet_placement(placement: DrawerPlacement) -> SheetPlacement {
    match placement {
        DrawerPlacement::Bottom => SheetPlacement::Bottom,
        DrawerPlacement::Left => SheetPlacement::Left,
        DrawerPlacement::Right => SheetPlacement::Right,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DrawerPartState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if state.slot == DrawerSlot::Root {
        classes.push(Cow::Borrowed(state.placement_class));

        if state.show_description {
            classes.push(Cow::Borrowed("ui-drawer--with-description"));
        } else {
            classes.push(Cow::Borrowed("ui-drawer--title-only"));
        }

        if state.show_footer {
            classes.push(Cow::Borrowed("ui-drawer--with-footer"));
        } else {
            classes.push(Cow::Borrowed("ui-drawer--no-footer"));
        }

        if state.show_close_button {
            classes.push(Cow::Borrowed("ui-drawer--close-shown"));
        } else {
            classes.push(Cow::Borrowed("ui-drawer--close-hidden"));
        }

        if state.placement_source_attr == "custom" {
            classes.push(Cow::Borrowed("ui-drawer--custom-placement"));
        }

        if state.has_custom_id_base {
            classes.push(Cow::Borrowed("ui-drawer--custom-id"));
        }

        if state.has_custom_title {
            classes.push(Cow::Borrowed("ui-drawer--custom-title"));
        }

        if state.has_custom_description {
            classes.push(Cow::Borrowed("ui-drawer--custom-description"));
        }

        if state.footer_source_attr == "custom" {
            classes.push(Cow::Borrowed("ui-drawer--custom-footer"));
        }

        if state.close_source_attr == "custom" {
            classes.push(Cow::Borrowed("ui-drawer--custom-close"));
        }

        if state.has_custom_motion {
            classes.push(Cow::Borrowed("ui-drawer--custom-motion"));
        }

        if state.has_on_exit_complete {
            classes.push(Cow::Borrowed("ui-drawer--custom-exit"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-drawer--custom-class"));
            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
