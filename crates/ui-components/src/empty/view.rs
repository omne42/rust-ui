use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}

impl EmptyMediaVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            EmptyMediaVariant::Default => "default",
            EmptyMediaVariant::Icon => "icon",
        }
    }
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn compose_class(base: &'static str, class_name: Option<String>) -> String {
    normalize_optional_text(class_name)
        .map(|class_name| format!("{base} {class_name}"))
        .unwrap_or_else(|| base.to_string())
}

#[component]
pub fn Empty(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-empty", class_name);

    view! {
        <div class=class_name data-slot="empty">
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyHeader(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-empty__header", class_name);

    view! {
        <div class=class_name data-slot="empty-header">
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyTitle(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-empty__title", class_name);

    view! {
        <div class=class_name data-slot="empty-title">
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyDescription(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-empty__description", class_name);

    view! {
        <div class=class_name data-slot="empty-description">
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyContent(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-empty__content", class_name);

    view! {
        <div class=class_name data-slot="empty-content">
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyMedia(
    #[prop(optional)] variant: EmptyMediaVariant,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-empty__media", class_name);

    view! {
        <div class=class_name data-slot="empty-icon" data-variant=variant.as_attr()>
            {children()}
        </div>
    }
}
