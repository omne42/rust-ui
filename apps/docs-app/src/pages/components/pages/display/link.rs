use super::*;

pub(crate) fn link() -> AnyView {
    let destination_options = vec![
        "internal".to_string(),
        "external".to_string(),
        "missing".to_string(),
    ];
    let (destination_index, set_destination_index) = signal(Some(0_usize));
    let destination_href = Signal::derive(move || match destination_index.get().unwrap_or(0) {
        1 => "https://example.com/docs".to_string(),
        2 => "   ".to_string(),
        _ => "#/docs/welcome".to_string(),
    });
    let destination_label = Signal::derive(move || match destination_index.get().unwrap_or(0) {
        1 => "External docs",
        2 => "Missing href",
        _ => "Internal docs link",
    });

    let rel_options = vec![
        "auto".to_string(),
        "sponsored".to_string(),
        "author + noopener".to_string(),
    ];
    let (rel_index, set_rel_index) = signal(Some(0_usize));
    let rel = Signal::derive(move || match rel_index.get().unwrap_or(0) {
        1 => Some("sponsored".to_string()),
        2 => Some("author noopener".to_string()),
        _ => None,
    });

    let (is_target_blank, set_is_target_blank) = signal(false);
    let (is_disabled, set_is_disabled) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_lang, set_custom_lang) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);

    let workbench_code = Signal::derive(move || {
        let href = destination_href.get();
        let label = destination_label.get();
        let rel = rel.get();
        let is_target_blank = is_target_blank.get();
        let is_disabled = is_disabled.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let custom_lang = custom_lang.get();
        let rtl = rtl_dir.get();

        let mut out = vec!["<Link".to_string(), format!("  href=\"{href}\".into()")];

        if is_target_blank {
            out.push("  target=\"_blank\"".to_string());
        }
        if let Some(rel) = rel {
            out.push(format!("  rel=\"{rel}\".into()"));
        }
        if is_disabled {
            out.push("  is_disabled=true".to_string());
        }
        if custom_aria {
            out.push("  aria_label=\"Open partner documentation\".into()".to_string());
        }
        if custom_class {
            out.push("  class_name=\"docs-link-custom\".into()".to_string());
        }
        if custom_lang {
            out.push("  lang=\"zh-CN\".into()".to_string());
        }
        out.push(format!(
            "  dir={}",
            if rtl {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            }
        ));

        out.push(">".to_string());
        out.push(format!("  \"{label}\""));
        out.push("</Link>".to_string());
        out.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let href = destination_href.get();
        let rel = rel.get();
        let is_target_blank = is_target_blank.get();
        let is_disabled = is_disabled.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let custom_lang = custom_lang.get();
        let dir = if rtl_dir.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };

        let has_href = !href.trim().is_empty();
        let data_state = if is_disabled {
            "disabled"
        } else if has_href {
            "enabled"
        } else {
            "missing-href"
        };
        let target_kind = if is_target_blank { "blank" } else { "self" };
        let rel_source = if rel.is_some() { "provided" } else { "auto" };
        let disabled_source = if is_disabled { "is-prop" } else { "default" };

        let mut classes = vec![
            "ui-link".to_string(),
            format!("ui-link--{data_state}"),
            if rel.is_some() {
                "ui-link--rel-provided".to_string()
            } else {
                "ui-link--rel-auto".to_string()
            },
        ];
        if is_target_blank {
            classes.push("ui-link--external".to_string());
        }
        if custom_aria {
            classes.push("ui-link--with-aria-label".to_string());
        }
        if custom_class {
            classes.push("ui-link--custom-class".to_string());
            classes.push("docs-link-custom".to_string());
        }

        format!(
            "LinkActualConfig {{\n  href: \"{href}\",\n  has_href: {has_href},\n  is_disabled: {is_disabled},\n  disabled_source: \"{disabled_source}\",\n  target: \"{target_kind}\",\n  rel: {:?},\n  rel_source: \"{rel_source}\",\n  aria_label: {:?},\n  class_name: {:?},\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  lang: {},\n  dir: {:?},\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            rel,
            if custom_aria {
                Some("Open partner documentation")
            } else {
                None
            },
            if custom_class {
                Some("docs-link-custom")
            } else {
                None
            },
            if custom_lang { "\"zh-CN\"" } else { "None" },
            dir,
            classes.join(" ")
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/link/src/styles.rs */\n{}",
            ui::link::styles::CSS
        )
    });

    let hello_world_code = Signal::derive(move || {
        r##"<Link href="#/docs/welcome".to_string()>"Read docs"</Link>"##.to_string()
    });

    let matrix_code = Signal::derive(move || {
        r##"<Link href="#/docs/welcome".to_string()>"Internal docs link"</Link>
<Link href="https://example.com".to_string() target="_blank">"External link"</Link>
<Link href="#/docs/welcome".to_string() is_disabled=true>"Disabled"</Link>
<Link href="   ".to_string()>"Missing href"</Link>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="Link"
            slug="link"
            group="Display"
            description="Text link with centralized disabled/target/rel state attrs and headless hover + focus-visible semantics."
        >
            <Playground title="Hello World (Default API)" code_signal=hello_world_code>
                <Link href="#/docs/welcome".to_string()>"Read docs"</Link>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="components/link/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="切换 href/target/disabled/rel/class/lang，并在同一面板查看实际 config + code + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Destination"</div>
                            <SegmentedControl
                                id_base="docs-link-workbench-destination".to_string()
                                options=destination_options.clone()
                                selected_index=destination_index
                                set_selected_index=set_destination_index
                                size=SegmentedControlSize::Sm
                                aria_label="Link destination".to_string()
                            />

                            <div class="docs-search__label">"Rel source"</div>
                            <SegmentedControl
                                id_base="docs-link-workbench-rel".to_string()
                                options=rel_options.clone()
                                selected_index=rel_index
                                set_selected_index=set_rel_index
                                size=SegmentedControlSize::Sm
                                aria_label="Link rel source".to_string()
                            />

                            <Switch checked=is_target_blank set_checked=set_is_target_blank>
                                "target=_blank"
                            </Switch>
                            <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                            <Switch checked=custom_aria set_checked=set_custom_aria>
                                "Custom aria_label"
                            </Switch>
                            <Switch checked=custom_class set_checked=set_custom_class>
                                "Custom class"
                            </Switch>
                            <Switch checked=custom_lang set_checked=set_custom_lang>"Lang=zh-CN"</Switch>
                            <Switch checked=rtl_dir set_checked=set_rtl_dir>"dir=rtl"</Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let href = destination_href.get();
                        let label = destination_label.get();
                        let rel_value = rel.get().unwrap_or_default();
                        let is_disabled = is_disabled.get();
                        let is_target_blank = is_target_blank.get();
                        let aria_label = if custom_aria.get() {
                            "Open partner documentation".to_string()
                        } else {
                            String::new()
                        };
                        let class_name = if custom_class.get() {
                            "docs-link-custom".to_string()
                        } else {
                            String::new()
                        };
                        let lang = if custom_lang.get() {
                            "zh-CN".to_string()
                        } else {
                            String::new()
                        };
                        let dir = if rtl_dir.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        };

                        if is_target_blank {
                            view! {
                                <Link
                                    href=href
                                    target="_blank"
                                    rel=rel_value
                                    is_disabled=is_disabled
                                    aria_label=aria_label
                                    class_name=class_name
                                    lang=lang
                                    dir=dir
                                >
                                    {label}
                                </Link>
                            }
                            .into_any()
                        } else {
                            view! {
                                <Link
                                    href=href
                                    rel=rel_value
                                    is_disabled=is_disabled
                                    aria_label=aria_label
                                    class_name=class_name
                                    lang=lang
                                    dir=dir
                                >
                                    {label}
                                </Link>
                            }
                            .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        {move || format!(
                            "target={}, rel_source={}",
                            if is_target_blank.get() { "_blank" } else { "_self" },
                            if rel.get().is_some() { "provided" } else { "auto" }
                        )}
                    </span>
                </div>
            </Playground>

            <Playground title="Comparison Matrix (Internal / External / Disabled / Missing)" code_signal=matrix_code>
                <div class="docs-row">
                    <Link href="#/docs/welcome".to_string()>"Internal docs link"</Link>
                    <Link href="https://example.com".to_string() target="_blank">
                        "External link"
                    </Link>
                    <Link href="#/docs/welcome".to_string() is_disabled=true>
                        "Disabled"
                    </Link>
                    <Link href="   ".to_string()>"Missing href"</Link>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
