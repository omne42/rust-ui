#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BadgeMotion {
    pub enter_ms: u16,
    pub exit_ms: u16,
    pub reduced_ms: u16,
}

impl Default for BadgeMotion {
    fn default() -> Self {
        Self {
            enter_ms: 120,
            exit_ms: 90,
            reduced_ms: 0,
        }
    }
}

pub fn sanitize_motion(motion: BadgeMotion) -> BadgeMotion {
    const MAX_MS: u16 = 5_000;

    BadgeMotion {
        enter_ms: motion.enter_ms.min(MAX_MS),
        exit_ms: motion.exit_ms.min(MAX_MS),
        reduced_ms: motion.reduced_ms.min(MAX_MS),
    }
}

impl BadgeMotion {
    pub fn attach_motion(self) -> String {
        let motion = sanitize_motion(self);

        format!(
            "--ui-badge-motion-enter-ms:{}ms; --ui-badge-motion-exit-ms:{}ms; --ui-badge-motion-reduced-ms:{}ms;",
            motion.enter_ms, motion.exit_ms, motion.reduced_ms
        )
    }
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
