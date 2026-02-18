use crate::color_area::ColorArea;
use crate::color_editor::{
    ColorEditorFormat, ColorEditorMotion, ColorEditorStateInput,
    logic::{self},
    motion as motion_contract,
};
use crate::color_field::ColorField;
use crate::color_slider::{ColorSlider, ColorSliderChannel};
use leptos::prelude::*;
use ui_headless as overlay_open;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn ColorEditor(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] selected_color: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_selected_color: Option<String>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional)] format: Option<Signal<ColorEditorFormat>>,
    #[prop(optional)] default_format: Option<ColorEditorFormat>,
    #[prop(optional)] on_format_change: Option<Callback<ColorEditorFormat>>,
    #[prop(optional)] hide_alpha_channel: bool,
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
    let default_selected_color = logic::sanitize_color(default_selected_color);
    let selected_state = overlay_open::use_controllable_state(
        selected_color,
        Some(default_selected_color),
        on_selected_change,
    );
    let selected_color = Memo::new(move |_| logic::sanitize_color(selected_state.value.get()));
    let selected_color_signal: Signal<Option<String>> = selected_state.value;

    let default_format = default_format.unwrap_or_default();
    let format_state =
        overlay_open::use_controllable_state(format, Some(default_format), on_format_change);
    let format = format_state.value;

    let (hue, set_hue) = signal(logic::sanitize_hue(
        default_hue.unwrap_or(logic::DEFAULT_HUE),
    ));
    let (alpha, set_alpha) = signal(logic::sanitize_alpha(
        default_alpha.unwrap_or(logic::DEFAULT_ALPHA),
    ));
    let (area, set_area) = signal(logic::sanitize_area(
        default_area.unwrap_or(logic::DEFAULT_AREA),
    ));
    let preview_color = logic::sanitize_color(preview_color).unwrap_or_else(|| {
        logic::compose_color_from_hsb(
            hue.get_untracked(),
            f64::from(area.get_untracked().0 * 100.0),
            f64::from(area.get_untracked().1 * 100.0),
            alpha.get_untracked(),
            true,
        )
    });
    let area_label = logic::normalize_optional_text(area_label)
        .unwrap_or_else(|| "Saturation / Brightness".to_string());
    let area_aria_label =
        logic::normalize_optional_text(area_aria_label).unwrap_or_else(|| "Color area".to_string());
    let hue_label = logic::normalize_optional_text(hue_label).unwrap_or_else(|| "Hue".to_string());
    let alpha_label =
        logic::normalize_optional_text(alpha_label).unwrap_or_else(|| "Alpha".to_string());
    let value_label =
        logic::normalize_optional_text(value_label).unwrap_or_else(|| "Value".to_string());
    let format_aria_label = logic::normalize_optional_text(format_aria_label)
        .unwrap_or_else(|| "Color format".to_string());
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);

    let hue_signal: Signal<f64> = hue.into();
    let alpha_signal: Signal<f64> = alpha.into();
    let area_signal: Signal<(f32, f32)> = area.into();

    let request_selected_change = selected_state.request_change;
    let request_selected_change_for_field = request_selected_change;
    let request_selected_change_for_area = request_selected_change;
    let request_selected_change_for_hue = request_selected_change;
    let request_selected_change_for_alpha = request_selected_change;

    let on_field_change = Callback::new(move |next: Option<String>| {
        request_selected_change_for_field.run(logic::sanitize_color(next));
    });

    let on_area_change = Callback::new(move |next: (f32, f32)| {
        if disabled {
            return;
        }

        let next = logic::sanitize_area(next);
        set_area.set(next);

        let next_color = logic::compose_color_from_hsb(
            hue.get_untracked(),
            f64::from(next.0 * 100.0),
            f64::from(next.1 * 100.0),
            alpha.get_untracked(),
            hide_alpha_channel,
        );
        request_selected_change_for_area.run(Some(next_color));
    });

    let on_hue_change = Callback::new(move |next: f64| {
        if disabled {
            return;
        }

        let next = logic::sanitize_hue(next);
        set_hue.set(next);

        let area = area.get_untracked();
        let next_color = logic::compose_color_from_hsb(
            next,
            f64::from(area.0 * 100.0),
            f64::from(area.1 * 100.0),
            alpha.get_untracked(),
            hide_alpha_channel,
        );
        request_selected_change_for_hue.run(Some(next_color));
    });

    let on_alpha_change = Callback::new(move |next: f64| {
        if disabled {
            return;
        }

        let next = logic::sanitize_alpha(next);
        set_alpha.set(next);

        let area = area.get_untracked();
        let next_color = logic::compose_color_from_hsb(
            hue.get_untracked(),
            f64::from(area.0 * 100.0),
            f64::from(area.1 * 100.0),
            next,
            hide_alpha_channel,
        );
        request_selected_change_for_alpha.run(Some(next_color));
    });

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let motion = StoredValue::new(motion_contract::sanitize_motion(motion));
    let has_custom_motion = motion_contract::source_attr(motion.get_value()) == "custom";

    let state = Memo::new(move |_| {
        logic::resolve_state(ColorEditorStateInput {
            disabled,
            hide_alpha_channel,
            format: format.get(),
            has_selection: selected_color.get().is_some(),
            has_custom_motion,
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name,
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
            hide_alpha_channel,
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

    let request_format_change = format_state.request_change;

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
            lang=locale.lang.clone()
            dir=locale.dir
        >
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

            <div class="ui-color-editor__canvas" data-slot="color-editor-canvas">
                <ColorArea
                    id_base=area_id
                    label=area_label.clone()
                    disabled=disabled
                    value=area_signal
                    on_value_change=on_area_change
                    preview_color=preview_color.clone()
                    aria_label=area_aria_label.clone()
                    class_name="ui-color-editor__area".to_string()
                />

                <div class="ui-color-editor__sliders" data-slot="color-editor-sliders">
                    <ColorSlider
                        id_base=hue_id
                        channel=ColorSliderChannel::Hue
                        label=hue_label.clone()
                        value=hue_signal
                        on_value_change=on_hue_change
                        disabled=disabled
                        motion=motion.get_value()
                        class_name="ui-color-editor__slider ui-color-editor__slider--hue".to_string()
                    />

                    <Show when=move || !state.get().hide_alpha_channel>
                        <ColorSlider
                            id_base=alpha_id.clone()
                            channel=ColorSliderChannel::Alpha
                            label=alpha_label.clone()
                            value=alpha_signal
                            on_value_change=on_alpha_change
                            disabled=disabled
                            motion=motion.get_value()
                            class_name="ui-color-editor__slider ui-color-editor__slider--alpha".to_string()
                        />
                    </Show>
                </div>
            </div>

            <div class="ui-color-editor__controls" data-slot="color-editor-controls">
                <div
                    id=formats_id
                    class="ui-color-editor__formats"
                    data-slot="color-editor-formats"
                    role="tablist"
                    aria-label=format_aria_label.clone()
                >
                    {[ColorEditorFormat::Hex, ColorEditorFormat::Rgb, ColorEditorFormat::Hsl, ColorEditorFormat::Hsb]
                        .into_iter()
                        .map(|candidate| {
                            let candidate_label = candidate.label();
                            let candidate_attr = candidate.as_attr();
                            let tab_id = format!("{formats_id}-{candidate_attr}-tab");
                            let controls_id = channels_id.clone();
                            let is_selected = move || format.get() == candidate;

                            view! {
                                <button
                                    type="button"
                                    id=tab_id
                                    class="ui-color-editor__format-button"
                                    role="tab"
                                    disabled=disabled
                                    tabindex=move || if is_selected() { 0 } else { -1 }
                                    aria-controls=controls_id
                                    aria-selected=move || if is_selected() { "true" } else { "false" }
                                    data-slot="color-editor-format-button"
                                    data-format=candidate_attr
                                    data-selected=move || is_selected().then_some("true")
                                    on:click=move |_| {
                                        if disabled {
                                            return;
                                        }

                                        request_format_change.run(candidate);
                                    }
                                >
                                    {candidate_label}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>

                <ColorField
                    id_base=field_id
                    label=value_label
                    value=selected_color_signal
                    on_value_change=on_field_change
                    disabled=disabled
                    class_name="ui-color-editor__field".to_string()
                />

                <div
                    id=channels_id
                    class="ui-color-editor__channels"
                    data-slot="color-editor-channels"
                    role="tabpanel"
                    aria-labelledby=move || active_tab_id.get()
                >
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
                </div>
            </div>
        </div>
    }
}
