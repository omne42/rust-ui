use std::borrow::Cow;

pub use ui_state_primitives::keyboard::{
    DEFAULT_ARIA_LABEL, KeyboardState, KeyboardStateInput, KeyboardTone, normalize_aria_label,
    normalize_optional_text, resolve_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardRootInput {
    pub tone: Option<KeyboardTone>,
    pub is_compact: Option<bool>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub lang: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardRootState {
    pub state: KeyboardState,
    pub class_name: String,
    pub aria_label: String,
    pub lang: Option<String>,
}

pub fn compose_class_name(base_class_name: Option<String>, state: KeyboardState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-keyboard"),
        Cow::Borrowed(state.tone_class),
    ];

    if state.is_compact {
        classes.push(Cow::Borrowed("ui-keyboard--compact"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-keyboard--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .into_iter()
        .map(Cow::into_owned)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn normalize_root_state(input: KeyboardRootInput) -> KeyboardRootState {
    let tone = input.tone.unwrap_or_default();
    let is_compact = input.is_compact.unwrap_or(false);
    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();
    let lang = normalize_optional_text(input.lang);

    let state = resolve_state(KeyboardStateInput {
        tone,
        compact: is_compact,
        has_custom_aria_label,
        has_custom_class_name,
    });

    KeyboardRootState {
        state,
        class_name: compose_class_name(class_name, state),
        aria_label,
        lang,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
