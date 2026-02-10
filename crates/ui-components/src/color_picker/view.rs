use crate::color_picker::{
    ColorPickerMotion, ColorPickerStateInput,
    logic::{self},
};
use crate::color_swatch::{ColorSwatch, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize};
use crate::{OnPress, Popover, overlay_open, presence::use_presence};
use leptos::{html, prelude::*};
use ui_headless::PopoverPlacement;

#[component]
pub fn ColorPicker(
    id_base: String,
    children: ChildrenFn,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] selected_color: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_selected_color: Option<String>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] motion: ColorPickerMotion,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] swatch_size: ColorSwatchSize,
    #[prop(optional)] swatch_rounding: ColorSwatchRounding,
    #[prop(optional)] swatch_shape: ColorSwatchShape,
    #[prop(optional, default = true)] swatch_bordered: bool,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_optional_text(Some(id_base))
        .unwrap_or_else(|| "ui-color-picker".to_string());

    let ids = logic::resolve_ids(&id_base);
    let trigger_id = StoredValue::new(ids.trigger_id);
    let label_id = StoredValue::new(ids.label_id);
    let panel_id = StoredValue::new(ids.panel_id);
    let content_id = StoredValue::new(ids.content_id);

    let default_selected_color = logic::sanitize_selected_color(default_selected_color);
    let selected_state = overlay_open::use_controllable_state(
        selected_color,
        Some(default_selected_color),
        on_selected_change,
    );
    let selected_color =
        Memo::new(move |_| logic::sanitize_selected_color(selected_state.value.get()));

    let is_open_controlled = open.is_some();
    let open_state = overlay_open::use_controllable_open_state(open, default_open, on_open_change);
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let presence = use_presence(open);

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let (trigger_aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());
    let trigger_aria_label = StoredValue::new(trigger_aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(ColorPickerStateInput {
            disabled,
            open: open.get(),
            has_selection: selected_color.get().is_some(),
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name,
            is_open_controlled,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if disabled {
            return;
        }

        request_open_change.run(!open.get_untracked());
    });

    let on_close: OnPress = Callback::new(move |_| request_open_change.run(false));

    let aria_controls = crate::a11y::aria_controls_when_open(open, panel_id.get_value());

    let children = StoredValue::new(children);

    view! {
        <div
            id=id_base
            class=move || class.get()
            data-slot="color-picker"
            data-state=move || state.get().data_state_attr
            data-open=move || state.get().is_open.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-selection-empty=move || state.get().selection_empty.then_some("true")
            data-open-mode=move || state.get().open_mode_attr
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if motion == ColorPickerMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=move || (motion != ColorPickerMotion::default()).then_some("true")
        >
            <button
                id=trigger_id.get_value()
                node_ref=anchor_ref
                class="ui-color-picker__trigger"
                type="button"
                disabled=disabled
                aria-haspopup="dialog"
                aria-expanded=open
                aria-controls=aria_controls
                aria-label=move || trigger_aria_label.get_value()
                on:click=move |_| on_trigger_press.run(())
                data-slot="color-picker-trigger"
            >
                <span class="ui-color-picker__swatch" data-slot="color-picker-swatch" aria-hidden="true">
                    <Show
                        when=move || selected_color.get().is_some()
                        fallback=move || view! {
                            <ColorSwatch
                                decorative=true
                                size=swatch_size
                                rounding=swatch_rounding
                                shape=swatch_shape
                                bordered=swatch_bordered
                            />
                        }
                    >
                        {move || {
                            view! {
                                <ColorSwatch
                                    color=selected_color.get().unwrap_or_default()
                                    decorative=true
                                    size=swatch_size
                                    rounding=swatch_rounding
                                    shape=swatch_shape
                                    bordered=swatch_bordered
                                />
                            }
                        }}
                    </Show>
                </span>

                <span id=label_id.get_value() class="ui-color-picker__label" data-slot="color-picker-label">
                    {label.get_value()}
                </span>

                <Show when=move || selected_color.get().is_some()>
                    <span class="ui-color-picker__value" data-slot="color-picker-value">
                        {move || selected_color.get().unwrap_or_default()}
                    </span>
                </Show>
            </button>

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=open
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
                    motion=motion.popover
                    on_exit_complete=presence.finish_exit
                >
                    {move || {
                        let children = children.get_value();
                        view! {
                            <div
                                id=panel_id.get_value()
                                class="ui-color-picker__panel"
                                role="dialog"
                                aria-modal="false"
                                aria-label=move || trigger_aria_label.get_value()
                                aria-labelledby=label_id.get_value()
                                data-slot="color-picker-panel"
                            >
                                <div
                                    id=content_id.get_value()
                                    class="ui-color-picker__content"
                                    data-slot="color-picker-content"
                                >
                                    {children()}
                                </div>
                            </div>
                        }
                    }}
                </Popover>
            </Show>
        </div>
    }
}
