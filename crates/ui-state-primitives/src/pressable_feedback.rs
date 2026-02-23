pub use crate::button::normalize_optional_text;

pub const DEFAULT_ARIA_LABEL: &str = "Pressable feedback";
pub const DEFAULT_IS_BOUNDED: bool = true;
pub const DEFAULT_IS_DISABLED: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PressableFeedbackTone {
    #[default]
    Default,
    Neutral,
    Accent,
}

impl PressableFeedbackTone {
    pub fn class_name(self) -> &'static str {
        match self {
            PressableFeedbackTone::Default => "ui-pressable-feedback--tone-default",
            PressableFeedbackTone::Neutral => "ui-pressable-feedback--tone-neutral",
            PressableFeedbackTone::Accent => "ui-pressable-feedback--tone-accent",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            PressableFeedbackTone::Default => "default",
            PressableFeedbackTone::Neutral => "neutral",
            PressableFeedbackTone::Accent => "accent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PressableFeedbackEffect {
    #[default]
    Scale,
    Highlight,
    Ripple,
    HighlightRipple,
}

impl PressableFeedbackEffect {
    pub fn class_name(self) -> &'static str {
        match self {
            PressableFeedbackEffect::Scale => "ui-pressable-feedback--effect-scale",
            PressableFeedbackEffect::Highlight => "ui-pressable-feedback--effect-highlight",
            PressableFeedbackEffect::Ripple => "ui-pressable-feedback--effect-ripple",
            PressableFeedbackEffect::HighlightRipple => {
                "ui-pressable-feedback--effect-highlight-ripple"
            }
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            PressableFeedbackEffect::Scale => "scale",
            PressableFeedbackEffect::Highlight => "highlight",
            PressableFeedbackEffect::Ripple => "ripple",
            PressableFeedbackEffect::HighlightRipple => "highlight-ripple",
        }
    }

    pub fn has_highlight(self) -> bool {
        matches!(
            self,
            PressableFeedbackEffect::Highlight | PressableFeedbackEffect::HighlightRipple
        )
    }

    pub fn has_ripple(self) -> bool {
        matches!(
            self,
            PressableFeedbackEffect::Ripple | PressableFeedbackEffect::HighlightRipple
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PressableFeedbackFlags {
    pub is_bounded: bool,
    pub is_disabled: bool,
}

pub fn normalize_flags(
    is_bounded: Option<bool>,
    is_disabled: Option<bool>,
) -> PressableFeedbackFlags {
    PressableFeedbackFlags {
        is_bounded: is_bounded.unwrap_or(DEFAULT_IS_BOUNDED),
        is_disabled: is_disabled.unwrap_or(DEFAULT_IS_DISABLED),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PressableFeedbackStateContractInput {
    pub effect: PressableFeedbackEffect,
    pub is_bounded: Option<bool>,
    pub is_disabled: Option<bool>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub has_custom_motion: bool,
    pub has_custom_press_handler: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PressableFeedbackStateContract {
    pub flags: PressableFeedbackFlags,
    pub aria_label: String,
    pub class_name: Option<String>,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_highlight: bool,
    pub has_ripple: bool,
    pub has_custom_motion: bool,
    pub has_custom_press_handler: bool,
}

pub fn normalize_state_contract(
    input: PressableFeedbackStateContractInput,
) -> PressableFeedbackStateContract {
    let flags = normalize_flags(input.is_bounded, input.is_disabled);
    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    PressableFeedbackStateContract {
        flags,
        aria_label,
        class_name,
        has_custom_aria_label,
        has_custom_class_name,
        has_highlight: input.effect.has_highlight(),
        has_ripple: input.effect.has_ripple(),
        has_custom_motion: input.has_custom_motion,
        has_custom_press_handler: input.has_custom_press_handler,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PressableFeedbackPressedMode {
    Controlled,
    Uncontrolled,
}

impl PressableFeedbackPressedMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }

    pub fn is_controlled(self) -> bool {
        matches!(self, Self::Controlled)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PressableFeedbackDefaultPressedSource {
    Provided,
    Default,
}

impl PressableFeedbackDefaultPressedSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Provided => "provided",
            Self::Default => "default",
        }
    }

    pub fn is_provided(self) -> bool {
        matches!(self, Self::Provided)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PressableFeedbackPressedChangeSource {
    Provided,
    None,
}

impl PressableFeedbackPressedChangeSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Provided => "provided",
            Self::None => "none",
        }
    }

    pub fn is_provided(self) -> bool {
        matches!(self, Self::Provided)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PressableFeedbackPressedAxisInput {
    pub has_controlled_value: bool,
    pub default_pressed: Option<bool>,
    pub has_on_pressed_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PressableFeedbackPressedAxisState {
    pub pressed_mode: PressableFeedbackPressedMode,
    pub default_pressed: bool,
    pub default_pressed_source: PressableFeedbackDefaultPressedSource,
    pub pressed_change_source: PressableFeedbackPressedChangeSource,
}

pub fn resolve_pressed_axis_state(
    input: PressableFeedbackPressedAxisInput,
) -> PressableFeedbackPressedAxisState {
    let pressed_mode = if input.has_controlled_value {
        PressableFeedbackPressedMode::Controlled
    } else {
        PressableFeedbackPressedMode::Uncontrolled
    };
    let default_pressed_source = if input.default_pressed.is_some() {
        PressableFeedbackDefaultPressedSource::Provided
    } else {
        PressableFeedbackDefaultPressedSource::Default
    };
    let pressed_change_source = if input.has_on_pressed_change {
        PressableFeedbackPressedChangeSource::Provided
    } else {
        PressableFeedbackPressedChangeSource::None
    };

    PressableFeedbackPressedAxisState {
        pressed_mode,
        default_pressed: input.default_pressed.unwrap_or(false),
        default_pressed_source,
        pressed_change_source,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PressableFeedbackStateInput {
    pub tone: PressableFeedbackTone,
    pub effect: PressableFeedbackEffect,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub bounded: bool,
    pub has_highlight: bool,
    pub has_ripple: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_press_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PressableFeedbackState {
    pub tone: PressableFeedbackTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub effect: PressableFeedbackEffect,
    pub effect_class: &'static str,
    pub effect_attr: &'static str,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_bounded: bool,
    pub is_unbounded: bool,
    pub boundary_class: &'static str,
    pub boundary_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub has_highlight: bool,
    pub has_ripple: bool,
    pub highlight_attr: &'static str,
    pub ripple_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: PressableFeedbackStateInput) -> PressableFeedbackState {
    let (boundary_class, boundary_attr) = if input.bounded {
        ("ui-pressable-feedback--boundary-bounded", "bounded")
    } else {
        ("ui-pressable-feedback--boundary-unbounded", "unbounded")
    };

    let (state_class, state_attr) = if input.is_disabled {
        ("ui-pressable-feedback--state-disabled", "disabled")
    } else if input.is_pressed {
        ("ui-pressable-feedback--state-pressed", "pressed")
    } else {
        ("ui-pressable-feedback--state-idle", "idle")
    };

    PressableFeedbackState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        effect: input.effect,
        effect_class: input.effect.class_name(),
        effect_attr: input.effect.as_attr(),
        is_disabled: input.is_disabled,
        is_pressed: input.is_pressed,
        is_bounded: input.bounded,
        is_unbounded: !input.bounded,
        boundary_class,
        boundary_attr,
        state_class,
        state_attr,
        has_highlight: input.has_highlight,
        has_ripple: input.has_ripple,
        highlight_attr: if input.has_highlight {
            "enabled"
        } else {
            "none"
        },
        ripple_attr: if input.has_ripple { "enabled" } else { "none" },
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
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
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: PressableFeedbackState,
) -> String {
    let mut classes = vec![
        "ui-pressable-feedback".to_string(),
        state.tone_class.into(),
        state.effect_class.into(),
        state.boundary_class.into(),
        state.state_class.into(),
    ];

    if state.has_highlight {
        classes.push("ui-pressable-feedback--highlight-enabled".to_string());
    }

    if state.has_ripple {
        classes.push("ui-pressable-feedback--ripple-enabled".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-pressable-feedback--has-handler".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-pressable-feedback--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/pressable_feedback.rs"]
mod tests;
