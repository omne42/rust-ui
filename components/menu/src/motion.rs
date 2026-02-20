use ui_visual_primitive::active_highlight::ActiveHighlightMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct MenuMotion {
    pub highlight: ActiveHighlightMotion,
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = ActiveHighlightMotion::default().spring;

    ui_motion::spring::SpringConfig {
        stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {
            value.stiffness
        } else {
            default.stiffness
        },
        damping: if value.damping.is_finite() && value.damping > 0.0 {
            value.damping
        } else {
            default.damping
        },
        mass: if value.mass.is_finite() && value.mass > 0.0 {
            value.mass
        } else {
            default.mass
        },
        precision: if value.precision.is_finite() && value.precision > 0.0 {
            value.precision
        } else {
            default.precision
        },
    }
}

pub fn sanitize_motion(motion: MenuMotion) -> MenuMotion {
    MenuMotion {
        highlight: ActiveHighlightMotion {
            spring: sanitize_spring(motion.highlight.spring),
        },
    }
}

pub fn source_attr(motion: MenuMotion) -> &'static str {
    if sanitize_motion(motion) == MenuMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(motion: MenuMotion) -> ActiveHighlightMotion {
    sanitize_motion(motion).highlight
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
