const DEFAULT_HOVER_TRANSITION_MS: u16 = 120;
const DEFAULT_FOCUS_RING_TRANSITION_MS: u16 = 90;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuActionMotion {
    pub hover_transition_ms: u16,
    pub focus_ring_transition_ms: u16,
}

impl Default for SidebarMenuActionMotion {
    fn default() -> Self {
        Self {
            hover_transition_ms: DEFAULT_HOVER_TRANSITION_MS,
            focus_ring_transition_ms: DEFAULT_FOCUS_RING_TRANSITION_MS,
        }
    }
}
