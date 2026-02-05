use crate::button::ButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct FileTriggerMotion {
    pub trigger: ButtonMotion,
}
