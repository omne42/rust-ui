#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClearButtonMotion {
    pub hover_ms: u16,
    pub press_ms: u16,
    pub reduced_ms: u16,
}

impl Default for ClearButtonMotion {
    fn default() -> Self {
        Self {
            hover_ms: 120,
            press_ms: 90,
            reduced_ms: 0,
        }
    }
}

pub fn sanitize_motion(motion: ClearButtonMotion) -> ClearButtonMotion {
    const MAX_MS: u16 = 5_000;

    ClearButtonMotion {
        hover_ms: motion.hover_ms.min(MAX_MS),
        press_ms: motion.press_ms.min(MAX_MS),
        reduced_ms: motion.reduced_ms.min(MAX_MS),
    }
}

impl ClearButtonMotion {
    pub fn attach_motion(self) -> String {
        let motion = sanitize_motion(self);

        format!(
            "--ui-clear-button-motion-hover-ms:{}ms; --ui-clear-button-motion-press-ms:{}ms; --ui-clear-button-motion-reduced-ms:{}ms;",
            motion.hover_ms, motion.press_ms, motion.reduced_ms
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_duration_values() {
        let motion = sanitize_motion(ClearButtonMotion {
            hover_ms: u16::MAX,
            press_ms: u16::MAX,
            reduced_ms: u16::MAX,
        });

        assert_eq!(motion.hover_ms, 5_000);
        assert_eq!(motion.press_ms, 5_000);
        assert_eq!(motion.reduced_ms, 5_000);
    }

    #[test]
    fn attach_motion_emits_css_variable_contract() {
        let style = ClearButtonMotion {
            hover_ms: 140,
            press_ms: 90,
            reduced_ms: 0,
        }
        .attach_motion();

        assert!(style.contains("--ui-clear-button-motion-hover-ms:140ms;"));
        assert!(style.contains("--ui-clear-button-motion-press-ms:90ms;"));
        assert!(style.contains("--ui-clear-button-motion-reduced-ms:0ms;"));
    }
}
