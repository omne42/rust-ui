use crate::button::ButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct FileTriggerMotion {
    pub trigger: ButtonMotion,
}

pub fn sanitize_motion(motion: FileTriggerMotion) -> FileTriggerMotion {
    FileTriggerMotion {
        trigger: crate::button::motion::sanitize_motion(motion.trigger),
    }
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
