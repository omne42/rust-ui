#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleButtonVariant {
    #[default]
    Default,
    Accent,
    Destructive,
    Outline,
    Secondary,
    Ghost,
}

impl ToggleButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ToggleButtonVariant::Default => "ui-toggle-button--variant-default",
            ToggleButtonVariant::Accent => "ui-toggle-button--variant-accent",
            ToggleButtonVariant::Destructive => "ui-toggle-button--variant-destructive",
            ToggleButtonVariant::Outline => "ui-toggle-button--variant-outline",
            ToggleButtonVariant::Secondary => "ui-toggle-button--variant-secondary",
            ToggleButtonVariant::Ghost => "ui-toggle-button--variant-ghost",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleButtonSize {
    Xs,
    S,
    #[default]
    M,
    L,
    Xl,
    IconXs,
    IconS,
    IconM,
    IconL,
    IconXl,
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ToggleButtonSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ToggleButtonSize::Xs => "ui-toggle-button--size-xs",
            ToggleButtonSize::S => "ui-toggle-button--size-s",
            ToggleButtonSize::M => "ui-toggle-button--size-m",
            ToggleButtonSize::L => "ui-toggle-button--size-l",
            ToggleButtonSize::Xl => "ui-toggle-button--size-xl",
            ToggleButtonSize::IconXs => "ui-toggle-button--size-icon-xs",
            ToggleButtonSize::IconS => "ui-toggle-button--size-icon-s",
            ToggleButtonSize::IconM => "ui-toggle-button--size-icon-m",
            ToggleButtonSize::IconL => "ui-toggle-button--size-icon-l",
            ToggleButtonSize::IconXl => "ui-toggle-button--size-icon-xl",
            ToggleButtonSize::Default => "ui-toggle-button--size-m",
            ToggleButtonSize::Sm => "ui-toggle-button--size-s",
            ToggleButtonSize::Lg => "ui-toggle-button--size-l",
            ToggleButtonSize::Icon => "ui-toggle-button--size-icon-m",
            ToggleButtonSize::IconSm => "ui-toggle-button--size-icon-s",
            ToggleButtonSize::IconLg => "ui-toggle-button--size-icon-l",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleButtonState {
    pub is_selected: bool,
    pub is_unselected: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
}

impl ToggleButtonState {
    pub fn data_state(self) -> &'static str {
        if self.is_selected {
            "selected"
        } else {
            "unselected"
        }
    }
}

pub fn resolve_state(
    is_selected: bool,
    is_disabled: bool,
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
    is_focus_visible: bool,
) -> ToggleButtonState {
    let is_enabled = !is_disabled;

    ToggleButtonState {
        is_selected,
        is_unselected: !is_selected,
        is_disabled,
        is_enabled,
        is_pressed: is_pressed && is_enabled,
        is_hovered: is_hovered && is_enabled,
        is_focused: is_focused && is_enabled,
        is_focus_visible: is_focus_visible && is_enabled,
    }
}

#[cfg(feature = "component-toggle_button_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[cfg(feature = "component-toggle_button_group")]
impl ToggleButtonGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            ToggleButtonGroupOrientation::Horizontal => "ui-toggle-button-group--horizontal",
            ToggleButtonGroupOrientation::Vertical => "ui-toggle-button-group--vertical",
        }
    }

    pub fn data_orientation(self) -> &'static str {
        match self {
            ToggleButtonGroupOrientation::Horizontal => "horizontal",
            ToggleButtonGroupOrientation::Vertical => "vertical",
        }
    }
}

#[cfg(feature = "component-toggle_button_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleButtonGroupState {
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub is_attached: bool,
    pub is_detached: bool,
    pub has_explicit_label: bool,
    pub has_fallback_label: bool,
}

#[cfg(feature = "component-toggle_button_group")]
pub fn normalize_toggle_button_group_aria_label(aria_label: Option<String>) -> (String, bool) {
    if let Some(label) = aria_label {
        let trimmed = label.trim();
        if !trimmed.is_empty() {
            return (trimmed.into(), true);
        }
    }

    ("Toggle group".to_string(), false)
}

#[cfg(feature = "component-toggle_button_group")]
pub fn resolve_toggle_button_group_state(
    orientation: ToggleButtonGroupOrientation,
    attached: bool,
    has_explicit_label: bool,
) -> ToggleButtonGroupState {
    ToggleButtonGroupState {
        is_horizontal: matches!(orientation, ToggleButtonGroupOrientation::Horizontal),
        is_vertical: matches!(orientation, ToggleButtonGroupOrientation::Vertical),
        is_attached: attached,
        is_detached: !attached,
        has_explicit_label,
        has_fallback_label: !has_explicit_label,
    }
}

#[cfg(test)]
#[path = "../../test/toggle_button/logic.rs"]
mod tests;
