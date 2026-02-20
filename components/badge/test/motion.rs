use super::*;

#[test]
fn sanitize_motion_clamps_duration_values() {
    let motion = sanitize_motion(BadgeMotion {
        enter_ms: u16::MAX,
        exit_ms: u16::MAX,
        reduced_ms: u16::MAX,
    });

    assert_eq!(motion.enter_ms, 5_000);
    assert_eq!(motion.exit_ms, 5_000);
    assert_eq!(motion.reduced_ms, 5_000);
}

#[test]
fn attach_motion_emits_css_variable_contract() {
    let style = BadgeMotion {
        enter_ms: 180,
        exit_ms: 120,
        reduced_ms: 0,
    }
    .attach_motion();

    assert!(style.contains("--ui-badge-motion-enter-ms:180ms;"));
    assert!(style.contains("--ui-badge-motion-exit-ms:120ms;"));
    assert!(style.contains("--ui-badge-motion-reduced-ms:0ms;"));
}
