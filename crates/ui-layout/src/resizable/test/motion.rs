use super::*;

#[test]
fn default_motion_comes_from_theme_tokens() {
    let motion = ResizableMotion::default();
    let tokens = default_text_field_motion_tokens();
    let duration_ms = u32::from(tokens.duration_ms);
    assert_eq!(motion.panel_duration_ms, duration_ms);
    assert_eq!(motion.handle_duration_ms, duration_ms);
    assert!(motion.enabled);
}

#[test]
fn sanitize_motion_clamps_durations() {
    let motion = sanitize_motion(ResizableMotion {
        enabled: true,
        panel_duration_ms: 0,
        handle_duration_ms: 99_999,
    });
    assert_eq!(motion.panel_duration_ms, 40);
    assert_eq!(motion.handle_duration_ms, 1_000);
}

#[test]
fn motion_style_vars_uses_theme_easing_and_durations() {
    let style = motion_style_vars(ResizableMotion {
        enabled: true,
        panel_duration_ms: 240,
        handle_duration_ms: 120,
    });

    assert!(style.contains("--ui-resizable-panel-duration: 240ms;"));
    assert!(style.contains("--ui-resizable-handle-duration: 120ms;"));
    assert!(style.contains("--ui-resizable-motion-easing: cubic-bezier(0.2, 0, 0, 1);"));
}
