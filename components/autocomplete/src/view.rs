use crate::{AutocompleteMotion, logic};
use leptos::{ev, html, portal::Portal, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless as overlay_open;
use ui_headless::use_presence;
use ui_headless::{
    A11yDirection, ComboBoxOptions, CommonStrings, FocusRingOptions, PopoverPlacement,
    PopoverPositionOptions, TextFieldOptions, use_combo_box, use_focus_ring, use_popover_position,
    use_text_field, use_ui_i18n, use_ui_id_provider,
};
use ui_visual_primitive::active_highlight::ActiveHighlightMotion;

#[derive(Clone)]
struct AutocompleteOptionViewCtx {
    id: String,
    label: String,
    option_attrs: Memo<ui_headless::ComboBoxOptionAttrs>,
    on_option_pointer_move: Callback<usize>,
    on_option_click: Callback<usize>,
    filtered_index: usize,
}

fn render_autocomplete_option(ctx: AutocompleteOptionViewCtx) -> impl IntoView {
    let AutocompleteOptionViewCtx {
        id,
        label,
        option_attrs,
        on_option_pointer_move,
        on_option_click,
        filtered_index,
    } = ctx;

    view! {
        <div
            id=id
            role=move || option_attrs.get().role
            aria-selected=move || option_attrs.get().aria_selected
            aria-disabled=move || option_attrs.get().aria_disabled
            class="ui-autocomplete__option"
            data-slot="autocomplete-option"
            data-selected=move || option_attrs.get().data_selected
            data-focused=move || option_attrs.get().data_focused
            data-disabled=move || option_attrs.get().data_disabled
            on:pointermove=move |_| on_option_pointer_move.run(filtered_index)
            on:click=move |_| on_option_click.run(filtered_index)
        >
            {label}
        </div>
    }
}

fn render_autocomplete_description(
    description: Option<String>,
    description_id: String,
) -> Option<AnyView> {
    description.map(|description| {
        view! {
            <div
                class="ui-autocomplete__description"
                id=description_id
                data-slot="autocomplete-description"
            >
                {description}
            </div>
        }
        .into_any()
    })
}

fn render_autocomplete_error(
    error: Option<String>,
    error_id: String,
    invalid: Signal<bool>,
) -> Option<AnyView> {
    error.map(|error| {
        let error_id = StoredValue::new(error_id);
        let error = StoredValue::new(error);
        view! {
            <Show when=move || invalid.get()>
                <div class="ui-autocomplete__error" id=move || error_id.get_value() data-slot="autocomplete-error">
                    {move || error.get_value()}
                </div>
            </Show>
        }
        .into_any()
    })
}

#[component]
fn AutocompletePanel(
    open: Signal<bool>,
    aria: ui_headless::ComboBoxAria,
    anchor_ref: NodeRef<html::Div>,
    aria_labelledby: String,
    filtered_indices: Memo<Vec<usize>>,
    items: StoredValue<Arc<[String]>>,
    empty_message: StoredValue<String>,
    motion: ActiveHighlightMotion,
    popover_motion: crate::motion::PopoverMotion,
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
    crate::motion::attach_popover_motion(
        panel_ref,
        open,
        position.placement.into(),
        on_exit_complete,
        popover_motion,
    );

    let options_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    ui_visual_primitive::active_highlight::attach_active_highlight_motion(
        options_ref,
        highlight_ref,
        aria.active_index,
        aria.option_id,
        motion,
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
                class="ui-autocomplete__panel"
                node_ref=panel_ref
                data-ui-overlay-portal=""
                data-slot="autocomplete-panel"
                data-placement=move || position.placement.get().as_str()
                style=panel_vars
                on:pointerdown=on_panel_pointer_down
            >
                <div
                    class="ui-autocomplete__listbox"
                    id=aria.listbox.id.clone()
                    role=aria.listbox.role
                    aria-disabled=aria.listbox.aria_disabled
                    aria-labelledby=aria_labelledby.clone()
                    lang=aria.listbox.lang.clone()
                    dir=aria.listbox.dir
                    data-slot="autocomplete-listbox"
                    data-empty=move || filtered_indices.get().is_empty().then_some("true")
                >
                    <div class="ui-autocomplete__options" node_ref=options_ref data-slot="autocomplete-options">
                        <div class="ui-active-highlight" node_ref=highlight_ref data-slot="autocomplete-highlight"></div>
                        {{
                            let option_id = aria.option_id;
                            let option_attrs = aria.option_attrs;
                            let on_option_pointer_move = aria.handlers.on_option_pointer_move;
                            let on_option_click = aria.handlers.on_option_click;

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
                                        let option_attrs =
                                            Memo::new(move |_| option_attrs.run(filtered_index));

                                        render_autocomplete_option(AutocompleteOptionViewCtx {
                                            id,
                                            label,
                                            option_attrs,
                                            on_option_pointer_move,
                                            on_option_click,
                                            filtered_index,
                                        })
                                    })
                                    .collect_view()
                            }
                        }}
                        <Show when=move || filtered_indices.get().is_empty()>
                            <div class="ui-autocomplete__empty" data-slot="autocomplete-empty">
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
pub fn Autocomplete(
    id_base: String,
    label: String,
    items: Vec<String>,
    #[prop(optional, into)] selected_index: Option<Signal<Option<usize>>>,
    #[prop(optional)] default_selected_index: Option<usize>,
    #[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>,
    #[prop(optional)] set_selected_index: Option<WriteSignal<Option<usize>>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional, into)] is_required: Option<Signal<bool>>,
    #[prop(optional, into)] required: Option<Signal<bool>>,
    #[prop(optional, into)] is_invalid: Option<Signal<bool>>,
    #[prop(optional, into)] invalid: Option<Signal<bool>>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_message: Option<String>,
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: AutocompleteMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let accessibility_state =
        logic::normalize_accessibility_state(logic::AccessibilityStateInput {
            is_disabled,
            disabled,
            is_required,
            required,
            is_invalid,
            invalid,
        });
    let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {
        is_open,
        open,
        default_open,
        on_open_change,
    });
    let is_disabled = accessibility_state.is_disabled;
    let required = accessibility_state.required;
    let invalid = accessibility_state.invalid;
    let open = normalized_open_state.open;
    let default_open = normalized_open_state.default_open;
    let on_open_change = normalized_open_state.on_open_change;

    let items: StoredValue<Arc<[String]>> = StoredValue::new(items.into());
    let item_count = items.get_value().len();
    let selection_change = logic::normalize_selection_change(logic::SelectionChangeInput {
        selected_index,
        default_selected_index,
        on_selected_index_change,
        set_selected_index,
        item_count,
    });
    let selected_source_attr = selection_change.selected_source.as_attr();
    let selected_change_source_attr = selection_change.change_source.as_attr();
    let is_selected_controlled = selection_change.is_controlled;
    let selected_state = overlay_open::use_controllable_state(
        selection_change.selected_index,
        Some(selection_change.default_selected_index),
        selection_change.on_selected_index_change,
    );
    let selected_index = selected_state.value;
    let request_selected_index_change = selected_state.request_change;
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let has_custom_id_base = logic::normalize_optional_text(Some(id_base.clone())).is_some();
    let generated_id_base = use_ui_id_provider()
        .map(|id_provider| {
            id_provider.next_prefixed_id(ui_state_primitives::autocomplete::DEFAULT_ID_BASE)
        })
        .unwrap_or_else(|| ui_state_primitives::autocomplete::DEFAULT_ID_BASE.to_string());
    let id_base = logic::resolve_id_base(id_base, generated_id_base);

    let motion = crate::motion::sanitize_motion(motion);
    let has_custom_motion = motion != AutocompleteMotion::default();
    let root_state = logic::normalize_root_state(logic::RootStateInput {
        id_base,
        has_custom_id_base,
        label,
        placeholder,
        empty_message,
        i18n_empty_message: Some(common.autocomplete_empty_message.to_string()),
        description,
        error,
        class_name,
        item_count,
        disabled_indices,
        is_disabled,
        is_controlled: normalized_open_state.is_controlled,
        has_custom_motion,
    });
    let id_base = StoredValue::new(root_state.id_base);
    let label = StoredValue::new(root_state.label);
    let placeholder = StoredValue::new(root_state.placeholder);
    let empty_message = StoredValue::new(root_state.empty_message);
    let description = root_state.description;
    let error = root_state.error;
    let disabled_indices: Arc<HashSet<usize>> =
        Arc::new(root_state.disabled_indices.into_iter().collect());
    let state = root_state.state;
    let class = root_state.class_name;

    let open_state = overlay_open::use_controllable_open_state_traced(
        "autocomplete",
        open,
        default_open,
        on_open_change,
    );
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
        let next = logic::reduce_sync_from_selection(
            logic::InputStateSource {
                query: query.get_untracked(),
                has_typed: has_typed.get_untracked(),
            },
            selected_label.get(),
        );
        set_query.set(next.query);
        set_has_typed.set(next.has_typed);
    });

    let filtered_indices = Memo::new(move |_| {
        let items = items.get_value();
        logic::filter_indices(items.as_ref(), &query.get(), has_typed.get())
    });

    let (filtered_count, set_filtered_count) = signal(0_usize);
    Effect::new(move |_| set_filtered_count.set(filtered_indices.get().len()));

    let selected_filtered_index = Memo::new(move |_| {
        logic::map_selected_to_filtered(selected_index.get(), &filtered_indices.get())
    });

    let is_item_disabled = state.has_disabled_options.then_some({
        let disabled_indices = disabled_indices.clone();
        Callback::new(move |filtered_index: usize| {
            let indices = filtered_indices.get_untracked();
            let Some(original) = logic::map_filtered_to_original(filtered_index, &indices) else {
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
        let Some(original_index) = logic::map_filtered_to_original(filtered_index, &indices) else {
            return;
        };
        request_selected_index_change.run(Some(original_index));

        let items = items.get_value();
        let next = logic::reduce_after_option_commit(
            logic::InputStateSource {
                query: query.get_untracked(),
                has_typed: has_typed.get_untracked(),
            },
            items.get(original_index).cloned().unwrap_or_default(),
        );
        set_query.set(next.query);
        set_has_typed.set(next.has_typed);
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
        lang,
        dir,
    });

    let on_key_down = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        let key_result = aria.handlers.on_input_key_down.run(key);
        if key_result.handled {
            ev.prevent_default();
        }
        if key_result.stop_propagation {
            ev.stop_propagation();
        }
    };

    let on_focus = move |_| {
        focus_ring.handlers.on_focus.run(());
        aria.handlers.open.run(());
    };

    let on_blur = move |_| {
        focus_ring.handlers.on_blur.run(());
        aria.handlers.close.run(());
        let next = logic::reduce_after_input_blur(logic::InputStateSource {
            query: query.get_untracked(),
            has_typed: has_typed.get_untracked(),
        });
        set_query.set(next.query);
        set_has_typed.set(next.has_typed);
    };

    let on_input = move |ev| {
        if state.is_disabled {
            return;
        }
        let next = logic::reduce_after_input_change(
            logic::InputStateSource {
                query: query.get_untracked(),
                has_typed: has_typed.get_untracked(),
            },
            event_target_value(&ev),
        );
        set_query.set(next.query);
        set_has_typed.set(next.has_typed);
        aria.handlers.open.run(());
    };

    let control_ref: NodeRef<html::Div> = NodeRef::new();
    let description_view =
        render_autocomplete_description(description, text_field.description.id.clone());
    let error_view = render_autocomplete_error(error, text_field.error.id.clone(), invalid);
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::AutocompleteAgentContractInput {
            is_open: is_open.get(),
            is_disabled: state.is_disabled,
            has_typed: has_typed.get(),
            has_selection: selected_index.get().is_some(),
            is_open_controlled: normalized_open_state.is_controlled,
            selected_source: selection_change.selected_source,
            selected_change_source: selection_change.change_source,
            render_state: state,
        })
    });

    view! {
        <div
            class=class
            class:ui-autocomplete--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-autocomplete--invalid=move || invalid.get()
            class:ui-autocomplete--disabled=state.is_disabled
            data-slot="autocomplete"
            data-state=move || {
                logic::resolve_root_data_state(is_open.get(), state.is_disabled).as_attr()
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
            data-selected-source=selected_source_attr
            data-selected-controlled=is_selected_controlled.then_some("true")
            data-selected-uncontrolled=(!is_selected_controlled).then_some("true")
            data-selected-change-source=selected_change_source_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()
            data-ui-state-source=move || agent_contract.get().state_source
            data-ui-motion-source=move || agent_contract.get().motion_source
            data-ui-selected-source=move || agent_contract.get().selected_source
            data-ui-selected-change-source=move || agent_contract.get().selected_change_source
            data-ui-open-value-source=move || agent_contract.get().open_value_source
            data-ui-config-policy=move || agent_contract.get().config_policy
        >
            <label
                class="ui-autocomplete__label"
                id=label_id.get_value()
                for=text_field.label.for_attr.clone()
                data-slot="autocomplete-label"
            >
                {label.get_value()}
            </label>

            <div
                class="ui-autocomplete__control"
                node_ref=control_ref
                data-slot="autocomplete-control"
            >
                <input
                    class="ui-autocomplete__input"
                    data-slot="autocomplete-input"
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
                    aria-controls=move || aria.input.aria_controls.get()
                    aria-expanded=move || aria.input.aria_expanded.get()
                    aria-activedescendant=move || aria.input.aria_activedescendant.get()
                    aria-describedby=move || text_field.input.aria_describedby.get()
                    aria-invalid=move || text_field.input.aria_invalid.get()
                    aria-required=move || text_field.input.aria_required.get()
                    aria-disabled=aria.input.aria_disabled
                    lang=aria.input.lang.clone()
                    dir=aria.input.dir
                    on:input=on_input
                    on:keydown=on_key_down
                    on:focus=on_focus
                    on:blur=on_blur
                />

                <Show when=move || presence.is_present.get()>
                    <AutocompletePanel
                        open=is_open
                        aria=aria.clone()
                        anchor_ref=control_ref
                        aria_labelledby=label_id.get_value()
                        filtered_indices=filtered_indices
                        items=items
                        empty_message=empty_message
                        motion=motion.highlight
                        popover_motion=motion.popover
                        on_exit_complete=presence.finish_exit
                    />
                </Show>
            </div>

            {description_view}
            {error_view}
        </div>
    }
}
