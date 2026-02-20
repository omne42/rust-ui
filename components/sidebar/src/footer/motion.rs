#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarFooterMotion {
    pub border_ms: u16,
    pub opacity_ms: u16,
    pub reduced_ms: u16,
}

impl Default for SidebarFooterMotion {
    fn default() -> Self {
        Self {
            border_ms: 120,
            opacity_ms: 120,
            reduced_ms: 0,
        }
    }
}

pub fn sanitize_motion(motion: SidebarFooterMotion) -> SidebarFooterMotion {
    const MAX_MS: u16 = 5_000;

    SidebarFooterMotion {
        border_ms: motion.border_ms.min(MAX_MS),
        opacity_ms: motion.opacity_ms.min(MAX_MS),
        reduced_ms: motion.reduced_ms.min(MAX_MS),
    }
}

impl SidebarFooterMotion {
    pub fn attach_motion(self) -> String {
        let motion = sanitize_motion(self);

        format!(
            "--ui-sidebar-footer-motion-border-ms:{}ms; --ui-sidebar-footer-motion-opacity-ms:{}ms; --ui-sidebar-footer-motion-reduced-ms:{}ms;",
            motion.border_ms, motion.opacity_ms, motion.reduced_ms
        )
    }
}

#[cfg(test)]
#[path = "../../test/footer/motion.rs"]
mod tests;
