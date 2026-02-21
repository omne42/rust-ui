use super::*;
use leptos::prelude::*;

#[test]
fn variant_default_labels_are_stable() {
    assert_eq!(ContextualHelpVariant::Help.default_label(), "Help");
    assert_eq!(ContextualHelpVariant::Info.default_label(), "Info");

    assert_eq!(ContextualHelpVariant::Help.as_attr(), "help");
    assert_eq!(ContextualHelpVariant::Info.as_attr(), "info");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-contextual-help ".to_string())),
        Some("docs-contextual-help".to_string())
    );
}

#[test]
fn resolve_trigger_aria_label_uses_custom_or_default() {
    assert_eq!(
        resolve_trigger_aria_label(ContextualHelpVariant::Help, None),
        ("Help".to_string(), false)
    );
    assert_eq!(
        resolve_trigger_aria_label(
            ContextualHelpVariant::Info,
            Some("  Learn more  ".to_string())
        ),
        ("Learn more".to_string(), true)
    );
    assert_eq!(
        resolve_trigger_aria_label(ContextualHelpVariant::Info, Some("  ".to_string())),
        ("Info".to_string(), false)
    );
}

#[test]
fn resolve_id_uses_custom_or_fallback() {
    assert_eq!(
        resolve_id(Some(" docs-help ".to_string()), "fallback".to_string()),
        ("docs-help".to_string(), true)
    );
    assert_eq!(
        resolve_id(Some("   ".to_string()), "fallback".to_string()),
        ("fallback".to_string(), false)
    );
}

#[test]
fn resolve_is_disabled_prefers_is_disabled_and_supports_legacy_disabled() {
    assert!(resolve_is_disabled(Some(true), Some(false)));
    assert!(!resolve_is_disabled(Some(false), Some(true)));
    assert!(resolve_is_disabled(None, Some(true)));
    assert!(!resolve_is_disabled(None, None));
}

#[test]
fn resolve_open_state_config_makes_default_open_single_source_and_explicit() {
    let (controlled_open, _set_controlled_open) = signal(true);
    let controlled_callback = Callback::new(|_: bool| {});
    let controlled = resolve_open_state_config(ContextualHelpOpenStateInput {
        open: Some(controlled_open.into()),
        default_open: Some(false),
        on_open_change: Some(controlled_callback),
    });
    assert!(controlled.has_custom_open);
    assert!(controlled.has_custom_default_open);
    assert!(controlled.has_custom_on_open_change);
    assert!(controlled.is_controlled);
    assert_eq!(controlled.default_open, None);

    let uncontrolled = resolve_open_state_config(ContextualHelpOpenStateInput {
        open: None,
        default_open: Some(true),
        on_open_change: None,
    });
    assert!(!uncontrolled.has_custom_open);
    assert!(uncontrolled.has_custom_default_open);
    assert!(!uncontrolled.has_custom_on_open_change);
    assert!(!uncontrolled.is_controlled);
    assert_eq!(uncontrolled.default_open, Some(true));
}

#[test]
fn resolve_generated_id_uses_provider_value_or_stable_fallback() {
    assert_eq!(
        resolve_generated_id(Some("ui-contextual-help-42".to_string())),
        "ui-contextual-help-42".to_string()
    );
    assert_eq!(
        resolve_generated_id(None),
        "ui-contextual-help-0".to_string()
    );
}

#[test]
fn resolve_state_tracks_flags_and_attrs() {
    let state = resolve_state(ContextualHelpStateInput {
        variant: ContextualHelpVariant::Info,
        placement: PopoverPlacement::TopEnd,
        is_disabled: true,
        has_custom_open: true,
        has_custom_default_open: true,
        has_custom_on_open_change: true,
        has_heading: false,
        has_footer: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_id: false,
        has_custom_motion: true,
        is_controlled: true,
    });

    assert_eq!(state.variant, ContextualHelpVariant::Info);
    assert_eq!(state.variant_class, "ui-contextual-help--variant-info");
    assert_eq!(state.variant_attr, "info");

    assert_eq!(state.placement, PopoverPlacement::TopEnd);
    assert_eq!(
        state.placement_class,
        "ui-contextual-help--placement-top-end"
    );
    assert_eq!(state.placement_attr, "top-end");

    assert!(state.is_disabled);
    assert_eq!(state.state_attr, "disabled");

    assert!(!state.has_heading);
    assert_eq!(state.heading_attr, "absent");

    assert!(state.has_footer);
    assert_eq!(state.footer_attr, "present");

    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.id_source_attr, "auto");
    assert_eq!(state.motion_source_attr, "custom");

    assert!(state.is_controlled);
    assert_eq!(state.open_mode_attr, "controlled");
    assert_eq!(state.open_source_attr, "custom");
    assert_eq!(state.default_open_source_attr, "provided");
    assert_eq!(state.open_change_source_attr, "provided");

    assert!(state.has_custom_class_name);
    assert!(state.has_custom_motion);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-help".to_string()),
        resolve_state(ContextualHelpStateInput {
            variant: ContextualHelpVariant::Help,
            placement: PopoverPlacement::BottomStart,
            is_disabled: false,
            has_custom_open: false,
            has_custom_default_open: false,
            has_custom_on_open_change: false,
            has_heading: true,
            has_footer: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_id: true,
            has_custom_motion: true,
            is_controlled: false,
        }),
    );

    for token in [
        "ui-contextual-help",
        "ui-contextual-help--variant-help",
        "ui-contextual-help--placement-bottom-start",
        "ui-contextual-help--enabled",
        "ui-contextual-help--with-heading",
        "ui-contextual-help--no-footer",
        "ui-contextual-help--uncontrolled",
        "ui-contextual-help--custom-class",
        "ui-contextual-help--custom-motion",
        "docs-help",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_agent_contract_is_typed_and_traceable() {
    let open_contract = resolve_agent_contract(
        ContextualHelpVariant::Info,
        ContextualHelpOpenInteractionSource::TriggerPress,
        true,
    );
    assert_eq!(open_contract.schema, CONTEXTUAL_HELP_AGENT_SCHEMA);
    assert_eq!(open_contract.intent, "info");
    assert_eq!(open_contract.action, "toggle-open");
    assert_eq!(open_contract.state, "open");
    assert_eq!(open_contract.source, "trigger-press");

    let close_contract = resolve_agent_contract(
        ContextualHelpVariant::Help,
        ContextualHelpOpenInteractionSource::DismissPress,
        false,
    );
    assert_eq!(close_contract.intent, "help");
    assert_eq!(close_contract.action, "dismiss");
    assert_eq!(close_contract.state, "closed");
    assert_eq!(close_contract.source, "dismiss-press");
}

#[test]
fn resolve_llm_output_mode_supports_streaming_and_snapshot_with_snapshot_fallback() {
    assert_eq!(
        CONTEXTUAL_HELP_LLM_OUTPUT_FALLBACK_MODE.as_attr(),
        "snapshot"
    );
    assert_eq!(resolve_llm_output_mode(false).as_attr(), "snapshot");
    assert_eq!(resolve_llm_output_mode(true).as_attr(), "streaming");
}

#[test]
fn resolve_streaming_policy_and_output_status_are_explicit_and_type_bounded() {
    let optional_policy = resolve_streaming_policy(false);
    assert_eq!(optional_policy.requirement.as_attr(), "optional");
    assert_eq!(optional_policy.fallback_mode.as_attr(), "snapshot");

    let required_policy = resolve_streaming_policy(true);
    assert_eq!(required_policy.requirement.as_attr(), "required");
    assert_eq!(required_policy.fallback_mode.as_attr(), "snapshot");

    assert_eq!(
        resolve_llm_output_status(ContextualHelpLlmOutputMode::Streaming).as_attr(),
        "draft"
    );
    assert_eq!(
        resolve_llm_output_status(ContextualHelpLlmOutputMode::Snapshot).as_attr(),
        "verified"
    );
    assert_eq!(
        ContextualHelpLlmOutputStatus::Submittable.as_attr(),
        "submittable"
    );
}
