use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = ToastMotion::default();

    let motion = sanitize_motion(ToastMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_y_px: f64::INFINITY,
        initial_scale: 0.0,
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_y_px, default.initial_y_px);
    assert_eq!(motion.initial_scale, default.initial_scale);
}

#[test]
fn supports_custom_spring_motion_contract() {
    let default = ToastMotion::default();
    let custom_spring = ui_motion::spring::SpringConfig {
        stiffness: default.spring.stiffness + 20.0,
        damping: default.spring.damping + 4.0,
        mass: default.spring.mass,
        precision: default.spring.precision * 2.0,
    };
    let custom_initial_y = default.initial_y_px + 8.0;
    let custom_initial_scale = (default.initial_scale - 0.04).clamp(0.0, 1.0);

    let motion = sanitize_motion(ToastMotion {
        spring: custom_spring,
        initial_y_px: custom_initial_y,
        initial_scale: custom_initial_scale,
    });

    assert_eq!(motion.spring.stiffness, custom_spring.stiffness);
    assert_eq!(motion.spring.damping, custom_spring.damping);
    assert_eq!(motion.spring.mass, custom_spring.mass);
    assert_eq!(motion.spring.precision, custom_spring.precision);
    assert_eq!(motion.initial_y_px, custom_initial_y);
    assert_eq!(motion.initial_scale, custom_initial_scale);
}

#[test]
fn default_motion_matches_slide_preset() {
    let motion = ToastMotion::default();
    assert_eq!(motion.spring, ui_motion::presets::spring_slide());
    assert!(motion.initial_y_px.abs() > 0.0);
    assert!(motion.initial_scale > 0.0);
    assert!(motion.initial_scale <= 1.0);
}
