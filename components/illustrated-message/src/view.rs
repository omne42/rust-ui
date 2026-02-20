use crate::{IllustratedMessageMotion, IllustratedMessageOrientation, motion};
use leptos::children::ViewFn;
use leptos::prelude::*;

#[component]
pub fn IllustratedMessage(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] illustration: Option<ViewFn>,
    #[prop(optional, into)] actions: Option<ViewFn>,
    #[prop(optional)] orientation: IllustratedMessageOrientation,
    #[prop(optional)] motion: IllustratedMessageMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = crate::motion::sanitize_motion(motion);
    let has_illustration = illustration.is_some();
    let has_actions = actions.is_some();
    let state = crate::logic::resolve_view_state(
        has_illustration,
        title.as_deref(),
        description.as_deref(),
        has_actions,
    );

    let base_class = format!("ui-illustrated-message {}", orientation.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let root_ref = NodeRef::new();
    motion::attach_motion(root_ref, motion);

    let illustration = illustration.map(StoredValue::new);
    let actions = actions.map(StoredValue::new);

    view! {
        <div class=class data-slot="illustrated-message" node_ref=root_ref>
            {state.show_illustration.then(|| {
                illustration
                    .map(|illustration| {
                        view! {
                            <div class="ui-illustrated-message__illustration" data-slot="illustrated-message-illustration">
                                {illustration.get_value().run()}
                            </div>
                        }
                        .into_any()
                    })
                    .unwrap_or_else(|| ().into_any())
            })}

            <div class="ui-illustrated-message__content" data-slot="illustrated-message-content">
                {state.show_title.then(|| {
                    let title = title.clone().unwrap_or_default();
                    view! {
                        <h3 class="ui-illustrated-message__title" data-slot="illustrated-message-title">
                            {title}
                        </h3>
                    }
                })}

                {state.show_description.then(|| {
                    let description = description.clone().unwrap_or_default();
                    view! {
                        <p class="ui-illustrated-message__description" data-slot="illustrated-message-description">
                            {description}
                        </p>
                    }
                })}

                {state.show_actions.then(|| {
                    actions
                        .map(|actions| {
                            view! {
                                <div class="ui-illustrated-message__actions" data-slot="illustrated-message-actions">
                                    {actions.get_value().run()}
                                </div>
                            }
                            .into_any()
                        })
                        .unwrap_or_else(|| ().into_any())
                })}
            </div>
        </div>
    }
}
