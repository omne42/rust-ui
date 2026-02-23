use super::*;

pub(crate) fn time_field() -> AnyView {
    let (workbench_value, set_workbench_value) = signal(Some("09:30".to_string()));
    let workbench_value_signal: Signal<Option<String>> = workbench_value.into();
    let (workbench_on_value_change_runs, set_workbench_on_value_change_runs) = signal(0_u32);
    let on_workbench_value_change = Callback::new(move |next: Option<String>| {
        set_workbench_value.set(next);
        set_workbench_on_value_change_runs.update(|count| *count += 1);
    });

    let (workbench_step_index, set_workbench_step_index) = signal(Some(2_usize));
    let step_options = vec![
        "1 minute".to_string(),
        "5 minutes".to_string(),
        "15 minutes".to_string(),
    ];
    let workbench_minute_step =
        Signal::derive(move || match workbench_step_index.get().unwrap_or(2) {
            0 => 1_u8,
            1 => 5_u8,
            _ => 15_u8,
        });

    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_custom_text, set_workbench_custom_text) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_reduced_motion, set_workbench_reduced_motion) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<TimeField
  id_base="docs-time-field-hello".to_string()
  label="Meeting time".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            "TimeFieldTone::Strong"
        } else {
            "TimeFieldTone::Default"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-time-field-custom"
        } else {
            ""
        };
        let label = if workbench_custom_text.get() {
            "Deployment time"
        } else {
            "Meeting time"
        };
        let placeholder = if workbench_custom_text.get() {
            "hh:mm"
        } else {
            "hour:minute"
        };
        let motion = if workbench_reduced_motion.get() {
            "TimeFieldMotion { hidden_scale: 1.0, hover_scale: 1.0, tap_scale: 1.0, ..TimeFieldMotion::default() }"
        } else {
            "TimeFieldMotion::default()"
        };

        [
            "<TimeField".to_string(),
            "  id_base=\"docs-time-field-workbench\".to_string()".to_string(),
            format!("  label={}", rust_string_literal(label)),
            format!("  placeholder={}", rust_string_literal(placeholder)),
            format!("  tone={tone}"),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled_alias.get())),
            "  value=workbench_value.into()".to_string(),
            "  default_value=\"09:30\".to_string()".to_string(),
            "  on_value_change=on_workbench_value_change".to_string(),
            format!("  minute_step={}", workbench_minute_step.get()),
            "  aria_label=\"Work period time\".to_string()".to_string(),
            "  hour_aria_label=\"Hour field\".to_string()".to_string(),
            "  minute_aria_label=\"Minute field\".to_string()".to_string(),
            "  clear_label=\"Clear time\".to_string()".to_string(),
            "  clear_aria_label=\"Clear selected time\".to_string()".to_string(),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            format!("  motion={motion}"),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            TimeFieldTone::Strong
        } else {
            TimeFieldTone::Default
        };
        let label = if workbench_custom_text.get() {
            "Deployment time"
        } else {
            "Meeting time"
        };
        let placeholder = if workbench_custom_text.get() {
            "hh:mm"
        } else {
            "hour:minute"
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-time-field-custom")
        } else {
            None
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let motion = if workbench_reduced_motion.get() {
            TimeFieldMotion {
                hidden_scale: 1.0,
                hover_scale: 1.0,
                tap_scale: 1.0,
                ..TimeFieldMotion::default()
            }
        } else {
            TimeFieldMotion::default()
        };
        let value = workbench_value.get();
        let value_text = value.as_ref().map_or_else(
            || "None".to_string(),
            |it| format!("Some({})", rust_string_literal(it)),
        );

        format!(
            "TimeFieldActualConfig {{\n  id_base: \"docs-time-field-workbench\",\n  label: {},\n  placeholder: {},\n  tone: {tone:?},\n  is_disabled: Some({}),\n  disabled: {},\n  value: {value_text},\n  default_value: Some(\"09:30\"),\n  on_value_change: \"runs={}\",\n  minute_step: {},\n  aria_label: \"Work period time\",\n  hour_aria_label: \"Hour field\",\n  minute_aria_label: \"Minute field\",\n  clear_label: \"Clear time\",\n  clear_aria_label: \"Clear selected time\",\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n  motion: {motion:?},\n  class_name: {class_name:?},\n}}",
            rust_string_literal(label),
            rust_string_literal(placeholder),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
            workbench_on_value_change_runs.get(),
            workbench_minute_step.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<TimeField id_base="time-default".to_string() label="Default".to_string() default_value="09:30".to_string() />
<TimeField id_base="time-strong".to_string() label="Strong".to_string() tone=TimeFieldTone::Strong minute_step=5 />
<TimeField id_base="time-disabled".to_string() label="Disabled".to_string() is_disabled=true disabled=true default_value="22:00".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="TimeField"
            slug="time-field"
            group="Forms"
            description="Time entry field playground with full API workbench coverage and callback feedback."
        >
            <Playground title="Hello World (Default)" code_signal=hello_code>
                <TimeField
                    id_base="docs-time-field-hello".to_string()
                    label="Meeting time".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="time-field-workbench-controls">
                        <SegmentedControl
                            id_base="docs-time-field-workbench-step".to_string()
                            options=step_options.clone()
                            selected_index=workbench_step_index
                            set_selected_index=set_workbench_step_index
                            size=SegmentedControlSize::Sm
                            aria_label="TimeField minute step".to_string()
                        />
                        <Switch checked=workbench_strong_tone set_checked=set_workbench_strong_tone>
                            "Strong tone"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled_alias set_checked=set_workbench_disabled_alias>
                            "disabled alias"
                        </Switch>
                        <Switch checked=workbench_custom_text set_checked=set_workbench_custom_text>
                            "Custom label + placeholder"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="time-field-workbench-preview">
                    <TimeField
                        id_base="docs-time-field-workbench".to_string()
                        label=if workbench_custom_text.get() {
                            "Deployment time".to_string()
                        } else {
                            "Meeting time".to_string()
                        }
                        placeholder=if workbench_custom_text.get() {
                            "hh:mm".to_string()
                        } else {
                            "hour:minute".to_string()
                        }
                        tone=if workbench_strong_tone.get() {
                            TimeFieldTone::Strong
                        } else {
                            TimeFieldTone::Default
                        }
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled_alias.get()
                        value=workbench_value_signal
                        default_value="09:30".to_string()
                        on_value_change=on_workbench_value_change
                        minute_step=workbench_minute_step.get()
                        aria_label="Work period time".to_string()
                        hour_aria_label="Hour field".to_string()
                        minute_aria_label="Minute field".to_string()
                        clear_label="Clear time".to_string()
                        clear_aria_label="Clear selected time".to_string()
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        motion=if workbench_reduced_motion.get() {
                            TimeFieldMotion {
                                hidden_scale: 1.0,
                                hover_scale: 1.0,
                                tap_scale: 1.0,
                                ..TimeFieldMotion::default()
                            }
                        } else {
                            TimeFieldMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-time-field-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted" data-slot="time-field-workbench-feedback">
                        "value: "
                        {move || workbench_value.get().unwrap_or_else(|| "none".to_string())}
                        " · minute_step: " {move || workbench_minute_step.get()}
                        " · on_value_change: " {move || workbench_on_value_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Strong / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="time-field-state-matrix">
                    <TimeField
                        id_base="docs-time-field-matrix-default".to_string()
                        label="Default".to_string()
                        default_value="09:30".to_string()
                    />
                    <TimeField
                        id_base="docs-time-field-matrix-strong".to_string()
                        label="Strong".to_string()
                        tone=TimeFieldTone::Strong
                        minute_step=5
                    />
                    <TimeField
                        id_base="docs-time-field-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        is_disabled=true
                        disabled=true
                        default_value="22:00".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
