use leptos::prelude::*;

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
pub fn Breadcrumb(
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let aria_label =
        normalize_optional_text(aria_label).unwrap_or_else(|| "breadcrumb".to_string());
    let class_name = compose_class("ui-breadcrumb", class_name);

    view! {
        <nav class=class_name aria-label=aria_label data-slot="breadcrumb">
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbList(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-breadcrumb__list", class_name);

    view! {
        <ol class=class_name data-slot="breadcrumb-list">
            {children()}
        </ol>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-breadcrumb__item", class_name);

    view! {
        <li class=class_name data-slot="breadcrumb-item">
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbLink(
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-breadcrumb__link", class_name);

    view! {
        <a class=class_name href=href data-slot="breadcrumb-link">
            {children()}
        </a>
    }
}

#[component]
pub fn BreadcrumbPage(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = compose_class("ui-breadcrumb__page", class_name);

    view! {
        <span
            class=class_name
            role="link"
            aria-disabled="true"
            aria-current="page"
            data-slot="breadcrumb-page"
        >
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class_name = compose_class("ui-breadcrumb__separator", class_name);

    view! {
        <li
            class=class_name
            role="presentation"
            aria-hidden="true"
            data-slot="breadcrumb-separator"
        >
            {children
                .map(|children| children())
                .unwrap_or_else(|| view! { <span>/</span> }.into_any())}
        </li>
    }
}

#[component]
pub fn BreadcrumbEllipsis(#[prop(optional, into)] class_name: Option<String>) -> impl IntoView {
    let class_name = compose_class("ui-breadcrumb__ellipsis", class_name);

    view! {
        <span class=class_name role="presentation" aria-hidden="true" data-slot="breadcrumb-ellipsis">
            <span data-slot="breadcrumb-ellipsis-content">"…"</span>
            <span class="sr-only">"More"</span>
        </span>
    }
}
