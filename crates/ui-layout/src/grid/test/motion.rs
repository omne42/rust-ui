use super::*;

#[test]
fn sanitize_motion_clamps_duration_values() {
    let motion = sanitize_motion(GridMotion {
        layout_ms: u16::MAX,
        fade_ms: u16::MAX,
        reduced_ms: u16::MAX,
    });

    assert_eq!(motion.layout_ms, 5_000);
    assert_eq!(motion.fade_ms, 5_000);
    assert_eq!(motion.reduced_ms, 5_000);
}

#[test]
fn attach_motion_emits_css_variable_contract() {
    let style = GridMotion {
        layout_ms: 200,
        fade_ms: 140,
        reduced_ms: 0,
    }
    .attach_motion();

    assert!(style.contains("--ui-grid-motion-layout-ms:200ms;"));
    assert!(style.contains("--ui-grid-motion-fade-ms:140ms;"));
    assert!(style.contains("--ui-grid-motion-reduced-ms:0ms;"));
}
