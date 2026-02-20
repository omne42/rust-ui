use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn default_motion_comes_from_theme_tokens() {
    let motion = HeaderMotion::default();
    let tokens = default_text_field_motion_tokens();
    assert_eq!(motion.duration_ms, f64::from(tokens.duration_ms));
}

#[test]
fn sanitize_motion_clamps_values() {
    assert_eq!(
        sanitize_motion(HeaderMotion {
            duration_ms: f64::NAN
        }),
        HeaderMotion::default()
    );
    assert_eq!(
        sanitize_motion(HeaderMotion { duration_ms: 0.0 }),
        HeaderMotion { duration_ms: 1.0 }
    );
    assert_eq!(
        sanitize_motion(HeaderMotion {
            duration_ms: 5000.0
        }),
        HeaderMotion {
            duration_ms: 1000.0
        }
    );
}

#[test]
fn source_attr_reflects_default_vs_custom() {
    assert_eq!(source_attr(HeaderMotion::default()), "default");
    assert_eq!(source_attr(HeaderMotion { duration_ms: 240.0 }), "custom");
}

#[test]
fn attach_motion_writes_css_var() {
    assert_eq!(
        attach_motion(None, HeaderMotion { duration_ms: 240.0 }),
        " --ui-header-motion-duration: 240ms;"
    );
}
