use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn default_motion_matches_contract() {
    let motion = CheckboxFieldMotion::default();
    let tokens = default_text_field_motion_tokens();

    assert!(motion.enabled);
    assert_eq!(motion.transition_ms, tokens.duration_ms);
    assert_eq!(motion.indicator_scale_pct, 100);
}

#[test]
fn sanitize_motion_clamps_invalid_values() {
    let motion = sanitize_motion(CheckboxFieldMotion {
        enabled: true,
        transition_ms: 0,
        indicator_scale_pct: 200,
    });

    assert_eq!(
        motion.transition_ms,
        default_text_field_motion_tokens().duration_ms
    );
    assert_eq!(motion.indicator_scale_pct, 140);

    let motion = sanitize_motion(CheckboxFieldMotion {
        enabled: true,
        transition_ms: 9000,
        indicator_scale_pct: 40,
    });
    assert_eq!(motion.transition_ms, 1200);
    assert_eq!(motion.indicator_scale_pct, 80);
}

#[test]
fn source_attr_tracks_default_vs_custom_motion() {
    let default_motion = CheckboxFieldMotion::default();
    assert_eq!(source_attr(default_motion), "default");

    let custom_motion = CheckboxFieldMotion {
        enabled: false,
        ..default_motion
    };
    assert_eq!(source_attr(custom_motion), "custom");
}

#[test]
fn resolve_effective_motion_respects_reduced_motion_contract() {
    let motion = CheckboxFieldMotion {
        enabled: true,
        transition_ms: 220,
        indicator_scale_pct: 112,
    };
    let effective = resolve_effective_motion(motion, false);
    assert_eq!(effective.transition_ms, 220);
    assert_eq!(effective.indicator_scale_pct, 112);

    let reduced = resolve_effective_motion(motion, true);
    assert_eq!(reduced.transition_ms, 1);
    assert_eq!(reduced.indicator_scale_pct, 100);
}

#[test]
fn attach_motion_emits_css_custom_properties() {
    let style = attach_motion(
        Some("--ui-existing-var:1;".to_string()),
        CheckboxFieldMotion {
            enabled: true,
            transition_ms: 220,
            indicator_scale_pct: 112,
        },
    );

    assert!(style.contains("--ui-existing-var:1;"));
    assert!(style.contains("--ui-checkbox-field-transition-ms:"));
    assert!(style.contains("--ui-checkbox-field-indicator-scale:"));
}
