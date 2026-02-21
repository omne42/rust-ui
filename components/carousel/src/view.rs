use crate::{
    CarouselItem, CarouselItemResolved, CarouselMotion, CarouselOrientation,
    CarouselPartStateInput, CarouselSlot, CarouselStrings, logic,
};
use leptos::{ev, html, prelude::*};
use std::sync::Arc;
use ui_headless::{
    A11yDirection, CarouselA11yOrientation, CarouselKeyCommand, CarouselRootOptions,
    carousel_slide_a11y_attrs, i18n, labeled_group_attrs, labeled_toolbar_attrs, use_carousel_root,
    use_controllable_state,
};
use ui_visual_primitive::active_highlight::ActiveHighlightMotion;

struct CarouselSlideRenderInput {
    index: usize,
    item: CarouselItemResolved,
    selected_index: Signal<Option<usize>>,
    focused_index: Signal<Option<usize>>,
}

fn render_carousel_slide(input: CarouselSlideRenderInput) -> impl IntoView {
    let index = input.index;
    let item_title = StoredValue::new(input.item.title);
    let item_description = StoredValue::new(input.item.description);
    let item_has_description = item_description.get_value().is_some();
    let item_slide_dom_id = StoredValue::new(input.item.slide_dom_id);
    let item_disabled = input.item.disabled;
    let selected_index = input.selected_index;
    let focused_index = input.focused_index;

    let item_state = Signal::derive(move || {
        logic::resolve_item_state_attrs(
            index,
            selected_index.get(),
            focused_index.get(),
            item_disabled,
        )
    });

    let item_slot = CarouselSlot::Item;
    let title_slot = CarouselSlot::Title;
    let description_slot = CarouselSlot::Description;

    view! {
        <article
            id=item_slide_dom_id.get_value()
            class=item_slot.base_class()
            role=carousel_slide_a11y_attrs(false).role
            aria-roledescription=carousel_slide_a11y_attrs(false).aria_roledescription
            aria-hidden=move || carousel_slide_a11y_attrs(item_state.get().is_selected).aria_hidden
            data-slot=item_slot.as_attr()
            data-index=index
            data-state=move || item_state.get().status.as_attr()
            data-selected=move || item_state.get().selected_attr
            data-focused=move || item_state.get().focused_attr
            data-disabled=move || item_state.get().disabled_attr
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
}

struct CarouselIndicatorRenderInput {
    index: usize,
    item: CarouselItemResolved,
    indicator_aria_label_template: String,
    selected_index: Signal<Option<usize>>,
    focused_index: Signal<Option<usize>>,
    set_focused_index: WriteSignal<Option<usize>>,
    request_selected_index_change: Callback<Option<usize>>,
}

fn render_carousel_indicator_dot() -> impl IntoView {
    let indicator_dot_slot = CarouselSlot::IndicatorDot;

    view! {
        <span class=indicator_dot_slot.base_class() data-slot=indicator_dot_slot.as_attr()></span>
    }
}

fn render_carousel_indicator(input: CarouselIndicatorRenderInput) -> impl IntoView {
    let index = input.index;
    let item_dot_dom_id = StoredValue::new(input.item.dot_dom_id);
    let item_title = StoredValue::new(input.item.title);
    let item_indicator_aria_label = StoredValue::new(logic::resolve_indicator_aria_label(
        &input.indicator_aria_label_template,
        item_title.get_value().as_str(),
    ));
    let item_disabled = input.item.disabled;
    let selected_index = input.selected_index;
    let focused_index = input.focused_index;
    let set_focused_index = input.set_focused_index;
    let request_selected_index_change = input.request_selected_index_change;

    let item_state = Signal::derive(move || {
        logic::resolve_item_state_attrs(
            index,
            selected_index.get(),
            focused_index.get(),
            item_disabled,
        )
    });

    let on_click = move |_| {
        if !logic::can_item_receive_selection(item_disabled) {
            return;
        }
        set_focused_index.set(Some(index));
        request_selected_index_change.run(Some(index));
    };

    let on_focus = move |_| {
        if !logic::can_item_receive_selection(item_disabled) {
            return;
        }
        set_focused_index.set(Some(index));
    };

    let indicator_slot = CarouselSlot::Indicator;

    view! {
        <button
            type="button"
            id=item_dot_dom_id.get_value()
            class=indicator_slot.base_class()
            aria-label=item_indicator_aria_label.get_value()
            disabled=item_disabled
            data-slot=indicator_slot.as_attr()
            data-index=index
            data-state=move || item_state.get().status.as_attr()
            data-selected=move || item_state.get().selected_attr
            data-focused=move || item_state.get().focused_attr
            data-disabled=move || item_state.get().disabled_attr
            on:click=on_click
            on:focus=on_focus
        >
            {render_carousel_indicator_dot()}
        </button>
    }
}

#[component]
pub fn Carousel(
    id_base: String,
    items: Vec<CarouselItem>,
    #[prop(optional)] selected_index: Option<Signal<Option<usize>>>,
    #[prop(optional)] default_selected_index: Option<usize>,
    #[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>,
    #[prop(optional)] orientation: CarouselOrientation,
    #[prop(default = true)] is_loop_navigation: bool,
    #[prop(optional)] motion: ActiveHighlightMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] controls_aria_label: Option<String>,
    #[prop(optional, into)] indicators_aria_label: Option<String>,
    #[prop(optional, into)] previous_label: Option<String>,
    #[prop(optional, into)] next_label: Option<String>,
    #[prop(optional, into)] indicator_aria_label_template: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;
    let id_base = StoredValue::new(id_base);

    let items = logic::resolve_items(&id_base.get_value(), items);
    let items: StoredValue<Arc<[CarouselItemResolved]>> = StoredValue::new(Arc::from(items));
    let item_count = items.get_value().len();
    let has_disabled_items = items.get_value().iter().any(|item| item.disabled);

    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<CarouselStrings>();
    let (aria_label, has_custom_aria_label) =
        logic::resolve_aria_label_with_fallback(aria_label, strings.aria_label.as_ref());
    let (controls_aria_label, _) = logic::resolve_label_with_fallback(
        controls_aria_label,
        strings.controls_aria_label.as_ref(),
    );
    let (indicators_aria_label, _) = logic::resolve_label_with_fallback(
        indicators_aria_label,
        strings.indicators_aria_label.as_ref(),
    );
    let (previous_label, _) =
        logic::resolve_label_with_fallback(previous_label, strings.previous_label.as_ref());
    let (next_label, _) =
        logic::resolve_label_with_fallback(next_label, strings.next_label.as_ref());
    let (indicator_aria_label_template, _) = logic::resolve_label_with_fallback(
        indicator_aria_label_template,
        strings.indicator_aria_label_template.as_ref(),
    );
    let lang = logic::normalize_optional_text(lang);
    let lang_for_subgroups = lang.clone();
    let dir_for_subgroups = dir;

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let previous_label = StoredValue::new(previous_label);
    let next_label = StoredValue::new(next_label);
    let indicator_aria_label_template = StoredValue::new(indicator_aria_label_template);

    let has_custom_orientation = orientation != logic::DEFAULT_ORIENTATION;
    let has_custom_loop_navigation = is_loop_navigation != logic::DEFAULT_LOOP_NAVIGATION;
    let has_custom_selected_index = selected_index.is_some();
    let has_custom_default_selected_index = default_selected_index.is_some();
    let has_custom_on_selected_index_change = on_selected_index_change.is_some();
    let has_custom_motion = motion != CarouselMotion::default();

    let default_selected_index =
        logic::resolve_default_selected_index(default_selected_index, items.get_value().as_ref());

    let is_controlled = has_custom_selected_index;
    let selected_state = use_controllable_state(
        selected_index,
        Some(default_selected_index),
        on_selected_index_change,
    );

    let selected_index = Signal::derive(move || {
        logic::resolve_selected_index(selected_state.value.get(), items.get_value().as_ref())
    });

    let (focused_index_raw, set_focused_index_raw) = signal(logic::resolve_initial_focused_index(
        items.get_value().as_ref(),
        selected_index.get_untracked(),
    ));

    let focused_index = Signal::derive(move || {
        logic::resolve_focused_index(
            focused_index_raw.get(),
            selected_index.get(),
            items.get_value().as_ref(),
        )
    });

    let root_state = Memo::new(move |_| {
        logic::resolve_state(CarouselPartStateInput {
            slot: CarouselSlot::Root,
            item_count,
            selected_index: selected_index.get(),
            focused_index: focused_index.get(),
            has_disabled_items,
            orientation,
            loop_navigation: is_loop_navigation,
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
    let root_state_for_agent = root_state;
    let agent_contract =
        Signal::derive(move || logic::resolve_agent_contract(root_state_for_agent.get()));

    let request_selected_index_change = Callback::new(move |next: Option<usize>| {
        let next = logic::sanitize_selected_index(next, items.get_value().as_ref());
        selected_state.request_change.run(next);
    });

    let step_selection = Callback::new(move |step: isize| {
        let items_ref = items.get_value();
        let next_index = logic::step_selected_index(
            selected_index.get_untracked(),
            step,
            is_loop_navigation,
            items_ref.as_ref(),
        );

        if let Some(next_index) = next_index {
            set_focused_index_raw.set(Some(next_index));
            request_selected_index_change.run(Some(next_index));
        }
    });

    let select_edge = Callback::new(move |pick_last: bool| {
        let next_index = logic::edge_selected_index(pick_last, items.get_value().as_ref());

        if let Some(next_index) = next_index {
            set_focused_index_raw.set(Some(next_index));
            request_selected_index_change.run(Some(next_index));
        }
    });

    let root_a11y = use_carousel_root(CarouselRootOptions {
        aria_label,
        orientation: match orientation {
            CarouselOrientation::Horizontal => CarouselA11yOrientation::Horizontal,
            CarouselOrientation::Vertical => CarouselA11yOrientation::Vertical,
        },
        lang,
        dir,
        on_key_command: Callback::new(move |command| match command {
            CarouselKeyCommand::StepBackward => step_selection.run(-1),
            CarouselKeyCommand::StepForward => step_selection.run(1),
            CarouselKeyCommand::SelectFirst => select_edge.run(false),
            CarouselKeyCommand::SelectLast => select_edge.run(true),
        }),
    });
    let controls_a11y = labeled_toolbar_attrs(
        controls_aria_label,
        match orientation {
            CarouselOrientation::Horizontal => "horizontal",
            CarouselOrientation::Vertical => "vertical",
        },
        false,
        lang_for_subgroups.clone(),
        dir_for_subgroups,
    );
    let indicators_a11y =
        labeled_group_attrs(indicators_aria_label, lang_for_subgroups, dir_for_subgroups);

    let key_down = root_a11y.handlers.on_key_down;
    let on_key_down = move |ev: ev::KeyboardEvent| {
        if key_down.run(ev.key()) {
            ev.prevent_default();
        }
    };

    let can_prev = Signal::derive(move || {
        logic::can_step_selection(
            selected_index.get(),
            -1,
            is_loop_navigation,
            items.get_value().as_ref(),
        )
    });

    let can_next = Signal::derive(move || {
        logic::can_step_selection(
            selected_index.get(),
            1,
            is_loop_navigation,
            items.get_value().as_ref(),
        )
    });

    let on_prev = move |_| step_selection.run(-1);
    let on_next = move |_| step_selection.run(1);

    let indicator_indices: StoredValue<Vec<usize>> = StoredValue::new((0..item_count).collect());

    let indicator_list_ref: NodeRef<html::Div> = NodeRef::new();
    let indicator_highlight_ref: NodeRef<html::Div> = NodeRef::new();

    let (active_index, set_active_index) = signal(logic::resolve_active_index(
        selected_index.get_untracked(),
        None,
        items.get_value().as_ref(),
    ));
    Effect::new(move |_| {
        let next = logic::resolve_active_index(
            selected_index.get(),
            focused_index.get(),
            items.get_value().as_ref(),
        );
        set_active_index.set(next);
    });

    let option_id = Callback::new(move |index: usize| {
        items
            .get_value()
            .get(index)
            .map(|item| item.dot_dom_id.clone())
            .unwrap_or_default()
    });

    crate::motion::attach_carousel_indicator_motion(
        indicator_list_ref,
        indicator_highlight_ref,
        active_index,
        option_id,
        motion,
    );

    let render_slide = move |index: usize| {
        render_carousel_slide(CarouselSlideRenderInput {
            index,
            item: items.get_value()[index].clone(),
            selected_index,
            focused_index,
        })
    };

    let render_indicator = move |index: usize| {
        render_carousel_indicator(CarouselIndicatorRenderInput {
            index,
            item: items.get_value()[index].clone(),
            indicator_aria_label_template: indicator_aria_label_template.get_value(),
            selected_index,
            focused_index,
            set_focused_index: set_focused_index_raw,
            request_selected_index_change,
        })
    };

    let viewport_slot = CarouselSlot::Viewport;
    let controls_slot = CarouselSlot::Controls;
    let prev_slot = CarouselSlot::PrevButton;
    let next_slot = CarouselSlot::NextButton;
    let indicators_slot = CarouselSlot::Indicators;
    let highlight_slot = CarouselSlot::IndicatorHighlight;

    let viewport_view = view! {
        <div class=viewport_slot.base_class() data-slot=viewport_slot.as_attr()>
            <For each=move || indicator_indices.get_value() key=|index| *index children=render_slide />
        </div>
    };

    let controls_view = view! {
        <div
            class=controls_slot.base_class()
            role=controls_a11y.role
            aria-label=controls_a11y.aria_label
            aria-orientation=controls_a11y.aria_orientation
            aria-disabled=controls_a11y.aria_disabled
            lang=controls_a11y.lang.clone()
            dir=controls_a11y.dir
            data-slot=controls_slot.as_attr()
        >
            <button
                type="button"
                class=prev_slot.base_class()
                aria-label=previous_label.get_value()
                data-slot=prev_slot.as_attr()
                disabled=move || !can_prev.get()
                on:click=on_prev
            >
                {previous_label.get_value()}
            </button>
            <button
                type="button"
                class=next_slot.base_class()
                aria-label=next_label.get_value()
                data-slot=next_slot.as_attr()
                disabled=move || !can_next.get()
                on:click=on_next
            >
                {next_label.get_value()}
            </button>
        </div>
    };

    let indicators_view = view! {
        <div
            class=indicators_slot.base_class()
            node_ref=indicator_list_ref
            role=indicators_a11y.role
            aria-label=indicators_a11y.aria_label
            lang=indicators_a11y.lang.clone()
            dir=indicators_a11y.dir
            data-slot=indicators_slot.as_attr()
        >
            <div
                class=highlight_slot.base_class()
                node_ref=indicator_highlight_ref
                data-slot=highlight_slot.as_attr()
            ></div>
            <For each=move || indicator_indices.get_value() key=|index| *index children=render_indicator />
        </div>
    };

    view! {
        <section
            class=move || root_class.get()
            role=root_a11y.attrs.role
            tabindex=root_a11y.attrs.tabindex
            aria-label=root_a11y.attrs.aria_label.clone()
            lang=root_a11y.attrs.lang.clone()
            dir=root_a11y.attrs.dir
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
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
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
            {viewport_view}
            {controls_view}
            {indicators_view}
        </section>
    }
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod tests;
