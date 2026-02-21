use super::*;
use serde::de::DeserializeOwned;

fn assert_serde<T>()
where
    T: Serialize + DeserializeOwned,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<FieldLabelComponentSchemaVersion>();
    assert_serde::<FieldLabelComponentSpec>();
}

#[test]
fn protocol_default_stays_v1() {
    let spec = FieldLabelComponentSpec::default();
    assert_eq!(spec.schema_version, FieldLabelComponentSchemaVersion::V1);
}
