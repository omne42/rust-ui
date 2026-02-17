use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    pagination::{PaginationItem, PaginationStrings, logic, resolve_pagination_range},
};
use leptos::prelude::*;
use ui_headless::OnPress;
use ui_headless::i18n;

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
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<PaginationStrings>();
    let on_change = StoredValue::new(on_change);

    let aria_label = logic::normalize_aria_label(aria_label, strings.aria_label.as_ref());

    let base_class = "ui-pagination".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let state = Signal::derive(move || {
        let current = page.get();
        logic::resolve_pagination_state(total_pages, current, disabled)
    });

    let page_items = move || {
        let current = state.get().current_page;
        resolve_pagination_range(total_pages, current, siblings, boundaries)
    };

    let prev_on_press: OnPress = Callback::new(move |_| {
        let state = logic::resolve_pagination_state(total_pages, page.get_untracked(), disabled);
        if state.is_prev_disabled {
            return;
        }

        let next = state.current_page.saturating_sub(1).max(1);
        if next == state.current_page {
            return;
        }

        set_page.set(next);
        if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    });

    let next_on_press: OnPress = Callback::new(move |_| {
        let state = logic::resolve_pagination_state(total_pages, page.get_untracked(), disabled);
        if state.is_next_disabled {
            return;
        }

        let next = (state.current_page + 1).min(state.effective_total_pages);
        if next == state.current_page {
            return;
        }

        set_page.set(next);
        if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    });

    let prev_page_label = strings.previous_page_aria_label.as_ref().to_string();
    let next_page_label = strings.next_page_aria_label.as_ref().to_string();

    view! {
        <nav
            class=class
            aria-label=aria_label
            data-slot="pagination"
            data-disabled=disabled.then_some("true")
            data-empty=(total_pages == 0).then_some("true")
            data-page=move || state.get().current_page.to_string()
            data-total-pages=total_pages.to_string()
            data-single-page=move || {
                (state.get().effective_total_pages <= 1).then_some("true")
            }
        >
            <ul class="ui-pagination__list" data-slot="pagination-list">
                {move || {
                    let is_prev_disabled = state.get().is_prev_disabled;
                    view! {
                        <li
                            class="ui-pagination__item"
                            data-slot="pagination-prev"
                            data-disabled=is_prev_disabled.then_some("true")
                        >
                            <Button
                                is_disabled=is_prev_disabled
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconSm
                                aria_label=prev_page_label.clone()
                                on_press=prev_on_press
                            >
                                "‹"
                            </Button>
                        </li>
                    }
                }}

                {move || {
                    let current_page = state.get().current_page;
                    page_items()
                        .into_iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let page_number = match item {
                                PaginationItem::Page(value) => Some(value),
                                PaginationItem::Dots => None,
                            };
                            let slot = if page_number.is_some() {
                                "pagination-page"
                            } else {
                                "pagination-dots"
                            };
                            let is_current = move || page_number == Some(current_page);
                            let aria_current = move || is_current().then_some("page");

                            let content: AnyView = match item {
                                PaginationItem::Dots => view! {
                                    <span class="ui-pagination__dots" data-slot="pagination-dots-label" aria-hidden="true">"…"</span>
                                }
                                .into_any(),
                                PaginationItem::Page(p) => {
                                    let on_press: OnPress = Callback::new(move |_| {
                                        if disabled {
                                            return;
                                        }

                                        let current = logic::resolve_pagination_state(
                                            total_pages,
                                            page.get_untracked(),
                                            disabled,
                                        )
                                        .current_page;
                                        if current == p {
                                            return;
                                        }

                                        set_page.set(p);
                                        if let Some(on_change) = on_change.get_value() {
                                            on_change.run(p);
                                        }
                                    });

                                    view! {
                                        <Button
                                            is_disabled=disabled
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
                                    data-slot=slot
                                    data-index=index
                                    data-page=page_number.map(|value| value.to_string())
                                    data-current=move || is_current().then_some("true")
                                >
                                    {content}
                                </li>
                            }
                        })
                        .collect_view()
                }}

                {move || {
                    let is_next_disabled = state.get().is_next_disabled;
                    view! {
                        <li
                            class="ui-pagination__item"
                            data-slot="pagination-next"
                            data-disabled=is_next_disabled.then_some("true")
                        >
                            <Button
                                is_disabled=is_next_disabled
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconSm
                                aria_label=next_page_label.clone()
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
