pub fn sanitize_motion(motion: crate::toast::ToastMotion) -> crate::toast::ToastMotion {
    crate::toast::motion::sanitize_motion(motion)
}

#[cfg(test)]
#[path = "../../test/sonner/motion.rs"]
mod tests;
