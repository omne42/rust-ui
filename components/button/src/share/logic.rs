use crate::button::ButtonSize;

pub use ui_state_primitives::share_button::{
    ShareButtonIconPlacement, ShareButtonItem, ShareButtonState, ShareButtonStateInput,
    SharePlatform, SharePlatformLabels, normalize_optional_text, resolve_items_with_fallback,
    resolve_label_with_fallback, resolve_state,
};

pub fn resolve_icon_button_size(size: ButtonSize) -> ButtonSize {
    match size {
        ButtonSize::Xs | ButtonSize::IconXs => ButtonSize::IconXs,
        ButtonSize::Sm | ButtonSize::S | ButtonSize::IconSm | ButtonSize::IconS => {
            ButtonSize::IconS
        }
        ButtonSize::Lg | ButtonSize::L | ButtonSize::IconLg | ButtonSize::IconL => {
            ButtonSize::IconL
        }
        ButtonSize::Xl | ButtonSize::IconXl => ButtonSize::IconXl,
        ButtonSize::Default | ButtonSize::M | ButtonSize::Icon | ButtonSize::IconM => {
            ButtonSize::IconM
        }
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ShareButtonState) -> String {
    let mut classes = vec![
        "ui-share-button".to_string(),
        state.state_class.into(),
        state.items_source_class.into(),
        state.icon_placement_class.into(),
        state.label_source_class.into(),
        state.handler_source_class.into(),
    ];

    if state.has_items {
        classes.push("ui-share-button--has-items".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-share-button--custom-class".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/share/logic.rs"]
mod tests;
