use super::*;

#[test]
fn default_motion_matches_contract() {
    let motion = LogicButtonMotion::default();

    assert_eq!(motion.transition_ms, 160);
    assert_eq!(motion.press_scale_pct, 97);
}

#[test]
fn sanitize_motion_clamps_values() {
    let motion = sanitize_motion(LogicButtonMotion {
        transition_ms: 0,
        press_scale_pct: 10,
    });

    assert_eq!(motion.transition_ms, 160);
    assert_eq!(motion.press_scale_pct, 50);

    let motion = sanitize_motion(LogicButtonMotion {
        transition_ms: 3400,
        press_scale_pct: 160,
    });

    assert_eq!(motion.transition_ms, 1200);
    assert_eq!(motion.press_scale_pct, 120);
}

#[test]
fn compose_style_vars_exposes_css_variables() {
    let vars = compose_style_vars(LogicButtonMotion {
        transition_ms: 220,
        press_scale_pct: 93,
    });

    assert!(vars.contains("--ui-logic-button-transition-ms:220ms"));
    assert!(vars.contains("--ui-logic-button-press-scale:0.930"));
}
