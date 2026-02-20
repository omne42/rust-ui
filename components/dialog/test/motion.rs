use super::*;

#[test]
fn default_motion_uses_default_overlay_motion_contract() {
    let motion = DialogMotion::default();

    assert_eq!(motion.overlay, crate::overlay::OverlayMotion::default());
}

#[test]
fn supports_custom_overlay_motion_contract() {
    let motion = DialogMotion {
        overlay: crate::overlay::OverlayMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 225.0,
                damping: 21.0,
                mass: 1.0,
                precision: 0.002,
            },
            initial_scale: 0.95,
            initial_y_px: 12.0,
        },
    };

    assert_eq!(motion.overlay.spring.stiffness, 225.0);
    assert_eq!(motion.overlay.spring.damping, 21.0);
    assert_eq!(motion.overlay.spring.mass, 1.0);
    assert_eq!(motion.overlay.spring.precision, 0.002);
    assert_eq!(motion.overlay.initial_scale, 0.95);
    assert_eq!(motion.overlay.initial_y_px, 12.0);
}

#[test]
fn sanitize_motion_delegates_to_overlay_contract() {
    let input = crate::overlay::OverlayMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_scale: f64::NAN,
        initial_y_px: -9999.0,
    };
    let motion = sanitize_motion(DialogMotion { overlay: input });
    let expected = crate::overlay::motion::sanitize_motion(input);

    assert_eq!(motion.overlay, expected);
    assert_eq!(motion.overlay.initial_scale, 0.96);
    assert_eq!(motion.overlay.initial_y_px, 320.0);
}
