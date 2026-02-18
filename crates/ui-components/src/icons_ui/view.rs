use super::{IconsUiSize, IconsUiTone, IconsetGlyph};
use crate::icons_ui::{IconsUiStateInput, logic};
use crate::iconset::Iconset;
use leptos::prelude::*;

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
    let (icon, _icon_reference_source, has_explicit_icon_reference, used_default_icon_reference) =
        logic::normalize_icon_reference(icon);
    let icon_reference = icon.clone();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name_for_wrapper = class_name.clone();
    let class_name_for_inner = class_name
        .map(|class_name| format!("ui-icons-ui {class_name}"))
        .unwrap_or_else(|| "ui-icons-ui".to_string());

    let aria_label = logic::normalize_optional_text(aria_label);
    let has_custom_aria_label = aria_label.is_some();
    let aria_label_for_inner = aria_label.unwrap_or_default();

    let state = logic::resolve_state(IconsUiStateInput {
        disabled,
        decorative,
        has_explicit_icon_reference,
        used_default_icon_reference,
        has_custom_aria_label,
        has_custom_class_name,
        has_custom_glyphs: !glyphs.is_empty(),
        has_custom_size: size != IconsUiSize::default(),
        has_custom_tone: tone != IconsUiTone::default(),
    });

    let class = logic::compose_class_name(class_name_for_wrapper, state);

    let mut registry = logic::default_ui_glyphs();
    registry.extend(glyphs);

    view! {
        <span
            class=class
            data-slot="icons-ui"
            data-state=state.state_attr
            data-icon-reference=icon_reference
            data-icon-reference-source=state.icon_reference_source_attr
            data-aria-source=state.aria_source_attr
            data-class-source=state.class_source_attr
            data-glyph-source=state.glyph_source_attr
            data-size-source=state.size_source_attr
            data-tone-source=state.tone_source_attr
            data-disabled=state.is_disabled.then_some("true")
            data-decorative=state.is_decorative.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-custom-glyphs=state.has_custom_glyphs.then_some("true")
            data-custom-size=state.has_custom_size.then_some("true")
            data-custom-tone=state.has_custom_tone.then_some("true")
        >
            <Iconset
                icon=icon
                iconset="ui".to_string()
                glyphs=registry
                size=size
                tone=tone
                disabled=disabled
                decorative=decorative
                aria_label=aria_label_for_inner
                class_name=class_name_for_inner
            />
        </span>
    }
}
