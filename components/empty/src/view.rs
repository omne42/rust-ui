use crate::{EmptyPartStateInput, EmptySlot, logic};
use leptos::prelude::*;

#[component]
pub fn Empty(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(EmptyPartStateInput {
        slot: EmptySlot::Root,
        media_variant: logic::EmptyMediaVariant::default(),
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class_name
            data-slot=state.slot_attr
            data-state=state.state_attr
            data-class-source=state.class_source_attr
            data-variant-source=state.variant_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyHeader(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(EmptyPartStateInput {
        slot: EmptySlot::Header,
        media_variant: logic::EmptyMediaVariant::default(),
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class_name
            data-slot=state.slot_attr
            data-state=state.state_attr
            data-class-source=state.class_source_attr
            data-variant-source=state.variant_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyTitle(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(EmptyPartStateInput {
        slot: EmptySlot::Title,
        media_variant: logic::EmptyMediaVariant::default(),
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class_name
            data-slot=state.slot_attr
            data-state=state.state_attr
            data-class-source=state.class_source_attr
            data-variant-source=state.variant_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyDescription(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(EmptyPartStateInput {
        slot: EmptySlot::Description,
        media_variant: logic::EmptyMediaVariant::default(),
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class_name
            data-slot=state.slot_attr
            data-state=state.state_attr
            data-class-source=state.class_source_attr
            data-variant-source=state.variant_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyContent(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(EmptyPartStateInput {
        slot: EmptySlot::Content,
        media_variant: logic::EmptyMediaVariant::default(),
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class_name
            data-slot=state.slot_attr
            data-state=state.state_attr
            data-class-source=state.class_source_attr
            data-variant-source=state.variant_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyMedia(
    #[prop(optional)] variant: logic::EmptyMediaVariant,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(EmptyPartStateInput {
        slot: EmptySlot::Media,
        media_variant: variant,
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class_name
            data-slot=state.slot_attr
            data-state=state.state_attr
            data-variant=state.media_variant_attr
            data-class-source=state.class_source_attr
            data-variant-source=state.variant_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </div>
    }
}
