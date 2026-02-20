#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SidebarTriggerMotion {
    pub duration_ms: f64,
}

impl Default for SidebarTriggerMotion {
    fn default() -> Self {
        Self { duration_ms: 160.0 }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: SidebarTriggerMotion) -> SidebarTriggerMotion {
    let default = SidebarTriggerMotion::default();

    SidebarTriggerMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
    }
}

pub fn attach_motion(motion: SidebarTriggerMotion) -> String {
    let motion = sanitize_motion(motion);
    format!(
        "--ui-sidebar-trigger-motion-duration: {}ms;",
        motion.duration_ms
    )
}

#[cfg(test)]
#[path = "../../test/trigger/motion.rs"]
mod tests;
