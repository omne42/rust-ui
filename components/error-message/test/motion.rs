use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn sanitize_motion_uses_theme_default_and_clamps_transition_range() {
    let tokens = default_text_field_motion_tokens();
    assert_eq!(
        ErrorMessageMotion::default().transition_ms,
        tokens.duration_ms
    );

    assert_eq!(
        sanitize_motion(ErrorMessageMotion { transition_ms: 0 }).transition_ms,
        tokens.duration_ms
    );
    assert_eq!(
        sanitize_motion(ErrorMessageMotion {
            transition_ms: 4_000
        })
        .transition_ms,
        MAX_TRANSITION_MS
    );
}

#[test]
fn source_attr_and_effective_transition_follow_contract() {
    assert_eq!(source_attr(ErrorMessageMotion::default()), "default");
    assert_eq!(
        source_attr(ErrorMessageMotion { transition_ms: 999 }),
        "custom"
    );
    assert_eq!(
        resolve_effective_transition_ms(ErrorMessageMotion::default(), true),
        MIN_TRANSITION_MS
    );
}

#[test]
fn attach_motion_exposes_css_variable() {
    let style = attach_motion(
        Some("--ui-any:1;".to_string()),
        ErrorMessageMotion::default(),
    );
    assert!(style.contains("--ui-any:1;"));
    assert!(style.contains("--ui-error-message-transition-ms:"));
    for forbidden in ["top:", "left:", "right:", "bottom:", "width:", "height:"] {
        assert!(
            !style.contains(forbidden),
            "attach_motion should not emit regular inline layout style `{forbidden}`."
        );
    }
}
