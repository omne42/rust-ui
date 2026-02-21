use ui_state_primitives::toggle_button as toggle_button_state;

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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::button::normalize_optional_text(value)
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    variant: ToggleButtonVariant,
    size: ToggleButtonSize,
) -> String {
    let mut classes = vec![
        "ui-toggle-button".to_string(),
        variant.class_name().to_string(),
        size.class_name().to_string(),
    ];

    if let Some(base_class_name) = base_class_name {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

pub use toggle_button_state::ToggleButtonState;

pub fn resolve_state(
    is_selected: bool,
    is_disabled: bool,
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
    is_focus_visible: bool,
) -> ToggleButtonState {
    toggle_button_state::resolve_toggle_button_state(
        is_selected,
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        is_focus_visible,
    )
}

#[cfg(feature = "component-toggle_button_group")]
pub use toggle_button_state::{ToggleButtonGroupOrientation, ToggleButtonGroupState};

#[cfg(feature = "component-toggle_button_group")]
pub fn normalize_toggle_button_group_aria_label(aria_label: Option<String>) -> (String, bool) {
    toggle_button_state::normalize_toggle_button_group_aria_label(aria_label)
}

#[cfg(feature = "component-toggle_button_group")]
pub fn resolve_toggle_button_group_state(
    orientation: ToggleButtonGroupOrientation,
    attached: bool,
    has_explicit_label: bool,
) -> ToggleButtonGroupState {
    toggle_button_state::resolve_toggle_button_group_state(
        orientation,
        attached,
        has_explicit_label,
    )
}

#[cfg(feature = "component-toggle_button_group")]
pub fn compose_toggle_button_group_class_name(
    base_class_name: Option<String>,
    orientation: ToggleButtonGroupOrientation,
    is_attached: bool,
) -> String {
    let mut classes = vec![
        "ui-toggle-button-group".to_string(),
        orientation.class_name().to_string(),
    ];

    if is_attached {
        classes.push("ui-toggle-button-group--attached".to_string());
    }

    if let Some(base_class_name) = base_class_name {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/toggle_button/logic.rs"]
mod tests;
