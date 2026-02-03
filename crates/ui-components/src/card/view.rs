use crate::card::CardVariant;
use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(optional)] variant: CardVariant,
    #[prop(optional, default = true)] padded: bool,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_class = format!("ui-card {}", variant.class_name());
    let base_class = if padded {
        base_class
    } else {
        format!("{base_class} ui-card--no-padding")
    };

    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <section class=class data-slot="card">
            {children()}
        </section>
    }
}
