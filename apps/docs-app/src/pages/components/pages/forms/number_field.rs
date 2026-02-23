use super::*;

pub(crate) fn number_field() -> AnyView {
    let (value, set_value) = signal(42_i64);
    let code = Signal::derive(move || {
        r#"let (value, set_value) = signal(42_i64);
<NumberField id="qty".to_string()
  label="Quantity".to_string()
  value=value
  set_value=set_value
  min=0
  max=100
/>"#
        .to_string()
    });

    let (required_value, set_required_value) = signal(7_i64);
    let required_flag: Signal<bool> = Signal::derive(|| true);

    let (invalid_value, set_invalid_value) = signal(120_i64);
    let invalid_flag: Signal<bool> = Signal::derive(|| true);

    let (disabled_value, set_disabled_value) = signal(18_i64);

    let states_code = Signal::derive(move || {
        r#"<NumberField id="qty-default".to_string() label="Default".to_string() value=value set_value=set_value min=0 max=100 />
<NumberField id="qty-required".to_string() label="Required".to_string() value=required_value set_value=set_required_value min=0 max=20 required=Signal::derive(|| true) description=Some("Required field".to_string()) />
<NumberField id="qty-invalid".to_string() label="Invalid".to_string() value=invalid_value set_value=set_invalid_value min=0 max=100 invalid=Signal::derive(|| true) error=Some("Out of range".to_string()) />
<NumberField id="qty-disabled".to_string() label="Disabled".to_string() value=disabled_value set_value=set_disabled_value min=0 max=100 disabled=true />"#.to_string()
    });

    let bounds_options = vec![
        "0..100".to_string(),
        "0..10".to_string(),
        "-20..20".to_string(),
    ];
    let (bounds_index, set_bounds_index) = signal(Some(1_usize));
    let workbench_min = Signal::derive(move || match bounds_index.get().unwrap_or(1) {
        1 => 0_i64,
        2 => -20_i64,
        _ => 0_i64,
    });
    let workbench_max = Signal::derive(move || match bounds_index.get().unwrap_or(1) {
        1 => 10_i64,
        2 => 20_i64,
        _ => 100_i64,
    });

    let step_options = vec!["1".to_string(), "5".to_string(), "10".to_string()];
    let (step_index, set_step_index) = signal(Some(0_usize));
    let workbench_step = Signal::derive(move || match step_index.get().unwrap_or(0) {
        1 => 5_i64,
        2 => 10_i64,
        _ => 1_i64,
    });

    let (workbench_value, set_workbench_value) = signal(12_i64);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_required_raw, set_workbench_required_raw) = signal(false);
    let (workbench_invalid_raw, set_workbench_invalid_raw) = signal(false);
    let (workbench_use_external_desc, set_workbench_use_external_desc) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_required: Signal<bool> = Signal::derive(move || workbench_required_raw.get());
    let workbench_invalid: Signal<bool> = Signal::derive(move || workbench_invalid_raw.get());
    let workbench_desc_id = "docs-number-field-workbench-help".to_string();
    let workbench_aria_describedby = {
        let workbench_desc_id = workbench_desc_id.clone();
        Signal::derive(move || {
            if workbench_use_external_desc.get() {
                Some(workbench_desc_id.clone())
            } else {
                None
            }
        })
    };
    let workbench_node_ref = NodeRef::<leptos::html::Input>::new();

    let (workbench_last_change, set_workbench_last_change) = signal("none".to_string());
    let on_workbench_change = Callback::new(move |next: i64| {
        set_workbench_last_change.set(next.to_string());
    });

    let workbench_desc_id_for_code = workbench_desc_id.clone();
    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "<NumberField".to_string(),
            "  id=\"docs-number-field-workbench\".into()".to_string(),
            "  label=\"Quantity\".into()".to_string(),
            "  value=value".to_string(),
            "  set_value=set_value".to_string(),
            format!("  min={}", workbench_min.get()),
            format!("  max={}", workbench_max.get()),
            format!("  step={}", workbench_step.get()),
            "  on_change=Callback::new(move |_| {})".to_string(),
            format!(
                "  required=Signal::derive(move || {})",
                bool_word(workbench_required_raw.get())
            ),
            format!(
                "  invalid=Signal::derive(move || {})",
                bool_word(workbench_invalid_raw.get())
            ),
            format!(
                "  aria_describedby=Signal::derive(move || {})",
                if workbench_use_external_desc.get() {
                    format!("Some({})", rust_string_literal(&workbench_desc_id_for_code))
                } else {
                    "None".to_string()
                }
            ),
            format!(
                "  description={}",
                rust_string_literal(if workbench_required_raw.get() {
                    "Required field"
                } else {
                    ""
                })
            ),
            format!(
                "  error={}",
                rust_string_literal(if workbench_invalid_raw.get() {
                    "Out of range"
                } else {
                    ""
                })
            ),
            "  placeholder=\"Enter quantity\".into()".to_string(),
            format!(
                "  class_name={}",
                rust_string_literal(if workbench_custom_class.get() {
                    "docs-number-field-custom"
                } else {
                    ""
                })
            ),
            "  node_ref=NodeRef::<leptos::html::Input>::new()".to_string(),
        ];
        lines.push(format!("  disabled={}", workbench_disabled.get()));
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/text-input/src/number_field/styles.rs */\n{}",
            ui::text_input::number_field::styles::CSS
        )
    });

    let workbench_desc_id_for_config = workbench_desc_id.clone();
    let workbench_actual_config = Signal::derive(move || {
        let description = if workbench_required_raw.get() {
            "Required field"
        } else {
            ""
        };
        let error = if workbench_invalid_raw.get() {
            "Out of range"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-number-field-custom"
        } else {
            ""
        };
        format!(
            "NumberFieldWorkbenchConfig {{\n  id: \"docs-number-field-workbench\",\n  label: \"Quantity\",\n  value: {},\n  set_value: \"set_workbench_value\",\n  disabled: {},\n  min: {:?},\n  max: {:?},\n  step: {},\n  on_change: \"set_workbench_last_change\",\n  required: {},\n  invalid: {},\n  aria_describedby: {},\n  description: {:?},\n  error: {:?},\n  placeholder: \"Enter quantity\",\n  class_name: {:?},\n  node_ref: \"workbench_node_ref\",\n  last_change: \"{}\",\n}}",
            workbench_value.get(),
            workbench_disabled.get(),
            workbench_min.get(),
            workbench_max.get(),
            workbench_step.get(),
            workbench_required_raw.get(),
            workbench_invalid_raw.get(),
            if workbench_use_external_desc.get() {
                format!(
                    "Some({})",
                    rust_string_literal(&workbench_desc_id_for_config)
                )
            } else {
                "None".to_string()
            },
            description,
            error,
            class_name,
            workbench_last_change.get()
        )
    });

    view! {
        <ComponentPage
            title="NumberField"
            slug="number-field"
            group="Forms"
            description="Numeric input with steppers and keyboard control."
        >
            <Playground title="Hello World (Default Stepper)" code_signal=code>
                <div class="docs-row">
                    <NumberField
                        id="docs-number-field".to_string()
                        label="Quantity".to_string()
                        value=value
                        set_value=set_value
                        min=0
                        max=100
                    />
                    <span class="ui-muted">"value: " {move || value.get()}</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                description="Display + Config + Code + CSS Test workbench for number-field semantics and stepping contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/text-input/src/number_field/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="number-field-workbench-controls">
                        <div class="docs-search__label">"Bounds"</div>
                        <SegmentedControl
                            id_base="docs-number-field-workbench-bounds".to_string()
                            options=bounds_options.clone()
                            selected_index=bounds_index
                            set_selected_index=set_bounds_index
                            size=SegmentedControlSize::Sm
                            aria_label="NumberField bounds".to_string()
                        />

                        <div class="docs-search__label">"Step"</div>
                        <SegmentedControl
                            id_base="docs-number-field-workbench-step".to_string()
                            options=step_options.clone()
                            selected_index=step_index
                            set_selected_index=set_step_index
                            size=SegmentedControlSize::Sm
                            aria_label="NumberField step".to_string()
                        />

                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_required_raw set_checked=set_workbench_required_raw>
                            "Required"
                        </Switch>
                        <Switch checked=workbench_invalid_raw set_checked=set_workbench_invalid_raw>
                            "Invalid"
                        </Switch>
                        <Switch
                            checked=workbench_use_external_desc
                            set_checked=set_workbench_use_external_desc
                        >
                            "aria_describedby"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" style="width: min(100%, 360px);">
                    <NumberField
                        id="docs-number-field-workbench".to_string()
                        label="Quantity".to_string()
                        value=workbench_value
                        set_value=set_workbench_value
                        min=workbench_min.get()
                        max=workbench_max.get()
                        step=workbench_step.get()
                        disabled=workbench_disabled.get()
                        required=workbench_required
                        invalid=workbench_invalid
                        aria_describedby=workbench_aria_describedby
                        description=if workbench_required_raw.get() {
                            "Required field".to_string()
                        } else {
                            String::new()
                        }
                        error=if workbench_invalid_raw.get() {
                            "Out of range".to_string()
                        } else {
                            String::new()
                        }
                        placeholder="Enter quantity".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-number-field-custom".to_string()
                        } else {
                            String::new()
                        }
                        node_ref=workbench_node_ref
                        on_change=on_workbench_change
                    />
                    <Show when=move || workbench_use_external_desc.get()>
                        <div id=workbench_desc_id.clone() class="ui-muted">
                            "External help text wired via aria_describedby."
                        </div>
                    </Show>
                    <span class="ui-muted">
                        "value: "
                        {move || workbench_value.get()}
                        " | last on_change: "
                        {move || workbench_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <NumberField
                            id="docs-number-field-default".to_string()
                            label="Default".to_string()
                            value=value
                            set_value=set_value
                            min=0
                            max=100
                        />
                        <NumberField
                            id="docs-number-field-required".to_string()
                            label="Required".to_string()
                            value=required_value
                            set_value=set_required_value
                            min=0
                            max=20
                            required=required_flag
                            description="Required field".to_string()
                        />
                    </div>
                    <div class="docs-row">
                        <NumberField
                            id="docs-number-field-invalid".to_string()
                            label="Invalid".to_string()
                            value=invalid_value
                            set_value=set_invalid_value
                            min=0
                            max=100
                            invalid=invalid_flag
                            error="Out of range".to_string()
                        />
                        <NumberField
                            id="docs-number-field-disabled".to_string()
                            label="Disabled".to_string()
                            value=disabled_value
                            set_value=set_disabled_value
                            min=0
                            max=100
                            disabled=true
                        />
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
