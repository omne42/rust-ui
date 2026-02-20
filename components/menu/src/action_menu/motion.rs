use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ActionMenuMotion {
    pub popover: PopoverMotion,
}

pub fn sanitize_motion(motion: ActionMenuMotion) -> ActionMenuMotion {
    ActionMenuMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
    }
}

#[cfg(test)]
#[path = "../../test/action_menu/motion.rs"]
mod tests;
