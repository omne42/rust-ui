const DEFAULT_COLOR_TRANSITION_MS: u16 = 180;
const DEFAULT_TRUNCATE_TRANSITION_MS: u16 = 120;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeadingMotion {
    pub color_transition_ms: u16,
    pub truncate_transition_ms: u16,
}

impl Default for HeadingMotion {
    fn default() -> Self {
        Self {
            color_transition_ms: DEFAULT_COLOR_TRANSITION_MS,
            truncate_transition_ms: DEFAULT_TRUNCATE_TRANSITION_MS,
        }
    }
}
