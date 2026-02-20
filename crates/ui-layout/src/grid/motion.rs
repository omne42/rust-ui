#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridMotion {
    pub layout_ms: u16,
    pub fade_ms: u16,
    pub reduced_ms: u16,
}

impl Default for GridMotion {
    fn default() -> Self {
        Self {
            layout_ms: 160,
            fade_ms: 120,
            reduced_ms: 0,
        }
    }
}

pub fn sanitize_motion(motion: GridMotion) -> GridMotion {
    const MAX_MS: u16 = 5_000;

    GridMotion {
        layout_ms: motion.layout_ms.min(MAX_MS),
        fade_ms: motion.fade_ms.min(MAX_MS),
        reduced_ms: motion.reduced_ms.min(MAX_MS),
    }
}

impl GridMotion {
    pub fn attach_motion(self) -> String {
        let motion = sanitize_motion(self);

        format!(
            "--ui-grid-motion-layout-ms:{}ms; --ui-grid-motion-fade-ms:{}ms; --ui-grid-motion-reduced-ms:{}ms;",
            motion.layout_ms, motion.fade_ms, motion.reduced_ms
        )
    }
}

#[cfg(test)]
#[path = "test/motion.rs"]
mod tests;
