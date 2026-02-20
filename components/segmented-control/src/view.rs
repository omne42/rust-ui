use crate::logic::{
    SegmentedControlControlMode, SegmentedControlSelectionOrigin, SegmentedControlSelectionSource,
    segmented_control_agent_contract,
};
use crate::{SegmentedControlMotion, SegmentedControlOrientation, SegmentedControlSize, motion};
use leptos::{ev, html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{
    A11yDirection, FocusRingOptions, HoverOptions, RadioGroupHandlers, RadioGroupOptions,
    RadioOptions, RadioState, RovingOrientation, use_focus_ring, use_hover, use_radio,
};
use ui_state_primitives::segmented_control::{SegmentedControlStateInput, resolve_state};

const DEFAULT_ARIA_LABEL: &str = "Segmented control";
const ROOT_CLASS: &str = "ui-segmented-control";
const ROOT_DISABLED_CLASS: &str = "ui-segmented-control--disabled";
const LABEL_CLASS: &str = "ui-segmented-control__label";
const OPTIONS_CLASS: &str = "ui-segmented-control__options";
const OPTION_CLASS: &str = "ui-segmented-control__option";
const OPTION_LABEL_CLASS: &str = "ui-segmented-control__option-label";
const INDICATOR_CLASS: &str = "ui-segmented-control__indicator";
const SLOT_ROOT: &str = "segmented-control";
const SLOT_LABEL: &str = "segmented-control-label";
const SLOT_OPTIONS: &str = "segmented-control-options";
const SLOT_OPTION: &str = "segmented-control-option";
const SLOT_INDICATOR: &str = "segmented-control-indicator";

#[cfg(target_arch = "wasm32")]
fn focus_option(option_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize) {
    let Some(node_ref) = option_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    drop(el.focus());
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_option(_option_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}

fn option_label_for_index(options: &[String], index: usize) -> String {
    options
        .get(index)
        .cloned()
        .and_then(|value| {
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| trimmed.into())
        })
        .unwrap_or_else(|| format!("Option {}", index + 1))
}

fn render_option_button(
    index: usize,
    option_label: String,
    node_ref: NodeRef<html::Button>,
    radio_state: RadioState,
    radio_handlers: RadioGroupHandlers,
    is_disabled: bool,
    set_selection_origin: WriteSignal<SegmentedControlSelectionOrigin>,
) -> impl IntoView {
    let is_selected = move || radio_state.selected_index.get() == Some(index);
    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
    let hover = use_hover(HoverOptions { is_disabled });

    view! {
        <button
            type="button"
            class=OPTION_CLASS
            class:ui-segmented-control__option--focus-visible=move || focus_ring.is_focus_visible.get()
            node_ref=node_ref
            id=radio_state.radio_id.run(index)
            role="radio"
            tabindex=move || if is_disabled { -1 } else if radio_state.active_index.get() == index { 0 } else { -1 }
            aria-checked=move || if is_selected() { "true" } else { "false" }
            aria-disabled=if is_disabled { Some("true") } else { None }
            disabled=is_disabled
            data-slot=SLOT_OPTION
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
                radio_handlers.on_radio_focus.run(index);
            }
            on:blur=move |_| focus_ring.handlers.on_blur.run(())
            on:click=move |_| {
                set_selection_origin.set(SegmentedControlSelectionOrigin::Pointer);
                radio_handlers.on_radio_click.run(index);
            }
        >
            <span class=OPTION_LABEL_CLASS>{option_label}</span>
        </button>
    }
}

fn render_label(label_id: String, label: String) -> impl IntoView {
    view! {
        <div
            class=LABEL_CLASS
            id=label_id
            data-slot=SLOT_LABEL
        >
            {label}
        </div>
    }
}

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
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = crate::motion::sanitize_motion(motion);
    let options: Arc<[String]> = options.into();
    let item_count = options.len();
    let (item_count_signal, _set_item_count) = signal(item_count);

    let disabled_index_set: HashSet<usize> = disabled_indices.into_iter().collect();
    let has_disabled = !disabled_index_set.is_empty();
    let disabled_indices_set: Arc<HashSet<usize>> = Arc::new(disabled_index_set);

    let is_item_disabled = has_disabled.then_some({
        let disabled_indices_set = disabled_indices_set.clone();
        Callback::new(move |index: usize| disabled_indices_set.contains(&index))
    });

    let aria = use_radio(RadioOptions {
        group: RadioGroupOptions {
            is_disabled: disabled,
            id_base: id_base.clone(),
            orientation: if orientation.is_vertical() {
                RovingOrientation::Vertical
            } else {
                RovingOrientation::Horizontal
            },
            item_count: item_count_signal,
            selected_index,
            set_selected_index,
            on_change: None,
            is_item_disabled,
        },
        lang,
        dir,
    });

    let option_refs: Arc<Vec<NodeRef<html::Button>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .or_else(|| label.clone())
        .unwrap_or_else(|| DEFAULT_ARIA_LABEL.into());

    let label = label.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    });
    let agent_contract = segmented_control_agent_contract();
    let has_label = label.is_some();
    let label_id = label.as_ref().map(|_| format!("{id_base}-label"));
    let label_view = label
        .clone()
        .zip(label_id.clone())
        .map(|(label, label_id)| render_label(label_id, label))
        .into_view();

    let disabled_indices_for_state = disabled_indices_set.clone();
    let state = Memo::new(move |_| {
        resolve_state(SegmentedControlStateInput {
            item_count,
            is_disabled: disabled,
            disabled_indices: disabled_indices_for_state.as_ref(),
            selected_index: aria.state.selected_index.get(),
            is_vertical: orientation.is_vertical(),
            has_label,
        })
    });
    let (selection_origin, set_selection_origin) =
        signal(SegmentedControlSelectionOrigin::Programmatic);

    let base_class = format!(
        "{ROOT_CLASS} {} {} {}",
        orientation.class_name(),
        size.class_name(),
        if disabled { ROOT_DISABLED_CLASS } else { "" }
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
        aria.state.active_index,
        aria.state.radio_id,
        motion,
    );

    let on_key_down = {
        let on_key_down = aria.handlers.on_key_down;
        let active_index = aria.state.active_index;
        let option_refs = option_refs.clone();
        move |ev: ev::KeyboardEvent| {
            if on_key_down.run(ev.key()) {
                set_selection_origin.set(SegmentedControlSelectionOrigin::Keyboard);
                ev.prevent_default();
                focus_option(&option_refs, active_index.get_untracked());
            }
        }
    };

    let option_buttons = (0..item_count)
        .map({
            let options = options.clone();
            let option_refs = option_refs.clone();
            let disabled_indices_set = disabled_indices_set.clone();
            let radio_state = aria.state.clone();
            let radio_handlers = aria.handlers.clone();
            move |index| {
                let option_label = option_label_for_index(options.as_ref(), index);
                let node_ref = option_refs[index];
                let is_disabled = disabled || disabled_indices_set.contains(&index);
                render_option_button(
                    index,
                    option_label,
                    node_ref,
                    radio_state.clone(),
                    radio_handlers.clone(),
                    is_disabled,
                    set_selection_origin,
                )
            }
        })
        .collect_view();

    view! {
        <div
            class=class
            role=aria.attrs.role
            aria-disabled=aria.attrs.aria_disabled
            aria-label=aria_label
            aria-labelledby=label_id
            aria-orientation=orientation.aria_orientation()
            lang=aria.attrs.lang
            dir=aria.attrs.dir
            data-slot=SLOT_ROOT
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action-model=agent_contract.action_model_attr
            data-ui-state-axis=agent_contract.state_axis_attr
            data-ui-source-axis=agent_contract.source_axis_attr
            data-control-mode=SegmentedControlControlMode::Controlled.as_attr()
            data-selection-source=move || {
                SegmentedControlSelectionSource::from_indices(
                    aria.state.selected_index.get(),
                    state.get().selected_index,
                )
                .as_attr()
            }
            data-selection-origin=move || selection_origin.get().as_attr()
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
            {label_view}
            <div
                class=OPTIONS_CLASS
                data-slot=SLOT_OPTIONS
                node_ref=options_ref
            >
                <div
                    class=INDICATOR_CLASS
                    node_ref=indicator_ref
                    aria-hidden="true"
                    data-slot=SLOT_INDICATOR
                ></div>
                {option_buttons}
            </div>
        </div>
    }
}
