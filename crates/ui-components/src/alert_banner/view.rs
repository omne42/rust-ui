use crate::alert_banner::{
    AlertBannerFill, AlertBannerMotion, AlertBannerTone, logic, motion as alert_banner_motion,
};
use leptos::children::ViewFn;
use leptos::{html, prelude::*};

#[component]
pub fn AlertBanner(
    #[prop(optional)] tone: AlertBannerTone,
    #[prop(optional)] fill: AlertBannerFill,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] hide_icon: bool,
    #[prop(optional, into)] icon_label: Option<String>,
    #[prop(optional, into)] start_content: Option<ViewFn>,
    #[prop(optional, into)] end_content: Option<ViewFn>,
    #[prop(optional)] motion: AlertBannerMotion,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let title = logic::normalize_optional_text(title);
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);

    let state =
        logic::resolve_view_state(tone, title.as_deref(), description.as_deref(), hide_icon);

    let node_ref: NodeRef<html::Section> = NodeRef::new();
    alert_banner_motion::attach_motion(node_ref, motion);

    let base_class = format!(
        "ui-alert-banner {} {}",
        tone.class_name(),
        fill.class_name()
    );
    let class = class_name
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let icon_label = icon_label
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .or_else(|| tone.default_icon_label().map(|v| v.to_string()));
    let icon_label = StoredValue::new(icon_label.unwrap_or_default());

    let start_content = start_content.map(StoredValue::new);
    let end_content = end_content.map(StoredValue::new);

    view! {
        <section
            class=class
            node_ref=node_ref
            data-slot="alert-banner"
            data-tone=tone.class_name().trim_start_matches("ui-alert-banner--tone-")
            data-fill=fill.class_name().trim_start_matches("ui-alert-banner--fill-")
            role=tone.role()
            aria-live=tone.aria_live()
            data-motion-source=if motion == AlertBannerMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != AlertBannerMotion::default()).then_some("true")
        >
            <Show when=move || state.show_icon>
                <span class="ui-alert-banner__icon" data-slot="alert-banner-icon">
                    <span class="ui-alert-banner__sr-only">{move || icon_label.get_value()}</span>
                    {match tone {
                        AlertBannerTone::Neutral => ().into_any(),
                        AlertBannerTone::Info => view! {
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                                <path d="M10 8v5" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                                <circle cx="10" cy="5.6" r="1" fill="currentColor" />
                            </svg>
                        }.into_any(),
                        AlertBannerTone::Positive => view! {
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                                <path d="M6.7 10.2l2.1 2.1 4.5-4.6" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" stroke_linejoin="round" />
                            </svg>
                        }.into_any(),
                        AlertBannerTone::Notice => view! {
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <path d="M10 3.5l6.5 11.2a1 1 0 0 1-.9 1.5H4.4a1 1 0 0 1-.9-1.5L10 3.5z" stroke="currentColor" stroke_width="1.5" stroke_linejoin="round" />
                                <path d="M10 8v4" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                                <circle cx="10" cy="14.2" r="1" fill="currentColor" />
                            </svg>
                        }.into_any(),
                        AlertBannerTone::Negative => view! {
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                                <path d="M7.6 7.6l4.8 4.8m0-4.8l-4.8 4.8" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                            </svg>
                        }.into_any(),
                    }}
                </span>
            </Show>

            <Show when=move || start_content.is_some()>
                <span class="ui-alert-banner__start" data-slot="alert-banner-start">
                    {start_content
                        .expect("checked start_content")
                        .get_value()
                        .run()}
                </span>
            </Show>

            <div class="ui-alert-banner__body" data-slot="alert-banner-body">
                {state.show_title.then(|| {
                    let title = title.clone().unwrap_or_default();
                    view! {
                        <div class="ui-alert-banner__title" data-slot="alert-banner-title">
                            {title}
                        </div>
                    }
                })}
                {state.show_description.then(|| {
                    let description = description.clone().unwrap_or_default();
                    view! {
                        <div class="ui-alert-banner__description" data-slot="alert-banner-description">
                            {description}
                        </div>
                    }
                })}
                <div class="ui-alert-banner__content" data-slot="alert-banner-content">
                    {children()}
                </div>
            </div>

            <Show when=move || end_content.is_some()>
                <span class="ui-alert-banner__end" data-slot="alert-banner-end">
                    {end_content
                        .expect("checked end_content")
                        .get_value()
                        .run()}
                </span>
            </Show>
        </section>
    }
}
