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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipButtonStateInput {
    pub direction: FlipDirection,
    pub is_hovered: bool,
    pub is_focus_within: bool,
    pub is_active: bool,
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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: FlipButtonStateInput) -> FlipButtonState {
    let (state_attr, state_class) = if input.is_active {
        ("active", "ui-flip-button--state-active")
    } else {
        ("inactive", "ui-flip-button--state-inactive")
    };

    let (hover_attr, hover_class) = if input.is_hovered {
        ("hovered", "ui-flip-button--hovered")
    } else {
        ("resting", "ui-flip-button--not-hovered")
    };

    let (focus_within_attr, focus_within_class) = if input.is_focus_within {
        ("active", "ui-flip-button--focus-within")
    } else {
        ("inactive", "ui-flip-button--no-focus-within")
    };

    FlipButtonState {
        direction: input.direction,
        direction_attr: input.direction.as_attr(),
        direction_class: input.direction.class_name(),
        is_active: input.is_active,
        is_inactive: !input.is_active,
        state_attr,
        state_class,
        is_hovered: input.is_hovered,
        hover_attr,
        hover_class,
        is_focus_within: input.is_focus_within,
        focus_within_attr,
        focus_within_class,
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
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
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(
            normalize_optional_text(Some("  custom  ".to_string())),
            Some("custom".to_string())
        );
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn resolve_state_tracks_interaction_and_source_metadata() {
        let active = resolve_state(FlipButtonStateInput {
            direction: FlipDirection::Left,
            is_hovered: true,
            is_focus_within: false,
            is_active: true,
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
            is_focus_within: true,
            is_active: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        });

        assert!(!inactive.is_active);
        assert!(inactive.is_inactive);
        assert!(!inactive.is_hovered);
        assert!(inactive.is_focus_within);
        assert_eq!(inactive.direction_attr, "bottom");
        assert_eq!(inactive.state_attr, "inactive");
        assert_eq!(inactive.hover_attr, "resting");
        assert_eq!(inactive.focus_within_attr, "active");
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
                is_active: true,
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
}
