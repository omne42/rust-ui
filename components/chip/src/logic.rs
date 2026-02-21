use std::borrow::Cow;

pub use ui_state_primitives::chip::{
    ChipSize, ChipState, ChipStateInput, ChipVariant, normalize_optional_text,
    resolve_dismiss_aria_label, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: ChipState) -> String {
    let mut classes: Vec<Cow<'_, str>> = vec![
        Cow::Borrowed("ui-chip"),
        Cow::Borrowed(state.variant_class),
        Cow::Borrowed(state.size_class),
        Cow::Borrowed(state.state_class),
        Cow::Borrowed(state.dismiss_label_source_class),
    ];

    if state.is_enabled {
        classes.push(Cow::Borrowed("ui-chip--enabled"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-chip--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    let separator_count = classes.len().saturating_sub(1);
    let total_len: usize = classes
        .iter()
        .map(|class_name| class_name.len())
        .sum::<usize>()
        + separator_count;
    let mut out = String::with_capacity(total_len);

    for (index, class_name) in classes.iter().enumerate() {
        if index > 0 {
            out.push(' ');
        }
        out.push_str(class_name);
    }

    out
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
