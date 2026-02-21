use std::borrow::Cow;

use leptos::prelude::{GetUntracked, ReadSignal, WriteSignal, signal};
use ui_state_primitives::checkbox::{
    CheckboxChangeHandlerSource, CheckboxCheckedAxisInput, CheckboxCheckedValueSource,
    resolve_checked_axis, resolve_checked_change_handler_source,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxVariant {
    #[default]
    Default,
    Accent,
}

impl CheckboxVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-checkbox--variant-default",
            Self::Accent => "ui-checkbox--variant-accent",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl CheckboxSize {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-checkbox--size-default",
            Self::Sm => "ui-checkbox--size-sm",
            Self::Lg => "ui-checkbox--size-lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxCheckedState {
    Checked,
    Unchecked,
}

impl CheckboxCheckedState {
    pub fn from_bool(is_checked: bool) -> Self {
        if is_checked {
            Self::Checked
        } else {
            Self::Unchecked
        }
    }

    pub fn is_checked(self) -> bool {
        matches!(self, Self::Checked)
    }
}

pub fn normalize_checked_signal(
    is_checked: Option<ReadSignal<bool>>,
    checked: Option<ReadSignal<bool>>,
) -> Option<ReadSignal<bool>> {
    match resolve_checked_axis(CheckboxCheckedAxisInput {
        is_checked: is_checked.map(|value| value.get_untracked()),
        checked: checked.map(|value| value.get_untracked()),
        default_checked: None,
    })
    .source
    {
        CheckboxCheckedValueSource::IsChecked => is_checked,
        CheckboxCheckedValueSource::CheckedAlias => checked,
        CheckboxCheckedValueSource::DefaultChecked
        | CheckboxCheckedValueSource::ImplicitDefault => None,
    }
}

pub fn normalize_checked_change_handler(
    on_checked_change: Option<WriteSignal<bool>>,
    set_checked: Option<WriteSignal<bool>>,
) -> Option<WriteSignal<bool>> {
    match resolve_checked_change_handler_source(on_checked_change.is_some(), set_checked.is_some())
    {
        CheckboxChangeHandlerSource::OnCheckedChange => on_checked_change,
        CheckboxChangeHandlerSource::SetCheckedAlias => set_checked,
        CheckboxChangeHandlerSource::Missing => None,
    }
}

pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        if value.trim().is_empty() {
            None
        } else if value.len() == value.trim().len() {
            Some(value)
        } else {
            let start = value.len() - value.trim_start().len();
            let end = value.trim_end().len();
            Some(value[start..end].into())
        }
    })
}

pub fn compose_class_name(
    custom_class_name: Option<String>,
    variant: CheckboxVariant,
    size: CheckboxSize,
) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-checkbox"),
        Cow::Borrowed(variant.class_name()),
        Cow::Borrowed(size.class_name()),
    ];
    if let Some(custom_class_name) = normalize_optional_text(custom_class_name) {
        classes.push(Cow::Owned(custom_class_name));
    }

    classes
        .iter()
        .map(|class_name| class_name.as_ref())
        .collect::<Vec<_>>()
        .join(" ")
}

pub type CheckedControlMode = ui_state_primitives::checkbox::CheckboxControlMode;

#[derive(Clone, Copy, Debug)]
pub struct CheckedControl {
    pub checked: ReadSignal<bool>,
    pub on_checked_change: Option<WriteSignal<bool>>,
    pub mode: CheckedControlMode,
    pub checked_source_attr: &'static str,
    pub handler_source_attr: &'static str,
}

pub fn resolve_checked_control(
    is_checked: Option<ReadSignal<bool>>,
    checked: Option<ReadSignal<bool>>,
    on_checked_change: Option<WriteSignal<bool>>,
    set_checked: Option<WriteSignal<bool>>,
    default_checked: Option<bool>,
) -> CheckedControl {
    let checked_axis = resolve_checked_axis(CheckboxCheckedAxisInput {
        is_checked: is_checked.map(|value| value.get_untracked()),
        checked: checked.map(|value| value.get_untracked()),
        default_checked,
    });
    let handler_source =
        resolve_checked_change_handler_source(on_checked_change.is_some(), set_checked.is_some());
    let on_checked_change = normalize_checked_change_handler(on_checked_change, set_checked);
    let controlled_checked = normalize_checked_signal(is_checked, checked);

    if checked_axis.mode == CheckedControlMode::Controlled {
        if let Some(checked) = controlled_checked {
            return CheckedControl {
                checked,
                on_checked_change,
                mode: checked_axis.mode,
                checked_source_attr: checked_axis.source.source_attr(),
                handler_source_attr: handler_source.source_attr(),
            };
        }

        let (checked, _set_checked) = signal(checked_axis.checked);
        return CheckedControl {
            checked,
            on_checked_change: None,
            mode: checked_axis.mode,
            checked_source_attr: checked_axis.source.source_attr(),
            handler_source_attr: CheckboxChangeHandlerSource::Missing.source_attr(),
        };
    }

    let (checked, set_checked) = signal(checked_axis.checked);
    CheckedControl {
        checked,
        on_checked_change: Some(set_checked),
        mode: checked_axis.mode,
        checked_source_attr: checked_axis.source.source_attr(),
        handler_source_attr: handler_source.source_attr(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxRenderStateInput {
    pub checked_state: CheckboxCheckedState,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
    pub control_mode: CheckedControlMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxRenderState {
    pub state: CheckboxState,
    pub state_source_attr: &'static str,
}

pub fn derive_render_state(input: CheckboxRenderStateInput) -> CheckboxRenderState {
    let state = resolve_state(CheckboxStateInput {
        is_checked: input.checked_state.is_checked(),
        is_disabled: input.is_disabled,
        is_pressed: input.is_pressed,
        is_hovered: input.is_hovered,
        is_focused: input.is_focused,
        is_focus_visible: input.is_focus_visible,
    });

    CheckboxRenderState {
        state,
        state_source_attr: input.control_mode.source_attr(),
    }
}

pub const CHECKBOX_AGENT_SCHEMA: &str = "ui.checkbox.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxAgentSchemaVersion {
    V1,
}

impl CheckboxAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxAgentIntent {
    SelectionToggle,
}

impl CheckboxAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectionToggle => "selection.toggle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxAgentAction {
    PressToggle,
}

impl CheckboxAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PressToggle => "press.toggle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxAgentState {
    Checked,
    Unchecked,
    Disabled,
}

impl CheckboxAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Checked => "checked",
            Self::Unchecked => "unchecked",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxAgentSource {
    StatePrimitives,
}

impl CheckboxAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxAgentStreamSupport {
    Optional,
}

impl CheckboxAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxAgentStreamFallback {
    Snapshot,
}

impl CheckboxAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxAgentOutputStatus {
    Verified,
}

impl CheckboxAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CheckboxAgentSchemaVersion,
    pub intent: CheckboxAgentIntent,
    pub action: CheckboxAgentAction,
    pub state: CheckboxAgentState,
    pub source: CheckboxAgentSource,
    pub state_source: &'static str,
    pub checked_source: &'static str,
    pub handler_source: &'static str,
    pub motion_source: &'static str,
    pub stream_support: CheckboxAgentStreamSupport,
    pub stream_fallback: CheckboxAgentStreamFallback,
    pub output_status: CheckboxAgentOutputStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxAgentContractInput {
    pub render_state: CheckboxRenderState,
    pub checked_source_attr: &'static str,
    pub handler_source_attr: &'static str,
    pub has_custom_motion: bool,
}

fn resolve_agent_state(render_state: CheckboxRenderState) -> CheckboxAgentState {
    if render_state.state.is_disabled {
        CheckboxAgentState::Disabled
    } else if render_state.state.is_checked {
        CheckboxAgentState::Checked
    } else {
        CheckboxAgentState::Unchecked
    }
}

pub fn resolve_agent_contract(input: CheckboxAgentContractInput) -> CheckboxAgentContract {
    CheckboxAgentContract {
        schema_name: CHECKBOX_AGENT_SCHEMA,
        schema_version: CheckboxAgentSchemaVersion::V1,
        intent: CheckboxAgentIntent::SelectionToggle,
        action: CheckboxAgentAction::PressToggle,
        state: resolve_agent_state(input.render_state),
        source: CheckboxAgentSource::StatePrimitives,
        state_source: input.render_state.state_source_attr,
        checked_source: input.checked_source_attr,
        handler_source: input.handler_source_attr,
        motion_source: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        stream_support: CheckboxAgentStreamSupport::Optional,
        stream_fallback: CheckboxAgentStreamFallback::Snapshot,
        output_status: CheckboxAgentOutputStatus::Verified,
    }
}

pub use ui_state_primitives::checkbox::{CheckboxState, CheckboxStateInput, resolve_state};

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
