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
#[path = "../test/motion.rs"]
mod tests;
