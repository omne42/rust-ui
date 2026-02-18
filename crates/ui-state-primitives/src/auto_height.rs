#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutoHeightStateInput {
    pub animate_height: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutoHeightState {
    pub overflow_hidden: bool,
    pub animate_height: bool,
    pub is_static: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

pub fn resolve_state(input: AutoHeightStateInput) -> AutoHeightState {
    AutoHeightState {
        overflow_hidden: true,
        animate_height: input.animate_height,
        is_static: !input.animate_height,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_is_overflow_hidden() {
        let state = resolve_state(AutoHeightStateInput {
            animate_height: true,
            has_custom_class_name: false,
            has_custom_motion: false,
        });

        assert!(state.overflow_hidden);
        assert!(state.animate_height);
        assert!(!state.is_static);
        assert!(!state.has_custom_class_name);
        assert!(!state.has_custom_motion);
    }

    #[test]
    fn resolve_state_tracks_static_and_custom_flags() {
        let state = resolve_state(AutoHeightStateInput {
            animate_height: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        assert!(state.overflow_hidden);
        assert!(!state.animate_height);
        assert!(state.is_static);
        assert!(state.has_custom_class_name);
        assert!(state.has_custom_motion);
    }
}
