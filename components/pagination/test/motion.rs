use super::*;

#[test]
fn default_motion_uses_theme_tokens() {
    let motion = PaginationMotion::default();
    let tokens = ui_theme::default_text_field_motion_tokens();

    assert_eq!(motion.duration_ms, tokens.duration_ms);
    assert_eq!(motion.easing, tokens.easing);
    assert!(motion.enabled);
}

#[test]
fn sanitize_motion_clamps_and_falls_back() {
    let motion = sanitize_motion(PaginationMotion {
        enabled: true,
        duration_ms: u16::MAX,
        easing: "   ",
    });

    assert_eq!(motion.duration_ms, 2000);
    assert_eq!(motion.easing, PaginationMotion::default().easing);
}

#[test]
fn source_attr_tracks_default_vs_custom() {
    assert_eq!(source_attr(PaginationMotion::default()), "default");
    assert_eq!(
        source_attr(PaginationMotion {
            duration_ms: PaginationMotion::default().duration_ms.saturating_add(30),
            ..PaginationMotion::default()
        }),
        "custom"
    );
}

#[test]
fn attach_motion_emits_css_vars_and_respects_disabled() {
    let enabled = attach_motion(None, PaginationMotion::default());
    assert!(enabled.contains("--ui-pagination-motion-duration: "));
    assert!(enabled.contains("--ui-pagination-motion-easing: "));

    let disabled = attach_motion(None, PaginationMotion::disabled());
    assert!(disabled.contains("--ui-pagination-motion-duration: 0ms;"));
}
