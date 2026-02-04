use crate::action_button::{ActionButton, ActionButtonSize};
use crate::action_menu::{ActionMenuMotion, logic};
use crate::{Menu, MenuItemKind, OnPress, Popover, presence::use_presence};
use leptos::{html, prelude::*};
use ui_headless::PopoverPlacement;

#[component]
pub fn ActionMenu(
    id_base: String,
    items: Vec<String>,
    on_action: Callback<usize>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] item_kinds: Vec<MenuItemKind>,
    #[prop(default = true)] close_on_action: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] size: ActionButtonSize,
    #[prop(optional)] is_quiet: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] motion: ActionMenuMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = StoredValue::new(id_base);
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

    let ids = logic::resolve_ids(&id_base.get_value());
    let menu_id = StoredValue::new(ids.menu_id);
    let presence = use_presence(is_open.into());

    let aria_label = StoredValue::new(logic::resolve_trigger_aria_label(aria_label.as_deref()));

    let base_class = "ui-action-menu".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div class=class data-slot="action-menu">
            <ActionButton
                node_ref=anchor_ref
                on_press=on_trigger_press
                size=size
                disabled=disabled
                is_quiet=is_quiet
                is_icon_only=true
                aria_label=aria_label.get_value()
                aria_haspopup="menu"
                aria_expanded=is_open.into()
                aria_controls=menu_id.get_value()
            >
                <svg viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <circle cx="10" cy="4" r="1.5" />
                    <circle cx="10" cy="10" r="1.5" />
                    <circle cx="10" cy="16" r="1.5" />
                </svg>
            </ActionButton>

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=is_open.into()
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
                    motion=motion.popover
                    on_exit_complete=presence.finish_exit
                >
                    <Menu
                        id_base=id_base.get_value()
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
