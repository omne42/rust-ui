use leptos::prelude::{Callback, Signal};
use ui_headless::RovingOrientation;

pub use ui_state_primitives::radio::{
    DEFAULT_CHECKED, RadioCheckedControlMode, RadioGroupOrientation, normalize_optional_text,
    resolve_accessible_name, resolve_state,
};
use ui_state_primitives::radio::{
    RadioCheckedAxisInput as PrimitiveRadioCheckedAxisInput, resolve_checked_axis,
};

pub struct DisabledPropInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub struct DisabledPropState {
    pub is_disabled: bool,
    pub disabled_source_attr: &'static str,
}

pub fn normalize_disabled_prop(input: DisabledPropInput) -> DisabledPropState {
    DisabledPropState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
        disabled_source_attr: if input.is_disabled.is_some() {
            "is_disabled"
        } else if input.disabled {
            "disabled"
        } else {
            "none"
        },
    }
}

pub struct CheckedAxisInput {
    pub is_checked: Option<Signal<bool>>,
    pub checked: Option<Signal<bool>>,
    pub default_checked: Option<bool>,
    pub on_checked_change: Option<Callback<bool>>,
    pub on_change: Option<Callback<bool>>,
}

pub struct CheckedAxisState {
    pub control_mode: RadioCheckedControlMode,
    pub controlled_checked: Option<Signal<bool>>,
    pub default_checked: bool,
    pub on_checked_change: Option<Callback<bool>>,
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub checked_source_attr: &'static str,
    pub default_checked_source_attr: &'static str,
    pub checked_change_source_attr: &'static str,
}

pub fn normalize_checked_axis(input: CheckedAxisInput) -> CheckedAxisState {
    let has_is_checked = input.is_checked.is_some();
    let has_checked = input.checked.is_some();
    let has_default_checked = input.default_checked.is_some();
    let has_on_checked_change = input.on_checked_change.is_some();
    let has_on_change = input.on_change.is_some();
    let primitive = resolve_checked_axis(PrimitiveRadioCheckedAxisInput {
        has_is_checked,
        has_checked,
        has_default_checked,
        has_on_checked_change,
        has_on_change,
    });

    CheckedAxisState {
        control_mode: primitive.control_mode,
        controlled_checked: input.is_checked.or(input.checked),
        default_checked: input.default_checked.unwrap_or(DEFAULT_CHECKED),
        on_checked_change: input.on_checked_change.or(input.on_change),
        is_controlled: primitive.is_controlled,
        control_mode_attr: primitive.control_mode_attr,
        checked_source_attr: primitive.checked_source_attr,
        default_checked_source_attr: primitive.default_checked_source_attr,
        checked_change_source_attr: primitive.checked_change_source_attr,
    }
}

pub fn roving_orientation(orientation: RadioGroupOrientation) -> RovingOrientation {
    match orientation {
        RadioGroupOrientation::Vertical => RovingOrientation::Vertical,
        RadioGroupOrientation::Horizontal => RovingOrientation::Horizontal,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
