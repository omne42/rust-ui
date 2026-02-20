use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = ImageMotion::default();

    let motion = sanitize_motion(ImageMotion {
        zoom_spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        zoom_scale: f64::NAN,
    });

    assert_eq!(motion.zoom_spring.stiffness, default.zoom_spring.stiffness);
    assert_eq!(motion.zoom_spring.damping, default.zoom_spring.damping);
    assert_eq!(motion.zoom_spring.mass, default.zoom_spring.mass);
    assert_eq!(motion.zoom_spring.precision, default.zoom_spring.precision);
    assert_eq!(motion.zoom_scale, default.zoom_scale);

    let capped = sanitize_motion(ImageMotion {
        zoom_scale: 99.0,
        ..ImageMotion::default()
    });
    assert_eq!(capped.zoom_scale, 4.0);
}

#[test]
fn default_motion_has_reasonable_params() {
    let motion = ImageMotion::default();
    assert_eq!(motion.zoom_spring, ui_motion::presets::spring_soft());
    assert!(motion.zoom_scale > 1.0);
}
