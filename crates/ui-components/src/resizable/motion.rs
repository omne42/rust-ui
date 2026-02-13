const DEFAULT_PANEL_TRANSITION_MS: u16 = 160;
const DEFAULT_HANDLE_TRANSITION_MS: u16 = 120;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResizableMotion {
    pub panel_transition_ms: u16,
    pub handle_transition_ms: u16,
}

impl Default for ResizableMotion {
    fn default() -> Self {
        Self {
            panel_transition_ms: DEFAULT_PANEL_TRANSITION_MS,
            handle_transition_ms: DEFAULT_HANDLE_TRANSITION_MS,
        }
    }
}
