use std::borrow::Cow;

use ui_state_primitives::color_picker as primitive;

pub use primitive::{ColorPickerState, ColorPickerStateInput};

pub const DEFAULT_LABEL: &str = primitive::DEFAULT_LABEL;
pub const DEFAULT_ARIA_LABEL: &str = primitive::DEFAULT_ARIA_LABEL;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorPickerDerivedStateInput {
    pub is_disabled: bool,
    pub is_open: bool,
    pub selected_color: Option<String>,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub is_open_controlled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorPickerIds {
    pub root_id: String,
    pub trigger_id: String,
    pub label_id: String,
    pub panel_id: String,
    pub content_id: String,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    primitive::normalize_optional_text(value)
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    let normalized = normalize_optional_text(value);
    let has_custom_label = normalized.is_some();
    let label: Cow<'static, str> = normalized
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(DEFAULT_LABEL));

    (label.into_owned(), has_custom_label)
}

pub fn normalize_aria_label(value: Option<String>, label: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    let label = label.trim();
    if !label.is_empty() {
        return (format!("{label} picker"), false);
    }

    let fallback: Cow<'static, str> = Cow::Borrowed(DEFAULT_ARIA_LABEL);
    (fallback.into_owned(), false)
}

pub fn sanitize_selected_color(value: Option<String>) -> Option<String> {
    primitive::sanitize_selected_color(value)
}

pub fn resolve_default_selected_color(
    default_value: Option<String>,
    default_selected_color: Option<String>,
) -> Option<String> {
    sanitize_selected_color(default_value.or(default_selected_color))
}

pub fn resolve_is_disabled(is_disabled: bool, disabled: Option<bool>) -> bool {
    disabled.unwrap_or(is_disabled)
}

pub fn resolve_selected_color_axis<T>(value: Option<T>, selected_color: Option<T>) -> Option<T> {
    value.or(selected_color)
}

pub fn resolve_selected_change_axis<T>(
    on_value_change: Option<T>,
    on_selected_change: Option<T>,
) -> Option<T> {
    on_value_change.or(on_selected_change)
}

pub fn resolve_state(input: ColorPickerStateInput) -> ColorPickerState {
    primitive::resolve_state(input)
}

pub fn resolve_derived_state(input: ColorPickerDerivedStateInput) -> ColorPickerState {
    resolve_state(ColorPickerStateInput {
        disabled: input.is_disabled,
        open: input.is_open,
        has_selection: input.selected_color.is_some(),
        has_custom_label: input.has_custom_label,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        is_open_controlled: input.is_open_controlled,
    })
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorPickerState) -> String {
    primitive::compose_class_name(base_class_name, state)
}

pub fn resolve_ids(id_base: &str) -> ColorPickerIds {
    ColorPickerIds {
        root_id: id_base.into(),
        trigger_id: format!("{id_base}-trigger"),
        label_id: format!("{id_base}-label"),
        panel_id: format!("{id_base}-panel"),
        content_id: format!("{id_base}-content"),
    }
}

pub const COLOR_PICKER_AGENT_SCHEMA: &str = "ui.color-picker.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerAgentSchemaVersion {
    V1,
}

impl ColorPickerAgentSchemaVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorPickerAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerAgentIntent {
    ColorSelection,
}

impl ColorPickerAgentIntent {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorPickerAgentIntent::ColorSelection => "color.selection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerAgentAction {
    SnapshotRender,
    ToggleOpen,
    ToggleClose,
}

impl ColorPickerAgentAction {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorPickerAgentAction::SnapshotRender => "snapshot-render",
            ColorPickerAgentAction::ToggleOpen => "toggle-open",
            ColorPickerAgentAction::ToggleClose => "toggle-close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerAgentState {
    Disabled,
    Open,
    Selected,
    Empty,
}

impl ColorPickerAgentState {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorPickerAgentState::Disabled => "disabled",
            ColorPickerAgentState::Open => "open",
            ColorPickerAgentState::Selected => "selected",
            ColorPickerAgentState::Empty => "empty",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerAgentSource {
    StatePrimitives,
}

impl ColorPickerAgentSource {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorPickerAgentSource::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerAgentOutputStatus {
    Verified,
    Submittable,
}

impl ColorPickerAgentOutputStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorPickerAgentOutputStatus::Verified => "verified",
            ColorPickerAgentOutputStatus::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerAgentStreamSupport {
    Unsupported,
}

impl ColorPickerAgentStreamSupport {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorPickerAgentStreamSupport::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerAgentStreamFallback {
    Snapshot,
}

impl ColorPickerAgentStreamFallback {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorPickerAgentStreamFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerAgentStreamMode {
    Snapshot,
}

impl ColorPickerAgentStreamMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorPickerAgentStreamMode::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorPickerAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ColorPickerAgentSchemaVersion,
    pub intent: ColorPickerAgentIntent,
    pub action: ColorPickerAgentAction,
    pub state: ColorPickerAgentState,
    pub source: ColorPickerAgentSource,
    pub output_status: ColorPickerAgentOutputStatus,
    pub stream_support: ColorPickerAgentStreamSupport,
    pub stream_fallback: ColorPickerAgentStreamFallback,
    pub stream_mode: ColorPickerAgentStreamMode,
    pub selection_source: &'static str,
    pub open_source: &'static str,
    pub motion_source: &'static str,
    pub label_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub config_policy: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorPickerAgentContractInput {
    pub render_state: ColorPickerState,
    pub action: ColorPickerAgentAction,
    pub is_selection_controlled: bool,
    pub is_custom_motion: bool,
}

fn resolve_agent_state(render_state: ColorPickerState) -> ColorPickerAgentState {
    if render_state.is_disabled {
        return ColorPickerAgentState::Disabled;
    }
    if render_state.is_open {
        return ColorPickerAgentState::Open;
    }
    if render_state.has_selection {
        return ColorPickerAgentState::Selected;
    }
    ColorPickerAgentState::Empty
}

fn resolve_control_source(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

fn resolve_motion_source(is_custom_motion: bool) -> &'static str {
    if is_custom_motion {
        "custom"
    } else {
        "default"
    }
}

fn resolve_output_status(action: ColorPickerAgentAction) -> ColorPickerAgentOutputStatus {
    match action {
        ColorPickerAgentAction::SnapshotRender => ColorPickerAgentOutputStatus::Verified,
        ColorPickerAgentAction::ToggleOpen | ColorPickerAgentAction::ToggleClose => {
            ColorPickerAgentOutputStatus::Submittable
        }
    }
}

pub fn resolve_agent_contract(input: ColorPickerAgentContractInput) -> ColorPickerAgentContract {
    ColorPickerAgentContract {
        schema_name: COLOR_PICKER_AGENT_SCHEMA,
        schema_version: ColorPickerAgentSchemaVersion::V1,
        intent: ColorPickerAgentIntent::ColorSelection,
        action: input.action,
        state: resolve_agent_state(input.render_state),
        source: ColorPickerAgentSource::StatePrimitives,
        output_status: resolve_output_status(input.action),
        stream_support: ColorPickerAgentStreamSupport::Unsupported,
        stream_fallback: ColorPickerAgentStreamFallback::Snapshot,
        stream_mode: ColorPickerAgentStreamMode::Snapshot,
        selection_source: resolve_control_source(input.is_selection_controlled),
        open_source: input.render_state.open_mode_attr,
        motion_source: resolve_motion_source(input.is_custom_motion),
        label_source: input.render_state.label_source_attr,
        aria_source: input.render_state.aria_source_attr,
        class_source: input.render_state.class_source_attr,
        config_policy: "whitelist",
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
