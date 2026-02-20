use super::*;

#[test]
fn default_motion_matches_contract() {
    let motion = CheckboxFieldMotion::default();

    assert_eq!(motion.transition_ms, 160);
    assert_eq!(motion.indicator_scale_pct, 100);
}

#[test]
fn sanitize_motion_clamps_invalid_values() {
    let motion = sanitize_motion(CheckboxFieldMotion {
        transition_ms: 0,
        indicator_scale_pct: 200,
    });

    assert_eq!(motion.transition_ms, 160);
    assert_eq!(motion.indicator_scale_pct, 140);

    let motion = sanitize_motion(CheckboxFieldMotion {
        transition_ms: 9000,
        indicator_scale_pct: 40,
    });
    assert_eq!(motion.transition_ms, 1200);
    assert_eq!(motion.indicator_scale_pct, 80);
}

#[test]
fn compose_style_vars_emits_css_custom_properties() {
    let style = compose_style_vars(CheckboxFieldMotion {
        transition_ms: 220,
        indicator_scale_pct: 112,
    });

    assert!(style.contains("--ui-checkbox-field-transition-ms:220ms"));
    assert!(style.contains("--ui-checkbox-field-indicator-scale:1.120"));
}
