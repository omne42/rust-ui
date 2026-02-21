use std::borrow::Cow;

use leptos::prelude::{GetUntracked, ReadSignal, WriteSignal, signal};
use ui_checkbox::CheckboxVariant;
use ui_state_primitives::checkbox::{resolve_checked_axis, resolve_checked_change_handler_source};

pub use ui_state_primitives::checkbox::{
    CheckboxChangeHandlerSource, CheckboxCheckedAxisInput, CheckboxCheckedValueSource,
    CheckboxControlMode,
};

#[cfg(test)]
pub use ui_state_primitives::checkbox_field::resolve_status;
pub use ui_state_primitives::checkbox_field::{
    CheckboxFieldIndicatorPlacement, CheckboxFieldState, CheckboxFieldStateInput,
    CheckboxFieldStatus, CheckboxFieldTone, DEFAULT_ARIA_LABEL, DEFAULT_LABEL,
    normalize_aria_label, normalize_id_base, normalize_label, normalize_optional_text,
    resolve_state,
};

pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}

pub fn normalize_is_invalid(is_invalid: Option<bool>, invalid: bool) -> bool {
    is_invalid.unwrap_or(invalid)
}

pub struct CheckboxFieldContentInput {
    pub id_base: Option<String>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CheckboxFieldContent {
    pub id_base: String,
    pub label: String,
    pub description_text: String,
    pub aria_label: String,
    pub class_name: Option<String>,
    pub has_description: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn resolve_content(input: CheckboxFieldContentInput) -> CheckboxFieldContent {
    let id_base = normalize_id_base(input.id_base);
    let (label, has_custom_label) = normalize_label(input.label);
    let description = normalize_optional_text(input.description);
    let has_description = description.is_some();
    let description_text = description.unwrap_or_default();
    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label, &label);
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    CheckboxFieldContent {
        id_base,
        label,
        description_text,
        aria_label,
        class_name,
        has_description,
        has_custom_label,
        has_custom_aria_label,
        has_custom_class_name,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CheckedControl {
    pub checked: ReadSignal<bool>,
    pub on_checked_change: Option<WriteSignal<bool>>,
    pub mode: CheckboxControlMode,
    pub checked_prop_source_attr: &'static str,
    pub checked_change_source_attr: &'static str,
    pub checked_default_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxFieldAgentSchemaVersion {
    V1,
}

impl CheckboxFieldAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxFieldAgentIntent {
    BooleanField,
}

impl CheckboxFieldAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BooleanField => "boolean-field",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxFieldAgentAction {
    ToggleControlled,
    ToggleUncontrolled,
    ReadOnlyControlled,
}

impl CheckboxFieldAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToggleControlled => "toggle-controlled",
            Self::ToggleUncontrolled => "toggle-uncontrolled",
            Self::ReadOnlyControlled => "read-only-controlled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxFieldAgentStateAxis {
    Unchecked,
    Checked,
    Disabled,
    Invalid,
    CheckedInvalid,
}

impl CheckboxFieldAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchecked => "unchecked",
            Self::Checked => "checked",
            Self::Disabled => "disabled",
            Self::Invalid => "invalid",
            Self::CheckedInvalid => "checked-invalid",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxFieldAgentSource {
    IsCheckedProp,
    CheckedAliasProp,
    DefaultChecked,
    ImplicitDefault,
}

impl CheckboxFieldAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IsCheckedProp => "is_checked",
            Self::CheckedAliasProp => "checked",
            Self::DefaultChecked => "default_checked",
            Self::ImplicitDefault => "implicit-default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxFieldAgentOutputStatus {
    Verified,
    Submittable,
}

impl CheckboxFieldAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CheckboxFieldAgentSchemaVersion,
    pub intent: CheckboxFieldAgentIntent,
    pub action: CheckboxFieldAgentAction,
    pub state: CheckboxFieldAgentStateAxis,
    pub source: CheckboxFieldAgentSource,
    pub output_status: CheckboxFieldAgentOutputStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldAgentContractInput {
    pub status: CheckboxFieldStatus,
    pub checked_mode: CheckboxControlMode,
    pub checked_prop_source_attr: &'static str,
    pub checked_change_source_attr: &'static str,
    pub checked_default_source_attr: &'static str,
}

fn resolve_agent_state_axis(status: CheckboxFieldStatus) -> CheckboxFieldAgentStateAxis {
    match status {
        CheckboxFieldStatus::Unchecked => CheckboxFieldAgentStateAxis::Unchecked,
        CheckboxFieldStatus::Checked => CheckboxFieldAgentStateAxis::Checked,
        CheckboxFieldStatus::Disabled => CheckboxFieldAgentStateAxis::Disabled,
        CheckboxFieldStatus::Invalid => CheckboxFieldAgentStateAxis::Invalid,
        CheckboxFieldStatus::CheckedInvalid => CheckboxFieldAgentStateAxis::CheckedInvalid,
    }
}

fn resolve_agent_action(input: CheckboxFieldAgentContractInput) -> CheckboxFieldAgentAction {
    if matches!(input.checked_mode, CheckboxControlMode::Uncontrolled) {
        CheckboxFieldAgentAction::ToggleUncontrolled
    } else if input.checked_change_source_attr == "none" {
        CheckboxFieldAgentAction::ReadOnlyControlled
    } else {
        CheckboxFieldAgentAction::ToggleControlled
    }
}

fn resolve_agent_source(input: CheckboxFieldAgentContractInput) -> CheckboxFieldAgentSource {
    if matches!(input.checked_mode, CheckboxControlMode::Controlled) {
        match input.checked_prop_source_attr {
            "is_checked" => CheckboxFieldAgentSource::IsCheckedProp,
            "checked" => CheckboxFieldAgentSource::CheckedAliasProp,
            _ => CheckboxFieldAgentSource::IsCheckedProp,
        }
    } else if input.checked_default_source_attr == "default_checked" {
        CheckboxFieldAgentSource::DefaultChecked
    } else {
        CheckboxFieldAgentSource::ImplicitDefault
    }
}

fn resolve_agent_output_status(
    input: CheckboxFieldAgentContractInput,
) -> CheckboxFieldAgentOutputStatus {
    if matches!(input.checked_mode, CheckboxControlMode::Controlled)
        && input.checked_change_source_attr == "none"
    {
        CheckboxFieldAgentOutputStatus::Verified
    } else {
        CheckboxFieldAgentOutputStatus::Submittable
    }
}

pub fn resolve_agent_contract(
    input: CheckboxFieldAgentContractInput,
) -> CheckboxFieldAgentContract {
    CheckboxFieldAgentContract {
        schema_name: "ui.checkbox-field.agent-contract",
        schema_version: CheckboxFieldAgentSchemaVersion::V1,
        intent: CheckboxFieldAgentIntent::BooleanField,
        action: resolve_agent_action(input),
        state: resolve_agent_state_axis(input.status),
        source: resolve_agent_source(input),
        output_status: resolve_agent_output_status(input),
    }
}

pub struct CheckboxFieldRenderStateInput {
    pub checked: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub tone: CheckboxFieldTone,
    pub indicator_placement: CheckboxFieldIndicatorPlacement,
    pub has_description: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckboxFieldRenderState {
    pub state: CheckboxFieldState,
    pub root_class_name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldAffordance {
    pub class_name: &'static str,
    pub variant: CheckboxVariant,
}

pub fn resolve_checkbox_affordance(
    indicator_placement: CheckboxFieldIndicatorPlacement,
    invalid: bool,
) -> CheckboxFieldAffordance {
    let class_name = if matches!(indicator_placement, CheckboxFieldIndicatorPlacement::End) {
        "ui-checkbox-field__checkbox ui-checkbox-field__checkbox--indicator-end"
    } else {
        "ui-checkbox-field__checkbox"
    };

    let variant = if invalid {
        CheckboxVariant::Accent
    } else {
        CheckboxVariant::Default
    };

    CheckboxFieldAffordance {
        class_name,
        variant,
    }
}

pub fn resolve_render_state(input: CheckboxFieldRenderStateInput) -> CheckboxFieldRenderState {
    let state = resolve_state(CheckboxFieldStateInput {
        checked: input.checked,
        disabled: input.disabled,
        invalid: input.invalid,
        tone: input.tone,
        indicator_placement: input.indicator_placement,
        has_description: input.has_description,
        has_custom_label: input.has_custom_label,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    });

    let root_class_name = compose_class_name(input.class_name, state);

    CheckboxFieldRenderState {
        state,
        root_class_name,
    }
}

pub fn resolve_checked_control(
    is_checked: Option<ReadSignal<bool>>,
    checked: Option<ReadSignal<bool>>,
    on_checked_change: Option<WriteSignal<bool>>,
    set_checked: Option<WriteSignal<bool>>,
    default_checked: Option<bool>,
) -> CheckedControl {
    let checked_axis = resolve_checked_axis(CheckboxCheckedAxisInput {
        is_checked: is_checked.as_ref().map(|value| value.get_untracked()),
        checked: checked.as_ref().map(|value| value.get_untracked()),
        default_checked,
    });

    let has_primary_on_checked_change = on_checked_change.is_some();
    let has_alias_set_checked = set_checked.is_some();
    let controlled_checked = is_checked.or(checked);
    let controlled_on_checked_change = on_checked_change.or(set_checked);

    let checked_prop_source_attr = match checked_axis.source {
        CheckboxCheckedValueSource::IsChecked => "is_checked",
        CheckboxCheckedValueSource::CheckedAlias => "checked",
        CheckboxCheckedValueSource::DefaultChecked
        | CheckboxCheckedValueSource::ImplicitDefault => "none",
    };

    let checked_default_source_attr = if default_checked.is_some() {
        "default_checked"
    } else {
        "implicit-default"
    };

    if matches!(checked_axis.mode, CheckboxControlMode::Controlled) {
        let checked = controlled_checked.unwrap_or_else(|| {
            unreachable!("controlled axis requires either `is_checked` or `checked` signal")
        });
        let checked_change_source_attr = match resolve_checked_change_handler_source(
            has_primary_on_checked_change,
            has_alias_set_checked,
        ) {
            CheckboxChangeHandlerSource::OnCheckedChange => "on_checked_change",
            CheckboxChangeHandlerSource::SetCheckedAlias => "set_checked",
            CheckboxChangeHandlerSource::Missing => "none",
        };

        return CheckedControl {
            checked,
            on_checked_change: controlled_on_checked_change,
            mode: checked_axis.mode,
            checked_prop_source_attr,
            checked_change_source_attr,
            checked_default_source_attr,
        };
    }

    let (checked, set_checked) = signal(checked_axis.checked);
    CheckedControl {
        checked,
        on_checked_change: Some(set_checked),
        mode: checked_axis.mode,
        checked_prop_source_attr,
        checked_change_source_attr: "internal",
        checked_default_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CheckboxFieldState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-checkbox-field"),
        Cow::Borrowed(state.tone_class),
        Cow::Borrowed(state.indicator_placement_class),
    ];

    if state.is_checked {
        classes.push(Cow::Borrowed("ui-checkbox-field--checked"));
    } else {
        classes.push(Cow::Borrowed("ui-checkbox-field--unchecked"));
    }

    if state.is_invalid {
        classes.push(Cow::Borrowed("ui-checkbox-field--invalid"));
    }

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-checkbox-field--disabled"));
    }

    if state.has_description {
        classes.push(Cow::Borrowed("ui-checkbox-field--with-description"));
    } else {
        classes.push(Cow::Borrowed("ui-checkbox-field--no-description"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-checkbox-field--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(|class_name| class_name.as_ref())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
