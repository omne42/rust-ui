use super::*;

pub(crate) fn color_area() -> AnyView {
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
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::color::area::ColorAreaMotion { duration_ms: 320.0 }
        } else {
            ui::color::area::ColorAreaMotion::default()
        }
    });

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
        let motion = workbench_motion.get();

        format!(
            "<ColorArea\n  id_base=\"docs-color-area-workbench-main\".into()\n  label=\"Color workbench\".into()\n  default_value=({:.2}, {:.2})\n  grid_size={}\n  step={:.2}\n  is_disabled={}\n  on_value_change=on_value_change\n  preview_color=\"{}\".into()\n  motion=ui::color::area::ColorAreaMotion {{ duration_ms: {:.1} }}\n  aria_label=\"Color workbench area\".into()\n  x_axis_label=\"{}\".into()\n  y_axis_label=\"{}\".into()\n  class_name=\"{}\".into()\n  lang={}.to_string()\n  dir={}\n/>",
            default_value.0,
            default_value.1,
            grid_size,
            step,
            disabled,
            if show_preview { preview_color } else { "" },
            motion.duration_ms,
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
            rust_string_literal(if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            }),
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
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
        let motion = workbench_motion.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        format!(
            "ColorAreaActualConfig {{\n  id_base: \"docs-color-area-workbench-main\",\n  label: Some(\"Color workbench\"),\n  is_disabled: Some({is_disabled}),\n  value: None,\n  default_value: Some(({default_x:.2}, {default_y:.2})) [{position}],\n  on_value_change: Some(\"on_value_change\"),\n  step: Some({step:.2}),\n  grid_size: Some({grid_size}),\n  preview_color: {preview_color},\n  motion: ColorAreaMotion {{ duration_ms: {motion_duration:.1} }},\n  aria_label: Some(\"Color workbench area\"),\n  x_axis_label: {x_axis_label},\n  y_axis_label: {y_axis_label},\n  class_name: {class_name},\n  lang: Some({lang:?}),\n  dir: Some({dir}),\n  show_preview: {show_preview},\n}}",
            is_disabled = is_disabled.get(),
            default_x = default_value.0,
            default_y = default_value.1,
            position = default_value.2,
            step = step,
            grid_size = grid_size,
            preview_color = if show_preview.get() {
                format!("Some({preview_color:?})")
            } else {
                "None".to_string()
            },
            motion_duration = motion.duration_ms,
            x_axis_label = if custom_axis_labels.get() {
                "Some(\"Saturation (X)\")"
            } else {
                "None"
            },
            y_axis_label = if custom_axis_labels.get() {
                "Some(\"Lightness (Y)\")"
            } else {
                "None"
            },
            class_name = if custom_class.get() {
                "Some(\"docs-color-area-workbench\")"
            } else {
                "None"
            },
            lang = lang,
            dir = dir,
            show_preview = show_preview.get(),
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/color-area/src/styles.rs */\n{}",
            ui::color::area::styles::CSS
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
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "dir=rtl"
                        </Switch>
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
                                        on_value_change=on_value_change
                                        grid_size=grid_size
                                        step=step
                                        is_disabled=is_disabled.get()
                                        preview_color=if show_preview.get() {
                                            preview_color.clone()
                                        } else {
                                            String::new()
                                        }
                                        motion=workbench_motion.get()
                                        aria_label="Color workbench area".to_string()
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
                                        lang=if workbench_lang_zh.get() {
                                            "zh-CN".to_string()
                                        } else {
                                            "en-US".to_string()
                                        }
                                        dir=if workbench_rtl.get() {
                                            A11yDirection::Rtl
                                        } else {
                                            A11yDirection::Ltr
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
                                " callback value="
                                {move || {
                                    let (x, y) = value.get();
                                    format!("({x:.2}, {y:.2})")
                                }}
                            </span>
                        </div>
                    }
                }}
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
                title="State Matrix (Controlled / Dense / Disabled)"
                code_signal=state_matrix_code
            >
                <div class="docs-row docs-row--wrap" style="gap: var(--ui-space-lg); align-items: flex-start;">
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Controlled"</span>
                        <ColorArea
                            id_base="docs-color-area-matrix-controlled-v2".to_string()
                            label="Controlled".to_string()
                            value=value.into()
                            on_value_change=on_value_change
                            aria_label="Controlled color area".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Dense grid"</span>
                        <ColorArea
                            id_base="docs-color-area-matrix-dense-v2".to_string()
                            label="Dense grid".to_string()
                            default_value=(0.25, 0.85)
                            grid_size=15
                            step=0.05
                            preview_color="#a78bfa".to_string()
                            motion=ui::color::area::ColorAreaMotion { duration_ms: 320.0 }
                            aria_label="Dense grid area".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Disabled RTL"</span>
                        <ColorArea
                            id_base="docs-color-area-matrix-disabled-v2".to_string()
                            label="Disabled".to_string()
                            is_disabled=true
                            class_name="docs-color-area-custom".to_string()
                            aria_label="Disabled color area".to_string()
                            lang="ar".to_string()
                            dir=A11yDirection::Rtl
                            motion=ui::color::area::ColorAreaMotion::default()
                        />
                    </div>
                </div>
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
                    <code>"use leptos::prelude::*; use ui::*;"</code>
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
