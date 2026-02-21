use super::*;
use leptos::prelude::{GetUntracked, Set, signal};
use ui_headless::A11yDirection;

#[test]
fn resolve_checkbox_group_ids_builds_legend_id() {
    assert_eq!(
        resolve_checkbox_group_ids("prefs"),
        CheckboxGroupIds {
            legend_id: "prefs-label".to_string(),
        }
    );
}

#[test]
fn resolve_checkbox_group_content_normalizes_defaults_and_presence_flags() {
    let content = resolve_checkbox_group_content(
        "   ".to_string(),
        Some("  helper text  ".to_string()),
        Some("   ".to_string()),
    );

    assert_eq!(content.label, "Options");
    assert_eq!(content.description.as_deref(), Some("helper text"));
    assert_eq!(content.error, None);
    assert!(content.has_description);
    assert!(!content.has_error);
}

#[test]
fn resolve_checkbox_group_class_name_uses_single_logic_default_source() {
    assert_eq!(
        resolve_checkbox_group_class_name(None),
        "ui-checkbox-group".to_string()
    );
    assert_eq!(
        resolve_checkbox_group_class_name(Some("  ".to_string())),
        "ui-checkbox-group".to_string()
    );
    assert_eq!(
        resolve_checkbox_group_class_name(Some("extra".to_string())),
        "ui-checkbox-group extra".to_string()
    );
}

#[test]
fn resolve_checkbox_group_state_is_consumed_from_primitives_contract() {
    let state = resolve_checkbox_group_state(true, true, true, true, true);

    assert!(state.is_disabled);
    assert!(!state.is_enabled);
    assert!(state.is_invalid);
    assert!(!state.is_valid);
    assert!(state.is_required);
    assert!(!state.is_optional);
    assert!(state.has_description);
    assert!(state.has_error);
    assert!(state.shows_error);
    assert!(state.has_messages);
}

#[test]
fn resolve_checkbox_group_view_state_centralizes_render_markers() {
    let primitive_state = resolve_checkbox_group_state(false, true, true, true, true);
    let view_state = resolve_checkbox_group_view_state(primitive_state);

    assert!(!view_state.is_disabled);
    assert!(view_state.is_enabled);
    assert!(view_state.is_invalid);
    assert!(!view_state.is_valid);
    assert!(view_state.is_required);
    assert!(!view_state.is_optional);
    assert!(view_state.has_description);
    assert!(view_state.has_error);
    assert!(view_state.shows_error);
    assert!(view_state.has_messages);
    assert_eq!(
        view_state.state_source,
        CheckboxGroupStateSource::SemanticProps
    );
    assert_eq!(view_state.motion_phase, CheckboxGroupMotionPhase::Active);
}

#[test]
fn resolve_checkbox_group_motion_phase_is_closed_set() {
    assert_eq!(
        resolve_checkbox_group_motion_phase(true),
        CheckboxGroupMotionPhase::Active
    );
    assert_eq!(
        resolve_checkbox_group_motion_phase(false),
        CheckboxGroupMotionPhase::Inactive
    );
    assert_eq!(CheckboxGroupMotionPhase::Active.as_data_attr(), "active");
    assert_eq!(
        CheckboxGroupMotionPhase::Inactive.as_data_attr(),
        "inactive"
    );
}

#[test]
fn resolve_checkbox_group_state_source_is_closed_set() {
    assert_eq!(
        resolve_checkbox_group_state_source(),
        CheckboxGroupStateSource::SemanticProps
    );
    assert_eq!(
        CheckboxGroupStateSource::SemanticProps.as_data_attr(),
        "semantic-props"
    );
}

#[test]
fn resolve_checkbox_group_agent_contract_is_closed_set_and_traceable() {
    let contract = resolve_checkbox_group_agent_contract(CheckboxGroupAgentContractInput {
        is_disabled: true,
        is_invalid: true,
        shows_error: true,
        state_source: CheckboxGroupStateSource::SemanticProps,
    });

    assert_eq!(contract.schema_name, CHECKBOX_GROUP_AGENT_SCHEMA);
    assert_eq!(contract.schema_version, CheckboxGroupAgentSchemaVersion::V1);
    assert_eq!(contract.intent, CheckboxGroupAgentIntent::GroupSelection);
    assert_eq!(
        contract.action,
        CheckboxGroupAgentAction::RenderSemanticWithError
    );
    assert_eq!(contract.state, CheckboxGroupAgentState::DisabledInvalid);
    assert_eq!(contract.source, CheckboxGroupAgentSource::SemanticProps);
    assert_eq!(
        contract.state_source,
        CheckboxGroupStateSource::SemanticProps
    );
    assert_eq!(
        contract.config_policy,
        CheckboxGroupAgentConfigPolicy::Whitelist
    );
}

#[test]
fn resolve_checkbox_group_agent_enum_attrs_are_stable() {
    assert_eq!(CheckboxGroupAgentSchemaVersion::V1.as_data_attr(), "v1");
    assert_eq!(
        CheckboxGroupAgentIntent::GroupSelection.as_data_attr(),
        "group-selection"
    );
    assert_eq!(
        CheckboxGroupAgentAction::RenderSemantic.as_data_attr(),
        "render-semantic"
    );
    assert_eq!(
        CheckboxGroupAgentAction::RenderSemanticWithError.as_data_attr(),
        "render-semantic-with-error"
    );
    assert_eq!(
        CheckboxGroupAgentState::EnabledValid.as_data_attr(),
        "enabled-valid"
    );
    assert_eq!(
        CheckboxGroupAgentState::DisabledInvalid.as_data_attr(),
        "disabled-invalid"
    );
    assert_eq!(
        CheckboxGroupAgentSource::SemanticProps.as_data_attr(),
        "semantic-props"
    );
    assert_eq!(
        CheckboxGroupAgentConfigPolicy::Whitelist.as_data_attr(),
        "whitelist"
    );
}

#[test]
fn use_checkbox_group_bridges_headless_locale_and_state_contract() {
    let (aria_describedby, set_aria_describedby) = signal(Some("external".to_string()));
    let (is_invalid, set_invalid) = signal(false);
    let (is_required, set_required) = signal(false);

    let a11y = use_checkbox_group(CheckboxGroupOptions {
        id: "prefs".to_string(),
        is_disabled: false,
        has_description: true,
        has_error: true,
        aria_describedby: aria_describedby.into(),
        is_invalid: is_invalid.into(),
        is_required: is_required.into(),
        lang: Some("  en-US ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(a11y.attrs.fieldset.lang.as_deref(), Some("en-US"));
    assert_eq!(a11y.attrs.fieldset.dir, Some("rtl"));
    assert_eq!(
        a11y.attrs
            .fieldset
            .aria_describedby
            .get_untracked()
            .as_deref(),
        Some("prefs-description external")
    );
    assert!(!a11y.state.resolved.get_untracked().shows_error);

    set_invalid.set(true);
    set_required.set(true);
    set_aria_describedby.set(Some("ext-a ext-b".to_string()));

    assert_eq!(
        a11y.attrs
            .fieldset
            .aria_describedby
            .get_untracked()
            .as_deref(),
        Some("prefs-description prefs-error ext-a ext-b")
    );
    let state = a11y.state.resolved.get_untracked();
    assert!(state.is_invalid);
    assert!(state.is_required);
    assert!(state.shows_error);
}
