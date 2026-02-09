use crate::active_highlight::{ActiveHighlightMotion, attach_active_highlight_motion};
use crate::navigation_menu::{NavigationMenuItem, logic};
use crate::overlay_open;
use leptos::{ev, html, prelude::*};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
fn focus_item(item_refs: &Arc<Vec<NodeRef<html::A>>>, index: usize) {
    let Some(node_ref) = item_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    let _ = el.focus();
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_item(_item_refs: &Arc<Vec<NodeRef<html::A>>>, _index: usize) {}

#[component]
pub fn NavigationMenu(
    id_base: String,
    items: Vec<NavigationMenuItem>,
    #[prop(optional)] selected_id: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_selected_id: Option<String>,
    #[prop(optional)] on_selected_id_change: Option<Callback<Option<String>>>,
    #[prop(default = true)] activate_on_focus: bool,
    #[prop(optional)] motion: ActiveHighlightMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);

    let items = logic::resolve_items(&id_base, items);
    let items: StoredValue<Arc<[logic::NavigationMenuItemResolved]>> =
        StoredValue::new(Arc::from(items));
    let item_count = items.get_value().len();

    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let default_selected_id = logic::sanitize_selected_id(
        logic::normalize_optional_text(default_selected_id),
        items.get_value().as_ref(),
    );

    let selected_state = overlay_open::use_controllable_state(
        selected_id,
        Some(default_selected_id),
        on_selected_id_change,
    );

    let selected_id = Signal::derive(move || {
        logic::sanitize_selected_id(selected_state.value.get(), items.get_value().as_ref())
    });

    let selected_index = Signal::derive(move || {
        logic::selected_index_for_id(items.get_value().as_ref(), selected_id.get())
    });

    let initial_focus_index = logic::resolve_initial_focus_index(
        items.get_value().as_ref(),
        selected_index.get_untracked(),
    );
    let (focused_index, set_focused_index) = signal(initial_focus_index);

    let focused_index = Signal::derive(move || {
        logic::sanitize_focused_index(focused_index.get(), items.get_value().as_ref()).or_else(
            || logic::resolve_initial_focus_index(items.get_value().as_ref(), selected_index.get()),
        )
    });

    let state = Signal::derive(move || {
        logic::resolve_state(logic::NavigationMenuStateInput {
            item_count,
            selected_index: selected_index.get(),
            focused_index: focused_index.get(),
            has_disabled_items: items.get_value().iter().any(|item| item.disabled),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));

    let item_refs: Arc<Vec<NodeRef<html::A>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let list_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();

    let (active_index, set_active_index) = signal(
        selected_index
            .get_untracked()
            .or(focused_index.get_untracked())
            .or_else(|| logic::first_enabled_index(items.get_value().as_ref()))
            .unwrap_or(0),
    );

    Effect::new(move |_| {
        let next = selected_index
            .get()
            .or(focused_index.get())
            .or_else(|| logic::first_enabled_index(items.get_value().as_ref()))
            .unwrap_or(0);
        set_active_index.set(next);
    });

    let option_id = Callback::new(move |index: usize| {
        items
            .get_value()
            .get(index)
            .map(|item| item.dom_id.clone())
            .unwrap_or_default()
    });

    attach_active_highlight_motion(list_ref, highlight_ref, active_index, option_id, motion);

    let item_indices: StoredValue<Vec<usize>> = StoredValue::new((0..item_count).collect());

    let render_item = move |index: usize| {
        let item = items.get_value()[index].clone();
        let item_id = StoredValue::new(item.id);
        let item_dom_id = StoredValue::new(item.dom_id);
        let item_label = StoredValue::new(item.label);
        let item_href = StoredValue::new(item.href);
        let item_disabled = item.disabled;
        let item_ref = item_refs[index];

        let on_focus = move |_| {
            if item_disabled {
                return;
            }

            set_focused_index.set(Some(index));
            if activate_on_focus {
                selected_state.request_change.run(Some(item_id.get_value()));
            }
        };

        let on_pointer_enter = move |_| {
            if item_disabled {
                return;
            }

            set_focused_index.set(Some(index));
            if activate_on_focus {
                selected_state.request_change.run(Some(item_id.get_value()));
            }
        };

        let on_key_down = {
            let item_refs = item_refs.clone();
            move |ev: ev::KeyboardEvent| {
                if item_disabled {
                    return;
                }

                let key = ev.key();
                match key.as_str() {
                    "ArrowRight" => {
                        if let Some(next_index) =
                            logic::next_enabled_index(items.get_value().as_ref(), index, 1)
                        {
                            set_focused_index.set(Some(next_index));
                            if activate_on_focus
                                && let Some(next_item) = items.get_value().get(next_index)
                            {
                                selected_state
                                    .request_change
                                    .run(Some(next_item.id.clone()));
                            }
                            focus_item(&item_refs, next_index);
                            ev.prevent_default();
                        }
                    }
                    "ArrowLeft" => {
                        if let Some(next_index) =
                            logic::next_enabled_index(items.get_value().as_ref(), index, -1)
                        {
                            set_focused_index.set(Some(next_index));
                            if activate_on_focus
                                && let Some(next_item) = items.get_value().get(next_index)
                            {
                                selected_state
                                    .request_change
                                    .run(Some(next_item.id.clone()));
                            }
                            focus_item(&item_refs, next_index);
                            ev.prevent_default();
                        }
                    }
                    "Home" => {
                        if let Some(next_index) =
                            logic::first_enabled_index(items.get_value().as_ref())
                        {
                            set_focused_index.set(Some(next_index));
                            if activate_on_focus
                                && let Some(next_item) = items.get_value().get(next_index)
                            {
                                selected_state
                                    .request_change
                                    .run(Some(next_item.id.clone()));
                            }
                            focus_item(&item_refs, next_index);
                            ev.prevent_default();
                        }
                    }
                    "End" => {
                        if let Some(next_index) =
                            logic::last_enabled_index(items.get_value().as_ref())
                        {
                            set_focused_index.set(Some(next_index));
                            if activate_on_focus
                                && let Some(next_item) = items.get_value().get(next_index)
                            {
                                selected_state
                                    .request_change
                                    .run(Some(next_item.id.clone()));
                            }
                            focus_item(&item_refs, next_index);
                            ev.prevent_default();
                        }
                    }
                    "Enter" | " " => {
                        selected_state.request_change.run(Some(item_id.get_value()));
                        ev.prevent_default();
                    }
                    _ => {}
                }
            }
        };

        let on_click = move |ev: ev::MouseEvent| {
            if item_disabled {
                ev.prevent_default();
                return;
            }

            set_focused_index.set(Some(index));
            selected_state.request_change.run(Some(item_id.get_value()));
        };

        view! {
            <a
                id=item_dom_id.get_value()
                class="ui-navigation-menu__item"
                node_ref=item_ref
                href=item_href.get_value()
                role="link"
                tabindex=move || {
                    if item_disabled {
                        "-1"
                    } else if focused_index.get() == Some(index) {
                        "0"
                    } else {
                        "-1"
                    }
                }
                aria-current=move || {
                    (selected_index.get() == Some(index)).then_some("page")
                }
                aria-disabled=item_disabled.then_some("true")
                data-slot="navigation-menu-item"
                data-index=index
                data-selected=move || (selected_index.get() == Some(index)).then_some("true")
                data-focused=move || (focused_index.get() == Some(index)).then_some("true")
                data-disabled=item_disabled.then_some("true")
                on:focus=on_focus
                on:pointerenter=on_pointer_enter
                on:keydown=on_key_down
                on:click=on_click
            >
                {item_label.get_value()}
            </a>
        }
    };

    view! {
        <nav
            class=move || class.get()
            role="navigation"
            aria-label=aria_label
            data-slot="navigation-menu"
            data-state=move || state.get().data_state_attr
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-item-count=move || state.get().item_count.to_string()
            data-selected-index=move || state.get().selected_index.map(|index| index.to_string())
            data-focused-index=move || state.get().focused_index.map(|index| index.to_string())
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-has-focus=move || state.get().has_focus.then_some("true")
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-custom-label=state.get_untracked().has_custom_aria_label.then_some("true")
            data-custom-class=state.get_untracked().has_custom_class_name.then_some("true")
            data-selected-id=move || selected_id.get()
        >
            <div class="ui-navigation-menu__list" node_ref=list_ref data-slot="navigation-menu-list">
                <div class="ui-active-highlight" node_ref=highlight_ref data-slot="navigation-menu-highlight"></div>
                <For each=move || item_indices.get_value() key=|index| *index children=render_item />
            </div>
        </nav>
    }
}
