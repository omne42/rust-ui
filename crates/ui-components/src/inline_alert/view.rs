use crate::inline_alert::{
    InlineAlertFill, InlineAlertMotion, InlineAlertTone, logic, motion as inline_alert_motion,
};
use leptos::children::ViewFn;
use leptos::{html, prelude::*};

#[component]
pub fn InlineAlert(
    #[prop(optional)] tone: InlineAlertTone,
    #[prop(optional)] fill: InlineAlertFill,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] hide_icon: bool,
    #[prop(optional, into)] icon_label: Option<String>,
    #[prop(optional, into)] start_content: Option<ViewFn>,
    #[prop(optional, into)] end_content: Option<ViewFn>,
    #[prop(optional)] motion: InlineAlertMotion,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let motion = crate::inline_alert::motion::sanitize_motion(motion);
    let state =
        logic::resolve_view_state(tone, title.as_deref(), description.as_deref(), hide_icon);

    let node_ref: NodeRef<html::Section> = NodeRef::new();
    inline_alert_motion::attach_motion(node_ref, motion);

    let base_class = format!(
        "ui-inline-alert {} {}",
        tone.class_name(),
        fill.class_name()
    );
    let class = class_name
        .filter(|value| !value.trim().is_empty())
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
            data-slot="inline-alert"
            role=tone.role()
            aria-live=tone.aria_live()
            data-motion-source=if motion == InlineAlertMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != InlineAlertMotion::default()).then_some("true")
        >
            <Show when=move || state.show_icon>
                <span class="ui-inline-alert__icon" data-slot="inline-alert-icon">
                    <span class="ui-inline-alert__sr-only">{move || icon_label.get_value()}</span>
                    {match tone {
                        InlineAlertTone::Neutral => ().into_any(),
                        InlineAlertTone::Info => view! {
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                                <path d="M10 9v5" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                                <circle cx="10" cy="6" r="1" fill="currentColor" />
                            </svg>
                        }.into_any(),
                        InlineAlertTone::Positive => view! {
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
                        }.into_any(),
                        InlineAlertTone::Notice | InlineAlertTone::Negative => view! {
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
                        }.into_any(),
                    }}
                </span>
            </Show>

            <Show when=move || start_content.is_some()>
                <span class="ui-inline-alert__start" data-slot="inline-alert-start">
                    {start_content
                        .expect("checked start_content")
                        .get_value()
                        .run()}
                </span>
            </Show>

            <div class="ui-inline-alert__body" data-slot="inline-alert-body">
                {state.show_title.then(|| {
                    let title = title.clone().unwrap_or_default();
                    view! { <div class="ui-inline-alert__title" data-slot="inline-alert-title">{title}</div> }
                })}
                {state.show_description.then(|| {
                    let description = description.clone().unwrap_or_default();
                    view! { <div class="ui-inline-alert__description" data-slot="inline-alert-description">{description}</div> }
                })}
                <div class="ui-inline-alert__content" data-slot="inline-alert-content">{children()}</div>
            </div>

            <Show when=move || end_content.is_some()>
                <span class="ui-inline-alert__end" data-slot="inline-alert-end">
                    {end_content
                        .expect("checked end_content")
                        .get_value()
                        .run()}
                </span>
            </Show>
        </section>
    }
}
