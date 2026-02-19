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
mod tests {
    use super::*;

    #[test]
    fn icon_size_maps_from_text_button_sizes() {
        assert_eq!(resolve_icon_button_size(ButtonSize::Xs), ButtonSize::IconXs);
        assert_eq!(resolve_icon_button_size(ButtonSize::Sm), ButtonSize::IconS);
        assert_eq!(resolve_icon_button_size(ButtonSize::S), ButtonSize::IconS);
        assert_eq!(
            resolve_icon_button_size(ButtonSize::Default),
            ButtonSize::IconM
        );
        assert_eq!(resolve_icon_button_size(ButtonSize::M), ButtonSize::IconM);
        assert_eq!(resolve_icon_button_size(ButtonSize::Lg), ButtonSize::IconL);
        assert_eq!(resolve_icon_button_size(ButtonSize::L), ButtonSize::IconL);
        assert_eq!(resolve_icon_button_size(ButtonSize::Xl), ButtonSize::IconXl);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ShareButtonStateInput {
                provided_item_count: 2,
                resolved_item_count: 2,
                uses_default_items: false,
                icon_placement: ShareButtonIconPlacement::Suffix,
                has_custom_label: true,
                has_custom_class_name: true,
                has_custom_press_handler: false,
            }),
        );

        for token in [
            "ui-share-button",
            "ui-share-button--state-ready",
            "ui-share-button--custom-items",
            "ui-share-button--icon-suffix",
            "ui-share-button--custom-label",
            "ui-share-button--without-handler",
            "ui-share-button--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
