use super::*;
use serde::de::DeserializeOwned;

fn assert_serde<T>()
where
    T: Serialize + DeserializeOwned,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<MeterComponentSchemaVersion>();
    assert_serde::<MeterComponentSpec>();
}

#[test]
fn agent_contract_attrs_are_typed_and_closed() {
    let state = crate::logic::resolve_state(crate::logic::MeterStateInput {
        variant: crate::logic::MeterVariant::Danger,
        size: crate::logic::MeterSize::Lg,
        has_custom_aria_label: true,
        has_custom_value_label: false,
        has_custom_motion: true,
        has_custom_class_name: true,
    });

    let attrs = crate::protocol::agent_data_attrs(state, crate::logic::MeterPhase::Indeterminate);

    assert_eq!(attrs.schema, crate::protocol::METER_AGENT_SCHEMA);
    assert_eq!(attrs.intent, "progress-meter");
    assert_eq!(attrs.action, "render");
    assert_eq!(attrs.stream_mode, "snapshot");
    assert_eq!(attrs.output_mode, "snapshot");
    assert_eq!(attrs.output_status, "validated");
    assert_eq!(attrs.state_phase, "indeterminate");
    assert_eq!(attrs.state_variant, "danger");
    assert_eq!(attrs.state_size, "lg");
    assert_eq!(attrs.source_label, "custom");
    assert_eq!(attrs.source_value_label, "auto");
    assert_eq!(attrs.source_motion, "custom");
    assert_eq!(attrs.source_class, "custom");
}
