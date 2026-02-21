use super::*;
use serde::de::DeserializeOwned;

fn assert_serde<T>()
where
    T: Serialize + DeserializeOwned,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<AssetComponentSchemaVersion>();
    assert_serde::<AssetAgentIntent>();
    assert_serde::<AssetAgentAction>();
    assert_serde::<AssetInteractionSource>();
    assert_serde::<AssetMotionSource>();
    assert_serde::<AssetStreamSupport>();
    assert_serde::<AssetStreamFallback>();
    assert_serde::<AssetOutputStatus>();
    assert_serde::<AssetComponentSpec>();
}
