use crate::command::CommandMotion;
use crate::overlay::OverlayMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CommandDialogMotion {
    pub command: CommandMotion,
    pub overlay: OverlayMotion,
}

fn sanitize_command_spring(
    spring: ui_motion::spring::SpringConfig,
) -> ui_motion::spring::SpringConfig {
    let default = CommandMotion::default().spring;

    ui_motion::spring::SpringConfig {
        stiffness: if spring.stiffness.is_finite() && spring.stiffness > 0.0 {
            spring.stiffness
        } else {
            default.stiffness
        },
        damping: if spring.damping.is_finite() && spring.damping > 0.0 {
            spring.damping
        } else {
            default.damping
        },
        mass: if spring.mass.is_finite() && spring.mass > 0.0 {
            spring.mass
        } else {
            default.mass
        },
        precision: if spring.precision.is_finite() && spring.precision > 0.0 {
            spring.precision
        } else {
            default.precision
        },
    }
}

fn sanitize_command_motion(motion: CommandMotion) -> CommandMotion {
    CommandMotion {
        spring: sanitize_command_spring(motion.spring),
    }
}

pub fn sanitize_motion(motion: CommandDialogMotion) -> CommandDialogMotion {
    CommandDialogMotion {
        command: sanitize_command_motion(motion.command),
        overlay: crate::overlay::motion::sanitize_motion(motion.overlay),
    }
}

pub fn attach_motion(command: CommandMotion, overlay: OverlayMotion) -> CommandDialogMotion {
    sanitize_motion(CommandDialogMotion { command, overlay })
}

#[cfg(test)]
mod tests {
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
}
