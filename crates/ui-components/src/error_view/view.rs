use crate::error_view::{
    ErrorViewMotion, ErrorViewStateInput,
    logic::{self, ErrorViewTone},
    motion,
};
use leptos::children::{Children, ViewFn};
use leptos::{html, prelude::*};

#[component]
pub fn ErrorView(
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] tone: ErrorViewTone,
    #[prop(optional)] compact: bool,
    #[prop(optional)] bordered: bool,
    #[prop(optional)] motion: ErrorViewMotion,
    #[prop(optional, into)] message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional, into)] actions: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (message, has_custom_message) = logic::normalize_message(message);
    let message = StoredValue::new(message);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let has_icon = icon.is_some();
    let has_actions = actions.is_some();
    let has_children = children.is_some();
    let has_custom_motion = motion != ErrorViewMotion::default();

    let icon = icon.map(StoredValue::new);
    let actions = actions.map(StoredValue::new);

    let state = Signal::derive(move || {
        logic::resolve_state(ErrorViewStateInput {
            tone,
            is_invalid,
            compact,
            bordered,
            has_icon,
            has_actions,
            has_children,
            has_custom_message,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_motion,
        })
    });

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
            data-state=move || state.get().state_attr
            data-invalid=move || state.get().is_visible.then_some("true")
            data-hidden=move || state.get().is_hidden.then_some("true")
            data-compact=move || state.get().is_compact.then_some("true")
            data-bordered=move || state.get().is_bordered.then_some("true")
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
        >
            {state.get().has_icon.then(|| {
                let icon = icon.expect("checked has_icon");
                view! {
                    <span class="ui-error-view__icon" data-slot="error-view-icon" aria-hidden="true">
                        {icon.get_value().run()}
                    </span>
                }
            })}

            {content}

            {state.get().has_actions.then(|| {
                let actions = actions.expect("checked has_actions");
                view! {
                    <div class="ui-error-view__actions" data-slot="error-view-actions">
                        {actions.get_value().run()}
                    </div>
                }
            })}
        </div>
    }
}
