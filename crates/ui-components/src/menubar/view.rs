use crate::menubar::{
    MenuOpenFocusStrategy, MenubarMenu, MenubarMotion, MenubarPartStateInput, MenubarSlot, logic,
};
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
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;
    let id_base = StoredValue::new(id_base);

    let menus = logic::resolve_menus(&id_base.get_value(), menus);
    let menus: StoredValue<Arc<[crate::menubar::MenubarMenuResolved]>> =
        StoredValue::new(Arc::from(menus));
    let menu_count = menus.get_value().len();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let has_custom_close_on_action = close_on_action != logic::DEFAULT_CLOSE_ON_ACTION;
    let has_custom_placement = placement != logic::DEFAULT_PLACEMENT;
    let has_custom_open_index = open_index.is_some();
    let has_custom_default_open_index = default_open_index.is_some();
    let has_custom_on_open_index_change = on_open_index_change.is_some();
    let has_custom_motion = motion != MenubarMotion::default();

    let default_open_index = logic::sanitize_open_index_for_menus(
        logic::normalize_open_index(default_open_index, menu_count),
        menus.get_value().as_ref(),
    );

    let is_controlled = has_custom_open_index;
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

    let root_state = Memo::new(move |_| {
        logic::resolve_state(MenubarPartStateInput {
            slot: MenubarSlot::Root,
            menu_count,
            open_index: open_index.get(),
            has_disabled_menus: menus
                .get_value()
                .iter()
                .any(|menu| menu.is_trigger_disabled),
            close_on_action,
            is_controlled,
            placement,
            has_custom_id_base,
            has_custom_class_name,
            has_custom_close_on_action,
            has_custom_placement,
            has_custom_open_index,
            has_custom_default_open_index,
            has_custom_on_open_index_change,
            has_custom_motion,
        })
    });
    let root_state_for_class = root_state;
    let root_class = Memo::new(move |_| {
        logic::compose_class_name(class_name.get_value(), root_state_for_class.get())
    });

    let (open_focus, set_open_focus) = signal(MenuOpenFocusStrategy::First);

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
                if let Some(focus_strategy) = crate::menubar::focus_strategy_for_open_key(&key) {
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
                            set_open_focus.set(MenuOpenFocusStrategy::First);
                            request_open_index_change.run(Some(next_index));
                            focus_trigger(&trigger_refs, next_index);
                            ev.prevent_default();
                        }
                    }
                    "ArrowLeft" => {
                        if let Some(next_index) =
                            logic::next_enabled_menu_index(menus.get_value().as_ref(), index, -1)
                        {
                            set_open_focus.set(MenuOpenFocusStrategy::First);
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

        let menu_slot = MenubarSlot::Menu;
        let trigger_slot = MenubarSlot::Trigger;

        view! {
            <div
                class=menu_slot.base_class()
                data-slot=menu_slot.as_attr()
                data-index=index
                data-state=move || {
                    if open.get() {
                        "open"
                    } else if menu_is_trigger_disabled {
                        "disabled"
                    } else {
                        "closed"
                    }
                }
                data-open=move || open.get().then_some("true")
                data-disabled=menu_is_trigger_disabled.then_some("true")
                data-empty=(!menu_has_items).then_some("true")
            >
                <button
                    type="button"
                    class=trigger_slot.base_class()
                    node_ref=trigger_ref
                    id=menu_trigger_id.get_value()
                    role="menuitem"
                    tabindex="0"
                    disabled=menu_is_trigger_disabled
                    aria-haspopup="menu"
                    aria-expanded=move || if open.get() { "true" } else { "false" }
                    aria-controls=move || open.get().then_some(menu_id.get_value())
                    data-slot=trigger_slot.as_attr()
                    data-state=move || {
                        if open.get() {
                            "open"
                        } else if menu_is_trigger_disabled {
                            "disabled"
                        } else {
                            "closed"
                        }
                    }
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
            class=move || root_class.get()
            role="menubar"
            data-slot=move || root_state.get().slot_attr
            data-state=move || root_state.get().state_attr
            data-menus=move || root_state.get().menu_attr
            data-open=move || root_state.get().open_attr
            data-closed=move || root_state.get().closed_attr
            data-empty=move || root_state.get().is_empty.then_some("true")
            data-has-menus=move || root_state.get().has_menus.then_some("true")
            data-open-index=move || root_state.get().open_index.map(|index| index.to_string())
            data-menu-count=move || root_state.get().menu_count.to_string()
            data-has-disabled-menus=move || root_state.get().has_disabled_menus.then_some("true")
            data-action-mode=move || root_state.get().action_attr
            data-open-mode=move || root_state.get().open_mode_attr
            data-controlled=move || root_state.get().is_controlled.then_some("true")
            data-uncontrolled=move || root_state.get().is_uncontrolled.then_some("true")
            data-placement=move || root_state.get().placement_attr
            data-close-on-action=move || root_state.get().close_on_action.then_some("true")
            data-keep-open-on-action=move || root_state.get().keep_open_on_action.then_some("true")
            data-id-source=move || root_state.get().id_source_attr
            data-class-source=move || root_state.get().class_source_attr
            data-close-on-action-source=move || root_state.get().close_on_action_source_attr
            data-placement-source=move || root_state.get().placement_source_attr
            data-open-index-source=move || root_state.get().open_index_source_attr
            data-default-open-index-source=move || root_state.get().default_open_index_source_attr
            data-open-index-change-source=move || root_state.get().open_index_change_source_attr
            data-motion-source=move || root_state.get().motion_source_attr
            data-custom-id=move || root_state.get().has_custom_id_base.then_some("true")
            data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
            data-custom-close-on-action=move || {
                root_state.get().has_custom_close_on_action.then_some("true")
            }
            data-custom-placement=move || root_state.get().has_custom_placement.then_some("true")
            data-custom-open-index=move || root_state.get().has_custom_open_index.then_some("true")
            data-custom-default-open-index=move || {
                root_state.get().has_custom_default_open_index.then_some("true")
            }
            data-custom-open-index-change=move || {
                root_state.get().has_custom_on_open_index_change.then_some("true")
            }
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
        >
            <For each=move || menu_indices.get_value() key=|index| *index children=render_menu />
        </div>
    }
}
