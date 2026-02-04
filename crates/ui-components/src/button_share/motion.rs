use crate::button_flip::FlipButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ShareButtonMotion {
    pub flip: FlipButtonMotion,
}
