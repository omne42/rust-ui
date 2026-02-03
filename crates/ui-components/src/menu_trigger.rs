use crate::{Button, Menu, OnPress, Overlay};
use leptos::prelude::*;

#[component]
pub fn MenuTrigger(
    id_base: String,
    items: Vec<String>,
    on_action: Callback<usize>,
    children: Children,
) -> impl IntoView {
    let (id_base, _set_id_base) = signal(id_base);
    let (items, _set_items) = signal(items);
    let (is_open, set_open) = signal(false);

    let on_trigger_press: OnPress = Callback::new(move |_| set_open.update(|v| *v = !*v));
    let on_close: OnPress = Callback::new(move |_| set_open.set(false));

    let on_action_and_close = Callback::new(move |index: usize| {
        on_action.run(index);
        set_open.set(false);
    });

    view! {
        <div class="ui-menu-trigger" style="display: inline-block;">
            <Button on_press=on_trigger_press>{children()}</Button>

            <Show when=move || is_open.get()>
                <Overlay on_close=on_close>
                    <Menu
                        id_base=id_base.get_untracked()
                        items=items.get_untracked()
                        on_action=on_action_and_close
                    />
                </Overlay>
            </Show>
        </div>
    }
}
