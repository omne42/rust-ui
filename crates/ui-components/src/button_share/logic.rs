use crate::button::ButtonSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SharePlatform {
    Github,
    X,
    Facebook,
}

impl SharePlatform {
    pub fn default_label(self) -> &'static str {
        match self {
            SharePlatform::Github => "GitHub",
            SharePlatform::X => "X",
            SharePlatform::Facebook => "Facebook",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SharePlatform::Github => "github",
            SharePlatform::X => "x",
            SharePlatform::Facebook => "facebook",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShareButtonItem {
    pub platform: SharePlatform,
    pub label: String,
}

impl ShareButtonItem {
    pub fn new(platform: SharePlatform, label: impl Into<String>) -> Self {
        Self {
            platform,
            label: label.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedShareItems {
    pub items: Vec<ShareButtonItem>,
    pub uses_default_items: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShareButtonIconPlacement {
    Prefix,
    #[default]
    Suffix,
    None,
}

impl ShareButtonIconPlacement {
    pub fn as_attr(self) -> &'static str {
        match self {
            ShareButtonIconPlacement::Prefix => "prefix",
            ShareButtonIconPlacement::Suffix => "suffix",
            ShareButtonIconPlacement::None => "none",
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            ShareButtonIconPlacement::Prefix => "ui-share-button--icon-prefix",
            ShareButtonIconPlacement::Suffix => "ui-share-button--icon-suffix",
            ShareButtonIconPlacement::None => "ui-share-button--icon-none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShareButtonStateInput {
    pub provided_item_count: usize,
    pub resolved_item_count: usize,
    pub uses_default_items: bool,
    pub icon_placement: ShareButtonIconPlacement,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShareButtonState {
    pub provided_item_count: usize,
    pub resolved_item_count: usize,
    pub has_items: bool,
    pub state_attr: &'static str,
    pub state_class: &'static str,
    pub uses_default_items: bool,
    pub items_source_attr: &'static str,
    pub items_source_class: &'static str,
    pub icon_placement: ShareButtonIconPlacement,
    pub icon_placement_attr: &'static str,
    pub icon_placement_class: &'static str,
    pub has_custom_label: bool,
    pub label_source_attr: &'static str,
    pub label_source_class: &'static str,
    pub has_custom_press_handler: bool,
    pub handler_source_attr: &'static str,
    pub handler_source_class: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn normalize_item_label(platform: SharePlatform, label: &str) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        platform.default_label().to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn resolve_items(items: &[ShareButtonItem]) -> ResolvedShareItems {
    let mut normalized = Vec::new();

    for item in items {
        if normalized
            .iter()
            .any(|existing: &ShareButtonItem| existing.platform == item.platform)
        {
            continue;
        }

        normalized.push(ShareButtonItem::new(
            item.platform,
            normalize_item_label(item.platform, &item.label),
        ));
    }

    if normalized.is_empty() {
        ResolvedShareItems {
            items: vec![
                ShareButtonItem::new(SharePlatform::Github, "GitHub"),
                ShareButtonItem::new(SharePlatform::X, "X"),
                ShareButtonItem::new(SharePlatform::Facebook, "Facebook"),
            ],
            uses_default_items: true,
        }
    } else {
        ResolvedShareItems {
            items: normalized,
            uses_default_items: false,
        }
    }
}

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

pub fn resolve_state(input: ShareButtonStateInput) -> ShareButtonState {
    let has_items = input.resolved_item_count > 0;
    let (state_attr, state_class) = if has_items {
        ("ready", "ui-share-button--state-ready")
    } else {
        ("empty", "ui-share-button--state-empty")
    };

    let (items_source_attr, items_source_class) = if input.uses_default_items {
        ("default", "ui-share-button--default-items")
    } else {
        ("custom", "ui-share-button--custom-items")
    };

    let (label_source_attr, label_source_class) = if input.has_custom_label {
        ("custom", "ui-share-button--custom-label")
    } else {
        ("default", "ui-share-button--default-label")
    };

    let (handler_source_attr, handler_source_class) = if input.has_custom_press_handler {
        ("provided", "ui-share-button--with-handler")
    } else {
        ("none", "ui-share-button--without-handler")
    };

    ShareButtonState {
        provided_item_count: input.provided_item_count,
        resolved_item_count: input.resolved_item_count,
        has_items,
        state_attr,
        state_class,
        uses_default_items: input.uses_default_items,
        items_source_attr,
        items_source_class,
        icon_placement: input.icon_placement,
        icon_placement_attr: input.icon_placement.as_attr(),
        icon_placement_class: input.icon_placement.class_name(),
        has_custom_label: input.has_custom_label,
        label_source_attr,
        label_source_class,
        has_custom_press_handler: input.has_custom_press_handler,
        handler_source_attr,
        handler_source_class,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ShareButtonState) -> String {
    let mut classes = vec![
        "ui-share-button".to_string(),
        state.state_class.to_string(),
        state.items_source_class.to_string(),
        state.icon_placement_class.to_string(),
        state.label_source_class.to_string(),
        state.handler_source_class.to_string(),
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
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(
            normalize_optional_text(Some("  Share now  ".to_string())),
            Some("Share now".to_string())
        );
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn resolve_items_defaults_to_three_platforms() {
        let resolved = resolve_items(&[]);
        assert!(resolved.uses_default_items);
        assert_eq!(resolved.items.len(), 3);
        assert_eq!(resolved.items[0].label, "GitHub");
    }

    #[test]
    fn resolve_items_deduplicates_platforms_and_normalizes_labels() {
        let resolved = resolve_items(&[
            ShareButtonItem::new(SharePlatform::Github, "  Repo  "),
            ShareButtonItem::new(SharePlatform::Github, "Ignore me"),
            ShareButtonItem::new(SharePlatform::X, "   "),
        ]);

        assert!(!resolved.uses_default_items);
        assert_eq!(resolved.items.len(), 2);
        assert_eq!(resolved.items[0].label, "Repo");
        assert_eq!(resolved.items[1].label, "X");
    }

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
    fn resolve_state_tracks_items_icon_placement_and_metadata() {
        let state = resolve_state(ShareButtonStateInput {
            provided_item_count: 0,
            resolved_item_count: 3,
            uses_default_items: true,
            icon_placement: ShareButtonIconPlacement::Prefix,
            has_custom_label: true,
            has_custom_class_name: true,
            has_custom_press_handler: true,
        });

        assert_eq!(state.provided_item_count, 0);
        assert_eq!(state.resolved_item_count, 3);
        assert!(state.has_items);
        assert_eq!(state.state_attr, "ready");
        assert!(state.uses_default_items);
        assert_eq!(state.items_source_attr, "default");
        assert_eq!(state.icon_placement_attr, "prefix");
        assert_eq!(state.icon_placement_class, "ui-share-button--icon-prefix");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.handler_source_attr, "provided");
        assert!(state.has_custom_class_name);
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
