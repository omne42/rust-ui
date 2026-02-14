use crate::accordion::{AccordionMotion, AccordionSelectionMode, logic, motion};
use leptos::{children::ChildrenFragment as Children, ev, html, prelude::*};
use std::{collections::BTreeSet, sync::Arc};
use ui_headless::a11y::aria_expanded;
use ui_headless::{
    FocusRingOptions, HoverOptions, PressOptions, RovingOrientation, RovingTabIndexOptions,
    use_focus_ring, use_hover, use_press, use_roving_tabindex,
};
use ui_state_primitives::controlled::{
    ControlledOnChange, ControlledStateOptions, use_controlled_state,
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

/// Accordion component with roving focus, disclosure semantics, and optional spring motion.
///
/// Public props:
/// - `labels`: trigger text for each panel; caller controls localization.
/// - `id_base`: stable prefix used for `id`/`aria-*` wiring between trigger and panel.
/// - `open_indices`: controlled open state (`Some`) for controlled mode.
/// - `default_open_indices`: uncontrolled initial open state.
/// - `on_open_change`: callback emitted after user toggles open state.
/// - `selection_mode`: single or multiple panel selection behavior.
/// - `disabled`: disable all triggers.
/// - `disabled_indices`: disable specific trigger indices.
/// - `motion`: per-instance motion contract overrides.
/// - `class_name`: optional extra class names merged onto root element.
/// - `children`: panel contents aligned by index with `labels`.
#[component]
pub fn Accordion(
    labels: Vec<String>,
    id_base: String,
    #[prop(optional)] open_indices: Option<Signal<BTreeSet<usize>>>,
    #[prop(optional)] default_open_indices: Option<BTreeSet<usize>>,
    #[prop(optional)] on_open_change: Option<Callback<BTreeSet<usize>>>,
    #[prop(optional)] selection_mode: AccordionSelectionMode,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] motion: AccordionMotion,
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

    let default_open_indices = logic::normalize_open_indices_for_items(
        selection_mode,
        &default_open_indices.unwrap_or_default(),
        item_count,
    );
    let open_state_source = if open_indices.is_some() {
        "controlled"
    } else {
        "uncontrolled"
    };

    let on_open_change: Option<ControlledOnChange<BTreeSet<usize>>> = on_open_change.map(|cb| {
        Arc::new(move |next: BTreeSet<usize>| cb.run(next)) as ControlledOnChange<BTreeSet<usize>>
    });

    let open_state = RwSignal::new(use_controlled_state(
        BTreeSet::new(),
        ControlledStateOptions {
            value: open_indices.map(|value| value.get_untracked()),
            default_value: Some(default_open_indices),
            on_change: on_open_change,
        },
    ));

    if let Some(open_indices) = open_indices {
        Effect::new(move |_| {
            open_state.update(|state| state.sync_controlled(Some(open_indices.get())));
        });
    }

    let open_indices = Memo::new({
        move |_| {
            open_state.with(|state| {
                logic::normalize_open_indices_for_items(selection_mode, state.value(), item_count)
            })
        }
    });
    let request_open_change = {
        Callback::new(move |next: BTreeSet<usize>| {
            let next = logic::normalize_open_indices_for_items(selection_mode, &next, item_count);
            open_state.update(|state| state.set_value(next));
        })
    };

    let disabled_indices: Arc<Vec<usize>> = Arc::new(disabled_indices);
    let has_per_item_disabled = !disabled_indices.is_empty();
    let has_disabled_items = disabled || has_per_item_disabled;
    let disabled_indices_for_cb = disabled_indices.clone();
    let is_item_disabled = has_per_item_disabled.then_some(Callback::new(move |index: usize| {
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

    let open_indices_for_state = open_indices;
    let state = Signal::derive(move || {
        logic::resolve_state(
            selection_mode,
            item_count,
            open_indices_for_state.get().len(),
            has_disabled_items,
        )
    });

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
                let open: Signal<bool> = Signal::derive(is_open);

                let indicator_ref: NodeRef<html::Span> = NodeRef::new();
                motion::attach_indicator_motion(indicator_ref, open, motion);

                let panel_ref: NodeRef<html::Div> = NodeRef::new();
                let panel_surface_ref: NodeRef<html::Div> = NodeRef::new();
                let panel_hidden = RwSignal::new(!open.get_untracked());
                motion::attach_panel_motion(panel_ref, panel_surface_ref, open, panel_hidden, motion);

                let on_press = Callback::new(move |_| {
                    let next = logic::toggle_open_indices_for_items(
                        selection_mode,
                        &open_indices.get_untracked(),
                        index,
                    );
                    request_open_change.run(next);
                });

                let press = use_press(PressOptions {
                    is_disabled,
                    on_press: Some(on_press),
                    prevent_default_for_keyboard: true,
                    ..Default::default()
                });

                let on_key_down = {
                    let on_key_down = roving.handlers.on_key_down;
                    let on_press_key_down = press.handlers.on_key_down;
                    let active_index = roving.active_index;
                    let trigger_refs = trigger_refs.clone();
                    move |ev: ev::KeyboardEvent| {
                        let key = ev.key();
                        let handled_roving = on_key_down.run(key.clone());
                        let handled_press = on_press_key_down.run(key);

                        if handled_roving || handled_press {
                            ev.prevent_default();
                        }

                        if handled_roving {
                            focus_trigger(&trigger_refs, active_index.get_untracked());
                        }
                    }
                };

                let on_key_up = move |ev: ev::KeyboardEvent| {
                    let key = ev.key();
                    if press.handlers.on_key_up.run(key) {
                        ev.prevent_default();
                    }
                };

                let node_ref = trigger_refs[index];

                view! {
                    <div
                        class="ui-accordion__item"
                        data-slot="accordion-item"
                        data-index=index
                        data-open=move || if open.get() { Some("true") } else { None }
                    >
                        <button
                            type="button"
                            class="ui-accordion__trigger"
                            class:ui-accordion__trigger--focus-visible=move || focus_ring.is_focus_visible.get()
                            node_ref=node_ref
                            id=trigger_id.clone()
                            disabled=is_disabled
                            data-slot="accordion-trigger"
                            data-index=index
                            data-open=move || open.get().then_some("true")
                            tabindex=move || {
                                if is_disabled {
                                    -1
                                } else if roving.active_index.get() == index {
                                    0
                                } else {
                                    -1
                                }
                            }
                            aria-expanded=aria_expanded(open)
                            aria-controls=panel_id.clone()
                            data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
                            data-pressed=move || if press.is_pressed.get() { Some("true") } else { None }
                            data-focused=move || focus_ring.is_focused.get().then_some("true")
                            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
                            data-disabled=is_disabled.then_some("true")
                            on:focus=move |_| {
                                focus_ring.handlers.on_focus.run(());
                                roving.handlers.on_item_focus.run(index);
                            }
                            on:blur=move |_| {
                                press.handlers.on_blur.run(());
                                focus_ring.handlers.on_blur.run(());
                            }
                            on:keydown=on_key_down
                            on:keyup=on_key_up
                            on:pointerdown=move |_| press.handlers.on_pointer_down.run(())
                            on:pointerup=move |_| press.handlers.on_pointer_up.run(())
                            on:pointercancel=move |_| press.handlers.on_pointer_cancel.run(())
                            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                            on:click=move |_| press.handlers.on_click.run(())
                        >
                            <span class="ui-accordion__label" data-slot="accordion-label">
                                {label}
                            </span>
                            <span
                                class="ui-accordion__indicator"
                                node_ref=indicator_ref
                                aria-hidden="true"
                                data-slot="accordion-indicator"
                            >
                                "›"
                            </span>
                        </button>

                        <div
                            id=panel_id
                            class="ui-accordion__panel"
                            node_ref=panel_ref
                            role="region"
                            aria-labelledby=trigger_id
                            hidden=move || panel_hidden.get()
                            data-open=move || if open.get() { Some("true") } else { None }
                            data-index=index
                            data-slot="accordion-panel"
                        >
                            <div
                                class="ui-accordion__panel-surface"
                                node_ref=panel_surface_ref
                                data-slot="accordion-panel-surface"
                            >
                                {panel}
                            </div>
                        </div>
                    </div>
                }
            }
        })
        .collect_view();

    view! {
        <div
            class=class
            data-slot="accordion"
            data-disabled=disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-open-count=move || state.get().open_count.to_string()
            data-all-closed=move || (!state.get().has_open_items).then_some("true")
            data-multiple-open=move || state.get().has_multiple_open.then_some("true")
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-open-state-source=open_state_source
            data-selection-mode=match selection_mode {
                AccordionSelectionMode::Single => "single",
                AccordionSelectionMode::Multiple => "multiple",
            }
            data-motion-source=if motion == AccordionMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != AccordionMotion::default()).then_some("true")
        >
            {items}
        </div>
    }
}
