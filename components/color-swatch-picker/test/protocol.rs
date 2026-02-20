use super::*;
use serde::de::DeserializeOwned;

fn assert_serde<T>()
where
    T: Serialize + DeserializeOwned,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<Swatch PickerComponentSchemaVersion>();
    assert_serde::<Swatch PickerComponentSpec>();
}
