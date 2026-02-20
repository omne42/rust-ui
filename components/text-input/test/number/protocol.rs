use super::*;
use serde::de::DeserializeOwned;

fn assert_serde<T>()
where
    T: Serialize + DeserializeOwned,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<NumberComponentSchemaVersion>();
    assert_serde::<NumberComponentSpec>();
}
