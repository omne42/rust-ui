use crate::ActiveHighlightMotion;
use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ComboBoxMotion {
    pub popover: PopoverMotion,
    pub highlight: ActiveHighlightMotion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_default_popover_and_highlight_motion() {
        let motion = ComboBoxMotion::default();

        assert_eq!(motion.popover, PopoverMotion::default());
        assert_eq!(motion.highlight, ActiveHighlightMotion::default());
    }

    #[test]
    fn supports_custom_popover_and_highlight_motion_contracts() {
        let motion = ComboBoxMotion {
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
}
