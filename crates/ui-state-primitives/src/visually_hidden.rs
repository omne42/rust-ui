#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisuallyHiddenStateInput {
    pub is_focusable: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisuallyHiddenState {
    pub is_focusable: bool,
    pub has_custom_class_name: bool,
    pub focusable_class: Option<&'static str>,
    pub focusable_attr: Option<&'static str>,
    pub custom_class_attr: Option<&'static str>,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_state(input: VisuallyHiddenStateInput) -> VisuallyHiddenState {
    VisuallyHiddenState {
        is_focusable: input.is_focusable,
        has_custom_class_name: input.has_custom_class_name,
        focusable_class: input
            .is_focusable
            .then_some("ui-visually-hidden--focusable"),
        focusable_attr: input.is_focusable.then_some("true"),
        custom_class_attr: input.has_custom_class_name.then_some("true"),
    }
}

#[cfg(test)]
#[path = "test/visually_hidden.rs"]
mod tests;
