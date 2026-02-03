use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    pagination::{PaginationItem, resolve_pagination_range},
};
use leptos::prelude::*;
use ui_headless::OnPress;

#[component]
pub fn Pagination(
    total_pages: usize,
    page: ReadSignal<usize>,
    set_page: WriteSignal<usize>,
    #[prop(optional)] siblings: usize,
    #[prop(optional)] boundaries: usize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<Callback<usize>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let on_change = StoredValue::new(on_change);

    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Pagination".to_string());

    let base_class = "ui-pagination".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let page_items =
        move || resolve_pagination_range(total_pages, page.get(), siblings, boundaries);

    let prev_on_press: OnPress = Callback::new(move |_| {
        let current = page.get_untracked().max(1);
        let next = current.saturating_sub(1).max(1);
        set_page.set(next);
        if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    });

    let next_on_press: OnPress = Callback::new(move |_| {
        let current = page.get_untracked().max(1);
        let next = (current + 1).min(total_pages.max(1));
        set_page.set(next);
        if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    });

    view! {
        <nav class=class aria-label=aria_label data-slot="pagination">
            <ul class="ui-pagination__list" data-slot="pagination-list">
                {move || {
                    view! {
                        <li class="ui-pagination__item" data-slot="pagination-prev">
                            <Button
                                disabled=disabled || page.get() <= 1
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconSm
                                aria_label="Previous page"
                                on_press=prev_on_press
                            >
                                "‹"
                            </Button>
                        </li>
                    }
                }}

                {move || {
                    page_items()
                        .into_iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let aria_current = move || match item {
                                PaginationItem::Page(p) if p == page.get() => Some("page"),
                                _ => None,
                            };

                            let content: AnyView = match item {
                                PaginationItem::Dots => view! {
                                    <span class="ui-pagination__dots" aria-hidden="true">"…"</span>
                                }
                                .into_any(),
                                PaginationItem::Page(p) => {
                                    let on_press: OnPress = Callback::new(move |_| {
                                        if disabled {
                                            return;
                                        }
                                        set_page.set(p);
                                        if let Some(on_change) = on_change.get_value() {
                                            on_change.run(p);
                                        }
                                    });

                                    view! {
                                        <Button
                                            disabled=disabled
                                            variant=ButtonVariant::Ghost
                                            size=ButtonSize::Sm
                                            on_press=on_press
                                        >
                                            {p}
                                        </Button>
                                    }
                                    .into_any()
                                }
                            };

                            view! {
                                <li
                                    class="ui-pagination__item"
                                    aria-current=aria_current
                                    data-index=index
                                >
                                    {content}
                                </li>
                            }
                        })
                        .collect_view()
                }}

                {move || {
                    view! {
                        <li class="ui-pagination__item" data-slot="pagination-next">
                            <Button
                                disabled=disabled || total_pages.max(1) <= page.get()
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconSm
                                aria_label="Next page"
                                on_press=next_on_press
                            >
                                "›"
                            </Button>
                        </li>
                    }
                }}
            </ul>
        </nav>
    }
}
