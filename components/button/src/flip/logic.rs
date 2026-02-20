use super::motion::{self, FlipButtonMotion};
use ui_state_primitives::button_flip::{FlipButtonStateCoreInput, resolve_state_core};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlipDirection {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl FlipDirection {
    pub fn as_attr(self) -> &'static str {
        match self {
            FlipDirection::Top => "top",
            FlipDirection::Bottom => "bottom",
            FlipDirection::Left => "left",
            FlipDirection::Right => "right",
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            FlipDirection::Top => "ui-flip-button--from-top",
            FlipDirection::Bottom => "ui-flip-button--from-bottom",
            FlipDirection::Left => "ui-flip-button--from-left",
            FlipDirection::Right => "ui-flip-button--from-right",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlipButtonInputNormalizationInput {
    pub from: Option<FlipDirection>,
    pub motion: Option<FlipButtonMotion>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlipButtonInputNormalization {
    pub direction: FlipDirection,
    pub motion: FlipButtonMotion,
    pub class_name: Option<String>,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipButtonStateInput {
    pub direction: FlipDirection,
    pub is_hovered: bool,
    pub is_focus_within: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipButtonState {
    pub direction: FlipDirection,
    pub direction_attr: &'static str,
    pub direction_class: &'static str,
    pub is_active: bool,
    pub is_inactive: bool,
    pub state_attr: &'static str,
    pub state_class: &'static str,
    pub is_hovered: bool,
    pub hover_attr: &'static str,
    pub hover_class: &'static str,
    pub is_focus_within: bool,
    pub focus_within_attr: &'static str,
    pub focus_within_class: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

pub fn normalize_input(input: FlipButtonInputNormalizationInput) -> FlipButtonInputNormalization {
    let direction = input.from.unwrap_or_default();
    let motion = input
        .motion
        .map(motion::sanitize_motion)
        .unwrap_or_default();
    let class_name = super::super::logic::normalize_optional_text(input.class_name);

    FlipButtonInputNormalization {
        direction,
        motion,
        has_custom_motion: motion != FlipButtonMotion::default(),
        has_custom_class_name: class_name.is_some(),
        class_name,
    }
}

pub fn resolve_state(input: FlipButtonStateInput) -> FlipButtonState {
    let core = resolve_state_core(FlipButtonStateCoreInput {
        is_hovered: input.is_hovered,
        is_focus_within: input.is_focus_within,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    });

    let state_class = if core.is_active {
        "ui-flip-button--state-active"
    } else {
        "ui-flip-button--state-inactive"
    };

    let hover_class = if input.is_hovered {
        "ui-flip-button--hovered"
    } else {
        "ui-flip-button--not-hovered"
    };

    let focus_within_class = if input.is_focus_within {
        "ui-flip-button--focus-within"
    } else {
        "ui-flip-button--no-focus-within"
    };

    FlipButtonState {
        direction: input.direction,
        direction_attr: input.direction.as_attr(),
        direction_class: input.direction.class_name(),
        is_active: core.is_active,
        is_inactive: core.is_inactive,
        state_attr: core.state_attr,
        state_class,
        is_hovered: input.is_hovered,
        hover_attr: core.hover_attr,
        hover_class,
        is_focus_within: input.is_focus_within,
        focus_within_attr: core.focus_within_attr,
        focus_within_class,
        class_source_attr: core.class_source_attr,
        motion_source_attr: core.motion_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FlipButtonState) -> String {
    let mut classes = vec![
        "ui-flip-button".to_string(),
        state.direction_class.into(),
        state.state_class.into(),
        state.hover_class.into(),
        state.focus_within_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-flip-button--custom-class".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-flip-button--custom-motion".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

pub fn resolve_agent_contract(state: FlipButtonState) -> super::super::logic::ButtonAgentContract {
    debug_assert_eq!(state.is_active, state.state_attr == "active");
    debug_assert_eq!(state.is_inactive, !state.is_active);
    super::super::logic::resolve_agent_contract_for_state_axis(
        super::super::logic::ButtonAgentStateAxis::Ready,
        false,
    )
}

#[cfg(test)]
#[path = "../../test/flip/logic.rs"]
mod tests;
