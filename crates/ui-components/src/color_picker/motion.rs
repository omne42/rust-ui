use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ColorPickerMotion {
    pub popover: PopoverMotion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_default_popover_motion() {
        let motion = ColorPickerMotion::default();

        assert_eq!(motion.popover, PopoverMotion::default());
    }

    #[test]
    fn supports_custom_popover_motion_contract() {
        let motion = ColorPickerMotion {
            popover: PopoverMotion {
                initial_scale: 0.96,
                offset_y_px: 8.0,
                ..PopoverMotion::default()
            },
        };

        assert_eq!(motion.popover.initial_scale, 0.96);
        assert_eq!(motion.popover.offset_y_px, 8.0);
    }
}
