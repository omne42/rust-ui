use crate::{
    ErrorViewMotion,
    logic::{self, ErrorViewTone},
    motion,
};
use leptos::children::{Children, ViewFn};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn ErrorView(
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] tone: Option<ErrorViewTone>,
    #[prop(optional)] is_compact: Option<bool>,
    #[prop(optional)] is_bordered: Option<bool>,
    #[prop(optional)] motion: ErrorViewMotion,
    #[prop(optional, into)] message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional, into)] actions: Option<ViewFn>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let has_icon = icon.is_some();
    let has_actions = actions.is_some();
    let has_children = children.is_some();
    let has_custom_motion = motion != ErrorViewMotion::default();
    let locale = locale_attrs(lang, dir);

    let normalized = logic::normalize_props(logic::ErrorViewNormalizeInput {
        tone,
        is_invalid,
        is_compact,
        is_bordered,
        message,
        aria_label,
        class_name,
        has_icon,
        has_actions,
        has_children,
        has_custom_motion,
    });
    let message = StoredValue::new(normalized.message);
    let aria_label = normalized.aria_label;
    let class_name = StoredValue::new(normalized.class_name);
    let state_input = StoredValue::new(normalized.state_input);

    let icon = icon.map(StoredValue::new);
    let actions = actions.map(StoredValue::new);

    let state = Signal::derive(move || logic::resolve_state(state_input.get_value()));

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));
    let visible = Signal::derive(move || state.get().is_visible);

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, visible, motion);

    let content: AnyView = if let Some(children) = children {
        view! {
            <div class="ui-error-view__content" data-slot="error-view-content">
                {children()}
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class="ui-error-view__content" data-slot="error-view-content">
                <p class="ui-error-view__text" data-slot="error-view-text">
                    {message.get_value()}
                </p>
            </div>
        }
        .into_any()
    };

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            data-slot="error-view"
            data-tone=move || state.get().tone_attr
            data-tone-source=normalized.tone_source_attr
            data-state=move || state.get().state_attr
            data-invalid=move || state.get().is_visible.then_some("true")
            data-hidden=move || state.get().is_hidden.then_some("true")
            data-compact=move || state.get().is_compact.then_some("true")
            data-compact-source=normalized.compact_source_attr
            data-bordered=move || state.get().is_bordered.then_some("true")
            data-bordered-source=normalized.bordered_source_attr
            data-icon=move || state.get().has_icon.then_some("true")
            data-actions=move || state.get().has_actions.then_some("true")
            data-content=move || state.get().content_attr
            data-message-source=move || state.get().message_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            role="alert"
            aria-live=move || if state.get().is_visible { "assertive" } else { "off" }
            aria-hidden=move || state.get().is_hidden.then_some("true")
            aria-label=aria_label
            lang=locale.lang
            dir=locale.dir
        >
            {icon.map(|icon| {
                view! {
                    <span class="ui-error-view__icon" data-slot="error-view-icon" aria-hidden="true">
                        {icon.get_value().run()}
                    </span>
                }
            })}

            {content}

            {actions.map(|actions| {
                view! {
                    <div class="ui-error-view__actions" data-slot="error-view-actions">
                        {actions.get_value().run()}
                    </div>
                }
            })}
        </div>
    }
}
