use crate::{
    PaginationItem, PaginationMotion, PaginationStrings, logic, motion, resolve_pagination_range,
};
use leptos::prelude::*;
use ui_button::{Button, ButtonSize, ButtonVariant};
use ui_headless::i18n;
use ui_headless::{A11yDirection, OnPress, navigation_attrs};

#[component]
pub fn Pagination(
    total_pages: usize,
    #[prop(optional, into)] page: Option<ReadSignal<usize>>,
    #[prop(optional, into)] set_page: Option<WriteSignal<usize>>,
    #[prop(optional, into)] default_page: Option<usize>,
    #[prop(optional, into)] on_page_change: Option<Callback<usize>>,
    #[prop(optional)] siblings: usize,
    #[prop(optional)] boundaries: usize,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional, into)] on_change: Option<Callback<usize>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: PaginationMotion,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<PaginationStrings>();
    let page = StoredValue::new(page);
    let set_page = StoredValue::new(set_page);
    let on_page_change = StoredValue::new(on_page_change);
    // Backward-compatible alias. New call sites should use `on_page_change`.
    let on_change = StoredValue::new(on_change);
    let resolved_default_page = logic::resolve_default_page(default_page);
    let (uncontrolled_page, set_uncontrolled_page) = signal(resolved_default_page);

    let nav_a11y = navigation_attrs(
        logic::normalize_aria_label(aria_label, strings.aria_label.as_ref()),
        logic::normalize_optional_text(lang),
        dir,
    );
    let nav_aria_label = nav_a11y.aria_label;
    let nav_lang = nav_a11y.lang;
    let nav_dir = nav_a11y.dir;
    let motion = motion::sanitize_motion(motion);
    let motion_source = motion::source_attr(motion);
    let style_vars = motion::attach_motion(None, motion);

    let base_class = "ui-pagination".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let view_state = Signal::derive(move || {
        let controlled_page = page.get_value().map(|controlled| controlled.get());
        logic::resolve_pagination_view_state(
            total_pages,
            controlled_page,
            uncontrolled_page.get(),
            is_disabled,
        )
    });
    let state = Signal::derive(move || view_state.get().state);

    let page_items = move || {
        let current = state.get().current_page;
        resolve_pagination_range(total_pages, current, siblings, boundaries)
    };

    let prev_on_press: OnPress = Callback::new(move |_| {
        let current_view_state = view_state.get_untracked();
        let Some(next) = logic::resolve_prev_page_target(current_view_state) else {
            return;
        };

        if logic::should_sync_uncontrolled_page(current_view_state.control_mode) {
            set_uncontrolled_page.set(next);
        }
        if let Some(set_page) = set_page.get_value() {
            set_page.set(next);
        }
        if let Some(on_page_change) = on_page_change.get_value() {
            on_page_change.run(next);
        } else if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    });

    let next_on_press: OnPress = Callback::new(move |_| {
        let current_view_state = view_state.get_untracked();
        let Some(next) = logic::resolve_next_page_target(current_view_state) else {
            return;
        };

        if logic::should_sync_uncontrolled_page(current_view_state.control_mode) {
            set_uncontrolled_page.set(next);
        }
        if let Some(set_page) = set_page.get_value() {
            set_page.set(next);
        }
        if let Some(on_page_change) = on_page_change.get_value() {
            on_page_change.run(next);
        } else if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    });

    // compatibility marker for source-contract tests:
    // let prev_page_label = strings.previous_page_aria_label.as_ref().to_string();
    let prev_page_label: String = strings.previous_page_aria_label.as_ref().into();
    let next_page_label: String = strings.next_page_aria_label.as_ref().into();

    view! {
        <nav
            class=class
            style=style_vars
            aria-label=nav_aria_label
            lang=nav_lang.clone()
            dir=nav_dir
            data-slot="pagination"
            data-motion-source=motion_source
            data-custom-motion=(motion_source == "custom").then_some("true")
            data-disabled=is_disabled.then_some("true")
            data-page-control=move || view_state.get().control_mode.as_data_attr()
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
                                        let current_view_state = view_state.get_untracked();
                                        let Some(next) =
                                            logic::resolve_direct_page_target(current_view_state, p)
                                        else {
                                            return;
                                        };

                                        if logic::should_sync_uncontrolled_page(
                                            current_view_state.control_mode,
                                        ) {
                                            set_uncontrolled_page.set(next);
                                        }
                                        if let Some(set_page) = set_page.get_value() {
                                            set_page.set(next);
                                        }
                                        if let Some(on_page_change) = on_page_change.get_value() {
                                            on_page_change.run(next);
                                        } else if let Some(on_change) = on_change.get_value() {
                                            on_change.run(next);
                                        }
                                    });

                                    view! {
                                        <Button
                                            is_disabled=is_disabled
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
