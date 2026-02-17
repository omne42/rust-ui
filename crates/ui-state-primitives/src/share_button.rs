pub use crate::button::normalize_optional_text;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SharePlatformLabels<'a> {
    pub github: &'a str,
    pub x: &'a str,
    pub facebook: &'a str,
}

impl<'a> SharePlatformLabels<'a> {
    pub fn for_platform(self, platform: SharePlatform) -> &'a str {
        match platform {
            SharePlatform::Github => self.github,
            SharePlatform::X => self.x,
            SharePlatform::Facebook => self.facebook,
        }
    }
}

impl Default for SharePlatformLabels<'static> {
    fn default() -> Self {
        Self {
            github: SharePlatform::Github.default_label(),
            x: SharePlatform::X.default_label(),
            facebook: SharePlatform::Facebook.default_label(),
        }
    }
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

pub fn resolve_label_with_fallback(label: Option<String>, fallback_label: &str) -> String {
    let trimmed_fallback = fallback_label.trim();
    if trimmed_fallback.is_empty() {
        label.unwrap_or_else(|| "Share".to_string())
    } else {
        label.unwrap_or_else(|| trimmed_fallback.to_string())
    }
}

pub fn resolve_label(label: Option<String>) -> String {
    resolve_label_with_fallback(label, "Share")
}

fn normalize_item_label(
    platform: SharePlatform,
    label: &str,
    labels: SharePlatformLabels<'_>,
) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        labels.for_platform(platform).trim().to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn resolve_items_with_fallback(
    items: &[ShareButtonItem],
    labels: SharePlatformLabels<'_>,
) -> ResolvedShareItems {
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
            normalize_item_label(item.platform, &item.label, labels),
        ));
    }

    if normalized.is_empty() {
        ResolvedShareItems {
            items: vec![
                ShareButtonItem::new(
                    SharePlatform::Github,
                    labels.for_platform(SharePlatform::Github),
                ),
                ShareButtonItem::new(SharePlatform::X, labels.for_platform(SharePlatform::X)),
                ShareButtonItem::new(
                    SharePlatform::Facebook,
                    labels.for_platform(SharePlatform::Facebook),
                ),
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

pub fn resolve_items(items: &[ShareButtonItem]) -> ResolvedShareItems {
    resolve_items_with_fallback(items, SharePlatformLabels::default())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_label_falls_back_to_default_label() {
        assert_eq!(resolve_label(Some("Share docs".to_string())), "Share docs");
        assert_eq!(resolve_label(None), "Share");
    }

    #[test]
    fn resolve_label_uses_custom_fallback_when_provided() {
        assert_eq!(
            resolve_label_with_fallback(None, "Partager"),
            "Partager".to_string()
        );
        assert_eq!(
            resolve_label_with_fallback(Some("Share docs".to_string()), "Partager"),
            "Share docs".to_string()
        );
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
    fn resolve_items_uses_label_fallback_overrides() {
        let labels = SharePlatformLabels {
            github: "Repositorio",
            x: "Publicar",
            facebook: "Facebook ES",
        };
        let resolved = resolve_items_with_fallback(&[], labels);

        assert!(resolved.uses_default_items);
        assert_eq!(resolved.items.len(), 3);
        assert_eq!(resolved.items[0].label, "Repositorio");
        assert_eq!(resolved.items[1].label, "Publicar");
        assert_eq!(resolved.items[2].label, "Facebook ES");
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
}
