use super::{IconsWorkflowSize, IconsWorkflowTone, IconsetGlyph};
use crate::icons_workflow::{IconsWorkflowStateInput, logic};
use crate::iconset::Iconset;
use crate::protocol;
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn IconsWorkflow(
    #[prop(into)] icon: String,
    #[prop(optional)] size: IconsWorkflowSize,
    #[prop(optional)] tone: IconsWorkflowTone,
    #[prop(optional)] is_disabled: bool,
    #[prop(default = true)] is_decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] glyphs: Vec<IconsetGlyph>,
) -> impl IntoView {
    let (icon, _icon_reference_source, has_explicit_icon_reference, used_default_icon_reference) =
        logic::normalize_icon_reference(icon);
    let icon_reference = icon.clone();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name_for_wrapper = class_name.clone();
    let class_name_for_inner = logic::resolve_inner_class_name(class_name);
    let normalized_lang = logic::normalize_optional_text(lang);
    let locale = locale_attrs(normalized_lang.clone(), dir);
    let icon_lang = normalized_lang.clone().unwrap_or_default();
    let icon_dir = dir.unwrap_or(A11yDirection::Ltr);

    let aria_label = logic::normalize_optional_text(aria_label);
    let has_custom_aria_label = aria_label.is_some();
    let aria_label_for_inner = logic::resolve_inner_aria_label(aria_label);

    let state = logic::resolve_state(IconsWorkflowStateInput {
        disabled: is_disabled,
        decorative: is_decorative,
        has_explicit_icon_reference,
        used_default_icon_reference,
        has_custom_aria_label,
        has_custom_class_name,
        has_custom_glyphs: !glyphs.is_empty(),
        has_custom_size: size != IconsWorkflowSize::default(),
        has_custom_tone: tone != IconsWorkflowTone::default(),
    });

    let class = logic::compose_class_name(class_name_for_wrapper, state);
    let agent_data = protocol::resolve_agent_data_attrs(protocol::IconAgentInput {
        intent: protocol::IconAgentIntent::IconsWorkflowResolve,
        state_attr: state.state_attr,
        source_attr: state.icon_reference_source_attr,
    });
    let output_data = protocol::resolve_output_data_attrs();

    let mut registry = logic::default_workflow_glyphs();
    registry.extend(glyphs);

    view! {
        <span
            class=class
            lang=locale.lang
            dir=locale.dir
            data-slot="icons-workflow"
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
            <Iconset
                icon=icon
                iconset="workflow"
                glyphs=registry
                size=size
                tone=tone
                is_disabled=is_disabled
                is_decorative=is_decorative
                aria_label=aria_label_for_inner
                class_name=class_name_for_inner
                lang=icon_lang
                dir=icon_dir
            />
        </span>
    }
}
