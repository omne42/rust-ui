use super::{IconsGlyph, IconsScale, IconsSet, IconsTone};
use crate::icons::{IconsStateInput, logic};
use crate::icons_ui::IconsUi;
use crate::icons_workflow::IconsWorkflow;
use crate::protocol;
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Icons(
    #[prop(into)] name: String,
    #[prop(optional)] set: IconsSet,
    #[prop(optional)] scale: IconsScale,
    #[prop(optional)] tone: IconsTone,
    #[prop(optional)] is_disabled: bool,
    #[prop(default = true)] is_decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] glyphs: Vec<IconsGlyph>,
) -> impl IntoView {
    let (resolved_set, has_set_prefix_in_name) = logic::resolve_set(&name, set);
    let normalized_name = logic::normalize_name(name, resolved_set);

    let aria_label = logic::normalize_optional_text(aria_label);
    let has_custom_aria_label = aria_label.is_some();
    let aria_label_for_inner: String = logic::resolve_inner_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name_for_wrapper = class_name.clone();
    let class_name_for_inner: String = logic::resolve_inner_class_name(class_name);
    let normalized_lang = logic::normalize_optional_text(lang);
    let locale = locale_attrs(normalized_lang.clone(), dir);
    let inner_lang: String = normalized_lang.clone().unwrap_or_default();
    let inner_dir: A11yDirection = dir.unwrap_or(A11yDirection::Ltr);

    let state = logic::resolve_state(IconsStateInput {
        set: resolved_set,
        scale,
        disabled: is_disabled,
        decorative: is_decorative,
        has_set_prefix_in_name,
        has_custom_set_prop: set != IconsSet::default(),
        has_custom_aria_label,
        has_custom_class_name,
        has_custom_glyphs: !glyphs.is_empty(),
        has_custom_tone: tone != IconsTone::default(),
    });

    let class = logic::compose_class_name(class_name_for_wrapper, state);
    let agent_data = protocol::resolve_agent_data_attrs(protocol::IconAgentInput {
        intent: protocol::IconAgentIntent::IconsResolve,
        state_attr: state.state_attr,
        source_attr: state.set_source_attr,
    });
    let output_data = protocol::resolve_output_data_attrs();

    let content = match resolved_set {
        IconsSet::Ui => view! {
            <IconsUi
                icon=normalized_name
                size=scale.as_ui_size()
                tone=tone
                is_disabled=is_disabled
                is_decorative=is_decorative
                aria_label=aria_label_for_inner
                class_name=class_name_for_inner
                lang=inner_lang.clone()
                dir=inner_dir
                glyphs=glyphs
            />
        }
        .into_any(),
        IconsSet::Workflow => view! {
            <IconsWorkflow
                icon=normalized_name
                size=scale.as_workflow_size()
                tone=tone
                is_disabled=is_disabled
                is_decorative=is_decorative
                aria_label=aria_label_for_inner
                class_name=class_name_for_inner
                lang=inner_lang.clone()
                dir=inner_dir
                glyphs=glyphs
            />
        }
        .into_any(),
    };

    view! {
        <span
            class=class
            lang=locale.lang
            dir=locale.dir
            data-slot="icons"
            data-set=state.set_attr
            data-scale=state.scale_attr
            data-state=state.state_attr
            data-set-source=state.set_source_attr
            data-aria-source=state.aria_source_attr
            data-class-source=state.class_source_attr
            data-glyph-source=state.glyph_source_attr
            data-tone-source=state.tone_source_attr
            data-disabled=state.is_disabled.then_some("true")
            data-decorative=state.is_decorative.then_some("true")
            data-custom-set=state.has_custom_set_prop.then_some("true")
            data-name-prefixed=state.has_set_prefix_in_name.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-custom-glyphs=state.has_custom_glyphs.then_some("true")
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
            {content}
        </span>
    }
}
