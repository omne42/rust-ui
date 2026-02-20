use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(TreeMotion {
        enabled: true,
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        collapsed_scale: f64::NAN,
        collapsed_opacity: f64::NEG_INFINITY,
    });

    let default = TreeMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.collapsed_scale, default.collapsed_scale);
    assert_eq!(motion.collapsed_opacity, default.collapsed_opacity);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!TreeMotion::disabled().enabled);
}

#[test]
fn resolve_motion_css_vars_matches_expanded_and_collapsed_states() {
    let motion = sanitize_motion(TreeMotion {
        enabled: true,
        spring: ui_motion::presets::spring_soft(),
        collapsed_scale: 0.97,
        collapsed_opacity: 0.88,
    });

    let expanded = resolve_motion_css_vars(true, motion);
    let collapsed = resolve_motion_css_vars(false, motion);

    assert_eq!(expanded, (1.0, 1.0));
    assert_eq!(collapsed, (0.97, 0.88));
}
