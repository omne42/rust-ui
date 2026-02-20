use crate::{
    BreadcrumbItem,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn Breadcrumb(
    items: Vec<BreadcrumbItem>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let logic::BreadcrumbRootState {
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
            data-slot="breadcrumb"
            aria-label=aria_label
            data-aria-source=aria_source_attr
            data-class-source=class_source_attr
            data-empty=state.is_empty.then_some("true")
            data-has-items=state.has_items.then_some("true")
            data-has-links=state.has_links.then_some("true")
            data-has-current-page=state.has_current_page.then_some("true")
            data-count=state.item_count
        >
            <ol class="ui-breadcrumb__list" data-slot="breadcrumb-list">
                {items
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| {
                        let is_last = index == last_index;
                        let href = logic::resolve_item_href(&item, is_last);

                        let href_for_attr = href.clone();

                        let separator = (!is_last).then_some(view! {
                            <span class="ui-breadcrumb__separator" data-slot="breadcrumb-separator" aria-hidden="true">
                                "/"
                            </span>
                        });

                        let content: AnyView = if is_last {
                            view! {
                                <span
                                    class="ui-breadcrumb__page"
                                    data-slot="breadcrumb-page"
                                    aria-current="page"
                                >
                                    {item.label}
                                </span>
                            }
                            .into_any()
                        } else if let Some(href) = href {
                            view! {
                                <a class="ui-breadcrumb__link" data-slot="breadcrumb-link" href=href>
                                    {item.label}
                                </a>
                            }
                            .into_any()
                        } else {
                            view! {
                                <span class="ui-breadcrumb__label" data-slot="breadcrumb-label">
                                    {item.label}
                                </span>
                            }
                            .into_any()
                        };

                        view! {
                            <li
                                class="ui-breadcrumb__item"
                                data-slot="breadcrumb-item"
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
