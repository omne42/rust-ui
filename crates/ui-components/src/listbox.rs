use leptos::prelude::*;
use std::{collections::HashSet, sync::Arc};
use ui_headless::{use_listbox, ListBoxOptions};

#[component]
pub fn ListBox(
    id_base: String,
    items: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] disabled_indices: Vec<usize>,
) -> impl IntoView {
    let items: Arc<Vec<String>> = Arc::new(items);
    let (item_count, _set_item_count) = signal(items.len());

    let disabled_set: HashSet<usize> = disabled_indices.into_iter().collect();
    let has_disabled = !disabled_set.is_empty();
    let disabled: Arc<HashSet<usize>> = Arc::new(disabled_set);

    let item_text = {
        let items = items.clone();
        Callback::new(move |index: usize| items.get(index).cloned().unwrap_or_default())
    };

    let is_item_disabled = has_disabled.then_some({
        let disabled = disabled.clone();
        Callback::new(move |index: usize| disabled.contains(&index))
    });

    let aria = use_listbox(ListBoxOptions {
        is_disabled: false,
        should_loop: true,
        id_base,
        item_count,
        selected_index,
        set_selected_index,
        on_action: None,
        is_item_disabled,
        item_text: Some(item_text),
    });

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if aria.handlers.on_key_down.run(ev.key()) {
            ev.prevent_default();
        }
    };

    view! {
        <div
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-activedescendant=move || aria.attrs.aria_activedescendant.get()
            on:keydown=on_key_down
            style="margin-top: 16px; border: 1px solid var(--ui-border); border-radius: var(--ui-radius-lg); padding: var(--ui-space-sm); width: 280px; background: var(--ui-bg); box-shadow: var(--ui-shadow-sm);"
        >
            <div style="font-size: 12px; color: var(--ui-fg-muted); margin: 0 0 8px 0;">
                "ListBox (Arrow keys + Enter/Space to select; typeahead supported)"
            </div>

            {items
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, label)| {
                    let id = aria.option_id.run(index);
                    let is_active = move || aria.active_index.get() == index;
                    let is_selected = move || aria.selected_index.get() == Some(index);
                    let is_disabled = disabled.contains(&index);

                    view! {
                        <div
                            id=id
                            role="option"
                            aria-selected=move || if is_selected() { Some("true") } else { None }
                            aria-disabled=if is_disabled { Some("true") } else { None }
                            class="ui-listbox__option"
                            class:ui-listbox__option--active=is_active
                            class:ui-listbox__option--selected=is_selected
                            class:ui-listbox__option--disabled=is_disabled
                            on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)
                            on:click=move |_| {
                                aria.handlers.on_option_pointer_move.run(index);
                                aria.handlers.on_option_click.run(index);
                            }
                            style="padding: 6px 8px; border-radius: 6px; cursor: default;"
                            style:background-color=move || if is_active() { "var(--ui-accent-soft)" } else { "transparent" }
                            style:font-weight=move || if is_selected() { "600" } else { "400" }
                            style:opacity=if is_disabled { "0.5" } else { "1" }
                        >
                            {label}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
