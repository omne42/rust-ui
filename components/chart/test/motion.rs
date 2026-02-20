use super::*;

#[test]
fn sanitize_motion_preserves_contract() {
    let motion = ChartMotion::default();
    assert_eq!(sanitize_motion(motion), motion);
}
