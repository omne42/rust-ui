pub const DEFAULT_ID_BASE: &str = "collapsible";
pub const DEFAULT_TITLE: &str = "Collapsible";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollapsibleStateInput {
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollapsibleState {
    pub is_open: bool,
    pub is_closed: bool,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub state_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return DEFAULT_ID_BASE.into();
    }

    let mut normalized = String::new();
    let mut previous_was_dash = false;

    for character in trimmed.chars() {
        let mapped = if character.is_ascii_alphanumeric() {
            character.to_ascii_lowercase()
        } else if character == '_' {
            '_'
        } else {
            '-'
        };

        if mapped == '-' {
            if previous_was_dash {
                continue;
            }
            previous_was_dash = true;
        } else {
            previous_was_dash = false;
        }

        normalized.push(mapped);
    }

    let normalized = normalized.trim_matches('-').trim_matches('_').to_string();

    if normalized.is_empty() {
        DEFAULT_ID_BASE.into()
    } else {
        normalized
    }
}

pub fn resolve_title(value: String) -> String {
    normalize_optional_text(Some(value)).unwrap_or_else(|| DEFAULT_TITLE.into())
}

pub fn resolve_aria_label(title: &str, value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (title.into(), false)
}

pub fn resolve_state(input: CollapsibleStateInput) -> CollapsibleState {
    CollapsibleState {
        is_open: input.is_open,
        is_closed: !input.is_open,
        is_disabled: input.is_disabled,
        is_controlled: input.is_controlled,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        state_attr: if input.is_disabled {
            "disabled"
        } else if input.is_open {
            "open"
        } else {
            "closed"
        },
        open_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        label_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "title"
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
#[path = "test/collapsible.rs"]
mod tests;
