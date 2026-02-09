use crate::pressable_feedback::{PressableFeedbackState, PressableFeedbackStateInput};

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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
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
        state.tone_class.to_string(),
        state.effect_class.to_string(),
        state.boundary_class.to_string(),
        state.state_class.to_string(),
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
mod tests {
    use super::*;
    use crate::pressable_feedback::PressableFeedbackStateInput;

    #[test]
    fn tone_and_effect_contracts_are_stable() {
        assert_eq!(
            PressableFeedbackTone::Default.class_name(),
            "ui-pressable-feedback--tone-default"
        );
        assert_eq!(
            PressableFeedbackTone::Neutral.class_name(),
            "ui-pressable-feedback--tone-neutral"
        );
        assert_eq!(
            PressableFeedbackTone::Accent.class_name(),
            "ui-pressable-feedback--tone-accent"
        );
        assert_eq!(PressableFeedbackTone::Default.as_attr(), "default");

        assert_eq!(
            PressableFeedbackEffect::Scale.class_name(),
            "ui-pressable-feedback--effect-scale"
        );
        assert_eq!(
            PressableFeedbackEffect::Highlight.class_name(),
            "ui-pressable-feedback--effect-highlight"
        );
        assert_eq!(
            PressableFeedbackEffect::Ripple.class_name(),
            "ui-pressable-feedback--effect-ripple"
        );
        assert_eq!(
            PressableFeedbackEffect::HighlightRipple.class_name(),
            "ui-pressable-feedback--effect-highlight-ripple"
        );

        assert!(PressableFeedbackEffect::Highlight.has_highlight());
        assert!(!PressableFeedbackEffect::Highlight.has_ripple());
        assert!(PressableFeedbackEffect::Ripple.has_ripple());
        assert!(!PressableFeedbackEffect::Ripple.has_highlight());
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-pressable-feedback  ".to_string())),
            Some("docs-pressable-feedback".to_string())
        );

        let (label, custom_label) = normalize_aria_label(Some("  Press card  ".to_string()));
        assert_eq!(label, "Press card");
        assert!(custom_label);

        let (label, custom_label) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom_label);
    }

    #[test]
    fn resolve_state_tracks_sources_and_visibility_flags() {
        let state = resolve_state(PressableFeedbackStateInput {
            tone: PressableFeedbackTone::Accent,
            effect: PressableFeedbackEffect::HighlightRipple,
            is_disabled: false,
            is_pressed: true,
            bounded: false,
            has_highlight: true,
            has_ripple: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
            has_custom_motion: true,
            has_custom_press_handler: true,
        });

        assert_eq!(state.tone_attr, "accent");
        assert_eq!(state.effect_attr, "highlight-ripple");
        assert_eq!(state.state_attr, "pressed");
        assert_eq!(state.boundary_attr, "unbounded");
        assert_eq!(state.highlight_attr, "enabled");
        assert_eq!(state.ripple_attr, "enabled");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.motion_source_attr, "custom");
        assert!(state.has_custom_press_handler);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-pressable-feedback".to_string()),
            resolve_state(PressableFeedbackStateInput {
                tone: PressableFeedbackTone::Neutral,
                effect: PressableFeedbackEffect::Highlight,
                is_disabled: false,
                is_pressed: false,
                bounded: true,
                has_highlight: true,
                has_ripple: false,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_motion: false,
                has_custom_press_handler: false,
            }),
        );

        for token in [
            "ui-pressable-feedback",
            "ui-pressable-feedback--tone-neutral",
            "ui-pressable-feedback--effect-highlight",
            "ui-pressable-feedback--boundary-bounded",
            "ui-pressable-feedback--state-idle",
            "ui-pressable-feedback--highlight-enabled",
            "ui-pressable-feedback--custom-class",
            "docs-pressable-feedback",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
