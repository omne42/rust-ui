use super::*;

pub(crate) fn segmented_control() -> AnyView {
    let workbench_item_specs = vec![
        SegmentedControlItemSpec::new("Overview"),
        SegmentedControlItemSpec::new("Details"),
        SegmentedControlItemSpec::new("Settings"),
    ];
    let (workbench_selected, set_workbench_selected) = signal(Some(0_usize));
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_small, set_workbench_small) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            let mut spring = SegmentedControlMotion::default().spring;
            spring.stiffness = 180.0;
            spring.damping = 26.0;
            spring.mass = 1.0;
            spring.precision = 0.001;
            SegmentedControlMotion { spring }
        } else {
            SegmentedControlMotion::default()
        }
    });

    let matrix_item_specs = vec![
        SegmentedControlItemSpec::new("System"),
        SegmentedControlItemSpec::new("Manual"),
        SegmentedControlItemSpec::new("Hybrid"),
    ];
    let (matrix_horizontal_selected, set_matrix_horizontal_selected) = signal(Some(1_usize));
    let (matrix_vertical_selected, set_matrix_vertical_selected) = signal(Some(0_usize));
    let (matrix_disabled_selected, set_matrix_disabled_selected) = signal(Some(2_usize));

    let hello_code = Signal::derive(move || {
        r#"<SegmentedControl id_base="seg-default".to_string() default_selected_index=0_usize>
  <SegmentedControlItem slot:item label="Overview".to_string() />
  <SegmentedControlItem slot:item label="Details".to_string() />
</SegmentedControl>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let orientation = if workbench_vertical.get() {
            "SegmentedControlOrientation::Vertical"
        } else {
            "SegmentedControlOrientation::Horizontal"
        };
        let size = if workbench_small.get() {
            "SegmentedControlSize::Sm"
        } else {
            "SegmentedControlSize::Default"
        };
        let disabled_indices = if workbench_disable_last.get() {
            "vec![2_usize]"
        } else {
            "Vec::<usize>::new()"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let motion = if workbench_custom_motion.get() {
            "SegmentedControlMotion { spring: SegmentedControlMotion::default().spring }"
        } else {
            "SegmentedControlMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-segmented-control-workbench\".to_string()"
        } else {
            "String::new()"
        };

        format!(
            "let (selected, set_selected) = signal(Some(0_usize));\n\n<SegmentedControl\n  id_base=\"docs-segments-workbench\".to_string()\n  item_specs=vec![\n    SegmentedControlItemSpec::new(\"Overview\"),\n    SegmentedControlItemSpec::new(\"Details\"),\n    SegmentedControlItemSpec::new(\"Settings\"),\n  ]\n  selected_index=selected\n  on_selected_index_change=set_selected\n  is_disabled={}\n  disabled_indices={disabled_indices}\n  orientation={orientation}\n  size={size}\n  motion={motion}\n  label=\"Workspace section\".to_string()\n  aria_label=\"Workspace segmented control\".to_string()\n  lang={}.to_string()\n  dir={dir}\n  class_name={class_name}\n/>",
            bool_word(workbench_disabled.get()),
            rust_string_literal(if workbench_rtl.get() { "ar" } else { "en-US" }),
        )
    });

    let workbench_item_specs_for_config = workbench_item_specs.clone();
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SegmentedControlWorkbenchActualConfig {{\n  id_base: \"docs-segments-workbench\",\n  item_specs: {:?},\n  selected_index: {:?},\n  on_selected_index_change: \"bound(set_workbench_selected)\",\n  is_disabled: {},\n  disabled_indices: {:?},\n  orientation: {:?},\n  size: {:?},\n  motion: {:?},\n  label: Some(\"Workspace section\"),\n  aria_label: Some(\"Workspace segmented control\"),\n  lang: Some({:?}),\n  dir: Some({:?}),\n  class_name: {:?},\n}}",
            workbench_item_specs_for_config.clone(),
            workbench_selected.get(),
            bool_word(workbench_disabled.get()),
            if workbench_disable_last.get() {
                vec![2_usize]
            } else {
                Vec::<usize>::new()
            },
            if workbench_vertical.get() {
                SegmentedControlOrientation::Vertical
            } else {
                SegmentedControlOrientation::Horizontal
            },
            if workbench_small.get() {
                SegmentedControlSize::Sm
            } else {
                SegmentedControlSize::Default
            },
            if workbench_custom_motion.get() {
                "custom"
            } else {
                "default"
            },
            if workbench_rtl.get() { "ar" } else { "en-US" },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
            if workbench_custom_class.get() {
                Some("docs-segmented-control-workbench")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SegmentedControl id_base="seg-horizontal".to_string() item_specs=vec![SegmentedControlItemSpec::new("System"), SegmentedControlItemSpec::new("Manual"), SegmentedControlItemSpec::new("Hybrid")] selected_index=selected on_selected_index_change=set_selected />
<SegmentedControl id_base="seg-vertical".to_string() item_specs=vec![SegmentedControlItemSpec::new("System"), SegmentedControlItemSpec::new("Manual"), SegmentedControlItemSpec::new("Hybrid")] selected_index=selected_vertical on_selected_index_change=set_selected_vertical orientation=SegmentedControlOrientation::Vertical size=SegmentedControlSize::Sm disabled_indices=vec![2] />
<SegmentedControl id_base="seg-disabled".to_string() item_specs=vec![SegmentedControlItemSpec::new("System"), SegmentedControlItemSpec::new("Manual"), SegmentedControlItemSpec::new("Hybrid")] selected_index=selected_disabled on_selected_index_change=set_selected_disabled is_disabled=true aria_label="Disabled options".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="SegmentedControl"
            slug="segmented-control"
            group="Forms"
            description="Segmented control with baseline-level indicator motion and baseline-style root state attrs."
        >
            <Playground title="Hello World (Default)" code_signal=hello_code>
                <SegmentedControl
                    id_base="docs-segments-hello".to_string()
                    default_selected_index=0_usize
                >
                    <SegmentedControlItem slot:item label="Overview".to_string() />
                    <SegmentedControlItem slot:item label="Details".to_string() />
                </SegmentedControl>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="segmented-control-workbench-controls">
                        <Switch checked=workbench_vertical set_checked=set_workbench_vertical>
                            "Vertical orientation"
                        </Switch>
                        <Switch checked=workbench_small set_checked=set_workbench_small>
                            "Small size"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_disable_last set_checked=set_workbench_disable_last>
                            "disable last option"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL (lang + dir)"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_selected.set(Some(0_usize))
                        >
                            "Reset selection"
                        </button>
                    </div>
                }
            >
                <div class="docs-stack">
                    <SegmentedControl
                        id_base="docs-segments-workbench".to_string()
                        item_specs=workbench_item_specs.clone()
                        selected_index=workbench_selected
                        on_selected_index_change=set_workbench_selected
                        is_disabled=workbench_disabled.get()
                        disabled_indices=if workbench_disable_last.get() {
                            vec![2_usize]
                        } else {
                            Vec::<usize>::new()
                        }
                        orientation=if workbench_vertical.get() {
                            SegmentedControlOrientation::Vertical
                        } else {
                            SegmentedControlOrientation::Horizontal
                        }
                        size=if workbench_small.get() {
                            SegmentedControlSize::Sm
                        } else {
                            SegmentedControlSize::Default
                        }
                        motion=workbench_motion.get()
                        label="Workspace section".to_string()
                        aria_label="Workspace segmented control".to_string()
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-segmented-control-workbench".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Horizontal / Vertical / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <SegmentedControl
                        id_base="docs-segments-matrix-horizontal".to_string()
                        item_specs=matrix_item_specs.clone()
                        selected_index=matrix_horizontal_selected
                        on_selected_index_change=set_matrix_horizontal_selected
                    />
                    <SegmentedControl
                        id_base="docs-segments-matrix-vertical".to_string()
                        item_specs=matrix_item_specs.clone()
                        selected_index=matrix_vertical_selected
                        on_selected_index_change=set_matrix_vertical_selected
                        orientation=SegmentedControlOrientation::Vertical
                        size=SegmentedControlSize::Sm
                        disabled_indices=vec![2_usize]
                    />
                    <SegmentedControl
                        id_base="docs-segments-matrix-disabled".to_string()
                        item_specs=matrix_item_specs
                        selected_index=matrix_disabled_selected
                        on_selected_index_change=set_matrix_disabled_selected
                        is_disabled=true
                        aria_label="Disabled options".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
