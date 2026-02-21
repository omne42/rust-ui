use std::borrow::Cow;

pub use ui_state_primitives::kbd::{KbdSize, KbdState, KbdStateInput, resolve_state};

pub struct KbdLogicInput {
    pub size: Option<KbdSize>,
    pub keys: Option<String>,
    pub class_name: Option<String>,
}

pub struct KbdViewModel {
    pub keys: Option<String>,
    pub class: String,
    pub state: KbdState,
}

pub fn normalize_size(value: Option<KbdSize>) -> KbdSize {
    value.unwrap_or_default()
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn compose_class_name(base_class_name: Option<String>, state: KbdState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-kbd"),
        Cow::Borrowed(state.size_class),
        Cow::Borrowed(state.state_class),
    ];

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-kbd--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    // Pre-size the output buffer to avoid repeated allocations while joining.
    let total_len =
        classes.iter().map(|class| class.len()).sum::<usize>() + classes.len().saturating_sub(1);
    let mut out = String::with_capacity(total_len);
    for (index, class) in classes.iter().enumerate() {
        if index > 0 {
            out.push(' ');
        }
        out.push_str(class.as_ref());
    }

    out
}

pub fn resolve_view_model(input: KbdLogicInput) -> KbdViewModel {
    let size = normalize_size(input.size);
    let keys = normalize_optional_text(input.keys);
    let class_name = normalize_optional_text(input.class_name);

    let state = resolve_state(KbdStateInput {
        size,
        has_keys: keys.is_some(),
        has_custom_class_name: class_name.is_some(),
    });
    let class = compose_class_name(class_name, state);

    KbdViewModel { keys, class, state }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
