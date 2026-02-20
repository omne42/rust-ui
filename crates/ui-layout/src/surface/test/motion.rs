use super::*;

#[test]
fn default_motion_is_noop_contract() {
    assert_eq!(
        SurfaceMotion::default(),
        SurfaceMotion { animate_in: false }
    );
}

#[test]
fn sanitize_motion_preserves_input() {
    let motion = SurfaceMotion { animate_in: true };
    assert_eq!(sanitize_motion(motion), motion);
}
