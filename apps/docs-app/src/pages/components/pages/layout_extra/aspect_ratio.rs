use super::*;

pub(crate) fn aspect_ratio() -> AnyView {
    let preset_code = Signal::derive(move || {
        r#"<AspectRatio ratio=AspectRatioPreset::Square radius=AspectRatioRadius::Sm fill=true>
  <View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::None>"1:1"</View>
</AspectRatio>
<AspectRatio ratio=AspectRatioPreset::Video radius=AspectRatioRadius::Md fill=true>
  <View background=ViewBackground::Accent border=ViewBorder::None padding=ViewPadding::Sm radius=ViewRadius::None>"16:9"</View>
</AspectRatio>
<AspectRatio ratio=AspectRatioPreset::Portrait radius=AspectRatioRadius::Md fill=true>
  <View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::None>"3:4"</View>
</AspectRatio>"#.to_string()
    });

    let framed_code = Signal::derive(move || {
        r#"<AspectRatio
  ratio=AspectRatioPreset::UltraWide
  radius=AspectRatioRadius::Lg
  bordered=true
  fill=true
  aria_label="Release trailer preview".to_string()
  class_name="docs-aspect-ratio-custom".to_string()
>
  <View
    background=ViewBackground::Accent
    border=ViewBorder::None
    padding=ViewPadding::Md
    radius=ViewRadius::None
  >
    "21:9 framed media"
  </View>
</AspectRatio>"#
            .to_string()
    });

    let (workbench_ratio_index, set_workbench_ratio_index) = signal(Some(2_usize));
    let workbench_ratio = Signal::derive(move || match workbench_ratio_index.get().unwrap_or(2) {
        0 => AspectRatioPreset::Square,
        1 => AspectRatioPreset::Standard,
        3 => AspectRatioPreset::Portrait,
        4 => AspectRatioPreset::UltraWide,
        _ => AspectRatioPreset::Video,
    });
    let (workbench_radius_index, set_workbench_radius_index) = signal(Some(2_usize));
    let workbench_radius =
        Signal::derive(move || match workbench_radius_index.get().unwrap_or(2) {
            0 => AspectRatioRadius::None,
            1 => AspectRatioRadius::Sm,
            3 => AspectRatioRadius::Lg,
            4 => AspectRatioRadius::Full,
            _ => AspectRatioRadius::Md,
        });
    let (workbench_bordered, set_workbench_bordered) = signal(false);
    let (workbench_fill, set_workbench_fill) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_code = Signal::derive(move || {
        let ratio = workbench_ratio.get();
        let radius = workbench_radius.get();
        let bordered = workbench_bordered.get();
        let fill = workbench_fill.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        format!(
            "<AspectRatio\n  ratio=AspectRatioPreset::{ratio:?}\n  radius=AspectRatioRadius::{radius:?}\n  bordered={bordered}\n  fill={fill}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir={}\n>\n  <View background=ViewBackground::Accent border=ViewBorder::None padding=ViewPadding::Sm radius=ViewRadius::None>\n    \"Workbench preview\"\n  </View>\n</AspectRatio>",
            if custom_aria {
                "\"Workbench media region\".into()"
            } else {
                "\"\".into()"
            },
            if custom_class {
                "\"docs-aspect-ratio-custom\".into()"
            } else {
                "\"\".into()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en-US\".to_string()"
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/aspect_ratio/styles.rs */\n{}",
            ui_layout::aspect_ratio::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let ratio = workbench_ratio.get();
        let radius = workbench_radius.get();
        let bordered = workbench_bordered.get();
        let fill = workbench_fill.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        let mut classes = vec![
            "ui-aspect-ratio".to_string(),
            ratio.class_name().into(),
            radius.class_name().into(),
        ];
        if bordered {
            classes.push("ui-aspect-ratio--bordered".to_string());
        }
        if fill {
            classes.push("ui-aspect-ratio--fill".to_string());
        }
        if custom_class {
            classes.push("ui-aspect-ratio--custom-class".to_string());
            classes.push("docs-aspect-ratio-custom".to_string());
        }

        let state_attr = if bordered && fill {
            "media"
        } else if bordered {
            "framed"
        } else if fill {
            "fill"
        } else {
            "plain"
        };

        format!(
            "AspectRatioActualConfig {{\n  ratio: {ratio:?},\n  radius: {radius:?},\n  bordered: {bordered},\n  fill: {fill},\n  aria_label: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  aria_source: \"{}\",\n  class_source: \"{}\",\n  data_state: \"{state_attr}\",\n  class: \"{}\",\n}}",
            if custom_aria {
                "Workbench media region"
            } else {
                ""
            },
            if custom_class {
                "docs-aspect-ratio-custom"
            } else {
                ""
            },
            if workbench_rtl.get() { "ar" } else { "en-US" },
            if workbench_rtl.get() {
                A11yDirection::Rtl
            } else {
                A11yDirection::Ltr
            },
            if custom_aria { "custom" } else { "default" },
            if custom_class { "custom" } else { "default" },
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="AspectRatio"
            slug="aspect-ratio"
            group="Layout"
            description="baseline-compatible media frame primitive with centralized ratio/radius/frame/source normalization and stable state-marker contracts."
        >
            <Playground title="Ratio Presets" code_signal=preset_code>
                <div class="docs-stack">
                    <AspectRatio ratio=AspectRatioPreset::Square radius=AspectRatioRadius::Sm fill=true>
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::None
                        >
                            "1:1"
                        </View>
                    </AspectRatio>

                    <AspectRatio ratio=AspectRatioPreset::Video radius=AspectRatioRadius::Md fill=true>
                        <View
                            background=ViewBackground::Accent
                            border=ViewBorder::None
                            padding=ViewPadding::Sm
                            radius=ViewRadius::None
                        >
                            "16:9"
                        </View>
                    </AspectRatio>

                    <AspectRatio
                        ratio=AspectRatioPreset::Portrait
                        radius=AspectRatioRadius::Md
                        fill=true
                    >
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::None
                        >
                            "3:4"
                        </View>
                    </AspectRatio>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/aspect_ratio/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="展示区对比 default/workbench 两组；Config 调参，Code 查看当前调用，CSS Test 验证样式契约。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="aspect-ratio-config-controls">
                        <div class="docs-search__label">"Ratio"</div>
                        <button
                            type="button"
                            data-action="toggle-ratio-config"
                            on:click=move |_| {
                                set_workbench_ratio_index.update(|value| {
                                    *value = Some((value.unwrap_or(2) + 1) % 5);
                                });
                            }
                        >
                            "Cycle ratio"
                        </button>
                        <div class="docs-search__label">"Radius"</div>
                        <button
                            type="button"
                            data-action="toggle-radius-config"
                            on:click=move |_| {
                                set_workbench_radius_index.update(|value| {
                                    *value = Some((value.unwrap_or(2) + 1) % 5);
                                });
                            }
                        >
                            "Cycle radius"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-bordered-config"
                            on:click=move |_| {
                                set_workbench_bordered.update(|value| *value = !*value);
                            }
                        >
                            "Toggle bordered"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-fill-config"
                            on:click=move |_| {
                                set_workbench_fill.update(|value| *value = !*value);
                            }
                        >
                            "Toggle fill"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-aria-config"
                            on:click=move |_| {
                                set_workbench_custom_aria.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom aria"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-class-config"
                            on:click=move |_| {
                                set_workbench_custom_class.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom class"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-rtl-config"
                            on:click=move |_| {
                                set_workbench_rtl.update(|value| *value = !*value);
                            }
                        >
                            "Toggle RTL locale"
                        </button>
                        <p class="ui-muted" data-slot="aspect-ratio-config-summary">
                            {move || {
                                format!(
                                    "config: ratio={:?} radius={:?} bordered={} fill={} aria={} class={}",
                                    workbench_ratio.get(),
                                    workbench_radius.get(),
                                    workbench_bordered.get(),
                                    workbench_fill.get(),
                                    if workbench_custom_aria.get() { "custom" } else { "default" },
                                    if workbench_custom_class.get() { "custom" } else { "default" },
                                )
                            }}
                        </p>
                    </div>
                }
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"default"</span>
                            <AspectRatio ratio=AspectRatioPreset::Video radius=AspectRatioRadius::Md fill=true>
                                <View
                                    background=ViewBackground::Subtle
                                    border=ViewBorder::Subtle
                                    padding=ViewPadding::Sm
                                    radius=ViewRadius::None
                                >
                                    "Default preview"
                                </View>
                            </AspectRatio>
                        </div>

                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"workbench"</span>
                            <AspectRatio
                                ratio=workbench_ratio.get()
                                radius=workbench_radius.get()
                                bordered=workbench_bordered.get()
                                fill=workbench_fill.get()
                                aria_label=if workbench_custom_aria.get() {
                                    "Workbench media region".to_string()
                                } else {
                                    "".to_string()
                                }
                                class_name=if workbench_custom_class.get() {
                                    "docs-aspect-ratio-custom".to_string()
                                } else {
                                    "".to_string()
                                }
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
                            >
                                <View
                                    background=ViewBackground::Accent
                                    border=ViewBorder::None
                                    padding=ViewPadding::Sm
                                    radius=ViewRadius::None
                                >
                                    "Workbench preview"
                                </View>
                            </AspectRatio>
                        </div>
                    </div>

                    <span class="ui-muted" data-slot="aspect-ratio-workbench-summary">
                        {move || {
                            format!(
                                "ratio={:?} radius={:?} bordered={} fill={} aria={} class={}",
                                workbench_ratio.get(),
                                workbench_radius.get(),
                                workbench_bordered.get(),
                                workbench_fill.get(),
                                if workbench_custom_aria.get() { "custom" } else { "default" },
                                if workbench_custom_class.get() { "custom" } else { "default" },
                            )
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Preset / Border / Locale Comparison)" code_signal=framed_code>
                <div class="docs-stack">
                    <AspectRatio
                        ratio=AspectRatioPreset::Square
                        radius=AspectRatioRadius::Sm
                        fill=true
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::None
                        >
                            "1:1"
                        </View>
                    </AspectRatio>
                    <AspectRatio
                        ratio=AspectRatioPreset::UltraWide
                        radius=AspectRatioRadius::Lg
                        bordered=true
                        fill=true
                        aria_label="Release trailer preview".to_string()
                        class_name="docs-aspect-ratio-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <View
                            background=ViewBackground::Accent
                            border=ViewBorder::None
                            padding=ViewPadding::Md
                            radius=ViewRadius::None
                        >
                            "21:9 framed media"
                        </View>
                    </AspectRatio>
                    <AspectRatio
                        ratio=AspectRatioPreset::Portrait
                        radius=AspectRatioRadius::Md
                        bordered=false
                        fill=true
                        aria_label="Arabic preview".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    >
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::None
                        >
                            "3:4"
                        </View>
                    </AspectRatio>
                </div>
            </Playground>

            <Playground title="Bordered + Fill + Custom Aria/Class" code_signal=framed_code>
                <AspectRatio
                    ratio=AspectRatioPreset::UltraWide
                    radius=AspectRatioRadius::Lg
                    bordered=true
                    fill=true
                    aria_label="Release trailer preview".to_string()
                    class_name="docs-aspect-ratio-custom".to_string()
                >
                    <View
                        background=ViewBackground::Accent
                        border=ViewBorder::None
                        padding=ViewPadding::Md
                        radius=ViewRadius::None
                    >
                        "21:9 framed media"
                    </View>
                </AspectRatio>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
