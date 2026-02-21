use crate::navigation_menu::{
    NavigationMenuItem, NavigationMenuMotion, NavigationMenuPartStateInput, NavigationMenuSlot,
    logic,
};
use leptos::{ev, html, prelude::*};
use std::sync::Arc;
use ui_headless as overlay_open;
use ui_visual_primitive::active_highlight::{
    ActiveHighlightMotion, attach_active_highlight_motion,
};

#[cfg(target_arch = "wasm32")]
fn focus_item(item_refs: &Arc<Vec<NodeRef<html::A>>>, index: usize) {
    let Some(node_ref) = item_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    ui_observability::observe_js_result!(el.focus());
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
    #[prop(optional)] motion: NavigationMenuMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion: ActiveHighlightMotion = crate::navigation_menu::motion::sanitize_motion(motion);
    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;
    let id_base = StoredValue::new(id_base);

    let items = logic::resolve_items(&id_base.get_value(), items);
    let items: StoredValue<Arc<[crate::navigation_menu::NavigationMenuItemResolved]>> =
        StoredValue::new(Arc::from(items));
    let item_count = items.get_value().len();

    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let has_custom_activate_on_focus = activate_on_focus != logic::DEFAULT_ACTIVATE_ON_FOCUS;
    let has_custom_selected_id = selected_id.is_some();
    let has_custom_default_selected_id = default_selected_id.is_some();
    let has_custom_on_selected_id_change = on_selected_id_change.is_some();
    let has_custom_motion = motion != NavigationMenuMotion::default();

    let default_selected_id = logic::sanitize_selected_id(
        logic::normalize_optional_text(default_selected_id),
        items.get_value().as_ref(),
    );

    let is_controlled = has_custom_selected_id;
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
    let (focused_index_raw, set_focused_index) = signal(initial_focus_index);

    let focused_index = Signal::derive(move || {
        logic::sanitize_focused_index(focused_index_raw.get(), items.get_value().as_ref()).or_else(
            || logic::resolve_initial_focus_index(items.get_value().as_ref(), selected_index.get()),
        )
    });

    let root_state = Memo::new(move |_| {
        logic::resolve_state(NavigationMenuPartStateInput {
            slot: NavigationMenuSlot::Root,
            item_count,
            selected_index: selected_index.get(),
            focused_index: focused_index.get(),
            has_disabled_items: items.get_value().iter().any(|item| item.disabled),
            activate_on_focus,
            is_controlled,
            has_custom_id_base,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_activate_on_focus,
            has_custom_selected_id,
            has_custom_default_selected_id,
            has_custom_on_selected_id_change,
            has_custom_motion,
        })
    });
    let root_state_for_class = root_state;
    let root_class = Memo::new(move |_| {
        logic::compose_class_name(class_name.get_value(), root_state_for_class.get())
    });

    let item_refs: Arc<Vec<NodeRef<html::A>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let list_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();

    let (active_index, set_active_index) = signal(logic::resolve_active_index(
        items.get_value().as_ref(),
        selected_index.get_untracked(),
        focused_index.get_untracked(),
    ));

    Effect::new(move |_| {
        let next = logic::resolve_active_index(
            items.get_value().as_ref(),
            selected_index.get(),
            focused_index.get(),
        );
        set_active_index.set(next);
    });

    let option_id = Callback::new(move |index: usize| {
        logic::resolve_option_id(items.get_value().as_ref(), index)
    });

    attach_active_highlight_motion(list_ref, highlight_ref, active_index, option_id, motion);

    let item_indices: StoredValue<Vec<usize>> = StoredValue::new((0..item_count).collect());

    let render_item = move |index: usize| {
        let item = items.get_value()[index].clone();
        let item_dom_id = StoredValue::new(item.dom_id);
        let item_label = StoredValue::new(item.label);
        let item_href = StoredValue::new(item.href);
        let item_disabled = item.disabled;
        let item_ref = item_refs[index];

        let on_focus = move |_| {
            if logic::should_ignore_item_interaction(item_disabled) {
                return;
            }

            set_focused_index.set(Some(index));
            if activate_on_focus
                && let Some(next_id) = logic::resolve_selected_id_for_target(
                    items.get_value().as_ref(),
                    index,
                    logic::NavigationSelectionTarget::Current,
                )
            {
                selected_state.request_change.run(Some(next_id));
            }
        };

        let on_pointer_enter = move |_| {
            if logic::should_ignore_item_interaction(item_disabled) {
                return;
            }

            set_focused_index.set(Some(index));
            if activate_on_focus
                && let Some(next_id) = logic::resolve_selected_id_for_target(
                    items.get_value().as_ref(),
                    index,
                    logic::NavigationSelectionTarget::Current,
                )
            {
                selected_state.request_change.run(Some(next_id));
            }
        };

        let on_key_down = {
            let item_refs = item_refs.clone();
            move |ev: ev::KeyboardEvent| {
                if let Some(decision) = logic::resolve_key_decision(
                    &ev.key(),
                    item_disabled,
                    index,
                    items.get_value().as_ref(),
                    activate_on_focus,
                ) {
                    if let Some(next_index) = decision.next_focus_index {
                        set_focused_index.set(Some(next_index));
                        focus_item(&item_refs, next_index);
                    }
                    if let Some(target) = decision.selection_target
                        && let Some(next_id) = logic::resolve_selected_id_for_target(
                            items.get_value().as_ref(),
                            index,
                            target,
                        )
                    {
                        selected_state.request_change.run(Some(next_id));
                    }
                    ev.prevent_default();
                }
            }
        };

        let on_click = move |ev: ev::MouseEvent| {
            if logic::should_ignore_item_interaction(item_disabled) {
                ev.prevent_default();
                return;
            }

            set_focused_index.set(Some(index));
            if let Some(next_id) = logic::resolve_selected_id_for_target(
                items.get_value().as_ref(),
                index,
                logic::NavigationSelectionTarget::Current,
            ) {
                selected_state.request_change.run(Some(next_id));
            }
        };

        let item_slot = NavigationMenuSlot::Item;

        view! {
            <a
                id=item_dom_id.get_value()
                class=item_slot.base_class()
                node_ref=item_ref
                href=item_href.get_value()
                tabindex=move || {
                    logic::resolve_item_tabindex(item_disabled, focused_index.get() == Some(index))
                }
                aria-current=move || (selected_index.get() == Some(index)).then_some("page")
                aria-disabled=item_disabled.then_some("true")
                data-slot=item_slot.as_attr()
                data-index=index
                data-state=move || {
                    logic::resolve_item_state_attr(
                        item_disabled,
                        selected_index.get() == Some(index),
                        focused_index.get() == Some(index),
                    )
                }
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

    let list_slot = NavigationMenuSlot::List;
    let highlight_slot = NavigationMenuSlot::Highlight;

    view! {
        <nav
            class=move || root_class.get()
            role="navigation"
            aria-label=aria_label.get_value()
            data-slot=move || root_state.get().slot_attr
            data-state=move || root_state.get().state_attr
            data-items=move || root_state.get().item_attr
            data-selection=move || root_state.get().selected_attr
            data-focus=move || root_state.get().focus_attr
            data-open=move || root_state.get().open_attr
            data-closed=move || root_state.get().closed_attr
            data-empty=move || root_state.get().is_empty.then_some("true")
            data-has-items=move || root_state.get().has_items.then_some("true")
            data-item-count=move || root_state.get().item_count
            data-selected-index=move || root_state.get().selected_index
            data-focused-index=move || root_state.get().focused_index
            data-has-selection=move || root_state.get().has_selection.then_some("true")
            data-has-focus=move || root_state.get().has_focus.then_some("true")
            data-has-disabled-items=move || root_state.get().has_disabled_items.then_some("true")
            data-focus-activation=move || root_state.get().focus_activation_attr
            data-selection-mode=move || root_state.get().selection_mode_attr
            data-selected-id=move || selected_id.get()
            data-id-source=move || root_state.get().id_source_attr
            data-aria-label-source=move || root_state.get().aria_label_source_attr
            data-class-source=move || root_state.get().class_source_attr
            data-activate-on-focus-source=move || root_state.get().activate_on_focus_source_attr
            data-selected-id-source=move || root_state.get().selected_id_source_attr
            data-default-selected-id-source=move || root_state.get().default_selected_id_source_attr
            data-selected-id-change-source=move || root_state.get().selected_id_change_source_attr
            data-motion-source=move || root_state.get().motion_source_attr
            data-custom-id=move || root_state.get().has_custom_id_base.then_some("true")
            data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some("true")
            data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
            data-custom-activate-on-focus=move || {
                root_state.get().has_custom_activate_on_focus.then_some("true")
            }
            data-custom-selected-id=move || root_state.get().has_custom_selected_id.then_some("true")
            data-custom-default-selected-id=move || {
                root_state.get().has_custom_default_selected_id.then_some("true")
            }
            data-custom-selected-id-change=move || {
                root_state.get().has_custom_on_selected_id_change.then_some("true")
            }
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
        >
            <div class=list_slot.base_class() node_ref=list_ref data-slot=list_slot.as_attr()>
                <div
                    class=highlight_slot.base_class()
                    node_ref=highlight_ref
                    data-slot=highlight_slot.as_attr()
                ></div>
                <For each=move || item_indices.get_value() key=|index| *index children=render_item />
            </div>
        </nav>
    }
}
