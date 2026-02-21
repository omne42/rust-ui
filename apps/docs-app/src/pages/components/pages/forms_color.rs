use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::color::handle::ColorHandleMotion;
use ui_components::color::loupe::ColorLoupeOutputState;
use ui_components::{
    ColorArea, ColorEditor, ColorEditorFormat, ColorField, ColorHandle, ColorLoupe, ColorPicker,
    ColorSlider, ColorSliderChannel, ColorSliderMotion, ColorSwatchPicker, ColorSwatchPickerItem,
    ColorThumb, ColorWheel, ColorWheelMotion, SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn color_field() -> AnyView {
    let (value, set_value) = signal(Some("#4f46e5".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));
    let (compare_value, set_compare_value) = signal(Some("#22c55e".to_string()));
    let on_compare_value_change =
        Callback::new(move |next: Option<String>| set_compare_value.set(next));

    let hello_code = Signal::derive(move || {
        r##"<ColorField
  id_base="docs-color-field-hello".to_string()
/>"##
            .to_string()
    });

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

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"let (controlled, set_controlled) = signal(Some("#22c55e".to_string()));
let on_controlled_change = Callback::new(move |next: Option<String>| set_controlled.set(next));

<ColorField
  id_base="docs-color-field-compare-controlled".to_string()
  label="Controlled".to_string()
  value=controlled.into()
  on_value_change=on_controlled_change
/>
<ColorField
  id_base="docs-color-field-compare-uncontrolled".to_string()
  label="Uncontrolled".to_string()
  default_value="#0ea5e9".to_string()
/>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorField is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "ColorField docs output mode: snapshot"
</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorField"
            slug="color-field"
            group="Forms"
            description="baseline-compatible color text field with centralized label/placeholder/aria/state normalization, sanitized preview rendering, and stable slot/data contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ColorField id_base="docs-color-field-hello".to_string() />
            </Playground>

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

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=controlled_vs_uncontrolled_code
            >
                <div class="docs-stack docs-stack--tight">
                    <ColorField
                        id_base="docs-color-field-compare-controlled".to_string()
                        label="Controlled".to_string()
                        value=compare_value.into()
                        on_value_change=on_compare_value_change
                    />
                    <ColorField
                        id_base="docs-color-field-compare-uncontrolled".to_string()
                        label="Uncontrolled".to_string()
                        default_value="#0ea5e9".to_string()
                    />
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
                        is_disabled=true
                    />
                </div>
            </Playground>

            <Playground title="Streaming Optional / Snapshot" code_signal=output_mode_code>
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-field-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorField is an input surface; docs-mode output is snapshot (`fallback=snapshot`)."
                    </span>
                    <ColorField
                        id_base="docs-color-field-snapshot".to_string()
                        label="Snapshot fallback".to_string()
                        default_value="#334155".to_string()
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
    let (compare_value, set_compare_value) = signal((0.35_f32, 0.65_f32));
    let on_compare_value_change =
        Callback::new(move |next: (f32, f32)| set_compare_value.set(next));
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

    let hello_code = Signal::derive(move || {
        r##"<ColorArea
  id_base="docs-color-area-hello".to_string()
/>"##
            .to_string()
    });

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
  is_disabled=true
  class_name="docs-color-area-custom".to_string()
/>"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<ColorArea
  id_base="docs-color-area-matrix-default".to_string()
  label="Default".to_string()
/>
<ColorArea
  id_base="docs-color-area-matrix-custom-grid".to_string()
  label="Dense grid".to_string()
  default_value=(0.25, 0.85)
  grid_size=15
  step=0.05
  preview_color="#a78bfa".to_string()
/>
<ColorArea
  id_base="docs-color-area-matrix-disabled".to_string()
  label="Disabled".to_string()
  is_disabled=true
  class_name="docs-color-area-custom".to_string()
/>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"let (controlled, set_controlled) = signal((0.35_f32, 0.65_f32));
let on_controlled_change = Callback::new(move |next: (f32, f32)| set_controlled.set(next));

<ColorArea
  id_base="docs-color-area-compare-controlled".to_string()
  label="Controlled".to_string()
  value=controlled.into()
  on_value_change=on_controlled_change
  preview_color="#22c55e".to_string()
/>
<ColorArea
  id_base="docs-color-area-compare-uncontrolled".to_string()
  label="Uncontrolled".to_string()
  default_value=(0.25, 0.85)
  preview_color="#0ea5e9".to_string()
/>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorArea is an input surface, not a long-form reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "ColorArea docs output mode: snapshot"
</div>
<ColorArea
  id_base="docs-color-area-snapshot".to_string()
  default_value=(0.6, 0.4)
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
            "<ColorArea\n  id_base=\"docs-color-area-workbench\".into()\n  label=\"Color workbench\".into()\n  default_value=({:.2}, {:.2})\n  grid_size={}\n  step={:.2}\n  is_disabled={}\n  preview_color=\"{}\".into()\n  x_axis_label=\"{}\".into()\n  y_axis_label=\"{}\".into()\n  class_name=\"{}\".into()\n/>",
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
            "/* components/color-area/src/styles.rs */\n{}",
            ui_components::color::area::styles::CSS
        )
    });

    view! {
        <ComponentPage
            title="ColorArea"
            slug="color-area"
            group="Forms"
            description="baseline-compatible two-axis color selection primitive with centralized step/grid normalization, keyboard navigation, and stable slot/data-state contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ColorArea id_base="docs-color-area-hello".to_string() />
            </Playground>

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

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=controlled_vs_uncontrolled_code
            >
                <div class="docs-row docs-row--wrap" style="gap: var(--ui-space-lg); align-items: flex-start;">
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Controlled"</span>
                        <ColorArea
                            id_base="docs-color-area-compare-controlled".to_string()
                            label="Controlled".to_string()
                            value=compare_value.into()
                            on_value_change=on_compare_value_change
                            preview_color="#22c55e".to_string()
                        />
                        <span class="ui-muted">
                            "value: "
                            {move || {
                                let (x, y) = compare_value.get();
                                format!("({:.2}, {:.2})", x, y)
                            }}
                        </span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Uncontrolled"</span>
                        <ColorArea
                            id_base="docs-color-area-compare-uncontrolled".to_string()
                            label="Uncontrolled".to_string()
                            default_value=(0.25, 0.85)
                            preview_color="#0ea5e9".to_string()
                        />
                    </div>
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

            <Playground title="State Matrix" code_signal=state_matrix_code>
                <div class="docs-row docs-row--wrap" style="gap: var(--ui-space-lg); align-items: flex-start;">
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Default"</span>
                        <ColorArea
                            id_base="docs-color-area-matrix-default".to_string()
                            label="Default".to_string()
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Dense grid + preview"</span>
                        <ColorArea
                            id_base="docs-color-area-matrix-custom-grid".to_string()
                            label="Dense grid".to_string()
                            default_value=(0.25, 0.85)
                            grid_size=15
                            step=0.05
                            preview_color="#a78bfa".to_string()
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Disabled"</span>
                        <ColorArea
                            id_base="docs-color-area-matrix-disabled".to_string()
                            label="Disabled".to_string()
                            is_disabled=true
                            class_name="docs-color-area-custom".to_string()
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Streaming Optional / Snapshot" code_signal=output_mode_code>
                <div
                    class="docs-stack docs-stack--tight"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <p class="ui-muted">
                        "ColorArea is an input surface; docs output mode remains snapshot (`fallback=snapshot`)."
                    </p>
                    <ColorArea id_base="docs-color-area-snapshot".to_string() default_value=(0.6, 0.4) />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含可调主样例 + 固定对照样例）。"
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="components/color-area/src/styles.rs".to_string()
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

            <section class="docs-card docs-prose" data-slot="color-area-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground code panel supports one-click copy, and copied snippets auto-inject missing imports via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p>
                    "Default import baseline: "
                    <code>"use leptos::prelude::*; use ui_components::*;"</code>
                    " (overridable per-playground with "
                    <code>"code_imports"</code>
                    ")."
                </p>
                <p>
                    "Source-first path: "
                    <code>"components/color-area/src/view.rs"</code>
                    ", styles: "
                    <code>"components/color-area/src/styles.rs"</code>
                    "."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_slider() -> AnyView {
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct ColorSliderWorkbenchState {
        channel_index: usize,
        value: f64,
        is_disabled: bool,
        has_custom_track: bool,
        has_custom_class: bool,
        reduced_motion: bool,
        preserve_context: bool,
    }

    impl Default for ColorSliderWorkbenchState {
        fn default() -> Self {
            Self {
                channel_index: 0,
                value: 220.0,
                is_disabled: false,
                has_custom_track: false,
                has_custom_class: false,
                reduced_motion: false,
                preserve_context: true,
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    impl ColorSliderWorkbenchState {
        fn parse(raw: &str) -> Option<Self> {
            let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
            if parts.len() != 7 {
                return None;
            }

            let parse_bool = |at: usize| match *parts.get(at)? {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            };

            Some(Self {
                channel_index: parts.get(0)?.parse::<usize>().ok()?.min(3),
                value: parts.get(1)?.parse::<f64>().ok()?,
                is_disabled: parse_bool(2)?,
                has_custom_track: parse_bool(3)?,
                has_custom_class: parse_bool(4)?,
                reduced_motion: parse_bool(5)?,
                preserve_context: parse_bool(6)?,
            })
        }

        fn encode(self) -> String {
            let bool_digit = |value: bool| if value { '1' } else { '0' };
            format!(
                "{},{:.4},{},{},{},{},{}",
                self.channel_index.min(3),
                self.value,
                bool_digit(self.is_disabled),
                bool_digit(self.has_custom_track),
                bool_digit(self.has_custom_class),
                bool_digit(self.reduced_motion),
                bool_digit(self.preserve_context),
            )
        }
    }

    #[cfg(target_arch = "wasm32")]
    const COLOR_SLIDER_WORKBENCH_STORAGE_KEY: &str = "docs:color-slider:workbench:state";

    #[cfg(target_arch = "wasm32")]
    fn load_color_slider_workbench_state() -> Option<ColorSliderWorkbenchState> {
        let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
        let raw = storage
            .get_item(COLOR_SLIDER_WORKBENCH_STORAGE_KEY)
            .ok()
            .flatten()?;
        ColorSliderWorkbenchState::parse(&raw)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_color_slider_workbench_state() -> Option<ColorSliderWorkbenchState> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    fn save_color_slider_workbench_state(state: ColorSliderWorkbenchState) {
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            drop(storage.set_item(COLOR_SLIDER_WORKBENCH_STORAGE_KEY, &state.encode()));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_color_slider_workbench_state(_state: ColorSliderWorkbenchState) {}

    #[cfg(target_arch = "wasm32")]
    fn clear_color_slider_workbench_state() {
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            drop(storage.remove_item(COLOR_SLIDER_WORKBENCH_STORAGE_KEY));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn clear_color_slider_workbench_state() {}

    let (hue, set_hue) = signal(220.0_f64);
    let on_hue_change = Callback::new(move |next: f64| set_hue.set(next));

    let (alpha, set_alpha) = signal(64.0_f64);
    let on_alpha_change = Callback::new(move |next: f64| set_alpha.set(next));

    let persisted_workbench_state = load_color_slider_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();

    let workbench_channel_options = vec![
        "Hue".to_string(),
        "Saturation".to_string(),
        "Lightness".to_string(),
        "Alpha".to_string(),
    ];
    let (workbench_channel_index, set_workbench_channel_index) =
        signal(Some(initial_workbench_state.channel_index.min(3)));
    let workbench_channel =
        Signal::derive(
            move || match workbench_channel_index.get().unwrap_or(0).min(3) {
                1 => ColorSliderChannel::Saturation,
                2 => ColorSliderChannel::Lightness,
                3 => ColorSliderChannel::Alpha,
                _ => ColorSliderChannel::Hue,
            },
        );
    let (workbench_value, set_workbench_value) = signal(initial_workbench_state.value);
    let on_workbench_value_change = Callback::new(move |next: f64| set_workbench_value.set(next));
    let workbench_value_signal: Signal<f64> = workbench_value.into();

    let (workbench_disabled, set_workbench_disabled) = signal(initial_workbench_state.is_disabled);
    let (workbench_custom_track, set_workbench_custom_track) =
        signal(initial_workbench_state.has_custom_track);
    let (workbench_custom_class, set_workbench_custom_class) =
        signal(initial_workbench_state.has_custom_class);
    let (workbench_reduced_motion, set_workbench_reduced_motion) =
        signal(initial_workbench_state.reduced_motion);
    let (workbench_preserve_context, set_workbench_preserve_context) =
        signal(initial_workbench_state.preserve_context);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    let last_workbench_channel_index =
        RwSignal::new(workbench_channel_index.get_untracked().unwrap_or(0).min(3));
    let reset_workbench_value = set_workbench_value;
    let reset_workbench_disabled = set_workbench_disabled;
    let reset_workbench_custom_track = set_workbench_custom_track;
    let reset_workbench_custom_class = set_workbench_custom_class;
    let reset_workbench_reduced_motion = set_workbench_reduced_motion;
    Effect::new(move |_| {
        let next_channel_index = workbench_channel_index.get().unwrap_or(0).min(3);
        let prev_channel_index = last_workbench_channel_index.get_untracked();
        if next_channel_index == prev_channel_index {
            return;
        }

        last_workbench_channel_index.set(next_channel_index);
        if !workbench_preserve_context.get() {
            let next_channel = match next_channel_index {
                1 => ColorSliderChannel::Saturation,
                2 => ColorSliderChannel::Lightness,
                3 => ColorSliderChannel::Alpha,
                _ => ColorSliderChannel::Hue,
            };
            reset_workbench_value.set(next_channel.default_value());
            reset_workbench_disabled.set(false);
            reset_workbench_custom_track.set(false);
            reset_workbench_custom_class.set(false);
            reset_workbench_reduced_motion.set(false);
        }
    });

    Effect::new(move |_| {
        let state = ColorSliderWorkbenchState {
            channel_index: workbench_channel_index.get().unwrap_or(0).min(3),
            value: workbench_value.get(),
            is_disabled: workbench_disabled.get(),
            has_custom_track: workbench_custom_track.get(),
            has_custom_class: workbench_custom_class.get(),
            reduced_motion: workbench_reduced_motion.get(),
            preserve_context: workbench_preserve_context.get(),
        };

        if workbench_persist_state.get() {
            save_color_slider_workbench_state(state);
        } else {
            clear_color_slider_workbench_state();
        }
    });

    let workbench_code = Signal::derive(move || {
        let channel = workbench_channel.get();
        let channel_literal = match channel {
            ColorSliderChannel::Hue => "ColorSliderChannel::Hue",
            ColorSliderChannel::Saturation => "ColorSliderChannel::Saturation",
            ColorSliderChannel::Lightness => "ColorSliderChannel::Lightness",
            ColorSliderChannel::Alpha => "ColorSliderChannel::Alpha",
            _ => "ColorSliderChannel::Hue",
        };

        let mut lines = vec![
            format!(
                "let (value, set_value) = signal({:.1}_f64);",
                workbench_value.get()
            ),
            "let on_value_change = Callback::new(move |next: f64| set_value.set(next));"
                .to_string(),
            String::new(),
            "<ColorSlider".to_string(),
            "  id_base=\"docs-color-slider-workbench\".into()".to_string(),
            format!("  channel={channel_literal}"),
            format!("  label=\"{}\".into()", channel.default_label()),
            "  value=value.into()".to_string(),
            "  on_value_change=on_value_change".to_string(),
            format!("  disabled={}", workbench_disabled.get()),
        ];

        if workbench_custom_track.get() {
            lines.push("  track_start_color=\"#0f172a\".into()".to_string());
            lines.push("  track_end_color=\"#38bdf8\".into()".to_string());
        }
        if workbench_reduced_motion.get() {
            lines.push("  motion=ColorSliderMotion::disabled()".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-color-slider-workbench\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/color-slider/src/styles.rs */\n{}\n\n/* apps/docs-app/dev-overrides.css */\n{}",
            ui_components::color::slider::styles::CSS,
            include_str!("../../../../dev-overrides.css"),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let channel = workbench_channel.get();
        let value = workbench_value.get();
        let is_disabled = workbench_disabled.get();
        let custom_track = workbench_custom_track.get();
        let custom_class = workbench_custom_class.get();
        let reduced_motion = workbench_reduced_motion.get();
        let preserve_context = workbench_preserve_context.get();
        let persist_state = workbench_persist_state.get();
        format!(
            "ColorSliderWorkbenchConfig {{\n  channel: {channel:?},\n  value: {value:.2},\n  is_disabled: {is_disabled},\n  custom_track: {custom_track},\n  custom_class: {custom_class},\n  reduced_motion: {reduced_motion},\n  preserve_context: {preserve_context},\n  persist_state: {persist_state},\n}}"
        )
    });

    let reduced_motion = ColorSliderMotion::disabled();

    let hello_code = Signal::derive(move || {
        r##"<ColorSlider
  id_base="docs-color-slider-hello".to_string()
/>"##
            .to_string()
    });

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

    let state_matrix_code = Signal::derive(move || {
        r##"<div class="docs-row">
  <ColorSlider
    id_base="docs-color-slider-matrix-hue".to_string()
    channel=ColorSliderChannel::Hue
    label="Hue".to_string()
    default_value=196.0
  />
  <ColorSlider
    id_base="docs-color-slider-matrix-saturation".to_string()
    channel=ColorSliderChannel::Saturation
    label="Saturation".to_string()
    default_value=72.0
  />
  <ColorSlider
    id_base="docs-color-slider-matrix-disabled".to_string()
    channel=ColorSliderChannel::Alpha
    label="Disabled alpha".to_string()
    default_value=40.0
    disabled=true
    motion=ColorSliderMotion::disabled()
  />
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"let (controlled_hue, set_controlled_hue) = signal(220.0_f64);
let on_controlled_hue_change =
  Callback::new(move |next: f64| set_controlled_hue.set(next));

<ColorSlider
  id_base="docs-color-slider-compare-controlled".to_string()
  channel=ColorSliderChannel::Hue
  label="Controlled".to_string()
  value=controlled_hue.into()
  on_value_change=on_controlled_hue_change
/>
<ColorSlider
  id_base="docs-color-slider-compare-uncontrolled".to_string()
  channel=ColorSliderChannel::Hue
  label="Uncontrolled".to_string()
  default_value=180.0
/>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorSlider is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "ColorSlider docs output mode: snapshot"
</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorSlider"
            slug="color-slider"
            group="Forms"
            description="baseline-compatible single-channel color slider with centralized channel/range/value normalization, spring motion integration, and stable slot/data-state contracts."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports="use leptos::prelude::*;\nuse ui_components::ColorSlider;".to_string()
            >
                <ColorSlider id_base="docs-color-slider-hello".to_string() />
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui_components::{ColorSlider, ColorSliderChannel, ColorSliderMotion};".to_string()
            >
                <div class="docs-row" data-slot="color-slider-state-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Ready · Hue"</div>
                        <ColorSlider
                            id_base="docs-color-slider-matrix-hue".to_string()
                            channel=ColorSliderChannel::Hue
                            label="Hue".to_string()
                            default_value=196.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Ready · Saturation"</div>
                        <ColorSlider
                            id_base="docs-color-slider-matrix-saturation".to_string()
                            channel=ColorSliderChannel::Saturation
                            label="Saturation".to_string()
                            default_value=72.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Disabled · Alpha"</div>
                        <ColorSlider
                            id_base="docs-color-slider-matrix-disabled".to_string()
                            channel=ColorSliderChannel::Alpha
                            label="Disabled alpha".to_string()
                            default_value=40.0
                            disabled=true
                            motion=reduced_motion
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=controlled_vs_uncontrolled_code
                code_imports="use leptos::prelude::*;\nuse ui_components::{ColorSlider, ColorSliderChannel};".to_string()
            >
                <div class="docs-row" data-slot="color-slider-controlled-vs-uncontrolled">
                    <div class="docs-card">
                        <div class="ui-muted">"Controlled"</div>
                        <ColorSlider
                            id_base="docs-color-slider-compare-controlled".to_string()
                            channel=ColorSliderChannel::Hue
                            label="Controlled".to_string()
                            value=hue.into()
                            on_value_change=on_hue_change
                        />
                        <span class="ui-muted">
                            "hue: " {move || format!("{:.0}°", hue.get())}
                        </span>
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Uncontrolled"</div>
                        <ColorSlider
                            id_base="docs-color-slider-compare-uncontrolled".to_string()
                            channel=ColorSliderChannel::Hue
                            label="Uncontrolled".to_string()
                            default_value=180.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                code_signal=output_mode_code
                code_imports="use leptos::prelude::*;\nuse ui_components::{ColorSlider, ColorSliderChannel};".to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-slider-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorSlider is an input surface; docs output mode remains snapshot (`fallback=snapshot`)."
                    </span>
                    <ColorSlider
                        id_base="docs-color-slider-snapshot".to_string()
                        channel=ColorSliderChannel::Hue
                        label="Snapshot fallback".to_string()
                        default_value=188.0
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Workbench (DX)"
                description="Scoped CSS test panel + dev-overrides.css 热更新路径；默认保留调试上下文，并提供可选持久化。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-slider/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-slider-workbench-controls">
                        <div class="docs-search__label">"Channel"</div>
                        <SegmentedControl
                            id_base="docs-color-slider-workbench-channel".to_string()
                            options=workbench_channel_options.clone()
                            selected_index=workbench_channel_index
                            set_selected_index=set_workbench_channel_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorSlider workbench channel".to_string()
                        />

                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_custom_track set_checked=set_workbench_custom_track>
                            "Custom track colors"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                        <Switch checked=workbench_preserve_context set_checked=set_workbench_preserve_context>
                            "Preserve context on channel change"
                        </Switch>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let channel = workbench_channel.get();
                    let reduced_motion = workbench_reduced_motion.get();
                    let custom_track = workbench_custom_track.get();
                    let custom_class = workbench_custom_class.get();
                    let is_disabled = workbench_disabled.get();
                    let value = workbench_value.get();
                    let motion = if reduced_motion {
                        ColorSliderMotion::disabled()
                    } else {
                        ColorSliderMotion::default()
                    };

                    let (track_start_color, track_end_color) = if custom_track {
                        ("#0f172a".to_string(), "#38bdf8".to_string())
                    } else {
                        (String::new(), String::new())
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-slider-workbench">
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="color-slider-workbench-canvas">
                                <ColorSlider
                                    id_base="docs-color-slider-workbench".to_string()
                                    channel=channel
                                    label=format!("{} (Workbench)", channel.default_label())
                                    value=workbench_value_signal
                                    on_value_change=on_workbench_value_change
                                    disabled=is_disabled
                                    track_start_color=track_start_color
                                    track_end_color=track_end_color
                                    motion=motion
                                    class_name=if custom_class {
                                        "docs-color-slider-workbench".to_string()
                                    } else {
                                        String::new()
                                    }
                                />
                            </div>

                            <span class="ui-muted" data-slot="color-slider-workbench-state">
                                "channel: " {channel.as_attr()}
                                " · value: " {format!("{value:.1}")}
                                " · preserve: " {if workbench_preserve_context.get() { "on" } else { "off" }}
                                " · persist: " {if workbench_persist_state.get() { "on" } else { "off" }}
                            </span>
                        </div>
                    }
                }}
            </Playground>

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

            <section class="docs-card docs-prose" data-slot="color-slider-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps snippet imports synchronized with "
                    <code>"code_imports"</code>
                    "."
                </p>
                <p data-slot="color-slider-source-paths">
                    "Source-first path: "
                    <code>"components/color-slider/src/mod.rs"</code>
                    ", "
                    <code>"components/color-slider/src/view.rs"</code>
                    ", "
                    <code>"components/color-slider/src/logic.rs"</code>
                    ", "
                    <code>"components/color-slider/src/styles.rs"</code>
                    ", "
                    <code>"components/color-slider/src/motion.rs"</code>
                    "."
                </p>
                <p data-slot="color-slider-source-prerequisites">
                    "Prerequisites: enable "
                    <code>"component-color_slider"</code>
                    " (and "
                    <code>"inject-css"</code>
                    " when runtime CSS injection is required) so copied snippets compile and render as expected."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_wheel() -> AnyView {
    #[derive(Clone, Copy, Debug)]
    struct ColorWheelWorkbenchPreset {
        label: &'static str,
        default_value: f64,
    }

    #[derive(Clone, Copy, Debug)]
    struct ColorWheelWorkbenchState {
        preset_index: usize,
        value: f64,
        is_disabled: bool,
        has_custom_class: bool,
        reduced_motion: bool,
        preserve_context: bool,
    }

    impl Default for ColorWheelWorkbenchState {
        fn default() -> Self {
            Self {
                preset_index: 0,
                value: 32.0,
                is_disabled: false,
                has_custom_class: false,
                reduced_motion: false,
                preserve_context: true,
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn bool_digit(value: bool) -> u8 {
        if value { 1 } else { 0 }
    }

    #[cfg(target_arch = "wasm32")]
    fn parse_bool(raw: &str) -> Option<bool> {
        match raw.trim() {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        }
    }

    impl ColorWheelWorkbenchState {
        #[cfg(target_arch = "wasm32")]
        fn parse(raw: &str) -> Option<Self> {
            let mut parts = raw.split(',');
            let preset_index = parts.next()?.trim().parse::<usize>().ok()?;
            let value = parts.next()?.trim().parse::<f64>().ok()?;
            let is_disabled = parse_bool(parts.next()?)?;
            let has_custom_class = parse_bool(parts.next()?)?;
            let reduced_motion = parse_bool(parts.next()?)?;
            let preserve_context = parse_bool(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }

            Some(Self {
                preset_index,
                value,
                is_disabled,
                has_custom_class,
                reduced_motion,
                preserve_context,
            })
        }

        #[cfg(target_arch = "wasm32")]
        fn serialize(self) -> String {
            format!(
                "{},{:.2},{},{},{},{}",
                self.preset_index,
                self.value,
                bool_digit(self.is_disabled),
                bool_digit(self.has_custom_class),
                bool_digit(self.reduced_motion),
                bool_digit(self.preserve_context),
            )
        }
    }

    #[cfg(target_arch = "wasm32")]
    const COLOR_WHEEL_WORKBENCH_STORAGE_KEY: &str = "docs:color-wheel:workbench:state";

    #[cfg(target_arch = "wasm32")]
    fn load_color_wheel_workbench_state() -> Option<ColorWheelWorkbenchState> {
        let storage = web_sys::window()?.local_storage().ok().flatten()?;
        let raw = storage
            .get_item(COLOR_WHEEL_WORKBENCH_STORAGE_KEY)
            .ok()
            .flatten()?;
        ColorWheelWorkbenchState::parse(&raw)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_color_wheel_workbench_state() -> Option<ColorWheelWorkbenchState> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    fn save_color_wheel_workbench_state(state: ColorWheelWorkbenchState) {
        if let Some(storage) = web_sys::window()
            .and_then(|window| window.local_storage().ok())
            .flatten()
        {
            drop(storage.set_item(COLOR_WHEEL_WORKBENCH_STORAGE_KEY, &state.serialize()));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_color_wheel_workbench_state(_state: ColorWheelWorkbenchState) {}

    #[cfg(target_arch = "wasm32")]
    fn clear_color_wheel_workbench_state() {
        if let Some(storage) = web_sys::window()
            .and_then(|window| window.local_storage().ok())
            .flatten()
        {
            drop(storage.remove_item(COLOR_WHEEL_WORKBENCH_STORAGE_KEY));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn clear_color_wheel_workbench_state() {}

    let (value, set_value) = signal(32.0_f64);
    let on_value_change = Callback::new(move |next: f64| set_value.set(next));

    let (disabled_value, set_disabled_value) = signal(248.0_f64);
    let on_disabled_change = Callback::new(move |next: f64| set_disabled_value.set(next));

    let reduced_motion = ColorWheelMotion::disabled();
    let workbench_preset_options = vec![
        "warm".to_string(),
        "cool".to_string(),
        "contrast".to_string(),
    ];
    let persisted_workbench_state = load_color_wheel_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let (workbench_preset_index, set_workbench_preset_index) =
        signal(Some(initial_workbench_state.preset_index.min(2)));
    let workbench_preset =
        Signal::derive(move || match workbench_preset_index.get().unwrap_or(0) {
            1 => ColorWheelWorkbenchPreset {
                label: "Cool hue",
                default_value: 216.0,
            },
            2 => ColorWheelWorkbenchPreset {
                label: "Contrast hue",
                default_value: 282.0,
            },
            _ => ColorWheelWorkbenchPreset {
                label: "Warm hue",
                default_value: 32.0,
            },
        });
    let (workbench_value, set_workbench_value) = signal(initial_workbench_state.value);
    let on_workbench_value_change = Callback::new(move |next: f64| set_workbench_value.set(next));
    let workbench_value_signal: Signal<f64> = workbench_value.into();
    let (workbench_disabled, set_workbench_disabled) = signal(initial_workbench_state.is_disabled);
    let (workbench_custom_class, set_workbench_custom_class) =
        signal(initial_workbench_state.has_custom_class);
    let (workbench_reduced_motion, set_workbench_reduced_motion) =
        signal(initial_workbench_state.reduced_motion);
    let (workbench_preserve_context, set_workbench_preserve_context) =
        signal(initial_workbench_state.preserve_context);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);
    let workbench_motion = Signal::derive(move || {
        if workbench_reduced_motion.get() {
            ColorWheelMotion::disabled()
        } else {
            ColorWheelMotion::default()
        }
    });

    let last_workbench_preset_index =
        RwSignal::new(workbench_preset_index.get_untracked().unwrap_or(0).min(2));
    let reset_workbench_value = set_workbench_value;
    let reset_workbench_disabled = set_workbench_disabled;
    let reset_workbench_custom_class = set_workbench_custom_class;
    let reset_workbench_reduced_motion = set_workbench_reduced_motion;

    Effect::new(move |_| {
        let next_preset_index = workbench_preset_index.get().unwrap_or(0).min(2);
        let prev_preset_index = last_workbench_preset_index.get_untracked();
        if next_preset_index == prev_preset_index {
            return;
        }

        last_workbench_preset_index.set(next_preset_index);
        if !workbench_preserve_context.get() {
            let preset = workbench_preset.get();
            reset_workbench_value.set(preset.default_value);
            reset_workbench_disabled.set(false);
            reset_workbench_custom_class.set(false);
            reset_workbench_reduced_motion.set(false);
        }
    });

    Effect::new(move |_| {
        let state = ColorWheelWorkbenchState {
            preset_index: workbench_preset_index.get().unwrap_or(0).min(2),
            value: workbench_value.get(),
            is_disabled: workbench_disabled.get(),
            has_custom_class: workbench_custom_class.get(),
            reduced_motion: workbench_reduced_motion.get(),
            preserve_context: workbench_preserve_context.get(),
        };

        if workbench_persist_state.get() {
            save_color_wheel_workbench_state(state);
        } else {
            clear_color_wheel_workbench_state();
        }
    });

    let hello_code = Signal::derive(move || {
        r##"<ColorWheel
  id_base="docs-color-wheel-hello".to_string()
/>"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<div class="docs-row" data-slot="color-wheel-state-matrix">
  <div class="docs-card">
    <div class="ui-muted">"Ready · Warm"</div>
    <ColorWheel
      id_base="docs-color-wheel-matrix-ready".to_string()
      label="Ready warm".to_string()
      default_value=32.0
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Ready · Cool"</div>
    <ColorWheel
      id_base="docs-color-wheel-matrix-cool".to_string()
      label="Ready cool".to_string()
      default_value=216.0
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Disabled · Reduced Motion"</div>
    <ColorWheel
      id_base="docs-color-wheel-matrix-disabled".to_string()
      label="Disabled wheel".to_string()
      default_value=282.0
      is_disabled=true
      motion=ColorWheelMotion::disabled()
    />
  </div>
</div>"##
            .to_string()
    });

    let parameter_matrix_code = Signal::derive(move || {
        r##"<div class="docs-row" data-slot="color-wheel-parameter-matrix">
  <div class="docs-card">
    <div class="ui-muted">"Defaults (logic.rs)"</div>
    <ColorWheel id_base="docs-color-wheel-param-default".to_string() />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Step + custom label"</div>
    <ColorWheel
      id_base="docs-color-wheel-param-step".to_string()
      label="Step 15°".to_string()
      step=15.0
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Hidden value label + custom class"</div>
    <ColorWheel
      id_base="docs-color-wheel-param-hidden-value".to_string()
      default_value=120.0
      is_value_label_visible=false
      aria_label="Hue selector".to_string()
      class_name="docs-color-wheel-custom".to_string()
    />
  </div>
</div>
<p class="ui-muted" data-slot="color-wheel-api-defaults-note">
  "Default API sync: step uses logic::DEFAULT_STEP when omitted; default_value falls back through logic::resolve_default_value; is_disabled defaults to false."
</p>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"let (value, set_value) = signal(32.0_f64);
let on_value_change = Callback::new(move |next: f64| set_value.set(next));

<ColorWheel
  id_base="docs-color-wheel-compare-controlled".to_string()
  label="Controlled".to_string()
  value=value.into()
  on_value_change=on_value_change
/>
<ColorWheel
  id_base="docs-color-wheel-compare-uncontrolled".to_string()
  label="Uncontrolled".to_string()
  default_value=180.0
/>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorWheel is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "ColorWheel docs output mode: snapshot"
</div>"##
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let preset = workbench_preset.get();
        let mut lines = vec![
            "let (value, set_value) = signal(32.0_f64);".to_string(),
            "let on_value_change = Callback::new(move |next: f64| set_value.set(next));"
                .to_string(),
            String::new(),
            "<ColorWheel".to_string(),
            "  id_base=\"docs-color-wheel-workbench\".into()".to_string(),
            format!("  label=\"{} (Workbench)\".into()", preset.label),
            "  value=value.into()".to_string(),
            "  on_value_change=on_value_change".to_string(),
            format!("  default_value={:.1}", workbench_value.get()),
            format!("  is_disabled={}", workbench_disabled.get()),
        ];

        if workbench_reduced_motion.get() {
            lines.push("  motion=ColorWheelMotion::disabled()".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-color-wheel-workbench\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        if workbench_custom_class.get() {
            ".docs-color-wheel-workbench {\n  --ui-color-wheel-size: clamp(14rem, 42vw, 17rem);\n  --ui-color-wheel-thumb-size: 1.2rem;\n  --ui-color-wheel-track-shadow: 0 14px 30px color-mix(in oklch, var(--ui-brand), transparent 70%);\n}"
                .to_string()
        } else {
            String::new()
        }
    });

    let workbench_actual_config = Signal::derive(move || {
        let preset = workbench_preset.get();
        let value = workbench_value.get();
        let is_disabled = workbench_disabled.get();
        let custom_class = workbench_custom_class.get();
        let reduced_motion = workbench_reduced_motion.get();
        let preserve_context = workbench_preserve_context.get();
        let persist_state = workbench_persist_state.get();
        format!(
            "ColorWheelWorkbenchConfig {{\n  preset_label: \"{}\",\n  value: {:.2},\n  is_disabled: {},\n  custom_class: {},\n  reduced_motion: {},\n  preserve_context: {},\n  persist_state: {},\n}}",
            preset.label,
            value,
            is_disabled,
            custom_class,
            reduced_motion,
            preserve_context,
            persist_state,
        )
    });

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
  is_disabled=true
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

    let baseline_code = Signal::derive(move || {
        r##"<div
  class="docs-stack docs-stack--tight"
  data-doc-visual-baseline="color-wheel-default-theme"
  data-doc-baseline-shot="color-wheel-default-theme-v1"
>
  <ColorWheel
    id_base="docs-color-wheel-baseline-primary".to_string()
    label="Primary hue".to_string()
    default_value=24.0
  />
  <ColorWheel
    id_base="docs-color-wheel-baseline-depth".to_string()
    label="Contrast depth".to_string()
    default_value=216.0
  />
</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorWheel"
            slug="color-wheel"
            group="Forms"
            description="baseline-compatible hue wheel with centralized value/step/wrap-around normalization, spring-driven thumb motion, and stable slot/data-state contracts."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports="use leptos::prelude::*;\nuse ui_components::ColorWheel;".to_string()
            >
                <ColorWheel id_base="docs-color-wheel-hello".to_string() />
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui_components::{ColorWheel, ColorWheelMotion};".to_string()
            >
                <div class="docs-row" data-slot="color-wheel-state-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Ready · Warm"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-matrix-ready".to_string()
                            label="Ready warm".to_string()
                            default_value=32.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Ready · Cool"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-matrix-cool".to_string()
                            label="Ready cool".to_string()
                            default_value=216.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Disabled · Reduced Motion"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-matrix-disabled".to_string()
                            label="Disabled wheel".to_string()
                            default_value=282.0
                            is_disabled=true
                            motion=reduced_motion
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Parameter Matrix"
                code_signal=parameter_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui_components::ColorWheel;".to_string()
            >
                <div class="docs-row" data-slot="color-wheel-parameter-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Defaults (logic.rs)"</div>
                        <ColorWheel id_base="docs-color-wheel-param-default".to_string() />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Step + custom label"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-param-step".to_string()
                            label="Step 15°".to_string()
                            step=15.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Hidden value label + custom class"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-param-hidden-value".to_string()
                            default_value=120.0
                            is_value_label_visible=false
                            aria_label="Hue selector".to_string()
                            class_name="docs-color-wheel-custom".to_string()
                        />
                    </div>
                </div>
                <p class="ui-muted" data-slot="color-wheel-api-defaults-note">
                    "Default API sync: step uses logic::DEFAULT_STEP when omitted; default_value falls back through logic::resolve_default_value; is_disabled defaults to false."
                </p>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=controlled_vs_uncontrolled_code
                code_imports="use leptos::prelude::*;\nuse ui_components::ColorWheel;".to_string()
            >
                <div class="docs-row" data-slot="color-wheel-controlled-vs-uncontrolled">
                    <div class="docs-card">
                        <div class="ui-muted">"Controlled"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-compare-controlled".to_string()
                            label="Controlled".to_string()
                            value=value.into()
                            on_value_change=on_value_change
                        />
                        <span class="ui-muted">
                            "hue: " {move || format!("{:.0}°", value.get())}
                        </span>
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Uncontrolled"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-compare-uncontrolled".to_string()
                            label="Uncontrolled".to_string()
                            default_value=180.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                code_signal=output_mode_code
                code_imports="use leptos::prelude::*;\nuse ui_components::ColorWheel;".to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-wheel-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorWheel is an input surface; docs output mode remains snapshot (`fallback=snapshot`)."
                    </span>
                    <ColorWheel
                        id_base="docs-color-wheel-snapshot".to_string()
                        label="Snapshot fallback".to_string()
                        default_value=188.0
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Workbench (DX)"
                description="Scoped CSS test panel + workbench 隔离画布；默认保留上下文并支持可选持久化。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-wheel/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-wheel-workbench-controls">
                        <div data-slot="color-wheel-workbench-preset">
                            <div class="docs-search__label">"Preset"</div>
                            <SegmentedControl
                                id_base="docs-color-wheel-workbench-preset".to_string()
                                options=workbench_preset_options.clone()
                                selected_index=workbench_preset_index
                                set_selected_index=set_workbench_preset_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorWheel workbench preset".to_string()
                            />
                        </div>

                        <div data-slot="color-wheel-workbench-toggle-disabled">
                            <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                                "Disabled"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-custom-class">
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-reduced-motion">
                            <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                                "Reduced motion"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-preserve-context">
                            <Switch checked=workbench_preserve_context set_checked=set_workbench_preserve_context>
                                "Preserve context on preset change"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-persist-state">
                            <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                                "Persist workbench state"
                            </Switch>
                        </div>
                    </div>
                }
            >
                {move || {
                    let preset = workbench_preset.get();
                    let value = workbench_value.get();
                    let is_disabled = workbench_disabled.get();
                    let motion = workbench_motion.get();
                    let class_name = if workbench_custom_class.get() {
                        "docs-color-wheel-workbench".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-wheel-workbench">
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="color-wheel-workbench-canvas">
                                <ColorWheel
                                    id_base="docs-color-wheel-workbench".to_string()
                                    label=format!("{} (Workbench)", preset.label)
                                    value=workbench_value_signal
                                    on_value_change=on_workbench_value_change
                                    is_disabled=is_disabled
                                    motion=motion
                                    class_name=class_name
                                />
                            </div>

                            <span class="ui-muted" data-slot="color-wheel-workbench-state">
                                "preset: " {preset.label}
                                " · value: " {format!("{value:.1}")}
                                " · preserve: " {if workbench_preserve_context.get() { "on" } else { "off" }}
                                " · persist: " {if workbench_persist_state.get() { "on" } else { "off" }}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="Default Theme Baseline"
                description="默认主题视觉基线：信息层级、对比层次，以及 hover/active/focus 交互反馈。"
                code_signal=baseline_code
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-doc-visual-baseline="color-wheel-default-theme"
                    data-doc-baseline-shot="color-wheel-default-theme-v1"
                    data-doc-visual-targets="label,value,hover,active,focus"
                >
                    <ColorWheel
                        id_base="docs-color-wheel-baseline-primary".to_string()
                        label="Primary hue".to_string()
                        default_value=24.0
                    />
                    <ColorWheel
                        id_base="docs-color-wheel-baseline-depth".to_string()
                        label="Contrast depth".to_string()
                        default_value=216.0
                    />
                </div>
            </Playground>

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
                        is_disabled=true
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

            <section class="docs-card docs-prose" data-slot="color-wheel-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps snippet imports synchronized with "
                    <code>"code_imports"</code>
                    "."
                </p>
                <p data-slot="color-wheel-source-paths">
                    "Source-first path: "
                    <code>"components/color-wheel/src/mod.rs"</code>
                    ", "
                    <code>"components/color-wheel/src/view.rs"</code>
                    ", "
                    <code>"components/color-wheel/src/logic.rs"</code>
                    ", "
                    <code>"components/color-wheel/src/styles.rs"</code>
                    ", "
                    <code>"components/color-wheel/src/motion.rs"</code>
                    "."
                </p>
                <p data-slot="color-wheel-source-prerequisites">
                    "Prerequisites: enable "
                    <code>"component-color_wheel"</code>
                    " (and "
                    <code>"inject-css"</code>
                    " when runtime CSS injection is required) so copied snippets compile and render as expected."
                </p>
                <p data-slot="color-wheel-source-first-contract">
                    "Contract: use any ColorWheel playground's "
                    <code>"Show code + Copy"</code>
                    " path for one-click runnable snippets; snippets stay synchronized with current props and imports."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_picker() -> AnyView {
    let (selected_color, set_selected_color) = signal(Some("#ef4444".to_string()));
    let on_selected_change =
        Callback::new(move |next: Option<String>| set_selected_color.set(next));
    let (compare_selected_color, set_compare_selected_color) = signal(Some("#22c55e".to_string()));
    let on_compare_selected_change =
        Callback::new(move |next: Option<String>| set_compare_selected_color.set(next));

    let (open, set_open) = signal(false);
    let on_open_change = Callback::new(move |next: bool| set_open.set(next));
    let (compare_open, set_compare_open) = signal(false);
    let on_compare_open_change = Callback::new(move |next: bool| set_compare_open.set(next));

    let (swatches, _set_swatches) = signal(vec![
        ColorSwatchPickerItem::named("#ef4444", "Red"),
        ColorSwatchPickerItem::named("#f59e0b", "Amber"),
        ColorSwatchPickerItem::named("#10b981", "Emerald"),
        ColorSwatchPickerItem::named("#3b82f6", "Blue"),
        ColorSwatchPickerItem::named("#8b5cf6", "Violet"),
    ]);

    let selected_color_signal: Signal<Option<String>> = selected_color.into();
    let open_signal: Signal<bool> = open.into();
    let compare_selected_color_signal: Signal<Option<String>> = compare_selected_color.into();
    let compare_open_signal: Signal<bool> = compare_open.into();
    let color_picker_imports = "use leptos::prelude::*;\nuse ui_components::{ColorPicker, ColorSwatchPicker, ColorSwatchPickerItem};".to_string();

    let hello_code = Signal::derive(move || {
        r##"<ColorPicker
  id_base="docs-color-picker-hello".to_string()
>
  <div class="ui-muted">"Default picker content"</div>
</ColorPicker>"##
            .to_string()
    });

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

    let state_matrix_code = Signal::derive(move || {
        r##"<div class="docs-row" data-slot="color-picker-state-matrix">
  <div class="docs-card">
    <div class="ui-muted">"Ready"</div>
    <ColorPicker
      id_base="docs-color-picker-matrix-ready".to_string()
      label="Ready".to_string()
      default_selected_color="#3b82f6".to_string()
    >
      <ColorSwatchPicker
        swatches=swatches
        selected_color=selected_color_signal
        on_selected_change=on_selected_change
      />
    </ColorPicker>
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Open"</div>
    <ColorPicker
      id_base="docs-color-picker-matrix-open".to_string()
      label="Open".to_string()
      default_selected_color="#8b5cf6".to_string()
      default_open=true
    >
      <div class="ui-muted">"Overlay preview"</div>
    </ColorPicker>
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Disabled"</div>
    <ColorPicker
      id_base="docs-color-picker-matrix-disabled".to_string()
      label="Disabled".to_string()
      default_selected_color="#0ea5e9".to_string()
      is_disabled=true
    >
      <div class="ui-muted">"Disabled picker"</div>
    </ColorPicker>
  </div>
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"let (controlled_color, set_controlled_color) = signal(Some("#22c55e".to_string()));
let on_controlled_color_change =
  Callback::new(move |next: Option<String>| set_controlled_color.set(next));
let (controlled_open, set_controlled_open) = signal(false);
let on_controlled_open_change = Callback::new(move |next: bool| set_controlled_open.set(next));

<ColorPicker
  id_base="docs-color-picker-compare-controlled".to_string()
  label="Controlled".to_string()
  selected_color=controlled_color.into()
  on_selected_change=on_controlled_color_change
  open=controlled_open.into()
  on_open_change=on_controlled_open_change
>
  <ColorSwatchPicker
    swatches=swatches
    selected_color=controlled_color.into()
    on_selected_change=on_controlled_color_change
  />
</ColorPicker>

<ColorPicker
  id_base="docs-color-picker-compare-uncontrolled".to_string()
  label="Uncontrolled".to_string()
  default_selected_color="#8b5cf6".to_string()
  default_open=true
>
  <div class="ui-muted">"Uncontrolled content"</div>
</ColorPicker>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorPicker is an interaction component, not an LLM text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "ColorPicker docs output mode: snapshot"
</div>"##
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r##"let swatches = vec![
  ColorSwatchPickerItem::named("#ef4444", "Red"),
  ColorSwatchPickerItem::named("#3b82f6", "Blue"),
];
let (selected, set_selected) = signal(Some("#ef4444".to_string()));
let on_selected_change = Callback::new(move |next: Option<String>| set_selected.set(next));

<ColorPicker
  id_base="docs-color-picker-source-first".to_string()
  label="Source-first starter".to_string()
  selected_color=selected.into()
  on_selected_change=on_selected_change
>
  <ColorSwatchPicker
    swatches=swatches
    selected_color=selected.into()
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
  is_disabled=true
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

    let baseline_code = Signal::derive(move || {
        r##"<div
  class="docs-stack docs-stack--tight"
  data-doc-visual-baseline="color-picker-default-theme"
  data-doc-baseline-shot="color-picker-default-theme-v1"
>
  <ColorPicker
    id_base="docs-color-picker-baseline-default".to_string()
    label="Primary".to_string()
    default_selected_color="#3b82f6".to_string()
  >
    <ColorSwatchPicker
      swatches=swatches
      selected_color=selected_color_signal
      on_selected_change=on_selected_change
    />
  </ColorPicker>

  <ColorPicker
    id_base="docs-color-picker-baseline-open".to_string()
    label="Overlay depth".to_string()
    default_selected_color="#8b5cf6".to_string()
    default_open=true
  >
    <div class="docs-stack docs-stack--tight">
      <span class="ui-muted">"Overlay baseline content"</span>
      <span class="ui-muted">"Hover/Focus/Depth target"</span>
    </div>
  </ColorPicker>
</div>"##
            .to_string()
    });

    let workbench_palette_options = vec!["Warm".to_string(), "Cool".to_string()];
    let (workbench_palette_index, set_workbench_palette_index) = signal(Some(0usize));
    let (workbench_selected_color, set_workbench_selected_color) =
        signal(Some("#ef4444".to_string()));
    let on_workbench_selected_change = Callback::new(move |next: Option<String>| {
        set_workbench_selected_color.set(next);
    });
    let workbench_selected_color_signal: Signal<Option<String>> = workbench_selected_color.into();
    let (workbench_open, set_workbench_open) = signal(false);
    let on_workbench_open_change = Callback::new(move |next: bool| set_workbench_open.set(next));
    let workbench_open_signal: Signal<bool> = workbench_open.into();
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_preserve_context, set_workbench_preserve_context) = signal(true);

    let last_workbench_palette_index =
        RwSignal::new(workbench_palette_index.get_untracked().unwrap_or(0).min(1));
    Effect::new(move |_| {
        let next_palette_index = workbench_palette_index.get().unwrap_or(0).min(1);
        let prev_palette_index = last_workbench_palette_index.get_untracked();
        if next_palette_index == prev_palette_index {
            return;
        }

        last_workbench_palette_index.set(next_palette_index);
        if !workbench_preserve_context.get() {
            let default_color = if next_palette_index == 0 {
                "#ef4444"
            } else {
                "#06b6d4"
            };
            set_workbench_selected_color.set(Some(default_color.to_string()));
            set_workbench_open.set(false);
            set_workbench_disabled.set(false);
            set_workbench_custom_class.set(false);
        }
    });

    let (workbench_swatches, set_workbench_swatches) = signal(
        match workbench_palette_index.get_untracked().unwrap_or(0).min(1) {
            0 => vec![
                ColorSwatchPickerItem::named("#ef4444", "Red"),
                ColorSwatchPickerItem::named("#f59e0b", "Amber"),
                ColorSwatchPickerItem::named("#f97316", "Orange"),
                ColorSwatchPickerItem::named("#f43f5e", "Rose"),
                ColorSwatchPickerItem::named("#eab308", "Yellow"),
            ],
            _ => vec![
                ColorSwatchPickerItem::named("#06b6d4", "Cyan"),
                ColorSwatchPickerItem::named("#3b82f6", "Blue"),
                ColorSwatchPickerItem::named("#8b5cf6", "Violet"),
                ColorSwatchPickerItem::named("#10b981", "Emerald"),
                ColorSwatchPickerItem::named("#14b8a6", "Teal"),
            ],
        },
    );
    Effect::new(move |_| {
        let next = match workbench_palette_index.get().unwrap_or(0).min(1) {
            0 => vec![
                ColorSwatchPickerItem::named("#ef4444", "Red"),
                ColorSwatchPickerItem::named("#f59e0b", "Amber"),
                ColorSwatchPickerItem::named("#f97316", "Orange"),
                ColorSwatchPickerItem::named("#f43f5e", "Rose"),
                ColorSwatchPickerItem::named("#eab308", "Yellow"),
            ],
            _ => vec![
                ColorSwatchPickerItem::named("#06b6d4", "Cyan"),
                ColorSwatchPickerItem::named("#3b82f6", "Blue"),
                ColorSwatchPickerItem::named("#8b5cf6", "Violet"),
                ColorSwatchPickerItem::named("#10b981", "Emerald"),
                ColorSwatchPickerItem::named("#14b8a6", "Teal"),
            ],
        };
        set_workbench_swatches.set(next);
    });

    let workbench_code = Signal::derive(move || {
        let palette = if workbench_palette_index.get().unwrap_or(0).min(1) == 0 {
            "warm"
        } else {
            "cool"
        };
        let selected = workbench_selected_color
            .get()
            .unwrap_or_else(|| "none".to_string());
        let open = workbench_open.get();
        let is_disabled = workbench_disabled.get();
        let custom_class = workbench_custom_class.get();
        let preserve_context = workbench_preserve_context.get();

        let class_name_line = if custom_class {
            "  class_name=\"docs-color-picker-workbench\".into()\n"
        } else {
            ""
        };

        format!(
            "<ColorPicker\n  id_base=\"docs-color-picker-workbench\".into()\n  label=\"{palette} palette\".into()\n  selected_color=selected.into()\n  on_selected_change=on_selected_change\n  open=open.into()\n  on_open_change=on_open_change\n  is_disabled={is_disabled}\n{class_name_line}>\n  <ColorSwatchPicker\n    swatches=swatches\n    selected_color=selected.into()\n    on_selected_change=on_selected_change\n  />\n</ColorPicker>\n\n// selected={selected}; open={open}; preserve_context={preserve_context}",
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        r#"
:scope .docs-color-picker-workbench[data-open="true"] .ui-color-picker__trigger {
  outline: 1px solid color-mix(in oklch, var(--ui-accent), white 24%);
  outline-offset: 2px;
}
:scope .docs-color-picker-workbench .ui-color-picker__panel {
  border-color: color-mix(in oklch, var(--ui-accent), transparent 72%);
}
"#
        .trim()
        .to_string()
    });

    let workbench_actual_config = Signal::derive(move || {
        let palette = if workbench_palette_index.get().unwrap_or(0).min(1) == 0 {
            "warm"
        } else {
            "cool"
        };
        let selected = workbench_selected_color
            .get()
            .unwrap_or_else(|| "none".to_string());
        let open = workbench_open.get();
        let is_disabled = workbench_disabled.get();
        let custom_class = workbench_custom_class.get();
        let preserve_context = workbench_preserve_context.get();
        format!(
            "{{\n  \"palette\": \"{palette}\",\n  \"selected\": \"{selected}\",\n  \"open\": {open},\n  \"disabled\": {is_disabled},\n  \"custom_class\": {custom_class},\n  \"preserve_context\": {preserve_context}\n}}",
        )
    });

    view! {
        <ComponentPage
            title="ColorPicker"
            slug="color-picker"
            group="Forms"
            description="baseline-compatible color picker primitive that composes swatch trigger + popover content with controllable color/open state and stable slot/data-state contracts."
        >
            <Playground
                title="Hello World（默认路径）"
                code_signal=hello_code
                code_imports=color_picker_imports.clone()
            >
                <ColorPicker id_base="docs-color-picker-hello".to_string()>
                    <div class="ui-muted">"Default picker content"</div>
                </ColorPicker>
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=color_picker_imports.clone()
            >
                <div class="docs-row" data-slot="color-picker-state-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Ready"</div>
                        <ColorPicker
                            id_base="docs-color-picker-matrix-ready".to_string()
                            label="Ready".to_string()
                            default_selected_color="#3b82f6".to_string()
                        >
                            <ColorSwatchPicker
                                swatches=swatches
                                selected_color=selected_color_signal
                                on_selected_change=on_selected_change
                            />
                        </ColorPicker>
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Open"</div>
                        <ColorPicker
                            id_base="docs-color-picker-matrix-open".to_string()
                            label="Open".to_string()
                            default_selected_color="#8b5cf6".to_string()
                            default_open=true
                        >
                            <div class="ui-muted">"Overlay preview"</div>
                        </ColorPicker>
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Disabled"</div>
                        <ColorPicker
                            id_base="docs-color-picker-matrix-disabled".to_string()
                            label="Disabled".to_string()
                            default_selected_color="#0ea5e9".to_string()
                            is_disabled=true
                        >
                            <div class="ui-muted">"Disabled picker"</div>
                        </ColorPicker>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Default Theme Baseline"
                description="默认主题视觉基线：信息层级、对比层次、hover/focus 反馈与 overlay 深度。"
                code_signal=baseline_code
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-doc-visual-baseline="color-picker-default-theme"
                    data-doc-baseline-shot="color-picker-default-theme-v1"
                    data-doc-visual-targets="trigger,overlay,content"
                >
                    <ColorPicker
                        id_base="docs-color-picker-baseline-default".to_string()
                        label="Primary".to_string()
                        default_selected_color="#3b82f6".to_string()
                    >
                        <ColorSwatchPicker
                            swatches=swatches
                            selected_color=selected_color_signal
                            on_selected_change=on_selected_change
                        />
                    </ColorPicker>

                    <ColorPicker
                        id_base="docs-color-picker-baseline-open".to_string()
                        label="Overlay depth".to_string()
                        default_selected_color="#8b5cf6".to_string()
                        default_open=true
                    >
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Overlay baseline content"</span>
                            <span class="ui-muted">"Hover/Focus/Depth target"</span>
                        </div>
                    </ColorPicker>
                </div>
            </Playground>

            <Playground
                title="Interactive Workbench (DX)"
                description="Scoped CSS test panel + 交互配置工作台；默认保留上下文，可按需关闭保留以复位状态。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-picker/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-picker-workbench-controls">
                        <div class="docs-search__label">"Palette"</div>
                        <SegmentedControl
                            id_base="docs-color-picker-workbench-palette".to_string()
                            options=workbench_palette_options.clone()
                            selected_index=workbench_palette_index
                            set_selected_index=set_workbench_palette_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorPicker workbench palette".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_preserve_context set_checked=set_workbench_preserve_context>
                            "Preserve context on palette change"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let palette_name = if workbench_palette_index.get().unwrap_or(0).min(1) == 0 {
                        "Warm"
                    } else {
                        "Cool"
                    };
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-picker-workbench">
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="color-picker-workbench-canvas">
                                <ColorPicker
                                    id_base="docs-color-picker-workbench".to_string()
                                    label=format!("{palette_name} palette")
                                    selected_color=workbench_selected_color_signal
                                    on_selected_change=on_workbench_selected_change
                                    open=workbench_open_signal
                                    on_open_change=on_workbench_open_change
                                    is_disabled=workbench_disabled.get()
                                    class_name=if workbench_custom_class.get() {
                                        "docs-color-picker-workbench".to_string()
                                    } else {
                                        String::new()
                                    }
                                >
                                    <ColorSwatchPicker
                                        swatches=workbench_swatches
                                        selected_color=workbench_selected_color_signal
                                        on_selected_change=on_workbench_selected_change
                                    />
                                </ColorPicker>
                            </div>

                            <span class="ui-muted" data-slot="color-picker-workbench-state">
                                "palette: " {palette_name}
                                " · selected: " {workbench_selected_color.get().unwrap_or_else(|| "none".to_string())}
                                " · open: " {if workbench_open.get() { "true" } else { "false" }}
                                " · preserve: " {if workbench_preserve_context.get() { "on" } else { "off" }}
                            </span>
                        </div>
                    }
                }}
            </Playground>

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

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=controlled_vs_uncontrolled_code
                code_imports=color_picker_imports.clone()
            >
                <div class="docs-row" data-slot="color-picker-controlled-vs-uncontrolled">
                    <div class="docs-card">
                        <div class="ui-muted">"Controlled"</div>
                        <ColorPicker
                            id_base="docs-color-picker-compare-controlled".to_string()
                            label="Controlled".to_string()
                            selected_color=compare_selected_color_signal
                            on_selected_change=on_compare_selected_change
                            open=compare_open_signal
                            on_open_change=on_compare_open_change
                        >
                            <ColorSwatchPicker
                                swatches=swatches
                                selected_color=compare_selected_color_signal
                                on_selected_change=on_compare_selected_change
                            />
                        </ColorPicker>
                        <span class="ui-muted">
                            "selected: "
                            {move || compare_selected_color.get().unwrap_or_else(|| "none".to_string())}
                            " · open: "
                            {move || if compare_open.get() { "true" } else { "false" }}
                        </span>
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Uncontrolled"</div>
                        <ColorPicker
                            id_base="docs-color-picker-compare-uncontrolled".to_string()
                            label="Uncontrolled".to_string()
                            default_selected_color="#8b5cf6".to_string()
                            default_open=true
                        >
                            <div class="ui-muted">"Uncontrolled content"</div>
                        </ColorPicker>
                    </div>
                </div>
            </Playground>

            <Playground title="Disabled + Default Open + Custom Class" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorPicker
                        id_base="docs-color-picker-disabled".to_string()
                        label="Disabled".to_string()
                        default_selected_color="#0ea5e9".to_string()
                        is_disabled=true
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

            <Playground
                title="Streaming Optional / Snapshot"
                code_signal=output_mode_code
                code_imports=color_picker_imports.clone()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-picker-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorPicker is an interaction component; docs output stays snapshot (`fallback=snapshot`)."
                    </span>
                    <ColorPicker
                        id_base="docs-color-picker-snapshot".to_string()
                        label="Snapshot fallback".to_string()
                        default_selected_color="#6366f1".to_string()
                    >
                        <div class="ui-muted">"Snapshot-only output surface"</div>
                    </ColorPicker>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                code_signal=source_first_code
                code_imports=color_picker_imports.clone()
            >
                <ColorPicker
                    id_base="docs-color-picker-source-first".to_string()
                    label="Source-first starter".to_string()
                    selected_color=selected_color_signal
                    on_selected_change=on_selected_change
                >
                    <ColorSwatchPicker
                        swatches=swatches
                        selected_color=selected_color_signal
                        on_selected_change=on_selected_change
                    />
                </ColorPicker>
            </Playground>

            <section class="docs-card docs-prose" data-slot="color-picker-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps snippet imports synchronized with "
                    <code>"code_imports"</code>
                    "."
                </p>
                <p data-slot="color-picker-source-paths">
                    "Source-first path: "
                    <code>"components/color-picker/src/mod.rs"</code>
                    ", "
                    <code>"components/color-picker/src/view.rs"</code>
                    ", "
                    <code>"components/color-picker/src/logic.rs"</code>
                    ", "
                    <code>"components/color-picker/src/styles.rs"</code>
                    ", "
                    <code>"components/color-picker/src/motion.rs"</code>
                    "."
                </p>
                <p data-slot="color-picker-source-prerequisites">
                    "Prerequisites: enable "
                    <code>"component-color_picker"</code>
                    " (and "
                    <code>"inject-css"</code>
                    " when runtime CSS injection is required) so copied snippets compile and render as expected."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_thumb() -> AnyView {
    #[derive(Clone, Debug, serde::Deserialize)]
    struct ColorThumbWorkbenchSpec {
        color: Option<String>,
        x_percent: Option<f32>,
        y_percent: Option<f32>,
        is_disabled: Option<bool>,
        is_focused: Option<bool>,
        is_dragging: Option<bool>,
        is_loupe_visible: Option<bool>,
        class_name: Option<String>,
    }

    let board_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm); background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);";
    let color_thumb_imports = "use leptos::prelude::*;\nuse ui_components::ColorThumb;".to_string();
    let (workbench_color, set_workbench_color) = signal("#10b981".to_string());
    let (workbench_x_percent, set_workbench_x_percent) = signal(48.0_f32);
    let (workbench_y_percent, set_workbench_y_percent) = signal(46.0_f32);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_focused, set_workbench_focused) = signal(true);
    let (workbench_dragging, set_workbench_dragging) = signal(false);
    let (workbench_loupe_visible, set_workbench_loupe_visible) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_replay_count, set_workbench_replay_count) = signal(0_u32);
    let (workbench_spec_input, set_workbench_spec_input) = signal(
        r##"{"color":"#0ea5e9","x_percent":66.0,"y_percent":34.0,"is_focused":true}"##.to_string(),
    );
    let workbench_spec = Signal::derive(move || {
        serde_json::from_str::<ColorThumbWorkbenchSpec>(&workbench_spec_input.get()).ok()
    });

    let hello_code = Signal::derive(move || {
        r##"<div style=board_style>
  <ColorThumb id_base="docs-color-thumb-hello".to_string() />
</div>"##
            .to_string()
    });

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
    is_focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorThumb
    id_base="docs-color-thumb-dragging".to_string()
    color="#3b82f6".to_string()
    is_dragging=true
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
    is_disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorThumb
    id_base="docs-color-thumb-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_dragging=true
    is_loupe_visible=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-thumb-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<div style=board_style data-slot="color-thumb-state-matrix">
  <ColorThumb
    id_base="docs-color-thumb-matrix-idle".to_string()
    color="#f59e0b".to_string()
    x_percent=22.0
    y_percent=72.0
  />
  <ColorThumb
    id_base="docs-color-thumb-matrix-focused".to_string()
    color="#10b981".to_string()
    is_focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorThumb
    id_base="docs-color-thumb-matrix-dragging".to_string()
    color="#3b82f6".to_string()
    is_dragging=true
    x_percent=82.0
    y_percent=28.0
  />
  <ColorThumb
    id_base="docs-color-thumb-matrix-disabled".to_string()
    color="#a78bfa".to_string()
    is_disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorThumb
    id_base="docs-color-thumb-matrix-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_dragging=true
    is_loupe_visible=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-thumb-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"<div class="docs-stack docs-stack--tight">
  <span class="ui-muted">
    "ColorThumb has no value/default_value/on_value_change axis; both examples are external-props driven."
  </span>
  <div class="docs-row" data-slot="color-thumb-controlled-vs-uncontrolled">
    <div class="docs-card">
      <div class="ui-muted">"External props path A"</div>
      <div style=board_style>
        <ColorThumb
          id_base="docs-color-thumb-controlled-like".to_string()
          color="#10b981".to_string()
          is_focused=true
          x_percent=48.0
          y_percent=46.0
        />
      </div>
    </div>
    <div class="docs-card">
      <div class="ui-muted">"External props path B"</div>
      <div style=board_style>
        <ColorThumb
          id_base="docs-color-thumb-uncontrolled-like".to_string()
          color="#10b981".to_string()
          is_focused=true
          x_percent=48.0
          y_percent=46.0
        />
      </div>
    </div>
  </div>
</div>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorThumb is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  <ColorThumb
    id_base="docs-color-thumb-snapshot".to_string()
    color="#334155".to_string()
    x_percent=44.0
    y_percent=56.0
  />
</div>"##
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r##"<div style=board_style>
  <ColorThumb
    id_base="docs-color-thumb-source-first".to_string()
    color="#14b8a6".to_string()
    is_focused=true
    x_percent=58.0
    y_percent=38.0
  />
</div>"##
            .to_string()
    });
    let workbench_code = Signal::derive(move || {
        let class_name_line = if workbench_custom_class.get() {
            "    class_name=\"docs-color-thumb-workbench\".to_string()\n"
        } else {
            ""
        };

        format!(
            "<div style=board_style>\n  <ColorThumb\n    id_base=\"docs-color-thumb-workbench\".to_string()\n    color=\"{}\".to_string()\n    is_disabled={}\n    is_focused={}\n    is_dragging={}\n    is_loupe_visible={}\n    x_percent={:.1}\n    y_percent={:.1}\n{}  />\n</div>\n\n// replay_count={}; spec_valid={}",
            workbench_color.get(),
            workbench_disabled.get(),
            workbench_focused.get(),
            workbench_dragging.get(),
            workbench_loupe_visible.get(),
            workbench_x_percent.get(),
            workbench_y_percent.get(),
            class_name_line,
            workbench_replay_count.get(),
            if workbench_spec.get().is_some() {
                "true"
            } else {
                "false"
            },
        )
    });
    let workbench_test_css_source = Signal::derive(move || {
        r#"
:scope .docs-color-thumb-workbench[data-state="focused"] .ui-color-thumb__handle {
  box-shadow: 0 0 0 2px color-mix(in oklch, var(--ui-accent), white 24%);
}

:scope .docs-color-thumb-workbench[data-state="dragging"] .ui-color-thumb__loupe {
  transform: translateY(-0.25rem);
}
"#
        .trim()
        .to_string()
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "{{\n  \"color\": \"{}\",\n  \"x_percent\": {:.1},\n  \"y_percent\": {:.1},\n  \"is_disabled\": {},\n  \"is_focused\": {},\n  \"is_dragging\": {},\n  \"is_loupe_visible\": {},\n  \"custom_class\": {},\n  \"replay_count\": {},\n  \"spec_valid\": {}\n}}",
            workbench_color.get(),
            workbench_x_percent.get(),
            workbench_y_percent.get(),
            workbench_disabled.get(),
            workbench_focused.get(),
            workbench_dragging.get(),
            workbench_loupe_visible.get(),
            workbench_custom_class.get(),
            workbench_replay_count.get(),
            workbench_spec.get().is_some(),
        )
    });

    view! {
        <ComponentPage
            title="ColorThumb"
            slug="color-thumb"
            group="Forms"
            description="baseline-compatible draggable color thumb primitive with focus/drag/loupe state contracts, sanitized color source handling, and stable slot/data-state markers."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div style=board_style>
                    <ColorThumb id_base="docs-color-thumb-hello".to_string() />
                </div>
            </Playground>

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
                        is_focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-dragging".to_string()
                        color="#3b82f6".to_string()
                        is_dragging=true
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
                        is_disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_dragging=true
                        is_loupe_visible=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-thumb-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=color_thumb_imports.clone()
            >
                <div style=board_style data-slot="color-thumb-state-matrix">
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-idle".to_string()
                        color="#f59e0b".to_string()
                        x_percent=22.0
                        y_percent=72.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-focused".to_string()
                        color="#10b981".to_string()
                        is_focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-dragging".to_string()
                        color="#3b82f6".to_string()
                        is_dragging=true
                        x_percent=82.0
                        y_percent=28.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-disabled".to_string()
                        color="#a78bfa".to_string()
                        is_disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_dragging=true
                        is_loupe_visible=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-thumb-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                code_signal=controlled_vs_uncontrolled_code
                code_imports=color_thumb_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">
                        "ColorThumb has no value/default_value/on_value_change axis; both examples are external-props driven."
                    </span>
                    <div class="docs-row" data-slot="color-thumb-controlled-vs-uncontrolled">
                        <div class="docs-card">
                            <div class="ui-muted">"External props path A"</div>
                            <div style=board_style>
                                <ColorThumb
                                    id_base="docs-color-thumb-controlled-like".to_string()
                                    color="#10b981".to_string()
                                    is_focused=true
                                    x_percent=48.0
                                    y_percent=46.0
                                />
                            </div>
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"External props path B"</div>
                            <div style=board_style>
                                <ColorThumb
                                    id_base="docs-color-thumb-uncontrolled-like".to_string()
                                    color="#10b981".to_string()
                                    is_focused=true
                                    x_percent=48.0
                                    y_percent=46.0
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                code_signal=output_mode_code
                code_imports=color_thumb_imports.clone()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-thumb-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorThumb is not a text-reading surface; docs output stays snapshot (`fallback=snapshot`)."
                    </span>
                    <div style=board_style>
                        <ColorThumb
                            id_base="docs-color-thumb-snapshot".to_string()
                            color="#334155".to_string()
                            x_percent=44.0
                            y_percent=56.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Interactive Workbench (DX)"
                description="在线 props/状态调节 + key-flow 回放 + Spec JSON 联动预览；作为可重复验收面。"
                code_signal=workbench_code
                code_imports=color_thumb_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-thumb/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-thumb-workbench-controls">
                        <label class="docs-search__label" data-slot="color-thumb-workbench-input-color-label">
                            "Color"
                        </label>
                        <input
                            type="text"
                            data-slot="color-thumb-workbench-input-color"
                            prop:value=move || workbench_color.get()
                            on:input=move |ev| set_workbench_color.set(event_target_value(&ev))
                        />

                        <label class="docs-search__label" data-slot="color-thumb-workbench-input-x-label">
                            "X Percent"
                        </label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            step="1"
                            data-slot="color-thumb-workbench-input-x"
                            prop:value=move || workbench_x_percent.get().to_string()
                            on:input=move |ev| {
                                if let Ok(next) = event_target_value(&ev).parse::<f32>() {
                                    set_workbench_x_percent.set(next.clamp(0.0, 100.0));
                                }
                            }
                        />

                        <label class="docs-search__label" data-slot="color-thumb-workbench-input-y-label">
                            "Y Percent"
                        </label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            step="1"
                            data-slot="color-thumb-workbench-input-y"
                            prop:value=move || workbench_y_percent.get().to_string()
                            on:input=move |ev| {
                                if let Ok(next) = event_target_value(&ev).parse::<f32>() {
                                    set_workbench_y_percent.set(next.clamp(0.0, 100.0));
                                }
                            }
                        />

                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_focused set_checked=set_workbench_focused>
                            "Focused"
                        </Switch>
                        <Switch checked=workbench_dragging set_checked=set_workbench_dragging>
                            "Dragging"
                        </Switch>
                        <Switch checked=workbench_loupe_visible set_checked=set_workbench_loupe_visible>
                            "Show Loupe"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>

                        <div class="docs-stack docs-stack--tight" data-slot="color-thumb-workbench-replay-controls">
                            <button
                                type="button"
                                data-slot="color-thumb-workbench-replay-idle"
                                on:click=move |_| {
                                    set_workbench_disabled.set(false);
                                    set_workbench_focused.set(false);
                                    set_workbench_dragging.set(false);
                                    set_workbench_loupe_visible.set(true);
                                    set_workbench_x_percent.set(44.0);
                                    set_workbench_y_percent.set(56.0);
                                    set_workbench_replay_count.update(|count| *count += 1);
                                }
                            >
                                "Replay Idle Path"
                            </button>
                            <button
                                type="button"
                                data-slot="color-thumb-workbench-replay-drag"
                                on:click=move |_| {
                                    set_workbench_disabled.set(false);
                                    set_workbench_focused.set(true);
                                    set_workbench_dragging.set(true);
                                    set_workbench_loupe_visible.set(false);
                                    set_workbench_x_percent.set(78.0);
                                    set_workbench_y_percent.set(26.0);
                                    set_workbench_replay_count.update(|count| *count += 1);
                                }
                            >
                                "Replay Drag Path"
                            </button>
                        </div>

                        <label class="docs-search__label" data-slot="color-thumb-workbench-spec-input-label">
                            "Spec JSON"
                        </label>
                        <textarea
                            class="playground__test-editor"
                            data-slot="color-thumb-workbench-spec-input"
                            prop:value=move || workbench_spec_input.get()
                            on:input=move |ev| set_workbench_spec_input.set(event_target_value(&ev))
                        ></textarea>
                    </div>
                }
            >
                {move || {
                    let parsed_spec = workbench_spec.get();
                    let spec_status = if parsed_spec.is_some() {
                        "ok"
                    } else {
                        "invalid-json"
                    };
                    let spec_color = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.color.clone())
                        .unwrap_or_else(|| "#0ea5e9".to_string());
                    let spec_x = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.x_percent)
                        .unwrap_or(66.0);
                    let spec_y = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.y_percent)
                        .unwrap_or(34.0);
                    let spec_disabled = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.is_disabled)
                        .unwrap_or(false);
                    let spec_focused = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.is_focused)
                        .unwrap_or(true);
                    let spec_dragging = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.is_dragging)
                        .unwrap_or(false);
                    let spec_loupe_visible = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.is_loupe_visible)
                        .unwrap_or(true);
                    let spec_class_name = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.class_name.clone())
                        .unwrap_or_default();

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-thumb-workbench">
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="color-thumb-workbench-canvas">
                                <div style=board_style>
                                    <ColorThumb
                                        id_base="docs-color-thumb-workbench".to_string()
                                        color=workbench_color.get()
                                        is_disabled=workbench_disabled.get()
                                        is_focused=workbench_focused.get()
                                        is_dragging=workbench_dragging.get()
                                        is_loupe_visible=workbench_loupe_visible.get()
                                        x_percent=workbench_x_percent.get()
                                        y_percent=workbench_y_percent.get()
                                        class_name=if workbench_custom_class.get() {
                                            "docs-color-thumb-workbench".to_string()
                                        } else {
                                            String::new()
                                        }
                                    />
                                </div>
                            </div>

                            <div class="docs-card docs-stack docs-stack--tight" data-slot="color-thumb-workbench-spec-preview">
                                <div class="ui-muted">
                                    "Spec input -> preview"
                                </div>
                                <div style=board_style>
                                    <ColorThumb
                                        id_base="docs-color-thumb-workbench-spec".to_string()
                                        color=spec_color
                                        is_disabled=spec_disabled
                                        is_focused=spec_focused
                                        is_dragging=spec_dragging
                                        is_loupe_visible=spec_loupe_visible
                                        x_percent=spec_x
                                        y_percent=spec_y
                                        class_name=spec_class_name
                                    />
                                </div>
                                <span class="ui-muted" data-slot="color-thumb-workbench-spec-state">
                                    "spec: " {spec_status}
                                </span>
                            </div>

                            <span class="ui-muted" data-slot="color-thumb-workbench-state">
                                "x: " {format!("{:.1}", workbench_x_percent.get())}
                                " · y: " {format!("{:.1}", workbench_y_percent.get())}
                                " · state: " {if workbench_disabled.get() {
                                    "disabled"
                                } else if workbench_dragging.get() {
                                    "dragging"
                                } else if workbench_focused.get() {
                                    "focused"
                                } else {
                                    "idle"
                                }}
                                " · replay: " {workbench_replay_count.get()}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                code_signal=source_first_code
                code_imports=color_thumb_imports.clone()
            >
                <div style=board_style>
                    <ColorThumb
                        id_base="docs-color-thumb-source-first".to_string()
                        color="#14b8a6".to_string()
                        is_focused=true
                        x_percent=58.0
                        y_percent=38.0
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="color-thumb-source-first-contract">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps snippet imports synchronized with "
                    <code>"code_imports"</code>
                    "."
                </p>
                <p data-slot="color-thumb-source-first-copy-action">
                    "Source-first usage stays copy-ready via "
                    <code>"Show code + Copy"</code>
                    ", and copied snippets are expected to run without manual import patching."
                </p>
                <p data-slot="color-thumb-source-paths">
                    "Source-first path: "
                    <code>"components/color-thumb/src/mod.rs"</code>
                    ", "
                    <code>"components/color-thumb/src/view.rs"</code>
                    ", "
                    <code>"components/color-thumb/src/logic.rs"</code>
                    ", "
                    <code>"components/color-thumb/src/styles.rs"</code>
                    ", "
                    <code>"components/color-thumb/src/motion.rs"</code>
                    "."
                </p>
                <p data-slot="color-thumb-source-prerequisites">
                    "Prerequisites: enable "
                    <code>"component-color_thumb"</code>
                    " (and "
                    <code>"inject-css"</code>
                    " when runtime CSS injection is required) so copied snippets compile and render as expected."
                </p>
            </section>
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

    let hello_code = Signal::derive(move || {
        r##"<ColorEditor
  id_base="docs-color-editor-hello".to_string()
/>"##
            .to_string()
    });

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
  is_alpha_channel_hidden=true
  is_disabled=true
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

    let state_matrix_code = Signal::derive(move || {
        r##"<div class="docs-row" data-slot="color-editor-state-matrix">
  <div class="docs-card">
    <div class="ui-muted">"Ready"</div>
    <ColorEditor
      id_base="docs-color-editor-matrix-ready".to_string()
      label="Ready".to_string()
      default_selected_color="#4f46e5".to_string()
      default_format=ColorEditorFormat::Hex
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Empty"</div>
    <ColorEditor
      id_base="docs-color-editor-matrix-empty".to_string()
      label="Empty".to_string()
      default_format=ColorEditorFormat::Hsl
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Disabled + Hidden Alpha"</div>
    <ColorEditor
      id_base="docs-color-editor-matrix-disabled".to_string()
      label="Disabled".to_string()
      default_selected_color="#0ea5e9".to_string()
      default_format=ColorEditorFormat::Rgb
      is_alpha_channel_hidden=true
      is_disabled=true
      motion=ColorSliderMotion::disabled()
    />
  </div>
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"let (selected_color, set_selected_color) = signal(Some("#4f46e5".to_string()));
let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));

let (format, set_format) = signal(ColorEditorFormat::Hex);
let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));

<div class="docs-row">
  <div class="docs-card">
    <ColorEditor
      id_base="docs-color-editor-controlled".to_string()
      label="Controlled".to_string()
      selected_color=selected_color.into()
      on_selected_change=on_selected_change
      format=format.into()
      on_format_change=on_format_change
    />
  </div>
  <div class="docs-card">
    <ColorEditor
      id_base="docs-color-editor-uncontrolled".to_string()
      label="Uncontrolled".to_string()
      default_selected_color="#22c55e".to_string()
      default_format=ColorEditorFormat::Rgb
      default_hue=132.0
      default_alpha=92.0
      default_area=(0.62, 0.18)
    />
  </div>
</div>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorEditor is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  <ColorEditor
    id_base="docs-color-editor-snapshot".to_string()
    label="Snapshot fallback".to_string()
    default_selected_color="#334155".to_string()
    default_format=ColorEditorFormat::Hex
  />
</div>"##
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
            "let (selected_color, set_selected_color) = signal(Some(\"#4f46e5\".into()));".to_string(),
            "let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));".to_string(),
            String::new(),
            format!("let (format, set_format) = signal({format_literal});"),
            "let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));".to_string(),
            String::new(),
            "<ColorEditor".to_string(),
            "  id_base=\"docs-color-editor-workbench\".into()".to_string(),
            "  selected_color=selected_color.into()".to_string(),
            "  on_selected_change=on_selected_change".to_string(),
            "  format=format.into()".to_string(),
            "  on_format_change=on_format_change".to_string(),
        ];

        if workbench_custom_label.get() {
            lines.push("  label=\"Brand color workspace\".into()".to_string());
        }
        if workbench_disabled.get() {
            lines.push("  is_disabled=true".to_string());
        }
        if workbench_hide_alpha.get() {
            lines.push("  is_alpha_channel_hidden=true".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-color-editor-workbench\".into()".to_string());
        }
        if workbench_reduced_motion.get() {
            lines.push("  motion=ColorSliderMotion::disabled()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/color-editor/src/styles.rs */\n{}",
            ui_components::color::editor::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let format = workbench_format.get();
        let is_disabled = workbench_disabled.get();
        let is_alpha_channel_hidden = workbench_hide_alpha.get();
        let has_selection = workbench_selected_color.get().is_some();
        let custom_label = workbench_custom_label.get();
        let custom_class = workbench_custom_class.get();
        let reduced_motion = workbench_reduced_motion.get();

        let data_state = if is_disabled {
            "disabled"
        } else if has_selection {
            "ready"
        } else {
            "empty"
        };
        let data_alpha = if is_alpha_channel_hidden {
            "hidden"
        } else {
            "visible"
        };
        let data_motion_source = if reduced_motion { "custom" } else { "default" };
        let data_label_source = if custom_label { "custom" } else { "default" };
        let data_class_source = if custom_class { "custom" } else { "default" };

        let mut classes = vec!["ui-color-editor".to_string(), format.class_name().into()];
        if is_disabled {
            classes.push("ui-color-editor--disabled".to_string());
        }
        if is_alpha_channel_hidden {
            classes.push("ui-color-editor--alpha-hidden".to_string());
        }
        if custom_class {
            classes.push("ui-color-editor--custom-class".to_string());
            classes.push("docs-color-editor-workbench".to_string());
        }

        format!(
            "ColorEditorActualConfig {{\n  format: {format:?},\n  is_disabled: {is_disabled},\n  is_alpha_channel_hidden: {is_alpha_channel_hidden},\n  has_selection: {has_selection},\n  custom_label: {custom_label},\n  custom_class: {custom_class},\n  reduced_motion: {reduced_motion},\n  data_state: \"{data_state}\",\n  data_alpha: \"{data_alpha}\",\n  data_motion_source: \"{data_motion_source}\",\n  data_label_source: \"{data_label_source}\",\n  data_class_source: \"{data_class_source}\",\n  class: \"{}\",\n}}",
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
            <Playground title="Hello World（默认路径）" code_signal=hello_code>
                <ColorEditor id_base="docs-color-editor-hello".to_string() />
            </Playground>

            <Playground title="State Matrix" code_signal=state_matrix_code>
                <div class="docs-row" data-slot="color-editor-state-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Ready"</div>
                        <ColorEditor
                            id_base="docs-color-editor-matrix-ready".to_string()
                            label="Ready".to_string()
                            default_selected_color="#4f46e5".to_string()
                            default_format=ColorEditorFormat::Hex
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Empty"</div>
                        <ColorEditor
                            id_base="docs-color-editor-matrix-empty".to_string()
                            label="Empty".to_string()
                            default_format=ColorEditorFormat::Hsl
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Disabled + Hidden Alpha"</div>
                        <ColorEditor
                            id_base="docs-color-editor-matrix-disabled".to_string()
                            label="Disabled".to_string()
                            default_selected_color="#0ea5e9".to_string()
                            default_format=ColorEditorFormat::Rgb
                            is_alpha_channel_hidden=true
                            is_disabled=true
                            motion=ColorSliderMotion::disabled()
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Controlled vs Uncontrolled" code_signal=controlled_vs_uncontrolled_code>
                <div class="docs-row" data-slot="color-editor-controlled-vs-uncontrolled">
                    <div class="docs-card">
                        <div class="ui-muted">"Controlled"</div>
                        <ColorEditor
                            id_base="docs-color-editor-controlled".to_string()
                            label="Controlled".to_string()
                            selected_color=selected_color_signal
                            on_selected_change=on_selected_change
                            format=format_signal
                            on_format_change=on_format_change
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Uncontrolled"</div>
                        <ColorEditor
                            id_base="docs-color-editor-uncontrolled".to_string()
                            label="Uncontrolled".to_string()
                            default_selected_color="#22c55e".to_string()
                            default_format=ColorEditorFormat::Rgb
                            default_hue=132.0
                            default_alpha=92.0
                            default_area=(0.62, 0.18)
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Streaming Optional / Snapshot" code_signal=output_mode_code>
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-editor-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorEditor is not a text-reading surface; docs output stays snapshot (`fallback=snapshot`)."
                    </span>
                    <ColorEditor
                        id_base="docs-color-editor-snapshot".to_string()
                        label="Snapshot fallback".to_string()
                        default_selected_color="#334155".to_string()
                        default_format=ColorEditorFormat::Hex
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-editor/src/styles.rs".to_string()
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
                                        is_alpha_channel_hidden=workbench_hide_alpha.get()
                                        is_disabled=workbench_disabled.get()
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
                                        is_alpha_channel_hidden=true
                                        is_disabled=true
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
                        is_alpha_channel_hidden=true
                        is_disabled=true
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

            <section class="docs-card docs-prose" data-slot="color-editor-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps snippet imports synchronized with "
                    <code>"code_imports"</code>
                    "."
                </p>
                <p>
                    "Source-first path: "
                    <code>"components/color-editor/src/mod.rs"</code>
                    ", "
                    <code>"components/color-editor/src/view.rs"</code>
                    ", "
                    <code>"components/color-editor/src/logic.rs"</code>
                    ", "
                    <code>"components/color-editor/src/styles.rs"</code>
                    ", "
                    <code>"components/color-editor/src/motion.rs"</code>
                    "."
                </p>
                <p data-slot="color-editor-source-prerequisites">
                    "Prerequisites: enable "
                    <code>"component-color_editor"</code>
                    " (and "
                    <code>"inject-css"</code>
                    " when runtime CSS injection is required) so copied snippets compile and render as expected."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_handle() -> AnyView {
    let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

    let hello_code = Signal::derive(move || {
        r##"let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

<div style=surface_style>
  <ColorHandle
    id_base="docs-color-handle-hello".to_string()
    color="#f59e0b".to_string()
  />
</div>"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
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
    is_focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorHandle
    id_base="docs-color-handle-dragging".to_string()
    color="#3b82f6".to_string()
    is_dragging=true
    x_percent=82.0
    y_percent=28.0
  />
  <ColorHandle
    id_base="docs-color-handle-disabled".to_string()
    color="#a78bfa".to_string()
    is_disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorHandle
    id_base="docs-color-handle-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_dragging=true
    is_loupe_visible=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-handle-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"// ColorHandle does not expose a value/default_value/on_value_change axis.
// Controlled vs uncontrolled is N/A; both examples are external-props driven.
let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

<div class="docs-row">
  <div style=surface_style>
    <ColorHandle
      id_base="docs-color-handle-controlled-like".to_string()
      color="#10b981".to_string()
      is_focused=true
      x_percent=48.0
      y_percent=46.0
    />
  </div>
  <div style=surface_style>
    <ColorHandle
      id_base="docs-color-handle-uncontrolled-like".to_string()
      color="#10b981".to_string()
      is_focused=true
      x_percent=48.0
      y_percent=46.0
    />
  </div>
</div>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorHandle is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "ColorHandle docs output mode: snapshot"
</div>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<div style=surface_style>
  <ColorHandle
    id_base="docs-color-handle-disabled".to_string()
    color="#a78bfa".to_string()
    is_disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorHandle
    id_base="docs-color-handle-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_dragging=true
    is_loupe_visible=false
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
            "  class_name=\"docs-color-handle-custom\".into()\n".to_string()
        } else {
            String::new()
        };
        format!(
            "<ColorHandle\n  id_base=\"docs-color-handle-workbench\".into()\n  color=\"{color}\".into()\n  x_percent={:.1}\n  y_percent={:.1}\n  is_disabled={}\n  is_focused={}\n  is_dragging={}\n  is_loupe_visible={}\n  motion=ColorHandleMotion {{ duration_ms: {} }}\n{class_name_line}/>",
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
            "/* components/color-handle/src/styles.rs */\n{}",
            ui_components::color::handle::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let is_disabled = workbench_disabled.get();
        let is_focused = workbench_focused.get();
        let is_dragging = workbench_dragging.get();
        let is_loupe_visible = workbench_show_loupe.get();
        let has_custom_class = workbench_custom_class.get();
        let state = if is_disabled {
            "disabled"
        } else if is_dragging {
            "dragging"
        } else if is_focused {
            "focused"
        } else {
            "color"
        };
        let loupe_visible = !is_disabled && is_loupe_visible && is_dragging;
        let mut classes = vec!["ui-color-handle".to_string()];
        if is_disabled {
            classes.push("ui-color-handle--disabled".to_string());
        }
        if is_focused {
            classes.push("ui-color-handle--focused".to_string());
        }
        if is_dragging {
            classes.push("ui-color-handle--dragging".to_string());
        }
        if has_custom_class {
            classes.push("ui-color-handle--custom-class".to_string());
            classes.push("docs-color-handle-custom".to_string());
        }
        format!(
            "ColorHandleActualConfig {{\n  state: \"{state}\",\n  x_percent: {:.1},\n  y_percent: {:.1},\n  is_disabled: {is_disabled},\n  is_focused: {is_focused},\n  is_dragging: {is_dragging},\n  is_loupe_visible: {is_loupe_visible},\n  loupe_visible: {loupe_visible},\n  motion_duration_ms: {},\n  class: \"{}\",\n}}",
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
            <Playground title="Hello World" code_signal=hello_code>
                <div style=surface_style>
                    <ColorHandle
                        id_base="docs-color-handle-hello".to_string()
                        color="#f59e0b".to_string()
                    />
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=state_matrix_code>
                <div style=surface_style data-slot="color-handle-state-matrix">
                    <ColorHandle
                        id_base="docs-color-handle-idle".to_string()
                        color="#f59e0b".to_string()
                        x_percent=22.0
                        y_percent=72.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-focused".to_string()
                        color="#10b981".to_string()
                        is_focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-dragging".to_string()
                        color="#3b82f6".to_string()
                        is_dragging=true
                        x_percent=82.0
                        y_percent=28.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-disabled".to_string()
                        color="#a78bfa".to_string()
                        is_disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_dragging=true
                        is_loupe_visible=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-handle-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                code_signal=controlled_vs_uncontrolled_code
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">
                        "ColorHandle has no value/default_value/on_value_change axis; both examples are external-props driven."
                    </span>
                    <div class="docs-row">
                        <div class="docs-card">
                            <div class="ui-muted">"External props path A"</div>
                            <div style=surface_style>
                                <ColorHandle
                                    id_base="docs-color-handle-controlled-like".to_string()
                                    color="#10b981".to_string()
                                    is_focused=true
                                    x_percent=48.0
                                    y_percent=46.0
                                />
                            </div>
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"External props path B"</div>
                            <div style=surface_style>
                                <ColorHandle
                                    id_base="docs-color-handle-uncontrolled-like".to_string()
                                    color="#10b981".to_string()
                                    is_focused=true
                                    x_percent=48.0
                                    y_percent=46.0
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground title="Streaming Optional / Snapshot" code_signal=output_mode_code>
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-handle-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorHandle is not a text-reading surface; docs output stays snapshot (`fallback=snapshot`)."
                    </span>
                    <div style=surface_style>
                        <ColorHandle
                            id_base="docs-color-handle-snapshot".to_string()
                            color="#334155".to_string()
                            x_percent=44.0
                            y_percent=56.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Disabled + Custom Class + Loupe Off" code_signal=states_code>
                <div style=surface_style>
                    <ColorHandle
                        id_base="docs-color-handle-disabled".to_string()
                        color="#a78bfa".to_string()
                        is_disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_dragging=true
                        is_loupe_visible=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-handle-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Parameter Matrix Workbench (Display + Config + Code + CSS Test)"
                description="参数矩阵 + 状态矩阵联动工作台：交互配置、copy-ready code、scoped CSS test。"
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-handle/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div
                        class="docs-stack docs-stack--tight"
                        data-slot="color-handle-workbench-controls"
                        data-parameter-matrix="color-handle"
                    >
                        <label class="docs-search__label">
                            "Color"
                            <select
                                data-slot="color-handle-workbench-color"
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
                                data-slot="color-handle-workbench-x"
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
                                data-slot="color-handle-workbench-y"
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
                                data-slot="color-handle-workbench-disabled"
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                data-slot="color-handle-workbench-focused"
                                type="checkbox"
                                prop:checked=move || workbench_focused.get()
                                on:change=move |ev| set_workbench_focused.set(event_target_checked(&ev))
                            />
                            " Focused"
                        </label>
                        <label class="docs-search__label">
                            <input
                                data-slot="color-handle-workbench-dragging"
                                type="checkbox"
                                prop:checked=move || workbench_dragging.get()
                                on:change=move |ev| set_workbench_dragging.set(event_target_checked(&ev))
                            />
                            " Dragging"
                        </label>
                        <label class="docs-search__label">
                            <input
                                data-slot="color-handle-workbench-show-loupe"
                                type="checkbox"
                                prop:checked=move || workbench_show_loupe.get()
                                on:change=move |ev| set_workbench_show_loupe.set(event_target_checked(&ev))
                            />
                            " Show loupe"
                        </label>
                        <label class="docs-search__label">
                            <input
                                data-slot="color-handle-workbench-custom-class"
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            "Motion (ms) "
                            <input
                                data-slot="color-handle-workbench-motion"
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
                    <span class="ui-muted" data-slot="color-handle-api-defaults">
                        "API defaults: is_loupe_visible=true, x_percent=50.0, y_percent=50.0, motion=ColorHandleMotion::default()."
                    </span>
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
                                            is_disabled=workbench_disabled.get()
                                            is_focused=workbench_focused.get()
                                            is_dragging=workbench_dragging.get()
                                            is_loupe_visible=workbench_show_loupe.get()
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

            <section class="docs-card docs-prose" data-slot="color-handle-copy-ready">
                <h3>"Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p>
                    "Source-first path: "
                    <code>"components/color-handle/src/view.rs"</code>
                    ", "
                    <code>"components/color-handle/src/logic.rs"</code>
                    ", "
                    <code>"components/color-handle/src/styles.rs"</code>
                    "."
                </p>
            </section>
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

    let hello_code = Signal::derive(move || {
        r##"<div style=surface_style>
  <ColorLoupe id_base="docs-color-loupe-hello".to_string() is_open=true />
</div>"##
            .to_string()
    });

    let basic_code = Signal::derive(move || {
        r##"let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm);";

<div style=surface_style>
  <ColorLoupe
    id_base="docs-color-loupe-start".to_string()
    color="#f59e0b".to_string()
    is_open=true
    x_percent=18.0
    y_percent=74.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-center".to_string()
    color="#10b981".to_string()
    is_open=true
    x_percent=50.0
    y_percent=48.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-end".to_string()
    color="#3b82f6".to_string()
    is_open=true
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
    is_open=true
    is_disabled=true
    x_percent=32.0
    y_percent=58.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_open=true
    x_percent=72.0
    y_percent=36.0
    aria_label="Accent loupe".to_string()
    class_name="docs-color-loupe-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"// ColorLoupe is snapshot-only: no internal mutable state axis.
// Controlled/uncontrolled state ownership is N/A.
<ColorLoupe
  id_base="docs-color-loupe-controlled".to_string()
  color="#10b981".to_string()
  is_open=true
  x_percent=50.0
  y_percent=48.0
/>
<ColorLoupe id_base="docs-color-loupe-uncontrolled-na".to_string() />"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<ColorLoupe
  id_base="docs-color-loupe-matrix-default".to_string()
/>
<ColorLoupe
  id_base="docs-color-loupe-matrix-open".to_string()
  color="#f59e0b".to_string()
  is_open=true
  x_percent=18.0
  y_percent=74.0
/>
<ColorLoupe
  id_base="docs-color-loupe-matrix-disabled".to_string()
  color="#a78bfa".to_string()
  is_open=true
  is_disabled=true
  x_percent=32.0
  y_percent=58.0
/>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  <ColorLoupe
    id_base="docs-color-loupe-snapshot".to_string()
    is_open=true
    output_state=ColorLoupeOutputState::Verified
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
        let is_open_value = is_open.get();
        let is_disabled_value = is_disabled.get();
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
            "<ColorLoupe\n  id_base=\"docs-color-loupe-workbench\".into()\n  color=\"{color}\".into() // {color_label}\n  is_open={is_open_value}\n  is_disabled={is_disabled_value}\n  x_percent={x_percent}\n  y_percent={y_percent} // {position_label}\n  aria_label=\"{aria_label}\".into()\n  class_name=\"{class_name}\".into()\n/>"
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
        let is_open_value = is_open.get();
        let is_disabled_value = is_disabled.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let aria_label = if custom_aria { "Workbench loupe" } else { "" };
        let class_name = if custom_class {
            "docs-color-loupe-workbench"
        } else {
            ""
        };

        format!(
            "ColorLoupeActualConfig {{\n  color: \"{color}\" ({color_label}),\n  position: \"{position_label}\" ({x_percent:.1}, {y_percent:.1}),\n  is_open: {is_open_value},\n  is_disabled: {is_disabled_value},\n  aria_label: \"{aria_label}\",\n  class_name: \"{class_name}\",\n}}"
        )
    });

    view! {
        <ComponentPage
            title="ColorLoupe"
            slug="color-loupe"
            group="Forms"
            description="baseline-compatible color loupe overlay primitive with centralized is_open/is_disabled/position normalization, checkerboard alpha preview, and stable slot/data-state contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div style=surface_style>
                    <ColorLoupe id_base="docs-color-loupe-hello".to_string() is_open=true />
                </div>
            </Playground>

            <Playground title="Open + Position Buckets" code_signal=basic_code>
                <div style=surface_style>
                    <ColorLoupe
                        id_base="docs-color-loupe-start".to_string()
                        color="#f59e0b".to_string()
                        is_open=true
                        x_percent=18.0
                        y_percent=74.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-center".to_string()
                        color="#10b981".to_string()
                        is_open=true
                        x_percent=50.0
                        y_percent=48.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-end".to_string()
                        color="#3b82f6".to_string()
                        is_open=true
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
                        is_open=true
                        is_disabled=true
                        x_percent=32.0
                        y_percent=58.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_open=true
                        x_percent=72.0
                        y_percent=36.0
                        aria_label="Accent loupe".to_string()
                        class_name="docs-color-loupe-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled（N/A）"
                code_signal=controlled_vs_uncontrolled_code
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">
                        "ColorLoupe 是 snapshot 展示组件，不持有内部可变状态轴；受控/非受控切换语义 N/A。"
                    </span>
                    <div style=surface_style>
                        <ColorLoupe
                            id_base="docs-color-loupe-controlled".to_string()
                            color="#10b981".to_string()
                            is_open=true
                            x_percent=50.0
                            y_percent=48.0
                        />
                        <ColorLoupe id_base="docs-color-loupe-uncontrolled-na".to_string() />
                    </div>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=state_matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">"default / open / disabled"</span>
                    <div style=surface_style>
                        <ColorLoupe id_base="docs-color-loupe-matrix-default".to_string() />
                        <ColorLoupe
                            id_base="docs-color-loupe-matrix-open".to_string()
                            color="#f59e0b".to_string()
                            is_open=true
                            x_percent=18.0
                            y_percent=74.0
                        />
                        <ColorLoupe
                            id_base="docs-color-loupe-matrix-disabled".to_string()
                            color="#a78bfa".to_string()
                            is_open=true
                            is_disabled=true
                            x_percent=32.0
                            y_percent=58.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Streaming Optional / Snapshot" code_signal=output_mode_code>
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-loupe-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorLoupe is not a text-reading surface; docs output mode stays snapshot (`fallback=snapshot`)."
                    </span>
                    <div style=surface_style>
                        <ColorLoupe
                            id_base="docs-color-loupe-snapshot-draft".to_string()
                            color="#f59e0b".to_string()
                            is_open=true
                            x_percent=18.0
                            y_percent=74.0
                            output_state=ColorLoupeOutputState::Draft
                        />
                        <ColorLoupe
                            id_base="docs-color-loupe-snapshot-verified".to_string()
                            color="#10b981".to_string()
                            is_open=true
                            x_percent=50.0
                            y_percent=48.0
                            output_state=ColorLoupeOutputState::Verified
                        />
                        <ColorLoupe
                            id_base="docs-color-loupe-snapshot-committable".to_string()
                            color="#3b82f6".to_string()
                            is_open=true
                            x_percent=82.0
                            y_percent=24.0
                            output_state=ColorLoupeOutputState::Committable
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_source_path="components/color-loupe/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-loupe-workbench-controls">
                        <div data-slot="color-loupe-workbench-color">
                            <div class="docs-search__label">"Color"</div>
                            <SegmentedControl
                                id_base="docs-color-loupe-workbench-color".to_string()
                                options=color_options.clone()
                                selected_index=color_index
                                set_selected_index=set_color_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorLoupe workbench color".to_string()
                            />
                        </div>

                        <div data-slot="color-loupe-workbench-position">
                            <div class="docs-search__label">"Position bucket"</div>
                            <SegmentedControl
                                id_base="docs-color-loupe-workbench-position".to_string()
                                options=position_options.clone()
                                selected_index=position_index
                                set_selected_index=set_position_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorLoupe workbench position".to_string()
                            />
                        </div>

                        <div data-slot="color-loupe-workbench-open">
                            <Switch checked=is_open set_checked=set_is_open>"Open"</Switch>
                        </div>
                        <div data-slot="color-loupe-workbench-disabled">
                            <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                        </div>
                        <div data-slot="color-loupe-workbench-custom-aria">
                            <Switch checked=custom_aria set_checked=set_custom_aria>
                                "Custom aria_label"
                            </Switch>
                        </div>
                        <div data-slot="color-loupe-workbench-custom-class">
                            <Switch checked=custom_class set_checked=set_custom_class>
                                "Custom class"
                            </Switch>
                        </div>
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
                    let is_open_value = is_open.get();
                    let is_disabled_value = is_disabled.get();
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
                        <div class="docs-stack docs-stack--tight" data-slot="color-loupe-workbench-canvas">
                            <div style=surface_style data-slot="color-loupe-workbench-surface">
                                <ColorLoupe
                                    id_base="docs-color-loupe-workbench-main".to_string()
                                    color=color
                                    is_open=is_open_value
                                    is_disabled=is_disabled_value
                                    x_percent=x_percent
                                    y_percent=y_percent
                                    aria_label=aria_label
                                    class_name=class_name
                                />
                                <ColorLoupe
                                    id_base="docs-color-loupe-workbench-compare".to_string()
                                    color="#3b82f6".to_string()
                                    is_open=true
                                    x_percent=82.0
                                    y_percent=24.0
                                    aria_label="Comparison loupe".to_string()
                                />
                            </div>
                            <span class="ui-muted">
                                "左侧可调，右侧固定对照（blue/end/is_open）。"
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <section class="docs-card docs-prose" data-slot="color-loupe-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p>
                    "Dependency prerequisites: enable "
                    <code>"ui-components features: component-color_loupe + inject-css"</code>
                    " and render inside "
                    <code>"UiRoot"</code>
                    " so copied snippets keep theme vars/components css injection."
                </p>
                <p>
                    "Source-first path: "
                    <code>"components/color-loupe/src/view.rs"</code>
                    ", "
                    <code>"components/color-loupe/src/logic.rs"</code>
                    ", "
                    <code>"components/color-loupe/src/styles.rs"</code>
                    "."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
