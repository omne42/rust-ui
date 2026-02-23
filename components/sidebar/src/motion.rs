use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMotion {
    pub duration_ms: u16,
    pub reduced_duration_ms: u16,
}

fn default_reduced_duration_ms() -> u16 {
    1
}

impl Default for SidebarMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            duration_ms: tokens.duration_ms,
            reduced_duration_ms: default_reduced_duration_ms(),
        }
    }
}

pub fn sanitize_motion(motion: SidebarMotion) -> SidebarMotion {
    SidebarMotion {
        duration_ms: motion.duration_ms.clamp(1, 5_000),
        reduced_duration_ms: motion.reduced_duration_ms.min(5_000),
    }
}

pub fn source_attr(motion: SidebarMotion) -> &'static str {
    if sanitize_motion(motion) == SidebarMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(motion: SidebarMotion) -> String {
    let motion = sanitize_motion(motion);
    let runtime_duration_ms = if ui_motion::web::prefers_reduced_motion() {
        motion.reduced_duration_ms
    } else {
        motion.duration_ms
    };

    format!(
        "--ui-sidebar-motion-duration: {}ms; --ui-sidebar-motion-reduced-duration: {}ms; --ui-sidebar-motion-runtime-duration: {}ms;",
        motion.duration_ms, motion.reduced_duration_ms, runtime_duration_ms
    )
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
