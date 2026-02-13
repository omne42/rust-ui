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
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_duration_values() {
        let motion = sanitize_motion(GridMotion {
            layout_ms: u16::MAX,
            fade_ms: u16::MAX,
            reduced_ms: u16::MAX,
        });

        assert_eq!(motion.layout_ms, 5_000);
        assert_eq!(motion.fade_ms, 5_000);
        assert_eq!(motion.reduced_ms, 5_000);
    }

    #[test]
    fn attach_motion_emits_css_variable_contract() {
        let style = GridMotion {
            layout_ms: 200,
            fade_ms: 140,
            reduced_ms: 0,
        }
        .attach_motion();

        assert!(style.contains("--ui-grid-motion-layout-ms:200ms;"));
        assert!(style.contains("--ui-grid-motion-fade-ms:140ms;"));
        assert!(style.contains("--ui-grid-motion-reduced-ms:0ms;"));
    }
}
