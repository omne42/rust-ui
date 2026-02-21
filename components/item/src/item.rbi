pub type A11yDirection = ui_headless::A11yDirection;
pub type ItemVariant = crate::logic::ItemVariant;
pub type ItemSize = crate::logic::ItemSize;
pub type ItemMediaVariant = crate::logic::ItemMediaVariant;

pub const ITEM_AGENT_SCHEMA: &str = "ui.item.agent-contract.v1";

pub enum ItemAgentIntent {
    CollectionItem,
}

pub enum ItemAgentAction {
    Render,
}

pub enum ItemAgentStreamMode {
    Streaming,
    Snapshot,
}

pub enum ItemAgentOutputMode {
    Snapshot,
}

pub enum ItemStreamingPolicy {
    Optional,
}

pub enum ItemStreamingFallback {
    Snapshot,
}

pub enum ItemOutputStatus {
    Validated,
}

pub fn ItemGroup(
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn ItemSeparator(
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> impl leptos::prelude::IntoView;

pub fn Item(
    variant: Option<ItemVariant>,
    size: Option<ItemSize>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn ItemMedia(
    variant: Option<ItemMediaVariant>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn ItemContent(
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn ItemTitle(
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn ItemDescription(
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn ItemActions(
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn ItemHeader(
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn ItemFooter(
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
