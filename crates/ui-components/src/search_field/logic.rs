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
    pub set_value: Option<WriteSignal<String>>,
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
    set_value: Option<WriteSignal<String>>,
) -> Option<Callback<String>> {
    on_value_change
        .or_else(|| set_value.map(|set_value| Callback::new(move |next| set_value.set(next))))
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let has_default_value = input.default_value.is_some();
    let has_on_value_change = input.on_value_change.is_some();
    let has_legacy_set_value = input.set_value.is_some();
    let is_controlled = input.value.is_some();

    let markers = resolve_value_axis_state(SearchFieldValueAxisInput {
        is_controlled,
        has_default_value,
        has_on_value_change,
        has_legacy_set_value,
    });

    ValueAxisState {
        value: input.value,
        default_value: normalize_default_value(input.default_value),
        on_value_change: normalize_on_value_change_handler(input.on_value_change, input.set_value),
        is_controlled: markers.is_controlled,
        control_mode_attr: markers.control_mode_attr,
        default_value_source_attr: markers.default_value_source_attr,
        value_change_source_attr: markers.value_change_source_attr,
        has_value_change_handler: markers.has_value_change_handler,
    }
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_read_only: Option<bool>,
    pub read_only: bool,
    pub is_required: Option<Signal<bool>>,
    pub required: Option<Signal<bool>>,
    pub is_invalid: Option<Signal<bool>>,
    pub invalid: Option<Signal<bool>>,
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
        .or(input.required)
        .unwrap_or_else(|| Signal::derive(|| false));
    let is_invalid = input
        .is_invalid
        .or(input.invalid)
        .unwrap_or_else(|| Signal::derive(|| false));

    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
        is_read_only: input.is_read_only.unwrap_or(input.read_only),
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
        .unwrap_or_else(|| base_class.to_string());

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
        aria_label: DEFAULT_CLEAR_BUTTON_ARIA_LABEL.to_string(),
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
mod tests {
    use super::*;

    #[test]
    fn normalize_value_axis_prefers_on_value_change_over_set_value_alias() {
        let (preferred_value, set_preferred_value) = signal(String::new());
        let (_legacy_value, set_legacy_value) = signal(String::new());
        let on_value_change = Callback::new(move |next: String| set_preferred_value.set(next));

        let state = normalize_value_axis(ValueAxisInput {
            value: None,
            default_value: Some("seed".to_string()),
            on_value_change: Some(on_value_change),
            set_value: Some(set_legacy_value),
        });

        let callback = state
            .on_value_change
            .expect("value axis should keep normalized callback");
        callback.run("next".to_string());

        assert_eq!(state.control_mode_attr, "uncontrolled");
        assert_eq!(state.default_value_source_attr, "custom");
        assert_eq!(state.value_change_source_attr, "on_value_change");
        assert_eq!(preferred_value.get_untracked(), "next");
    }

    #[test]
    fn normalize_value_axis_supports_legacy_set_value_alias() {
        let (legacy_value, set_legacy_value) = signal(String::new());
        let state = normalize_value_axis(ValueAxisInput {
            value: None,
            default_value: None,
            on_value_change: None,
            set_value: Some(set_legacy_value),
        });

        let callback = state
            .on_value_change
            .expect("legacy set_value should map to on_value_change");
        callback.run("legacy".to_string());

        assert_eq!(legacy_value.get_untracked(), "legacy");
        assert_eq!(state.value_change_source_attr, "set_value");
    }

    #[test]
    fn normalize_accessibility_state_prefers_is_prefixed_inputs() {
        let (preferred_required, _set_preferred_required) = signal(true);
        let (legacy_required, _set_legacy_required) = signal(false);
        let (preferred_invalid, _set_preferred_invalid) = signal(true);
        let (legacy_invalid, _set_legacy_invalid) = signal(false);

        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: Some(true),
            disabled: false,
            is_read_only: Some(true),
            read_only: false,
            is_required: Some(preferred_required.into()),
            required: Some(legacy_required.into()),
            is_invalid: Some(preferred_invalid.into()),
            invalid: Some(legacy_invalid.into()),
        });

        assert!(state.is_disabled);
        assert!(state.is_read_only);
        assert!(state.is_required.get_untracked());
        assert!(state.is_invalid.get_untracked());
    }

    #[test]
    fn resolve_root_class_normalizes_optional_class_name() {
        let base = resolve_root_class(None);
        assert_eq!(base.class, "ui-search-field");
        assert!(!base.has_custom_class_name);
        assert_eq!(base.class_source_attr, "default");

        let custom = resolve_root_class(Some("  docs-search  ".to_string()));
        assert_eq!(custom.class, "ui-search-field docs-search");
        assert!(custom.has_custom_class_name);
        assert_eq!(custom.class_source_attr, "custom");
    }

    #[test]
    fn resolve_clear_button_label_prefers_prop_then_i18n_then_default() {
        let prop = resolve_clear_button_label(ClearButtonLabelInput {
            aria_label: Some("  Clear search box  ".to_string()),
            i18n_clear_aria_label: Some("Effacer".to_string()),
        });
        assert_eq!(prop.aria_label, "Clear search box");
        assert_eq!(prop.source_attr, "prop");

        let i18n = resolve_clear_button_label(ClearButtonLabelInput {
            aria_label: None,
            i18n_clear_aria_label: Some("  Effacer  ".to_string()),
        });
        assert_eq!(i18n.aria_label, "Effacer");
        assert_eq!(i18n.source_attr, "i18n");

        let fallback = resolve_clear_button_label(ClearButtonLabelInput {
            aria_label: Some("   ".to_string()),
            i18n_clear_aria_label: None,
        });
        assert_eq!(fallback.aria_label, DEFAULT_CLEAR_BUTTON_ARIA_LABEL);
        assert_eq!(fallback.source_attr, "default");
    }

    #[test]
    fn search_field_agent_contract_exposes_closed_schema_markers() {
        let contract = search_field_agent_contract();
        assert_eq!(contract.schema_attr, "ui.search-field");
        assert_eq!(
            contract.schema_version_attr,
            SearchFieldAgentSchemaVersion::V1.as_attr()
        );
        assert_eq!(
            contract.intent_attr,
            SearchFieldAgentIntent::FormSearchInput.as_attr()
        );
        assert_eq!(
            contract.action_model_attr,
            SearchFieldAgentActionModel::InputSubmitClear.as_attr()
        );
        assert_eq!(
            contract.state_axis_attr,
            "state|value|requirement|disabled|readonly|focus-visible|empty"
        );
        assert_eq!(
            contract.source_axis_attr,
            "class|clear-label|value-axis|locale"
        );
    }
}
