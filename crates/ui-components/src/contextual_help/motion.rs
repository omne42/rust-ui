#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ContextualHelpMotion {
    pub popover: crate::popover::PopoverMotion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_default_popover_motion() {
        let motion = ContextualHelpMotion::default();

        assert_eq!(motion.popover, crate::popover::PopoverMotion::default());
    }

    #[test]
    fn supports_custom_popover_motion_contract() {
        let motion = ContextualHelpMotion {
            popover: crate::popover::PopoverMotion {
                initial_scale: 0.96,
                offset_y_px: 8.0,
                ..crate::popover::PopoverMotion::default()
            },
        };

        assert_eq!(motion.popover.initial_scale, 0.96);
        assert_eq!(motion.popover.offset_y_px, 8.0);
    }
}
