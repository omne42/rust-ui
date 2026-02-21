use super::*;

#[test]
fn resolves_flags() {
    let state = resolve_view_state(true, Some("Hello"), Some("World"), true);
    assert!(state.show_illustration);
    assert!(state.show_title);
    assert!(state.show_description);
    assert!(state.show_actions);

    let state = resolve_view_state(false, Some(" "), None, false);
    assert!(!state.show_illustration);
    assert!(!state.show_title);
    assert!(!state.show_description);
    assert!(!state.show_actions);
}

#[test]
fn resolves_text_defaults_in_logic_only() {
    let illustration = ();
    let resolved = resolve_view_model(
        Some("Title".to_string()),
        Some("   ".to_string()),
        Some(&illustration),
        None::<&()>,
    );
    assert!(resolved.state.show_illustration);
    assert!(resolved.state.show_title);
    assert!(!resolved.state.show_description);
    assert!(!resolved.state.show_actions);
    assert_eq!(resolved.title, "Title");
    assert_eq!(resolved.description, "");
    assert_eq!(
        resolved.view_state,
        IllustratedMessageRenderMarker::Populated
    );
    assert_eq!(resolved.title_state, IllustratedMessageStateMarker::Shown);
    assert_eq!(
        resolved.description_state,
        IllustratedMessageStateMarker::Hidden
    );
    assert_eq!(
        resolved.illustration_state,
        IllustratedMessageStateMarker::Shown
    );
    assert_eq!(
        resolved.actions_state,
        IllustratedMessageStateMarker::Hidden
    );
    assert_eq!(resolved.content_state, IllustratedMessageStateMarker::Shown);
    assert_eq!(
        resolved.title_source,
        IllustratedMessageTextSource::Provided
    );
    assert_eq!(
        resolved.description_source,
        IllustratedMessageTextSource::Blank
    );
    assert_eq!(
        resolved.illustration_source,
        IllustratedMessageSlotSource::Provided
    );
    assert_eq!(
        resolved.actions_source,
        IllustratedMessageSlotSource::Missing
    );

    let resolved = resolve_view_model(None, None, None::<&()>, None::<&()>);
    assert!(!resolved.state.show_title);
    assert!(!resolved.state.show_description);
    assert!(!resolved.state.show_illustration);
    assert!(!resolved.state.show_actions);
    assert_eq!(resolved.title, "");
    assert_eq!(resolved.description, "");
    assert_eq!(resolved.view_state, IllustratedMessageRenderMarker::Empty);
    assert_eq!(resolved.title_state, IllustratedMessageStateMarker::Hidden);
    assert_eq!(
        resolved.description_state,
        IllustratedMessageStateMarker::Hidden
    );
    assert_eq!(
        resolved.illustration_state,
        IllustratedMessageStateMarker::Hidden
    );
    assert_eq!(
        resolved.actions_state,
        IllustratedMessageStateMarker::Hidden
    );
    assert_eq!(
        resolved.content_state,
        IllustratedMessageStateMarker::Hidden
    );
    assert_eq!(resolved.title_source, IllustratedMessageTextSource::Missing);
    assert_eq!(
        resolved.description_source,
        IllustratedMessageTextSource::Missing
    );
    assert_eq!(
        resolved.illustration_source,
        IllustratedMessageSlotSource::Missing
    );
    assert_eq!(
        resolved.actions_source,
        IllustratedMessageSlotSource::Missing
    );
}

#[test]
fn normalizes_root_class_name_in_logic() {
    let class = resolve_root_class(crate::IllustratedMessageOrientation::Vertical, None);
    assert_eq!(
        class,
        "ui-illustrated-message ui-illustrated-message--orientation-vertical"
    );

    let class = resolve_root_class(
        crate::IllustratedMessageOrientation::Horizontal,
        Some("  extra ".to_string()),
    );
    assert_eq!(
        class,
        "ui-illustrated-message ui-illustrated-message--orientation-horizontal extra"
    );
}

#[test]
fn resolves_agent_contract_attrs_from_typed_state_and_source_axes() {
    let default_resolved = resolve_view_model(None, None, None::<&()>, None::<&()>);
    let default_contract = resolve_agent_contract_attrs(&default_resolved);
    assert_eq!(
        default_contract.schema_attr,
        ILLUSTRATED_MESSAGE_AGENT_SCHEMA
    );
    assert_eq!(
        default_contract.schema_version_attr,
        ILLUSTRATED_MESSAGE_AGENT_SCHEMA_VERSION
    );
    assert_eq!(
        default_contract.intent_attr,
        IllustratedMessageAgentIntent::EmptyStateDisplay.as_data_attr()
    );
    assert_eq!(
        default_contract.action_attr,
        IllustratedMessageAgentAction::RenderSnapshot.as_data_attr()
    );
    assert_eq!(
        default_contract.state_attr,
        IllustratedMessageAgentState::Empty.as_data_attr()
    );
    assert_eq!(
        default_contract.source_attr,
        IllustratedMessageAgentSource::Default.as_data_attr()
    );
    assert_eq!(
        default_contract.config_policy_attr,
        IllustratedMessageAgentConfigPolicy::Whitelist.as_data_attr()
    );
    assert_eq!(
        default_contract.streaming_policy_attr,
        IllustratedMessageAgentStreamingPolicy::Optional.as_data_attr()
    );
    assert_eq!(
        default_contract.streaming_fallback_attr,
        IllustratedMessageAgentStreamingFallback::Snapshot.as_data_attr()
    );
    assert_eq!(
        default_contract.output_status_attr,
        IllustratedMessageAgentOutputStatus::Validated.as_data_attr()
    );

    let custom_resolved =
        resolve_view_model(Some("No results".to_string()), None, None::<&()>, Some(&()));
    let custom_contract = resolve_agent_contract_attrs(&custom_resolved);
    assert_eq!(
        custom_contract.state_attr,
        IllustratedMessageAgentState::Populated.as_data_attr()
    );
    assert_eq!(
        custom_contract.source_attr,
        IllustratedMessageAgentSource::Custom.as_data_attr()
    );
    assert_eq!(
        custom_contract.streaming_policy_attr,
        IllustratedMessageAgentStreamingPolicy::Optional.as_data_attr()
    );
    assert_eq!(
        custom_contract.streaming_fallback_attr,
        IllustratedMessageAgentStreamingFallback::Snapshot.as_data_attr()
    );
    assert_eq!(
        custom_contract.output_status_attr,
        IllustratedMessageAgentOutputStatus::Validated.as_data_attr()
    );
}
