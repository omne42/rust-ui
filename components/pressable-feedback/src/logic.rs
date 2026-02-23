use leptos::prelude::{Callback, Signal};

pub use ui_state_primitives::pressable_feedback::{
    DEFAULT_ARIA_LABEL, PressableFeedbackDefaultPressedSource, PressableFeedbackEffect,
    PressableFeedbackPressedAxisInput, PressableFeedbackPressedAxisState,
    PressableFeedbackPressedChangeSource, PressableFeedbackPressedMode,
    PressableFeedbackStateContractInput, PressableFeedbackStateInput, PressableFeedbackTone,
    compose_class_name, normalize_state_contract, resolve_pressed_axis_state, resolve_state,
};
#[cfg(test)]
pub use ui_state_primitives::pressable_feedback::{
    DEFAULT_IS_BOUNDED, DEFAULT_IS_DISABLED, normalize_aria_label, normalize_flags,
    normalize_optional_text,
};

#[derive(Clone)]
pub struct PressableFeedbackPressedAxis {
    pub value: Option<Signal<bool>>,
    pub default_value: bool,
    pub on_value_change: Option<Callback<bool>>,
    pub pressed_mode: PressableFeedbackPressedMode,
    pub default_pressed_source: PressableFeedbackDefaultPressedSource,
    pub pressed_change_source: PressableFeedbackPressedChangeSource,
}

pub fn normalize_pressed_axis(
    is_pressed: Option<Signal<bool>>,
    default_pressed: Option<bool>,
    on_pressed_change: Option<Callback<bool>>,
) -> PressableFeedbackPressedAxis {
    let pressed_axis_state: PressableFeedbackPressedAxisState =
        resolve_pressed_axis_state(PressableFeedbackPressedAxisInput {
            has_controlled_value: is_pressed.is_some(),
            default_pressed,
            has_on_pressed_change: on_pressed_change.is_some(),
        });

    PressableFeedbackPressedAxis {
        value: is_pressed,
        default_value: pressed_axis_state.default_pressed,
        on_value_change: on_pressed_change,
        pressed_mode: pressed_axis_state.pressed_mode,
        default_pressed_source: pressed_axis_state.default_pressed_source,
        pressed_change_source: pressed_axis_state.pressed_change_source,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
