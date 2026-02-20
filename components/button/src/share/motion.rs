use super::super::flip::FlipButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ShareButtonMotion {
    pub flip: FlipButtonMotion,
}

pub fn sanitize_motion(motion: ShareButtonMotion) -> ShareButtonMotion {
    ShareButtonMotion {
        flip: super::super::flip::motion::sanitize_motion(motion.flip),
    }
}

#[cfg(test)]
#[path = "../../test/share/motion.rs"]
mod tests;
