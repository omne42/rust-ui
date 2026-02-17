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
        state.direction_class.to_string(),
        state.state_class.to_string(),
        state.hover_class.to_string(),
        state.focus_within_class.to_string(),
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
mod tests {
    use super::*;

    #[test]
    fn attr_and_class_mapping_are_stable() {
        assert_eq!(FlipDirection::Top.as_attr(), "top");
        assert_eq!(FlipDirection::Bottom.as_attr(), "bottom");
        assert_eq!(FlipDirection::Left.as_attr(), "left");
        assert_eq!(FlipDirection::Right.as_attr(), "right");

        assert_eq!(FlipDirection::Top.class_name(), "ui-flip-button--from-top");
        assert_eq!(
            FlipDirection::Bottom.class_name(),
            "ui-flip-button--from-bottom"
        );
        assert_eq!(
            FlipDirection::Left.class_name(),
            "ui-flip-button--from-left"
        );
        assert_eq!(
            FlipDirection::Right.class_name(),
            "ui-flip-button--from-right"
        );
    }

    #[test]
    fn normalize_input_centralizes_defaults_and_sources() {
        let normalized = normalize_input(FlipButtonInputNormalizationInput {
            from: None,
            motion: None,
            class_name: Some("  ".to_string()),
        });

        assert_eq!(normalized.direction, FlipDirection::Top);
        assert_eq!(normalized.motion, FlipButtonMotion::default());
        assert!(!normalized.has_custom_motion);
        assert!(!normalized.has_custom_class_name);
        assert_eq!(normalized.class_name, None);

        let custom = normalize_input(FlipButtonInputNormalizationInput {
            from: Some(FlipDirection::Right),
            motion: Some(FlipButtonMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: 320.0,
                    damping: 18.0,
                    mass: 1.0,
                    precision: 0.001,
                },
            }),
            class_name: Some(" custom ".to_string()),
        });

        assert_eq!(custom.direction, FlipDirection::Right);
        assert!(custom.has_custom_motion);
        assert!(custom.has_custom_class_name);
        assert_eq!(custom.class_name, Some("custom".to_string()));
    }

    #[test]
    fn resolve_state_tracks_interaction_and_source_metadata() {
        let active = resolve_state(FlipButtonStateInput {
            direction: FlipDirection::Left,
            is_hovered: true,
            is_focus_within: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        assert!(active.is_active);
        assert!(!active.is_inactive);
        assert!(active.is_hovered);
        assert!(!active.is_focus_within);
        assert_eq!(active.direction, FlipDirection::Left);
        assert_eq!(active.direction_attr, "left");
        assert_eq!(active.direction_class, "ui-flip-button--from-left");
        assert_eq!(active.state_attr, "active");
        assert_eq!(active.state_class, "ui-flip-button--state-active");
        assert_eq!(active.hover_attr, "hovered");
        assert_eq!(active.hover_class, "ui-flip-button--hovered");
        assert_eq!(active.focus_within_attr, "inactive");
        assert_eq!(active.focus_within_class, "ui-flip-button--no-focus-within");
        assert_eq!(active.class_source_attr, "custom");
        assert_eq!(active.motion_source_attr, "custom");
        assert!(active.has_custom_class_name);
        assert!(active.has_custom_motion);

        let inactive = resolve_state(FlipButtonStateInput {
            direction: FlipDirection::Bottom,
            is_hovered: false,
            is_focus_within: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        });

        assert!(!inactive.is_active);
        assert!(inactive.is_inactive);
        assert!(!inactive.is_hovered);
        assert!(!inactive.is_focus_within);
        assert_eq!(inactive.direction_attr, "bottom");
        assert_eq!(inactive.state_attr, "inactive");
        assert_eq!(inactive.hover_attr, "resting");
        assert_eq!(inactive.focus_within_attr, "inactive");
        assert_eq!(inactive.class_source_attr, "default");
        assert_eq!(inactive.motion_source_attr, "default");
        assert!(!inactive.has_custom_class_name);
        assert!(!inactive.has_custom_motion);
    }

    #[test]
    fn compose_class_name_includes_state_markers_and_custom_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(FlipButtonStateInput {
                direction: FlipDirection::Right,
                is_hovered: true,
                is_focus_within: true,
                has_custom_class_name: true,
                has_custom_motion: true,
            }),
        );

        for token in [
            "ui-flip-button",
            "ui-flip-button--from-right",
            "ui-flip-button--state-active",
            "ui-flip-button--hovered",
            "ui-flip-button--focus-within",
            "ui-flip-button--custom-class",
            "ui-flip-button--custom-motion",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn resolve_agent_contract_reuses_button_agent_schema_contract() {
        let contract = resolve_agent_contract(resolve_state(FlipButtonStateInput {
            direction: FlipDirection::Top,
            is_hovered: false,
            is_focus_within: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        }));

        assert_eq!(contract.schema_name, "ui.button.agent-contract");
        assert_eq!(contract.schema_version.as_str(), "1");
        assert_eq!(contract.intent.as_str(), "trigger");
        assert_eq!(contract.state.as_str(), "ready");
        assert!(contract.capabilities.can_press);
        assert!(contract.capabilities.can_focus);
        assert!(contract.capabilities.can_hover);
        assert!(!contract.capabilities.can_popup_trigger);
    }
}
