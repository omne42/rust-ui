use super::*;

#[test]
fn sanitize_motion_preserves_default_contract() {
    let motion = sanitize_motion(FlexMotion::default());
    assert_eq!(motion, FlexMotion::default());
}
