pub use ui_headless::A11yDirection;
use ui_headless::{A11yLocaleAttrs, locale_attrs};

pub fn resolve_locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs {
    locale_attrs(lang, dir)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemStateSource {
    Default,
    Prop,
}

impl ItemStateSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ItemStateSource::Default => "default",
            ItemStateSource::Prop => "prop",
        }
    }
}

pub fn source_from_optional<T>(value: Option<T>) -> ItemStateSource {
    if value.is_some() {
        return ItemStateSource::Prop;
    }

    ItemStateSource::Default
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            ItemVariant::Default => "default",
            ItemVariant::Outline => "outline",
            ItemVariant::Muted => "muted",
        }
    }
}

pub fn normalize_item_variant(variant: Option<ItemVariant>) -> ItemVariant {
    variant.unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemRenderState {
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub variant_source_attr: &'static str,
    pub size_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

impl ItemSize {
    pub fn as_attr(self) -> &'static str {
        match self {
            ItemSize::Default => "default",
            ItemSize::Sm => "sm",
        }
    }
}

pub fn normalize_item_size(size: Option<ItemSize>) -> ItemSize {
    size.unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            ItemMediaVariant::Default => "default",
            ItemMediaVariant::Icon => "icon",
            ItemMediaVariant::Image => "image",
        }
    }
}

pub fn normalize_item_media_variant(variant: Option<ItemMediaVariant>) -> ItemMediaVariant {
    variant.unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemMediaRenderState {
    pub variant_attr: &'static str,
    pub variant_source_attr: &'static str,
}

pub fn derive_item_render_state(
    variant: Option<ItemVariant>,
    size: Option<ItemSize>,
) -> ItemRenderState {
    let variant_source = source_from_optional(variant);
    let size_source = source_from_optional(size);
    let variant = normalize_item_variant(variant);
    let size = normalize_item_size(size);

    ItemRenderState {
        variant_attr: variant.as_attr(),
        size_attr: size.as_attr(),
        variant_source_attr: variant_source.as_attr(),
        size_source_attr: size_source.as_attr(),
    }
}

pub fn derive_item_media_render_state(variant: Option<ItemMediaVariant>) -> ItemMediaRenderState {
    let variant_source = source_from_optional(variant);
    let variant = normalize_item_media_variant(variant);

    ItemMediaRenderState {
        variant_attr: variant.as_attr(),
        variant_source_attr: variant_source.as_attr(),
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn compose_class(base: &'static str, class_name: Option<String>) -> String {
    normalize_optional_text(class_name)
        .map(|class_name| format!("{base} {class_name}"))
        .unwrap_or_else(|| base.into())
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
