use crate::menubar::{MenubarMenu, MenubarMotion, logic};
use crate::overlay_open;
use crate::{Menu, OnPress, Popover, presence::use_presence};
use leptos::{ev, html, prelude::*};
use std::sync::Arc;
use ui_headless::PopoverPlacement;

#[cfg(target_arch = "wasm32")]
fn focus_trigger(trigger_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize) {
    let Some(node_ref) = trigger_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    let _ = el.focus();
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_trigger(_trigger_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}

#[component]
pub fn Menubar(
    id_base: String,
    menus: Vec<MenubarMenu>,
    on_action: Callback<(usize, usize)>,
    #[prop(default = true)] close_on_action: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] open_index: Option<Signal<Option<usize>>>,
    #[prop(optional)] default_open_index: Option<usize>,
    #[prop(optional)] on_open_index_change: Option<Callback<Option<usize>>>,
    #[prop(optional)] motion: MenubarMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let id_base = StoredValue::new(id_base);

    let menus = logic::resolve_menus(&id_base.get_value(), menus);
    let menus: StoredValue<Arc<[logic::MenubarMenuResolved]>> = StoredValue::new(Arc::from(menus));
    let menu_count = menus.get_value().len();

    let default_open_index = logic::sanitize_open_index_for_menus(
        logic::normalize_open_index(default_open_index, menu_count),
        menus.get_value().as_ref(),
    );

    let is_controlled = open_index.is_some();
    let open_state = overlay_open::use_controllable_state(
        open_index,
        Some(default_open_index),
        on_open_index_change,
    );

    let open_index = Signal::derive(move || {
        logic::sanitize_open_index_for_menus(open_state.value.get(), menus.get_value().as_ref())
    });

    let request_open_index_change = Callback::new(move |next: Option<usize>| {
        let next = logic::sanitize_open_index_for_menus(next, menus.get_value().as_ref());
        open_state.request_change.run(next);
    });

    let has_custom_class_name = class_name.is_some();

    let state = Signal::derive(move || {
        logic::resolve_state(logic::MenubarStateInput {
            menu_count,
            open_index: open_index.get(),
            has_disabled_menus: menus
                .get_value()
                .iter()
                .any(|menu| menu.is_trigger_disabled),
            has_custom_class_name,
            is_controlled,
            placement,
        })
    });

    let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));

    let (open_focus, set_open_focus) = signal(logic::MenuOpenFocusStrategy::First);

    let trigger_refs: Arc<Vec<NodeRef<html::Button>>> =
        Arc::new((0..menu_count).map(|_| NodeRef::new()).collect());

    let menu_indices: StoredValue<Vec<usize>> = StoredValue::new((0..menu_count).collect());

    let render_menu = move |index: usize| {
        let menu = menus.get_value()[index].clone();
        let trigger_ref = trigger_refs[index];
        let menu_is_trigger_disabled = menu.is_trigger_disabled;
        let menu_has_items = menu.has_items;
        let menu_label = StoredValue::new(menu.label);
        let menu_trigger_id = StoredValue::new(menu.trigger_id);
        let menu_id = StoredValue::new(menu.menu_id);
        let menu_items = StoredValue::new(menu.items);
        let menu_disabled_indices = StoredValue::new(menu.disabled_indices);
        let menu_item_kinds = StoredValue::new(menu.item_kinds);

        let open = Signal::derive(move || open_index.get() == Some(index));
        let presence = use_presence(open);

        let on_close: OnPress = Callback::new(move |_| request_open_index_change.run(None));

        let on_action_wrapped = Callback::new(move |item_index: usize| {
            on_action.run((index, item_index));
            if close_on_action {
                request_open_index_change.run(None);
            }
        });

        let on_trigger_press: OnPress = Callback::new(move |_| {
            if menu_is_trigger_disabled {
                return;
            }

            let next_open_index = if open_index.get_untracked() == Some(index) {
                None
            } else {
                Some(index)
            };
            request_open_index_change.run(next_open_index);
        });

        let on_key_down = {
            let trigger_refs = trigger_refs.clone();
            move |ev: ev::KeyboardEvent| {
                if menu_is_trigger_disabled {
                    return;
                }

                let key = ev.key();
                if let Some(focus_strategy) = logic::focus_strategy_for_open_key(&key) {
                    set_open_focus.set(focus_strategy);
                    request_open_index_change.run(Some(index));
                    ev.prevent_default();
                    return;
                }

                match key.as_str() {
                    "ArrowRight" => {
                        if let Some(next_index) =
                            logic::next_enabled_menu_index(menus.get_value().as_ref(), index, 1)
                        {
                            set_open_focus.set(logic::MenuOpenFocusStrategy::First);
                            request_open_index_change.run(Some(next_index));
                            focus_trigger(&trigger_refs, next_index);
                            ev.prevent_default();
                        }
                    }
                    "ArrowLeft" => {
                        if let Some(next_index) =
                            logic::next_enabled_menu_index(menus.get_value().as_ref(), index, -1)
                        {
                            set_open_focus.set(logic::MenuOpenFocusStrategy::First);
                            request_open_index_change.run(Some(next_index));
                            focus_trigger(&trigger_refs, next_index);
                            ev.prevent_default();
                        }
                    }
                    "Escape" => {
                        request_open_index_change.run(None);
                        ev.prevent_default();
                    }
                    _ => {}
                }
            }
        };

        let on_pointer_enter = move |_| {
            if menu_is_trigger_disabled {
                return;
            }

            let active_open = open_index.get_untracked();
            if active_open.is_some() && active_open != Some(index) {
                request_open_index_change.run(Some(index));
            }
        };

        view! {
            <div
                class="ui-menubar__menu"
                data-slot="menubar-menu"
                data-index=index
                data-open=move || open.get().then_some("true")
                data-disabled=menu_is_trigger_disabled.then_some("true")
                data-empty=(!menu_has_items).then_some("true")
            >
                <button
                    type="button"
                    class="ui-menubar__trigger"
                    node_ref=trigger_ref
                    id=menu_trigger_id.get_value()
                    role="menuitem"
                    tabindex="0"
                    disabled=menu_is_trigger_disabled
                    aria-haspopup="menu"
                    aria-expanded=move || {
                        if open.get() {
                            "true"
                        } else {
                            "false"
                        }
                    }
                    aria-controls=move || open.get().then_some(menu_id.get_value())
                    data-slot="menubar-trigger"
                    on:click=move |_| on_trigger_press.run(())
                    on:keydown=on_key_down
                    on:pointerenter=on_pointer_enter
                >
                    {menu_label.get_value()}
                </button>

                <Show when=move || presence.is_present.get()>
                    <Popover
                        open=open
                        anchor_ref=trigger_ref
                        on_close=on_close
                        placement=placement
                        motion=motion.popover
                        is_modal=false
                        on_exit_complete=presence.finish_exit
                    >
                        {move || {
                            let default_item_index =
                                open_focus.get_untracked().default_index(menu_items.get_value().len());

                            view! {
                                <Menu
                                    id_base=id_base.get_value()
                                    id=menu_id.get_value()
                                    aria_labelledby=menu_trigger_id.get_value()
                                    items=menu_items.get_value()
                                    on_action=on_action_wrapped
                                    disabled_indices=menu_disabled_indices.get_value()
                                    item_kinds=menu_item_kinds.get_value()
                                    default_index=default_item_index
                                />
                            }
                        }}
                    </Popover>
                </Show>
            </div>
        }
    };

    view! {
        <div
            class=move || class.get()
            role="menubar"
            data-slot="menubar"
            data-state=move || state.get().data_state_attr
            data-open=move || state.get().has_open_menu.then_some("true")
            data-closed=move || (!state.get().has_open_menu).then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-menus=move || state.get().has_menus.then_some("true")
            data-open-index=move || state.get().open_index.map(|index| index.to_string())
            data-menu-count=move || state.get().menu_count.to_string()
            data-has-disabled-menus=move || state.get().has_disabled_menus.then_some("true")
            data-controlled=move || state.get().is_controlled.then_some("true")
            data-uncontrolled=move || state.get().is_uncontrolled.then_some("true")
            data-placement=move || state.get().placement_attr
        >
            <For each=move || menu_indices.get_value() key=|index| *index children=render_menu />
        </div>
    }
}
