use super::*;

#[test]
fn default_motion_is_static() {
    let motion = EmptyStateMotion::default();
    assert!(!motion.animate_in);
}

#[test]
fn sanitize_motion_keeps_explicit_animation_flag() {
    let motion = sanitize_motion(EmptyStateMotion { animate_in: true });
    assert!(motion.animate_in);
}
