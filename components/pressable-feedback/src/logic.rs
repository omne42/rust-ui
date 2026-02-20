use super::{PressableFeedbackState, PressableFeedbackStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Pressable feedback";

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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
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
#[path = "../test/logic.rs"]
mod tests;
