use crate::{
    logic::{self, A11yDirection, ItemMediaVariant, ItemSize, ItemVariant},
    protocol,
};
use leptos::prelude::*;

#[component]
pub fn ItemGroup(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item-group", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <div class=class_name role="list" data-slot="item-group" lang=locale.lang dir=locale.dir>
            {children()}
        </div>
    }
}

#[component]
pub fn ItemSeparator(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item-separator", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <div
            class=class_name
            role="separator"
            data-slot="item-separator"
            aria-orientation="horizontal"
            lang=locale.lang
            dir=locale.dir
        />
    }
}

#[component]
pub fn Item(
    #[prop(optional, into)] variant: Option<ItemVariant>,
    #[prop(optional, into)] size: Option<ItemSize>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let render_state = logic::derive_item_render_state(variant, size);
    let agent_attrs = protocol::agent_data_attrs(render_state);
    let class_name = logic::compose_class("ui-item", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <div
            class=class_name role="listitem" data-slot="item"
            data-variant=render_state.variant_attr data-size=render_state.size_attr
            data-variant-source=render_state.variant_source_attr
            data-size-source=render_state.size_source_attr
            data-ui-schema=agent_attrs.schema data-ui-intent=agent_attrs.intent
            data-ui-action=agent_attrs.action
            data-ui-streaming-policy=agent_attrs.streaming_policy
            data-ui-streaming-fallback=agent_attrs.streaming_fallback
            data-ui-stream-mode=agent_attrs.stream_mode data-ui-output-mode=agent_attrs.output_mode
            data-ui-output-status=agent_attrs.output_status
            data-ui-state-variant=agent_attrs.state_variant
            data-ui-state-size=agent_attrs.state_size data-ui-source-variant=agent_attrs.source_variant
            data-ui-source-size=agent_attrs.source_size lang=locale.lang dir=locale.dir
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemMedia(
    #[prop(optional, into)] variant: Option<ItemMediaVariant>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let render_state = logic::derive_item_media_render_state(variant);
    let class_name = logic::compose_class("ui-item__media", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <div
            class=class_name
            data-slot="item-media"
            data-variant=render_state.variant_attr
            data-variant-source=render_state.variant_source_attr
            lang=locale.lang
            dir=locale.dir
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemContent(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__content", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <div class=class_name data-slot="item-content" lang=locale.lang dir=locale.dir>
            {children()}
        </div>
    }
}

#[component]
pub fn ItemTitle(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__title", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <div class=class_name data-slot="item-title" lang=locale.lang dir=locale.dir>
            {children()}
        </div>
    }
}

#[component]
pub fn ItemDescription(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__description", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <p class=class_name data-slot="item-description" lang=locale.lang dir=locale.dir>
            {children()}
        </p>
    }
}

#[component]
pub fn ItemActions(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__actions", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <div class=class_name data-slot="item-actions" lang=locale.lang dir=locale.dir>
            {children()}
        </div>
    }
}

#[component]
pub fn ItemHeader(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__header", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <div class=class_name data-slot="item-header" lang=locale.lang dir=locale.dir>
            {children()}
        </div>
    }
}

#[component]
pub fn ItemFooter(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::compose_class("ui-item__footer", class_name);
    let locale = logic::resolve_locale_attrs(lang, dir);

    view! {
        <div class=class_name data-slot="item-footer" lang=locale.lang dir=locale.dir>
            {children()}
        </div>
    }
}
