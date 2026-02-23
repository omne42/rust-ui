pub const DEFAULT_CHECKED: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchCheckedControlMode {
    Controlled,
    Uncontrolled,
}

impl SwitchCheckedControlMode {
    pub const fn from_is_controlled(is_controlled: bool) -> Self {
        if is_controlled {
            Self::Controlled
        } else {
            Self::Uncontrolled
        }
    }

    pub const fn is_controlled(self) -> bool {
        matches!(self, Self::Controlled)
    }

    pub const fn data_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchCheckedAxisInput {
    pub has_checked: bool,
    pub has_default_checked: bool,
    pub has_on_checked_change: bool,
    pub has_set_checked: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchCheckedAxisState {
    pub control_mode: SwitchCheckedControlMode,
    pub is_controlled: bool,
    pub checked_source_attr: &'static str,
    pub default_checked_source_attr: &'static str,
    pub checked_change_source_attr: &'static str,
}

pub fn resolve_checked_axis(input: SwitchCheckedAxisInput) -> SwitchCheckedAxisState {
    let control_mode = SwitchCheckedControlMode::from_is_controlled(input.has_checked);
    let is_controlled = control_mode.is_controlled();
    let checked_source_attr = if input.has_checked {
        "checked"
    } else {
        "default"
    };
    let default_checked_source_attr = if input.has_default_checked {
        "provided"
    } else {
        "default"
    };
    let checked_change_source_attr = match (input.has_on_checked_change, input.has_set_checked) {
        (true, true) => "on_checked_change+set_checked",
        (true, false) => "on_checked_change",
        (false, true) => "set_checked",
        (false, false) => "none",
    };

    SwitchCheckedAxisState {
        control_mode,
        is_controlled,
        checked_source_attr,
        default_checked_source_attr,
        checked_change_source_attr,
    }
}

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
