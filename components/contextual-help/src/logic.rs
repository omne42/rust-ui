use ui_headless::PopoverPlacement;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContextualHelpVariant {
    #[default]
    Help,
    Info,
}

impl ContextualHelpVariant {
    pub fn default_label(self) -> &'static str {
        match self {
            ContextualHelpVariant::Help => "Help",
            ContextualHelpVariant::Info => "Info",
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            ContextualHelpVariant::Help => "ui-contextual-help--variant-help",
            ContextualHelpVariant::Info => "ui-contextual-help--variant-info",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ContextualHelpVariant::Help => "help",
            ContextualHelpVariant::Info => "info",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpStateInput {
    pub variant: ContextualHelpVariant,
    pub placement: PopoverPlacement,
    pub disabled: bool,
    pub has_heading: bool,
    pub has_footer: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_id: bool,
    pub has_custom_motion: bool,
    pub is_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpState {
    pub variant: ContextualHelpVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub placement: PopoverPlacement,
    pub placement_class: &'static str,
    pub placement_attr: &'static str,
    pub is_disabled: bool,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub has_heading: bool,
    pub heading_class: &'static str,
    pub heading_attr: &'static str,
    pub has_footer: bool,
    pub footer_class: &'static str,
    pub footer_attr: &'static str,
    pub has_custom_aria_label: bool,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_id: bool,
    pub id_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_motion: bool,
    pub is_controlled: bool,
    pub open_mode_class: &'static str,
    pub open_mode_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_trigger_aria_label(
    variant: ContextualHelpVariant,
    aria_label: Option<String>,
) -> (String, bool) {
    if let Some(label) = normalize_optional_text(aria_label) {
        (label, true)
    } else {
        (variant.default_label().to_string(), false)
    }
}

pub fn resolve_id(id: Option<String>, fallback: String) -> (String, bool) {
    if let Some(id) = normalize_optional_text(id) {
        (id, true)
    } else {
        (fallback, false)
    }
}

fn placement_class(placement: PopoverPlacement) -> &'static str {
    match placement {
        PopoverPlacement::BottomStart => "ui-contextual-help--placement-bottom-start",
        PopoverPlacement::BottomEnd => "ui-contextual-help--placement-bottom-end",
        PopoverPlacement::TopStart => "ui-contextual-help--placement-top-start",
        PopoverPlacement::TopEnd => "ui-contextual-help--placement-top-end",
    }
}

pub fn resolve_state(input: ContextualHelpStateInput) -> ContextualHelpState {
    ContextualHelpState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        placement: input.placement,
        placement_class: placement_class(input.placement),
        placement_attr: input.placement.as_str(),
        is_disabled: input.disabled,
        state_class: if input.disabled {
            "ui-contextual-help--disabled"
        } else {
            "ui-contextual-help--enabled"
        },
        state_attr: if input.disabled {
            "disabled"
        } else {
            "enabled"
        },
        has_heading: input.has_heading,
        heading_class: if input.has_heading {
            "ui-contextual-help--with-heading"
        } else {
            "ui-contextual-help--no-heading"
        },
        heading_attr: if input.has_heading {
            "present"
        } else {
            "absent"
        },
        has_footer: input.has_footer,
        footer_class: if input.has_footer {
            "ui-contextual-help--with-footer"
        } else {
            "ui-contextual-help--no-footer"
        },
        footer_attr: if input.has_footer {
            "present"
        } else {
            "absent"
        },
        has_custom_aria_label: input.has_custom_aria_label,
        label_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
        has_custom_id: input.has_custom_id,
        id_source_attr: if input.has_custom_id {
            "custom"
        } else {
            "auto"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        has_custom_motion: input.has_custom_motion,
        is_controlled: input.is_controlled,
        open_mode_class: if input.is_controlled {
            "ui-contextual-help--controlled"
        } else {
            "ui-contextual-help--uncontrolled"
        },
        open_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ContextualHelpState) -> String {
    let mut classes = vec![
        "ui-contextual-help".to_string(),
        state.variant_class.into(),
        state.placement_class.into(),
        state.state_class.into(),
        state.heading_class.into(),
        state.footer_class.into(),
        state.open_mode_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-contextual-help--custom-class".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-contextual-help--custom-motion".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
