use crate::button::ButtonSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SharePlatform {
    Github,
    X,
    Facebook,
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

pub fn resolve_items(items: &[ShareButtonItem]) -> Vec<ShareButtonItem> {
    if !items.is_empty() {
        return items.to_vec();
    }
    vec![
        ShareButtonItem::new(SharePlatform::Github, "GitHub"),
        ShareButtonItem::new(SharePlatform::X, "X"),
        ShareButtonItem::new(SharePlatform::Facebook, "Facebook"),
    ]
}

pub fn resolve_icon_button_size(size: ButtonSize) -> ButtonSize {
    match size {
        ButtonSize::Sm | ButtonSize::IconSm => ButtonSize::IconSm,
        ButtonSize::Lg | ButtonSize::IconLg => ButtonSize::IconLg,
        _ => ButtonSize::Icon,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShareButtonIconPlacement {
    Prefix,
    #[default]
    Suffix,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_three_platforms() {
        let items = resolve_items(&[]);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].label, "GitHub");
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
}
