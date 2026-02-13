use crate::active_highlight::{ActiveHighlightMotion, attach_active_highlight_motion};
use crate::listbox::logic;
use leptos::{html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{FocusRingOptions, ListBoxOptions, use_focus_ring, use_listbox};

#[component]
pub fn ListBox(
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
    #[prop(optional)] motion: ActiveHighlightMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = crate::listbox::motion::sanitize_motion(motion);
    let has_custom_motion = motion != ActiveHighlightMotion::default();

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
