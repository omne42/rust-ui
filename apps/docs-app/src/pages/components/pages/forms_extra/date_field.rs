use super::*;

pub(crate) fn date_field() -> AnyView {
    let tone_options = vec!["Default".to_string(), "Strong".to_string()];
    let id_base_options = vec![
        "invoice-date".to_string(),
        "ship-date".to_string(),
        "due-date".to_string(),
    ];
    let motion_options = vec![
        "Default".to_string(),
        "No Motion".to_string(),
        "Long Motion".to_string(),
    ];

    let (workbench_id_base_index, set_workbench_id_base_index) = signal(Some(0_usize));
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_with_default, set_workbench_with_default) = signal(true);
    let (workbench_with_label, set_workbench_with_label) = signal(true);
    let (workbench_with_placeholder, set_workbench_with_placeholder) = signal(true);
    let (workbench_with_aria_label, set_workbench_with_aria_label) = signal(true);
    let (workbench_with_custom_class, set_workbench_with_custom_class) = signal(false);
    let (workbench_with_callback, set_workbench_with_callback) = signal(true);

    let (workbench_value, set_workbench_value) = signal(Some("2026-03-22".to_string()));
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let (workbench_last_change, set_workbench_last_change) = signal("none".to_string());
    let on_workbench_value_change = Callback::new(move |next: Option<String>| {
        if workbench_with_callback.get() {
            set_workbench_change_count.update(|count| *count += 1);
            set_workbench_last_change.set(next.clone().unwrap_or_else(|| "none".to_string()));
            set_workbench_value.set(next);
        }
    });

    let workbench_id_base =
        Signal::derive(move || match workbench_id_base_index.get().unwrap_or(0) {
            1 => "ship-date".to_string(),
            2 => "due-date".to_string(),
            _ => "invoice-date".to_string(),
        });
    let workbench_tone = Signal::derive(move || {
        if workbench_tone_index.get().unwrap_or(0) == 1 {
            DateFieldTone::Strong
        } else {
            DateFieldTone::Default
        }
    });
    let workbench_motion =
        Signal::derive(move || match workbench_motion_index.get().unwrap_or(0) {
            1 => ui::text_input::date_field::DateFieldMotion::disabled(),
            2 => ui::text_input::date_field::DateFieldMotion {
                enabled: true,
                duration_ms: 420,
            },
            _ => ui::text_input::date_field::DateFieldMotion::default(),
        });
    let workbench_controlled_value = Signal::derive(move || {
        if workbench_controlled.get() {
            workbench_value.get()
        } else {
            None
        }
    });

    let showcase_code =
        Signal::derive(move || r#"<DateField id_base="invoice-date".to_string() />"#.to_string());

    let workbench_code = Signal::derive(move || {
        format!(
            "<DateField\n  id_base={}.to_string()\n  label={}\n  placeholder={}\n  tone={:?}\n  is_disabled={}\n  value={}\n  default_value={}\n  on_value_change={}\n  aria_label={}\n  motion=DateFieldMotion {{ enabled: {}, duration_ms: {} }}\n  class_name={}\n/>",
            rust_string_literal(&workbench_id_base.get()),
            if workbench_with_label.get() {
                format!("Some({}.to_string())", rust_string_literal("Invoice date"))
            } else {
                "None".to_string()
            },
            if workbench_with_placeholder.get() {
                format!("Some({}.to_string())", rust_string_literal("yyyy-mm-dd"))
            } else {
                "None".to_string()
            },
            workbench_tone.get(),
            bool_word(workbench_disabled.get()),
            if workbench_controlled.get() {
                "value".to_string()
            } else {
                "Signal::derive(move || None::<String>)".to_string()
            },
            if workbench_with_default.get() {
                format!("{}.to_string()", rust_string_literal("2026-03-22"))
            } else {
                "\"\".to_string()".to_string()
            },
            if workbench_with_callback.get() {
                "on_value_change".to_string()
            } else {
                "Callback::new(|_: Option<String>| {})".to_string()
            },
            if workbench_with_aria_label.get() {
                format!("{}.to_string()", rust_string_literal("Invoice date field"))
            } else {
                "\"\".to_string()".to_string()
            },
            bool_word(workbench_motion.get().enabled),
            workbench_motion.get().duration_ms,
            if workbench_with_custom_class.get() {
                format!(
                    "{}.to_string()",
                    rust_string_literal("docs-date-field-custom")
                )
            } else {
                "\"\".to_string()".to_string()
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "DateFieldWorkbenchActualConfig {{\n  id_base: {:?},\n  label: {:?},\n  placeholder: {:?},\n  tone: {:?},\n  is_disabled: {},\n  value: {:?},\n  default_value: {:?},\n  on_value_change: {},\n  aria_label: {:?},\n  motion: DateFieldMotion {{ enabled: {}, duration_ms: {} }},\n  class_name: {:?},\n}}",
            workbench_id_base.get(),
            if workbench_with_label.get() {
                Some("Invoice date")
            } else {
                None
            },
            if workbench_with_placeholder.get() {
                Some("yyyy-mm-dd")
            } else {
                None
            },
            workbench_tone.get(),
            bool_word(workbench_disabled.get()),
            if workbench_controlled.get() {
                Some(workbench_value.get())
            } else {
                None
            },
            if workbench_with_default.get() {
                Some("2026-03-22")
            } else {
                None
            },
            bool_word(workbench_with_callback.get()),
            if workbench_with_aria_label.get() {
                Some("Invoice date field")
            } else {
                None
            },
            bool_word(workbench_motion.get().enabled),
            workbench_motion.get().duration_ms,
            if workbench_with_custom_class.get() {
                Some("docs-date-field-custom")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<DateField id_base="matrix-default".to_string() label="Default".to_string() />
<DateField
  id_base="matrix-strong".to_string()
  label="Strong tone".to_string()
  tone=DateFieldTone::Strong
  placeholder="yyyy-mm-dd".to_string()
  aria_label="Strong date field".to_string()
/>
<DateField
  id_base="matrix-disabled".to_string()
  label="Disabled".to_string()
  is_disabled=true
  default_value=Some("2026-06-01".to_string())
  motion=DateFieldMotion::disabled()
  class_name="docs-date-field-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="DateField"
            slug="date-field"
            group="Forms"
            description="Segmented date entry field with centralized year/month/day normalization and baseline-style state/source contracts."
        >
            <Playground title="Hello World (Default DateField)" code_signal=showcase_code>
                <div class="docs-stack">
                    <DateField id_base="docs-date-field-showcase".to_string() />
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="date-field-workbench-controls">
                        <SegmentedControl
                            id_base="docs-date-field-workbench-id-base".to_string()
                            options=id_base_options.clone()
                            selected_index=workbench_id_base_index
                            set_selected_index=set_workbench_id_base_index
                            size=SegmentedControlSize::Sm
                            aria_label="DateField id_base".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-date-field-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="DateField tone".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-date-field-workbench-motion".to_string()
                            options=motion_options.clone()
                            selected_index=workbench_motion_index
                            set_selected_index=set_workbench_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="DateField motion".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_controlled set_checked=set_workbench_controlled>
                            "value (controlled)"
                        </Switch>
                        <Switch checked=workbench_with_default set_checked=set_workbench_with_default>
                            "default_value"
                        </Switch>
                        <Switch checked=workbench_with_label set_checked=set_workbench_with_label>
                            "label"
                        </Switch>
                        <Switch checked=workbench_with_placeholder set_checked=set_workbench_with_placeholder>
                            "placeholder"
                        </Switch>
                        <Switch checked=workbench_with_aria_label set_checked=set_workbench_with_aria_label>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_with_custom_class set_checked=set_workbench_with_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_with_callback set_checked=set_workbench_with_callback>
                            "on_value_change"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <DateField
                        id_base=workbench_id_base.get()
                        label=if workbench_with_label.get() {
                            "Invoice date".to_string()
                        } else {
                            String::new()
                        }
                        placeholder=if workbench_with_placeholder.get() {
                            "yyyy-mm-dd".to_string()
                        } else {
                            String::new()
                        }
                        tone=workbench_tone.get()
                        is_disabled=workbench_disabled.get()
                        value=workbench_controlled_value
                        default_value=if workbench_with_default.get() {
                            "2026-03-22".to_string()
                        } else {
                            String::new()
                        }
                        on_value_change=on_workbench_value_change
                        aria_label=if workbench_with_aria_label.get() {
                            "Invoice date field".to_string()
                        } else {
                            String::new()
                        }
                        motion=workbench_motion.get()
                        class_name=if workbench_with_custom_class.get() {
                            "docs-date-field-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "current value: "
                        {move || workbench_value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                    <span class="ui-muted">
                        "change_count="
                        {move || workbench_change_count.get()}
                        " · last_change="
                        {move || workbench_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Strong / Disabled)" code_signal=matrix_code>
                <div class="docs-stack" data-slot="date-field-state-matrix">
                    <DateField
                        id_base="docs-date-field-matrix-default".to_string()
                        label="Default".to_string()
                    />
                    <DateField
                        id_base="docs-date-field-matrix-strong".to_string()
                        label="Strong tone".to_string()
                        tone=DateFieldTone::Strong
                        placeholder="yyyy-mm-dd".to_string()
                        aria_label="Strong tone date field".to_string()
                    />
                    <DateField
                        id_base="docs-date-field-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        is_disabled=true
                        default_value="2026-06-01".to_string()
                        motion=ui::text_input::date_field::DateFieldMotion::disabled()
                        class_name="docs-date-field-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
