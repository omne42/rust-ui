use super::logic::{
    TabsSelectionTrigger, normalize_index_skipping_disabled, resolve_next_selected_index,
};
use crate::tabs::{TabsKeyboardActivation, TabsMotion, motion};
use leptos::{children::ChildrenFragment as Children, ev, html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{
    FocusRingOptions, HoverOptions, PressOptions, RovingOrientation, RovingTabIndexOptions,
    use_focus_ring, use_hover, use_press, use_roving_tabindex,
};

#[cfg(target_arch = "wasm32")]
fn focus_tab(tab_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize) {
    let Some(node_ref) = tab_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    let _ = el.focus();
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_tab(_tab_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}

#[component]
pub fn Tabs(
    labels: Vec<&'static str>,
    id_base: String,
    #[prop(optional)] keyboard_activation: TabsKeyboardActivation,
    #[prop(optional)] default_selected_index: usize,
    #[prop(optional)] selected_index: Option<ReadSignal<usize>>,
    #[prop(optional)] on_selection_change: Option<Callback<usize>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] motion: TabsMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let panels = children().nodes;

    debug_assert_eq!(
        labels.len(),
        panels.iter().len(),
        "Tabs: expected `labels.len() == children.len()`; got labels={}, children={}",
        labels.len(),
        panels.iter().len()
    );

    let item_count = labels.len().min(panels.iter().len());
    let (item_count_signal, _set_item_count) = signal(item_count);

    let disabled_indices: Arc<HashSet<usize>> = Arc::new(disabled_indices.into_iter().collect());

    let initial_selected = normalize_index_skipping_disabled(
        selected_index
            .map(|signal| signal.get_untracked())
            .unwrap_or(default_selected_index),
        item_count,
        {
            let disabled_indices = disabled_indices.clone();
            move |index: usize| disabled || disabled_indices.contains(&index)
        },
    );
    let (uncontrolled_selected, set_uncontrolled_selected) = signal(initial_selected);

    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: disabled,
        default_index: initial_selected,
        should_loop: true,
        orientation: RovingOrientation::Horizontal,
        item_count: item_count_signal,
        is_item_disabled: (!disabled_indices.is_empty()).then_some({
            let disabled_indices = disabled_indices.clone();
            Callback::new(move |index: usize| disabled_indices.contains(&index))
        }),
    });

    let selected_raw = selected_index.unwrap_or(uncontrolled_selected);
    let selected = Signal::derive({
        let disabled_indices = disabled_indices.clone();
        move || {
            normalize_index_skipping_disabled(selected_raw.get(), item_count, |index: usize| {
                disabled || disabled_indices.contains(&index)
            })
        }
    });

    let set_selected = Callback::new({
        let is_controlled = selected_index.is_some();
        let disabled_indices = disabled_indices.clone();
        move |index: usize| {
            if item_count == 0 {
                return;
            }
            let next = normalize_index_skipping_disabled(index, item_count, |idx: usize| {
                disabled || disabled_indices.contains(&idx)
            });
            if disabled || disabled_indices.contains(&next) {
                return;
            }
            if selected.get_untracked() == next {
                return;
            }
            if let Some(on_selection_change) = on_selection_change {
                on_selection_change.run(next);
            }
            if !is_controlled {
                set_uncontrolled_selected.set(next);
            }
        }
    });

    let tab_refs: Arc<Vec<NodeRef<html::Button>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let list_ref: NodeRef<html::Div> = NodeRef::new();
    let indicator_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(list_ref, indicator_ref, tab_refs.clone(), selected, motion);

    let tabs_view = labels
        .into_iter()
        .take(item_count)
        .enumerate()
        .map({
            let id_base = id_base.clone();
            let roving = roving.clone();
            let tab_refs = tab_refs.clone();
            move |(index, label)| {
                let tab_id = format!("{id_base}-tab-{index}");
                let panel_id = format!("{id_base}-panel-{index}");
                let node_ref = tab_refs[index];

                let is_selected = move || selected.get() == index;
                let is_disabled = disabled || disabled_indices.contains(&index);

                let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
                let hover = use_hover(HoverOptions { is_disabled });
                let press = use_press(PressOptions {
                    is_disabled,
                    on_press: Some(Callback::new({
                        let disabled_indices = disabled_indices.clone();
                        move |_| {
                            let current = selected.get_untracked();
                            let next = resolve_next_selected_index(
                                current,
                                index,
                                item_count,
                                |idx: usize| disabled || disabled_indices.contains(&idx),
                                keyboard_activation,
                                TabsSelectionTrigger::Press,
                            );
                            if next != current {
                                set_selected.run(next);
                            }
                        }
                    })),
                    prevent_default_for_keyboard: true,
                    ..Default::default()
                });

                let on_focus = {
                    let disabled_indices = disabled_indices.clone();
                    move |_| {
                        roving.handlers.on_item_focus.run(index);
                        focus_ring.handlers.on_focus.run(());
                        let current = selected.get_untracked();
                        let next = resolve_next_selected_index(
                            current,
                            index,
                            item_count,
                            |idx: usize| disabled || disabled_indices.contains(&idx),
                            keyboard_activation,
                            TabsSelectionTrigger::Focus,
                        );
                        if next != current {
                            set_selected.run(next);
                        }
                    }
                };

                let on_blur = move |_| {
                    press.handlers.on_blur.run(());
                    focus_ring.handlers.on_blur.run(());
                };

                let on_key_down = {
                    let on_key_down = roving.handlers.on_key_down;
                    let on_press_key_down = press.handlers.on_key_down;
                    let active_index = roving.active_index;
                    let tab_refs = tab_refs.clone();
                    move |ev: ev::KeyboardEvent| {
                        let key = ev.key();
                        let handled_roving = on_key_down.run(key.clone());
                        let handled_press = on_press_key_down.run(key);
                        if handled_roving || handled_press {
                            ev.prevent_default();
                        }
                        if handled_roving {
                            focus_tab(&tab_refs, active_index.get_untracked());
                        }
                    }
                };

                let on_key_up = move |ev: ev::KeyboardEvent| {
                    let key = ev.key();
                    if press.handlers.on_key_up.run(key) {
                        ev.prevent_default();
                    }
                };

                view! {
                    <button
                        type="button"
                        class="ui-tabs__tab"
                        id=tab_id
                        node_ref=node_ref
                        role="tab"
                        class:ui-tabs__tab--focus-visible=move || focus_ring.is_focus_visible.get()
                        disabled=is_disabled
                        tabindex=move || if roving.active_index.get() == index { 0 } else { -1 }
                        aria-selected=move || if is_selected() { "true" } else { "false" }
                        aria-controls=panel_id
                        aria-disabled=if is_disabled { Some("true") } else { None }
                        data-selected=move || if is_selected() { Some("true") } else { None }
                        data-disabled=if is_disabled { Some("true") } else { None }
                        data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
                        data-pressed=move || if press.is_pressed.get() { Some("true") } else { None }
                        on:pointerdown=move |_| press.handlers.on_pointer_down.run(())
                        on:pointerup=move |_| press.handlers.on_pointer_up.run(())
                        on:pointercancel=move |_| press.handlers.on_pointer_cancel.run(())
                        on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                        on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                        on:click=move |_| press.handlers.on_click.run(())
                        on:keydown=on_key_down
                        on:keyup=on_key_up
                        on:focus=on_focus
                        on:blur=on_blur
                    >
                        {label}
                    </button>
                }
            }
        })
        .collect_view();

    let panels_view = panels
        .into_iter()
        .take(item_count)
        .enumerate()
        .map({
            let id_base = id_base.clone();
            move |(index, panel)| {
                let tab_id = format!("{id_base}-tab-{index}");
                let panel_id = format!("{id_base}-panel-{index}");
                let is_selected = move || selected.get() == index;

                view! {
                    <div
                        class="ui-tabs__panel"
                        id=panel_id
                        role="tabpanel"
                        aria-labelledby=tab_id
                        hidden=move || !is_selected()
                    >
                        {panel}
                    </div>
                }
            }
        })
        .collect_view();

    view! {
        <div
            class=class_name
                .filter(|value| !value.trim().is_empty())
                .map(|value| format!("ui-tabs {value}"))
                .unwrap_or_else(|| "ui-tabs".to_string())
            data-slot="tabs"
        >
            <div
                class="ui-tabs__list"
                node_ref=list_ref
                role="tablist"
                aria-label=aria_label
                data-slot="tabs-list"
            >
                <div class="ui-tabs__indicator" node_ref=indicator_ref aria-hidden="true"></div>
                {tabs_view}
            </div>
            {panels_view}
        </div>
    }
}
