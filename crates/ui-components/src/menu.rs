use leptos::prelude::*;
use std::{collections::HashSet, sync::Arc};
use ui_headless::{use_menu, MenuOptions};

#[component]
pub fn Menu(
    id_base: String,
    items: Vec<String>,
    on_action: Callback<usize>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
) -> impl IntoView {
    let items: Arc<Vec<String>> = Arc::new(items);
    let (item_count, _set_item_count) = signal(items.len());

    let disabled_set: HashSet<usize> = disabled_indices.into_iter().collect();
    let has_disabled = !disabled_set.is_empty();
    let disabled_items: Arc<HashSet<usize>> = Arc::new(disabled_set);

    let item_text = {
        let items = items.clone();
        Callback::new(move |index: usize| items.get(index).cloned().unwrap_or_default())
    };

    let is_item_disabled = has_disabled.then_some({
        let disabled_items = disabled_items.clone();
        Callback::new(move |index: usize| disabled_items.contains(&index))
    });

    let aria = use_menu(MenuOptions {
        is_disabled: disabled,
        should_loop: true,
        id_base,
        item_count,
        on_action: Some(on_action),
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
            style="display: flex; flex-direction: column; gap: 4px;"
        >
            {items
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, label)| {
                    let id = aria.option_id.run(index);
                    let is_active = move || aria.active_index.get() == index;
                    let is_disabled = disabled_items.contains(&index);

                    view! {
                        <div
                            id=id
                            role="menuitem"
                            aria-disabled=if is_disabled { Some("true") } else { None }
                            class="ui-menu__item"
                            class:ui-menu__item--active=is_active
                            class:ui-menu__item--disabled=is_disabled
                            on:pointermove=move |_| aria.handlers.on_item_pointer_move.run(index)
                            on:click=move |_| aria.handlers.on_item_click.run(index)
                            style="padding: 6px 10px; border-radius: 8px; cursor: default;"
                            style:opacity=if is_disabled { "0.5" } else { "1" }
                            style:background-color=move || if is_active() { "#eff6ff" } else { "transparent" }
                        >
                            {label}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
