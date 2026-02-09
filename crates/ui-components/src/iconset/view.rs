use super::{IconsetSize, IconsetTone};
use crate::Icon;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IconsetGlyph {
    pub name: String,
    pub glyph: String,
    pub aria_label: Option<String>,
}

impl IconsetGlyph {
    pub fn new(name: impl Into<String>, glyph: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            glyph: glyph.into(),
            aria_label: None,
        }
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn parse_icon_reference(icon: &str) -> (Option<String>, String) {
    let trimmed = icon.trim();
    if let Some((iconset, icon_name)) = trimmed.split_once(':') {
        let iconset = normalize_optional_text(Some(iconset.to_string()));
        let icon_name = normalize_optional_text(Some(icon_name.to_string())).unwrap_or_default();
        return (iconset, icon_name);
    }

    (None, trimmed.to_string())
}

fn glyph_matches(candidate_name: &str, iconset: &str, icon_name: &str) -> bool {
    let Some(candidate_name) = normalize_optional_text(Some(candidate_name.to_string())) else {
        return false;
    };

    if candidate_name.eq_ignore_ascii_case(icon_name) {
        return true;
    }

    if let Some((candidate_set, candidate_icon_name)) = candidate_name.split_once(':') {
        return candidate_set.eq_ignore_ascii_case(iconset)
            && candidate_icon_name.eq_ignore_ascii_case(icon_name);
    }

    false
}

#[component]
pub fn Iconset(
    #[prop(into)] icon: String,
    #[prop(optional, into)] iconset: Option<String>,
    #[prop(optional)] glyphs: Vec<IconsetGlyph>,
    #[prop(optional)] size: IconsetSize,
    #[prop(optional)] tone: IconsetTone,
    #[prop(optional)] disabled: bool,
    #[prop(default = true)] decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (iconset_from_icon, mut icon_name) = parse_icon_reference(&icon);
    let iconset_from_prop = normalize_optional_text(iconset);

    let (resolved_iconset, iconset_source) = if let Some(iconset) = iconset_from_prop {
        (iconset, "prop")
    } else if let Some(iconset) = iconset_from_icon {
        (iconset, "icon")
    } else {
        ("custom-icons".to_string(), "default")
    };

    icon_name = normalize_optional_text(Some(icon_name)).unwrap_or_else(|| "unknown".to_string());

    let registry_match = glyphs
        .into_iter()
        .find(|glyph| glyph_matches(&glyph.name, &resolved_iconset, &icon_name));

    let (glyph_content, icon_source, registry_label) = if let Some(glyph) = registry_match {
        let glyph_content =
            normalize_optional_text(Some(glyph.glyph)).unwrap_or_else(|| "⬚".to_string());
        (
            glyph_content,
            "registry",
            normalize_optional_text(glyph.aria_label),
        )
    } else {
        ("⬚".to_string(), "fallback", None)
    };

    let computed_aria_label = if decorative {
        String::new()
    } else {
        normalize_optional_text(aria_label)
            .or(registry_label)
            .unwrap_or_else(|| icon_name.replace(['-', '_'], " "))
    };

    let class_name = normalize_optional_text(class_name)
        .map(|class_name| format!("ui-iconset {class_name}"))
        .unwrap_or_else(|| "ui-iconset".to_string());

    view! {
        <span
            class=class_name
            data-slot="iconset"
            data-iconset=resolved_iconset
            data-icon-name=icon_name
            data-icon-source=icon_source
            data-iconset-source=iconset_source
            data-disabled=disabled.then_some("true")
            data-decorative=decorative.then_some("true")
        >
            <Icon
                size=size
                tone=tone
                disabled=disabled
                decorative=decorative
                aria_label=computed_aria_label
                class_name="ui-iconset__icon".to_string()
            >
                {glyph_content}
            </Icon>
        </span>
    }
}
