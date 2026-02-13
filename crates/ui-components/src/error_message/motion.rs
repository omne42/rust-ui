const DEFAULT_TRANSITION_MS: u16 = 140;
const MAX_TRANSITION_MS: u16 = 1_200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorMessageMotion {
    pub transition_ms: u16,
}

impl Default for ErrorMessageMotion {
    fn default() -> Self {
        Self {
            transition_ms: DEFAULT_TRANSITION_MS,
        }
    }
}

pub fn sanitize_motion(motion: ErrorMessageMotion) -> ErrorMessageMotion {
    ErrorMessageMotion {
        transition_ms: motion.transition_ms.clamp(1, MAX_TRANSITION_MS),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_transition_range() {
        assert_eq!(
            sanitize_motion(ErrorMessageMotion { transition_ms: 0 }).transition_ms,
            1
        );
        assert_eq!(
            sanitize_motion(ErrorMessageMotion {
                transition_ms: 4_000
            })
            .transition_ms,
            MAX_TRANSITION_MS
        );
    }
}
