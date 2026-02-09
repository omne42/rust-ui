use crate::active_highlight::{ActiveHighlightMotion, attach_active_highlight_motion};
use crate::carousel::{CarouselItem, CarouselOrientation, logic};
use crate::overlay_open;
use leptos::{ev, prelude::*};
use std::sync::Arc;

#[component]
pub fn Carousel(
    id_base: String,
    items: Vec<CarouselItem>,
    #[prop(optional)] selected_index: Option<Signal<Option<usize>>>,
    #[prop(optional)] default_selected_index: Option<usize>,
    #[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>,
    #[prop(optional)] orientation: CarouselOrientation,
    #[prop(default = true)] loop_navigation: bool,
    #[prop(optional)] motion: ActiveHighlightMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);

    let items = logic::resolve_items(&id_base, items);
    let items: StoredValue<Arc<[logic::CarouselItemResolved]>> = StoredValue::new(Arc::from(items));
    let item_count = items.get_value().len();

    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let default_selected_index = logic::resolve_initial_selected_index(
        items.get_value().as_ref(),
        logic::sanitize_index(default_selected_index, item_count),
    );

    let selected_state = overlay_open::use_controllable_state(
        selected_index,
        Some(default_selected_index),
        on_selected_index_change,
    );

    let selected_index = Signal::derive(move || {
        logic::sanitize_selected_index(selected_state.value.get(), items.get_value().as_ref())
            .or_else(|| logic::first_enabled_index(items.get_value().as_ref()))
    });

    let (focused_index_raw, set_focused_index_raw) = signal(logic::resolve_initial_focused_index(
        items.get_value().as_ref(),
        selected_index.get_untracked(),
    ));

    let focused_index = Signal::derive(move || {
        logic::sanitize_focused_index(focused_index_raw.get(), items.get_value().as_ref())
            .or(selected_index.get())
    });

    let state = Signal::derive(move || {
        logic::resolve_state(logic::CarouselStateInput {
            item_count,
            selected_index: selected_index.get(),
            focused_index: focused_index.get(),
            has_disabled_items: items.get_value().iter().any(|item| item.disabled),
            has_custom_aria_label,
            has_custom_class_name,
            orientation,
            loop_navigation,
        })
    });

    let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));

    let request_selected_index_change = Callback::new(move |next: Option<usize>| {
        let next = logic::sanitize_selected_index(next, items.get_value().as_ref());
        selected_state.request_change.run(next);
    });

    let step_selection = Callback::new(move |step: isize| {
        let items_ref = items.get_value();
        if items_ref.is_empty() {
            return;
        }

        let current_index = selected_index
            .get_untracked()
            .or_else(|| logic::first_enabled_index(items_ref.as_ref()));

        let next_index = if let Some(current_index) = current_index {
            logic::adjacent_enabled_index(items_ref.as_ref(), current_index, step, loop_navigation)
        } else if step > 0 {
            logic::first_enabled_index(items_ref.as_ref())
        } else {
            logic::last_enabled_index(items_ref.as_ref())
        };

        if let Some(next_index) = next_index {
            set_focused_index_raw.set(Some(next_index));
            request_selected_index_change.run(Some(next_index));
        }
    });

    let select_edge = Callback::new(move |pick_last: bool| {
        let next_index = if pick_last {
            logic::last_enabled_index(items.get_value().as_ref())
        } else {
            logic::first_enabled_index(items.get_value().as_ref())
        };

        if let Some(next_index) = next_index {
            set_focused_index_raw.set(Some(next_index));
            request_selected_index_change.run(Some(next_index));
        }
    });

    let prev_key = orientation.prev_key().to_string();
    let next_key = orientation.next_key().to_string();

    let on_key_down = move |ev: ev::KeyboardEvent| {
        let key = ev.key();

        if key == prev_key {
            step_selection.run(-1);
            ev.prevent_default();
            return;
        }

        if key == next_key {
            step_selection.run(1);
            ev.prevent_default();
            return;
        }

        match key.as_str() {
            "Home" => {
                select_edge.run(false);
                ev.prevent_default();
            }
            "End" => {
                select_edge.run(true);
                ev.prevent_default();
            }
            _ => {}
        }
    };

    let can_prev = Signal::derive(move || {
        let items_ref = items.get_value();
        let current_index = selected_index
            .get()
            .or_else(|| logic::first_enabled_index(items_ref.as_ref()));

        let Some(current_index) = current_index else {
            return false;
        };

        logic::adjacent_enabled_index(items_ref.as_ref(), current_index, -1, loop_navigation)
            .is_some()
    });

    let can_next = Signal::derive(move || {
        let items_ref = items.get_value();
        let current_index = selected_index
            .get()
            .or_else(|| logic::first_enabled_index(items_ref.as_ref()));

        let Some(current_index) = current_index else {
            return false;
        };

        logic::adjacent_enabled_index(items_ref.as_ref(), current_index, 1, loop_navigation)
            .is_some()
    });

    let on_prev = move |_| step_selection.run(-1);
    let on_next = move |_| step_selection.run(1);

    let indicator_indices: StoredValue<Vec<usize>> = StoredValue::new((0..item_count).collect());

    let indicator_list_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let indicator_highlight_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    let (active_index, set_active_index) = signal(selected_index.get_untracked().unwrap_or(0));
    Effect::new(move |_| {
        let next = selected_index
            .get()
            .or(focused_index.get())
            .or_else(|| logic::first_enabled_index(items.get_value().as_ref()))
            .unwrap_or(0);
        set_active_index.set(next);
    });

    let option_id = Callback::new(move |index: usize| {
        items
            .get_value()
            .get(index)
            .map(|item| item.dot_dom_id.clone())
            .unwrap_or_default()
    });

    attach_active_highlight_motion(
        indicator_list_ref,
        indicator_highlight_ref,
        active_index,
        option_id,
        motion,
    );

    let render_slide = move |index: usize| {
        let item = items.get_value()[index].clone();

        let item_title = StoredValue::new(item.title);
        let item_description = StoredValue::new(item.description);
        let item_has_description = item_description.get_value().is_some();
        let item_slide_dom_id = StoredValue::new(item.slide_dom_id);
        let item_disabled = item.disabled;

        view! {
            <article
                id=item_slide_dom_id.get_value()
                class="ui-carousel__slide"
                role="group"
                aria-roledescription="slide"
                aria-hidden=move || {
                    if selected_index.get() == Some(index) {
                        "false"
                    } else {
                        "true"
                    }
                }
                data-slot="carousel-item"
                data-index=index
                data-selected=move || (selected_index.get() == Some(index)).then_some("true")
                data-focused=move || (focused_index.get() == Some(index)).then_some("true")
                data-disabled=item_disabled.then_some("true")
            >
                <h3 class="ui-carousel__title" data-slot="carousel-title">
                    {item_title.get_value()}
                </h3>
                <Show when=move || item_has_description>
                    <p class="ui-carousel__description" data-slot="carousel-description">
                        {item_description.get_value().unwrap_or_default()}
                    </p>
                </Show>
            </article>
        }
    };

    let render_indicator = move |index: usize| {
        let item = items.get_value()[index].clone();

        let item_dot_dom_id = StoredValue::new(item.dot_dom_id);
        let item_title = StoredValue::new(item.title);
        let item_disabled = item.disabled;

        let on_click = move |_| {
            if item_disabled {
                return;
            }
            set_focused_index_raw.set(Some(index));
            request_selected_index_change.run(Some(index));
        };

        let on_focus = move |_| {
            if item_disabled {
                return;
            }
            set_focused_index_raw.set(Some(index));
        };

        view! {
            <button
                type="button"
                id=item_dot_dom_id.get_value()
                class="ui-carousel__indicator"
                aria-label=move || format!("Go to {}", item_title.get_value())
                disabled=item_disabled
                data-slot="carousel-indicator"
                data-index=index
                data-selected=move || (selected_index.get() == Some(index)).then_some("true")
                data-focused=move || (focused_index.get() == Some(index)).then_some("true")
                data-disabled=item_disabled.then_some("true")
                on:click=on_click
                on:focus=on_focus
            >
                <span class="ui-carousel__indicator-dot" data-slot="carousel-indicator-dot"></span>
            </button>
        }
    };

    view! {
        <section
            class=move || class.get()
            role="region"
            tabindex="0"
            aria-label=aria_label
            data-slot="carousel"
            data-state=move || state.get().data_state_attr
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-item-count=move || state.get().item_count.to_string()
            data-selected-index=move || state.get().selected_index.map(|index| index.to_string())
            data-focused-index=move || state.get().focused_index.map(|index| index.to_string())
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-has-focus=move || state.get().has_focus.then_some("true")
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-custom-label=has_custom_aria_label.then_some("true")
            data-custom-class=has_custom_class_name.then_some("true")
            data-orientation=orientation.attr()
            data-loop=loop_navigation.then_some("true")
            on:keydown=on_key_down
        >
            <div class="ui-carousel__viewport" data-slot="carousel-viewport">
                <For each=move || indicator_indices.get_value() key=|index| *index children=render_slide />
            </div>

            <div class="ui-carousel__controls" data-slot="carousel-controls">
                <button
                    type="button"
                    class="ui-carousel__button ui-carousel__button--prev"
                    data-slot="carousel-prev"
                    disabled=move || !can_prev.get()
                    on:click=on_prev
                >
                    "Previous"
                </button>
                <button
                    type="button"
                    class="ui-carousel__button ui-carousel__button--next"
                    data-slot="carousel-next"
                    disabled=move || !can_next.get()
                    on:click=on_next
                >
                    "Next"
                </button>
            </div>

            <div
                class="ui-carousel__indicators"
                node_ref=indicator_list_ref
                data-slot="carousel-indicators"
            >
                <div
                    class="ui-active-highlight"
                    node_ref=indicator_highlight_ref
                    data-slot="carousel-indicator-highlight"
                ></div>
                <For each=move || indicator_indices.get_value() key=|index| *index children=render_indicator />
            </div>
        </section>
    }
}
