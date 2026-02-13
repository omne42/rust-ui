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
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_duration_values() {
        let motion = sanitize_motion(BadgeMotion {
            enter_ms: u16::MAX,
            exit_ms: u16::MAX,
            reduced_ms: u16::MAX,
        });

        assert_eq!(motion.enter_ms, 5_000);
        assert_eq!(motion.exit_ms, 5_000);
        assert_eq!(motion.reduced_ms, 5_000);
    }

    #[test]
    fn attach_motion_emits_css_variable_contract() {
        let style = BadgeMotion {
            enter_ms: 180,
            exit_ms: 120,
            reduced_ms: 0,
        }
        .attach_motion();

        assert!(style.contains("--ui-badge-motion-enter-ms:180ms;"));
        assert!(style.contains("--ui-badge-motion-exit-ms:120ms;"));
        assert!(style.contains("--ui-badge-motion-reduced-ms:0ms;"));
    }
}
