pub const DEFAULT_ARIA_LABEL: &str = "Footer";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FooterTone {
    #[default]
    Default,
    Muted,
}

impl FooterTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FooterTone::Default => "ui-footer--tone-default",
            FooterTone::Muted => "ui-footer--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FooterTone::Default => "default",
            FooterTone::Muted => "muted",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FooterStateInput {
    pub tone: FooterTone,
    pub bordered: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FooterState {
    pub tone: FooterTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_bordered: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
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

pub fn resolve_state(input: FooterStateInput) -> FooterState {
    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let data_state_attr = if input.bordered && input.tone == FooterTone::Muted {
        "muted-bordered"
    } else if input.bordered {
        "bordered"
    } else if input.tone == FooterTone::Muted {
        "muted"
    } else {
        "default"
    };

    FooterState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_bordered: input.bordered,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FooterState) -> String {
    let mut classes = vec!["ui-footer".to_string(), state.tone_class.into()];

    if state.is_bordered {
        classes.push("ui-footer--bordered".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-footer--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/footer.rs"]
mod tests;
