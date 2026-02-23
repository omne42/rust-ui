use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputGroupMotion {
    pub border_ms: u16,
    pub fill_ms: u16,
    pub reduced_ms: u16,
}

impl Default for InputGroupMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            border_ms: tokens.duration_ms,
            fill_ms: tokens.duration_ms,
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
#[path = "../../../test/input/group/motion.rs"]
mod tests;
