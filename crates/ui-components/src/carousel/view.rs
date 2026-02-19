use crate::carousel::{
    CarouselItem, CarouselItemResolved, CarouselMotion, CarouselOrientation,
    CarouselPartStateInput, CarouselSlot, logic,
};
use leptos::{ev, html, prelude::*};
use std::sync::Arc;
use ui_headless as overlay_open;
use ui_visual_primitive::active_highlight::{
    ActiveHighlightMotion, attach_active_highlight_motion,
};

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
    let motion = crate::carousel::motion::sanitize_motion(motion);

    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;
    let id_base = StoredValue::new(id_base);

    let items = logic::resolve_items(&id_base.get_value(), items);
    let items: StoredValue<Arc<[CarouselItemResolved]>> = StoredValue::new(Arc::from(items));
    let item_count = items.get_value().len();
    let has_disabled_items = items.get_value().iter().any(|item| item.disabled);

    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let has_custom_orientation = orientation != logic::DEFAULT_ORIENTATION;
    let has_custom_loop_navigation = loop_navigation != logic::DEFAULT_LOOP_NAVIGATION;
    let has_custom_selected_index = selected_index.is_some();
    let has_custom_default_selected_index = default_selected_index.is_some();
    let has_custom_on_selected_index_change = on_selected_index_change.is_some();
    let has_custom_motion = motion != CarouselMotion::default();

    let default_selected_index = logic::resolve_initial_selected_index(
        items.get_value().as_ref(),
        logic::sanitize_index(default_selected_index, item_count),
    );

    let is_controlled = has_custom_selected_index;
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

    let root_state = Memo::new(move |_| {
        logic::resolve_state(CarouselPartStateInput {
            slot: CarouselSlot::Root,
            item_count,
            selected_index: selected_index.get(),
            focused_index: focused_index.get(),
            has_disabled_items,
            orientation,
            loop_navigation,
            is_controlled,
            has_custom_id_base,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_orientation,
            has_custom_loop_navigation,
            has_custom_selected_index,
            has_custom_default_selected_index,
            has_custom_on_selected_index_change,
            has_custom_motion,
        })
    });
    let root_state_for_class = root_state;

    let root_class = Memo::new(move |_| {
        logic::compose_class_name(class_name.get_value(), root_state_for_class.get())
    });

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
        let current_index = selected_index
            .get()
            .or_else(|| logic::first_enabled_index(items.get_value().as_ref()));

        let Some(current_index) = current_index else {
            return false;
        };

        logic::adjacent_enabled_index(
            items.get_value().as_ref(),
            current_index,
            -1,
            loop_navigation,
        )
        .is_some()
    });

    let can_next = Signal::derive(move || {
        let current_index = selected_index
            .get()
            .or_else(|| logic::first_enabled_index(items.get_value().as_ref()));

        let Some(current_index) = current_index else {
            return false;
        };

        logic::adjacent_enabled_index(
            items.get_value().as_ref(),
            current_index,
            1,
            loop_navigation,
        )
        .is_some()
    });

    let on_prev = move |_| step_selection.run(-1);
    let on_next = move |_| step_selection.run(1);

    let indicator_indices: StoredValue<Vec<usize>> = StoredValue::new((0..item_count).collect());

    let indicator_list_ref: NodeRef<html::Div> = NodeRef::new();
    let indicator_highlight_ref: NodeRef<html::Div> = NodeRef::new();

    let (active_index, set_active_index) = signal(
        selected_index
            .get_untracked()
            .or_else(|| logic::first_enabled_index(items.get_value().as_ref()))
            .unwrap_or(0),
    );
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

        let item_slot = CarouselSlot::Item;
        let title_slot = CarouselSlot::Title;
        let description_slot = CarouselSlot::Description;

        view! {
            <article
                id=item_slide_dom_id.get_value()
                class=item_slot.base_class()
                role="group"
                aria-roledescription="slide"
                aria-hidden=move || {
                    if selected_index.get() == Some(index) {
                        "false"
                    } else {
                        "true"
                    }
                }
                data-slot=item_slot.as_attr()
                data-index=index
                data-state=move || {
                    if item_disabled {
                        "disabled"
                    } else if selected_index.get() == Some(index) {
                        "selected"
                    } else if focused_index.get() == Some(index) {
                        "focused"
                    } else {
                        "idle"
                    }
                }
                data-selected=move || (selected_index.get() == Some(index)).then_some("true")
                data-focused=move || (focused_index.get() == Some(index)).then_some("true")
                data-disabled=item_disabled.then_some("true")
            >
                <h3 class=title_slot.base_class() data-slot=title_slot.as_attr()>
                    {item_title.get_value()}
                </h3>
                <Show when=move || item_has_description>
                    <p class=description_slot.base_class() data-slot=description_slot.as_attr()>
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

        let indicator_slot = CarouselSlot::Indicator;
        let indicator_dot_slot = CarouselSlot::IndicatorDot;

        view! {
            <button
                type="button"
                id=item_dot_dom_id.get_value()
                class=indicator_slot.base_class()
                aria-label=move || format!("Go to {}", item_title.get_value())
                disabled=item_disabled
                data-slot=indicator_slot.as_attr()
                data-index=index
                data-state=move || {
                    if item_disabled {
                        "disabled"
                    } else if selected_index.get() == Some(index) {
                        "selected"
                    } else if focused_index.get() == Some(index) {
                        "focused"
                    } else {
                        "idle"
                    }
                }
                data-selected=move || (selected_index.get() == Some(index)).then_some("true")
                data-focused=move || (focused_index.get() == Some(index)).then_some("true")
                data-disabled=item_disabled.then_some("true")
                on:click=on_click
                on:focus=on_focus
            >
                <span class=indicator_dot_slot.base_class() data-slot=indicator_dot_slot.as_attr()></span>
            </button>
        }
    };

    let viewport_slot = CarouselSlot::Viewport;
    let controls_slot = CarouselSlot::Controls;
    let prev_slot = CarouselSlot::PrevButton;
    let next_slot = CarouselSlot::NextButton;
    let indicators_slot = CarouselSlot::Indicators;
    let highlight_slot = CarouselSlot::IndicatorHighlight;

    view! {
        <section
            class=move || root_class.get()
            role="region"
            tabindex="0"
            aria-label=aria_label.get_value()
            data-slot=move || root_state.get().slot_attr
            data-state=move || root_state.get().state_attr
            data-items=move || root_state.get().item_attr
            data-selection=move || root_state.get().selected_attr
            data-focus=move || root_state.get().focus_attr
            data-empty=move || root_state.get().is_empty.then_some("true")
            data-has-items=move || root_state.get().has_items.then_some("true")
            data-item-count=move || root_state.get().item_count
            data-selected-index=move || root_state.get().selected_index
            data-focused-index=move || root_state.get().focused_index
            data-has-selection=move || root_state.get().has_selection.then_some("true")
            data-has-focus=move || root_state.get().has_focus.then_some("true")
            data-has-disabled-items=move || root_state.get().has_disabled_items.then_some("true")
            data-orientation=move || root_state.get().orientation_attr
            data-navigation-mode=move || root_state.get().navigation_attr
            data-selection-mode=move || root_state.get().selection_mode_attr
            data-loop=move || root_state.get().loop_attr
            data-bounded=move || root_state.get().bounded_attr
            data-id-source=move || root_state.get().id_source_attr
            data-aria-label-source=move || root_state.get().aria_label_source_attr
            data-class-source=move || root_state.get().class_source_attr
            data-orientation-source=move || root_state.get().orientation_source_attr
            data-loop-navigation-source=move || root_state.get().loop_navigation_source_attr
            data-selected-index-source=move || root_state.get().selected_index_source_attr
            data-default-selected-index-source=move || root_state.get().default_selected_index_source_attr
            data-selected-index-change-source=move || root_state.get().selected_index_change_source_attr
            data-motion-source=move || root_state.get().motion_source_attr
            data-custom-id=move || root_state.get().has_custom_id_base.then_some("true")
            data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some("true")
            data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
            data-custom-orientation=move || root_state.get().has_custom_orientation.then_some("true")
            data-custom-loop-navigation=move || {
                root_state.get().has_custom_loop_navigation.then_some("true")
            }
            data-custom-selected-index=move || root_state.get().has_custom_selected_index.then_some("true")
            data-custom-default-selected-index=move || {
                root_state.get().has_custom_default_selected_index.then_some("true")
            }
            data-custom-selected-index-change=move || {
                root_state.get().has_custom_on_selected_index_change.then_some("true")
            }
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
            on:keydown=on_key_down
        >
            <div class=viewport_slot.base_class() data-slot=viewport_slot.as_attr()>
                <For each=move || indicator_indices.get_value() key=|index| *index children=render_slide />
            </div>

            <div class=controls_slot.base_class() data-slot=controls_slot.as_attr()>
                <button
                    type="button"
                    class=prev_slot.base_class()
                    data-slot=prev_slot.as_attr()
                    disabled=move || !can_prev.get()
                    on:click=on_prev
                >
                    "Previous"
                </button>
                <button
                    type="button"
                    class=next_slot.base_class()
                    data-slot=next_slot.as_attr()
                    disabled=move || !can_next.get()
                    on:click=on_next
                >
                    "Next"
                </button>
            </div>

            <div
                class=indicators_slot.base_class()
                node_ref=indicator_list_ref
                data-slot=indicators_slot.as_attr()
            >
                <div
                    class=highlight_slot.base_class()
                    node_ref=indicator_highlight_ref
                    data-slot=highlight_slot.as_attr()
                ></div>
                <For each=move || indicator_indices.get_value() key=|index| *index children=render_indicator />
            </div>
        </section>
    }
}
