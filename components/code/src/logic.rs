use std::borrow::Cow;

pub use ui_state_primitives::code::{
    CodeState, CodeStateInput, CodeVariant, normalize_optional_text, resolve_state,
};

pub struct CodeViewInput {
    pub variant: Option<CodeVariant>,
    pub class_name: Option<String>,
}

pub struct CodeViewState {
    pub state: CodeState,
    pub class: String,
}

pub fn compose_class_name(base_class_name: Option<String>, state: CodeState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-code"),
        Cow::Borrowed(state.variant_class),
        Cow::Borrowed(state.state_class),
    ];

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-code--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn resolve_view_state(input: CodeViewInput) -> CodeViewState {
    let class_name = normalize_optional_text(input.class_name);
    let state = resolve_state(CodeStateInput {
        variant: input.variant.unwrap_or_default(),
        has_custom_class_name: class_name.is_some(),
    });

    let class = compose_class_name(class_name, state);
    CodeViewState { state, class }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
