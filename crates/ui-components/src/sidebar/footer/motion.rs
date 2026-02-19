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
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_duration_values() {
        let motion = sanitize_motion(SidebarFooterMotion {
            border_ms: u16::MAX,
            opacity_ms: u16::MAX,
            reduced_ms: u16::MAX,
        });

        assert_eq!(motion.border_ms, 5_000);
        assert_eq!(motion.opacity_ms, 5_000);
        assert_eq!(motion.reduced_ms, 5_000);
    }

    #[test]
    fn attach_motion_emits_css_variable_contract() {
        let style = SidebarFooterMotion {
            border_ms: 110,
            opacity_ms: 130,
            reduced_ms: 0,
        }
        .attach_motion();

        assert!(style.contains("--ui-sidebar-footer-motion-border-ms:110ms;"));
        assert!(style.contains("--ui-sidebar-footer-motion-opacity-ms:130ms;"));
        assert!(style.contains("--ui-sidebar-footer-motion-reduced-ms:0ms;"));
    }
}
