#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipButtonStateCoreInput {
    pub is_hovered: bool,
    pub is_focus_within: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipButtonStateCore {
    pub is_active: bool,
    pub is_inactive: bool,
    pub state_attr: &'static str,
    pub hover_attr: &'static str,
    pub focus_within_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}

pub fn resolve_state_core(input: FlipButtonStateCoreInput) -> FlipButtonStateCore {
    let is_active = input.is_hovered || input.is_focus_within;

    FlipButtonStateCore {
        is_active,
        is_inactive: !is_active,
        state_attr: if is_active { "active" } else { "inactive" },
        hover_attr: if input.is_hovered {
            "hovered"
        } else {
            "resting"
        },
        focus_within_attr: if input.is_focus_within {
            "active"
        } else {
            "inactive"
        },
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_state_core_tracks_interaction_and_source_metadata() {
        let active = resolve_state_core(FlipButtonStateCoreInput {
            is_hovered: true,
            is_focus_within: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        assert!(active.is_active);
        assert!(!active.is_inactive);
        assert_eq!(active.state_attr, "active");
        assert_eq!(active.hover_attr, "hovered");
        assert_eq!(active.focus_within_attr, "inactive");
        assert_eq!(active.class_source_attr, "custom");
        assert_eq!(active.motion_source_attr, "custom");

        let inactive = resolve_state_core(FlipButtonStateCoreInput {
            is_hovered: false,
            is_focus_within: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        });

        assert!(!inactive.is_active);
        assert!(inactive.is_inactive);
        assert_eq!(inactive.state_attr, "inactive");
        assert_eq!(inactive.hover_attr, "resting");
        assert_eq!(inactive.focus_within_attr, "inactive");
        assert_eq!(inactive.class_source_attr, "default");
        assert_eq!(inactive.motion_source_attr, "default");
    }
}
