use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct MenuTriggerMotion {
    pub popover: PopoverMotion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_default_popover_motion() {
        let motion = MenuTriggerMotion::default();

        assert_eq!(motion.popover, PopoverMotion::default());
    }

    #[test]
    fn supports_custom_popover_motion_contract() {
        let motion = MenuTriggerMotion {
            popover: PopoverMotion {
                initial_scale: 0.95,
                offset_y_px: 10.0,
                ..PopoverMotion::default()
            },
        };

        assert_eq!(motion.popover.initial_scale, 0.95);
        assert_eq!(motion.popover.offset_y_px, 10.0);
    }
}
