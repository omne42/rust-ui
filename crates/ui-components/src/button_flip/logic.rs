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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipButtonState {
    pub is_active: bool,
    pub is_inactive: bool,
    pub is_hovered: bool,
    pub is_focus_within: bool,
    pub direction: FlipDirection,
    pub direction_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_class_name(class_name: Option<String>) -> Option<String> {
    class_name.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(
    is_hovered: bool,
    is_focus_within: bool,
    is_active: bool,
    from: FlipDirection,
    has_custom_class_name: bool,
) -> FlipButtonState {
    FlipButtonState {
        is_active,
        is_inactive: !is_active,
        is_hovered,
        is_focus_within,
        direction: from,
        direction_attr: from.as_attr(),
        has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FlipButtonState) -> String {
    let mut classes = vec![
        "ui-flip-button".to_string(),
        format!("ui-flip-button--from-{}", state.direction_attr),
    ];

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
    fn attr_mapping_is_stable() {
        assert_eq!(FlipDirection::Top.as_attr(), "top");
        assert_eq!(FlipDirection::Bottom.as_attr(), "bottom");
        assert_eq!(FlipDirection::Left.as_attr(), "left");
        assert_eq!(FlipDirection::Right.as_attr(), "right");
    }

    #[test]
    fn normalize_class_name_trims_and_filters_blank_values() {
        assert_eq!(
            normalize_class_name(Some("  custom  ".to_string())),
            Some("custom".to_string())
        );
        assert_eq!(normalize_class_name(Some("  ".to_string())), None);
        assert_eq!(normalize_class_name(None), None);
    }

    #[test]
    fn resolve_state_tracks_interaction_and_direction_metadata() {
        let active = resolve_state(true, false, true, FlipDirection::Left, true);
        assert!(active.is_active);
        assert!(!active.is_inactive);
        assert!(active.is_hovered);
        assert!(!active.is_focus_within);
        assert_eq!(active.direction, FlipDirection::Left);
        assert_eq!(active.direction_attr, "left");
        assert!(active.has_custom_class_name);

        let inactive = resolve_state(false, true, false, FlipDirection::Bottom, false);
        assert!(!inactive.is_active);
        assert!(inactive.is_inactive);
        assert!(!inactive.is_hovered);
        assert!(inactive.is_focus_within);
        assert_eq!(inactive.direction_attr, "bottom");
        assert!(!inactive.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_direction_and_custom_class() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(false, false, false, FlipDirection::Right, true),
        );

        for token in ["ui-flip-button", "ui-flip-button--from-right", "custom"] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
