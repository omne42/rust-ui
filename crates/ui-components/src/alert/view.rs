use crate::alert::{
    AlertVariant,
    logic::{self, AlertStateInput},
};
use leptos::prelude::*;

#[component]
pub fn Alert(
    #[prop(optional)] variant: AlertVariant,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let title = logic::normalize_optional_text(title);
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(AlertStateInput {
        variant,
        has_title: title.is_some(),
        has_description: description.is_some(),
        has_actions: true,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    view! {
        <section
            class=class
            data-slot="alert"
            data-state=state.state_attr
            data-variant=state.variant_attr
            data-title=state.title_attr
            data-description=state.description_attr
            data-actions=state.actions_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            role=state.role_attr
            aria-live=state.live_attr
        >
            {title.map(|title| {
                view! {
                    <div class="ui-alert__title" data-slot="alert-title">
                        {title}
                    </div>
                }
            })}
            {description.map(|description| {
                view! {
                    <div class="ui-alert__description" data-slot="alert-description">
                        {description}
                    </div>
                }
            })}
            <div class="ui-alert__actions" data-slot="alert-actions">
                {children()}
            </div>
        </section>
    }
}
