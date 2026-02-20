#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ContextualHelpMotion {
    pub popover: crate::popover::PopoverMotion,
}

pub fn sanitize_motion(motion: ContextualHelpMotion) -> ContextualHelpMotion {
    ContextualHelpMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
    }
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
