use super::*;

#[test]
fn sanitize_motion_clamps_duration_values() {
    let motion = sanitize_motion(ClearButtonMotion {
        hover_ms: u16::MAX,
        press_ms: u16::MAX,
        reduced_ms: u16::MAX,
    });

    assert_eq!(motion.hover_ms, 5_000);
    assert_eq!(motion.press_ms, 5_000);
    assert_eq!(motion.reduced_ms, 5_000);
}

#[test]
fn attach_motion_emits_css_variable_contract() {
    let style = ClearButtonMotion {
        hover_ms: 140,
        press_ms: 90,
        reduced_ms: 0,
    }
    .attach_motion();

    assert!(style.contains("--ui-clear-button-motion-hover-ms:140ms;"));
    assert!(style.contains("--ui-clear-button-motion-press-ms:90ms;"));
    assert!(style.contains("--ui-clear-button-motion-reduced-ms:0ms;"));
}
