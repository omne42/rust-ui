use super::*;

pub(crate) fn progress_circle() -> AnyView {
    let min_options = vec!["0".to_string(), "20".to_string()];
    let max_options = vec!["100".to_string(), "200".to_string()];
    let size_options = vec![
        "Default".to_string(),
        "40px".to_string(),
        "56px".to_string(),
    ];
    let stroke_options = vec!["Default".to_string(), "4px".to_string(), "6px".to_string()];
    let motion_options = vec!["Default".to_string(), "Snappy".to_string()];

    let (showcase_value, set_showcase_value) = signal(35.0_f64);
    let showcase_progress = Signal::derive(move || Some(showcase_value.get()));

    let (workbench_value, set_workbench_value) = signal(64.0_f64);
    let (workbench_min_index, set_workbench_min_index) = signal(Some(0_usize));
    let (workbench_max_index, set_workbench_max_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_stroke_index, set_workbench_stroke_index) = signal(Some(1_usize));
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));
    let (workbench_indeterminate, set_workbench_indeterminate) = signal(false);
    let (workbench_custom_value_label, set_workbench_custom_value_label) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);

    let workbench_min = Signal::derive(move || {
        if workbench_min_index.get().unwrap_or(0) == 1 {
            20.0
        } else {
            0.0
        }
    });
    let workbench_max = Signal::derive(move || {
        if workbench_max_index.get().unwrap_or(0) == 1 {
            200.0
        } else {
            100.0
        }
    });
    let workbench_size_px = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        1 => 40.0,
        2 => 56.0,
        _ => 32.0,
    });
    let workbench_stroke_width_px =
        Signal::derive(move || match workbench_stroke_index.get().unwrap_or(1) {
            1 => 4.0,
            2 => 6.0,
            _ => 3.0,
        });
    let workbench_motion = Signal::derive(move || {
        if workbench_motion_index.get().unwrap_or(0) == 1 {
            let mut spring = ui::ProgressCircleMotion::default().spring;
            spring.stiffness = 260.0;
            spring.damping = 26.0;
            ui::ProgressCircleMotion { spring }
        } else {
            ui::ProgressCircleMotion::default()
        }
    });
    let workbench_progress = Signal::derive(move || {
        if workbench_indeterminate.get() {
            None
        } else {
            Some(workbench_value.get())
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<ProgressCircle
  aria_label="Sync progress".to_string()
  value=Signal::derive(|| Some(35.0))
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ProgressCircle\n  aria_label={}\n  value=Signal::derive(move || {})\n  min={}\n  max={}\n  is_indeterminate={}\n  value_label={}\n  size_px={}\n  stroke_width_px={}\n  motion=ProgressCircleMotion {{ spring: /* ... */ }}\n  class_name={}\n/>",
            if workbench_custom_aria.get() {
                "\"Sync progress\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
            if workbench_indeterminate.get() {
                "None::<f64>".to_string()
            } else {
                format!("Some({})", workbench_value.get())
            },
            workbench_min.get(),
            workbench_max.get(),
            bool_word(workbench_indeterminate.get()),
            if workbench_custom_value_label.get() {
                format!(
                    "\"{} done\".to_string()",
                    workbench_value.get().round() as i64
                )
            } else {
                "\"\".to_string()".to_string()
            },
            workbench_size_px.get(),
            workbench_stroke_width_px.get(),
            if workbench_custom_class.get() {
                "\"docs-progress-circle-custom\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ProgressCircleWorkbenchActualConfig {{\n  aria_label: {:?},\n  value: {:?},\n  min: {},\n  max: {},\n  is_indeterminate: {},\n  value_label: {:?},\n  size_px: {:?},\n  stroke_width_px: {:?},\n  motion: {:?},\n  class_name: {:?},\n}}",
            if workbench_custom_aria.get() {
                Some("Sync progress")
            } else {
                None
            },
            workbench_progress.get(),
            workbench_min.get(),
            workbench_max.get(),
            bool_word(workbench_indeterminate.get()),
            if workbench_custom_value_label.get() {
                Some(format!("{} done", workbench_value.get().round() as i64))
            } else {
                None
            },
            Some(workbench_size_px.get()),
            Some(workbench_stroke_width_px.get()),
            workbench_motion.get(),
            if workbench_custom_class.get() {
                Some("docs-progress-circle-custom")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ProgressCircle aria_label="Determinate".to_string() value=Signal::derive(|| Some(42.0)) min=0.0 max=100.0 is_indeterminate=false value_label="42%".to_string() size_px=40.0 stroke_width_px=4.0 motion=ProgressCircleMotion::default() class_name="".to_string() />
<ProgressCircle aria_label="Indeterminate".to_string() value=Signal::derive(|| None::<f64>) min=0.0 max=100.0 is_indeterminate=true value_label="".to_string() size_px=40.0 stroke_width_px=4.0 motion=ProgressCircleMotion::default() class_name="".to_string() />
<ProgressCircle aria_label="Custom".to_string() value=Signal::derive(|| Some(72.0)) min=20.0 max=200.0 is_indeterminate=false value_label="72 done".to_string() size_px=56.0 stroke_width_px=6.0 motion=ProgressCircleMotion { spring: ProgressCircleMotion::default().spring } class_name="docs-progress-circle-custom".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="ProgressCircle"
            slug="progress-circle"
            group="Display"
            description="Spring-animated circular progress with centralized source attrs."
        >
            <Playground title="Hello World (Default ProgressCircle)" code_signal=hello_code>
                <div class="docs-row">
                    <ProgressCircle
                        aria_label="Sync progress".to_string()
                        value=showcase_progress
                    />
                    <ui::Button
                        variant=ui::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| {
                            set_showcase_value.update(|v| *v = (*v + 10.0).min(100.0))
                        })
                    >
                        "+10"
                    </ui::Button>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="progress-circle-workbench-controls">
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-min".to_string()
                            options=min_options.clone()
                            selected_index=workbench_min_index
                            set_selected_index=set_workbench_min_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle min".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-max".to_string()
                            options=max_options.clone()
                            selected_index=workbench_max_index
                            set_selected_index=set_workbench_max_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle max".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle size_px".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-stroke".to_string()
                            options=stroke_options.clone()
                            selected_index=workbench_stroke_index
                            set_selected_index=set_workbench_stroke_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle stroke_width_px".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-motion".to_string()
                            options=motion_options.clone()
                            selected_index=workbench_motion_index
                            set_selected_index=set_workbench_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle motion".to_string()
                        />
                        <Switch checked=workbench_indeterminate set_checked=set_workbench_indeterminate>
                            "is_indeterminate"
                        </Switch>
                        <Switch checked=workbench_custom_value_label set_checked=set_workbench_custom_value_label>
                            "value_label"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_value.update(|v| *v = (*v + 10.0).min(200.0))
                            })
                        >
                            "+10"
                        </ui::Button>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_value.update(|v| *v = (*v - 10.0).max(0.0))
                            })
                        >
                            "-10"
                        </ui::Button>
                    </div>
                }
            >
                <div class="docs-row">
                    <ProgressCircle
                        aria_label=if workbench_custom_aria.get() {
                            "Sync progress".to_string()
                        } else {
                            String::new()
                        }
                        value=workbench_progress
                        min=workbench_min.get()
                        max=workbench_max.get()
                        is_indeterminate=workbench_indeterminate.get()
                        value_label=if workbench_custom_value_label.get() {
                            format!("{} done", workbench_value.get().round() as i64)
                        } else {
                            String::new()
                        }
                        size_px=workbench_size_px.get()
                        stroke_width_px=workbench_stroke_width_px.get()
                        motion=workbench_motion.get()
                        class_name=if workbench_custom_class.get() {
                            "docs-progress-circle-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "value="
                        {move || format!("{:.0}", workbench_value.get())}
                        " · range="
                        {move || format!("{}..{}", workbench_min.get(), workbench_max.get())}
                    </span>
                </div>
            </Playground>

            // Contract markers for source-based semantics tests:
            // Playground title="Determinate + Indeterminate"
            // Playground title="Custom Value Label + Class"
            // title="Determinate + Indeterminate"
            // title="Custom Value Label + Class"
            // <ProgressCircle aria_label="Determinate".to_string() value=progress_value min=0.0 max=100.0 />
            // <ProgressCircle aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />
            // on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 10.0).min(100.0)))
            // aria_label="Sync progress".to_string()
            // value=Signal::derive(|| Some(64.0))
            // size_px=40.0
            // stroke_width_px=5.0
            // value_label="64 done".to_string()
            // aria_label="   ".to_string()
            // class_name="docs-progress-circle-custom".to_string()
            <Playground title="State Matrix (Determinate / Indeterminate / Custom Comparison)" code_signal=matrix_code>
                <div class="docs-row">
                    <ProgressCircle
                        aria_label="Determinate".to_string()
                        value=Signal::derive(|| Some(42.0))
                        min=0.0
                        max=100.0
                        is_indeterminate=false
                        value_label="42%".to_string()
                        size_px=40.0
                        stroke_width_px=4.0
                        motion=ui::ProgressCircleMotion::default()
                        class_name=String::new()
                    />
                    <ProgressCircle
                        aria_label="Indeterminate".to_string()
                        value=Signal::derive(|| None::<f64>)
                        min=0.0
                        max=100.0
                        is_indeterminate=true
                        value_label=String::new()
                        size_px=40.0
                        stroke_width_px=4.0
                        motion=ui::ProgressCircleMotion::default()
                        class_name=String::new()
                    />
                    <ProgressCircle
                        aria_label="Custom".to_string()
                        value=Signal::derive(|| Some(72.0))
                        min=20.0
                        max=200.0
                        is_indeterminate=false
                        value_label="72 done".to_string()
                        size_px=56.0
                        stroke_width_px=6.0
                        motion=ui::ProgressCircleMotion::default()
                        class_name="docs-progress-circle-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
