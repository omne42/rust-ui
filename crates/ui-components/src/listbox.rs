use leptos::prelude::*;
use ui_headless::{use_listbox, ListBoxOptions};

#[component]
pub fn ListBox(
    id_base: String,
    items: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let (item_count, _set_item_count) = signal(items.len());

    let aria = use_listbox(ListBoxOptions {
        is_disabled: false,
        should_loop: true,
        id_base,
        item_count,
        selected_index,
        set_selected_index,
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
            style="margin-top: 16px; border: 1px solid #d1d5db; border-radius: 8px; padding: 8px; width: 280px;"
        >
            <div style="font-size: 12px; color: #6b7280; margin: 0 0 8px 0;">
                "ListBox (Arrow keys + Enter/Space to select)"
            </div>

            {items
                .into_iter()
                .enumerate()
                .map(|(index, label)| {
                    let id = aria.option_id.run(index);
                    let is_active = move || aria.active_index.get() == index;
                    let is_selected = move || aria.selected_index.get() == Some(index);

                    view! {
                        <div
                            id=id
                            role="option"
                            aria-selected=move || if is_selected() { Some("true") } else { None }
                            class="ui-listbox__option"
                            class:ui-listbox__option--active=is_active
                            class:ui-listbox__option--selected=is_selected
                            on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)
                            on:click=move |_| {
                                aria.handlers.on_option_pointer_move.run(index);
                                aria.handlers.on_option_click.run(index);
                            }
                            style="padding: 6px 8px; border-radius: 6px; cursor: default;"
                        >
                            {label}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
