use crate::logic::{self, ItemMediaVariant, ItemSize, ItemVariant};
use leptos::prelude::*;

#[component]
pub fn ItemGroup(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item-group", class_name);

    view! {
        <div class=class_name role="list" data-slot="item-group">
            {children()}
        </div>
    }
}

#[component]
pub fn ItemSeparator(#[prop(optional, into)] class_name: Option<String>) -> impl IntoView {
    let class_name = logic::compose_class("ui-item-separator", class_name);

    view! {
        <div class=class_name role="separator" data-slot="item-separator" aria-orientation="horizontal" />
    }
}

#[component]
pub fn Item(
    #[prop(optional)] variant: ItemVariant,
    #[prop(optional)] size: ItemSize,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item", class_name);

    view! {
        <div class=class_name role="listitem" data-slot="item" data-variant=variant.as_attr() data-size=size.as_attr()>
            {children()}
        </div>
    }
}

#[component]
pub fn ItemMedia(
    #[prop(optional)] variant: ItemMediaVariant,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__media", class_name);

    view! {
        <div class=class_name data-slot="item-media" data-variant=variant.as_attr()>
            {children()}
        </div>
    }
}

#[component]
pub fn ItemContent(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__content", class_name);

    view! {
        <div class=class_name data-slot="item-content">
            {children()}
        </div>
    }
}

#[component]
pub fn ItemTitle(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__title", class_name);

    view! {
        <div class=class_name data-slot="item-title">
            {children()}
        </div>
    }
}

#[component]
pub fn ItemDescription(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__description", class_name);

    view! {
        <p class=class_name data-slot="item-description">
            {children()}
        </p>
    }
}

#[component]
pub fn ItemActions(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__actions", class_name);

    view! {
        <div class=class_name data-slot="item-actions">
            {children()}
        </div>
    }
}

#[component]
pub fn ItemHeader(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__header", class_name);

    view! {
        <div class=class_name data-slot="item-header">
            {children()}
        </div>
    }
}

#[component]
pub fn ItemFooter(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__footer", class_name);

    view! {
        <div class=class_name data-slot="item-footer">
            {children()}
        </div>
    }
}
