use crate::ActiveHighlightMotion;

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
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(MenuMotion {
            highlight: ActiveHighlightMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: f64::NAN,
                    damping: -1.0,
                    mass: 0.0,
                    precision: f64::INFINITY,
                },
            },
        });

        let default = ActiveHighlightMotion::default();
        assert_eq!(motion.highlight.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.highlight.spring.damping, default.spring.damping);
        assert_eq!(motion.highlight.spring.mass, default.spring.mass);
        assert_eq!(motion.highlight.spring.precision, default.spring.precision);
    }

    #[test]
    fn attach_motion_returns_sanitized_highlight_motion() {
        let attached = attach_motion(MenuMotion {
            highlight: ActiveHighlightMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: 280.0,
                    damping: 24.0,
                    mass: 1.0,
                    precision: 0.002,
                },
            },
        });

        assert_eq!(attached.spring.stiffness, 280.0);
        assert_eq!(attached.spring.damping, 24.0);
    }
}
