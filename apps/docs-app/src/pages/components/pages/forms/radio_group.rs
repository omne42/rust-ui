use super::*;

pub(crate) fn radio_group() -> AnyView {
    let options = vec![
        "Small".to_string(),
        "Medium".to_string(),
        "Large".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));
    let has_selection = Signal::derive(move || selected.get().is_some());

    let workbench_options = vec![
        "Monthly".to_string(),
        "Quarterly".to_string(),
        "Yearly".to_string(),
    ];
    let (workbench_selected, set_workbench_selected) = signal(Some(2_usize));
    let workbench_external_label_id = "docs-radio-group-workbench-label".to_string();
    let (workbench_is_horizontal, set_workbench_is_horizontal) = signal(true);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_middle, set_workbench_disable_middle) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let matrix_options = vec![
        "Monthly".to_string(),
        "Quarterly".to_string(),
        "Yearly".to_string(),
    ];
    let (matrix_selected, set_matrix_selected) = signal(Some(2_usize));
    let (matrix_vertical_selected, set_matrix_vertical_selected) = signal(Some(0_usize));
    let empty_options = Vec::<String>::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));

<RadioGroup id_base="size".to_string() options=vec!["Small".to_string(), "Medium".to_string(), "Large".to_string()] label="Size".to_string() selected_index=selected set_selected_index=set_selected />"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let orientation = if workbench_is_horizontal.get() {
            "RadioGroupOrientation::Horizontal"
        } else {
            "RadioGroupOrientation::Vertical"
        };
        let disabled_indices = if workbench_disable_middle.get() {
            "vec![1_usize]"
        } else {
            "Vec::<usize>::new()"
        };
        let motion = if workbench_custom_motion.get() {
            "RadioMotion { hover_scale: 1.08, tap_scale: 0.94, ..RadioMotion::default() }"
        } else {
            "RadioMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-radio-group-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let lang = if workbench_rtl.get() {
            "\"ar\".to_string()"
        } else {
            "\"en-US\".to_string()"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        format!(
            "let (selected, set_selected) = signal(Some(2_usize));\n\n<RadioGroup\n  id_base=\"docs-radio-group-workbench\".to_string()\n  options=vec![\n    \"Monthly\".to_string(),\n    \"Quarterly\".to_string(),\n    \"Yearly\".to_string(),\n  ]\n  selected_index=selected\n  set_selected_index=set_selected\n  is_disabled={}\n  disabled={}\n  disabled_indices={disabled_indices}\n  orientation={orientation}\n  label=\"Billing cycle\".to_string()\n  aria_label=\"Billing cycle options\".to_string()\n  aria_labelledby=\"docs-radio-group-workbench-label\".to_string()\n  lang={lang}\n  dir={dir}\n  motion={motion}\n  class_name={class_name}\n/>",
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
        )
    });

    let workbench_options_for_config = workbench_options.clone();
    let workbench_actual_config = Signal::derive(move || {
        let orientation = if workbench_is_horizontal.get() {
            "horizontal"
        } else {
            "vertical"
        };
        let disabled_indices = if workbench_disable_middle.get() {
            vec![1_usize]
        } else {
            Vec::<usize>::new()
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-radio-group-workbench")
        } else {
            None
        };
        let motion = if workbench_custom_motion.get() {
            "custom"
        } else {
            "default"
        };
        let lang = if workbench_rtl.get() { "ar" } else { "en-US" };
        let dir = if workbench_rtl.get() { "rtl" } else { "ltr" };

        format!(
            "RadioGroupWorkbenchActualConfig {{\n  id_base: \"docs-radio-group-workbench\",\n  options: {:?},\n  selected_index: {:?},\n  set_selected_index: \"bound(set_workbench_selected)\",\n  is_disabled: Some({}),\n  disabled: {},\n  disabled_indices: {:?},\n  orientation: \"{orientation}\",\n  label: Some(\"Billing cycle\"),\n  aria_label: Some(\"Billing cycle options\"),\n  aria_labelledby: Some(\"docs-radio-group-workbench-label\"),\n  lang: Some({lang:?}),\n  dir: Some({dir:?}),\n  motion: \"{motion}\",\n  class_name: {class_name:?},\n}}",
            workbench_options_for_config.clone(),
            workbench_selected.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            disabled_indices,
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"let (billing_selected, set_billing_selected) = signal(Some(2_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<RadioGroup
  id_base="billing".to_string()
  options=vec![
    "Monthly".to_string(),
    "Quarterly".to_string(),
    "Yearly".to_string(),
  ]
  orientation=RadioGroupOrientation::Horizontal
  disabled_indices=vec![1]
  selected_index=billing_selected
  set_selected_index=set_billing_selected
/>
<RadioGroup
  id_base="billing-vertical".to_string()
  options=vec![
    "Monthly".to_string(),
    "Quarterly".to_string(),
    "Yearly".to_string(),
  ]
  orientation=RadioGroupOrientation::Vertical
  is_disabled=true
  aria_labelledby="docs-radio-group-billing-label".to_string()
  selected_index=billing_selected
  set_selected_index=set_billing_selected
/>
<RadioGroup
  id_base="empty".to_string()
  options=Vec::<String>::new()
  is_disabled=true
  aria_label="No options available".to_string()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="RadioGroup"
            slug="radio-group"
            group="Forms"
            description="Roving tabindex radiogroup with baseline-level spring motion and baseline-style root state attrs."
        >
            <Playground title="Hello World（默认路径）" code_signal=code>
                <div class="docs-stack">
                    <RadioGroup
                        id_base="docs-radio-group".to_string()
                        options=options
                        label="Size".to_string()
                        selected_index=selected
                        set_selected_index=set_selected
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · has selection: "
                        {move || has_selection.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="radio-group-workbench-controls">
                        <Switch checked=workbench_is_horizontal set_checked=set_workbench_is_horizontal>
                            "Horizontal orientation"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_disable_middle set_checked=set_workbench_disable_middle>
                            "Disable middle option"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL (lang + dir)"
                        </Switch>
                        <div class="docs-row">
                            <button type="button" on:click=move |_| set_workbench_selected.set(Some(0))>
                                "Select first"
                            </button>
                            <button type="button" on:click=move |_| set_workbench_selected.set(None)>
                                "Clear selection"
                            </button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack">
                    <div id=workbench_external_label_id.clone() class="ui-muted">"Billing cycle"</div>
                    {move || {
                        let orientation = if workbench_is_horizontal.get() {
                            RadioGroupOrientation::Horizontal
                        } else {
                            RadioGroupOrientation::Vertical
                        };
                        let disabled_indices = if workbench_disable_middle.get() {
                            vec![1_usize]
                        } else {
                            Vec::new()
                        };
                        let is_disabled = workbench_is_disabled.get();
                        view! {
                            <RadioGroup
                                id_base="docs-radio-group-workbench".to_string()
                                options=workbench_options.clone()
                                orientation=orientation
                                is_disabled=is_disabled
                                disabled=workbench_disabled.get()
                                disabled_indices=disabled_indices
                                label="Billing cycle".to_string()
                                aria_label="Billing cycle options".to_string()
                                aria_labelledby=workbench_external_label_id.clone()
                                selected_index=workbench_selected
                                set_selected_index=set_workbench_selected
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
                                motion=if workbench_custom_motion.get() {
                                    ui::radio::RadioMotion {
                                        hover_scale: 1.08,
                                        tap_scale: 0.94,
                                        ..ui::radio::RadioMotion::default()
                                    }
                                } else {
                                    ui::radio::RadioMotion::default()
                                }
                                class_name=if workbench_custom_class.get() {
                                    "docs-radio-group-workbench".to_string()
                                } else {
                                    String::new()
                                }
                            />
                        }
                    }}
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · orientation: "
                        {move || if workbench_is_horizontal.get() { "horizontal" } else { "vertical" }}
                        " · is_disabled: "
                        {move || workbench_is_disabled.get()}
                        " · disabled: "
                        {move || workbench_disabled.get()}
                        " · disabled options: "
                        {move || if workbench_disable_middle.get() { "1" } else { "0" }}
                        " · lang/dir: "
                        {move || if workbench_rtl.get() { "ar/rtl" } else { "en-US/ltr" }}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Horizontal / Vertical / Empty)" code_signal=matrix_code>
                <div class="docs-stack">
                    <div id="docs-radio-group-billing-label" class="ui-muted">"Billing cycle"</div>
                    <RadioGroup
                        id_base="docs-radio-group-matrix-horizontal".to_string()
                        options=matrix_options.clone()
                        orientation=RadioGroupOrientation::Horizontal
                        disabled_indices=vec![1_usize]
                        selected_index=matrix_selected
                        set_selected_index=set_matrix_selected
                    />
                    <RadioGroup
                        id_base="docs-radio-group-matrix-vertical".to_string()
                        options=matrix_options
                        orientation=RadioGroupOrientation::Vertical
                        is_disabled=true
                        aria_labelledby="docs-radio-group-billing-label".to_string()
                        selected_index=matrix_vertical_selected
                        set_selected_index=set_matrix_vertical_selected
                    />
                    <RadioGroup
                        id_base="docs-radio-group-empty".to_string()
                        options=empty_options
                        is_disabled=true
                        aria_label="No options available".to_string()
                        selected_index=empty_selected
                        set_selected_index=set_empty_selected
                    />
                    <span class="ui-muted">
                        "empty selected: "
                        {move || empty_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
