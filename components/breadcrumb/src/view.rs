use crate::{
    BreadcrumbItem,
    logic::{self},
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, navigation_attrs};
use ui_headless::{CommonStrings, use_ui_i18n};

fn render_breadcrumb_separator(separator_text: String, is_current_page: bool) -> Option<AnyView> {
    (!is_current_page).then(|| {
        view! {
            <span class="ui-breadcrumb__separator" data-slot="breadcrumb-separator" aria-hidden="true">
                {separator_text}
            </span>
        }
        .into_any()
    })
}

fn render_breadcrumb_item_content(
    label: String,
    href: Option<String>,
    is_current_page: bool,
) -> AnyView {
    if is_current_page {
        view! {
            <span class="ui-breadcrumb__page" data-slot="breadcrumb-page" aria-current="page">
                {label}
            </span>
        }
        .into_any()
    } else if let Some(href) = href {
        view! {
            <a class="ui-breadcrumb__link" data-slot="breadcrumb-link" href=href>
                {label}
            </a>
        }
        .into_any()
    } else {
        view! {
            <span class="ui-breadcrumb__label" data-slot="breadcrumb-label">
                {label}
            </span>
        }
        .into_any()
    }
}

fn render_breadcrumb_item(
    index: usize,
    item: BreadcrumbItem,
    item_count: usize,
    separator_text: String,
) -> impl IntoView {
    let is_current_page = logic::is_current_page(index, item_count);
    let href = logic::resolve_item_href(&item, index, item_count);
    let href_for_attr = href.clone();
    let separator = render_breadcrumb_separator(separator_text, is_current_page);
    let content = render_breadcrumb_item_content(item.label, href, is_current_page);

    view! {
        <li
            class="ui-breadcrumb__item"
            data-slot="breadcrumb-item"
            data-index=index
            data-last=is_current_page.then_some("true")
            data-href=href_for_attr
        >
            {content}
            {separator}
        </li>
    }
}

fn render_breadcrumb_list(
    items: Vec<BreadcrumbItem>,
    item_count: usize,
    separator_text: String,
) -> impl IntoView {
    let items = items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            render_breadcrumb_item(index, item, item_count, separator_text.clone())
        })
        .collect_view();

    view! {
        <ol class="ui-breadcrumb__list" data-slot="breadcrumb-list">
            {items}
        </ol>
    }
}

#[component]
pub fn Breadcrumb(
    items: Vec<BreadcrumbItem>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] separator: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common_strings = i18n.strings::<CommonStrings>();
    let aria_label_fallback = common_strings.breadcrumb_aria_label.as_ref();
    let separator_fallback = common_strings.breadcrumb_separator.as_ref();
    let logic::BreadcrumbRootState {
        aria_label,
        aria_source_attr,
        class_name,
        class_source_attr,
    } = logic::resolve_root_state(aria_label, Some(aria_label_fallback), class_name);
    let logic::BreadcrumbSeparatorState {
        separator: separator_text,
        separator_source_attr,
    } = logic::resolve_separator(separator, separator_fallback);
    let a11y = navigation_attrs(aria_label, lang, dir);
    let state = logic::resolve_state(&items);
    let agent_contract = logic::resolve_agent_contract(
        &state,
        aria_source_attr,
        class_source_attr,
        separator_source_attr,
    );
    let item_count = items.len();

    view! {
        <nav
            class=class_name
            data-slot="breadcrumb"
            aria-label=a11y.aria_label
            lang=a11y.lang
            dir=a11y.dir
            data-aria-source=aria_source_attr
            data-class-source=class_source_attr
            data-empty=state.is_empty.then_some("true")
            data-has-items=state.has_items.then_some("true")
            data-has-links=state.has_links.then_some("true")
            data-has-current-page=state.has_current_page.then_some("true")
            data-count=state.item_count
            data-separator-source=separator_source_attr
            data-ui-schema=agent_contract.schema_name
            data-ui-schema-version=agent_contract.schema_version.as_str()
            data-ui-intent=agent_contract.intent.as_str()
            data-ui-action=agent_contract.action.as_str()
            data-ui-state=agent_contract.state.as_str()
            data-ui-source=agent_contract.source.as_str()
            data-ui-render-mode=agent_contract.render_mode.as_str()
            data-ui-stream-support=agent_contract.stream_support.as_str()
            data-ui-stream-fallback=agent_contract.stream_fallback.as_str()
            data-ui-output-status=agent_contract.output_status.as_str()
        >
            {render_breadcrumb_list(items, item_count, separator_text.into_owned())}
        </nav>
    }
}
