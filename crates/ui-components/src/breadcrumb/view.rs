use crate::breadcrumb::{
    BreadcrumbLinkStateInput, BreadcrumbRootStateInput, BreadcrumbSeparatorStateInput,
    BreadcrumbSlotStateInput, DEFAULT_ELLIPSIS_LABEL,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn Breadcrumb(
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_root_state(BreadcrumbRootStateInput {
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class_name =
        logic::compose_class_name("ui-breadcrumb", class_name, state.has_custom_class_name);

    view! {
        <nav
            class=class_name
            aria-label=aria_label
            data-slot="breadcrumb"
            data-state=state.state_attr
            data-aria-source=state.aria_source_attr
            data-class-source=state.class_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbList(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_slot_state(BreadcrumbSlotStateInput {
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(
        "ui-breadcrumb__list",
        class_name,
        state.has_custom_class_name,
    );

    view! {
        <ol
            class=class_name
            data-slot="breadcrumb-list"
            data-state=state.state_attr
            data-class-source=state.class_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </ol>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_slot_state(BreadcrumbSlotStateInput {
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(
        "ui-breadcrumb__item",
        class_name,
        state.has_custom_class_name,
    );

    view! {
        <li
            class=class_name
            data-slot="breadcrumb-item"
            data-state=state.state_attr
            data-class-source=state.class_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
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
    let href = logic::normalize_href(href);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_link_state(BreadcrumbLinkStateInput {
        has_href: href.is_some(),
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_link_class_name(class_name, state);

    view! {
        <a
            class=class_name
            href=href
            data-slot="breadcrumb-link"
            data-state=state.state_attr
            data-href-state=state.href_state_attr
            data-interactive=state.interactive.then_some("true")
            data-class-source=state.class_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </a>
    }
}

#[component]
pub fn BreadcrumbPage(
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_slot_state(BreadcrumbSlotStateInput {
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(
        "ui-breadcrumb__page",
        class_name,
        state.has_custom_class_name,
    );

    view! {
        <span
            class=class_name
            aria-current="page"
            data-slot="breadcrumb-page"
            data-state=state.state_attr
            data-class-source=state.class_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
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
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_content = children.is_some();

    let state = logic::resolve_separator_state(BreadcrumbSeparatorStateInput {
        has_custom_content,
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_separator_class_name(class_name, state);

    view! {
        <li
            class=class_name
            role="presentation"
            aria-hidden="true"
            data-slot="breadcrumb-separator"
            data-state=state.state_attr
            data-content-source=state.content_source_attr
            data-class-source=state.class_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children
                .map(|children| children())
                .unwrap_or_else(|| view! { <span data-slot="breadcrumb-separator-default">"/"</span> }.into_any())}
        </li>
    }
}

#[component]
pub fn BreadcrumbEllipsis(#[prop(optional, into)] class_name: Option<String>) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_slot_state(BreadcrumbSlotStateInput {
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = logic::compose_class_name(
        "ui-breadcrumb__ellipsis",
        class_name,
        state.has_custom_class_name,
    );

    view! {
        <span
            class=class_name
            role="presentation"
            aria-hidden="true"
            data-slot="breadcrumb-ellipsis"
            data-state=state.state_attr
            data-label-source="default"
            data-class-source=state.class_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            <span data-slot="breadcrumb-ellipsis-icon" class="ui-breadcrumb__ellipsis-icon">"…"</span>
            <span class="ui-breadcrumb__ellipsis-label">{DEFAULT_ELLIPSIS_LABEL}</span>
        </span>
    }
}
