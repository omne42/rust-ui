#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputGroupMotion {
    pub border_ms: u16,
    pub fill_ms: u16,
    pub reduced_ms: u16,
}

impl Default for InputGroupMotion {
    fn default() -> Self {
        Self {
            border_ms: 120,
            fill_ms: 140,
            reduced_ms: 0,
        }
    }
}

pub fn sanitize_motion(motion: InputGroupMotion) -> InputGroupMotion {
    const MAX_MS: u16 = 5_000;

    InputGroupMotion {
        border_ms: motion.border_ms.min(MAX_MS),
        fill_ms: motion.fill_ms.min(MAX_MS),
        reduced_ms: motion.reduced_ms.min(MAX_MS),
    }
}

impl InputGroupMotion {
    pub fn attach_motion(self) -> String {
        let motion = sanitize_motion(self);

        format!(
            "--ui-input-group-motion-border-ms:{}ms; --ui-input-group-motion-fill-ms:{}ms; --ui-input-group-motion-reduced-ms:{}ms;",
            motion.border_ms, motion.fill_ms, motion.reduced_ms
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_duration_values() {
        let motion = sanitize_motion(InputGroupMotion {
            border_ms: u16::MAX,
            fill_ms: u16::MAX,
            reduced_ms: u16::MAX,
        });

        assert_eq!(motion.border_ms, 5_000);
        assert_eq!(motion.fill_ms, 5_000);
        assert_eq!(motion.reduced_ms, 5_000);
    }

    #[test]
    fn attach_motion_emits_css_variable_contract() {
        let style = InputGroupMotion {
            border_ms: 150,
            fill_ms: 160,
            reduced_ms: 0,
        }
        .attach_motion();

        assert!(style.contains("--ui-input-group-motion-border-ms:150ms;"));
        assert!(style.contains("--ui-input-group-motion-fill-ms:160ms;"));
        assert!(style.contains("--ui-input-group-motion-reduced-ms:0ms;"));
    }
}
