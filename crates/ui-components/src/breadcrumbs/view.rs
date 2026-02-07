use crate::breadcrumbs::{BreadcrumbItem, logic};
use leptos::prelude::*;

#[component]
pub fn Breadcrumbs(
    items: Vec<BreadcrumbItem>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Breadcrumb".to_string());

    let base_class = "ui-breadcrumbs".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let state = logic::resolve_state(&items);
    let last_index = items.len().saturating_sub(1);

    view! {
        <nav
            class=class
            data-slot="breadcrumbs"
            aria-label=aria_label
            data-empty=state.is_empty.then_some("true")
            data-has-items=state.has_items.then_some("true")
            data-has-links=state.has_links.then_some("true")
            data-has-current-page=state.has_current_page.then_some("true")
            data-count=state.item_count.to_string()
        >
            <ol class="ui-breadcrumbs__list" data-slot="breadcrumbs-list">
                {items
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| {
                        let is_last = index == last_index;
                        let href = (!is_last)
                            .then_some(())
                            .and(item.href)
                            .and_then(|href| {
                                let trimmed = href.trim();
                                (!trimmed.is_empty()).then(|| trimmed.to_string())
                            });

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
