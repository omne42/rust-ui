use leptos::prelude::{Callable, Callback, Set, Signal, WriteSignal};
use ui_state_primitives::switch::{
    SwitchCheckedAxisInput as PrimitiveSwitchCheckedAxisInput, resolve_checked_axis,
};

const ROOT_CLASS: &str = "ui-switch";
pub use ui_state_primitives::switch::{DEFAULT_CHECKED, SwitchCheckedControlMode};

pub struct CheckedAxisInput {
    pub checked: Option<Signal<bool>>,
    pub set_checked: Option<WriteSignal<bool>>,
    pub default_checked: Option<bool>,
    pub on_checked_change: Option<Callback<bool>>,
}

pub struct CheckedAxisState {
    pub control_mode: SwitchCheckedControlMode,
    pub controlled_checked: Option<Signal<bool>>,
    pub default_checked: bool,
    pub on_checked_change: Option<Callback<bool>>,
    pub is_controlled: bool,
    pub checked_source_attr: &'static str,
    pub default_checked_source_attr: &'static str,
    pub checked_change_source_attr: &'static str,
}

pub fn normalize_checked_axis(input: CheckedAxisInput) -> CheckedAxisState {
    let has_on_checked_change = input.on_checked_change.is_some();
    let has_set_checked = input.set_checked.is_some();
    let primitive = resolve_checked_axis(PrimitiveSwitchCheckedAxisInput {
        has_checked: input.checked.is_some(),
        has_default_checked: input.default_checked.is_some(),
        has_on_checked_change,
        has_set_checked,
    });

    let on_checked_change = match (input.on_checked_change, input.set_checked) {
        (Some(on_checked_change), Some(set_checked)) => {
            Some(Callback::new(move |next| {
                // Keep legacy ordering: write signal first, then callback.
                set_checked.set(next);
                on_checked_change.run(next);
            }))
        }
        (Some(on_checked_change), None) => Some(on_checked_change),
        (None, Some(set_checked)) => Some(Callback::new(move |next| set_checked.set(next))),
        (None, None) => None,
    };

    CheckedAxisState {
        control_mode: primitive.control_mode,
        controlled_checked: input.checked,
        default_checked: input.default_checked.unwrap_or(DEFAULT_CHECKED),
        on_checked_change,
        is_controlled: primitive.is_controlled,
        checked_source_attr: primitive.checked_source_attr,
        default_checked_source_attr: primitive.default_checked_source_attr,
        checked_change_source_attr: primitive.checked_change_source_attr,
    }
}

pub const fn next_checked(is_checked: bool) -> bool {
    !is_checked
}

pub fn compose_class_name(class_name: Option<String>) -> String {
    match class_name {
        Some(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                ROOT_CLASS.to_string()
            } else {
                format!("{ROOT_CLASS} {trimmed}")
            }
        }
        None => ROOT_CLASS.to_string(),
    }
}

pub fn resolve_motion_markers(is_custom_motion: bool) -> (&'static str, Option<&'static str>) {
    if is_custom_motion {
        ("custom", Some("true"))
    } else {
        ("default", None)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn default_thumb_size_px() -> f64 {
    f64::from(ui_theme::default_switch_layout_tokens().thumb_size_px)
}

#[cfg(target_arch = "wasm32")]
pub fn checked_thumb_x_px(thumb_width_px: f64) -> f64 {
    let tokens = ui_theme::default_switch_layout_tokens();
    let track_width_px = f64::from(tokens.track_width_px);
    let track_padding_px = f64::from(tokens.track_padding_px);
    let inner_width = track_width_px - (track_padding_px * 2.0);
    (inner_width - thumb_width_px).max(0.0)
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
