use super::*;

pub(crate) fn static_number() -> AnyView {
    let (workbench_number_key, set_workbench_number_key) = signal("positive".to_string());
    let (workbench_decimal_places_key, set_workbench_decimal_places_key) = signal("2".to_string());
    let (workbench_decimal_sep_key, set_workbench_decimal_sep_key) = signal("dot".to_string());
    let (workbench_thousand_sep_key, set_workbench_thousand_sep_key) = signal("comma".to_string());
    let (workbench_pad_start, set_workbench_pad_start) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl_locale, set_workbench_rtl_locale) = signal(false);

    let workbench_number = Signal::derive(move || match workbench_number_key.get().as_str() {
        "negative" => -9876.5,
        "nan" => f64::NAN,
        _ => 12345.67,
    });
    let workbench_decimal_places =
        Signal::derive(move || match workbench_decimal_places_key.get().as_str() {
            "auto" => None,
            "0" => Some(0_u32),
            "6" => Some(6_u32),
            _ => Some(2_u32),
        });
    let workbench_decimal_separator = Signal::derive(move || {
        if workbench_decimal_sep_key.get() == "comma" {
            ",".to_string()
        } else {
            String::new()
        }
    });
    let workbench_thousand_separator =
        Signal::derive(move || match workbench_thousand_sep_key.get().as_str() {
            "none" => String::new(),
            "space" => " ".to_string(),
            _ => ",".to_string(),
        });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-static-number-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<StaticNumber
  number=12345.67
  decimal_places=2
  thousand_separator=",".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "<StaticNumber".to_string(),
            format!("  number={}", workbench_number.get()),
            format!("  pad_start={}", bool_word(workbench_pad_start.get())),
            format!(
                "  decimal_separator={}",
                rust_string_literal(&workbench_decimal_separator.get()),
            ),
            format!(
                "  thousand_separator={}",
                rust_string_literal(&workbench_thousand_separator.get()),
            ),
            format!(
                "  class_name={}",
                rust_string_literal(&workbench_class_name.get()),
            ),
            format!("  lang={}", rust_string_literal(&workbench_lang.get())),
            format!(
                "  dir={}",
                if matches!(workbench_dir.get(), A11yDirection::Rtl) {
                    "A11yDirection::Rtl"
                } else {
                    "A11yDirection::Ltr"
                },
            ),
        ];
        if let Some(decimal_places) = workbench_decimal_places.get() {
            lines.push(format!("  decimal_places={decimal_places}"));
        }
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
        let number = workbench_number.get();
        let sanitized = if number.is_finite() { number } else { 0.0 };
        format!(
            "StaticNumberActualConfig {{\n  number: {number},\n  pad_start: {},\n  decimal_separator: {:?},\n  decimal_places: {:?},\n  thousand_separator: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  sanitized_number: {sanitized},\n}}",
            workbench_pad_start.get(),
            workbench_decimal_separator.get(),
            workbench_decimal_places.get(),
            workbench_thousand_separator.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<StaticNumber
  number=12345.67
  pad_start=false
  decimal_separator=".".to_string()
  decimal_places=2
  thousand_separator=",".to_string()
  class_name="".to_string()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
/>
<StaticNumber
  number=-9876.5
  pad_start=true
  decimal_separator=",".to_string()
  decimal_places=1
  thousand_separator=" ".to_string()
  class_name="docs-static-number-custom".to_string()
  lang="ar".to_string()
  dir=A11yDirection::Rtl
/>
<StaticNumber
  number=f64::NAN
  pad_start=false
  decimal_separator="".to_string()
  thousand_separator="".to_string()
  class_name="docs-static-number-custom".to_string()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="StaticNumber"
            slug="static-number"
            group="Display"
            description="Static number formatting with centralized sign/separator/class source attrs."
        >
            <Playground title="Default Showcase" code_signal=showcase_code>
                <div class="docs-row">
                    <StaticNumber
                        number=12345.67
                        decimal_places=2
                        thousand_separator=",".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (All API Config)"
                description="Button-style playground with display/config/code/css-test panels for number formatting contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="components/text-input/src/number/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="static-number-workbench-controls">
                        <label class="docs-search__label">
                            "Value"
                            <select
                                prop:value=move || workbench_number_key.get()
                                on:change=move |ev| set_workbench_number_key.set(event_target_value(&ev))
                            >
                                <option value="positive">"Positive"</option>
                                <option value="negative">"Negative"</option>
                                <option value="nan">"NaN (sanitized)"</option>
                            </select>
                        </label>
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
                                prop:checked=move || workbench_rtl_locale.get()
                                on:change=move |ev| set_workbench_rtl_locale.set(event_target_checked(&ev))
                            />
                            " lang/dir Arabic"
                        </label>
                    </div>
                }
            >
                {move || {
                    let number = workbench_number.get();
                    let decimal_places = workbench_decimal_places.get();
                    let decimal_separator = workbench_decimal_separator.get();
                    let thousand_separator = workbench_thousand_separator.get();
                    let class_name = workbench_class_name.get();
                    let lang = workbench_lang.get();
                    let dir = workbench_dir.get();

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-card docs-stack docs-stack--tight">
                                {if let Some(decimal_places) = decimal_places {
                                    view! {
                                        <StaticNumber
                                            number=number
                                            pad_start=workbench_pad_start.get()
                                            decimal_separator=decimal_separator.clone()
                                            decimal_places=decimal_places
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                            lang=lang.clone()
                                            dir=dir
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <StaticNumber
                                            number=number
                                            pad_start=workbench_pad_start.get()
                                            decimal_separator=decimal_separator.clone()
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                            lang=lang.clone()
                                            dir=dir
                                        />
                                    }
                                        .into_any()
                                }}
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Locale / Separator / Sign Comparison)"
                code_signal=matrix_code
                code_imports="use ui::color::area::A11yDirection;\nuse ui::StaticNumber;".to_string()
            >
                <div class="docs-row">
                    <StaticNumber
                        number=12345.67
                        pad_start=false
                        decimal_separator=".".to_string()
                        decimal_places=2
                        thousand_separator=",".to_string()
                        class_name="".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <StaticNumber
                        number=-9876.5
                        pad_start=true
                        decimal_separator=",".to_string()
                        decimal_places=1
                        thousand_separator=" ".to_string()
                        class_name="docs-static-number-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                    <StaticNumber
                        number=f64::NAN
                        pad_start=false
                        decimal_separator="".to_string()
                        thousand_separator="".to_string()
                        class_name="docs-static-number-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
