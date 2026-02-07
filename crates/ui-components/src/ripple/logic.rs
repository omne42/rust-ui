#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RipplePhase {
    Animated,
    Static,
}

impl RipplePhase {
    pub fn class_name(self) -> &'static str {
        match self {
            RipplePhase::Animated => "ui-ripple--state-animated",
            RipplePhase::Static => "ui-ripple--state-static",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            RipplePhase::Animated => "animated",
            RipplePhase::Static => "static",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RippleBoundary {
    Bounded,
    Unbounded,
}

impl RippleBoundary {
    pub fn class_name(self) -> &'static str {
        match self {
            RippleBoundary::Bounded => "ui-ripple--boundary-bounded",
            RippleBoundary::Unbounded => "ui-ripple--boundary-unbounded",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            RippleBoundary::Bounded => "bounded",
            RippleBoundary::Unbounded => "unbounded",
        }
    }

    pub fn is_bounded(self) -> bool {
        matches!(self, RippleBoundary::Bounded)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RippleStateInput {
    pub phase: RipplePhase,
    pub boundary: RippleBoundary,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RippleState {
    pub phase: RipplePhase,
    pub phase_class: &'static str,
    pub phase_attr: &'static str,
    pub boundary: RippleBoundary,
    pub boundary_class: &'static str,
    pub boundary_attr: &'static str,
    pub is_bounded: bool,
    pub is_unbounded: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
    pub motion_source_class: &'static str,
    pub motion_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_phase(animated: bool) -> RipplePhase {
    if animated {
        RipplePhase::Animated
    } else {
        RipplePhase::Static
    }
}

pub fn resolve_boundary(bounded: bool) -> RippleBoundary {
    if bounded {
        RippleBoundary::Bounded
    } else {
        RippleBoundary::Unbounded
    }
}

pub fn resolve_state(input: RippleStateInput) -> RippleState {
    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-ripple--motion-custom", "custom")
    } else {
        ("ui-ripple--motion-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    RippleState {
        phase: input.phase,
        phase_class: input.phase.class_name(),
        phase_attr: input.phase.as_str(),
        boundary: input.boundary,
        boundary_class: input.boundary.class_name(),
        boundary_attr: input.boundary.as_str(),
        is_bounded: input.boundary.is_bounded(),
        is_unbounded: !input.boundary.is_bounded(),
        has_custom_motion: input.has_custom_motion,
        has_custom_class_name: input.has_custom_class_name,
        motion_source_class,
        motion_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: RippleState) -> String {
    let mut classes = vec![
        "ui-ripple".to_string(),
        state.phase_class.to_string(),
        state.boundary_class.to_string(),
        state.motion_source_class.to_string(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-ripple--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_contract_is_stable() {
        assert_eq!(
            RipplePhase::Animated.class_name(),
            "ui-ripple--state-animated"
        );
        assert_eq!(RipplePhase::Animated.as_str(), "animated");
        assert_eq!(RipplePhase::Static.class_name(), "ui-ripple--state-static");
        assert_eq!(RipplePhase::Static.as_str(), "static");
    }

    #[test]
    fn boundary_contract_is_stable() {
        assert_eq!(
            RippleBoundary::Bounded.class_name(),
            "ui-ripple--boundary-bounded"
        );
        assert_eq!(RippleBoundary::Bounded.as_str(), "bounded");
        assert!(RippleBoundary::Bounded.is_bounded());

        assert_eq!(
            RippleBoundary::Unbounded.class_name(),
            "ui-ripple--boundary-unbounded"
        );
        assert_eq!(RippleBoundary::Unbounded.as_str(), "unbounded");
        assert!(!RippleBoundary::Unbounded.is_bounded());
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-ripple-item  ".to_string())),
            Some("docs-ripple-item".to_string())
        );
    }

    #[test]
    fn resolve_helpers_keep_state_derivation_explicit() {
        assert_eq!(resolve_phase(true), RipplePhase::Animated);
        assert_eq!(resolve_phase(false), RipplePhase::Static);
        assert_eq!(resolve_boundary(true), RippleBoundary::Bounded);
        assert_eq!(resolve_boundary(false), RippleBoundary::Unbounded);
    }

    #[test]
    fn resolve_state_tracks_source_markers() {
        let state = resolve_state(RippleStateInput {
            phase: RipplePhase::Animated,
            boundary: RippleBoundary::Unbounded,
            has_custom_motion: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.phase_class, "ui-ripple--state-animated");
        assert_eq!(state.phase_attr, "animated");
        assert_eq!(state.boundary_class, "ui-ripple--boundary-unbounded");
        assert_eq!(state.boundary_attr, "unbounded");
        assert!(state.is_unbounded);
        assert!(!state.is_bounded);
        assert_eq!(state.motion_source_class, "ui-ripple--motion-custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(RippleStateInput {
            phase: RipplePhase::Static,
            boundary: RippleBoundary::Bounded,
            has_custom_motion: false,
            has_custom_class_name: true,
        });
        let class_name = compose_class_name(Some("docs-ripple-item".to_string()), state);

        for token in [
            "ui-ripple",
            "ui-ripple--state-static",
            "ui-ripple--boundary-bounded",
            "ui-ripple--motion-default",
            "ui-ripple--custom-class",
            "docs-ripple-item",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
