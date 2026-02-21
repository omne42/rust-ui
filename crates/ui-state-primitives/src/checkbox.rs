#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxStateInput {
    pub is_checked: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxControlMode {
    Controlled,
    Uncontrolled,
}

impl CheckboxControlMode {
    pub const fn source_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxCheckedValueSource {
    IsChecked,
    CheckedAlias,
    DefaultChecked,
    ImplicitDefault,
}

impl CheckboxCheckedValueSource {
    pub const fn source_attr(self) -> &'static str {
        match self {
            Self::IsChecked => "is-checked",
            Self::CheckedAlias => "checked-alias",
            Self::DefaultChecked => "default-checked",
            Self::ImplicitDefault => "implicit-default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxCheckedAxisInput {
    pub is_checked: Option<bool>,
    pub checked: Option<bool>,
    pub default_checked: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxCheckedAxis {
    pub mode: CheckboxControlMode,
    pub checked: bool,
    pub source: CheckboxCheckedValueSource,
}

pub fn resolve_checked_axis(input: CheckboxCheckedAxisInput) -> CheckboxCheckedAxis {
    if let Some(is_checked) = input.is_checked {
        return CheckboxCheckedAxis {
            mode: CheckboxControlMode::Controlled,
            checked: is_checked,
            source: CheckboxCheckedValueSource::IsChecked,
        };
    }

    if let Some(checked) = input.checked {
        return CheckboxCheckedAxis {
            mode: CheckboxControlMode::Controlled,
            checked,
            source: CheckboxCheckedValueSource::CheckedAlias,
        };
    }

    if let Some(default_checked) = input.default_checked {
        return CheckboxCheckedAxis {
            mode: CheckboxControlMode::Uncontrolled,
            checked: default_checked,
            source: CheckboxCheckedValueSource::DefaultChecked,
        };
    }

    CheckboxCheckedAxis {
        mode: CheckboxControlMode::Uncontrolled,
        checked: false,
        source: CheckboxCheckedValueSource::ImplicitDefault,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxChangeHandlerSource {
    OnCheckedChange,
    SetCheckedAlias,
    Missing,
}

impl CheckboxChangeHandlerSource {
    pub const fn source_attr(self) -> &'static str {
        match self {
            Self::OnCheckedChange => "on-checked-change",
            Self::SetCheckedAlias => "set-checked-alias",
            Self::Missing => "missing",
        }
    }
}

pub fn resolve_checked_change_handler_source(
    has_on_checked_change: bool,
    has_set_checked: bool,
) -> CheckboxChangeHandlerSource {
    if has_on_checked_change {
        CheckboxChangeHandlerSource::OnCheckedChange
    } else if has_set_checked {
        CheckboxChangeHandlerSource::SetCheckedAlias
    } else {
        CheckboxChangeHandlerSource::Missing
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxState {
    pub is_checked: bool,
    pub is_unchecked: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
}

impl CheckboxState {
    pub fn data_state(self) -> &'static str {
        if self.is_checked {
            "checked"
        } else {
            "unchecked"
        }
    }
}

pub fn resolve_state(input: CheckboxStateInput) -> CheckboxState {
    let is_enabled = !input.is_disabled;

    CheckboxState {
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
#[path = "test/checkbox.rs"]
mod tests;
