use crate::{Button, Menu, MenuItemKind, OnPress, Popover, presence::use_presence};
use leptos::{ev, html, prelude::*};
use ui_headless::PopoverPlacement;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum MenuOpenFocusStrategy {
    #[default]
    First,
    Last,
}

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
    let id_base = StoredValue::new(id_base);
    let items: StoredValue<std::sync::Arc<[String]>> = StoredValue::new(items.into());
    let disabled_indices: StoredValue<Vec<usize>> = StoredValue::new(disabled_indices);
    let item_kinds: StoredValue<Vec<MenuItemKind>> = StoredValue::new(item_kinds);

    let (is_open, set_open) = signal(false);
    let (open_focus, set_open_focus) = signal(MenuOpenFocusStrategy::First);

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if items.get_value().is_empty() {
            return;
        }
        set_open_focus.set(MenuOpenFocusStrategy::First);
        set_open.update(|v| *v = !*v);
    });
    let on_close: OnPress = Callback::new(move |_| set_open.set(false));

    let on_action_wrapped = Callback::new(move |index: usize| {
        on_action.run(index);
        if close_on_action {
            set_open.set(false);
        }
    });

    let menu_id: StoredValue<String> = StoredValue::new(format!("{}-menu", id_base.get_value()));
    let presence = use_presence(is_open.into());

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if items.get_value().is_empty() {
            return;
        }
        if is_open.get_untracked() {
            return;
        }

        match ev.key().as_str() {
            "ArrowDown" => {
                set_open_focus.set(MenuOpenFocusStrategy::First);
                set_open.set(true);
                ev.prevent_default();
            }
            "ArrowUp" => {
                set_open_focus.set(MenuOpenFocusStrategy::Last);
                set_open.set(true);
                ev.prevent_default();
            }
            "Enter" => {
                set_open_focus.set(MenuOpenFocusStrategy::First);
            }
            _ => {}
        }
    };

    let on_key_up = move |ev: ev::KeyboardEvent| {
        if items.get_value().is_empty() {
            return;
        }
        if matches!(ev.key().as_str(), " " | "Space" | "Spacebar") {
            set_open_focus.set(MenuOpenFocusStrategy::First);
        }
    };

    view! {
        <div class="ui-menu-trigger" on:keydown=on_key_down on:keyup=on_key_up>
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
                    {move || {
                        let default_index = match open_focus.get_untracked() {
                            MenuOpenFocusStrategy::First => 0,
                            MenuOpenFocusStrategy::Last => items.get_value().len().saturating_sub(1),
                        };

                        view! {
                            <Menu
                                id_base=id_base.get_value()
                                id=menu_id.get_value()
                                items=items.get_value()
                                on_action=on_action_wrapped
                                disabled_indices=disabled_indices.get_value()
                                item_kinds=item_kinds.get_value()
                                default_index=default_index
                            />
                        }
                    }}
                </Popover>
            </Show>
        </div>
    }
}
