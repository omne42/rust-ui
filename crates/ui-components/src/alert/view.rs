use crate::alert::AlertVariant;
use leptos::prelude::*;

#[component]
pub fn Alert(
    #[prop(optional)] variant: AlertVariant,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_class = format!("ui-alert {}", variant.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <section class=class data-slot="alert" role="status" aria-live="polite">
            {title.map(|title| view! { <div class="ui-alert__title" data-slot="alert-title">{title}</div> })}
            {description.map(|description| {
                view! { <div class="ui-alert__description" data-slot="alert-description">{description}</div> }
            })}
            <div class="ui-alert__actions" data-slot="alert-actions">
                {children()}
            </div>
        </section>
    }
}
