use crate::contextual_help::ContextualHelpMotion;
use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CoachmarkMotion {
    pub popover: PopoverMotion,
}

pub fn sanitize_motion(motion: CoachmarkMotion) -> CoachmarkMotion {
    CoachmarkMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
    }
}

pub fn resolve_motion(motion: CoachmarkMotion) -> ContextualHelpMotion {
    let motion = sanitize_motion(motion);
    ContextualHelpMotion {
        popover: motion.popover,
    }
}
