use super::*;

#[test]
fn default_label_layout_is_top_start() {
    let resolved = resolve_props(None, None, None, None, None, None);
    let view = resolve_view_state(&resolved);
    assert_eq!(view.label_position, "top");
    assert_eq!(view.label_align, "start");
}

#[test]
fn attr_mapping_matches_enum_variants() {
    assert_eq!(FormLabelPosition::Left.as_attr(), "left");
    assert_eq!(FormLabelAlign::End.as_attr(), "end");
}

#[test]
fn resolve_props_applies_default_priority_in_logic() {
    let resolved = resolve_props(None, None, None, None, None, None);

    assert!(!resolved.disabled);
    assert!(!resolved.read_only);
    assert!(!resolved.required);
    assert_eq!(resolved.label_position, FormLabelPosition::Top);
    assert_eq!(resolved.label_align, FormLabelAlign::Start);
    assert_eq!(resolved.class_name, "ui-form");
}

#[test]
fn resolve_props_prefers_explicit_inputs_and_trims_class_name() {
    let resolved = resolve_props(
        Some(true),
        Some(true),
        Some(true),
        Some(FormLabelPosition::Left),
        Some(FormLabelAlign::End),
        Some(" custom-form ".to_string()),
    );

    assert!(resolved.disabled);
    assert!(resolved.read_only);
    assert!(resolved.required);
    assert_eq!(resolved.label_position, FormLabelPosition::Left);
    assert_eq!(resolved.label_align, FormLabelAlign::End);
    assert_eq!(resolved.class_name, "ui-form custom-form");
}

#[test]
fn resolve_props_falls_back_to_base_class_when_custom_class_is_blank() {
    let resolved = resolve_props(
        Some(false),
        Some(false),
        Some(false),
        Some(FormLabelPosition::Top),
        Some(FormLabelAlign::Start),
        Some("   ".to_string()),
    );

    assert_eq!(resolved.class_name, "ui-form");
}

#[test]
fn resolve_view_state_derives_render_markers_in_logic() {
    let resolved = resolve_props(
        Some(true),
        Some(false),
        Some(true),
        Some(FormLabelPosition::Left),
        Some(FormLabelAlign::End),
        None,
    );
    let view = resolve_view_state(&resolved);

    assert_eq!(view.data_disabled, Some("true"));
    assert_eq!(view.data_read_only, None);
    assert_eq!(view.data_required, Some("true"));
    assert_eq!(view.label_position, "left");
    assert_eq!(view.label_align, "end");
    assert_eq!(view.aria_disabled, Some("true"));
    assert_eq!(view.state_source, "logic.rs::resolve_view_state");
}

#[test]
fn resolve_agent_contract_attrs_is_typed_and_traceable() {
    let resolved = resolve_props(
        Some(true),
        Some(false),
        Some(true),
        Some(FormLabelPosition::Left),
        Some(FormLabelAlign::End),
        Some("custom".to_string()),
    );
    let view = resolve_view_state(&resolved);
    let agent_contract = resolve_agent_contract_attrs(&view);

    assert_eq!(agent_contract.schema, FORM_AGENT_SCHEMA);
    assert_eq!(agent_contract.schema_version, FORM_AGENT_SCHEMA_VERSION);
    assert_eq!(agent_contract.intent, "form-container");
    assert_eq!(agent_contract.action, "render");
    assert_eq!(FormAgentStreamMode::Streaming.as_attr(), "streaming");
    assert_eq!(FormAgentStreamMode::Snapshot.as_attr(), "snapshot");
    assert_eq!(agent_contract.stream_mode, "snapshot");
    assert_eq!(FormAgentStreamingPolicy::Optional.as_attr(), "optional");
    assert_eq!(FormAgentStreamingPolicy::Required.as_attr(), "required");
    assert_eq!(agent_contract.streaming_policy, "optional");
    assert_eq!(FormAgentStreamingFallback::Snapshot.as_attr(), "snapshot");
    assert_eq!(agent_contract.streaming_fallback, "snapshot");
    assert_eq!(FormAgentOutputStatus::Draft.as_attr(), "draft");
    assert_eq!(FormAgentOutputStatus::Verified.as_attr(), "verified");
    assert_eq!(FormAgentOutputStatus::Submittable.as_attr(), "submittable");
    assert_eq!(agent_contract.output_status, "verified");
    assert_eq!(agent_contract.state_disabled, "true");
    assert_eq!(agent_contract.state_read_only, "false");
    assert_eq!(agent_contract.state_required, "true");
    assert_eq!(agent_contract.source, "logic-resolved");
}
