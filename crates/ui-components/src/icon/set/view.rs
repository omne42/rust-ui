use crate::icon::Icon;
use crate::iconset::{IconsetGlyph, IconsetSize, IconsetStateInput, IconsetTone, logic};
use leptos::prelude::*;

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
    let (iconset_from_icon, icon_name) = logic::parse_icon_reference(&icon);
    let iconset_from_prop = logic::normalize_optional_text(iconset);
    let (resolved_iconset, _iconset_source, has_custom_iconset_prop, has_iconset_in_icon_reference) =
        logic::resolve_iconset_namespace(iconset_from_prop, iconset_from_icon);

    let icon_name =
        logic::normalize_optional_text(Some(icon_name)).unwrap_or_else(|| "unknown".to_string());

    let (glyph_content, has_registry_match, registry_label) =
        logic::resolve_registry_glyph(glyphs, &resolved_iconset, &icon_name);

    let custom_aria_label = logic::normalize_optional_text(aria_label);
    let has_custom_aria_label = custom_aria_label.is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let has_custom_size = size != IconsetSize::default();
    let has_custom_tone = tone != IconsetTone::default();

    let state = logic::resolve_state(IconsetStateInput {
        disabled,
        decorative,
        has_registry_match,
        has_registry_label: registry_label.is_some(),
        has_custom_iconset_prop,
        has_iconset_in_icon_reference,
        has_custom_aria_label,
        has_custom_class_name,
        has_custom_size,
        has_custom_tone,
    });

    let computed_aria_label =
        logic::resolve_accessible_label(decorative, custom_aria_label, registry_label, &icon_name);

    let class_name = logic::compose_class_name(class_name, state);

    view! {
        <span
            class=class_name
            data-slot="iconset"
            data-state=state.state_attr
            data-iconset=resolved_iconset
            data-icon-name=icon_name
            data-icon-source=state.icon_source_attr
            data-iconset-source=state.iconset_source_attr
            data-label-source=state.label_source_attr
            data-class-source=state.class_source_attr
            data-size-source=state.size_source_attr
            data-tone-source=state.tone_source_attr
            data-disabled=state.is_disabled.then_some("true")
            data-decorative=state.is_decorative.then_some("true")
            data-custom-iconset=state.has_custom_iconset_prop.then_some("true")
            data-icon-ref-namespace=state.has_iconset_in_icon_reference.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-custom-size=state.has_custom_size.then_some("true")
            data-custom-tone=state.has_custom_tone.then_some("true")
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
