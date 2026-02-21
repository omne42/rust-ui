use std::borrow::Cow;

use super::{FormFieldState, FormFieldStateInput};
use leptos::prelude::{Callback, Signal};
use ui_checkbox::CheckboxVariant;
use ui_state_primitives::radio::{RadioCheckedAxisInput, resolve_checked_axis};

pub const DEFAULT_LABEL: &str = "Form field";
pub const DEFAULT_ARIA_LABEL: &str = "Form field control";
pub const DEFAULT_ERROR_MESSAGE: &str = "Selection is required";
pub const DEFAULT_ID_BASE: &str = "ui-form-field";
pub const DEFAULT_SELECTED: bool = ui_state_primitives::radio::DEFAULT_CHECKED;
pub const FORM_FIELD_AGENT_SCHEMA: &str = "ui.form_field.agent-contract.v1";
pub const FORM_FIELD_AGENT_SCHEMA_VERSION: &str = "v1";

#[derive(Clone)]
pub struct FormFieldSelectedAxisInput {
    pub is_selected: Option<Signal<bool>>,
    pub default_selected: Option<bool>,
    pub on_selected_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct FormFieldSelectedAxisState {
    pub controlled_selected: Option<Signal<bool>>,
    pub default_selected: bool,
    pub on_selected_change: Option<Callback<bool>>,
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub default_selected_source_attr: &'static str,
    pub selected_change_source_attr: &'static str,
}

pub fn normalize_selected_axis(input: FormFieldSelectedAxisInput) -> FormFieldSelectedAxisState {
    let controlled_selected = input.is_selected;
    let primitive = resolve_checked_axis(RadioCheckedAxisInput {
        has_is_checked: controlled_selected.is_some(),
        has_checked: false,
        has_default_checked: input.default_selected.is_some(),
        has_on_checked_change: input.on_selected_change.is_some(),
        has_on_change: false,
    });

    let selected_change_source_attr = match primitive.checked_change_source_attr {
        "on_checked_change" => "on_selected_change",
        other => other,
    };

    FormFieldSelectedAxisState {
        controlled_selected,
        default_selected: input.default_selected.unwrap_or(DEFAULT_SELECTED),
        on_selected_change: input.on_selected_change,
        is_controlled: primitive.is_controlled,
        control_mode_attr: primitive.control_mode_attr,
        default_selected_source_attr: primitive.default_checked_source_attr,
        selected_change_source_attr,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormFieldTone {
    #[default]
    Default,
    Quiet,
}

impl FormFieldTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FormFieldTone::Default => "ui-form-field--tone-default",
            FormFieldTone::Quiet => "ui-form-field--tone-quiet",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldTone::Default => "default",
            FormFieldTone::Quiet => "quiet",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormFieldIndicatorVariant {
    #[default]
    Switch,
    Checkbox,
}

impl FormFieldIndicatorVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            FormFieldIndicatorVariant::Switch => "ui-form-field--indicator-switch",
            FormFieldIndicatorVariant::Checkbox => "ui-form-field--indicator-checkbox",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldIndicatorVariant::Switch => "switch",
            FormFieldIndicatorVariant::Checkbox => "checkbox",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormFieldIndicatorPlacement {
    Start,
    #[default]
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFieldAgentIntent {
    SelectionControl,
}

impl FormFieldAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldAgentIntent::SelectionControl => "selection-control",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFieldAgentAction {
    RenderSnapshot,
}

impl FormFieldAgentAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldAgentAction::RenderSnapshot => "render-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFieldAgentStateAxis {
    Unselected,
    Selected,
    Disabled,
    Invalid,
    SelectedDisabled,
    SelectedInvalid,
    InvalidDisabled,
}

impl FormFieldAgentStateAxis {
    pub fn from_state_attr(state_attr: &'static str) -> Self {
        match state_attr {
            "selected" => FormFieldAgentStateAxis::Selected,
            "disabled" => FormFieldAgentStateAxis::Disabled,
            "invalid" => FormFieldAgentStateAxis::Invalid,
            "selected-disabled" => FormFieldAgentStateAxis::SelectedDisabled,
            "selected-invalid" => FormFieldAgentStateAxis::SelectedInvalid,
            "invalid-disabled" => FormFieldAgentStateAxis::InvalidDisabled,
            _ => FormFieldAgentStateAxis::Unselected,
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldAgentStateAxis::Unselected => "unselected",
            FormFieldAgentStateAxis::Selected => "selected",
            FormFieldAgentStateAxis::Disabled => "disabled",
            FormFieldAgentStateAxis::Invalid => "invalid",
            FormFieldAgentStateAxis::SelectedDisabled => "selected-disabled",
            FormFieldAgentStateAxis::SelectedInvalid => "selected-invalid",
            FormFieldAgentStateAxis::InvalidDisabled => "invalid-disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFieldAgentSourceAxis {
    Controlled,
    Uncontrolled,
}

impl FormFieldAgentSourceAxis {
    pub fn from_control_mode_attr(control_mode_attr: &'static str) -> Self {
        match control_mode_attr {
            "controlled" => FormFieldAgentSourceAxis::Controlled,
            _ => FormFieldAgentSourceAxis::Uncontrolled,
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldAgentSourceAxis::Controlled => "controlled",
            FormFieldAgentSourceAxis::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFieldAgentOutputStatus {
    Verified,
}

impl FormFieldAgentOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldAgentOutputStatus::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFieldAgentStreamSupport {
    Optional,
}

impl FormFieldAgentStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldAgentStreamSupport::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFieldAgentStreamFallback {
    Snapshot,
}

impl FormFieldAgentStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldAgentStreamFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormFieldAgentContractAttrs {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
}

pub fn resolve_agent_contract_attrs(
    state: FormFieldState,
    selected_control_mode_attr: &'static str,
) -> FormFieldAgentContractAttrs {
    let state_axis = FormFieldAgentStateAxis::from_state_attr(state.state_attr);
    let source_axis = FormFieldAgentSourceAxis::from_control_mode_attr(selected_control_mode_attr);

    FormFieldAgentContractAttrs {
        schema_name: FORM_FIELD_AGENT_SCHEMA,
        schema_version: FORM_FIELD_AGENT_SCHEMA_VERSION,
        intent_attr: FormFieldAgentIntent::SelectionControl.as_attr(),
        action_attr: FormFieldAgentAction::RenderSnapshot.as_attr(),
        state_attr: state_axis.as_attr(),
        source_attr: source_axis.as_attr(),
        stream_support_attr: FormFieldAgentStreamSupport::Optional.as_attr(),
        stream_fallback_attr: FormFieldAgentStreamFallback::Snapshot.as_attr(),
        output_status_attr: FormFieldAgentOutputStatus::Verified.as_attr(),
    }
}

impl FormFieldIndicatorPlacement {
    pub fn class_name(self) -> &'static str {
        match self {
            FormFieldIndicatorPlacement::Start => "ui-form-field--placement-start",
            FormFieldIndicatorPlacement::End => "ui-form-field--placement-end",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldIndicatorPlacement::Start => "start",
            FormFieldIndicatorPlacement::End => "end",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: Option<String>) -> String {
    if let Some(id_base) = normalize_optional_text(value) {
        id_base
    } else {
        DEFAULT_ID_BASE.into()
    }
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else {
        (DEFAULT_LABEL.into(), false)
    }
}

pub fn normalize_aria_label(value: Option<String>, fallback_label: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else if !fallback_label.trim().is_empty() {
        (fallback_label.trim().into(), false)
    } else {
        (DEFAULT_ARIA_LABEL.into(), false)
    }
}

pub fn normalize_error_message(value: Option<String>, is_invalid: bool) -> (Option<String>, bool) {
    if !is_invalid {
        return (None, false);
    }

    if let Some(message) = normalize_optional_text(value) {
        return (Some(message), true);
    }

    (Some(DEFAULT_ERROR_MESSAGE.into()), false)
}

pub fn resolve_checkbox_variant(is_invalid: bool) -> CheckboxVariant {
    if is_invalid {
        CheckboxVariant::Accent
    } else {
        CheckboxVariant::Default
    }
}

pub fn compose_describedby(
    has_description: bool,
    shows_error: bool,
    description_id: String,
    error_id: String,
) -> Option<String> {
    let mut ids = Vec::new();

    if has_description {
        ids.push(description_id);
    }

    if shows_error {
        ids.push(error_id);
    }

    if ids.is_empty() {
        None
    } else {
        Some(ids.join(" "))
    }
}

pub fn resolve_state(input: FormFieldStateInput) -> FormFieldState {
    let shows_error = input.is_invalid && input.has_error_message;

    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else if input.has_custom_label {
        "label"
    } else {
        "default"
    };

    let error_source_attr = if !input.has_error_message {
        "none"
    } else if input.has_custom_error_message {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let message_kind_attr = if shows_error {
        "error"
    } else if input.has_description {
        "description"
    } else {
        "none"
    };

    let state_attr = if input.is_invalid && input.is_disabled {
        "invalid-disabled"
    } else if input.is_invalid && input.is_selected {
        "selected-invalid"
    } else if input.is_invalid {
        "invalid"
    } else if input.is_disabled && input.is_selected {
        "selected-disabled"
    } else if input.is_disabled {
        "disabled"
    } else if input.is_selected {
        "selected"
    } else {
        "unselected"
    };

    FormFieldState {
        is_selected: input.is_selected,
        is_unselected: !input.is_selected,
        is_disabled: input.is_disabled,
        is_invalid: input.is_invalid,
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        indicator_variant: input.indicator_variant,
        indicator_variant_class: input.indicator_variant.class_name(),
        indicator_variant_attr: input.indicator_variant.as_attr(),
        indicator_placement: input.indicator_placement,
        indicator_placement_class: input.indicator_placement.class_name(),
        indicator_placement_attr: input.indicator_placement.as_attr(),
        has_description: input.has_description,
        has_error_message: input.has_error_message,
        shows_error,
        message_kind_attr,
        state_attr,
        label_source_attr,
        aria_source_attr,
        error_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FormFieldState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-form-field"),
        Cow::Borrowed(state.tone_class),
        Cow::Borrowed(state.indicator_variant_class),
        Cow::Borrowed(state.indicator_placement_class),
    ];

    if state.is_selected {
        classes.push(Cow::Borrowed("ui-form-field--selected"));
    } else {
        classes.push(Cow::Borrowed("ui-form-field--unselected"));
    }

    if state.is_invalid {
        classes.push(Cow::Borrowed("ui-form-field--invalid"));
    }

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-form-field--disabled"));
    }

    if state.has_description {
        classes.push(Cow::Borrowed("ui-form-field--with-description"));
    }

    if state.has_error_message {
        classes.push(Cow::Borrowed("ui-form-field--with-error"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-form-field--custom-class"));
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
