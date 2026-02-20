#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchStateInput {
    pub is_checked: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
}

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

pub fn resolve_state(input: SwitchStateInput) -> SwitchState {
    let is_enabled = !input.is_disabled;

    SwitchState {
        is_checked: input.is_checked,
        is_unchecked: !input.is_checked,
        is_disabled: input.is_disabled,
        is_enabled,
        is_pressed: input.is_pressed && is_enabled,
        is_hovered: input.is_hovered && is_enabled,
        is_focused: input.is_focused && is_enabled,
        is_focus_visible: input.is_focus_visible && is_enabled,
    }
}

#[cfg(test)]
#[path = "test/switch.rs"]
mod tests;
