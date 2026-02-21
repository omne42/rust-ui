use crate::{
    AlertFill, AlertLayout, AlertMotion, AlertTone, AlertVariant,
    logic::{self, AlertStateInput},
    motion as alert_motion,
};
use leptos::children::ViewFn;
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, locale_attrs};

fn render_alert_icon(layout: AlertLayout, tone: AlertTone) -> AnyView {
    match (layout, tone) {
        (AlertLayout::Inline, AlertTone::Neutral) => ().into_any(),
        (AlertLayout::Inline, AlertTone::Info) => view! {
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                <path d="M10 9v5" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                <circle cx="10" cy="6" r="1" fill="currentColor" />
            </svg>
        }
        .into_any(),
        (AlertLayout::Inline, AlertTone::Positive) => view! {
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                <path
                    d="M6.5 10.2l2.4 2.4 4.6-5"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linecap="round"
                    stroke_linejoin="round"
                />
            </svg>
        }
        .into_any(),
        (AlertLayout::Inline, AlertTone::Notice | AlertTone::Negative) => view! {
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <path
                    d="M10 3.5l6.5 11.2a1 1 0 0 1-.9 1.5H4.4a1 1 0 0 1-.9-1.5L10 3.5z"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linejoin="round"
                />
                <path d="M10 8v4" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                <circle cx="10" cy="14.2" r="1" fill="currentColor" />
            </svg>
        }
        .into_any(),
        (AlertLayout::Banner, AlertTone::Neutral) => ().into_any(),
        (AlertLayout::Banner, AlertTone::Info) => view! {
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                <path d="M10 8v5" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                <circle cx="10" cy="5.6" r="1" fill="currentColor" />
            </svg>
        }
        .into_any(),
        (AlertLayout::Banner, AlertTone::Positive) => view! {
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                <path
                    d="M6.7 10.2l2.1 2.1 4.5-4.6"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linecap="round"
                    stroke_linejoin="round"
                />
            </svg>
        }
        .into_any(),
        (AlertLayout::Banner, AlertTone::Notice) => view! {
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <path
                    d="M10 3.5l6.5 11.2a1 1 0 0 1-.9 1.5H4.4a1 1 0 0 1-.9-1.5L10 3.5z"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linejoin="round"
                />
                <path d="M10 8v4" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                <circle cx="10" cy="14.2" r="1" fill="currentColor" />
            </svg>
        }
        .into_any(),
        (AlertLayout::Banner, AlertTone::Negative) => view! {
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                <path
                    d="M7.6 7.6l4.8 4.8m0-4.8l-4.8 4.8"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linecap="round"
                />
            </svg>
        }
        .into_any(),
    }
}

#[component]
pub fn Alert(
    #[prop(optional)] tone: Option<AlertTone>,
    #[prop(optional)] variant: Option<AlertVariant>,
    #[prop(optional)] layout: Option<AlertLayout>,
    #[prop(optional)] fill: Option<AlertFill>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] is_hide_icon: Option<bool>,
    #[prop(optional)] hide_icon: Option<bool>,
    #[prop(optional, into)] icon_label: Option<String>,
    #[prop(optional, into)] start_content: Option<ViewFn>,
    #[prop(optional, into)] end_content: Option<ViewFn>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: AlertMotion,
    children: Children,
) -> impl IntoView {
    let motion = crate::motion::sanitize_motion(motion);
    let title = logic::normalize_optional_text(title);
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);
    let locale = locale_attrs(lang, dir);

    let (hide_icon, hide_icon_source) = logic::resolve_hide_icon(is_hide_icon, hide_icon);

    let state = logic::resolve_state(AlertStateInput {
        tone,
        variant,
        layout,
        fill,
        has_title: title.is_some(),
        has_description: description.is_some(),
        hide_icon,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    let node_ref: NodeRef<html::Section> = NodeRef::new();
    alert_motion::attach_motion(node_ref, motion);

    let (icon_label, icon_label_source) = logic::resolve_icon_label(icon_label, state.tone);
    let motion_source = logic::resolve_motion_source(motion == AlertMotion::default());
    let agent_source = logic::resolve_agent_source(state.variant_source_attr);
    let icon_label = StoredValue::new(icon_label);

    let start_content = start_content.map(StoredValue::new);
    let end_content = end_content.map(StoredValue::new);

    view! {
        <section
            class=class
            node_ref=node_ref
            data-slot="alert"
            data-layout=state.layout_attr
            data-tone=state.tone_attr
            data-fill=state.fill_attr
            data-title=state.title_attr
            data-description=state.description_attr
            data-icon=state.icon_attr
            data-variant-source=state.variant_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-hide-icon=hide_icon.then_some("true")
            data-hide-icon-source=hide_icon_source
            data-icon-label-source=icon_label_source.as_attr()
            role=state.role_attr
            aria-live=state.live_attr
            lang=locale.lang
            dir=locale.dir
            data-motion-source=motion_source.as_attr()
            data-custom-motion=(motion_source == logic::AlertMotionSource::Custom).then_some("true")
            data-ui-schema=logic::AlertAgentSchema::V1.as_attr()
            data-ui-intent=logic::AlertAgentIntent::StatusRegion.as_attr()
            data-ui-action=logic::AlertAgentAction::Announce.as_attr()
            data-ui-state=logic::AlertAgentState::Snapshot.as_attr()
            data-ui-source=agent_source.as_attr()
            data-ui-streaming=logic::AlertStreamingPolicy::Optional.as_attr()
            data-ui-fallback=logic::AlertStreamingFallback::Snapshot.as_attr()
            data-ui-output-status=logic::AlertOutputStatus::Verified.as_attr()
        >
            <Show when=move || state.show_icon>
                <span class="ui-alert__icon" data-slot="alert-icon">
                    <span class="ui-alert__sr-only">{move || icon_label.get_value()}</span>
                    {render_alert_icon(state.layout, state.tone)}
                </span>
            </Show>

            {start_content.map(|content| {
                view! {
                    <span class="ui-alert__start" data-slot="alert-start">
                        {content.get_value().run()}
                    </span>
                }
            })}

            <div class="ui-alert__body" data-slot="alert-body">
                {title.clone().filter(|_| state.show_title).map(|title| {
                    view! {
                        <div class="ui-alert__title" data-slot="alert-title">
                            {title}
                        </div>
                    }
                })}
                {description.clone().filter(|_| state.show_description).map(|description| {
                    view! {
                        <div class="ui-alert__description" data-slot="alert-description">
                            {description}
                        </div>
                    }
                })}
                <div class="ui-alert__content" data-slot="alert-content">
                    {children()}
                </div>
            </div>

            {end_content.map(|content| {
                view! {
                    <span class="ui-alert__end" data-slot="alert-end">
                        {content.get_value().run()}
                    </span>
                }
            })}
        </section>
    }
}
