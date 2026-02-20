use super::*;

#[test]
fn default_motion_uses_default_overlay_and_command_motion() {
    let motion = CommandDialogMotion::default();

    assert_eq!(motion.command, CommandMotion::default());
    assert_eq!(motion.overlay, OverlayMotion::default());
}

#[test]
fn sanitize_motion_falls_back_for_invalid_numbers() {
    let input = CommandDialogMotion {
        command: CommandMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        },
        overlay: OverlayMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_scale: f64::NAN,
            initial_y_px: -9999.0,
        },
    };

    let sanitized = sanitize_motion(input);

    assert_eq!(sanitized.command, CommandMotion::default());
    assert_eq!(
        sanitized.overlay,
        crate::overlay::motion::sanitize_motion(input.overlay)
    );
}

#[test]
fn attach_motion_sanitizes_command_and_overlay() {
    let motion = attach_motion(CommandMotion::default(), OverlayMotion::default());

    assert_eq!(motion.command, CommandMotion::default());
    assert_eq!(motion.overlay, OverlayMotion::default());
}
