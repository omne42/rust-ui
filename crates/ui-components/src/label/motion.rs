const DEFAULT_COLOR_TRANSITION_MS: u16 = 140;
const DEFAULT_WEIGHT_TRANSITION_MS: u16 = 120;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelMotion {
    pub color_transition_ms: u16,
    pub weight_transition_ms: u16,
}

impl Default for LabelMotion {
    fn default() -> Self {
        Self {
            color_transition_ms: DEFAULT_COLOR_TRANSITION_MS,
            weight_transition_ms: DEFAULT_WEIGHT_TRANSITION_MS,
        }
    }
}
