use crate::ColorSwatch;
use crate::color_area::{
    ColorAreaMotion, ColorAreaStateInput,
    logic::{self},
    motion,
};
use leptos::prelude::*;
use ui_headless as overlay_open;

#[component]
pub fn ColorArea(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] value: Option<Signal<(f32, f32)>>,
    #[prop(optional)] default_value: Option<(f32, f32)>,
    #[prop(optional)] on_value_change: Option<Callback<(f32, f32)>>,
    #[prop(optional, default = logic::DEFAULT_STEP)] step: f32,
    #[prop(optional, default = logic::DEFAULT_GRID_SIZE)] grid_size: usize,
    #[prop(optional, into)] preview_color: Option<String>,
    #[prop(optional)] motion: ColorAreaMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let default_value = logic::clamp_value(default_value.unwrap_or((1.0, 1.0)));
    let value_state =
        overlay_open::use_controllable_state(value, Some(default_value), on_value_change);
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let step = logic::sanitize_step(step);
    let grid_size = logic::sanitize_grid_size(grid_size);

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let preview_color = logic::sanitize_preview_color(preview_color);
    let has_preview_color = preview_color.is_some();
    let preview_color = StoredValue::new(preview_color);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(ColorAreaStateInput {
            disabled,
            step,
            value: value.get(),
            grid_size,
            has_preview_color,
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != ColorAreaMotion::default();

    let label_id = format!("{id_base}-label");
    let x_input_id = format!("{id_base}-x");
    let y_input_id = format!("{id_base}-y");

    let label_id_for_root = label_id.clone();
    let label_id_for_label = label_id.clone();
    let x_input_id_for_main_label = x_input_id.clone();
    let x_input_id_for_axis_label = x_input_id.clone();
    let x_input_id_for_input = x_input_id.clone();
    let y_input_id_for_axis_label = y_input_id.clone();

    let on_x_input = move |ev| {
        if disabled {
            return;
        }

        let Some(next_x) = logic::parse_axis_percent(&event_target_value(&ev)) else {
            return;
        };

        let current = logic::clamp_value(value.get_untracked());
        request_value_change.run((next_x, current.1));
    };

    let on_y_input = move |ev| {
        if disabled {
            return;
        }

        let Some(next_y) = logic::parse_axis_percent(&event_target_value(&ev)) else {
            return;
        };

        let current = logic::clamp_value(value.get_untracked());
        request_value_change.run((current.0, next_y));
    };

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if disabled {
            return;
        }

        let next = match ev.key().as_str() {
            "ArrowLeft" => Some(logic::move_value_by_delta(
                value.get_untracked(),
                -1.0,
                0.0,
                step,
            )),
            "ArrowRight" => Some(logic::move_value_by_delta(
                value.get_untracked(),
                1.0,
                0.0,
                step,
            )),
            "ArrowUp" => Some(logic::move_value_by_delta(
                value.get_untracked(),
                0.0,
                1.0,
                step,
            )),
            "ArrowDown" => Some(logic::move_value_by_delta(
                value.get_untracked(),
                0.0,
                -1.0,
                step,
            )),
            "Home" => Some((0.0, 0.0)),
            "End" => Some((1.0, 1.0)),
            _ => None,
        };

        if let Some(next) = next {
            request_value_change.run(next);
            ev.prevent_default();
        }
    };

    view! {
        <div
            id=id_base
            class=move || class.get()
            role="group"
            aria-label=aria_label
            aria-labelledby=label_id_for_root
            data-slot="color-area"
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-step=move || state.get().step.to_string()
            data-grid-size=move || state.get().grid_size.to_string()
            data-value-x=move || state.get().value_x_percent.to_string()
            data-value-y=move || state.get().value_y_percent.to_string()
            data-selected-col=move || state.get().selected_col.to_string()
            data-selected-row=move || state.get().selected_row.to_string()
            data-has-preview=move || state.get().has_preview_color.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            tabindex=if disabled { -1 } else { 0 }
            on:keydown=on_key_down
        >
            <label
                id=label_id_for_label
                class="ui-color-area__label"
                data-slot="color-area-label"
                for=x_input_id_for_main_label
            >
                {label.get_value()}
            </label>

            <Show when=move || has_preview_color>
                {move || {
                    let swatch = if let Some(color) = preview_color.get_value() {
                        view! { <ColorSwatch color=color decorative=true /> }.into_any()
                    } else {
                        view! { <ColorSwatch decorative=true /> }.into_any()
                    };

                    view! {
                        <span class="ui-color-area__preview" data-slot="color-area-preview" aria-hidden="true">
                            {swatch}
                        </span>
                    }
                }}
            </Show>

            <div
                class="ui-color-area__grid"
                data-slot="color-area-grid"
                role="grid"
                aria-disabled=disabled.then_some("true")
            >
                {move || {
                    let state = state.get();
                    (0..state.grid_size)
                        .map(|row| {
                            view! {
                                <div class="ui-color-area__row" data-slot="color-area-row" role="row">
                                    {(0..state.grid_size)
                                        .map(|col| {
                                            let is_selected = row == state.selected_row && col == state.selected_col;
                                            let cell_value = logic::value_from_cell(col, row, state.grid_size);
                                            let aria_label = format!(
                                                "Saturation {}%, Lightness {}%",
                                                (cell_value.0 * 100.0).round() as u8,
                                                (cell_value.1 * 100.0).round() as u8
                                            );

                                            view! {
                                                <button
                                                    type="button"
                                                    class="ui-color-area__cell"
                                                    role="gridcell"
                                                    aria-label=aria_label
                                                    aria-selected=is_selected.then_some("true")
                                                    tabindex=if is_selected && !disabled { 0 } else { -1 }
                                                    disabled=disabled
                                                    data-slot="color-area-cell"
                                                    data-row=row
                                                    data-col=col
                                                    data-selected=is_selected.then_some("true")
                                                    on:click=move |_| {
                                                        if disabled {
                                                            return;
                                                        }
                                                        request_value_change.run(cell_value);
                                                    }
                                                >
                                                    <span class="ui-color-area__thumb" data-slot="color-area-thumb"></span>
                                                </button>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>

            <div class="ui-color-area__axes" data-slot="color-area-axes">
                <label class="ui-color-area__axis-label" for=x_input_id_for_axis_label>
                    "X"
                </label>
                <input
                    id=x_input_id_for_input
                    class="ui-color-area__axis-input"
                    data-slot="color-area-axis-x"
                    type="range"
                    min="0"
                    max="100"
                    step=move || (state.get().step * 100.0).to_string()
                    prop:value=move || state.get().value_x_percent.to_string()
                    disabled=disabled
                    aria-label="Saturation"
                    on:input=on_x_input
                />

                <label class="ui-color-area__axis-label" for=y_input_id_for_axis_label>
                    "Y"
                </label>
                <input
                    id=y_input_id
                    class="ui-color-area__axis-input"
                    data-slot="color-area-axis-y"
                    type="range"
                    min="0"
                    max="100"
                    step=move || (state.get().step * 100.0).to_string()
                    prop:value=move || state.get().value_y_percent.to_string()
                    disabled=disabled
                    aria-label="Lightness"
                    on:input=on_y_input
                />
            </div>
        </div>
    }
}
