use super::*;

#[test]
fn default_motion_matches_contract() {
    let motion = ColorSwatchPickerMotion::default();

    assert_eq!(motion.transition_ms, 140);
    assert_eq!(motion.focus_ring_width_px, 5);
}

#[test]
fn sanitize_motion_clamps_values() {
    let motion = sanitize_motion(ColorSwatchPickerMotion {
        transition_ms: 0,
        focus_ring_width_px: 0,
    });

    assert_eq!(motion.transition_ms, 140);
    assert_eq!(motion.focus_ring_width_px, 2);

    let motion = sanitize_motion(ColorSwatchPickerMotion {
        transition_ms: 5000,
        focus_ring_width_px: 18,
    });

    assert_eq!(motion.transition_ms, 1200);
    assert_eq!(motion.focus_ring_width_px, 12);
}

#[test]
fn compose_style_vars_exposes_css_variables() {
    let vars = compose_style_vars(ColorSwatchPickerMotion {
        transition_ms: 220,
        focus_ring_width_px: 7,
    });

    assert!(vars.contains("--ui-color-swatch-picker-transition-ms:220ms"));
    assert!(vars.contains("--ui-color-swatch-picker-focus-ring-width:7px"));
}
