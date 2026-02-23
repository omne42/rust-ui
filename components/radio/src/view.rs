use crate::{RadioGroupOrientation, RadioMotion, logic, motion};
use leptos::{ev, html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{
    A11yDirection, FocusRingOptions, HoverOptions, PressOptions, RadioGroupOptions, RadioOptions,
    use_controllable_state, use_focus_ring, use_hover, use_press, use_radio,
};

#[cfg(target_arch = "wasm32")]
fn focus_radio(radio_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize) {
    let Some(node_ref) = radio_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    ui_observability::observe_js_result!(el.focus());
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_radio(_radio_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}

fn radio_indicator_view() -> impl IntoView {
    view! {
        <span class="ui-radio__indicator" aria-hidden="true">
            <span class="ui-radio__dot"></span>
        </span>
    }
}

#[component]
pub fn RadioGroup(
    id_base: String,
    options: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] orientation: RadioGroupOrientation,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: RadioMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let options: StoredValue<Arc<[String]>> = StoredValue::new(options.into());
    let item_count = options.get_value().len();
    let (item_count_signal, _set_item_count) = signal(item_count);
    let disabled_state = logic::normalize_disabled_prop(logic::DisabledPropInput {
        is_disabled,
        disabled,
    });
    let is_disabled = disabled_state.is_disabled;
    let disabled_source_attr = disabled_state.disabled_source_attr;

    let disabled_index_set: HashSet<usize> = disabled_indices.into_iter().collect();
    let has_disabled = !disabled_index_set.is_empty();
    let disabled_indices_set: Arc<HashSet<usize>> = Arc::new(disabled_index_set);

    let is_item_disabled = has_disabled.then_some({
        let disabled_indices = disabled_indices_set.clone();
        Callback::new(move |index: usize| disabled_indices.contains(&index))
    });

    let aria = use_radio(RadioOptions {
        group: RadioGroupOptions {
            is_disabled,
            id_base: id_base.clone(),
            orientation: logic::roving_orientation(orientation),
            item_count: item_count_signal,
            selected_index,
            set_selected_index,
            on_change: None,
            is_item_disabled,
        },
        lang,
        dir,
    });

    let radio_refs: Arc<Vec<NodeRef<html::Button>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let label = logic::normalize_optional_text(label);
    let label_id = label.as_ref().map(|_| format!("{id_base}-label"));
    let has_label = label.is_some();

    let accessible_name =
        logic::resolve_accessible_name(aria_label, aria_labelledby, label_id.clone());
    let aria_label = StoredValue::new(accessible_name.aria_label);
    let aria_labelledby = StoredValue::new(accessible_name.aria_labelledby);

    let disabled_indices_for_state = disabled_indices_set.clone();

    let state = Memo::new(move |_| {
        logic::resolve_state(
            item_count,
            is_disabled,
            disabled_indices_for_state.as_ref(),
            aria.state.selected_index.get(),
            orientation,
            has_label,
        )
    });

    let label_id = StoredValue::new(label_id);
    let label = StoredValue::new(label);

    let base_class = format!("ui-radio-group {}", orientation.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let motion_source = if motion == RadioMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != RadioMotion::default()).then_some("true");

    let radios = (0..item_count)
        .map({
            let aria = aria.clone();
            let radio_refs = radio_refs.clone();
            let disabled_indices_set = disabled_indices_set.clone();
            move |index| {
                let label = options
                    .get_value()
                    .get(index)
                    .cloned()
                    .and_then(|label| {
                        let trimmed = label.trim();
                        (!trimmed.is_empty()).then(|| trimmed.into())
                    })
                    .unwrap_or_else(|| format!("Option {}", index + 1));
                let node_ref = radio_refs[index];
                let is_checked = move || aria.state.selected_index.get() == Some(index);
                let is_disabled = is_disabled || disabled_indices_set.contains(&index);

                let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
                let hover = use_hover(HoverOptions { is_disabled });
                let press = use_press(PressOptions {
                    is_disabled,
                    on_press: None,
                    ..Default::default()
                });

                motion::attach_motion(
                    node_ref,
                    hover.is_hovered,
                    press.is_pressed,
                    is_disabled,
                    motion,
                );

                let on_key_down = {
                    let on_key_down = aria.handlers.on_key_down;
                    let active_index = aria.state.active_index;
                    let radio_refs = radio_refs.clone();
                    move |ev: ev::KeyboardEvent| {
                        if on_key_down.run(ev.key()) {
                            ev.prevent_default();
                            focus_radio(&radio_refs, active_index.get_untracked());
                        }
                    }
                };

                view! {
                    <button
                        type="button"
                        class="ui-radio"
                        class:ui-radio--focus-visible=move || focus_ring.is_focus_visible.get()
                        node_ref=node_ref
                        id=aria.state.radio_id.run(index)
                        role="radio"
                        tabindex=move || if is_disabled { -1 } else if aria.state.active_index.get() == index { 0 } else { -1 }
                        aria-checked=move || if is_checked() { "true" } else { "false" }
                        aria-disabled=if is_disabled { Some("true") } else { None }
                        disabled=is_disabled
                        data-slot="radio"
                        data-index=index
                        data-checked=move || is_checked().then_some("true")
                        data-disabled=is_disabled.then_some("true")
                        data-active=move || (aria.state.active_index.get() == index).then_some("true")
                        data-hovered=move || hover.is_hovered.get().then_some("true")
                        data-pressed=move || press.is_pressed.get().then_some("true")
                        data-focused=move || focus_ring.is_focused.get().then_some("true")
                        data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
                        data-motion-source=motion_source
                        data-custom-motion=custom_motion
                        on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                        on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                        on:pointerdown=move |_| press.handlers.on_pointer_down.run(())
                        on:pointerup=move |_| press.handlers.on_pointer_up.run(())
                        on:pointercancel=move |_| press.handlers.on_pointer_cancel.run(())
                        on:focus=move |_| {
                            focus_ring.handlers.on_focus.run(());
                            aria.handlers.on_radio_focus.run(index);
                        }
                        on:blur=move |_| {
                            press.handlers.on_blur.run(());
                            focus_ring.handlers.on_blur.run(());
                        }
                        on:click=move |_| aria.handlers.on_radio_click.run(index)
                        on:keydown=on_key_down
                    >
                        {radio_indicator_view()}
                        <span class="ui-radio__label">{label}</span>
                    </button>
                }
            }
        })
        .collect_view();

    view! {
        <div
            class=class
            role=aria.attrs.role
            aria-label=aria_label.get_value()
            aria-labelledby=aria_labelledby.get_value()
            aria-disabled=aria.attrs.aria_disabled
            lang=aria.attrs.lang
            dir=aria.attrs.dir
            aria-orientation=orientation.aria_orientation()
            data-slot="radio-group"
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-disabled-source=disabled_source_attr
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
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            data-stream-mode="snapshot"
            data-stream-fallback="snapshot"
            data-output-status="verified"
            data-ui-schema="ui.radio-group"
            data-ui-schema-version="1"
            data-ui-intent="single-selection"
            data-ui-action="select-option"
            data-ui-state=move || if state.get().has_selection { "has-selection" } else { "empty-selection" }
            data-ui-source="external-selected-index"
        >
            {label.get_value().map(|label| {
                let label_id = label_id.get_value();
                view! {
                    <div
                        class="ui-radio-group__label"
                        id=label_id
                        data-slot="radio-group-label"
                    >
                        {label}
                    </div>
                }
            })}
            {radios}
        </div>
    }
}

#[component]
pub fn Radio(
    id: String,
    label: String,
    #[prop(optional, into)] is_checked: Option<Signal<bool>>,
    #[prop(optional, into)] checked: Option<Signal<bool>>,
    #[prop(optional)] default_checked: Option<bool>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: RadioMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(optional)] on_change: Option<Callback<bool>>,
) -> impl IntoView {
    let checked_axis = logic::normalize_checked_axis(logic::CheckedAxisInput {
        is_checked,
        checked,
        default_checked,
        on_checked_change,
        on_change,
    });
    let controllable_checked = use_controllable_state(
        checked_axis.controlled_checked,
        Some(checked_axis.default_checked),
        checked_axis.on_checked_change,
    );
    let checked = controllable_checked.value;
    let request_checked_change = controllable_checked.request_change;
    let checked_source_attr = checked_axis.checked_source_attr;
    let default_checked_source_attr = checked_axis.default_checked_source_attr;
    let checked_change_source_attr = checked_axis.checked_change_source_attr;
    let checked_control_mode_attr = checked_axis.control_mode_attr;
    let is_checked_controlled = checked_axis.is_controlled;
    debug_assert_eq!(
        checked_control_mode_attr,
        checked_axis.control_mode.data_attr()
    );
    let disabled_state = logic::normalize_disabled_prop(logic::DisabledPropInput {
        is_disabled,
        disabled,
    });
    let is_disabled = disabled_state.is_disabled;
    let disabled_source_attr = disabled_state.disabled_source_attr;
    let node_ref: NodeRef<html::Button> = NodeRef::new();
    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
    let hover = use_hover(HoverOptions { is_disabled });
    let press = use_press(PressOptions {
        is_disabled,
        on_press: None,
        ..Default::default()
    });

    motion::attach_motion(
        node_ref,
        hover.is_hovered,
        press.is_pressed,
        is_disabled,
        motion,
    );

    let base_class = "ui-radio".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let motion_source = if motion == RadioMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != RadioMotion::default()).then_some("true");

    let on_click = move |_| {
        if is_disabled {
            return;
        }
        request_checked_change.run(!checked.get_untracked());
    };

    view! {
        <button
            id=id
            type="button"
            class=class
            class:ui-radio--focus-visible=move || focus_ring.is_focus_visible.get()
            node_ref=node_ref
            role="radio"
            tabindex=if is_disabled { -1 } else { 0 }
            aria-checked=move || if checked.get() { "true" } else { "false" }
            aria-disabled=if is_disabled { Some("true") } else { None }
            disabled=is_disabled
            data-slot="radio"
            data-checked=move || checked.get().then_some("true")
            data-checked-control-mode=checked_control_mode_attr
            data-checked-controlled=is_checked_controlled.then_some("true")
            data-checked-uncontrolled=(!is_checked_controlled).then_some("true")
            data-checked-source=checked_source_attr
            data-default-checked-source=default_checked_source_attr
            data-checked-change-source=checked_change_source_attr
            data-disabled=is_disabled.then_some("true")
            data-disabled-source=disabled_source_attr
            data-hovered=move || hover.is_hovered.get().then_some("true")
            data-pressed=move || press.is_pressed.get().then_some("true")
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            data-stream-mode="snapshot"
            data-stream-fallback="snapshot"
            data-output-status="verified"
            data-ui-schema="ui.radio"
            data-ui-schema-version="1"
            data-ui-intent="single-option"
            data-ui-action="toggle-request"
            data-ui-state=move || if checked.get() { "checked" } else { "unchecked" }
            data-ui-source=checked_source_attr
            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
            on:pointerdown=move |_| press.handlers.on_pointer_down.run(())
            on:pointerup=move |_| press.handlers.on_pointer_up.run(())
            on:pointercancel=move |_| press.handlers.on_pointer_cancel.run(())
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| {
                press.handlers.on_blur.run(());
                focus_ring.handlers.on_blur.run(());
            }
            on:click=on_click
        >
            {radio_indicator_view()}
            <span class="ui-radio__label">{label}</span>
        </button>
    }
}
