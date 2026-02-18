use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::color_handle::ColorHandleMotion;
use ui_components::{
    ColorArea, ColorEditor, ColorEditorFormat, ColorField, ColorHandle, ColorLoupe, ColorPicker,
    ColorSlider, ColorSliderChannel, ColorSliderMotion, ColorSwatchPicker, ColorSwatchPickerItem,
    ColorThumb, ColorWheel, ColorWheelMotion, SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn color_field() -> AnyView {
    let (value, set_value) = signal(Some("#4f46e5".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));

    let basic_code = Signal::derive(move || {
        r##"let (value, set_value) = signal(Some("#4f46e5".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));

<ColorField
  id_base="docs-color-field-basic".to_string()
  label="Fill color".to_string()
  value=value.into()
  on_value_change=on_value_change
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<ColorField
  id_base="docs-color-field-invalid".to_string()
  label="Brand color".to_string()
  default_value="javascript:alert(1)".to_string()
  class_name="docs-color-field-custom".to_string()
/>
<ColorField
  id_base="docs-color-field-disabled".to_string()
  label="Accent color".to_string()
  default_value="#0ea5e9".to_string()
  is_disabled=true
/>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorField"
            slug="color-field"
            group="Forms"
            description="baseline-compatible color text field with centralized label/placeholder/aria/state normalization, sanitized preview rendering, and stable slot/data contracts."
        >
            <Playground title="Controlled Value" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorField
                        id_base="docs-color-field-basic".to_string()
                        label="Fill color".to_string()
                        value=value.into()
                        on_value_change=on_value_change
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Invalid + Disabled + Custom Class" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorField
                        id_base="docs-color-field-invalid".to_string()
                        label="Brand color".to_string()
                        default_value="javascript:alert(1)".to_string()
                        class_name="docs-color-field-custom".to_string()
                    />
                    <ColorField
                        id_base="docs-color-field-disabled".to_string()
                        label="Accent color".to_string()
                        default_value="#0ea5e9".to_string()
                        disabled=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_area() -> AnyView {
    let (value, set_value) = signal((0.6_f32, 0.4_f32));
    let on_value_change = Callback::new(move |next: (f32, f32)| set_value.set(next));
    let grid_options = vec!["9".to_string(), "11".to_string(), "15".to_string()];
    let step_options = vec!["0.05".to_string(), "0.10".to_string(), "0.20".to_string()];
    let color_options = vec![
        "violet".to_string(),
        "emerald".to_string(),
        "amber".to_string(),
        "none".to_string(),
    ];
    let position_options = vec!["start".to_string(), "center".to_string(), "end".to_string()];

    let (grid_index, set_grid_index) = signal(Some(1_usize));
    let (step_index, set_step_index) = signal(Some(1_usize));
    let (color_index, set_color_index) = signal(Some(0_usize));
    let (position_index, set_position_index) = signal(Some(1_usize));
    let (is_disabled, set_is_disabled) = signal(false);
    let (custom_axis_labels, set_custom_axis_labels) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (show_preview, set_show_preview) = signal(true);

    let basic_code = Signal::derive(move || {
        r##"let (value, set_value) = signal((0.6_f32, 0.4_f32));
let on_value_change = Callback::new(move |next: (f32, f32)| set_value.set(next));

<ColorArea
  id_base="docs-color-area-basic".to_string()
  label="Saturation / Lightness".to_string()
  value=value.into()
  on_value_change=on_value_change
  preview_color="#7c3aed".to_string()
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<ColorArea
  id_base="docs-color-area-disabled".to_string()
  label="Accent area".to_string()
  default_value=(0.25, 0.85)
  grid_size=15
  step=0.05
  disabled=true
  class_name="docs-color-area-custom".to_string()
/>"##
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let grid_size = match grid_index.get().unwrap_or(1) {
            0 => 9,
            2 => 15,
            _ => 11,
        };
        let step = match step_index.get().unwrap_or(1) {
            0 => 0.05_f32,
            2 => 0.2_f32,
            _ => 0.1_f32,
        };
        let preview_color = match color_index.get().unwrap_or(0) {
            0 => "#7c3aed",
            1 => "#10b981",
            2 => "#f59e0b",
            _ => "",
        };
        let default_value = match position_index.get().unwrap_or(1) {
            0 => (0.2_f32, 0.8_f32),
            2 => (0.85_f32, 0.25_f32),
            _ => (0.6_f32, 0.4_f32),
        };
        let disabled = is_disabled.get();
        let custom_axis_labels = custom_axis_labels.get();
        let custom_class = custom_class.get();
        let show_preview = show_preview.get();

        format!(
            "<ColorArea\n  id_base=\"docs-color-area-workbench\".to_string()\n  label=\"Color workbench\".to_string()\n  default_value=({:.2}, {:.2})\n  grid_size={}\n  step={:.2}\n  is_disabled={}\n  preview_color=\"{}\".to_string()\n  x_axis_label=\"{}\".to_string()\n  y_axis_label=\"{}\".to_string()\n  class_name=\"{}\".to_string()\n/>",
            default_value.0,
            default_value.1,
            grid_size,
            step,
            disabled,
            if show_preview { preview_color } else { "" },
            if custom_axis_labels {
                "Saturation (X)"
            } else {
                ""
            },
            if custom_axis_labels {
                "Lightness (Y)"
            } else {
                ""
            },
            if custom_class {
                "docs-color-area-workbench"
            } else {
                ""
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let grid_size = match grid_index.get().unwrap_or(1) {
            0 => 9,
            2 => 15,
            _ => 11,
        };
        let step = match step_index.get().unwrap_or(1) {
            0 => 0.05_f32,
            2 => 0.2_f32,
            _ => 0.1_f32,
        };
        let preview_color = match color_index.get().unwrap_or(0) {
            0 => "#7c3aed",
            1 => "#10b981",
            2 => "#f59e0b",
            _ => "",
        };
        let default_value = match position_index.get().unwrap_or(1) {
            0 => (0.2_f32, 0.8_f32, "start"),
            2 => (0.85_f32, 0.25_f32, "end"),
            _ => (0.6_f32, 0.4_f32, "center"),
        };

        format!(
            "ColorAreaActualConfig {{\n  default_value: ({:.2}, {:.2}) [{}],\n  grid_size: {},\n  step: {:.2},\n  is_disabled: {},\n  preview_color: \"{}\",\n  custom_axis_labels: {},\n  custom_class: {},\n  show_preview: {},\n}}",
            default_value.0,
            default_value.1,
            default_value.2,
            grid_size,
            step,
            is_disabled.get(),
            if show_preview.get() {
                preview_color
            } else {
                ""
            },
            custom_axis_labels.get(),
            custom_class.get(),
            show_preview.get(),
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/color_area/styles.rs */\n{}",
            ui_components::color_area::styles::CSS
        )
    });

    view! {
        <ComponentPage
            title="ColorArea"
            slug="color-area"
            group="Forms"
            description="baseline-compatible two-axis color selection primitive with centralized step/grid normalization, keyboard navigation, and stable slot/data-state contracts."
        >
            <Playground title="Controlled Grid Selection" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorArea
                        id_base="docs-color-area-basic".to_string()
                        label="Saturation / Lightness".to_string()
                        value=value.into()
                        on_value_change=on_value_change
                        preview_color="#7c3aed".to_string()
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || {
                            let (x, y) = value.get();
                            format!("({:.2}, {:.2})", x, y)
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Custom Grid + Custom Class" code_signal=states_code>
                <ColorArea
                    id_base="docs-color-area-disabled".to_string()
                    label="Accent area".to_string()
                    default_value=(0.25, 0.85)
                    grid_size=15
                    step=0.05
                    is_disabled=true
                    class_name="docs-color-area-custom".to_string()
                />
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含可调主样例 + 固定对照样例）。"
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="crates/ui-components/src/color_area/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Grid size"</div>
                        <SegmentedControl
                            id_base="docs-color-area-workbench-grid".to_string()
                            options=grid_options.clone()
                            selected_index=grid_index
                            set_selected_index=set_grid_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorArea grid size".to_string()
                        />

                        <div class="docs-search__label">"Step"</div>
                        <SegmentedControl
                            id_base="docs-color-area-workbench-step".to_string()
                            options=step_options.clone()
                            selected_index=step_index
                            set_selected_index=set_step_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorArea step".to_string()
                        />

                        <div class="docs-search__label">"Preview color"</div>
                        <SegmentedControl
                            id_base="docs-color-area-workbench-color".to_string()
                            options=color_options.clone()
                            selected_index=color_index
                            set_selected_index=set_color_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorArea preview color".to_string()
                        />

                        <div class="docs-search__label">"Default position"</div>
                        <SegmentedControl
                            id_base="docs-color-area-workbench-position".to_string()
                            options=position_options.clone()
                            selected_index=position_index
                            set_selected_index=set_position_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorArea default position".to_string()
                        />

                        <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                        <Switch checked=custom_axis_labels set_checked=set_custom_axis_labels>
                            "Custom axis labels"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=show_preview set_checked=set_show_preview>"Show preview color"</Switch>
                    </div>
                }
            >
                {move || {
                    let (default_x, default_y) = match position_index.get().unwrap_or(1) {
                        0 => (0.2_f32, 0.8_f32),
                        2 => (0.85_f32, 0.25_f32),
                        _ => (0.6_f32, 0.4_f32),
                    };
                    let grid_size = match grid_index.get().unwrap_or(1) {
                        0 => 9,
                        2 => 15,
                        _ => 11,
                    };
                    let step = match step_index.get().unwrap_or(1) {
                        0 => 0.05_f32,
                        2 => 0.2_f32,
                        _ => 0.1_f32,
                    };
                    let preview_color = match color_index.get().unwrap_or(0) {
                        0 => "#7c3aed".to_string(),
                        1 => "#10b981".to_string(),
                        2 => "#f59e0b".to_string(),
                        _ => String::new(),
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-row docs-row--wrap" style="align-items: flex-start; gap: var(--ui-space-lg);">
                                <div class="docs-stack docs-stack--tight">
                                    <span class="ui-muted">"主样例（可调）"</span>
                                    <ColorArea
                                        id_base="docs-color-area-workbench-main".to_string()
                                        label="Color workbench".to_string()
                                        default_value=(default_x, default_y)
                                        grid_size=grid_size
                                        step=step
                                        is_disabled=is_disabled.get()
                                        preview_color=if show_preview.get() {
                                            preview_color.clone()
                                        } else {
                                            String::new()
                                        }
                                        x_axis_label=if custom_axis_labels.get() {
                                            "Saturation (X)".to_string()
                                        } else {
                                            String::new()
                                        }
                                        y_axis_label=if custom_axis_labels.get() {
                                            "Lightness (Y)".to_string()
                                        } else {
                                            String::new()
                                        }
                                        class_name=if custom_class.get() {
                                            "docs-color-area-workbench".to_string()
                                        } else {
                                            String::new()
                                        }
                                    />
                                </div>

                                <div class="docs-stack docs-stack--tight">
                                    <span class="ui-muted">"对照 A（固定中心）"</span>
                                    <ColorArea
                                        id_base="docs-color-area-workbench-compare-a".to_string()
                                        label="Compare A".to_string()
                                        default_value=(0.5, 0.5)
                                        grid_size=11
                                        step=0.1
                                        preview_color="#3b82f6".to_string()
                                    />
                                </div>

                                <div class="docs-stack docs-stack--tight">
                                    <span class="ui-muted">"对照 B（固定禁用）"</span>
                                    <ColorArea
                                        id_base="docs-color-area-workbench-compare-b".to_string()
                                        label="Compare B".to_string()
                                        default_value=(0.25, 0.85)
                                        grid_size=15
                                        step=0.05
                                        is_disabled=true
                                        preview_color="#a78bfa".to_string()
                                    />
                                </div>
                            </div>
                            <span class="ui-muted">
                                "左侧工作台可调，右侧两个固定对照用于状态/样式差异比对。"
                            </span>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_slider() -> AnyView {
    let (hue, set_hue) = signal(220.0_f64);
    let on_hue_change = Callback::new(move |next: f64| set_hue.set(next));

    let (alpha, set_alpha) = signal(64.0_f64);
    let on_alpha_change = Callback::new(move |next: f64| set_alpha.set(next));

    let reduced_motion = ColorSliderMotion::disabled();

    let basic_code = Signal::derive(move || {
        r##"let (hue, set_hue) = signal(220.0_f64);
let on_hue_change = Callback::new(move |next: f64| set_hue.set(next));

<ColorSlider
  id_base="docs-color-slider-hue".to_string()
  channel=ColorSliderChannel::Hue
  label="Hue".to_string()
  value=hue.into()
  on_value_change=on_hue_change
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"let reduced_motion = ColorSliderMotion::disabled();

<ColorSlider
  id_base="docs-color-slider-alpha".to_string()
  channel=ColorSliderChannel::Alpha
  label="Alpha".to_string()
  value=alpha.into()
  on_value_change=on_alpha_change
  disabled=true
/>
<ColorSlider
  id_base="docs-color-slider-custom".to_string()
  channel=ColorSliderChannel::Blue
  label="Brand blue".to_string()
  default_value=172.0
  track_start_color="#0f172a".to_string()
  track_end_color="#38bdf8".to_string()
  motion=reduced_motion
  class_name="docs-color-slider-custom".to_string()
/>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorSlider"
            slug="color-slider"
            group="Forms"
            description="baseline-compatible single-channel color slider with centralized channel/range/value normalization, spring motion integration, and stable slot/data-state contracts."
        >
            <Playground title="Controlled Hue Channel" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorSlider
                        id_base="docs-color-slider-hue".to_string()
                        channel=ColorSliderChannel::Hue
                        label="Hue".to_string()
                        value=hue.into()
                        on_value_change=on_hue_change
                    />
                    <span class="ui-muted">
                        "hue: " {move || format!("{:.0}°", hue.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled Alpha + Custom Track + Reduced Motion" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorSlider
                        id_base="docs-color-slider-alpha".to_string()
                        channel=ColorSliderChannel::Alpha
                        label="Alpha".to_string()
                        value=alpha.into()
                        on_value_change=on_alpha_change
                        disabled=true
                    />
                    <ColorSlider
                        id_base="docs-color-slider-custom".to_string()
                        channel=ColorSliderChannel::Blue
                        label="Brand blue".to_string()
                        default_value=172.0
                        track_start_color="#0f172a".to_string()
                        track_end_color="#38bdf8".to_string()
                        motion=reduced_motion
                        class_name="docs-color-slider-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_wheel() -> AnyView {
    let (value, set_value) = signal(32.0_f64);
    let on_value_change = Callback::new(move |next: f64| set_value.set(next));

    let (disabled_value, set_disabled_value) = signal(248.0_f64);
    let on_disabled_change = Callback::new(move |next: f64| set_disabled_value.set(next));

    let reduced_motion = ColorWheelMotion::disabled();

    let basic_code = Signal::derive(move || {
        r##"let (value, set_value) = signal(32.0_f64);
let on_value_change = Callback::new(move |next: f64| set_value.set(next));

<ColorWheel
  id_base="docs-color-wheel-hue".to_string()
  label="Hue wheel".to_string()
  value=value.into()
  on_value_change=on_value_change
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"let reduced_motion = ColorWheelMotion::disabled();

<ColorWheel
  id_base="docs-color-wheel-disabled".to_string()
  label="Disabled wheel".to_string()
  value=disabled_value.into()
  on_value_change=on_disabled_change
  disabled=true
/>
<ColorWheel
  id_base="docs-color-wheel-custom".to_string()
  label="Brand hue".to_string()
  default_value=282.0
  motion=reduced_motion
  class_name="docs-color-wheel-custom".to_string()
/>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorWheel"
            slug="color-wheel"
            group="Forms"
            description="baseline-compatible hue wheel with centralized value/step/wrap-around normalization, spring-driven thumb motion, and stable slot/data-state contracts."
        >
            <Playground title="Controlled Hue Wheel" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorWheel
                        id_base="docs-color-wheel-hue".to_string()
                        label="Hue wheel".to_string()
                        value=value.into()
                        on_value_change=on_value_change
                    />
                    <span class="ui-muted">
                        "hue: " {move || format!("{:.0}°", value.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Reduced Motion + Custom Class" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorWheel
                        id_base="docs-color-wheel-disabled".to_string()
                        label="Disabled wheel".to_string()
                        value=disabled_value.into()
                        on_value_change=on_disabled_change
                        disabled=true
                    />
                    <ColorWheel
                        id_base="docs-color-wheel-custom".to_string()
                        label="Brand hue".to_string()
                        default_value=282.0
                        motion=reduced_motion
                        class_name="docs-color-wheel-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_picker() -> AnyView {
    let (selected_color, set_selected_color) = signal(Some("#ef4444".to_string()));
    let on_selected_change =
        Callback::new(move |next: Option<String>| set_selected_color.set(next));

    let (open, set_open) = signal(false);
    let on_open_change = Callback::new(move |next: bool| set_open.set(next));

    let (swatches, _set_swatches) = signal(vec![
        ColorSwatchPickerItem::named("#ef4444", "Red"),
        ColorSwatchPickerItem::named("#f59e0b", "Amber"),
        ColorSwatchPickerItem::named("#10b981", "Emerald"),
        ColorSwatchPickerItem::named("#3b82f6", "Blue"),
        ColorSwatchPickerItem::named("#8b5cf6", "Violet"),
    ]);

    let selected_color_signal: Signal<Option<String>> = selected_color.into();
    let open_signal: Signal<bool> = open.into();

    let basic_code = Signal::derive(move || {
        r##"let (selected_color, set_selected_color) = signal(Some("#ef4444".to_string()));
let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));
let (open, set_open) = signal(false);
let on_open_change = Callback::new(move |next: bool| set_open.set(next));

let (swatches, _set_swatches) = signal(vec![
  ColorSwatchPickerItem::named("#ef4444", "Red"),
  ColorSwatchPickerItem::named("#3b82f6", "Blue"),
]);
let selected_color_signal: Signal<Option<String>> = selected_color.into();
let open_signal: Signal<bool> = open.into();

<ColorPicker
  id_base="docs-color-picker-basic".to_string()
  label="Fill".to_string()
  selected_color=selected_color_signal
  on_selected_change=on_selected_change
  open=open_signal
  on_open_change=on_open_change
>
  <ColorSwatchPicker
    swatches=swatches
    selected_color=selected_color_signal
    on_selected_change=on_selected_change
  />
</ColorPicker>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<ColorPicker
  id_base="docs-color-picker-disabled".to_string()
  label="Disabled".to_string()
  default_selected_color="#0ea5e9".to_string()
  disabled=true
  class_name="docs-color-picker-custom".to_string()
>
  <div class="ui-muted">"Disabled picker content"</div>
</ColorPicker>

<ColorPicker
  id_base="docs-color-picker-open".to_string()
  label="Open by default".to_string()
  default_selected_color="#8b5cf6".to_string()
  default_open=true
>
  <div class="docs-stack docs-stack--tight">
    <span class="ui-muted">"Custom content area"</span>
  </div>
</ColorPicker>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorPicker"
            slug="color-picker"
            group="Forms"
            description="baseline-compatible color picker primitive that composes swatch trigger + popover content with controllable color/open state and stable slot/data-state contracts."
        >
            <Playground title="Controlled Color + Controlled Open" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorPicker
                        id_base="docs-color-picker-basic".to_string()
                        label="Fill".to_string()
                        selected_color=selected_color_signal
                        on_selected_change=on_selected_change
                        open=open_signal
                        on_open_change=on_open_change
                    >
                        <ColorSwatchPicker
                            swatches=swatches
                            selected_color=selected_color_signal
                            on_selected_change=on_selected_change
                        />
                    </ColorPicker>

                    <span class="ui-muted">
                        "selected: " {move || selected_color.get().unwrap_or_else(|| "none".to_string())}
                        " · open: " {move || if open.get() { "true" } else { "false" }}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Default Open + Custom Class" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorPicker
                        id_base="docs-color-picker-disabled".to_string()
                        label="Disabled".to_string()
                        default_selected_color="#0ea5e9".to_string()
                        disabled=true
                        class_name="docs-color-picker-custom".to_string()
                    >
                        <div class="ui-muted">"Disabled picker content"</div>
                    </ColorPicker>

                    <ColorPicker
                        id_base="docs-color-picker-open".to_string()
                        label="Open by default".to_string()
                        default_selected_color="#8b5cf6".to_string()
                        default_open=true
                    >
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Custom content area"</span>
                        </div>
                    </ColorPicker>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_thumb() -> AnyView {
    let board_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm); background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);";

    let basic_code = Signal::derive(move || {
        r##"let board_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm);";

<div style=board_style>
  <ColorThumb
    id_base="docs-color-thumb-idle".to_string()
    color="#f59e0b".to_string()
    x_percent=22.0
    y_percent=72.0
  />
  <ColorThumb
    id_base="docs-color-thumb-focused".to_string()
    color="#10b981".to_string()
    focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorThumb
    id_base="docs-color-thumb-dragging".to_string()
    color="#3b82f6".to_string()
    dragging=true
    x_percent=82.0
    y_percent=28.0
  />
</div>"##.to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<div style=board_style>
  <ColorThumb
    id_base="docs-color-thumb-disabled".to_string()
    color="#a78bfa".to_string()
    disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorThumb
    id_base="docs-color-thumb-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    dragging=true
    show_loupe=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-thumb-custom".to_string()
  />
</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorThumb"
            slug="color-thumb"
            group="Forms"
            description="baseline-compatible draggable color thumb primitive with focus/drag/loupe state contracts, sanitized color source handling, and stable slot/data-state markers."
        >
            <Playground title="Focused + Dragging + Position" code_signal=basic_code>
                <div style=board_style>
                    <ColorThumb
                        id_base="docs-color-thumb-idle".to_string()
                        color="#f59e0b".to_string()
                        x_percent=22.0
                        y_percent=72.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-focused".to_string()
                        color="#10b981".to_string()
                        focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-dragging".to_string()
                        color="#3b82f6".to_string()
                        dragging=true
                        x_percent=82.0
                        y_percent=28.0
                    />
                </div>
            </Playground>

            <Playground title="Disabled + Custom Class + Loupe Off" code_signal=states_code>
                <div style=board_style>
                    <ColorThumb
                        id_base="docs-color-thumb-disabled".to_string()
                        color="#a78bfa".to_string()
                        disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        dragging=true
                        show_loupe=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-thumb-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_editor() -> AnyView {
    let (selected_color, set_selected_color) = signal(Some("#4f46e5".to_string()));
    let on_selected_change =
        Callback::new(move |next: Option<String>| set_selected_color.set(next));

    let (format, set_format) = signal(ColorEditorFormat::Hex);
    let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));

    let selected_color_signal: Signal<Option<String>> = selected_color.into();
    let format_signal: Signal<ColorEditorFormat> = format.into();

    let reduced_motion = ColorSliderMotion::disabled();

    let basic_code = Signal::derive(move || {
        r##"let (selected_color, set_selected_color) = signal(Some("#4f46e5".to_string()));
let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));

let (format, set_format) = signal(ColorEditorFormat::Hex);
let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));

let selected_color_signal: Signal<Option<String>> = selected_color.into();
let format_signal: Signal<ColorEditorFormat> = format.into();

<ColorEditor
  id_base="docs-color-editor-basic".to_string()
  label="Color editor".to_string()
  selected_color=selected_color_signal
  on_selected_change=on_selected_change
  format=format_signal
  on_format_change=on_format_change
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"let reduced_motion = ColorSliderMotion::disabled();

<ColorEditor
  id_base="docs-color-editor-disabled".to_string()
  label="Disabled editor".to_string()
  default_selected_color="#0ea5e9".to_string()
  default_format=ColorEditorFormat::Rgb
  hide_alpha_channel=true
  disabled=true
  class_name="docs-color-editor-custom".to_string()
/>

<ColorEditor
  id_base="docs-color-editor-motion".to_string()
  label="Brand editor".to_string()
  default_format=ColorEditorFormat::Hsb
  default_hue=282.0
  default_alpha=64.0
  default_area=(0.46, 0.88)
  motion=reduced_motion
/>"##
            .to_string()
    });

    let workbench_format_options = vec![
        "hex".to_string(),
        "rgb".to_string(),
        "hsl".to_string(),
        "hsb".to_string(),
    ];
    let (workbench_format_index, set_workbench_format_index) = signal(Some(0usize));
    let workbench_format =
        Signal::derive(move || match workbench_format_index.get().unwrap_or(0) {
            1 => ColorEditorFormat::Rgb,
            2 => ColorEditorFormat::Hsl,
            3 => ColorEditorFormat::Hsb,
            _ => ColorEditorFormat::Hex,
        });
    let on_workbench_format_change = Callback::new(move |next: ColorEditorFormat| {
        set_workbench_format_index.set(Some(match next {
            ColorEditorFormat::Hex => 0,
            ColorEditorFormat::Rgb => 1,
            ColorEditorFormat::Hsl => 2,
            ColorEditorFormat::Hsb => 3,
        }))
    });

    let (workbench_selected_color, set_workbench_selected_color) =
        signal(Some("#4f46e5".to_string()));
    let on_workbench_selected_change = Callback::new(move |next: Option<String>| {
        set_workbench_selected_color.set(next);
    });
    let workbench_selected_color_signal: Signal<Option<String>> = workbench_selected_color.into();
    let workbench_format_signal: Signal<ColorEditorFormat> = workbench_format;

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_hide_alpha, set_workbench_hide_alpha) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_reduced_motion, set_workbench_reduced_motion) = signal(false);

    let workbench_code = Signal::derive(move || {
        let format = workbench_format.get();
        let format_literal = match format {
            ColorEditorFormat::Hex => "ColorEditorFormat::Hex",
            ColorEditorFormat::Rgb => "ColorEditorFormat::Rgb",
            ColorEditorFormat::Hsl => "ColorEditorFormat::Hsl",
            ColorEditorFormat::Hsb => "ColorEditorFormat::Hsb",
        };

        let mut lines = vec![
            "let (selected_color, set_selected_color) = signal(Some(\"#4f46e5\".to_string()));"
                .to_string(),
            "let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));"
                .to_string(),
            String::new(),
            format!("let (format, set_format) = signal({format_literal});"),
            "let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));"
                .to_string(),
            String::new(),
            "<ColorEditor".to_string(),
            "  id_base=\"docs-color-editor-workbench\".to_string()".to_string(),
            "  selected_color=selected_color.into()".to_string(),
            "  on_selected_change=on_selected_change".to_string(),
            "  format=format.into()".to_string(),
            "  on_format_change=on_format_change".to_string(),
        ];

        if workbench_custom_label.get() {
            lines.push("  label=\"Brand color workspace\".to_string()".to_string());
        }
        if workbench_disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if workbench_hide_alpha.get() {
            lines.push("  hide_alpha_channel=true".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-color-editor-workbench\".to_string()".to_string());
        }
        if workbench_reduced_motion.get() {
            lines.push("  motion=ColorSliderMotion::disabled()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/color_editor/styles.rs */\n{}",
            ui_components::color_editor::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let format = workbench_format.get();
        let disabled = workbench_disabled.get();
        let hide_alpha = workbench_hide_alpha.get();
        let has_selection = workbench_selected_color.get().is_some();
        let custom_label = workbench_custom_label.get();
        let custom_class = workbench_custom_class.get();
        let reduced_motion = workbench_reduced_motion.get();

        let data_state = if disabled {
            "disabled"
        } else if has_selection {
            "ready"
        } else {
            "empty"
        };
        let data_alpha = if hide_alpha { "hidden" } else { "visible" };
        let data_motion_source = if reduced_motion { "custom" } else { "default" };
        let data_label_source = if custom_label { "custom" } else { "default" };
        let data_class_source = if custom_class { "custom" } else { "default" };

        let mut classes = vec![
            "ui-color-editor".to_string(),
            format.class_name().to_string(),
        ];
        if disabled {
            classes.push("ui-color-editor--disabled".to_string());
        }
        if hide_alpha {
            classes.push("ui-color-editor--alpha-hidden".to_string());
        }
        if custom_class {
            classes.push("ui-color-editor--custom-class".to_string());
            classes.push("docs-color-editor-workbench".to_string());
        }

        format!(
            "ColorEditorActualConfig {{\n  format: {format:?},\n  disabled: {disabled},\n  hide_alpha_channel: {hide_alpha},\n  has_selection: {has_selection},\n  custom_label: {custom_label},\n  custom_class: {custom_class},\n  reduced_motion: {reduced_motion},\n  data_state: \"{data_state}\",\n  data_alpha: \"{data_alpha}\",\n  data_motion_source: \"{data_motion_source}\",\n  data_label_source: \"{data_label_source}\",\n  data_class_source: \"{data_class_source}\",\n  class: \"{}\",\n}}",
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="ColorEditor"
            slug="color-editor"
            group="Forms"
            description="baseline-compatible color editor primitive that composes color area + sliders + field + format switching with controllable color/format state and stable slot/data-state contracts."
        >
            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/color_editor/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Format"</div>
                        <SegmentedControl
                            id_base="docs-color-editor-workbench-format".to_string()
                            options=workbench_format_options.clone()
                            selected_index=workbench_format_index
                            set_selected_index=set_workbench_format_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorEditor workbench format".to_string()
                        />

                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_hide_alpha set_checked=set_workbench_hide_alpha>
                            "Hide alpha channel"
                        </Switch>
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "Custom label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let reduced_motion = if workbench_reduced_motion.get() {
                        ColorSliderMotion::disabled()
                    } else {
                        ColorSliderMotion::default()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-row" style="align-items: flex-start;">
                                <div class="docs-card" style="flex: 1 1 28rem; min-width: 20rem;">
                                    <div class="ui-muted">"Workbench"</div>
                                    <ColorEditor
                                        id_base="docs-color-editor-workbench".to_string()
                                        label=if workbench_custom_label.get() {
                                            "Brand color workspace".to_string()
                                        } else {
                                            "Color editor".to_string()
                                        }
                                        selected_color=workbench_selected_color_signal
                                        on_selected_change=on_workbench_selected_change
                                        format=workbench_format_signal
                                        on_format_change=on_workbench_format_change
                                        hide_alpha_channel=workbench_hide_alpha.get()
                                        disabled=workbench_disabled.get()
                                        class_name=if workbench_custom_class.get() {
                                            "docs-color-editor-workbench".to_string()
                                        } else {
                                            String::new()
                                        }
                                        motion=reduced_motion
                                    />
                                </div>

                                <div class="docs-card" style="flex: 1 1 28rem; min-width: 20rem;">
                                    <div class="ui-muted">"Comparison (Disabled + Alpha Hidden)"</div>
                                    <ColorEditor
                                        id_base="docs-color-editor-workbench-compare".to_string()
                                        label="Comparison".to_string()
                                        default_selected_color="#0ea5e9".to_string()
                                        default_format=ColorEditorFormat::Rgb
                                        hide_alpha_channel=true
                                        disabled=true
                                        motion=ColorSliderMotion::disabled()
                                    />
                                </div>
                            </div>

                            <span class="ui-muted">
                                "value: "
                                {move || {
                                    workbench_selected_color
                                        .get()
                                        .unwrap_or_else(|| "none".to_string())
                                }}
                                " · format: "
                                {move || workbench_format.get().as_attr()}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Controlled Color + Controlled Format" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorEditor
                        id_base="docs-color-editor-basic".to_string()
                        label="Color editor".to_string()
                        selected_color=selected_color_signal
                        on_selected_change=on_selected_change
                        format=format_signal
                        on_format_change=on_format_change
                    />

                    <span class="ui-muted">
                        "value: "
                        {move || selected_color.get().unwrap_or_else(|| "none".to_string())}
                        " · format: "
                        {move || format.get().as_attr()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Alpha Hidden + Reduced Motion" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorEditor
                        id_base="docs-color-editor-disabled".to_string()
                        label="Disabled editor".to_string()
                        default_selected_color="#0ea5e9".to_string()
                        default_format=ColorEditorFormat::Rgb
                        hide_alpha_channel=true
                        disabled=true
                        class_name="docs-color-editor-custom".to_string()
                    />

                    <ColorEditor
                        id_base="docs-color-editor-motion".to_string()
                        label="Brand editor".to_string()
                        default_format=ColorEditorFormat::Hsb
                        default_hue=282.0
                        default_alpha=64.0
                        default_area=(0.46, 0.88)
                        motion=reduced_motion
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_handle() -> AnyView {
    let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

    let basic_code = Signal::derive(move || {
        r##"let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

<div style=surface_style>
  <ColorHandle
    id_base="docs-color-handle-idle".to_string()
    color="#f59e0b".to_string()
    x_percent=22.0
    y_percent=72.0
  />
  <ColorHandle
    id_base="docs-color-handle-focused".to_string()
    color="#10b981".to_string()
    focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorHandle
    id_base="docs-color-handle-dragging".to_string()
    color="#3b82f6".to_string()
    dragging=true
    x_percent=82.0
    y_percent=28.0
  />
</div>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<div style=surface_style>
  <ColorHandle
    id_base="docs-color-handle-disabled".to_string()
    color="#a78bfa".to_string()
    disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorHandle
    id_base="docs-color-handle-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    dragging=true
    show_loupe=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-handle-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let (workbench_color_key, set_workbench_color_key) = signal("amber".to_string());
    let (workbench_x_percent, set_workbench_x_percent) = signal(68_f32);
    let (workbench_y_percent, set_workbench_y_percent) = signal(36_f32);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_focused, set_workbench_focused) = signal(true);
    let (workbench_dragging, set_workbench_dragging) = signal(false);
    let (workbench_show_loupe, set_workbench_show_loupe) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_motion_ms, set_workbench_motion_ms) = signal(180_u16);

    let workbench_code = Signal::derive(move || {
        let color = match workbench_color_key.get().as_str() {
            "mint" => "#10b981",
            "sky" => "#0ea5e9",
            "violet" => "#8b5cf6",
            _ => "#f59e0b",
        };
        let class_name_line = if workbench_custom_class.get() {
            "  class_name=\"docs-color-handle-custom\".to_string()\n".to_string()
        } else {
            String::new()
        };
        format!(
            "<ColorHandle\n  id_base=\"docs-color-handle-workbench\".to_string()\n  color=\"{color}\".to_string()\n  x_percent={:.1}\n  y_percent={:.1}\n  disabled={}\n  focused={}\n  dragging={}\n  show_loupe={}\n  motion=ColorHandleMotion {{ duration_ms: {} }}\n{class_name_line}/>",
            workbench_x_percent.get(),
            workbench_y_percent.get(),
            workbench_disabled.get(),
            workbench_focused.get(),
            workbench_dragging.get(),
            workbench_show_loupe.get(),
            workbench_motion_ms.get(),
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/color_handle/styles.rs */\n{}",
            ui_components::color_handle::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let disabled = workbench_disabled.get();
        let focused = workbench_focused.get();
        let dragging = workbench_dragging.get();
        let show_loupe = workbench_show_loupe.get();
        let has_custom_class = workbench_custom_class.get();
        let state = if disabled {
            "disabled"
        } else if dragging {
            "dragging"
        } else if focused {
            "focused"
        } else {
            "color"
        };
        let loupe_visible = !disabled && show_loupe && dragging;
        let mut classes = vec!["ui-color-handle".to_string()];
        if disabled {
            classes.push("ui-color-handle--disabled".to_string());
        }
        if focused {
            classes.push("ui-color-handle--focused".to_string());
        }
        if dragging {
            classes.push("ui-color-handle--dragging".to_string());
        }
        if has_custom_class {
            classes.push("ui-color-handle--custom-class".to_string());
            classes.push("docs-color-handle-custom".to_string());
        }
        format!(
            "ColorHandleActualConfig {{\n  state: \"{state}\",\n  x_percent: {:.1},\n  y_percent: {:.1},\n  disabled: {disabled},\n  focused: {focused},\n  dragging: {dragging},\n  show_loupe: {show_loupe},\n  loupe_visible: {loupe_visible},\n  motion_duration_ms: {},\n  class: \"{}\",\n}}",
            workbench_x_percent.get(),
            workbench_y_percent.get(),
            workbench_motion_ms.get(),
            classes.join(" "),
        )
    });

    view! {
        <ComponentPage
            title="ColorHandle"
            slug="color-handle"
            group="Forms"
            description="baseline-compatible draggable color handle primitive with composed thumb/loupe behavior, centralized state derivation, and stable slot/data-state contracts."
        >
            <Playground title="Focused + Dragging + Position" code_signal=basic_code>
                <div style=surface_style>
                    <ColorHandle
                        id_base="docs-color-handle-idle".to_string()
                        color="#f59e0b".to_string()
                        x_percent=22.0
                        y_percent=72.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-focused".to_string()
                        color="#10b981".to_string()
                        focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-dragging".to_string()
                        color="#3b82f6".to_string()
                        dragging=true
                        x_percent=82.0
                        y_percent=28.0
                    />
                </div>
            </Playground>

            <Playground title="Disabled + Custom Class + Loupe Off" code_signal=states_code>
                <div style=surface_style>
                    <ColorHandle
                        id_base="docs-color-handle-disabled".to_string()
                        color="#a78bfa".to_string()
                        disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        dragging=true
                        show_loupe=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-handle-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with interactive config, copy-ready code, and scoped CSS test panel."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/color_handle/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-handle-workbench-controls">
                        <label class="docs-search__label">
                            "Color"
                            <select
                                prop:value=move || workbench_color_key.get()
                                on:change=move |ev| set_workbench_color_key.set(event_target_value(&ev))
                            >
                                <option value="amber">"Amber"</option>
                                <option value="mint">"Mint"</option>
                                <option value="sky">"Sky"</option>
                                <option value="violet">"Violet"</option>
                            </select>
                        </label>

                        <label class="docs-search__label">
                            "X position "
                            <input
                                type="range"
                                min="0"
                                max="100"
                                prop:value=move || format!("{:.0}", workbench_x_percent.get())
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<f32>() {
                                        set_workbench_x_percent.set(next.clamp(0.0, 100.0));
                                    }
                                }
                            />
                        </label>

                        <label class="docs-search__label">
                            "Y position "
                            <input
                                type="range"
                                min="0"
                                max="100"
                                prop:value=move || format!("{:.0}", workbench_y_percent.get())
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<f32>() {
                                        set_workbench_y_percent.set(next.clamp(0.0, 100.0));
                                    }
                                }
                            />
                        </label>

                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_focused.get()
                                on:change=move |ev| set_workbench_focused.set(event_target_checked(&ev))
                            />
                            " Focused"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_dragging.get()
                                on:change=move |ev| set_workbench_dragging.set(event_target_checked(&ev))
                            />
                            " Dragging"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_show_loupe.get()
                                on:change=move |ev| set_workbench_show_loupe.set(event_target_checked(&ev))
                            />
                            " Show loupe"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            "Motion (ms) "
                            <input
                                type="range"
                                min="60"
                                max="600"
                                prop:value=move || workbench_motion_ms.get().to_string()
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<u16>() {
                                        set_workbench_motion_ms.set(next.clamp(60, 600));
                                    }
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-handle-workbench">
                    <span class="ui-muted">
                        "display: baseline vs configured"
                    </span>
                    <div class="docs-row">
                        <div class="docs-card">
                            <div class="ui-muted">"Baseline"</div>
                            <div style=surface_style>
                                <ColorHandle
                                    id_base="docs-color-handle-baseline".to_string()
                                    color="#f59e0b".to_string()
                                    x_percent=44.0
                                    y_percent=56.0
                                />
                            </div>
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"Configured"</div>
                            <div style=surface_style>
                                {move || {
                                    let color = match workbench_color_key.get().as_str() {
                                        "mint" => "#10b981",
                                        "sky" => "#0ea5e9",
                                        "violet" => "#8b5cf6",
                                        _ => "#f59e0b",
                                    };
                                    let class_name = if workbench_custom_class.get() {
                                        "docs-color-handle-custom".to_string()
                                    } else {
                                        String::new()
                                    };
                                    let motion = ColorHandleMotion {
                                        duration_ms: workbench_motion_ms.get(),
                                    };
                                    view! {
                                        <ColorHandle
                                            id_base="docs-color-handle-workbench".to_string()
                                            color=color.to_string()
                                            disabled=workbench_disabled.get()
                                            focused=workbench_focused.get()
                                            dragging=workbench_dragging.get()
                                            show_loupe=workbench_show_loupe.get()
                                            x_percent=workbench_x_percent.get()
                                            y_percent=workbench_y_percent.get()
                                            class_name=class_name
                                            motion=motion
                                        />
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_loupe() -> AnyView {
    let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm); background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);";
    let color_options = vec![
        "Amber".to_string(),
        "Emerald".to_string(),
        "Sky".to_string(),
        "Alpha".to_string(),
    ];
    let position_options = vec!["Start".to_string(), "Center".to_string(), "End".to_string()];
    let (color_index, set_color_index) = signal(Some(0_usize));
    let (position_index, set_position_index) = signal(Some(1_usize));
    let (is_open, set_is_open) = signal(true);
    let (is_disabled, set_is_disabled) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);

    let basic_code = Signal::derive(move || {
        r##"let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm);";

<div style=surface_style>
  <ColorLoupe
    id_base="docs-color-loupe-start".to_string()
    color="#f59e0b".to_string()
    open=true
    x_percent=18.0
    y_percent=74.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-center".to_string()
    color="#10b981".to_string()
    open=true
    x_percent=50.0
    y_percent=48.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-end".to_string()
    color="#3b82f6".to_string()
    open=true
    x_percent=82.0
    y_percent=24.0
  />
</div>"##.to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<div style=surface_style>
  <ColorLoupe
    id_base="docs-color-loupe-disabled".to_string()
    color="#a78bfa".to_string()
    open=true
    disabled=true
    x_percent=32.0
    y_percent=58.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    open=true
    x_percent=72.0
    y_percent=36.0
    aria_label="Accent loupe".to_string()
    class_name="docs-color-loupe-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let (color, color_label) = match color_index.get().unwrap_or(0) {
            0 => ("#f59e0b", "amber"),
            1 => ("#10b981", "emerald"),
            2 => ("#3b82f6", "sky"),
            _ => ("rgba(56, 189, 248, 0.72)", "alpha"),
        };
        let (x_percent, y_percent, position_label) = match position_index.get().unwrap_or(0) {
            0 => (18.0_f32, 74.0_f32, "start"),
            1 => (50.0_f32, 48.0_f32, "center"),
            _ => (82.0_f32, 24.0_f32, "end"),
        };
        let open = is_open.get();
        let disabled = is_disabled.get();
        let aria_label = if custom_aria.get() {
            "Workbench loupe"
        } else {
            ""
        };
        let class_name = if custom_class.get() {
            "docs-color-loupe-workbench"
        } else {
            ""
        };

        format!(
            "<ColorLoupe\n  id_base=\"docs-color-loupe-workbench\".to_string()\n  color=\"{color}\".to_string() // {color_label}\n  open={open}\n  disabled={disabled}\n  x_percent={x_percent}\n  y_percent={y_percent} // {position_label}\n  aria_label=\"{aria_label}\".to_string()\n  class_name=\"{class_name}\".to_string()\n/>"
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let (color, color_label) = match color_index.get().unwrap_or(0) {
            0 => ("#f59e0b", "amber"),
            1 => ("#10b981", "emerald"),
            2 => ("#3b82f6", "sky"),
            _ => ("rgba(56, 189, 248, 0.72)", "alpha"),
        };
        let (x_percent, y_percent, position_label) = match position_index.get().unwrap_or(0) {
            0 => (18.0_f32, 74.0_f32, "start"),
            1 => (50.0_f32, 48.0_f32, "center"),
            _ => (82.0_f32, 24.0_f32, "end"),
        };
        let open = is_open.get();
        let disabled = is_disabled.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let aria_label = if custom_aria { "Workbench loupe" } else { "" };
        let class_name = if custom_class {
            "docs-color-loupe-workbench"
        } else {
            ""
        };

        format!(
            "ColorLoupeActualConfig {{\n  color: \"{color}\" ({color_label}),\n  position: \"{position_label}\" ({x_percent:.1}, {y_percent:.1}),\n  open: {open},\n  disabled: {disabled},\n  aria_label: \"{aria_label}\",\n  class_name: \"{class_name}\",\n}}"
        )
    });

    view! {
        <ComponentPage
            title="ColorLoupe"
            slug="color-loupe"
            group="Forms"
            description="baseline-compatible color loupe overlay primitive with centralized open/disabled/position normalization, checkerboard alpha preview, and stable slot/data-state contracts."
        >
            <Playground title="Open + Position Buckets" code_signal=basic_code>
                <div style=surface_style>
                    <ColorLoupe
                        id_base="docs-color-loupe-start".to_string()
                        color="#f59e0b".to_string()
                        open=true
                        x_percent=18.0
                        y_percent=74.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-center".to_string()
                        color="#10b981".to_string()
                        open=true
                        x_percent=50.0
                        y_percent=48.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-end".to_string()
                        color="#3b82f6".to_string()
                        open=true
                        x_percent=82.0
                        y_percent=24.0
                    />
                </div>
            </Playground>

            <Playground title="Disabled + Custom Label + Custom Class" code_signal=states_code>
                <div style=surface_style>
                    <ColorLoupe
                        id_base="docs-color-loupe-disabled".to_string()
                        color="#a78bfa".to_string()
                        open=true
                        disabled=true
                        x_percent=32.0
                        y_percent=58.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        open=true
                        x_percent=72.0
                        y_percent=36.0
                        aria_label="Accent loupe".to_string()
                        class_name="docs-color-loupe-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_source_path="crates/ui-components/src/color_loupe/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Color"</div>
                        <SegmentedControl
                            id_base="docs-color-loupe-workbench-color".to_string()
                            options=color_options.clone()
                            selected_index=color_index
                            set_selected_index=set_color_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorLoupe workbench color".to_string()
                        />

                        <div class="docs-search__label">"Position bucket"</div>
                        <SegmentedControl
                            id_base="docs-color-loupe-workbench-position".to_string()
                            options=position_options.clone()
                            selected_index=position_index
                            set_selected_index=set_position_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorLoupe workbench position".to_string()
                        />

                        <Switch checked=is_open set_checked=set_is_open>"Open"</Switch>
                        <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>
                            "Custom aria_label"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let color = match color_index.get().unwrap_or(0) {
                        0 => "#f59e0b".to_string(),
                        1 => "#10b981".to_string(),
                        2 => "#3b82f6".to_string(),
                        _ => "rgba(56, 189, 248, 0.72)".to_string(),
                    };
                    let (x_percent, y_percent) = match position_index.get().unwrap_or(0) {
                        0 => (18.0_f32, 74.0_f32),
                        1 => (50.0_f32, 48.0_f32),
                        _ => (82.0_f32, 24.0_f32),
                    };
                    let open = is_open.get();
                    let disabled = is_disabled.get();
                    let aria_label = if custom_aria.get() {
                        "Workbench loupe".to_string()
                    } else {
                        "".to_string()
                    };
                    let class_name = if custom_class.get() {
                        "docs-color-loupe-workbench".to_string()
                    } else {
                        "".to_string()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div style=surface_style>
                                <ColorLoupe
                                    id_base="docs-color-loupe-workbench-main".to_string()
                                    color=color
                                    open=open
                                    disabled=disabled
                                    x_percent=x_percent
                                    y_percent=y_percent
                                    aria_label=aria_label
                                    class_name=class_name
                                />
                                <ColorLoupe
                                    id_base="docs-color-loupe-workbench-compare".to_string()
                                    color="#3b82f6".to_string()
                                    open=true
                                    x_percent=82.0
                                    y_percent=24.0
                                    aria_label="Comparison loupe".to_string()
                                />
                            </div>
                            <span class="ui-muted">
                                "左侧可调，右侧固定对照（blue/end/open）。"
                            </span>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
