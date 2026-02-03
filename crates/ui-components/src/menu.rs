use leptos::prelude::*;
use ui_headless::{use_menu, MenuOptions};

#[component]
pub fn Menu(
    id_base: String,
    items: Vec<String>,
    on_action: Callback<usize>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let (item_count, _set_item_count) = signal(items.len());

    let aria = use_menu(MenuOptions {
        is_disabled: disabled,
        should_loop: true,
        id_base,
        item_count,
        on_action: Some(on_action),
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
                .into_iter()
                .enumerate()
                .map(|(index, label)| {
                    let id = aria.option_id.run(index);
                    let is_active = move || aria.active_index.get() == index;

                    view! {
                        <div
                            id=id
                            role="menuitem"
                            class="ui-menu__item"
                            class:ui-menu__item--active=is_active
                            on:pointermove=move |_| aria.handlers.on_item_pointer_move.run(index)
                            on:click=move |_| aria.handlers.on_item_click.run(index)
                            style="padding: 6px 10px; border-radius: 8px; cursor: default;"
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
