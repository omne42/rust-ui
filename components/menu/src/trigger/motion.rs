use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct MenuTriggerMotion {
    pub popover: PopoverMotion,
}

pub fn sanitize_motion(motion: MenuTriggerMotion) -> MenuTriggerMotion {
    MenuTriggerMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
    }
}

#[cfg(test)]
#[path = "../../test/trigger/motion.rs"]
mod tests;
