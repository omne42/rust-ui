use std::borrow::Cow;

#[cfg(test)]
pub use ui_state_primitives::overlays::DEFAULT_ID_BASE;
pub use ui_state_primitives::overlays::{
    OverlaysRootState, OverlaysRootStateInput, normalize_id_base, normalize_optional_text,
    resolve_root_state,
};

pub const DEFAULT_ARIA_LABEL: &str = "Overlays";

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn compose_root_class_name(
    base_class_name: Option<String>,
    state: OverlaysRootState,
) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-overlays"),
        Cow::Borrowed(state.layer_kind_class),
    ];

    if state.is_open {
        classes.push(Cow::Borrowed("ui-overlays--open"));
    } else {
        classes.push(Cow::Borrowed("ui-overlays--closed"));
    }

    if state.has_custom_id_base {
        classes.push(Cow::Borrowed("ui-overlays--custom-id"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-overlays--custom-class"));
        if let Some(base_class_name) = normalize_optional_text(base_class_name) {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
