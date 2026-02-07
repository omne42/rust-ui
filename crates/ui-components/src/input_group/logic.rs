pub const DEFAULT_ARIA_LABEL: &str = "Input group";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputGroupPhase {
    Enabled,
    Disabled,
}

impl InputGroupPhase {
    pub fn class_name(self) -> &'static str {
        match self {
            InputGroupPhase::Enabled => "ui-input-group--state-enabled",
            InputGroupPhase::Disabled => "ui-input-group--state-disabled",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            InputGroupPhase::Enabled => "enabled",
            InputGroupPhase::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputGroupAttachment {
    Attached,
    Detached,
}

impl InputGroupAttachment {
    pub fn class_name(self) -> &'static str {
        match self {
            InputGroupAttachment::Attached => "ui-input-group--attached",
            InputGroupAttachment::Detached => "ui-input-group--detached",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            InputGroupAttachment::Attached => "attached",
            InputGroupAttachment::Detached => "detached",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputGroupStateInput {
    pub disabled: bool,
    pub invalid: bool,
    pub attached: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputGroupState {
    pub phase: InputGroupPhase,
    pub attachment: InputGroupAttachment,
    pub phase_class: &'static str,
    pub attachment_class: &'static str,
    pub phase_attr: &'static str,
    pub attachment_attr: &'static str,
    pub is_enabled: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_attached: bool,
    pub is_detached: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    let Some(value) = value else {
        return (DEFAULT_ARIA_LABEL.to_string(), false);
    };

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return (DEFAULT_ARIA_LABEL.to_string(), false);
    }

    let label = trimmed.to_string();
    let has_custom_label = label != DEFAULT_ARIA_LABEL;

    (label, has_custom_label)
}

pub fn resolve_state(input: InputGroupStateInput) -> InputGroupState {
    let phase = if input.disabled {
        InputGroupPhase::Disabled
    } else {
        InputGroupPhase::Enabled
    };

    let attachment = if input.attached {
        InputGroupAttachment::Attached
    } else {
        InputGroupAttachment::Detached
    };

    let (label_source_class, label_source_attr) = if input.has_custom_label {
        ("ui-input-group--label-custom", "custom")
    } else {
        ("ui-input-group--label-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    InputGroupState {
        phase,
        attachment,
        phase_class: phase.class_name(),
        attachment_class: attachment.class_name(),
        phase_attr: phase.as_str(),
        attachment_attr: attachment.as_str(),
        is_enabled: !input.disabled,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        is_attached: input.attached,
        is_detached: !input.attached,
        has_start_content: input.has_start_content,
        has_end_content: input.has_end_content,
        has_custom_label: input.has_custom_label,
        has_custom_class_name: input.has_custom_class_name,
        label_source_class,
        label_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: InputGroupState) -> String {
    let mut classes = vec![
        "ui-input-group".to_string(),
        state.phase_class.to_string(),
        state.attachment_class.to_string(),
        state.label_source_class.to_string(),
    ];

    if state.is_invalid {
        classes.push("ui-input-group--invalid".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-input-group--custom-class".to_string());
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
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-input-group ".to_string())),
            Some("docs-input-group".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_trimmed_label_or_fallback() {
        let (label, explicit) = normalize_aria_label(Some("  Query controls  ".to_string()));
        assert_eq!(label, "Query controls");
        assert!(explicit);

        let (label, explicit) = normalize_aria_label(Some("   ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!explicit);

        let (label, explicit) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!explicit);
    }

    #[test]
    fn resolve_state_tracks_phase_attachment_and_source_markers() {
        let state = resolve_state(InputGroupStateInput {
            disabled: true,
            invalid: true,
            attached: false,
            has_start_content: true,
            has_end_content: false,
            has_custom_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.phase_class, "ui-input-group--state-disabled");
        assert_eq!(state.phase_attr, "disabled");
        assert!(state.is_disabled);
        assert!(!state.is_enabled);

        assert_eq!(state.attachment_class, "ui-input-group--detached");
        assert_eq!(state.attachment_attr, "detached");
        assert!(state.is_detached);
        assert!(!state.is_attached);

        assert!(state.is_invalid);
        assert!(state.has_start_content);
        assert!(!state.has_end_content);
        assert_eq!(state.label_source_class, "ui-input-group--label-custom");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(InputGroupStateInput {
            disabled: false,
            invalid: true,
            attached: true,
            has_start_content: true,
            has_end_content: true,
            has_custom_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-input-group".to_string()), state);

        for token in [
            "ui-input-group",
            "ui-input-group--state-enabled",
            "ui-input-group--attached",
            "ui-input-group--label-default",
            "ui-input-group--invalid",
            "ui-input-group--custom-class",
            "docs-input-group",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
