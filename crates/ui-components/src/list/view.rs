use crate::list::logic;
use crate::list::motion::{ListMotion, ListSectionMotion};
use leptos::{children::Children, html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{FocusRingOptions, ListBoxOptions, use_focus_ring, use_listbox};
use ui_visual_primitive::active_highlight::attach_active_highlight_motion;

#[component]
pub fn List(
    id_base: String,
    #[prop(into)] items: Arc<[String]>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] on_action: Option<Callback<usize>>,
    #[prop(optional, default = 0)] default_index: usize,
    #[prop(optional, default = true)] sync_active_index_to_selected: bool,
    #[prop(optional)] motion: ListMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = crate::list::motion::sanitize_motion(motion);
    let has_custom_motion = motion != ListMotion::default();

    let item_count_value = items.len();
    let (item_count, _set_item_count) = signal(item_count_value);

    let disabled_index_set: HashSet<usize> = disabled_indices.into_iter().collect();
    let has_disabled = !disabled_index_set.is_empty();
    let disabled_indices: Arc<HashSet<usize>> = Arc::new(disabled_index_set);

    let item_text = {
        let items = items.clone();
        Callback::new(move |index: usize| items.get(index).cloned().unwrap_or_default())
    };

    let is_item_disabled = has_disabled.then_some({
        let disabled_indices = disabled_indices.clone();
        Callback::new(move |index: usize| disabled_indices.contains(&index))
    });

    let aria = use_listbox(ListBoxOptions {
        is_disabled: disabled,
        should_loop: true,
        id_base,
        default_index,
        sync_active_index_to_selected,
        item_count,
        selected_index,
        set_selected_index,
        on_action,
        is_item_disabled,
        item_text: Some(item_text),
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if aria.handlers.on_key_down.run(ev.key()) {
            ev.prevent_default();
        }
    };

    let options_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    attach_active_highlight_motion(
        options_ref,
        highlight_ref,
        aria.active_index,
        aria.option_id,
        motion,
    );

    let base_class = "ui-listbox".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let accessible_name = logic::resolve_accessible_name(aria_label, aria_labelledby);
    let aria_label = StoredValue::new(accessible_name.aria_label);
    let aria_labelledby = StoredValue::new(accessible_name.aria_labelledby);

    let state = Signal::derive(move || {
        logic::resolve_state(
            item_count_value,
            aria.selected_index.get(),
            has_disabled || disabled,
        )
    });

    view! {
        <div
            class=class
            class:ui-listbox--focus-visible=move || focus_ring.is_focus_visible.get()
            id=id
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-label=aria_label.get_value()
            aria-labelledby=aria_labelledby.get_value()
            aria-disabled=aria.attrs.aria_disabled
            aria-activedescendant=move || aria.attrs.aria_activedescendant.get()
            data-slot="listbox"
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-disabled=disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-selection-empty=move || (!state.get().has_selection).then_some("true")
            data-has-disabled-options=move || {
                state.get().has_disabled_options.then_some("true")
            }
            on:keydown=on_key_down
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| focus_ring.handlers.on_blur.run(())
        >
            <div class="ui-listbox__options" node_ref=options_ref data-slot="listbox-options">
                <div class="ui-active-highlight" node_ref=highlight_ref data-slot="listbox-highlight"></div>
                {items
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(index, label)| {
                        let id = aria.option_id.run(index);
                        let is_selected = move || aria.selected_index.get() == Some(index);
                        let is_disabled = disabled || disabled_indices.contains(&index);

                        view! {
                            <div
                                id=id
                                role="option"
                                aria-selected=move || if is_selected() { Some("true") } else { None }
                                aria-disabled=if is_disabled { Some("true") } else { None }
                                class="ui-listbox__option"
                                data-slot="listbox-option"
                                data-index=index
                                data-selected=move || if is_selected() { Some("true") } else { None }
                                data-focused=move || {
                                    (aria.active_index.get() == index).then_some("true")
                                }
                                data-disabled=if is_disabled { Some("true") } else { None }
                                on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)
                                on:click=move |_| {
                                    aria.handlers.on_option_pointer_move.run(index);
                                    aria.handlers.on_option_click.run(index);
                                }
                            >
                                {label}
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn ListItem(
    children: Children,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] index: Option<usize>,
    #[prop(optional)] selected: bool,
    #[prop(optional)] focused: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] show_selection_indicator: bool,
    #[prop(optional)] has_divider: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] on_press: Option<Callback<()>>,
    #[prop(optional)] on_pointer_move: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let on_press = on_press.unwrap_or_else(|| Callback::new(|()| {}));
    let on_pointer_move = on_pointer_move.unwrap_or_else(|| Callback::new(|()| {}));

    let (aria_label, has_custom_aria_label) = logic::item::normalize_aria_label(aria_label);

    let class_name = logic::item::normalize_class_name(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Signal::derive(move || {
        logic::item::resolve_state(logic::item::ListItemStateInput {
            selected,
            focused,
            disabled,
            show_selection_indicator,
            has_divider,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Signal::derive(move || {
        logic::item::compose_class_name(class_name.get_value(), state.get())
    });

    let selection_indicator = logic::item::resolve_selection_indicator(show_selection_indicator);
    let indicator_text = move || selection_indicator.marker(state.get().is_selected);
    let index_text = index;

    view! {
        <div
            class=move || class.get()
            id=id
            role="option"
            tabindex=if disabled { Some(-1) } else { Some(0) }
            aria-label=aria_label
            aria-selected=selected.then_some("true")
            aria-disabled=disabled.then_some("true")
            data-slot="listbox-item"
            data-index=index_text
            data-state=move || state.get().data_state_attr
            data-selected=move || state.get().is_selected.then_some("true")
            data-unselected=move || (!state.get().is_selected).then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-divider=move || state.get().has_divider.then_some("true")
            data-show-selection-indicator=move || {
                state.get().show_selection_indicator.then_some("true")
            }
            data-selection-indicator=move || state.get().selection_indicator_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            on:pointermove=move |_| {
                if disabled {
                    return;
                }
                on_pointer_move.run(());
            }
            on:click=move |_| {
                if disabled {
                    return;
                }
                on_press.run(());
            }
        >
            <span
                class="ui-listbox-item__indicator"
                data-slot="listbox-item-indicator"
                data-visible=move || {
                    indicator_text().is_some().then_some("true")
                }
            >
                {indicator_text}
            </span>

            <span class="ui-listbox-item__label" data-slot="listbox-item-label">
                {children()}
            </span>

            <Show when=move || selection_indicator != logic::ListItemSelectionIndicator::Hidden>
                <span class="ui-listbox-item__selection-sr" data-slot="listbox-item-selection-sr">
                    {move || if state.get().is_selected { "selected" } else { "not selected" }}
                </span>
            </Show>

            <Show when=move || state.get().has_divider>
                <span class="ui-listbox-item__divider" data-slot="listbox-item-divider"></span>
            </Show>
        </div>
    }
}

#[component]
pub fn ListSection(
    children: Children,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] item_count: Option<usize>,
    #[prop(optional)] heading_tone: logic::ListSectionHeadingTone,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] sticky_heading: bool,
    #[prop(optional)] show_divider: bool,
    #[prop(optional)] motion: ListSectionMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let title = logic::section::normalize_title(title);
    let has_title = title.is_some();
    let title = StoredValue::new(title);

    let resolved_item_count = item_count.unwrap_or(1);

    let (aria_label, has_custom_aria_label) = logic::section::normalize_aria_label(aria_label);
    let motion = crate::list::motion::sanitize_section_motion(motion);
    let has_custom_motion = motion != ListSectionMotion::default();
    let items_ref: NodeRef<html::Div> = NodeRef::new();
    crate::list::motion::attach_section_motion(items_ref, motion);

    let class_name = logic::section::normalize_class_name(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Signal::derive(move || {
        logic::section::resolve_state(logic::section::ListSectionStateInput {
            heading_tone,
            item_count: resolved_item_count,
            disabled,
            sticky_heading,
            show_divider,
            has_title,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Signal::derive(move || {
        logic::section::compose_class_name(class_name.get_value(), state.get())
    });

    view! {
        <section
            class=move || class.get()
            role="group"
            aria-label=aria_label
            aria-disabled=disabled.then_some("true")
            data-slot="listbox-section"
            data-tone=move || state.get().heading_tone_attr
            data-state=move || state.get().data_state_attr
            data-item-count=move || state.get().item_count
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-has-title=move || state.get().has_title.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-sticky-heading=move || state.get().is_sticky_heading.then_some("true")
            data-divided=move || state.get().has_divider.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-title-source=move || state.get().title_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
        >
            <Show when=move || state.get().has_title>
                <header
                    class="ui-listbox-section__header"
                    data-slot="listbox-section-header"
                    data-sticky=move || state.get().is_sticky_heading.then_some("true")
                >
                    {move || title.get_value().unwrap_or_default()}
                </header>
            </Show>

            <div node_ref=items_ref class="ui-listbox-section__items" data-slot="listbox-section-items">
                {children()}
            </div>

            <Show when=move || state.get().has_divider>
                <div class="ui-listbox-section__divider" data-slot="listbox-section-divider"></div>
            </Show>
        </section>
    }
}
