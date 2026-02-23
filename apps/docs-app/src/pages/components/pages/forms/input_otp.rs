use super::*;

pub(crate) fn input_otp() -> AnyView {
    let (value, set_value) = signal(String::new());
    let code = Signal::derive(move || {
        r#"let (value, set_value) = signal(String::new());
<InputOtp id_base="otp".to_string()
  label="One-time code".to_string()
  value=value
  set_value=set_value
  length=6
/>"#
        .to_string()
    });

    let length_options = vec!["4".to_string(), "6".to_string(), "8".to_string()];
    let (workbench_length_index, set_workbench_length_index) = signal(Some(1_usize));
    let workbench_length =
        Signal::derive(move || match workbench_length_index.get().unwrap_or(1) {
            0 => 4_usize,
            2 => 8_usize,
            _ => 6_usize,
        });
    let (workbench_value, set_workbench_value) = signal(String::new());
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_required, set_workbench_required) = signal(false);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_error, set_workbench_show_error) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_use_external_desc, set_workbench_use_external_desc) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let workbench_desc_id = "docs-input-otp-workbench-help".to_string();
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
    let on_workbench_change = Callback::new(move |next: String| {
        set_workbench_last_change.set(next);
    });
    let (workbench_last_complete, set_workbench_last_complete) = signal("none".to_string());
    let on_workbench_complete =
        Callback::new(move |next: String| set_workbench_last_complete.set(next));

    let workbench_desc_id_for_code = workbench_desc_id.clone();
    let workbench_code = Signal::derive(move || {
        let length = workbench_length.get();
        format!(
            "let (value, set_value) = signal(String::new());\n\n<InputOtp\n  id_base=\"docs-otp-workbench\".into()\n  label=\"One-time code\".into()\n  value=value\n  set_value=set_value\n  length={length}\n  disabled={}\n  on_change=Callback::new(move |_| {{}})\n  on_complete=Callback::new(move |_| {{}})\n  aria_label={}\n  required=Signal::derive(move || {})\n  invalid=Signal::derive(move || {})\n  aria_describedby=Signal::derive(move || {})\n  description={}\n  error={}\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=NodeRef::<leptos::html::Input>::new()\n/>",
            workbench_disabled.get(),
            rust_string_literal(if workbench_custom_aria.get() {
                "Verification code"
            } else {
                ""
            }),
            bool_word(workbench_required.get()),
            bool_word(workbench_invalid.get()),
            if workbench_use_external_desc.get() {
                format!("Some({})", rust_string_literal(&workbench_desc_id_for_code))
            } else {
                "None".to_string()
            },
            rust_string_literal(if workbench_show_description.get() {
                "We sent a code to your device."
            } else {
                ""
            }),
            rust_string_literal(if workbench_show_error.get() {
                "Code does not match."
            } else {
                ""
            }),
            rust_string_literal(if workbench_custom_class.get() {
                "docs-input-otp-workbench"
            } else {
                ""
            }),
            if workbench_lang_zh.get() {
                "Some(\"zh-CN\".to_string())".to_string()
            } else {
                "Some(\"en-US\".to_string())".to_string()
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)".to_string()
            } else {
                "Some(A11yDirection::Ltr)".to_string()
            }
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/text-input/src/input_otp/styles.rs */\n{}",
            ui::text_input::input_otp::styles::CSS
        )
    });

    let workbench_desc_id_for_config = workbench_desc_id.clone();
    let workbench_actual_config = Signal::derive(move || {
        let description = if workbench_show_description.get() {
            "We sent a code to your device."
        } else {
            ""
        };
        let error = if workbench_show_error.get() {
            "Code does not match."
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-input-otp-workbench"
        } else {
            ""
        };
        format!(
            "InputOtpWorkbenchConfig {{\n  id_base: \"docs-otp-workbench\",\n  value: {:?},\n  set_value: \"set_workbench_value\",\n  length: {},\n  disabled: {},\n  on_change: \"set_workbench_last_change\",\n  on_complete: \"set_workbench_last_complete\",\n  label: \"One-time code\",\n  aria_label: {:?},\n  required: {},\n  invalid: {},\n  aria_describedby: {},\n  description: {:?},\n  error: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {},\n  node_ref: \"workbench_node_ref\",\n  last_change: {:?},\n  last_complete: {:?},\n}}",
            workbench_value.get(),
            workbench_length.get(),
            workbench_disabled.get(),
            if workbench_custom_aria.get() {
                "Verification code"
            } else {
                ""
            },
            workbench_required.get(),
            workbench_invalid.get(),
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
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            workbench_last_change.get(),
            workbench_last_complete.get(),
        )
    });

    let (compare_default, set_compare_default) = signal(String::new());
    let (compare_disabled, set_compare_disabled) = signal("2468".to_string());
    let (compare_invalid, set_compare_invalid) = signal("12".to_string());
    let state_compare_code = Signal::derive(move || {
        r#"<InputOtp id_base="otp-default".to_string() value=default_value set_value=set_default_value length=6 />
<InputOtp id_base="otp-disabled".to_string() value=disabled_value set_value=set_disabled_value length=4 disabled=true />
<InputOtp
  id_base="otp-invalid".to_string()
  value=invalid_value
  set_value=set_invalid_value
  length=6
  invalid=Signal::derive(move || true)
  error="Code does not match.".to_string()
/>"#.to_string()
    });

    view! {
        <ComponentPage
            title="InputOtp"
            slug="input-otp"
            group="Forms"
            description="baseline-style OTP input with a single hidden input and slot chrome."
        >
            <Playground title="Hello World (Default OTP)" code_signal=code>
                <div class="docs-stack">
                    <InputOtp
                        id_base="docs-otp".to_string()
                        label="One-time code".to_string()
                        value=value
                        set_value=set_value
                        length=6
                    />
                    <span class="ui-muted">"value: " {move || value.get()}</span>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/text-input/src/input_otp/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Length"</div>
                        <SegmentedControl
                            id_base="docs-input-otp-workbench-length".to_string()
                            options=length_options.clone()
                            selected_index=workbench_length_index
                            set_selected_index=set_workbench_length_index
                            size=SegmentedControlSize::Sm
                            aria_label="InputOtp workbench length".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "Required"
                        </Switch>
                        <Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "Invalid"
                        </Switch>
                        <Switch
                            checked=workbench_show_description
                            set_checked=set_workbench_show_description
                        >
                            "Show description"
                        </Switch>
                        <Switch checked=workbench_show_error set_checked=set_workbench_show_error>
                            "Show error"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria_label"
                        </Switch>
                        <Switch
                            checked=workbench_use_external_desc
                            set_checked=set_workbench_use_external_desc
                        >
                            "aria_describedby"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="input-otp-workbench">
                    <InputOtp
                        id_base="docs-otp-workbench".to_string()
                        label="One-time code".to_string()
                        value=workbench_value
                        set_value=set_workbench_value
                        length=workbench_length.get()
                        disabled=workbench_disabled.get()
                        required=workbench_required
                        invalid=workbench_invalid
                        on_change=on_workbench_change
                        description=if workbench_show_description.get() {
                            "We sent a code to your device.".to_string()
                        } else {
                            String::new()
                        }
                        error=if workbench_show_error.get() {
                            "Code does not match.".to_string()
                        } else {
                            String::new()
                        }
                        aria_describedby=workbench_aria_describedby
                        class_name=if workbench_custom_class.get() {
                            "docs-input-otp-workbench".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if workbench_custom_aria.get() {
                            "Verification code".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl_dir.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        node_ref=workbench_node_ref
                        on_complete=on_workbench_complete
                    />
                    <Show when=move || workbench_use_external_desc.get()>
                        <div id=workbench_desc_id.clone() class="ui-muted">
                            "External helper text attached via aria_describedby."
                        </div>
                    </Show>
                    <span class="ui-muted">"value: " {move || workbench_value.get()}</span>
                    <span class="ui-muted">
                        "last change: "
                        {move || workbench_last_change.get()}
                    </span>
                    <span class="ui-muted">
                        "last complete: "
                        {move || workbench_last_complete.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Comparison" code_signal=state_compare_code>
                <div class="docs-stack docs-stack--tight" data-slot="input-otp-state-compare">
                    <div class="ui-muted">"Default"</div>
                    <InputOtp
                        id_base="docs-otp-compare-default".to_string()
                        label="Default OTP".to_string()
                        value=compare_default
                        set_value=set_compare_default
                        length=6
                    />
                    <span class="ui-muted">"value: " {move || compare_default.get()}</span>

                    <div class="ui-muted">"Disabled (prefilled)"</div>
                    <InputOtp
                        id_base="docs-otp-compare-disabled".to_string()
                        label="Disabled OTP".to_string()
                        value=compare_disabled
                        set_value=set_compare_disabled
                        length=4
                        disabled=true
                    />

                    <div class="ui-muted">"Invalid + error"</div>
                    <InputOtp
                        id_base="docs-otp-compare-invalid".to_string()
                        label="Invalid OTP".to_string()
                        value=compare_invalid
                        set_value=set_compare_invalid
                        length=6
                        invalid=Signal::derive(move || true)
                        error="Code does not match.".to_string()
                    />
                    <span class="ui-muted">"value: " {move || compare_invalid.get()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
