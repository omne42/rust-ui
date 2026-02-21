use crate::{EmptySlot, logic};
use leptos::prelude::*;

fn render_part(
    class_name: String,
    state: logic::EmptyPartState,
    lang: Option<String>,
    dir: Option<String>,
    variant_attr: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let agent_contract = logic::resolve_agent_contract(state);

    view! {
        <div
            class=class_name
            lang=lang
            dir=dir
            data-slot=state.slot_attr
            data-state=state.state_attr
            data-variant=variant_attr
            data-class-source=state.class_source_attr
            data-variant-source=state.variant_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-ui-schema=agent_contract.schema_name
            data-ui-schema-version=agent_contract.schema_version
            data-ui-intent=agent_contract.intent.as_attr()
            data-ui-action=agent_contract.action.as_attr()
            data-ui-state=agent_contract.state
            data-ui-source=agent_contract.source.as_attr()
            data-ui-stream-support=agent_contract.stream_support.as_attr()
            data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()
            data-ui-output-status=agent_contract.output_status.as_attr()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Empty(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional, into)] dir: Option<String>,
    children: Children,
) -> impl IntoView {
    let (class_name, state) = logic::normalize_part(EmptySlot::Root, class_name, None);
    render_part(class_name, state, lang, dir, None, children)
}

#[component]
pub fn EmptyHeader(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (class_name, state) = logic::normalize_part(EmptySlot::Header, class_name, None);
    render_part(class_name, state, None, None, None, children)
}

#[component]
pub fn EmptyTitle(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (class_name, state) = logic::normalize_part(EmptySlot::Title, class_name, None);
    render_part(class_name, state, None, None, None, children)
}

#[component]
pub fn EmptyDescription(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (class_name, state) = logic::normalize_part(EmptySlot::Description, class_name, None);
    render_part(class_name, state, None, None, None, children)
}

#[component]
pub fn EmptyContent(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (class_name, state) = logic::normalize_part(EmptySlot::Content, class_name, None);
    render_part(class_name, state, None, None, None, children)
}

#[component]
pub fn EmptyMedia(
    #[prop(optional, into)] variant: Option<logic::EmptyMediaVariant>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (class_name, state) = logic::normalize_part(EmptySlot::Media, class_name, variant);
    render_part(
        class_name,
        state,
        None,
        None,
        Some(state.media_variant_attr),
        children,
    )
}
