use super::*;

pub(crate) fn badge() -> AnyView {
    let hello_world_code = Signal::derive(move || r#"<Badge>"New"</Badge>"#.to_string());

    let matrix_code = Signal::derive(move || {
        r#"<Badge variant=BadgeVariant::Default>"Default"</Badge>
<Badge variant=BadgeVariant::Accent>"Accent"</Badge>
<Badge variant=BadgeVariant::Danger>"Danger"</Badge>
<Badge variant=BadgeVariant::Outline>"Outline"</Badge>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Badge variant=BadgeVariant::Accent class_name="docs-badge-custom".to_string()>
  "Release"
</Badge>
<Badge variant=BadgeVariant::Outline class_name="docs-badge-custom".to_string()>
  "Beta"
</Badge>"#
            .to_string()
    });

    let variant_options = vec![
        "default".to_string(),
        "accent".to_string(),
        "danger".to_string(),
        "outline".to_string(),
    ];
    let locale_options = vec!["en-US".to_string(), "zh-CN".to_string(), "ar".to_string()];
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_locale_index, set_workbench_locale_index) = signal(Some(0_usize));
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => BadgeVariant::Accent,
            2 => BadgeVariant::Danger,
            3 => BadgeVariant::Outline,
            _ => BadgeVariant::Default,
        });

    let workbench_code = Signal::derive(move || {
        let variant = workbench_variant.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => Some("zh-CN"),
            2 => Some("ar"),
            _ => None,
        };
        let label = match locale_index {
            1 => "新品",
            2 => "جديد",
            _ => "New",
        };

        let mut lines = vec!["<Badge".to_string()];
        if variant != BadgeVariant::Default {
            lines.push(format!("  variant=BadgeVariant::{variant:?}"));
        }
        if custom_class {
            lines.push("  class_name=\"docs-badge-custom\".into()".to_string());
        }
        if let Some(lang) = lang {
            lines.push(format!("  lang=\"{lang}\".into()"));
        }
        if rtl {
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.extend([
            ">".to_string(),
            format!("  \"{label}\""),
            "</Badge>".to_string(),
        ]);
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/badge/src/styles.rs */\n{}",
            ui::badge::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant = workbench_variant.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => "zh-CN",
            2 => "ar",
            _ => "default",
        };

        let mut class = vec![
            "ui-badge".to_string(),
            variant.class_name().into(),
            variant.fill_class().into(),
        ];
        if custom_class {
            class.push("ui-badge--custom-class".to_string());
            class.push("docs-badge-custom".to_string());
        }

        format!(
            "BadgeActualConfig {{\n  variant: {variant:?},\n  class_name: {},\n  variant_attr: \"{}\",\n  fill_attr: \"{}\",\n  class_source: \"{}\",\n  lang: \"{lang}\",\n  dir: \"{}\",\n  class: \"{}\",\n}}",
            if custom_class {
                "Some(\"docs-badge-custom\")"
            } else {
                "None"
            },
            variant.as_attr(),
            variant.fill_attr(),
            if custom_class { "custom" } else { "default" },
            if rtl { "rtl" } else { "auto" },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Badge"
            slug="badge"
            group="Display"
            description="Status badge with centralized variant/fill state attrs and custom-class contract."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="docs-row">
                    <Badge>"New"</Badge>
                </div>
            </Playground>

            <Playground
                title="Badge Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/badge/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="Button-like workbench: display compare + live config/code/css test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="badge-workbench-controls">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-badge-workbench-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Badge variant".to_string()
                        />

                        <div class="docs-search__label">"Locale"</div>
                        <SegmentedControl
                            id_base="docs-badge-workbench-locale".to_string()
                            options=locale_options.clone()
                            selected_index=workbench_locale_index
                            set_selected_index=set_workbench_locale_index
                            size=SegmentedControlSize::Sm
                            aria_label="Badge locale".to_string()
                        />

                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL direction"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let variant = workbench_variant.get();
                    let locale_index = workbench_locale_index.get().unwrap_or(0);
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();
                    let lang = match locale_index {
                        1 => "zh-CN".to_string(),
                        2 => "ar".to_string(),
                        _ => String::new(),
                    };
                    let label = match locale_index {
                        1 => "新品",
                        2 => "جديد",
                        _ => "New",
                    };
                    let class_name = if custom_class {
                        "docs-badge-custom".to_string()
                    } else {
                        String::new()
                    };
                    let dir = if rtl {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    view! {
                        <div class="docs-stack" data-slot="badge-workbench-compare">
                            <div class="docs-row">
                                <div class="docs-stack docs-stack--tight">
                                    <div class="docs-search__label">"Baseline"</div>
                                    <Badge>"New"</Badge>
                                </div>
                                <div class="docs-stack docs-stack--tight">
                                    <div class="docs-search__label">"Configured"</div>
                                    <Badge variant=variant class_name=class_name lang=lang dir=dir>
                                        {label}
                                    </Badge>
                                </div>
                            </div>

                            <div class="docs-search__label">"Scenario compare"</div>
                            <div class="docs-row">
                                <Badge variant=BadgeVariant::Default>"default"</Badge>
                                <Badge variant=BadgeVariant::Accent>"accent"</Badge>
                                <Badge variant=BadgeVariant::Danger>"danger"</Badge>
                                <Badge variant=BadgeVariant::Outline>"outline"</Badge>
                            </div>
                        </div>
                    }
                }}
            </Playground>



            <Playground title="Variants (Default / Accent / Danger / Outline)" code_signal=matrix_code>
                <div class="docs-row">
                    <Badge variant=BadgeVariant::Default>"Default"</Badge>
                    <Badge variant=BadgeVariant::Accent>"Accent"</Badge>
                    <Badge variant=BadgeVariant::Danger>"Danger"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                </div>
            </Playground>

            <Playground title="Custom Class + Outline" code_signal=custom_code>
                <div class="docs-row">
                    <Badge variant=BadgeVariant::Accent class_name="docs-badge-custom".to_string()>
                        "Release"
                    </Badge>
                    <Badge variant=BadgeVariant::Outline class_name="docs-badge-custom".to_string()>
                        "Beta"
                    </Badge>
                </div>
            </Playground>



            <Playground
                title="Comparison Matrix (Variant + Fill)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <Badge variant=BadgeVariant::Default>"default"</Badge>
                    <Badge variant=BadgeVariant::Accent>"accent"</Badge>
                    <Badge variant=BadgeVariant::Danger>"danger"</Badge>
                    <Badge variant=BadgeVariant::Outline>"outline"</Badge>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
