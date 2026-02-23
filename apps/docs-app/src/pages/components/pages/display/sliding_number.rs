use super::*;

pub(crate) fn sliding_number() -> AnyView {
    let (value, set_value) = signal(12345.67_f64);
    let number_signal: Signal<f64> = Signal::derive(move || value.get());
    let (workbench_decimal_places_key, set_workbench_decimal_places_key) = signal("2".to_string());
    let (workbench_decimal_sep_key, set_workbench_decimal_sep_key) = signal("dot".to_string());
    let (workbench_thousand_sep_key, set_workbench_thousand_sep_key) = signal("comma".to_string());
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_animate, set_workbench_animate) = signal(true);
    let (workbench_pad_start, set_workbench_pad_start) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_dir_rtl, set_workbench_dir_rtl) = signal(false);
    let (workbench_show_compare, set_workbench_show_compare) = signal(true);

    let workbench_decimal_places =
        Signal::derive(move || match workbench_decimal_places_key.get().as_str() {
            "auto" => None,
            "0" => Some(0_u32),
            "6" => Some(6_u32),
            _ => Some(2_u32),
        });
    let workbench_decimal_separator = Signal::derive(move || {
        if workbench_decimal_sep_key.get() == "comma" {
            Some(",".to_string())
        } else {
            None
        }
    });
    let workbench_thousand_separator =
        Signal::derive(move || match workbench_thousand_sep_key.get().as_str() {
            "none" => None,
            "space" => Some(" ".to_string()),
            _ => Some(",".to_string()),
        });

    let workbench_motion = Signal::derive(move || {
        let mut motion = ui::SlidingNumberMotion {
            animate: workbench_animate.get(),
            ..Default::default()
        };
        if workbench_custom_motion.get() {
            motion.spring.stiffness = 420.0;
            motion.spring.damping = 34.0;
        }
        motion
    });

    let workbench_code = Signal::derive(move || {
        let decimal_places = workbench_decimal_places.get();
        let decimal_separator = workbench_decimal_separator.get();
        let thousand_separator = workbench_thousand_separator.get();
        let motion = workbench_motion.get();

        let mut lines = vec![
            "let (value, set_value) = signal(12345.67_f64);".to_string(),
            "<SlidingNumber".to_string(),
            "  number=Signal::derive(move || value.get())".to_string(),
            format!("  pad_start={}", bool_word(workbench_pad_start.get())),
        ];
        if let Some(separator) = decimal_separator {
            lines.push(format!("  decimal_separator={separator:?}.into()"));
        }
        if let Some(places) = decimal_places {
            lines.push(format!("  decimal_places={places}"));
        }
        if let Some(separator) = thousand_separator {
            lines.push(format!("  thousand_separator={separator:?}.into()"));
        }
        if motion != ui::SlidingNumberMotion::default() {
            lines.push(format!(
                "  motion=SlidingNumberMotion {{ animate: {}, ..Default::default() }}",
                motion.animate
            ));
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-sliding-number-custom\".into()".to_string());
        }
        lines.push(if workbench_lang_zh.get() {
            "  lang=\"zh-CN\".into()".to_string()
        } else {
            "  lang=\"en-US\".into()".to_string()
        });
        lines.push(if workbench_dir_rtl.get() {
            "  dir=Some(A11yDirection::Rtl)".to_string()
        } else {
            "  dir=Some(A11yDirection::Ltr)".to_string()
        });
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/text-input/src/number/styles.rs */\n{}",
            ui::text_input::number::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let number = value.get();
        let decimal_separator_source = if workbench_decimal_separator.get().is_some() {
            "custom"
        } else {
            "default"
        };
        let decimal_places_source = if workbench_decimal_places.get().is_some() {
            "custom"
        } else {
            "auto"
        };
        let thousand_separator_source = if workbench_thousand_separator.get().is_some() {
            "custom"
        } else {
            "none"
        };
        let motion = workbench_motion.get();
        let motion_source = if motion == ui::SlidingNumberMotion::default() {
            "default"
        } else {
            "custom"
        };
        let decimal_separator = workbench_decimal_separator.get();
        let decimal_places = workbench_decimal_places.get();
        let thousand_separator = workbench_thousand_separator.get();
        let class_name = if workbench_custom_class.get() {
            Some("docs-sliding-number-custom")
        } else {
            None
        };
        let class_source = if workbench_custom_class.get() {
            "custom"
        } else {
            "default"
        };
        let mut classes = vec![
            "ui-sliding-number".to_string(),
            format!(
                "data-state:{}",
                if motion.animate { "animated" } else { "static" }
            ),
        ];
        if workbench_custom_class.get() {
            classes.push("docs-sliding-number-custom".to_string());
        }

        format!(
            "SlidingNumberActualConfig {{\n  number: {number},\n  motion: \"{motion_source}\",\n  pad_start: {},\n  decimal_separator: {:?},\n  decimal_places: {:?},\n  thousand_separator: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {},\n  animate: {},\n  decimal_separator_source: \"{decimal_separator_source}\",\n  decimal_places_source: \"{decimal_places_source}\",\n  thousand_separator_source: \"{thousand_separator_source}\",\n  motion_source: \"{motion_source}\",\n  class_source: \"{class_source}\",\n  class: \"{}\",\n}}",
            workbench_pad_start.get(),
            decimal_separator,
            decimal_places,
            thousand_separator,
            class_name,
            if workbench_lang_zh.get() {
                Some("zh-CN")
            } else {
                Some("en-US")
            },
            if workbench_dir_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            motion.animate,
            classes.join(" "),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SlidingNumber
  number=Signal::derive(move || value.get())
  decimal_places=2
  thousand_separator=",".to_string()
/>
<SlidingNumber number=Signal::derive(move || value.get()) decimal_places=0 />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<SlidingNumber
  number=Signal::derive(|| 42123.456)
  decimal_separator=",".to_string()
  decimal_places=30
  thousand_separator=" ".to_string()
  class_name="docs-sliding-number-custom".to_string()
/>
<SlidingNumber
  number=Signal::derive(|| f64::NAN)
  decimal_places=2
  motion=ui::SlidingNumberMotion { animate: false, ..Default::default() }
  class_name="docs-sliding-number-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="SlidingNumber"
            slug="sliding-number"
            group="Display"
            description="Spring-animated number transitions with centralized sign/source/motion attrs."
        >
            <Playground title="Hello World (Default Animated Number)" code_signal=matrix_code>
                <div class="docs-stack">
                    <SlidingNumber
                        number=number_signal
                        decimal_places=2
                        thousand_separator=",".to_string()
                    />
                    <SlidingNumber number=number_signal decimal_places=0 />
                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v += 250.0))
                        >
                            "+250"
                        </ui::Button>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v -= 100.0))
                        >
                            "-100"
                        </ui::Button>
                        <span class="ui-muted">"value: " {move || value.get()}</span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for sliding number motion and format contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="components/text-input/src/number/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sliding-number-workbench-controls">
                        <label class="docs-search__label">
                            "Decimal places"
                            <select
                                prop:value=move || workbench_decimal_places_key.get()
                                on:change=move |ev| set_workbench_decimal_places_key.set(event_target_value(&ev))
                            >
                                <option value="auto">"Auto"</option>
                                <option value="0">"0"</option>
                                <option value="2">"2"</option>
                                <option value="6">"6"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Decimal separator"
                            <select
                                prop:value=move || workbench_decimal_sep_key.get()
                                on:change=move |ev| set_workbench_decimal_sep_key.set(event_target_value(&ev))
                            >
                                <option value="dot">"Default ."</option>
                                <option value="comma">"Custom ,"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Thousand separator"
                            <select
                                prop:value=move || workbench_thousand_sep_key.get()
                                on:change=move |ev| set_workbench_thousand_sep_key.set(event_target_value(&ev))
                            >
                                <option value="none">"None"</option>
                                <option value="comma">"Comma"</option>
                                <option value="space">"Space"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_animate.get()
                                on:change=move |ev| set_workbench_animate.set(event_target_checked(&ev))
                            />
                            " Animate"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " Custom motion"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_pad_start.get()
                                on:change=move |ev| set_workbench_pad_start.set(event_target_checked(&ev))
                            />
                            " Pad start"
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
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_show_compare.get()
                                on:change=move |ev| set_workbench_show_compare.set(event_target_checked(&ev))
                            />
                            " Show compare"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_lang_zh.get()
                                on:change=move |ev| set_workbench_lang_zh.set(event_target_checked(&ev))
                            />
                            " lang zh-CN"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_dir_rtl.get()
                                on:change=move |ev| set_workbench_dir_rtl.set(event_target_checked(&ev))
                            />
                            " dir RTL"
                        </label>
                    </div>
                }
            >
                {move || {
                    let decimal_places = workbench_decimal_places.get();
                    let decimal_separator = workbench_decimal_separator.get().unwrap_or_default();
                    let thousand_separator = workbench_thousand_separator.get().unwrap_or_default();
                    let motion = workbench_motion.get();
                    let show_compare = workbench_show_compare.get();
                    let class_name = if workbench_custom_class.get() {
                        "docs-sliding-number-custom".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <div class="docs-card docs-stack docs-stack--tight">
                                {if let Some(decimal_places) = decimal_places {
                                    view! {
                                        <SlidingNumber
                                            number=number_signal
                                            motion=motion
                                            pad_start=workbench_pad_start.get()
                                            decimal_separator=decimal_separator.clone()
                                            decimal_places=decimal_places
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                            lang=if workbench_lang_zh.get() {
                                                "zh-CN".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if workbench_dir_rtl.get() {
                                                A11yDirection::Rtl
                                            } else {
                                                A11yDirection::Ltr
                                            }
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <SlidingNumber
                                            number=number_signal
                                            motion=motion
                                            pad_start=workbench_pad_start.get()
                                            decimal_separator=decimal_separator.clone()
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                            lang=if workbench_lang_zh.get() {
                                                "zh-CN".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if workbench_dir_rtl.get() {
                                                A11yDirection::Rtl
                                            } else {
                                                A11yDirection::Ltr
                                            }
                                        />
                                    }
                                        .into_any()
                                }}
                                <div class="docs-row">
                                    <ui::Button
                                        variant=ui::ButtonVariant::Secondary
                                        on_press=Callback::new(move |_| set_value.update(|v| *v += 250.0))
                                    >
                                        "+250"
                                    </ui::Button>
                                    <ui::Button
                                        variant=ui::ButtonVariant::Secondary
                                        on_press=Callback::new(move |_| set_value.update(|v| *v -= 100.0))
                                    >
                                        "-100"
                                    </ui::Button>
                                    <span class="ui-muted">"value: " {move || value.get()}</span>
                                </div>
                            </div>

                            <Show when=move || show_compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight">
                                    <SlidingNumber
                                        number=Signal::derive(move || value.get())
                                        decimal_places=2
                                        thousand_separator=",".to_string()
                                    />
                                    <SlidingNumber
                                        number=Signal::derive(move || value.get())
                                        decimal_places=0
                                        motion=ui::SlidingNumberMotion {
                                            animate: false,
                                            ..Default::default()
                                        }
                                        class_name="docs-sliding-number-custom".to_string()
                                    />
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Format + Motion Comparison)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <SlidingNumber
                        number=Signal::derive(move || value.get())
                        decimal_places=2
                        thousand_separator=",".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <SlidingNumber
                        number=Signal::derive(move || value.get())
                        motion=ui::SlidingNumberMotion {
                            animate: false,
                            ..Default::default()
                        }
                        pad_start=true
                        decimal_separator=",".to_string()
                        decimal_places=0
                        thousand_separator=" ".to_string()
                        class_name="docs-sliding-number-custom".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <Playground title="Custom Separators + Motion + Class" code_signal=custom_code>
                <div class="docs-stack">
                    <SlidingNumber
                        number=Signal::derive(|| 42123.456)
                        decimal_separator=",".to_string()
                        decimal_places=30
                        thousand_separator=" ".to_string()
                        class_name="docs-sliding-number-custom".to_string()
                    />
                    <SlidingNumber
                        number=Signal::derive(|| f64::NAN)
                        decimal_places=2
                        motion=ui::SlidingNumberMotion {
                            animate: false,
                            ..Default::default()
                        }
                        class_name="docs-sliding-number-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
