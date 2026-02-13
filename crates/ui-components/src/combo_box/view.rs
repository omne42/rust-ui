use super::{logic, motion::ComboBoxMotion};
use crate::{overlay_open, presence::use_presence};
use leptos::{ev, html, portal::Portal, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{
    ComboBoxOptions, FocusRingOptions, PopoverPlacement, PopoverPositionOptions, TextFieldOptions,
    use_combo_box, use_focus_ring, use_popover_position, use_text_field,
};

#[component]
fn ComboBoxPanel(
    open: Signal<bool>,
    aria: ui_headless::ComboBoxAria,
    anchor_ref: NodeRef<html::Div>,
    aria_labelledby: String,
    filtered_indices: Memo<Vec<usize>>,
    items: StoredValue<Arc<[String]>>,
    disabled_indices: Arc<HashSet<usize>>,
    selected_index: ReadSignal<Option<usize>>,
    empty_message: StoredValue<String>,
    motion: ComboBoxMotion,
    on_exit_complete: Callback<()>,
) -> impl IntoView {
    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    let position = use_popover_position(PopoverPositionOptions {
        anchor_ref,
        panel_ref,
        placement: PopoverPlacement::BottomStart,
        offset_px: 6.0,
        ..Default::default()
    });

    crate::popover::motion::attach_motion(
        panel_ref,
        open,
        position.placement.into(),
        on_exit_complete,
        motion.popover,
    );

    let options_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    crate::active_highlight::attach_active_highlight_motion(
        options_ref,
        highlight_ref,
        aria.active_index,
        aria.option_id,
        motion.highlight,
    );

    let on_panel_pointer_down = move |ev: ev::PointerEvent| {
        ev.prevent_default();
    };

    let panel_vars = move || {
        format!(
            "--ui-popover-top: {}px; --ui-popover-left: {}px; --ui-popover-anchor-width: {}px;",
            position.top_px.get(),
            position.left_px.get(),
            position.anchor_width_px.get()
        )
    };

    view! {
        <Portal>
            <div
                class="ui-combo-box__panel"
                node_ref=panel_ref
                data-ui-overlay-portal=""
                data-slot="combo-box-panel"
                data-placement=move || position.placement.get().as_str()
                style=panel_vars
                on:pointerdown=on_panel_pointer_down
            >
                <div
                    class="ui-combo-box__listbox"
                    id=aria.listbox.id.clone()
                    role=aria.listbox.role
                    aria-disabled=aria.listbox.aria_disabled
                    aria-labelledby=aria_labelledby.clone()
                    data-slot="combo-box-listbox"
                    data-empty=move || filtered_indices.get().is_empty().then_some("true")
                >
                    <div class="ui-combo-box__options" node_ref=options_ref data-slot="combo-box-options">
                        <div class="ui-active-highlight" node_ref=highlight_ref data-slot="combo-box-highlight"></div>
                        {{
                            let disabled_indices = disabled_indices.clone();
                            let option_id = aria.option_id;
                            let on_option_pointer_move = aria.handlers.on_option_pointer_move;
                            let on_option_click = aria.handlers.on_option_click;
                            let active_index = aria.active_index;

                            move || {
                                let indices = filtered_indices.get();
                                let items = items.get_value();
                                indices
                                    .iter()
                                    .copied()
                                    .enumerate()
                                    .map(|(filtered_index, original_index)| {
                                        let id = option_id.run(filtered_index);
                                        let label = items.get(original_index).cloned().unwrap_or_default();
                                        let is_selected = move || selected_index.get() == Some(original_index);
                                        let is_disabled = disabled_indices.contains(&original_index);

                                        view! {
                                            <div
                                                id=id
                                                role="option"
                                                aria-selected=move || if is_selected() { Some("true") } else { None }
                                                aria-disabled=if is_disabled { Some("true") } else { None }
                                                class="ui-combo-box__option"
                                                data-slot="combo-box-option"
                                                data-selected=move || if is_selected() { Some("true") } else { None }
                                                data-focused=move || (active_index.get() == filtered_index).then_some("true")
                                                data-disabled=if is_disabled { Some("true") } else { None }
                                                on:pointermove=move |_| on_option_pointer_move.run(filtered_index)
                                                on:click=move |_| on_option_click.run(filtered_index)
                                            >
                                                {label}
                                            </div>
                                        }
                                    })
                                    .collect_view()
                            }
                        }}
                        <Show when=move || filtered_indices.get().is_empty()>
                            <div class="ui-combo-box__empty" data-slot="combo-box-empty">
                                {move || empty_message.get_value()}
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </Portal>
    }
}

#[component]
pub fn ComboBox(
    id_base: String,
    label: String,
    items: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_message: Option<String>,
    #[prop(optional, into)] toggle_button_aria_label: Option<String>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] motion: ComboBoxMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let has_custom_id_base = logic::normalize_optional_text(Some(id_base.clone())).is_some();
    let id_base = logic::normalize_id_base(id_base);
    let id_base = StoredValue::new(id_base);

    let items: StoredValue<Arc<[String]>> = StoredValue::new(items.into());
    let item_count = items.get_value().len();

    let disabled_indices = logic::normalize_disabled_indices(disabled_indices, item_count);
    let disabled_option_count = disabled_indices.len();
    let disabled_indices: Arc<HashSet<usize>> = Arc::new(disabled_indices.into_iter().collect());

    let has_custom_label = !label.trim().is_empty();
    let label = StoredValue::new(logic::normalize_label(label));

    let has_custom_placeholder = logic::normalize_optional_text(placeholder.clone()).is_some();
    let placeholder = StoredValue::new(logic::resolve_placeholder(placeholder));
    let empty_message = StoredValue::new(logic::resolve_empty_message(empty_message));
    let toggle_button_aria_label =
        StoredValue::new(logic::resolve_toggle_aria_label(toggle_button_aria_label));

    let description = logic::normalize_optional_text(description);
    let error = logic::normalize_optional_text(error);
    let has_custom_description = description.is_some();
    let has_custom_error = error.is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let motion = crate::combo_box::motion::sanitize_motion(motion);
    let has_custom_motion = motion != ComboBoxMotion::default();
    let is_controlled = open.is_some();

    let state = logic::resolve_state(logic::ComboBoxStateInput {
        item_count,
        disabled_option_count,
        is_disabled: disabled,
        has_custom_label,
        has_custom_description,
        has_custom_error,
        has_custom_placeholder,
        has_custom_id_base,
        has_custom_class_name,
        has_custom_motion,
        is_controlled,
    });

    let class = logic::compose_class_name(class_name, state);

    let open_state = overlay_open::use_controllable_open_state(open, default_open, on_open_change);
    let is_open = open_state.open;
    let set_open = open_state.request_open_change;
    let presence = use_presence(is_open);

    let (has_typed, set_has_typed) = signal(false);
    let (query, set_query) = signal(String::new());

    let selected_label = Memo::new(move |_| {
        let items = items.get_value();
        selected_index
            .get()
            .and_then(|index| items.get(index).cloned())
    });

    Effect::new(move |_| {
        if is_open.get() {
            return;
        }
        set_has_typed.set(false);
        let value = selected_label.get().unwrap_or_default();
        set_query.set(value);
    });

    let filtered_indices = Memo::new(move |_| {
        let items = items.get_value();
        if !has_typed.get() {
            return (0..items.len()).collect::<Vec<_>>();
        }
        let q = query.get();
        let q = q.trim().to_ascii_lowercase();
        if q.is_empty() {
            return (0..items.len()).collect::<Vec<_>>();
        }
        items
            .iter()
            .enumerate()
            .filter_map(|(idx, label)| label.to_ascii_lowercase().contains(&q).then_some(idx))
            .collect()
    });

    let (filtered_count, set_filtered_count) = signal(0_usize);
    Effect::new(move |_| {
        let next = filtered_indices.get().len();
        set_filtered_count.set(next);
    });

    let selected_filtered_index = Memo::new(move |_| {
        let selected = selected_index.get()?;
        let indices = filtered_indices.get();
        indices.iter().position(|&idx| idx == selected)
    });

    let is_item_disabled = state.has_disabled_options.then_some({
        let disabled_indices = disabled_indices.clone();
        Callback::new(move |filtered_index: usize| {
            let indices = filtered_indices.get_untracked();
            let Some(original) = indices.get(filtered_index).copied() else {
                return false;
            };
            disabled_indices.contains(&original)
        })
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: state.is_disabled,
    });

    let input_id = format!("{}-input", id_base.get_value());
    let label_id = StoredValue::new(format!("{}-label", id_base.get_value()));

    let text_field = use_text_field(TextFieldOptions {
        id: input_id,
        has_description: state.has_description,
        has_error: state.has_error,
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let on_action = Callback::new(move |filtered_index: usize| {
        let indices = filtered_indices.get_untracked();
        let Some(original_index) = indices.get(filtered_index).copied() else {
            return;
        };
        set_selected_index.set(Some(original_index));

        let items = items.get_value();
        let label = items.get(original_index).cloned().unwrap_or_default();
        set_query.set(label);
        set_has_typed.set(false);
    });

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: state.is_disabled,
        id_base: id_base.get_value(),
        is_open,
        set_open,
        item_count: filtered_count,
        selected_index: selected_filtered_index.into(),
        on_action: Some(on_action),
        is_item_disabled,
    });

    let aria_controls = aria.input.aria_controls.clone();

    let on_key_down = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        let was_open = is_open.get_untracked();
        if aria.handlers.on_input_key_down.run(key.clone()) {
            ev.prevent_default();
            if key == "Escape" && was_open {
                ev.stop_propagation();
            }
        }
    };

    let on_focus = move |_| focus_ring.handlers.on_focus.run(());
    let on_blur = move |_| {
        focus_ring.handlers.on_blur.run(());
        aria.handlers.close.run(());
        set_has_typed.set(false);
    };

    let on_input = move |ev| {
        if state.is_disabled {
            return;
        }
        set_has_typed.set(true);
        set_query.set(event_target_value(&ev));
        aria.handlers.open.run(());
    };

    let on_trigger_click = move |_| {
        if state.is_disabled {
            return;
        }
        set_has_typed.set(false);
        aria.handlers.toggle.run(());
    };

    let on_trigger_pointer_down = move |ev: ev::PointerEvent| {
        ev.prevent_default();
    };

    let control_ref: NodeRef<html::Div> = NodeRef::new();

    view! {
        <div
            class=class
            class:ui-combo-box--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-combo-box--invalid=move || invalid.get()
            class:ui-combo-box--disabled=state.is_disabled
            data-slot="combo-box"
            data-state=move || {
                if is_open.get() {
                    "open"
                } else if state.is_disabled {
                    "disabled"
                } else {
                    "closed"
                }
            }
            data-open=move || is_open.get().then_some("true")
            data-closed=move || (!is_open.get()).then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-enabled=state.is_enabled.then_some("true")
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-invalid=move || invalid.get().then_some("true")
            data-valid=move || (!invalid.get()).then_some("true")
            data-required=move || required.get().then_some("true")
            data-optional=move || (!required.get()).then_some("true")
            data-empty=move || (filtered_count.get() == 0).then_some("true")
            data-has-items=state.has_items.then_some("true")
            data-has-filtered-items=move || (filtered_count.get() > 0).then_some("true")
            data-selection-empty=move || selected_index.get().is_none().then_some("true")
            data-has-selection=move || selected_index.get().is_some().then_some("true")
            data-has-description=state.has_description.then_some("true")
            data-has-error=state.has_error.then_some("true")
            data-has-disabled-options=state.has_disabled_options.then_some("true")
            data-controlled=state.is_controlled.then_some("true")
            data-uncontrolled=state.is_uncontrolled.then_some("true")
            data-label-source=state.label_source_attr
            data-description-source=state.description_source_attr
            data-error-source=state.error_source_attr
            data-placeholder-source=state.placeholder_source_attr
            data-id-source=state.id_source_attr
            data-class-source=state.class_source_attr
            data-motion-source=state.motion_source_attr
            data-custom-label=state.has_custom_label.then_some("true")
            data-custom-description=state.has_custom_description.then_some("true")
            data-custom-error=state.has_custom_error.then_some("true")
            data-custom-placeholder=state.has_custom_placeholder.then_some("true")
            data-custom-id=state.has_custom_id_base.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-custom-motion=state.has_custom_motion.then_some("true")
            data-typed=move || has_typed.get().then_some("true")
            data-count=state.item_count.to_string()
            data-filtered-count=move || filtered_count.get().to_string()
            data-disabled-option-count=state.disabled_option_count.to_string()
        >
            <label
                class="ui-combo-box__label"
                id=label_id.get_value()
                for=text_field.label.for_attr.clone()
                data-slot="combo-box-label"
            >
                {label.get_value()}
            </label>

            <div class="ui-combo-box__field" data-slot="combo-box-field">
                <div
                    class="ui-combo-box__control"
                    node_ref=control_ref
                    data-slot="combo-box-control"
                >
                    <input
                        class="ui-combo-box__input"
                        data-slot="combo-box-input"
                        id=aria.input.id.clone()
                        prop:value=move || query.get()
                        placeholder=move || {
                            (selected_label.get().is_none() && !has_typed.get())
                                .then(|| placeholder.get_value())
                        }
                        disabled=state.is_disabled
                        required=move || required.get()
                        role=aria.input.role
                        aria-autocomplete=aria.input.aria_autocomplete
                        aria-controls=move || is_open.get().then(|| aria_controls.clone())
                        aria-expanded=move || aria.input.aria_expanded.get()
                        aria-activedescendant=move || aria.input.aria_activedescendant.get()
                        aria-describedby=move || text_field.input.aria_describedby.get()
                        aria-invalid=move || text_field.input.aria_invalid.get()
                        aria-required=move || text_field.input.aria_required.get()
                        aria-disabled=aria.input.aria_disabled
                        on:input=on_input
                        on:keydown=on_key_down
                        on:focus=on_focus
                        on:blur=on_blur
                    />

                    <button
                        class="ui-combo-box__trigger"
                        type="button"
                        aria-label=move || toggle_button_aria_label.get_value()
                        data-slot="combo-box-trigger"
                        disabled=state.is_disabled
                        tabindex="-1"
                        on:pointerdown=on_trigger_pointer_down
                        on:click=on_trigger_click
                    >
                        "▾"
                    </button>
                </div>

                <Show when=move || presence.is_present.get()>
                    <ComboBoxPanel
                        open=is_open
                        aria=aria.clone()
                        anchor_ref=control_ref
                        aria_labelledby=label_id.get_value()
                        filtered_indices=filtered_indices
                        items=items
                        disabled_indices=disabled_indices.clone()
                        selected_index=selected_index
                        empty_message=empty_message
                        motion=motion
                        on_exit_complete=presence.finish_exit
                    />
                </Show>
            </div>

            {description.map(|description| {
                let description_id = text_field.description.id.clone();
                view! {
                    <div
                        class="ui-combo-box__description"
                        id=description_id
                        data-slot="combo-box-description"
                    >
                        {description}
                    </div>
                }
            })}

            {error.map(|error| {
                let error_id = text_field.error.id.clone();
                let error_id = StoredValue::new(error_id);
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || invalid.get()>
                        <div
                            class="ui-combo-box__error"
                            id=move || error_id.get_value()
                            data-slot="combo-box-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
