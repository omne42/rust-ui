#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum KbdSize {
    Sm,
    #[default]
    Md,
}

impl KbdSize {
    pub fn class_name(self) -> &'static str {
        match self {
            KbdSize::Sm => "ui-kbd--size-sm",
            KbdSize::Md => "ui-kbd--size-md",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            KbdSize::Sm => "sm",
            KbdSize::Md => "md",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KbdStateInput {
    pub size: KbdSize,
    pub has_keys: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KbdState {
    pub size: KbdSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub has_keys: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_state(input: KbdStateInput) -> KbdState {
    let (state_class, state_attr) = if input.has_keys {
        ("ui-kbd--state-with-keys", "with-keys")
    } else {
        ("ui-kbd--state-label-only", "label-only")
    };

    KbdState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_attr(),
        state_class,
        state_attr,
        has_keys: input.has_keys,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: KbdState) -> String {
    let mut classes = vec![
        "ui-kbd".to_string(),
        state.size_class.into(),
        state.state_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-kbd--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
