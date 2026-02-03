use crate::{Button, OnPress, Popover};
use leptos::{ev, html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{use_listbox, ListBoxOptions};

#[component]
pub fn Select(
    id_base: String,
    items: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled_indices: Vec<usize>,
) -> impl IntoView {
    let (id_base, _set_id_base) = signal(id_base);
    let items: Arc<Vec<String>> = Arc::new(items);
    let (items, _set_items) = signal(items);
    let (is_open, set_open) = signal(false);

    let disabled_set: HashSet<usize> = disabled_indices.into_iter().collect();
    let has_disabled = !disabled_set.is_empty();
    let disabled_items: Arc<HashSet<usize>> = Arc::new(disabled_set);
    let (disabled_items, _set_disabled_items) = signal(disabled_items);

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_trigger_press: OnPress = Callback::new(move |_| set_open.update(|open| *open = !*open));
    let on_close: OnPress = Callback::new(move |_| set_open.set(false));

    let placeholder = placeholder.unwrap_or_else(|| "Select…".to_string());
    let trigger_label = Memo::new({
        let placeholder = placeholder.clone();
        move |_| {
            let items = items.get();
            selected_index
                .get()
                .and_then(|i| items.get(i).cloned())
                .unwrap_or_else(|| placeholder.clone())
        }
    });

    let (item_count, _set_item_count) = signal(items.get_untracked().len());
    let item_text = Callback::new(move |index: usize| {
        items
            .get_untracked()
            .get(index)
            .cloned()
            .unwrap_or_default()
    });

    let is_item_disabled = has_disabled.then_some(Callback::new(move |index: usize| {
        disabled_items.get_untracked().contains(&index)
    }));

    let aria = use_listbox(ListBoxOptions {
        is_disabled: disabled,
        should_loop: true,
        id_base: id_base.get_untracked(),
        item_count,
        selected_index,
        set_selected_index,
        on_action: Some(Callback::new(move |_| set_open.set(false))),
        is_item_disabled,
        item_text: Some(item_text),
    });

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if aria.handlers.on_key_down.run(ev.key()) {
            ev.prevent_default();
        }
    };

    view! {
        <div class="ui-select" style="display: inline-block;">
            <Button disabled=disabled node_ref=anchor_ref on_press=on_trigger_press>
                {move || trigger_label.get()}
            </Button>

            <Show when=move || is_open.get()>
                <Popover anchor_ref=anchor_ref on_close=on_close>
                    <div style="min-width: 240px;">
                        <div
                            role=aria.attrs.role
                            tabindex=aria.attrs.tabindex
                            aria-disabled=aria.attrs.aria_disabled
                            aria-activedescendant=move || aria.attrs.aria_activedescendant.get()
                            on:keydown=on_key_down
                            style="display: flex; flex-direction: column; gap: 4px;"
                        >
                            {items
                                .get_untracked()
                                .iter()
                                .cloned()
                                .enumerate()
                                .map(|(index, label)| {
                                    let id = aria.option_id.run(index);
                                    let is_active = move || aria.active_index.get() == index;
                                    let is_selected = move || aria.selected_index.get() == Some(index);
                                    let is_disabled =
                                        disabled_items.get_untracked().contains(&index);

                                    view! {
                                        <div
                                            id=id
                                            role="option"
                                            aria-selected=move || if is_selected() { Some("true") } else { None }
                                            aria-disabled=if is_disabled { Some("true") } else { None }
                                            class="ui-select__option"
                                            class:ui-select__option--active=is_active
                                            class:ui-select__option--selected=is_selected
                                            class:ui-select__option--disabled=is_disabled
                                            on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)
                                            on:click=move |_| {
                                                aria.handlers.on_option_pointer_move.run(index);
                                                aria.handlers.on_option_click.run(index);
                                            }
                                            style="padding: 6px 10px; border-radius: 8px; cursor: default;"
                                            style:opacity=if is_disabled { "0.5" } else { "1" }
                                            style:background-color=move || if is_active() { "var(--ui-accent-soft)" } else { "transparent" }
                                            style:font-weight=move || if is_selected() { "600" } else { "400" }
                                        >
                                            {label}
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </Popover>
            </Show>
        </div>
    }
}
