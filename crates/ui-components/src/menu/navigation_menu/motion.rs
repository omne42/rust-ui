fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = ui_visual_primitive::active_highlight::ActiveHighlightMotion::default().spring;

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

pub fn sanitize_motion(
    motion: ui_visual_primitive::active_highlight::ActiveHighlightMotion,
) -> ui_visual_primitive::active_highlight::ActiveHighlightMotion {
    ui_visual_primitive::active_highlight::ActiveHighlightMotion {
        spring: sanitize_spring(motion.spring),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_invalid_spring_values() {
        let default = ui_visual_primitive::active_highlight::ActiveHighlightMotion::default();
        let motion = sanitize_motion(
            ui_visual_primitive::active_highlight::ActiveHighlightMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: f64::NAN,
                    damping: -1.0,
                    mass: 0.0,
                    precision: f64::INFINITY,
                },
            },
        );

        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
    }
}
