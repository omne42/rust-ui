use crate::popover::PopoverMotion;
use ui_visual_primitive::active_highlight::ActiveHighlightMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AutocompleteMotion {
    pub popover: PopoverMotion,
    pub highlight: ActiveHighlightMotion,
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = ActiveHighlightMotion::default().spring;
    ui_motion::spring::sanitize_config(value, default)
}

fn sanitize_highlight(motion: ActiveHighlightMotion) -> ActiveHighlightMotion {
    ActiveHighlightMotion {
        spring: sanitize_spring(motion.spring),
    }
}

pub fn sanitize_motion(motion: AutocompleteMotion) -> AutocompleteMotion {
    AutocompleteMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
        highlight: sanitize_highlight(motion.highlight),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_default_popover_and_highlight_motion() {
        let motion = AutocompleteMotion::default();

        assert_eq!(motion.popover, PopoverMotion::default());
        assert_eq!(motion.highlight, ActiveHighlightMotion::default());
    }

    #[test]
    fn supports_custom_popover_and_highlight_motion_contracts() {
        let motion = AutocompleteMotion {
            popover: PopoverMotion {
                initial_scale: 0.95,
                offset_y_px: 10.0,
                ..PopoverMotion::default()
            },
            highlight: ActiveHighlightMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: 240.0,
                    damping: 22.0,
                    mass: 1.0,
                    precision: 0.002,
                },
            },
        };

        assert_eq!(motion.popover.initial_scale, 0.95);
        assert_eq!(motion.popover.offset_y_px, 10.0);
        assert_eq!(motion.highlight.spring.stiffness, 240.0);
        assert_eq!(motion.highlight.spring.damping, 22.0);
        assert_eq!(motion.highlight.spring.mass, 1.0);
        assert_eq!(motion.highlight.spring.precision, 0.002);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_nested_values() {
        let input = AutocompleteMotion {
            popover: PopoverMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: f64::NAN,
                    damping: -1.0,
                    mass: 0.0,
                    precision: f64::INFINITY,
                },
                initial_scale: f64::NAN,
                offset_y_px: -9999.0,
            },
            highlight: ActiveHighlightMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: f64::NAN,
                    damping: -1.0,
                    mass: 0.0,
                    precision: f64::INFINITY,
                },
            },
        };
        let motion = sanitize_motion(input);

        assert_eq!(
            motion.popover,
            crate::popover::motion::sanitize_motion(input.popover)
        );
        assert_eq!(motion.popover.initial_scale, 0.98);
        assert_eq!(motion.popover.offset_y_px, 240.0);

        let default_highlight = ActiveHighlightMotion::default();
        assert_eq!(
            motion.highlight.spring.stiffness,
            default_highlight.spring.stiffness
        );
        assert_eq!(
            motion.highlight.spring.damping,
            default_highlight.spring.damping
        );
        assert_eq!(motion.highlight.spring.mass, default_highlight.spring.mass);
        assert_eq!(
            motion.highlight.spring.precision,
            default_highlight.spring.precision
        );
    }
}
