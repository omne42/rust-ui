use crate::color::area::ColorArea;
use crate::color::editor::{
    ColorEditorFormat, ColorEditorMotion, ColorEditorStateInput,
    logic::{self},
    motion as motion_contract,
};
use crate::color::field::ColorField;
use crate::color::slider::{ColorSlider, ColorSliderChannel};
use leptos::{ev, html, prelude::*};
use ui_headless as overlay_open;
use ui_headless::{
    A11yDirection, RovingOrientation, RovingTabIndexOptions, RovingTabIndexState, locale_attrs,
    tabs_list_a11y_attrs, tabs_tab_a11y_attrs, use_roving_tabindex,
};

const FORMAT_OPTIONS: [ColorEditorFormat; 4] = [
    ColorEditorFormat::Hex,
    ColorEditorFormat::Rgb,
    ColorEditorFormat::Hsl,
    ColorEditorFormat::Hsb,
];

fn format_to_index(format: ColorEditorFormat) -> usize {
    match format {
        ColorEditorFormat::Hex => 0,
        ColorEditorFormat::Rgb => 1,
        ColorEditorFormat::Hsl => 2,
        ColorEditorFormat::Hsb => 3,
    }
}

#[cfg(target_arch = "wasm32")]
fn focus_format_tab(tab_refs: &[NodeRef<html::Button>], index: usize) {
    let Some(node_ref) = tab_refs.get(index) else {
        return;
    };
    let Some(button) = node_ref.get_untracked() else {
        return;
    };
    ui_observability::observe_js_result!(button.focus());
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_format_tab(_tab_refs: &[NodeRef<html::Button>], _index: usize) {}

fn render_editor_header(
    label_id: String,
    label: StoredValue<String>,
    selected_color: Memo<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="ui-color-editor__header" data-slot="color-editor-header">
            <label id=label_id class="ui-color-editor__label" data-slot="color-editor-label">
                {label.get_value()}
            </label>

            <Show when=move || selected_color.get().is_some()>
                <span class="ui-color-editor__value" data-slot="color-editor-value">
                    {move || selected_color.get().unwrap_or_default()}
                </span>
            </Show>
        </div>
    }
}

#[derive(Clone)]
struct ColorEditorSlidersRenderInput {
    hue_id: String,
    alpha_id: String,
    hue_label: String,
    alpha_label: String,
    hue_signal: Signal<f64>,
    alpha_signal: Signal<f64>,
    on_hue_change: Callback<f64>,
    on_alpha_change: Callback<f64>,
    is_disabled: bool,
    is_alpha_channel_hidden: bool,
    motion: StoredValue<ColorEditorMotion>,
}

fn render_editor_sliders(input: ColorEditorSlidersRenderInput) -> impl IntoView {
    let ColorEditorSlidersRenderInput {
        hue_id,
        alpha_id,
        hue_label,
        alpha_label,
        hue_signal,
        alpha_signal,
        on_hue_change,
        on_alpha_change,
        is_disabled,
        is_alpha_channel_hidden,
        motion,
    } = input;
    let alpha_id = StoredValue::new(alpha_id);
    let alpha_label = StoredValue::new(alpha_label);

    view! {
        <div class="ui-color-editor__sliders" data-slot="color-editor-sliders">
            <ColorSlider
                id_base=hue_id
                channel=ColorSliderChannel::Hue
                label=hue_label
                value=hue_signal
                on_value_change=on_hue_change
                is_disabled=is_disabled
                motion=motion.get_value()
                class_name="ui-color-editor__slider ui-color-editor__slider--hue".to_string()
            />

            <Show when=move || !is_alpha_channel_hidden>
                {move || {
                    view! {
                        <ColorSlider
                            id_base=alpha_id.get_value()
                            channel=ColorSliderChannel::Alpha
                            label=alpha_label.get_value()
                            value=alpha_signal
                            on_value_change=on_alpha_change
                            is_disabled=is_disabled
                            motion=motion.get_value()
                            class_name="ui-color-editor__slider ui-color-editor__slider--alpha"
                                .to_string()
                        />
                    }
                }}
            </Show>
        </div>
    }
}

#[derive(Clone)]
struct ColorEditorCanvasRenderInput {
    area_id: String,
    area_label: String,
    area_aria_label: String,
    is_disabled: bool,
    area_signal: Signal<(f32, f32)>,
    on_area_change: Callback<(f32, f32)>,
    preview_color: String,
    sliders: ColorEditorSlidersRenderInput,
}

fn render_editor_canvas(input: ColorEditorCanvasRenderInput) -> impl IntoView {
    let ColorEditorCanvasRenderInput {
        area_id,
        area_label,
        area_aria_label,
        is_disabled,
        area_signal,
        on_area_change,
        preview_color,
        sliders,
    } = input;

    let sliders = render_editor_sliders(sliders);

    view! {
        <div class="ui-color-editor__canvas" data-slot="color-editor-canvas">
            <ColorArea
                id_base=area_id
                label=area_label
                is_disabled=is_disabled
                value=area_signal
                on_value_change=on_area_change
                preview_color=preview_color
                aria_label=area_aria_label
                class_name="ui-color-editor__area".to_string()
            />

            {sliders}
        </div>
    }
}

#[derive(Clone)]
struct ColorEditorFormatTabsRenderInput {
    formats_id: String,
    channels_id: String,
    format: Signal<ColorEditorFormat>,
    format_aria_label: String,
    normalized_lang: Option<String>,
    dir: Option<A11yDirection>,
    is_disabled: bool,
    roving: RovingTabIndexState,
    format_tab_refs: StoredValue<Vec<NodeRef<html::Button>>>,
    request_format_change: Callback<ColorEditorFormat>,
}

fn render_format_tabs(input: ColorEditorFormatTabsRenderInput) -> impl IntoView {
    let ColorEditorFormatTabsRenderInput {
        formats_id,
        channels_id,
        format,
        format_aria_label,
        normalized_lang,
        dir,
        is_disabled,
        roving,
        format_tab_refs,
        request_format_change,
    } = input;

    let tabs_list_a11y =
        tabs_list_a11y_attrs(Some(format_aria_label), normalized_lang.clone(), dir);
    let tabs_list_role = tabs_list_a11y.role;
    let tabs_list_aria_label = tabs_list_a11y.aria_label;
    let tabs_list_lang = tabs_list_a11y.lang;
    let tabs_list_dir = tabs_list_a11y.dir;
    let formats_id_for_tabs = formats_id.clone();

    view! {
        <div
            id=formats_id.clone()
            class="ui-color-editor__formats"
            data-slot="color-editor-formats"
            role=tabs_list_role
            aria-label=tabs_list_aria_label.clone()
            lang=tabs_list_lang.clone()
            dir=tabs_list_dir
        >
            {FORMAT_OPTIONS
                .into_iter()
                .enumerate()
                .map(move |(index, candidate)| {
                    let candidate_label = candidate.label();
                    let candidate_attr = candidate.as_attr();
                    let tab_id = format!("{formats_id_for_tabs}-{candidate_attr}-tab");
                    let controls_id = channels_id.clone();
                    let is_selected = Signal::derive(move || format.get() == candidate);
                    let tab_a11y = tabs_tab_a11y_attrs(
                        is_selected,
                        controls_id,
                        is_disabled,
                        normalized_lang.clone(),
                        dir,
                    );
                    let tab_role = tab_a11y.role;
                    let tab_aria_selected = tab_a11y.aria_selected;
                    let tab_aria_controls = tab_a11y.aria_controls;
                    let tab_aria_disabled = tab_a11y.aria_disabled;
                    let tab_lang = tab_a11y.lang;
                    let tab_dir = tab_a11y.dir;
                    let tab_ref = format_tab_refs.with_value(|refs| refs[index]);
                    let roving_on_item_focus = roving.handlers.on_item_focus;
                    let roving_on_key_down = roving.handlers.on_key_down;
                    let roving_active_index = roving.active_index;
                    let roving_active_index_for_keydown = roving.active_index;
                    let request_format_change_on_focus = request_format_change;
                    let request_format_change_on_click = request_format_change;

                    view! {
                        <button
                            type="button"
                            id=tab_id
                            class="ui-color-editor__format-button"
                            node_ref=tab_ref
                            role=tab_role
                            disabled=is_disabled
                            tabindex=move || if roving_active_index.get() == index { 0 } else { -1 }
                            aria-controls=tab_aria_controls
                            aria-selected=move || tab_aria_selected.get()
                            aria-disabled=tab_aria_disabled
                            lang=tab_lang.clone()
                            dir=tab_dir
                            data-slot="color-editor-format-button"
                            data-format=candidate_attr
                            data-selected=move || is_selected.get().then_some("true")
                            on:focus=move |_| {
                                roving_on_item_focus.run(index);
                                if is_disabled {
                                    return;
                                }
                                request_format_change_on_focus.run(candidate);
                            }
                            on:keydown=move |ev: ev::KeyboardEvent| {
                                if roving_on_key_down.run(ev.key()) {
                                    ev.prevent_default();
                                    let next = roving_active_index_for_keydown.get_untracked();
                                    format_tab_refs.with_value(|refs| focus_format_tab(refs, next));
                                }
                            }
                            on:click=move |_| {
                                if is_disabled {
                                    return;
                                }

                                request_format_change_on_click.run(candidate);
                            }
                        >
                            {candidate_label}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

fn render_channel_rows(channel_preview: Memo<Vec<(String, String)>>) -> impl IntoView {
    view! {
        {move || {
            channel_preview
                .get()
                .into_iter()
                .map(|(key, value)| {
                    view! {
                        <div class="ui-color-editor__channel-row" data-slot="color-editor-channel-row">
                            <span class="ui-color-editor__channel-key" data-slot="color-editor-channel-key">
                                {key}
                            </span>
                            <span class="ui-color-editor__channel-value" data-slot="color-editor-channel-value">
                                {value}
                            </span>
                        </div>
                    }
                })
                .collect_view()
        }}
    }
}

#[derive(Clone)]
struct ColorEditorControlsRenderInput {
    tabs: ColorEditorFormatTabsRenderInput,
    field_id: String,
    value_label: String,
    selected_color_signal: Signal<Option<String>>,
    on_field_change: Callback<Option<String>>,
    is_disabled: bool,
    channels_id: String,
    active_tab_id: Memo<String>,
    channel_preview: Memo<Vec<(String, String)>>,
}

fn render_editor_controls(input: ColorEditorControlsRenderInput) -> impl IntoView {
    let ColorEditorControlsRenderInput {
        tabs,
        field_id,
        value_label,
        selected_color_signal,
        on_field_change,
        is_disabled,
        channels_id,
        active_tab_id,
        channel_preview,
    } = input;

    let format_tabs = render_format_tabs(tabs);
    let channel_rows = render_channel_rows(channel_preview);

    view! {
        <div class="ui-color-editor__controls" data-slot="color-editor-controls">
            {format_tabs}

            <ColorField
                id_base=field_id
                label=value_label
                value=selected_color_signal
                on_value_change=on_field_change
                is_disabled=is_disabled
                class_name="ui-color-editor__field".to_string()
            />

            <div
                id=channels_id
                class="ui-color-editor__channels"
                data-slot="color-editor-channels"
                role="tabpanel"
                aria-labelledby=move || active_tab_id.get()
            >
                {channel_rows}
            </div>
        </div>
    }
}

#[component]
pub fn ColorEditor(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] selected_color: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_selected_color: Option<String>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional)] format: Option<Signal<ColorEditorFormat>>,
    #[prop(optional)] default_format: Option<ColorEditorFormat>,
    #[prop(optional)] on_format_change: Option<Callback<ColorEditorFormat>>,
    #[prop(optional)] is_alpha_channel_hidden: bool,
    #[prop(optional)] default_hue: Option<f64>,
    #[prop(optional)] default_alpha: Option<f64>,
    #[prop(optional)] default_area: Option<(f32, f32)>,
    #[prop(optional, into)] area_label: Option<String>,
    #[prop(optional, into)] area_aria_label: Option<String>,
    #[prop(optional, into)] hue_label: Option<String>,
    #[prop(optional, into)] alpha_label: Option<String>,
    #[prop(optional, into)] value_label: Option<String>,
    #[prop(optional, into)] format_aria_label: Option<String>,
    #[prop(optional, into)] preview_color: Option<String>,
    #[prop(optional)] motion: ColorEditorMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let is_selected_controlled = selected_color.is_some();
    let is_format_controlled = format.is_some();

    let normalized_defaults = logic::normalize_default_inputs(logic::ColorEditorDefaultInput {
        default_selected_color,
        default_format,
        default_hue,
        default_alpha,
        default_area,
        area_label,
        area_aria_label,
        hue_label,
        alpha_label,
        value_label,
        format_aria_label,
        preview_color,
        class_name,
        lang,
    });
    let logic::ColorEditorDefaultState {
        default_selected_color,
        default_format,
        default_hue,
        default_alpha,
        default_area,
        area_label,
        area_aria_label,
        hue_label,
        alpha_label,
        value_label,
        format_aria_label,
        preview_color,
        class_name: normalized_class_name,
        normalized_lang,
    } = normalized_defaults;

    let selected_state = overlay_open::use_controllable_state(
        selected_color,
        Some(default_selected_color),
        on_selected_change,
    );
    let selected_color = Memo::new(move |_| logic::sanitize_color(selected_state.value.get()));
    let selected_color_signal: Signal<Option<String>> = selected_state.value;

    let format_state =
        overlay_open::use_controllable_state(format, Some(default_format), on_format_change);
    let format = format_state.value;

    let (hue, set_hue) = signal(default_hue);
    let (alpha, set_alpha) = signal(default_alpha);
    let (area, set_area) = signal(default_area);
    let locale = locale_attrs(normalized_lang.clone(), dir);

    let hue_signal: Signal<f64> = hue.into();
    let alpha_signal: Signal<f64> = alpha.into();
    let area_signal: Signal<(f32, f32)> = area.into();

    let request_selected_change = selected_state.request_change;
    let request_selected_change_for_field = request_selected_change;
    let request_selected_change_for_area = request_selected_change;
    let request_selected_change_for_hue = request_selected_change;
    let request_selected_change_for_alpha = request_selected_change;
    let (ui_action, set_ui_action) = signal(logic::ColorEditorAgentAction::SnapshotRender);

    let on_field_change = Callback::new(move |next: Option<String>| {
        set_ui_action.set(logic::ColorEditorAgentAction::FieldInput);
        request_selected_change_for_field.run(logic::resolve_field_change(next));
    });

    let on_area_change = Callback::new(move |next: (f32, f32)| {
        if is_disabled {
            return;
        }

        set_ui_action.set(logic::ColorEditorAgentAction::AreaDragUpdate);
        let (next_area, next_color) = logic::resolve_area_change(
            next,
            hue.get_untracked(),
            alpha.get_untracked(),
            is_alpha_channel_hidden,
        );
        set_area.set(next_area);
        request_selected_change_for_area.run(Some(next_color));
    });

    let on_hue_change = Callback::new(move |next: f64| {
        if is_disabled {
            return;
        }

        set_ui_action.set(logic::ColorEditorAgentAction::HueDragUpdate);
        let (next_hue, next_color) = logic::resolve_hue_change(
            next,
            area.get_untracked(),
            alpha.get_untracked(),
            is_alpha_channel_hidden,
        );
        set_hue.set(next_hue);
        request_selected_change_for_hue.run(Some(next_color));
    });

    let on_alpha_change = Callback::new(move |next: f64| {
        if is_disabled {
            return;
        }

        set_ui_action.set(logic::ColorEditorAgentAction::AlphaDragUpdate);
        let (next_alpha, next_color) = logic::resolve_alpha_change(
            next,
            hue.get_untracked(),
            area.get_untracked(),
            is_alpha_channel_hidden,
        );
        set_alpha.set(next_alpha);
        request_selected_change_for_alpha.run(Some(next_color));
    });

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());
    let aria_label = StoredValue::new(aria_label);

    let has_custom_class_name = normalized_class_name.is_some();
    let class_name = StoredValue::new(normalized_class_name);

    let motion = StoredValue::new(motion_contract::attach_motion(motion));
    let has_custom_motion = motion_contract::source_attr(motion.get_value()) == "custom";

    let state = Memo::new(move |_| {
        logic::resolve_state(ColorEditorStateInput {
            disabled: is_disabled,
            hide_alpha_channel: is_alpha_channel_hidden,
            format: format.get(),
            has_selection: selected_color.get().is_some(),
            has_custom_motion,
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });
    let agent_contract = Memo::new(move |_| {
        logic::resolve_agent_contract(logic::ColorEditorAgentContractInput {
            render_state: state.get(),
            action: ui_action.get(),
            is_selected_controlled,
            is_format_controlled,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let channel_preview = Memo::new(move |_| {
        let area = area.get();
        logic::format_channel_preview(
            format.get(),
            hue.get(),
            f64::from(area.0 * 100.0),
            f64::from(area.1 * 100.0),
            alpha.get(),
            is_alpha_channel_hidden,
        )
    });

    let label_id = format!("{id_base}-label");
    let label_id_for_root = label_id.clone();
    let area_id = format!("{id_base}-area");
    let hue_id = format!("{id_base}-hue");
    let alpha_id = format!("{id_base}-alpha");
    let field_id = format!("{id_base}-field");
    let formats_id = format!("{id_base}-formats");
    let channels_id = format!("{id_base}-channels");
    let formats_id_for_active_tab = formats_id.clone();
    let active_tab_id = Memo::new(move |_| {
        let format_attr = format.get().as_attr();
        format!("{formats_id_for_active_tab}-{format_attr}-tab")
    });

    let request_format_change_inner = format_state.request_change;
    let request_format_change = Callback::new(move |next| {
        set_ui_action.set(logic::ColorEditorAgentAction::FormatChange);
        request_format_change_inner.run(next);
    });
    let (format_count, _set_format_count) = signal(FORMAT_OPTIONS.len());
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled,
        default_index: format_to_index(format.get_untracked()),
        should_loop: true,
        orientation: RovingOrientation::Horizontal,
        item_count: format_count,
        is_item_disabled: None,
    });
    let format_tab_refs = StoredValue::new(
        (0..FORMAT_OPTIONS.len())
            .map(|_| NodeRef::<html::Button>::new())
            .collect::<Vec<_>>(),
    );

    Effect::new(move |_| {
        roving
            .handlers
            .on_item_focus
            .run(format_to_index(format.get()));
    });

    let header = render_editor_header(label_id, label, selected_color);
    let canvas = render_editor_canvas(ColorEditorCanvasRenderInput {
        area_id,
        area_label,
        area_aria_label,
        is_disabled,
        area_signal,
        on_area_change,
        preview_color,
        sliders: ColorEditorSlidersRenderInput {
            hue_id,
            alpha_id,
            hue_label,
            alpha_label,
            hue_signal,
            alpha_signal,
            on_hue_change,
            on_alpha_change,
            is_disabled,
            is_alpha_channel_hidden,
            motion,
        },
    });
    let controls = render_editor_controls(ColorEditorControlsRenderInput {
        tabs: ColorEditorFormatTabsRenderInput {
            formats_id,
            channels_id: channels_id.clone(),
            format,
            format_aria_label,
            normalized_lang,
            dir,
            is_disabled,
            roving,
            format_tab_refs,
            request_format_change,
        },
        field_id,
        value_label,
        selected_color_signal,
        on_field_change,
        is_disabled,
        channels_id,
        active_tab_id,
        channel_preview,
    });

    view! {
        <div
            id=id_base
            class=move || class.get()
            role="group"
            aria-label=move || aria_label.get_value()
            aria-labelledby=label_id_for_root
            data-slot="color-editor"
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-format=move || state.get().format_attr
            data-alpha=move || state.get().alpha_visibility_attr
            data-motion-source=move || state.get().motion_source_attr
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode="snapshot"
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-selection-source=move || agent_contract.get().selection_source
            data-ui-format-source=move || agent_contract.get().format_source
            data-ui-motion-source=move || agent_contract.get().motion_source
            data-ui-label-source=move || agent_contract.get().label_source
            data-ui-aria-source=move || agent_contract.get().aria_source
            data-ui-class-source=move || agent_contract.get().class_source
            lang=locale.lang.clone()
            dir=locale.dir
        >
            {header}
            {canvas}
            {controls}
        </div>
    }
}
