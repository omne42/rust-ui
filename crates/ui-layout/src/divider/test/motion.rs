use super::*;

#[test]
fn default_motion_disables_entry_animation() {
    let motion = DividerMotion::default();
    assert!(!motion.animate_in);
}

#[test]
fn sanitize_motion_keeps_explicit_entry_flag() {
    let motion = sanitize_motion(DividerMotion { animate_in: true });
    assert!(motion.animate_in);
}
