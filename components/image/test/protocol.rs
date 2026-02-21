use super::*;
use serde::de::DeserializeOwned;

fn assert_serde<T>()
where
    T: Serialize + DeserializeOwned,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<ImageComponentSchemaVersion>();
    assert_serde::<ImageComponentSpec>();
}

#[test]
fn protocol_declares_agent_contract_schema_types() {
    assert_eq!(IMAGE_AGENT_SCHEMA, "ui.image.agent-contract/v1");
    assert_eq!(ImageAgentIntent::Display.as_attr(), "display");
    assert_eq!(ImageAgentAction::InitialRender.as_attr(), "initial-render");
    assert_eq!(ImageAgentAction::ResourceEvent.as_attr(), "resource-event");
    assert_eq!(
        ImageAgentPropSource::ExternalProp.as_attr(),
        "external-prop"
    );
    assert_eq!(ImageContentSource::Primary.as_attr(), "primary");
    assert_eq!(ImageContentSource::Fallback.as_attr(), "fallback");
    assert_eq!(ImageContentSource::Empty.as_attr(), "empty");
    assert_eq!(ImageStreamSupport::Optional.as_attr(), "optional");
    assert_eq!(ImageStreamFallback::Snapshot.as_attr(), "snapshot");
    assert_eq!(ImageLlmRenderMode::Streaming.as_attr(), "streaming");
    assert_eq!(ImageLlmRenderMode::Snapshot.as_attr(), "snapshot");
    assert_eq!(ImageOutputStatus::Draft.as_attr(), "draft");
    assert_eq!(ImageOutputStatus::Verified.as_attr(), "verified");
    assert_eq!(ImageOutputStatus::Submittable.as_attr(), "submittable");
}
