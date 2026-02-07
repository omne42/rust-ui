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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShareButtonState {
    pub item_count: usize,
    pub has_items: bool,
    pub is_empty: bool,
    pub uses_default_items: bool,
    pub icon_placement: ShareButtonIconPlacement,
    pub icon_placement_attr: &'static str,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
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
        ButtonSize::Sm | ButtonSize::IconSm => ButtonSize::IconSm,
        ButtonSize::Lg | ButtonSize::IconLg => ButtonSize::IconLg,
        _ => ButtonSize::Icon,
    }
}

pub fn resolve_state(
    item_count: usize,
    uses_default_items: bool,
    icon_placement: ShareButtonIconPlacement,
    has_custom_label: bool,
    has_custom_class_name: bool,
    has_custom_press_handler: bool,
) -> ShareButtonState {
    ShareButtonState {
        item_count,
        has_items: item_count > 0,
        is_empty: item_count == 0,
        uses_default_items,
        icon_placement,
        icon_placement_attr: icon_placement.as_attr(),
        has_custom_label,
        has_custom_class_name,
        has_custom_press_handler,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ShareButtonState) -> String {
    let mut classes = vec![
        "ui-share-button".to_string(),
        format!("ui-share-button--icon-{}", state.icon_placement_attr),
    ];

    if state.has_items {
        classes.push("ui-share-button--has-items".to_string());
    }
    if state.is_empty {
        classes.push("ui-share-button--empty".to_string());
    }
    if state.uses_default_items {
        classes.push("ui-share-button--default-items".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-share-button--custom-label".to_string());
    }
    if state.has_custom_press_handler {
        classes.push("ui-share-button--with-handler".to_string());
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
        assert_eq!(resolve_icon_button_size(ButtonSize::Sm), ButtonSize::IconSm);
        assert_eq!(
            resolve_icon_button_size(ButtonSize::Default),
            ButtonSize::Icon
        );
        assert_eq!(resolve_icon_button_size(ButtonSize::Lg), ButtonSize::IconLg);
    }

    #[test]
    fn resolve_state_tracks_items_icon_placement_and_metadata() {
        let state = resolve_state(3, true, ShareButtonIconPlacement::Prefix, true, true, true);
        assert_eq!(state.item_count, 3);
        assert!(state.has_items);
        assert!(!state.is_empty);
        assert!(state.uses_default_items);
        assert_eq!(state.icon_placement_attr, "prefix");
        assert!(state.has_custom_label);
        assert!(state.has_custom_class_name);
        assert!(state.has_custom_press_handler);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(2, false, ShareButtonIconPlacement::Suffix, true, true, true),
        );

        for token in [
            "ui-share-button",
            "ui-share-button--icon-suffix",
            "ui-share-button--has-items",
            "ui-share-button--custom-label",
            "ui-share-button--with-handler",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
