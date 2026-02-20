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
#[path = "../test/motion.rs"]
mod tests;
