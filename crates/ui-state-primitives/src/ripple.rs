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
        (!trimmed.is_empty()).then(|| trimmed.into())
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
        state.phase_class.into(),
        state.boundary_class.into(),
        state.motion_source_class.into(),
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
#[path = "test/ripple.rs"]
mod tests;
