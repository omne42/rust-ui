#[cfg(target_arch = "wasm32")]
pub const TRACK_WIDTH_PX: f64 = 32.0;

#[cfg(target_arch = "wasm32")]
pub const TRACK_PADDING_PX: f64 = 2.0;

#[cfg(target_arch = "wasm32")]
pub const THUMB_WIDTH_PX: f64 = 16.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchState {
    pub is_checked: bool,
    pub is_unchecked: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
}

impl SwitchState {
    pub fn data_state(self) -> &'static str {
        if self.is_checked {
            "checked"
        } else {
            "unchecked"
        }
    }
}

pub fn resolve_state(
    is_checked: bool,
    is_disabled: bool,
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
    is_focus_visible: bool,
) -> SwitchState {
    let is_enabled = !is_disabled;

    SwitchState {
        is_checked,
        is_unchecked: !is_checked,
        is_disabled,
        is_enabled,
        is_pressed: is_pressed && is_enabled,
        is_hovered: is_hovered && is_enabled,
        is_focused: is_focused && is_enabled,
        is_focus_visible: is_focus_visible && is_enabled,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn checked_thumb_x_px(thumb_width_px: f64) -> f64 {
    let inner_width = TRACK_WIDTH_PX - (TRACK_PADDING_PX * 2.0);
    (inner_width - thumb_width_px).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_state_tracks_checked_enabled_interactions() {
        let state = resolve_state(true, false, true, true, true, true);

        assert!(state.is_checked);
        assert!(!state.is_unchecked);
        assert!(!state.is_disabled);
        assert!(state.is_enabled);
        assert!(state.is_pressed);
        assert!(state.is_hovered);
        assert!(state.is_focused);
        assert!(state.is_focus_visible);
        assert_eq!(state.data_state(), "checked");
    }

    #[test]
    fn resolve_state_clears_interaction_flags_when_disabled() {
        let state = resolve_state(false, true, true, true, true, true);

        assert!(!state.is_checked);
        assert!(state.is_unchecked);
        assert!(state.is_disabled);
        assert!(!state.is_enabled);
        assert!(!state.is_pressed);
        assert!(!state.is_hovered);
        assert!(!state.is_focused);
        assert!(!state.is_focus_visible);
        assert_eq!(state.data_state(), "unchecked");
    }
}
