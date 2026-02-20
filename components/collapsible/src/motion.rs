pub fn sanitize_motion(motion: ui_disclosure::DisclosureMotion) -> ui_disclosure::DisclosureMotion {
    ui_disclosure::motion::sanitize_motion(motion)
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
