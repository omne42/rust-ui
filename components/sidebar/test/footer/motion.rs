use super::*;

#[test]
fn sanitize_motion_clamps_duration_values() {
    let motion = sanitize_motion(SidebarFooterMotion {
        border_ms: u16::MAX,
        opacity_ms: u16::MAX,
        reduced_ms: u16::MAX,
    });

    assert_eq!(motion.border_ms, 5_000);
    assert_eq!(motion.opacity_ms, 5_000);
    assert_eq!(motion.reduced_ms, 5_000);
}

#[test]
fn attach_motion_emits_css_variable_contract() {
    let style = SidebarFooterMotion {
        border_ms: 110,
        opacity_ms: 130,
        reduced_ms: 0,
    }
    .attach_motion();

    assert!(style.contains("--ui-sidebar-footer-motion-border-ms:110ms;"));
    assert!(style.contains("--ui-sidebar-footer-motion-opacity-ms:130ms;"));
    assert!(style.contains("--ui-sidebar-footer-motion-reduced-ms:0ms;"));
}
