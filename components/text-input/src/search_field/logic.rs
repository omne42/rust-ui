use leptos::prelude::*;
use ui_state_primitives::search_field::{
    DEFAULT_CLEAR_BUTTON_ARIA_LABEL, SearchFieldSemanticState, SearchFieldSemanticStateInput,
    SearchFieldValueAxisInput, normalize_optional_text, resolve_semantic_state,
    resolve_value_axis_state, source_attr_from_presence,
};

#[derive(Clone)]
pub struct SearchFieldState {
    pub semantic: Memo<SearchFieldSemanticState>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFieldAgentSchemaVersion {
    V1,
}

impl SearchFieldAgentSchemaVersion {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFieldAgentIntent {
    FormSearchInput,
}

impl SearchFieldAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::FormSearchInput => "form-search-input",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFieldAgentActionModel {
    InputSubmitClear,
}

impl SearchFieldAgentActionModel {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::InputSubmitClear => "input|submit|clear",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchFieldAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_model_attr: &'static str,
    pub state_axis_attr: &'static str,
    pub source_axis_attr: &'static str,
}

pub fn search_field_agent_contract() -> SearchFieldAgentContract {
    SearchFieldAgentContract {
        schema_attr: "ui.search-field",
        schema_version_attr: SearchFieldAgentSchemaVersion::V1.as_attr(),
        intent_attr: SearchFieldAgentIntent::FormSearchInput.as_attr(),
        action_model_attr: SearchFieldAgentActionModel::InputSubmitClear.as_attr(),
        state_axis_attr: "state|value|requirement|disabled|readonly|focus-visible|empty",
        source_axis_attr: "class|clear-label|value-axis|locale",
    }
}

pub struct ValueAxisInput {
    pub value: Option<Signal<String>>,
    pub default_value: Option<String>,
    pub on_value_change: Option<Callback<String>>,
}

pub struct ValueAxisState {
    pub value: Option<Signal<String>>,
    pub default_value: String,
    pub on_value_change: Option<Callback<String>>,
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
    pub has_value_change_handler: bool,
}

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub fn normalize_on_value_change_handler(
    on_value_change: Option<Callback<String>>,
) -> Option<Callback<String>> {
    on_value_change
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let has_default_value = input.default_value.is_some();
    let has_on_value_change = input.on_value_change.is_some();
    let is_controlled = input.value.is_some();

    let markers = resolve_value_axis_state(SearchFieldValueAxisInput {
        is_controlled,
        has_default_value,
        has_on_value_change,
    });

    ValueAxisState {
        value: input.value,
        default_value: normalize_default_value(input.default_value),
        on_value_change: normalize_on_value_change_handler(input.on_value_change),
        is_controlled: markers.is_controlled,
        control_mode_attr: markers.control_mode_attr,
        default_value_source_attr: markers.default_value_source_attr,
        value_change_source_attr: markers.value_change_source_attr,
        has_value_change_handler: markers.has_value_change_handler,
    }
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub is_read_only: Option<bool>,
    pub is_required: Option<Signal<bool>>,
    pub is_invalid: Option<Signal<bool>>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    let is_required = input
        .is_required
        .unwrap_or_else(|| Signal::derive(|| false));
    let is_invalid = input.is_invalid.unwrap_or_else(|| Signal::derive(|| false));

    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(false),
        is_read_only: input.is_read_only.unwrap_or(false),
        is_required,
        is_invalid,
    }
}

pub struct RootClassState {
    pub class: String,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
}

pub fn resolve_root_class(class_name: Option<String>) -> RootClassState {
    let base_class = "ui-search-field";
    let custom = normalize_optional_text(class_name);
    let has_custom_class_name = custom.is_some();
    let class = custom
        .as_ref()
        .map(|custom| format!("{base_class} {custom}"))
        .unwrap_or_else(|| base_class.into());

    RootClassState {
        class,
        has_custom_class_name,
        class_source_attr: source_attr_from_presence(has_custom_class_name),
    }
}

pub struct ClearButtonLabelInput {
    pub aria_label: Option<String>,
    pub i18n_clear_aria_label: Option<String>,
}

pub struct ClearButtonLabelState {
    pub aria_label: String,
    pub source_attr: &'static str,
}

pub fn resolve_clear_button_label(input: ClearButtonLabelInput) -> ClearButtonLabelState {
    if let Some(label) = normalize_optional_text(input.aria_label) {
        return ClearButtonLabelState {
            aria_label: label,
            source_attr: "prop",
        };
    }

    if let Some(label) = normalize_optional_text(input.i18n_clear_aria_label) {
        return ClearButtonLabelState {
            aria_label: label,
            source_attr: "i18n",
        };
    }

    ClearButtonLabelState {
        aria_label: DEFAULT_CLEAR_BUTTON_ARIA_LABEL.into(),
        source_attr: "default",
    }
}

pub fn use_search_field(
    value: Signal<String>,
    is_disabled: bool,
    is_read_only: bool,
    is_invalid: Signal<bool>,
    is_required: Signal<bool>,
) -> SearchFieldState {
    let semantic = Memo::new(move |_| {
        resolve_semantic_state(SearchFieldSemanticStateInput {
            is_disabled,
            is_invalid: is_invalid.get(),
            is_read_only,
            is_required: is_required.get(),
            has_value: !value.get().is_empty(),
        })
    });

    SearchFieldState { semantic }
}

#[cfg(test)]
#[path = "../../test/search_field/logic.rs"]
mod tests;
