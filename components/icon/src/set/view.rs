use crate::Icon;
use crate::iconset::{IconsetGlyph, IconsetSize, IconsetStateInput, IconsetTone, logic};
use crate::protocol;
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Iconset(
    #[prop(into)] icon: String,
    #[prop(optional, into)] iconset: Option<String>,
    #[prop(optional)] glyphs: Vec<IconsetGlyph>,
    #[prop(optional)] size: IconsetSize,
    #[prop(optional)] tone: IconsetTone,
    #[prop(optional)] is_disabled: bool,
    #[prop(default = true)] is_decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let (iconset_from_icon, icon_name) = logic::parse_icon_reference(&icon);
    let iconset_from_prop = logic::normalize_optional_text(iconset);
    let (resolved_iconset, _iconset_source, has_custom_iconset_prop, has_iconset_in_icon_reference) =
        logic::resolve_iconset_namespace(iconset_from_prop, iconset_from_icon);

    let icon_name = logic::normalize_icon_name(icon_name);

    let (glyph_content, has_registry_match, registry_label) =
        logic::resolve_registry_glyph(glyphs, &resolved_iconset, &icon_name);

    let custom_aria_label = logic::normalize_optional_text(aria_label);
    let has_custom_aria_label = custom_aria_label.is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let normalized_lang = logic::normalize_optional_text(lang);
    let locale = locale_attrs(normalized_lang.clone(), dir);
    let icon_lang: String = normalized_lang.clone().unwrap_or_default();
    let icon_dir: A11yDirection = dir.unwrap_or(A11yDirection::Ltr);

    let has_custom_size = size != IconsetSize::default();
    let has_custom_tone = tone != IconsetTone::default();

    let state = logic::resolve_state(IconsetStateInput {
        disabled: is_disabled,
        decorative: is_decorative,
        has_registry_match,
        has_registry_label: registry_label.is_some(),
        has_custom_iconset_prop,
        has_iconset_in_icon_reference,
        has_custom_aria_label,
        has_custom_class_name,
        has_custom_size,
        has_custom_tone,
    });

    let computed_aria_label: String = logic::resolve_accessible_label(
        is_decorative,
        custom_aria_label,
        registry_label,
        &icon_name,
    );

    let class_name = logic::compose_class_name(class_name, state);
    let agent_data = protocol::resolve_agent_data_attrs(protocol::IconAgentInput {
        intent: protocol::IconAgentIntent::IconsetResolve,
        state_attr: state.state_attr,
        source_attr: state.icon_source_attr,
    });
    let output_data = protocol::resolve_output_data_attrs();

    view! {
        <span
            class=class_name
            lang=locale.lang
            dir=locale.dir
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
            data-ui-schema=agent_data.schema_name
            data-ui-schema-version=agent_data.schema_version.as_attr()
            data-ui-intent=agent_data.intent.as_attr()
            data-ui-action=agent_data.action.as_attr()
            data-ui-state=agent_data.state.as_attr()
            data-ui-source=agent_data.source.as_attr()
            data-ui-streaming=output_data.streaming.as_attr()
            data-ui-streaming-fallback=output_data.fallback.as_attr()
            data-ui-output-mode=output_data.mode.as_attr()
            data-ui-output-status=output_data.status.as_attr()
        >
            <Icon
                size=size
                tone=tone
                is_disabled=is_disabled
                is_decorative=is_decorative
                aria_label=computed_aria_label
                class_name="ui-iconset__icon"
                lang=icon_lang
                dir=icon_dir
            >
                {glyph_content}
            </Icon>
        </span>
    }
}
