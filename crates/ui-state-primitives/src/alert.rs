#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertStateCoreInput {
    pub has_title: bool,
    pub has_description: bool,
    pub has_actions: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertStateCore {
    pub has_title: bool,
    pub title_attr: &'static str,
    pub has_description: bool,
    pub description_attr: &'static str,
    pub has_actions: bool,
    pub actions_attr: &'static str,
    pub state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_state_core(input: AlertStateCoreInput) -> AlertStateCore {
    let title_attr = if input.has_title { "present" } else { "absent" };
    let description_attr = if input.has_description {
        "present"
    } else {
        "absent"
    };
    let actions_attr = if input.has_actions {
        "present"
    } else {
        "absent"
    };
    let state_attr = if input.has_title && input.has_description {
        "detailed"
    } else {
        "compact"
    };
    AlertStateCore {
        has_title: input.has_title,
        title_attr,
        has_description: input.has_description,
        description_attr,
        has_actions: input.has_actions,
        actions_attr,
        state_attr,
    }
}

#[cfg(test)]
#[path = "test/alert.rs"]
mod tests;
