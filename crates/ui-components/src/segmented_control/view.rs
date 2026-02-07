use crate::segmented_control::{
    SegmentedControlMotion, SegmentedControlOrientation, SegmentedControlSize, logic, motion,
};
use leptos::{ev, html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{
    FocusRingOptions, HoverOptions, RadioGroupOptions, use_focus_ring, use_hover, use_radio_group,
};

#[cfg(target_arch = "wasm32")]
fn focus_option(option_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize) {
    let Some(node_ref) = option_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    let _ = el.focus();
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_option(_option_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}

#[component]
pub fn SegmentedControl(
    id_base: String,
    options: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] orientation: SegmentedControlOrientation,
    #[prop(optional)] size: SegmentedControlSize,
    #[prop(optional)] motion: SegmentedControlMotion,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let options: StoredValue<Arc<[String]>> = StoredValue::new(options.into());
    let item_count = options.get_value().len();
    let (item_count_signal, _set_item_count) = signal(item_count);

    let disabled_index_set: HashSet<usize> = disabled_indices.into_iter().collect();
    let has_disabled = !disabled_index_set.is_empty();
    let disabled_indices_set: Arc<HashSet<usize>> = Arc::new(disabled_index_set);

    let is_item_disabled = has_disabled.then_some({
        let disabled_indices_set = disabled_indices_set.clone();
        Callback::new(move |index: usize| disabled_indices_set.contains(&index))
    });

    let aria = use_radio_group(RadioGroupOptions {
        is_disabled: disabled,
        id_base: id_base.clone(),
        orientation: orientation.roving_orientation(),
        item_count: item_count_signal,
        selected_index,
        set_selected_index,
        on_change: None,
        is_item_disabled,
    });

    let option_refs: Arc<Vec<NodeRef<html::Button>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .or_else(|| label.clone())
        .unwrap_or_else(|| "Segmented control".to_string());

    let label = label.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    });
    let has_label = label.is_some();
    let label_id = label.as_ref().map(|_| format!("{id_base}-label"));
    let label_id = StoredValue::new(label_id);
    let label = StoredValue::new(label);

    let disabled_indices_for_state = disabled_indices_set.clone();
    let state = Memo::new(move |_| {
        logic::resolve_state(
            item_count,
            disabled,
            disabled_indices_for_state.as_ref(),
            aria.selected_index.get(),
            orientation,
            has_label,
        )
    });

    let base_class = format!(
        "ui-segmented-control {} {} {}",
        orientation.class_name(),
        size.class_name(),
        if disabled {
            "ui-segmented-control--disabled"
        } else {
            ""
        }
    );
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let options_ref: NodeRef<html::Div> = NodeRef::new();
    let indicator_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_indicator_motion(
        options_ref,
        indicator_ref,
        aria.active_index,
        aria.radio_id,
        motion,
    );

    let on_key_down = {
        let on_key_down = aria.handlers.on_key_down;
        let active_index = aria.active_index;
        let option_refs = option_refs.clone();
        move |ev: ev::KeyboardEvent| {
            if on_key_down.run(ev.key()) {
                ev.prevent_default();
                focus_option(&option_refs, active_index.get_untracked());
            }
        }
    };

    let option_buttons = (0..item_count)
        .map({
            let disabled_indices_set = disabled_indices_set.clone();
            move |index| {
                let option_label = options
                    .get_value()
                    .get(index)
                    .cloned()
                    .and_then(|value| {
                        let trimmed = value.trim();
                        (!trimmed.is_empty()).then(|| trimmed.to_string())
                    })
                    .unwrap_or_else(|| format!("Option {}", index + 1));
                let node_ref = option_refs[index];
                let is_selected = move || aria.selected_index.get() == Some(index);
                let is_disabled = disabled || disabled_indices_set.contains(&index);

                let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
                let hover = use_hover(HoverOptions { is_disabled });

                view! {
                    <button
                        type="button"
                        class="ui-segmented-control__option"
                        class:ui-segmented-control__option--focus-visible=move || focus_ring.is_focus_visible.get()
                        node_ref=node_ref
                        id=aria.radio_id.run(index)
                        role="radio"
                        tabindex=move || if is_disabled { -1 } else if aria.active_index.get() == index { 0 } else { -1 }
                        aria-checked=move || if is_selected() { "true" } else { "false" }
                        aria-disabled=if is_disabled { Some("true") } else { None }
                        disabled=is_disabled
                        data-slot="segmented-control-option"
                        data-index=index
                        data-selected=move || is_selected().then_some("true")
                        data-hovered=move || hover.is_hovered.get().then_some("true")
                        data-disabled=is_disabled.then_some("true")
                        data-focused=move || focus_ring.is_focused.get().then_some("true")
                        data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
                        on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                        on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                        on:focus=move |_| {
                            focus_ring.handlers.on_focus.run(());
                            aria.handlers.on_radio_focus.run(index);
                        }
                        on:blur=move |_| focus_ring.handlers.on_blur.run(())
                        on:click=move |_| aria.handlers.on_radio_click.run(index)
                    >
                        <span class="ui-segmented-control__option-label">{option_label}</span>
                    </button>
                }
            }
        })
        .collect_view();

    view! {
        <div
            class=class
            role=aria.attrs.role
            aria-disabled=aria.attrs.aria_disabled
            aria-label=aria_label
            aria-labelledby=label_id.get_value()
            aria-orientation=orientation.aria_orientation()
            data-slot="segmented-control"
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-count=move || state.get().item_count.to_string()
            data-has-disabled-options=move || state.get().has_disabled_options.then_some("true")
            data-disabled-option-count=move || state.get().disabled_option_count.to_string()
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-selection-empty=move || state.get().selection_empty.then_some("true")
            data-selected-index=move || state.get().selected_index.map(|index| index.to_string())
            data-orientation=orientation.data_orientation()
            data-horizontal=move || state.get().is_horizontal.then_some("true")
            data-vertical=move || state.get().is_vertical.then_some("true")
            data-has-label=move || state.get().has_label.then_some("true")
            on:keydown=on_key_down
        >
            {label.get_value().map(|label| {
                let label_id = label_id.get_value();
                view! {
                    <div
                        class="ui-segmented-control__label"
                        id=label_id
                        data-slot="segmented-control-label"
                    >
                        {label}
                    </div>
                }
            })}
            <div
                class="ui-segmented-control__options"
                data-slot="segmented-control-options"
                node_ref=options_ref
            >
                <div
                    class="ui-segmented-control__indicator"
                    node_ref=indicator_ref
                    aria-hidden="true"
                    data-slot="segmented-control-indicator"
                ></div>
                {option_buttons}
            </div>
        </div>
    }
}
