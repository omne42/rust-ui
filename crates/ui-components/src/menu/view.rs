use crate::menu::logic;
use leptos::{html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{MenuItemKind, MenuItemOptions, MenuOptions, use_menu, use_menu_item};
use ui_visual_primitive::active_highlight::{
    ActiveHighlightMotion, attach_active_highlight_motion,
};

#[component]
pub fn Menu(
    id_base: String,
    #[prop(into)] items: Arc<[String]>,
    on_action: Callback<usize>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] item_kinds: Vec<MenuItemKind>,
    #[prop(optional, default = 0)] default_index: usize,
    #[prop(optional)] motion: ActiveHighlightMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let item_count_value = items.len();
    let (item_count, _set_item_count) = signal(item_count_value);

    let disabled_set: HashSet<usize> = disabled_indices.into_iter().collect();
    let has_disabled = !disabled_set.is_empty();
    let disabled_indices: Arc<HashSet<usize>> = Arc::new(disabled_set);

    let item_kinds: Arc<Vec<MenuItemKind>> = Arc::new(item_kinds);

    let item_text = {
        let items = items.clone();
        Callback::new(move |index: usize| items.get(index).cloned().unwrap_or_default())
    };

    let is_item_disabled = has_disabled.then_some({
        let disabled_indices = disabled_indices.clone();
        Callback::new(move |index: usize| disabled_indices.contains(&index))
    });

    let aria = use_menu(MenuOptions {
        is_disabled: disabled,
        should_loop: true,
        id_base,
        item_count,
        default_index,
        on_action: Some(on_action),
        is_item_disabled,
        item_text: Some(item_text),
    });
    let menu = aria.clone();

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if aria.handlers.on_key_down.run(ev.key()) {
            ev.prevent_default();
        }
    };

    let items_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    attach_active_highlight_motion(
        items_ref,
        highlight_ref,
        aria.active_index,
        aria.option_id,
        motion,
    );

    let base_class = "ui-menu".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let motion_source = if motion == ActiveHighlightMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != ActiveHighlightMotion::default()).then_some("true");

    let accessible_name = logic::resolve_accessible_name(aria_label, aria_labelledby);
    let aria_label = StoredValue::new(accessible_name.aria_label);
    let aria_labelledby = StoredValue::new(accessible_name.aria_labelledby);

    let has_checked_items = Signal::derive({
        let item_kinds = item_kinds.clone();
        move || {
            item_kinds.iter().any(|kind| match kind {
                MenuItemKind::Action => false,
                MenuItemKind::Checkbox { is_checked } | MenuItemKind::Radio { is_checked } => {
                    is_checked.get()
                }
            })
        }
    });

    let state = Signal::derive(move || {
        logic::resolve_state(
            item_count_value,
            has_checked_items.get(),
            has_disabled || disabled,
        )
    });

    view! {
        <div
            class=class
            id=id
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-label=aria_label.get_value()
            aria-labelledby=aria_labelledby.get_value()
            aria-disabled=aria.attrs.aria_disabled
            aria-activedescendant=move || aria.attrs.aria_activedescendant.get()
            data-slot="menu"
            data-disabled=disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-has-checked-items=move || state.get().has_checked_items.then_some("true")
            data-checked-empty=move || (!state.get().has_checked_items).then_some("true")
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            on:keydown=on_key_down
        >
            <div class="ui-menu__items" node_ref=items_ref data-slot="menu-items">
                <div class="ui-active-highlight" node_ref=highlight_ref data-slot="menu-highlight"></div>
                {items
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(move |(index, label)| {
                        let kind = item_kinds.get(index).copied().unwrap_or(MenuItemKind::Action);
                        let is_disabled = disabled || disabled_indices.contains(&index);
                        let item = use_menu_item(
                            &menu,
                            MenuItemOptions {
                                index,
                                kind,
                                is_disabled,
                            },
                        );

                        let indicator = move || match kind {
                            MenuItemKind::Action => None,
                            MenuItemKind::Checkbox { is_checked } => {
                                is_checked.get().then_some("✓")
                            }
                            MenuItemKind::Radio { is_checked } => is_checked.get().then_some("●"),
                        };

                        view! {
                            <div
                                id=item.attrs.id
                                role=item.attrs.role
                                aria-checked=move || item.attrs.aria_checked.get()
                                aria-disabled=item.attrs.aria_disabled
                                class="ui-menu__item"
                                data-slot="menu-item"
                                data-index=index
                                data-kind=item.attrs.role
                                data-checked=move || {
                                    item.attrs
                                        .aria_checked
                                        .get()
                                        .filter(|state| *state == "true")
                                }
                                data-focused=move || {
                                    (aria.active_index.get() == index).then_some("true")
                                }
                                data-disabled=if is_disabled { Some("true") } else { None }
                                on:pointermove=move |_| item.handlers.on_pointer_move.run(())
                                on:click=move |_| item.handlers.on_click.run(())
                            >
                                <span class="ui-menu__indicator">{indicator}</span>
                                {label}
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
