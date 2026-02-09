use crate::active_highlight::attach_active_highlight_motion;
use crate::overlay_open;
use crate::sidebar_menu::SidebarMenuMotion;
use crate::sidebar_menu::logic::{
    self, SidebarMenuItem, SidebarMenuState, SidebarMenuStateInput, SidebarMenuSubItem,
};
use leptos::{ev, html, prelude::*};
use std::collections::BTreeSet;
use std::sync::Arc;

#[derive(Clone)]
struct SidebarMenuRenderCtx {
    active_id: Signal<Option<String>>,
    open_sub_ids: ReadSignal<BTreeSet<String>>,
    show_badges: bool,
    show_actions: bool,
    allow_submenu_collapse: bool,
    disabled: bool,
    id_base: StoredValue<String>,
    select_id: Callback<String>,
    trigger_action: Callback<String>,
    trigger_item_action: Callback<String>,
    toggle_submenu: Callback<String>,
}

impl SidebarMenuRenderCtx {
    fn render_menu_item(&self, index: usize, item: &SidebarMenuItem) -> AnyView {
        let item_id = item.id.clone();
        let item_label = item.label.clone();
        let item_href = item.href.clone();
        let item_badge = item.badge.clone();
        let item_action_label = item.action_label.clone();
        let item_disabled = self.disabled || item.disabled;
        let has_sub = !item.sub_items.is_empty();

        let sub_items: Arc<[SidebarMenuSubItem]> = item.sub_items.clone().into();

        let active_id = self.active_id;
        let open_sub_ids = self.open_sub_ids;
        let select_id = self.select_id;
        let trigger_action = self.trigger_action;
        let trigger_item_action = self.trigger_item_action;
        let toggle_submenu = self.toggle_submenu;
        let allow_submenu_collapse = self.allow_submenu_collapse;
        let show_badges = self.show_badges;
        let show_actions = self.show_actions;
        let id_base = self.id_base;

        let on_focus = {
            let item_id = item_id.clone();
            move |_: ev::FocusEvent| {
                select_id.run(item_id.clone());
            }
        };

        let on_pointer_enter = {
            let item_id = item_id.clone();
            move |_: ev::PointerEvent| {
                select_id.run(item_id.clone());
            }
        };

        let on_click = {
            let item_id = item_id.clone();
            move |_| {
                trigger_action.run(item_id.clone());
            }
        };

        let on_toggle_sub = {
            let item_id = item_id.clone();
            move |_| {
                toggle_submenu.run(item_id.clone());
            }
        };

        let on_item_action_click = {
            let item_id = item_id.clone();
            move |_| {
                trigger_item_action.run(item_id.clone());
            }
        };

        let item_active = {
            let item_id = item_id.clone();
            move || (active_id.get().as_deref() == Some(item_id.as_str())).then_some("true")
        };

        let item_aria_current = {
            let item_id = item_id.clone();
            move || {
                if active_id.get().as_deref() == Some(item_id.as_str()) {
                    Some("page")
                } else {
                    None
                }
            }
        };

        view! {
            <section
                class="ui-sidebar-menu__item"
                data-slot="sidebar-menu-item"
                data-index=index.to_string()
                data-id=item_id.clone()
                data-active=item_active
                data-disabled=item_disabled.then_some("true")
                data-has-sub=has_sub.then_some("true")
                data-sub-open={
                    let item_id = item_id.clone();
                    move || open_sub_ids.get().contains(&item_id).then_some("true")
                }
                id=move || format!("{}-item-{index}", id_base.get_value())
            >
                <div class="ui-sidebar-menu__item-main" data-slot="sidebar-menu-item-main">
                    <button
                        class="ui-sidebar-menu__button"
                        data-slot="sidebar-menu-button"
                        type="button"
                        disabled=item_disabled
                        aria-disabled=item_disabled.then_some("true")
                        aria-current=item_aria_current
                        on:focus=on_focus
                        on:pointerenter=on_pointer_enter
                        on:click=on_click
                    >
                        <span class="ui-sidebar-menu__label" data-slot="sidebar-menu-label">{item_label}</span>
                        {match item_href {
                            Some(item_href) => {
                                view! {
                                    <span class="ui-sidebar-menu__href" data-slot="sidebar-menu-href">
                                        {item_href}
                                    </span>
                                }
                                .into_any()
                            }
                            None => ().into_any(),
                        }}

                        {if show_badges {
                            match item_badge {
                                Some(item_badge) => {
                                    view! {
                                        <span class="ui-sidebar-menu__badge" data-slot="sidebar-menu-badge">
                                            {item_badge}
                                        </span>
                                    }
                                    .into_any()
                                }
                                None => ().into_any(),
                            }
                        } else {
                            ().into_any()
                        }}
                    </button>

                    {if show_actions {
                        view! {
                            <button
                                class="ui-sidebar-menu__action"
                                data-slot="sidebar-menu-action"
                                type="button"
                                disabled=item_disabled
                                aria-disabled=item_disabled.then_some("true")
                                aria-label=item_action_label.unwrap_or_else(|| "item action".to_string())
                                on:click=on_item_action_click
                            >
                                "⋯"
                            </button>
                        }
                        .into_any()
                    } else {
                        ().into_any()
                    }}

                    {if allow_submenu_collapse && has_sub {
                        view! {
                            <button
                                class="ui-sidebar-menu__toggle"
                                data-slot="sidebar-menu-toggle"
                                data-open={
                                    let item_id = item_id.clone();
                                    move || open_sub_ids.get().contains(&item_id).then_some("true")
                                }
                                type="button"
                                disabled=item_disabled
                                aria-disabled=item_disabled.then_some("true")
                                aria-expanded={
                                    let item_id = item_id.clone();
                                    move || {
                                        if open_sub_ids.get().contains(&item_id) {
                                            "true"
                                        } else {
                                            "false"
                                        }
                                    }
                                }
                                aria-label="toggle submenu"
                                on:click=on_toggle_sub
                            >
                                "▸"
                            </button>
                        }
                        .into_any()
                    } else {
                        ().into_any()
                    }}
                </div>

                <Show when={
                    let item_id = item_id.clone();
                    move || {
                        has_sub
                            && (!allow_submenu_collapse || open_sub_ids.get().contains(&item_id))
                    }
                }>
                    <div class="ui-sidebar-menu__sub" data-slot="sidebar-menu-sub">
                        {sub_items
                            .iter()
                            .enumerate()
                            .map(|(sub_index, sub_item)| {
                                render_sub_item(
                                    sub_index,
                                    sub_item,
                                    active_id,
                                    item_disabled,
                                    select_id,
                                    trigger_action,
                                )
                            })
                            .collect_view()}
                    </div>
                </Show>
            </section>
        }
        .into_any()
    }
}

#[component]
pub fn SidebarMenu(
    items: Vec<SidebarMenuItem>,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional)] active_id: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_active_id: Option<String>,
    #[prop(optional)] on_active_id_change: Option<Callback<Option<String>>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional)] on_item_action: Option<Callback<String>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = true)] show_badges: bool,
    #[prop(optional, default = true)] show_actions: bool,
    #[prop(optional, default = true)] allow_submenu_collapse: bool,
    #[prop(optional, default = true)] enable_keyboard_shortcut: bool,
    #[prop(optional, into)] keyboard_shortcut_key: Option<String>,
    #[prop(optional)] motion: SidebarMenuMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let class_name = logic::normalize_optional_text(class_name);
    let aria_label = logic::normalize_aria_label(aria_label);
    let shortcut_key = logic::normalize_optional_text(keyboard_shortcut_key)
        .map(|key| key.to_ascii_lowercase())
        .filter(|_| enable_keyboard_shortcut);

    let items: Arc<[SidebarMenuItem]> = logic::normalize_items(items).into();
    let item_count = items.len();

    let default_active_id = logic::default_active_id(items.as_ref(), default_active_id);
    let is_controlled = active_id.is_some();
    let active_state = overlay_open::use_controllable_state(
        active_id,
        Some(default_active_id),
        on_active_id_change,
    );
    let active_id = active_state.value;
    let request_active_id_change = active_state.request_change;

    let (active_index_read, set_active_index_read) = signal(0_usize);

    let open_sub_default: BTreeSet<String> = logic::default_open_sub_ids(items.as_ref())
        .into_iter()
        .collect();
    let (open_sub_ids, set_open_sub_ids) = signal(open_sub_default);

    let id_base = StoredValue::new(id_base);
    let class_name = StoredValue::new(class_name);
    let aria_label = StoredValue::new(aria_label);
    let items = StoredValue::new(items);
    let on_action = StoredValue::new(on_action);
    let on_item_action = StoredValue::new(on_item_action);
    let shortcut_key = StoredValue::new(shortcut_key);

    let state: Signal<SidebarMenuState> = Signal::derive(move || {
        logic::resolve_state(SidebarMenuStateInput {
            item_count,
            disabled,
            show_badges,
            show_actions,
            allow_submenu_collapse,
            is_controlled,
            has_custom_class_name: class_name.get_value().is_some(),
            has_shortcut: shortcut_key.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let legend_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    let option_id =
        Callback::new(move |index: usize| format!("{}-item-{index}", id_base.get_value()));
    attach_active_highlight_motion(
        legend_ref,
        highlight_ref,
        active_index_read,
        option_id,
        motion,
    );

    Effect::new(move |_| {
        let active = active_id.get();
        let linear_ids = logic::linear_enabled_ids(items.get_value().as_ref());
        let next_index = active
            .as_ref()
            .and_then(|active| linear_ids.iter().position(|id| id == active))
            .unwrap_or(0);
        set_active_index_read.set(next_index);
    });

    let select_id = Callback::new(move |id: String| {
        if disabled || !logic::contains_id(items.get_value().as_ref(), &id) {
            return;
        }
        request_active_id_change.run(Some(id));
    });

    let trigger_action = Callback::new(move |id: String| {
        if disabled {
            return;
        }

        select_id.run(id.clone());

        if let Some(callback) = on_action.get_value() {
            callback.run(id);
        }
    });

    let trigger_item_action = Callback::new(move |id: String| {
        if disabled {
            return;
        }

        if let Some(callback) = on_item_action.get_value() {
            callback.run(id);
        }
    });

    let toggle_submenu = Callback::new(move |id: String| {
        if disabled || !allow_submenu_collapse {
            return;
        }

        set_open_sub_ids.update(|open_sub_ids| {
            if open_sub_ids.contains(&id) {
                open_sub_ids.remove(&id);
            } else {
                open_sub_ids.insert(id);
            }
        });
    });

    let on_key_down = move |event: ev::KeyboardEvent| {
        if disabled {
            return;
        }

        if let Some(shortcut_key) = shortcut_key.get_value()
            && (event.ctrl_key() || event.meta_key())
            && event.key().eq_ignore_ascii_case(&shortcut_key)
        {
            let first = logic::first_enabled_id(items.get_value().as_ref());
            request_active_id_change.run(first);
            event.prevent_default();
            return;
        }

        if let Some(next) =
            logic::next_id_for_key(&event.key(), items.get_value().as_ref(), active_id.get())
        {
            request_active_id_change.run(Some(next));
            event.prevent_default();
            return;
        }

        if (event.key() == "Enter" || event.key() == " ")
            && let Some(active) = active_id.get()
        {
            trigger_action.run(active);
            event.prevent_default();
        }
    };

    let render_ctx = SidebarMenuRenderCtx {
        active_id,
        open_sub_ids,
        show_badges,
        show_actions,
        allow_submenu_collapse,
        disabled,
        id_base,
        select_id,
        trigger_action,
        trigger_item_action,
        toggle_submenu,
    };

    view! {
        <nav
            class=move || class.get()
            data-slot="sidebar-menu"
            data-state=move || state.get().state_attr
            data-count=item_count.to_string()
            data-empty=move || state.get().is_empty.then_some("true")
            data-disabled=move || state.get().disabled.then_some("true")
            data-enabled=move || state.get().enabled.then_some("true")
            data-show-badges=move || state.get().show_badges.then_some("true")
            data-show-actions=move || state.get().show_actions.then_some("true")
            data-collapsible-sub=move || state.get().allow_submenu_collapse.then_some("true")
            data-controlled=move || state.get().is_controlled.then_some("true")
            data-uncontrolled=move || state.get().is_uncontrolled.then_some("true")
            data-control-mode=move || state.get().control_attr
            data-active-id=move || active_id.get().unwrap_or_default()
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            role="navigation"
            aria-label=aria_label.get_value()
            on:keydown=on_key_down
        >
            <div class="ui-sidebar-menu__list" data-slot="sidebar-menu-list" node_ref=legend_ref>
                <div class="ui-sidebar-menu__highlight" data-slot="sidebar-menu-highlight" node_ref=highlight_ref></div>

                {move || {
                    items
                        .get_value()
                        .iter()
                        .enumerate()
                        .map(|(index, item)| render_ctx.render_menu_item(index, item))
                        .collect_view()
                }}
            </div>
        </nav>
    }
}

fn render_sub_item(
    index: usize,
    sub_item: &SidebarMenuSubItem,
    active_id: Signal<Option<String>>,
    parent_disabled: bool,
    select_id: Callback<String>,
    trigger_action: Callback<String>,
) -> AnyView {
    let sub_id = sub_item.id.clone();
    let sub_label = sub_item.label.clone();
    let sub_href = sub_item.href.clone();
    let sub_disabled = parent_disabled || sub_item.disabled;

    let on_focus = {
        let sub_id = sub_id.clone();
        move |_: ev::FocusEvent| {
            select_id.run(sub_id.clone());
        }
    };

    let on_pointer_enter = {
        let sub_id = sub_id.clone();
        move |_: ev::PointerEvent| {
            select_id.run(sub_id.clone());
        }
    };

    let on_click = {
        let sub_id = sub_id.clone();
        move |_| {
            trigger_action.run(sub_id.clone());
        }
    };

    let sub_active = {
        let sub_id = sub_id.clone();
        move || (active_id.get().as_deref() == Some(sub_id.as_str())).then_some("true")
    };

    view! {
        <button
            class="ui-sidebar-menu__sub-button"
            data-slot="sidebar-menu-sub-button"
            data-index=index.to_string()
            data-id=sub_id
            data-active=sub_active
            type="button"
            disabled=sub_disabled
            aria-disabled=sub_disabled.then_some("true")
            on:focus=on_focus
            on:pointerenter=on_pointer_enter
            on:click=on_click
        >
            <span class="ui-sidebar-menu__sub-label" data-slot="sidebar-menu-sub-label">{sub_label}</span>
            {match sub_href {
                Some(sub_href) => {
                    view! {
                        <span class="ui-sidebar-menu__sub-href" data-slot="sidebar-menu-sub-href">
                            {sub_href}
                        </span>
                    }
                    .into_any()
                }
                None => ().into_any(),
            }}
        </button>
    }
    .into_any()
}
