use super::*;

#[test]
fn sanitize_motion_preserves_default_contract() {
    let motion = sanitize_motion(ColorSwatchMotion::default());
    assert_eq!(motion, ColorSwatchMotion::default());
}
