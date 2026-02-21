use crate::{
    A11yDirection, ColorAreaMotion,
    logic::{self, ColorAreaDisableInput, ColorAreaRootInput},
    motion,
};
use leptos::prelude::*;
use ui_headless::{
    ColorAreaCellInput, ColorAreaKeyboardInput, ColorAreaOptions, CommonStrings, use_color_area,
    use_controllable_state, use_ui_i18n,
};

const SLOT_COLOR_AREA: &str = "color-area";
const SLOT_COLOR_AREA_LABEL: &str = "color-area-label";
const SLOT_COLOR_AREA_PREVIEW: &str = "color-area-preview";
const SLOT_COLOR_AREA_GRID: &str = "color-area-grid";
const SLOT_COLOR_AREA_ROW: &str = "color-area-row";
const SLOT_COLOR_AREA_CELL: &str = "color-area-cell";
const SLOT_COLOR_AREA_THUMB: &str = "color-area-thumb";
const SLOT_COLOR_AREA_AXES: &str = "color-area-axes";
const SLOT_COLOR_AREA_AXIS_X: &str = "color-area-axis-x";
const SLOT_COLOR_AREA_AXIS_Y: &str = "color-area-axis-y";
const BOOL_TRUE: &str = "true";

fn resolve_semantics(
    root_state: logic::ColorAreaRootState,
    label_id: String,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> ui_headless::ColorAreaContract {
    use_color_area(ColorAreaOptions {
        state: root_state.state,
        aria_label: root_state.aria_label,
        label_id,
        x_axis_label: root_state.x_axis_label,
        y_axis_label: root_state.y_axis_label,
        lang,
        dir,
    })
}

#[component]
pub fn ColorArea(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] value: Option<Signal<(f32, f32)>>,
    #[prop(optional)] default_value: Option<(f32, f32)>,
    #[prop(optional)] on_value_change: Option<Callback<(f32, f32)>>,
    #[prop(optional)] step: Option<f32>,
    #[prop(optional)] grid_size: Option<usize>,
    #[prop(optional, into)] preview_color: Option<String>,
    #[prop(optional)] motion: ColorAreaMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] x_axis_label: Option<String>,
    #[prop(optional, into)] y_axis_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();

    let default_value = logic::normalize_default_value(default_value);
    let step = logic::normalize_step(step);
    let grid_size = logic::normalize_grid_size(grid_size);
    let value_axis = logic::normalize_value_axis(value.is_some());
    let controllable = use_controllable_state(value, Some(default_value), on_value_change);
    let value = controllable.value;
    let request_value_change = controllable.request_change;

    let label_id = format!("{id_base}-label");
    let x_input_id = format!("{id_base}-x");
    let y_input_id = format!("{id_base}-y");
    let label_id_for_semantics = StoredValue::new(label_id.clone());
    let lang_for_semantics = StoredValue::new(lang.clone());
    let dir_for_semantics = StoredValue::new(dir);

    let root = Memo::new(move |_| {
        logic::normalize_root_state(ColorAreaRootInput {
            class_name: class_name.clone(),
            label: label.clone(),
            fallback_label: common.color_area_label.as_ref().into(),
            aria_label: aria_label.clone(),
            fallback_aria_label: common.color_area_aria_label.as_ref().into(),
            x_axis_label: x_axis_label.clone(),
            fallback_x_axis_label: common.color_area_x_axis_label.as_ref().into(),
            y_axis_label: y_axis_label.clone(),
            fallback_y_axis_label: common.color_area_y_axis_label.as_ref().into(),
            preview_color: preview_color.clone(),
            value: value.get(),
            step,
            grid_size,
            disabled: ColorAreaDisableInput { is_disabled },
        })
    });

    let class =
        Memo::new(move |_| logic::compose_class_name(root.get().class_name, root.get().state));

    let motion = motion::sanitize_motion(motion);
    let motion_source = motion::resolve_source(motion);
    let has_custom_motion = motion_source.is_custom();
    let inline_style = Memo::new(move |_| {
        let preview = root
            .get()
            .preview_color
            .map(|color| format!("--ui-color-area-preview-color: {color};"));
        motion::attach_motion(preview, motion)
    });

    let on_x_input = move |ev| {
        let parsed = resolve_semantics(
            root.get_untracked(),
            label_id_for_semantics.get_value(),
            lang_for_semantics.get_value(),
            dir_for_semantics.get_value(),
        )
        .handlers
        .parse_axis_input
        .run(event_target_value(&ev));
        let outcome = logic::reduce_axis_input(
            logic::resolve_interactivity(root.get_untracked().state),
            value.get_untracked(),
            logic::ColorAreaAxis::X,
            parsed,
        );

        let Some(next_value) = outcome.next_value else {
            return;
        };
        request_value_change.run(next_value);
    };

    let on_y_input = move |ev| {
        let parsed = resolve_semantics(
            root.get_untracked(),
            label_id_for_semantics.get_value(),
            lang_for_semantics.get_value(),
            dir_for_semantics.get_value(),
        )
        .handlers
        .parse_axis_input
        .run(event_target_value(&ev));
        let outcome = logic::reduce_axis_input(
            logic::resolve_interactivity(root.get_untracked().state),
            value.get_untracked(),
            logic::ColorAreaAxis::Y,
            parsed,
        );

        let Some(next_value) = outcome.next_value else {
            return;
        };
        request_value_change.run(next_value);
    };

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        let key = ev.key();
        let result = resolve_semantics(
            root.get_untracked(),
            label_id_for_semantics.get_value(),
            lang_for_semantics.get_value(),
            dir_for_semantics.get_value(),
        )
        .handlers
        .on_key_down
        .run(ColorAreaKeyboardInput {
            key,
            current_value: value.get_untracked(),
        });

        let (next_value, prevent_default) = if let Some(result) = result {
            (Some(result.next_value), result.prevent_default)
        } else {
            (None, false)
        };
        let outcome = logic::reduce_keyboard_result(
            logic::resolve_interactivity(root.get_untracked().state),
            next_value,
            prevent_default,
        );

        if let Some(next_value) = outcome.next_value {
            request_value_change.run(next_value);
        }
        if outcome.prevent_default {
            ev.prevent_default();
        }
    };

    view! {
        <div
            id=id_base
            class=move || class.get()
            style=move || inline_style.get()
            role=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .role
            aria-label=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .aria_label
            aria-labelledby=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .aria_labelledby
            lang=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .lang
            dir=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .dir
            data-slot=SLOT_COLOR_AREA
            data-motion-source=motion_source.as_attr()
            data-custom-motion=has_custom_motion.then_some(BOOL_TRUE)
            data-state=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_state
            data-disabled=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_disabled
            data-disabled-source=move || root.get().disabled_source_attr.as_attr()
            data-step=move || root.get().state.step
            data-grid-size=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_grid_size
            data-value-x=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_value_x
            data-value-y=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_value_y
            data-selected-col=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_selected_col
            data-selected-row=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_selected_row
            data-has-preview=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_has_preview
            data-label-source=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_label_source
            data-aria-source=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_aria_source
            data-custom-class=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_custom_class
            data-class-source=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_class_source
            data-x-axis-source=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_x_axis_source
            data-y-axis-source=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .data_y_axis_source
            data-value-control-mode=value_axis.control_mode.as_attr()
            data-value-source=value_axis.value_source.as_attr()
            data-ui-schema=move || logic::resolve_agent_contract(root.get().state, value_axis).schema_attr
            data-ui-stream-support=move || logic::resolve_agent_contract(root.get().state, value_axis).stream_support_attr
            data-ui-stream-fallback=move || logic::resolve_agent_contract(root.get().state, value_axis).stream_fallback_attr
            data-ui-stream-mode=move || logic::resolve_agent_contract(root.get().state, value_axis).stream_mode_attr
            data-ui-output-status=move || logic::resolve_agent_contract(root.get().state, value_axis).output_status_attr
            data-ui-intent=move || logic::resolve_agent_contract(root.get().state, value_axis).intent_attr
            data-ui-action=move || logic::resolve_agent_contract(root.get().state, value_axis).action_attr
            data-ui-state=move || logic::resolve_agent_contract(root.get().state, value_axis).state_attr
            data-ui-source=move || logic::resolve_agent_contract(root.get().state, value_axis).source_attr
            tabindex=move || resolve_semantics(
                root.get(),
                label_id_for_semantics.get_value(),
                lang_for_semantics.get_value(),
                dir_for_semantics.get_value(),
            )
            .root_attrs
            .tabindex
            on:keydown=on_key_down
        >
            <label
                id=label_id.clone()
                class="ui-color-area__label"
                data-slot=SLOT_COLOR_AREA_LABEL
                for=x_input_id.clone()
            >
                {move || root.get().label}
            </label>

            <Show when=move || root.get().preview_color.is_some()>
                <span
                    class="ui-color-area__preview"
                    data-slot=SLOT_COLOR_AREA_PREVIEW
                    aria-hidden="true"
                ></span>
            </Show>

            <div
                class="ui-color-area__grid"
                data-slot=SLOT_COLOR_AREA_GRID
                role=move || resolve_semantics(
                    root.get(),
                    label_id_for_semantics.get_value(),
                    lang_for_semantics.get_value(),
                    dir_for_semantics.get_value(),
                )
                .grid_attrs
                .role
                aria-disabled=move || resolve_semantics(
                    root.get(),
                    label_id_for_semantics.get_value(),
                    lang_for_semantics.get_value(),
                    dir_for_semantics.get_value(),
                )
                .grid_attrs
                .aria_disabled
            >
                {move || {
                    let root_state = root.get();
                    let state = root_state.state;
                    (0..state.grid_size)
                        .map(|row| {
                            view! {
                                <div class="ui-color-area__row" data-slot=SLOT_COLOR_AREA_ROW role="row">
                                    {(0..state.grid_size)
                                        .map(|col| {
                                            let cell = resolve_semantics(
                                                root.get_untracked(),
                                                label_id_for_semantics.get_value(),
                                                lang_for_semantics.get_value(),
                                                dir_for_semantics.get_value(),
                                            )
                                            .handlers
                                            .resolve_cell
                                            .run(ColorAreaCellInput { col, row });
                                            let cell_value = cell.value;
                                            let cell_disabled = cell.attrs.disabled;
                                            let cell_role = cell.attrs.role;
                                            let cell_aria_label = cell.attrs.aria_label;
                                            let cell_aria_selected = cell.attrs.aria_selected;
                                            let cell_tabindex = cell.attrs.tabindex;
                                            let cell_data_selected = cell.attrs.data_selected;

                                            view! {
                                                <button
                                                    type="button"
                                                    class="ui-color-area__cell"
                                                    role=cell_role
                                                    aria-label=cell_aria_label
                                                    aria-selected=cell_aria_selected
                                                    tabindex=cell_tabindex
                                                    disabled=cell_disabled
                                                    data-slot=SLOT_COLOR_AREA_CELL
                                                    data-row=row
                                                    data-col=col
                                                    data-selected=cell_data_selected
                                                    on:click=move |_| {
                                                        let outcome = logic::reduce_cell_select(
                                                            logic::resolve_interactivity(root.get_untracked().state),
                                                            cell_value,
                                                        );
                                                        if let Some(next_value) = outcome.next_value {
                                                            request_value_change.run(next_value);
                                                        }
                                                    }
                                                >
                                                    <span class="ui-color-area__thumb" data-slot=SLOT_COLOR_AREA_THUMB></span>
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

            <div class="ui-color-area__axes" data-slot=SLOT_COLOR_AREA_AXES>
                <label class="ui-color-area__axis-label" for=x_input_id.clone()>
                    {move || root.get().x_axis_label}
                </label>
                <input
                    id=x_input_id
                    class="ui-color-area__axis-input"
                    data-slot=SLOT_COLOR_AREA_AXIS_X
                    type="range"
                    min="0"
                    max="100"
                    step=move || root.get().state.step * 100.0
                    prop:value=move || root.get().state.value_x_percent
                    disabled=move || root.get().state.is_disabled
                    aria-disabled=move || resolve_semantics(
                        root.get(),
                        label_id_for_semantics.get_value(),
                        lang_for_semantics.get_value(),
                        dir_for_semantics.get_value(),
                    )
                    .axis_attrs
                    .aria_disabled
                    aria-label=move || root.get().x_axis_label
                    on:input=on_x_input
                />

                <label class="ui-color-area__axis-label" for=y_input_id.clone()>
                    {move || root.get().y_axis_label}
                </label>
                <input
                    id=y_input_id
                    class="ui-color-area__axis-input"
                    data-slot=SLOT_COLOR_AREA_AXIS_Y
                    type="range"
                    min="0"
                    max="100"
                    step=move || root.get().state.step * 100.0
                    prop:value=move || root.get().state.value_y_percent
                    disabled=move || root.get().state.is_disabled
                    aria-disabled=move || resolve_semantics(
                        root.get(),
                        label_id_for_semantics.get_value(),
                        lang_for_semantics.get_value(),
                        dir_for_semantics.get_value(),
                    )
                    .axis_attrs
                    .aria_disabled
                    aria-label=move || root.get().y_axis_label
                    on:input=on_y_input
                />
            </div>
        </div>
    }
}
