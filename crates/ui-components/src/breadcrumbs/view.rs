use crate::breadcrumbs::BreadcrumbItem;
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

    let last_index = items.len().saturating_sub(1);

    view! {
        <nav class=class data-slot="breadcrumbs" aria-label=aria_label>
            <ol class="ui-breadcrumbs__list" data-slot="breadcrumbs-list">
                {items
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| {
                        let is_last = index == last_index;
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
                        } else if let Some(href) = item.href {
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
                            <li class="ui-breadcrumbs__item" data-slot="breadcrumbs-item">
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
