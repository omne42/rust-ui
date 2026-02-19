use crate::breadcrumbs::{BreadcrumbItem, logic};
use leptos::prelude::*;

#[component]
pub fn Breadcrumb(
    items: Vec<BreadcrumbItem>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    match (aria_label, class_name) {
        (Some(aria_label), Some(class_name)) => {
            view! { <Breadcrumbs items=items aria_label=aria_label class_name=class_name /> }
                .into_any()
        }
        (Some(aria_label), None) => {
            view! { <Breadcrumbs items=items aria_label=aria_label /> }.into_any()
        }
        (None, Some(class_name)) => {
            view! { <Breadcrumbs items=items class_name=class_name /> }.into_any()
        }
        (None, None) => view! { <Breadcrumbs items=items /> }.into_any(),
    }
}

#[component]
pub fn Breadcrumbs(
    items: Vec<BreadcrumbItem>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let logic::BreadcrumbsRootState {
        aria_label,
        aria_source_attr,
        class_name,
        class_source_attr,
    } = logic::resolve_root_state(aria_label, class_name);
    let state = logic::resolve_state(&items);
    let last_index = items.len().saturating_sub(1);

    view! {
        <nav
            class=class_name
            data-slot="breadcrumbs"
            aria-label=aria_label
            data-aria-source=aria_source_attr
            data-class-source=class_source_attr
            data-empty=state.is_empty.then_some("true")
            data-has-items=state.has_items.then_some("true")
            data-has-links=state.has_links.then_some("true")
            data-has-current-page=state.has_current_page.then_some("true")
            data-count=state.item_count
        >
            <ol class="ui-breadcrumbs__list" data-slot="breadcrumbs-list">
                {items
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| {
                        let is_last = index == last_index;
                        let href = logic::resolve_item_href(&item, is_last);

                        let href_for_attr = href.clone();

                        let separator = (!is_last).then_some(view! {
                            <span class="ui-breadcrumbs__separator" data-slot="breadcrumbs-separator" aria-hidden="true">
                                "/"
                            </span>
                        });

                        let content: AnyView = if is_last {
                            view! {
                                <span
                                    class="ui-breadcrumbs__current"
                                    data-slot="breadcrumbs-current"
                                    aria-current="page"
                                >
                                    {item.label}
                                </span>
                            }
                            .into_any()
                        } else if let Some(href) = href {
                            view! {
                                <a class="ui-breadcrumbs__link" data-slot="breadcrumbs-link" href=href>
                                    {item.label}
                                </a>
                            }
                            .into_any()
                        } else {
                            view! {
                                <span class="ui-breadcrumbs__current" data-slot="breadcrumbs-label">
                                    {item.label}
                                </span>
                            }
                            .into_any()
                        };

                        view! {
                            <li
                                class="ui-breadcrumbs__item"
                                data-slot="breadcrumbs-item"
                                data-index=index
                                data-last=is_last.then_some("true")
                                data-href=href_for_attr
                            >
                                {content}
                                {separator}
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </nav>
    }
}
