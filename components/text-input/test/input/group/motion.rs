use super::*;

#[test]
fn sanitize_motion_clamps_duration_values() {
    let motion = sanitize_motion(InputGroupMotion {
        border_ms: u16::MAX,
        fill_ms: u16::MAX,
        reduced_ms: u16::MAX,
    });

    assert_eq!(motion.border_ms, 5_000);
    assert_eq!(motion.fill_ms, 5_000);
    assert_eq!(motion.reduced_ms, 5_000);
}

#[test]
fn attach_motion_emits_css_variable_contract() {
    let style = InputGroupMotion {
        border_ms: 150,
        fill_ms: 160,
        reduced_ms: 0,
    }
    .attach_motion();

    assert!(style.contains("--ui-input-group-motion-border-ms:150ms;"));
    assert!(style.contains("--ui-input-group-motion-fill-ms:160ms;"));
    assert!(style.contains("--ui-input-group-motion-reduced-ms:0ms;"));
}
