use super::{IconsUiSize, IconsUiTone, IconsetGlyph};
use crate::Iconset;
use leptos::prelude::*;

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn normalize_icon_reference(icon: String) -> String {
    let icon = icon.trim();
    if icon.is_empty() {
        return "ui:help".to_string();
    }

    if icon.contains(':') {
        return icon.to_string();
    }

    format!("ui:{icon}")
}

fn default_ui_glyphs() -> Vec<IconsetGlyph> {
    vec![
        IconsetGlyph::new("ui:help", "?").with_aria_label("UI Help"),
        IconsetGlyph::new("ui:check", "✓").with_aria_label("UI Check"),
        IconsetGlyph::new("ui:close", "✕").with_aria_label("UI Close"),
        IconsetGlyph::new("ui:chevron-right", "›").with_aria_label("UI Chevron Right"),
        IconsetGlyph::new("ui:chevron-left", "‹").with_aria_label("UI Chevron Left"),
    ]
}

#[component]
pub fn IconsUi(
    #[prop(into)] icon: String,
    #[prop(optional)] size: IconsUiSize,
    #[prop(optional)] tone: IconsUiTone,
    #[prop(optional)] disabled: bool,
    #[prop(default = true)] decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] glyphs: Vec<IconsetGlyph>,
) -> impl IntoView {
    let icon = normalize_icon_reference(icon);
    let icon_reference = icon.clone();

    let mut registry = default_ui_glyphs();
    registry.extend(glyphs);

    let aria_label = normalize_optional_text(aria_label).unwrap_or_default();

    let class_name = normalize_optional_text(class_name)
        .map(|class_name| format!("ui-icons-ui {class_name}"))
        .unwrap_or_else(|| "ui-icons-ui".to_string());

    view! {
        <span data-slot="icons-ui" data-icon-reference=icon_reference data-disabled=disabled.then_some("true")>
            <Iconset
                icon=icon
                iconset="ui".to_string()
                glyphs=registry
                size=size
                tone=tone
                disabled=disabled
                decorative=decorative
                aria_label=aria_label
                class_name=class_name
            />
        </span>
    }
}
