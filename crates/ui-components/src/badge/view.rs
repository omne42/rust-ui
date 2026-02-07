use crate::badge::{
    BadgeVariant,
    logic::{self, BadgeStateInput},
};
use leptos::prelude::*;

#[component]
pub fn Badge(
    #[prop(optional)] variant: BadgeVariant,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(BadgeStateInput {
        variant,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    view! {
        <span
            class=class
            data-slot="badge"
            data-variant=state.variant_attr
            data-fill=state.fill_attr
            data-state=state.fill_attr
            data-solid=state.is_solid.then_some("true")
            data-outline=state.is_outline.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </span>
    }
}
