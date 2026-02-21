use super::*;
use serde::de::DeserializeOwned;

fn assert_serde<T>()
where
    T: Serialize + DeserializeOwned,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<ItemComponentSchemaVersion>();
    assert_serde::<ItemComponentSpec>();
}

#[test]
fn item_agent_contract_is_typed_and_closed() {
    assert_eq!(ITEM_AGENT_SCHEMA, "ui.item.agent-contract.v1");
    assert_eq!(ItemAgentIntent::CollectionItem.as_attr(), "collection-item");
    assert_eq!(ItemAgentAction::Render.as_attr(), "render");
    assert_eq!(ItemStreamingPolicy::Optional.as_attr(), "optional");
    assert_eq!(ItemStreamingFallback::Snapshot.as_attr(), "snapshot");
    assert_eq!(ItemAgentStreamMode::Streaming.as_attr(), "streaming");
    assert_eq!(ItemAgentStreamMode::Snapshot.as_attr(), "snapshot");
    assert_eq!(ItemAgentOutputMode::Snapshot.as_attr(), "snapshot");
    assert_eq!(ItemOutputStatus::Validated.as_attr(), "validated");
}

#[test]
fn agent_data_attrs_maps_state_and_source_axes_without_stringly_logic() {
    let state = crate::logic::ItemRenderState {
        variant_attr: "outline",
        size_attr: "sm",
        variant_source_attr: "prop",
        size_source_attr: "default",
    };

    let attrs = agent_data_attrs(state);
    assert_eq!(attrs.schema, ITEM_AGENT_SCHEMA);
    assert_eq!(attrs.intent, "collection-item");
    assert_eq!(attrs.action, "render");
    assert_eq!(attrs.streaming_policy, "optional");
    assert_eq!(attrs.streaming_fallback, "snapshot");
    assert_eq!(attrs.stream_mode, "snapshot");
    assert_eq!(attrs.output_mode, "snapshot");
    assert_eq!(attrs.output_status, "validated");
    assert_eq!(attrs.state_variant, "outline");
    assert_eq!(attrs.state_size, "sm");
    assert_eq!(attrs.source_variant, "prop");
    assert_eq!(attrs.source_size, "default");
}
