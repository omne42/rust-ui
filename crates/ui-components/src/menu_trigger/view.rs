use crate::menu_trigger::logic;
use crate::overlay_open;
use crate::{Button, Menu, MenuItemKind, OnPress, Popover, presence::use_presence};
use leptos::{ev, html, prelude::*};
use ui_headless::PopoverPlacement;

#[component]
pub fn MenuTrigger(
    id_base: String,
    items: Vec<String>,
    on_action: Callback<usize>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] item_kinds: Vec<MenuItemKind>,
    #[prop(default = true)] close_on_action: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let id_base = StoredValue::new(id_base);
    let items: StoredValue<std::sync::Arc<[String]>> = StoredValue::new(items.into());
    let disabled_indices: StoredValue<Vec<usize>> = StoredValue::new(disabled_indices);
    let item_kinds: StoredValue<Vec<MenuItemKind>> = StoredValue::new(item_kinds);

    let open_state = overlay_open::use_controllable_open_state(open, default_open, on_open_change);
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let (open_focus, set_open_focus) = signal(logic::MenuOpenFocusStrategy::First);

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if disabled || items.get_value().is_empty() {
            return;
        }
        let next_open = !open.get_untracked();
        if next_open {
            set_open_focus.set(logic::MenuOpenFocusStrategy::First);
        }
        request_open_change.run(next_open);
    });
    let on_close: OnPress = Callback::new(move |_| request_open_change.run(false));

    let on_action_wrapped = Callback::new(move |index: usize| {
        on_action.run(index);
        if close_on_action {
            request_open_change.run(false);
        }
    });

    let ids = logic::resolve_ids(&id_base.get_value());
    let trigger_id = StoredValue::new(ids.trigger_id);
    let menu_id = StoredValue::new(ids.menu_id);

    let presence = use_presence(open);

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if disabled || items.get_value().is_empty() {
            return;
        }
        if open.get_untracked() {
            return;
        }

        let key = ev.key();
        if let Some(strategy) = logic::focus_strategy_for_open_key(&key) {
            set_open_focus.set(strategy);
            request_open_change.run(true);
            ev.prevent_default();
        }
    };

    view! {
        <div class="ui-menu-trigger" on:keydown=on_key_down>
            <Button
                node_ref=anchor_ref
                on_press=on_trigger_press
                id=trigger_id.get_value()
                disabled=disabled
                aria_haspopup="menu"
                aria_expanded=open
                aria_controls=menu_id.get_value()
            >
                {children()}
            </Button>

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=open
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
                    on_exit_complete=presence.finish_exit
                >
                    {move || {
                        let default_index =
                            open_focus.get_untracked().default_index(items.get_value().len());

                        view! {
                            <Menu
                                id_base=id_base.get_value()
                                id=menu_id.get_value()
                                aria_labelledby=trigger_id.get_value()
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
