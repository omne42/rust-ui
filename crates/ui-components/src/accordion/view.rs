use crate::accordion::{AccordionSelectionMode, logic::toggle_open_indices};
use leptos::{children::ChildrenFragment as Children, ev, html, prelude::*};
use std::{collections::BTreeSet, sync::Arc};
use ui_headless::{
    FocusRingOptions, HoverOptions, RovingOrientation, RovingTabIndexOptions, use_focus_ring,
    use_hover, use_roving_tabindex,
};

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
pub fn Accordion(
    labels: Vec<String>,
    id_base: String,
    open_indices: ReadSignal<BTreeSet<usize>>,
    set_open_indices: WriteSignal<BTreeSet<usize>>,
    #[prop(optional)] selection_mode: AccordionSelectionMode,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let panels = children().nodes;

    debug_assert_eq!(
        labels.len(),
        panels.iter().len(),
        "Accordion: expected `labels.len() == children.len()`; got labels={}, children={}",
        labels.len(),
        panels.iter().len()
    );

    let item_count = labels.len().min(panels.iter().len());
    let (item_count_signal, _set_item_count) = signal(item_count);

    let disabled_indices: Arc<Vec<usize>> = Arc::new(disabled_indices);
    let has_disabled = !disabled_indices.is_empty();
    let disabled_indices_for_cb = disabled_indices.clone();
    let is_item_disabled = has_disabled.then_some(Callback::new(move |index: usize| {
        disabled_indices_for_cb.contains(&index)
    }));

    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: disabled,
        default_index: 0,
        should_loop: true,
        orientation: RovingOrientation::Vertical,
        item_count: item_count_signal,
        is_item_disabled,
    });

    let trigger_refs: Arc<Vec<NodeRef<html::Button>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let base_class = "ui-accordion".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let labels = labels.into_iter().take(item_count).collect::<Vec<_>>();
    let panels = panels.into_iter().take(item_count);

    let items = labels
        .into_iter()
        .zip(panels)
        .enumerate()
        .map({
            let id_base = id_base.clone();
            let roving = roving.clone();
            let trigger_refs = trigger_refs.clone();
            let disabled_indices = disabled_indices.clone();
            move |(index, (label, panel))| {
                let trigger_id = format!("{id_base}-trigger-{index}");
                let panel_id = format!("{id_base}-panel-{index}");

                let is_disabled = disabled || disabled_indices.contains(&index);

                let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
                let hover = use_hover(HoverOptions { is_disabled });

                let is_open = move || open_indices.with(|set| set.contains(&index));

                let on_toggle = move |_| {
                    if is_disabled {
                        return;
                    }
                    set_open_indices.update(|set| {
                        *set = toggle_open_indices(selection_mode, set, index);
                    });
                };

                let on_key_down = {
                    let on_key_down = roving.handlers.on_key_down;
                    let active_index = roving.active_index;
                    let trigger_refs = trigger_refs.clone();
                    move |ev: ev::KeyboardEvent| {
                        if on_key_down.run(ev.key()) {
                            ev.prevent_default();
                            focus_trigger(&trigger_refs, active_index.get_untracked());
                        }
                    }
                };

                let node_ref = trigger_refs[index];

                view! {
                    <div
                        class="ui-accordion__item"
                        data-slot="accordion-item"
                        data-open=move || if is_open() { Some("true") } else { None }
                    >
                        <button
                            type="button"
                            class="ui-accordion__trigger"
                            class:ui-accordion__trigger--focus-visible=move || focus_ring.is_focus_visible.get()
                            node_ref=node_ref
                            id=trigger_id.clone()
                            disabled=is_disabled
                            tabindex=move || {
                                if is_disabled {
                                    -1
                                } else if roving.active_index.get() == index {
                                    0
                                } else {
                                    -1
                                }
                            }
                            aria-expanded=move || if is_open() { "true" } else { "false" }
                            aria-controls=panel_id.clone()
                            data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
                            on:focus=move |_| {
                                focus_ring.handlers.on_focus.run(());
                                roving.handlers.on_item_focus.run(index);
                            }
                            on:blur=move |_| focus_ring.handlers.on_blur.run(())
                            on:keydown=on_key_down
                            on:click=on_toggle
                            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                        >
                            <span class="ui-accordion__label" data-slot="accordion-label">
                                {label}
                            </span>
                            <span class="ui-accordion__indicator" aria-hidden="true" data-slot="accordion-indicator">
                                "›"
                            </span>
                        </button>

                        <div
                            id=panel_id
                            class="ui-accordion__panel"
                            role="region"
                            aria-labelledby=trigger_id
                            hidden=move || !is_open()
                            data-slot="accordion-panel"
                        >
                            {panel}
                        </div>
                    </div>
                }
            }
        })
        .collect_view();

    view! {
        <div class=class data-slot="accordion">
            {items}
        </div>
    }
}
