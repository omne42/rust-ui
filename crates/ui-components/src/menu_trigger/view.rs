use crate::{Button, Menu, MenuItemKind, OnPress, Popover, presence::use_presence};
use leptos::{html, prelude::*};
use ui_headless::PopoverPlacement;

#[component]
pub fn MenuTrigger(
    id_base: String,
    items: Vec<String>,
    on_action: Callback<usize>,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] item_kinds: Vec<MenuItemKind>,
    #[prop(default = true)] close_on_action: bool,
    #[prop(optional)] placement: PopoverPlacement,
    children: Children,
) -> impl IntoView {
    let (id_base, _set_id_base) = signal(id_base);
    let (items, _set_items) = signal(items);
    let (disabled_indices, _set_disabled_indices) = signal(disabled_indices);
    let (item_kinds, _set_item_kinds) = signal(item_kinds);
    let (is_open, set_open) = signal(false);

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_trigger_press: OnPress = Callback::new(move |_| set_open.update(|v| *v = !*v));
    let on_close: OnPress = Callback::new(move |_| set_open.set(false));

    let on_action_wrapped = Callback::new(move |index: usize| {
        on_action.run(index);
        if close_on_action {
            set_open.set(false);
        }
    });

    let menu_id: StoredValue<String> =
        StoredValue::new(format!("{}-menu", id_base.get_untracked()));
    let presence = use_presence(is_open.into());

    view! {
        <div class="ui-menu-trigger">
            <Button
                node_ref=anchor_ref
                on_press=on_trigger_press
                aria_haspopup="menu"
                aria_expanded=is_open.into()
                aria_controls=menu_id.get_value()
            >
                {children()}
            </Button>

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=is_open.into()
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
                    on_exit_complete=presence.finish_exit
                >
                    <Menu
                        id_base=id_base.get_untracked()
                        id=menu_id.get_value()
                        items=items.get_untracked()
                        on_action=on_action_wrapped
                        disabled_indices=disabled_indices.get_untracked()
                        item_kinds=item_kinds.get_untracked()
                    />
                </Popover>
            </Show>
        </div>
    }
}
