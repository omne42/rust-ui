use super::*;

#[test]
fn sanitize_motion_delegates_to_thumbnail_contract() {
    let motion = sanitize_motion(AssetMotion::default());
    assert_eq!(motion, AssetMotion::default());
}
