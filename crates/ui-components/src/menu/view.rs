use crate::active_highlight::{ActiveHighlightMotion, attach_active_highlight_motion};
use leptos::{html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{MenuItemKind, MenuItemOptions, MenuOptions, use_menu, use_menu_item};

#[component]
pub fn Menu(
    id_base: String,
    #[prop(into)] items: Arc<[String]>,
    on_action: Callback<usize>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] item_kinds: Vec<MenuItemKind>,
    #[prop(optional, default = 0)] default_index: usize,
    #[prop(optional)] motion: ActiveHighlightMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (item_count, _set_item_count) = signal(items.len());

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

    view! {
        <div
            class=class
            id=id
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-activedescendant=move || aria.attrs.aria_activedescendant.get()
            on:keydown=on_key_down
        >
            <div class="ui-menu__items" node_ref=items_ref data-slot="menu-items">
                <div class="ui-active-highlight" node_ref=highlight_ref data-slot="menu-highlight"></div>
                {items
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(move |(index, label)| {
                        let kind = item_kinds
                            .get(index)
                            .copied()
                            .unwrap_or(MenuItemKind::Action);
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
                            MenuItemKind::Checkbox { is_checked } => is_checked.get().then_some("✓"),
                            MenuItemKind::Radio { is_checked } => is_checked.get().then_some("●"),
                        };

                        view! {
                            <div
                                id=item.attrs.id
                                role=item.attrs.role
                                aria-checked=move || item.attrs.aria_checked.get()
                                aria-disabled=item.attrs.aria_disabled
                                class="ui-menu__item"
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
