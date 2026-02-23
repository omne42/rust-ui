use super::*;

pub(crate) fn contextual_help() -> AnyView {
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_controlled_open_change =
        Callback::new(move |next: bool| set_controlled_open_raw.set(next));
    let toggle_controlled: OnPress = Callback::new(move |_| {
        set_controlled_open_raw.update(|open| *open = !*open);
    });

    let semantic_code = Signal::derive(move || {
        r#"<ContextualHelp
  heading="Contextual help".to_string()
  footer=move || view! { "Popover-based" }
>
  <div>"Content"</div>
</ContextualHelp>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);

<ContextualHelp
  variant=ContextualHelpVariant::Info
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))
  aria_label="More info".to_string()
  class_name="docs-contextual-help-custom".to_string()
>
  <div>"Controlled content"</div>
</ContextualHelp>"#
            .to_string()
    });
    let output_mode_code = Signal::derive(move || {
        r#"<ContextualHelp
  heading="LLM output contract".to_string()
  footer=move || view! { "Streaming Optional; fallback=snapshot." }
>
  <div>"This component defaults to snapshot rendering while exposing streaming/snapshot markers."</div>
</ContextualHelp>"#
            .to_string()
    });

    let variant_options = vec!["help".to_string(), "info".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant_value: Signal<ContextualHelpVariant> =
        Signal::derive(move || match variant_index.get().unwrap_or(0) {
            1 => ContextualHelpVariant::Info,
            _ => ContextualHelpVariant::Help,
        });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let on_workbench_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let toggle_workbench_open: OnPress =
        Callback::new(move |_| set_workbench_open_raw.update(|open| *open = !*open));
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_top_end, set_workbench_top_end) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let workbench_code = Signal::derive(move || {
        let variant = variant_value.get();
        let controlled_mode = workbench_controlled.get();
        let disabled = workbench_disabled.get();
        let open = workbench_open_raw.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let top_end = workbench_top_end.get();
        let custom_motion = workbench_custom_motion.get();
        let custom_id = workbench_custom_id.get();
        let zh_lang = workbench_zh_lang.get();
        let rtl_dir = workbench_rtl_dir.get();

        let mut lines = vec!["<ContextualHelp".to_string()];
        if variant != ContextualHelpVariant::Help {
            lines.push("  variant=ContextualHelpVariant::Info".to_string());
        }
        if controlled_mode {
            lines.push("  open=Signal::derive(move || open_raw.get())".to_string());
            lines.push(
                "  on_open_change=Callback::new(move |next| set_open_raw.set(next))".to_string(),
            );
        } else {
            lines.push(format!("  default_open={open}"));
        }
        if disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if top_end {
            lines.push("  placement=PopoverPlacement::TopEnd".to_string());
        }
        if custom_motion {
            lines.push(
                "  motion=ContextualHelpMotion { popover: PopoverMotion { initial_scale: 0.95, offset_y_px: 10.0, ..PopoverMotion::default() } }".to_string(),
            );
        }
        lines.push("  heading=\"Contextual help\".into()".to_string());
        lines.push("  footer=move || view! { \"Popover-based\" }".to_string());
        if custom_id {
            lines.push("  id=\"docs-contextual-help-workbench\".into()".to_string());
        }
        lines.push(format!(
            "  lang={:?}.into()",
            if zh_lang { "zh-CN" } else { "en-US" }
        ));
        lines.push(format!(
            "  dir={}",
            if rtl_dir {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            }
        ));
        if custom_aria {
            lines.push("  aria_label=\"More info\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-contextual-help-custom\".into()".to_string());
        }
        lines.push(">".to_string());
        lines.push("  <div>\"Workbench content\"</div>".to_string());
        lines.push("</ContextualHelp>".to_string());
        lines.join("\n")
    });

    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/contextual_help/styles.rs */\n{}",
            ui::contextual_help::styles::CSS
        )
    });

    let actual_config = Signal::derive(move || {
        let variant = variant_value.get();
        let disabled = workbench_disabled.get();
        let controlled_mode = workbench_controlled.get();
        let open = workbench_open_raw.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let top_end = workbench_top_end.get();
        let custom_motion = workbench_custom_motion.get();
        let custom_id = workbench_custom_id.get();
        let zh_lang = workbench_zh_lang.get();
        let rtl_dir = workbench_rtl_dir.get();

        let mut class_tokens = vec!["ui-contextual-help".to_string()];
        class_tokens.push(match variant {
            ContextualHelpVariant::Help => "ui-contextual-help--variant-help".to_string(),
            ContextualHelpVariant::Info => "ui-contextual-help--variant-info".to_string(),
        });
        class_tokens.push(if disabled {
            "ui-contextual-help--disabled".to_string()
        } else {
            "ui-contextual-help--enabled".to_string()
        });
        class_tokens.push(if controlled_mode {
            "ui-contextual-help--controlled".to_string()
        } else {
            "ui-contextual-help--uncontrolled".to_string()
        });
        if custom_class {
            class_tokens.push("ui-contextual-help--custom-class".to_string());
            class_tokens.push("docs-contextual-help-custom".to_string());
        }

        format!(
            "ContextualHelpActualConfig {{\n  variant: {variant:?},\n  aria_label: {},\n  is_disabled: {disabled},\n  disabled: Some({disabled}),\n  placement: {},\n  motion: {},\n  open: {},\n  default_open: {},\n  on_open_change: {},\n  heading: Some(\"Contextual help\"),\n  footer: Some(\"Popover-based\"),\n  class_name: {},\n  id: {},\n  lang: {},\n  dir: {},\n  controlled_mode: {controlled_mode},\n  custom_aria_label: {custom_aria},\n  custom_class_name: {custom_class},\n  class: \"{}\",\n}}",
            if custom_aria {
                "Some(\"More info\")"
            } else {
                "None"
            },
            if top_end {
                "PopoverPlacement::TopEnd"
            } else {
                "PopoverPlacement::BottomStart"
            },
            if custom_motion {
                "ContextualHelpMotion::custom"
            } else {
                "ContextualHelpMotion::default"
            },
            if controlled_mode {
                format!("Some({open})")
            } else {
                "None".to_string()
            },
            if controlled_mode {
                "None".to_string()
            } else {
                format!("Some({open})")
            },
            if controlled_mode {
                "Some(\"provided\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-contextual-help-custom\")"
            } else {
                "None"
            },
            if custom_id {
                "Some(\"docs-contextual-help-workbench\")"
            } else {
                "None"
            },
            if zh_lang {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if rtl_dir {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            class_tokens.join(" ")
        )
    });

    let comparison_code = Signal::derive(move || {
        r#"<ContextualHelp heading="Help".to_string() footer=move || view! { "Default" }>
  <div>"Default Help"</div>
</ContextualHelp>
<ContextualHelp variant=ContextualHelpVariant::Info heading="Info".to_string() footer=move || view! { "Info Variant" }>
  <div>"Info Help"</div>
</ContextualHelp>
<ContextualHelp variant=ContextualHelpVariant::Info is_disabled=true aria_label="Disabled info".to_string() class_name="docs-contextual-help-custom".to_string()>
  <div>"Disabled Trigger"</div>
</ContextualHelp>"#.to_string()
    });

    view! {
        <ComponentPage
            title="ContextualHelp"
            slug="contextual-help"
            group="Overlays"
            description="Non-modal popover help trigger with centralized variant/placement/heading/footer state attrs."
        >
            <Playground title="Hello World (Default API)" code_signal=semantic_code>
                <div class="docs-row">
                    <ContextualHelp
                        heading="Contextual help".to_string()
                        footer=move || view! { "Popover-based" }
                    >
                        <div class="docs-stack">
                            <div>"Uses Button + Popover + spring motion."</div>
                            <div class="ui-muted">"Works in Light/Dark/OLED via tokens."</div>
                        </div>
                    </ContextualHelp>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-like playground surface: display/config/code/css-test with stable state/source markers."
                code_signal=workbench_code
                test_css_source=test_css_source
                test_source_path="crates/ui/src/contextual_help/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-contextual-help-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ContextualHelp variant".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_controlled set_checked=set_workbench_controlled>
                            "Controlled mode"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_top_end set_checked=set_workbench_top_end>
                            "placement TopEnd"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                            "Custom id"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let open = workbench_open_raw.get();
                    let controlled_mode = workbench_controlled.get();
                    let disabled = workbench_disabled.get();
                    let custom_aria = workbench_custom_aria.get();
                    let custom_class = workbench_custom_class.get();
                    let variant = variant_value.get();
                    let top_end = workbench_top_end.get();
                    let custom_motion = workbench_custom_motion.get();
                    let custom_id = workbench_custom_id.get();
                    let zh_lang = workbench_zh_lang.get();
                    let rtl_dir = workbench_rtl_dir.get();
                    let aria_label = if custom_aria {
                        "More info".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-contextual-help-custom".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-row">
                                <Button variant=ButtonVariant::Secondary on_press=toggle_workbench_open>
                                    "Toggle workbench open"
                                </Button>
                                <span class="ui-muted">
                                    "mode: " {if controlled_mode { "controlled" } else { "uncontrolled" }}
                                    " | open: " {open}
                                </span>
                            </div>

                            <div class="docs-row">
                                {if controlled_mode {
                                    view! {
                                        <ContextualHelp
                                            variant=variant
                                            open=workbench_open
                                            on_open_change=on_workbench_open_change
                                            is_disabled=disabled
                                            heading="Contextual help".to_string()
                                            footer=move || view! { "Popover-based" }
                                            placement=if top_end {
                                                ui_headless::PopoverPlacement::TopEnd
                                            } else {
                                                ui_headless::PopoverPlacement::BottomStart
                                            }
                                            motion=if custom_motion {
                                                ui::ContextualHelpMotion {
                                                    popover: PopoverMotion {
                                                        initial_scale: 0.95,
                                                        offset_y_px: 10.0,
                                                        ..PopoverMotion::default()
                                                    },
                                                }
                                            } else {
                                                ui::ContextualHelpMotion::default()
                                            }
                                            aria_label=aria_label.clone()
                                            class_name=class_name.clone()
                                            id=if custom_id {
                                                "docs-contextual-help-workbench".to_string()
                                            } else {
                                                String::new()
                                            }
                                            lang=if zh_lang {
                                                "zh-CN".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if rtl_dir {
                                                ui_headless::A11yDirection::Rtl
                                            } else {
                                                ui_headless::A11yDirection::Ltr
                                            }
                                        >
                                            <div class="docs-stack docs-stack--tight">
                                                <div>"Workbench content"</div>
                                                <div class="ui-muted">"Inspect data-state / data-open-mode / data-*-source markers."</div>
                                            </div>
                                        </ContextualHelp>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <ContextualHelp
                                            variant=variant
                                            default_open=open
                                            is_disabled=disabled
                                            heading="Contextual help".to_string()
                                            footer=move || view! { "Popover-based" }
                                            placement=if top_end {
                                                ui_headless::PopoverPlacement::TopEnd
                                            } else {
                                                ui_headless::PopoverPlacement::BottomStart
                                            }
                                            motion=if custom_motion {
                                                ui::ContextualHelpMotion {
                                                    popover: PopoverMotion {
                                                        initial_scale: 0.95,
                                                        offset_y_px: 10.0,
                                                        ..PopoverMotion::default()
                                                    },
                                                }
                                            } else {
                                                ui::ContextualHelpMotion::default()
                                            }
                                            aria_label=aria_label
                                            class_name=class_name
                                            id=if custom_id {
                                                "docs-contextual-help-workbench".to_string()
                                            } else {
                                                String::new()
                                            }
                                            lang=if zh_lang {
                                                "zh-CN".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if rtl_dir {
                                                ui_headless::A11yDirection::Rtl
                                            } else {
                                                ui_headless::A11yDirection::Ltr
                                            }
                                        >
                                            <div class="docs-stack docs-stack--tight">
                                                <div>"Workbench content"</div>
                                                <div class="ui-muted">"Inspect data-state / data-open-mode / data-*-source markers."</div>
                                            </div>
                                        </ContextualHelp>
                                    }
                                        .into_any()
                                }}
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix" code_signal=comparison_code>
                <div class="docs-row">
                    <ContextualHelp heading="Help".to_string() footer=move || view! { "Default" }>
                        <div>"Default Help"</div>
                    </ContextualHelp>
                    <ContextualHelp
                        variant=ContextualHelpVariant::Info
                        heading="Info".to_string()
                        footer=move || view! { "Info Variant" }
                    >
                        <div>"Info Help"</div>
                    </ContextualHelp>
                    <ContextualHelp
                        variant=ContextualHelpVariant::Info
                        is_disabled=true
                        aria_label="Disabled info".to_string()
                        class_name="docs-contextual-help-custom".to_string()
                    >
                        <div>"Disabled Trigger"</div>
                    </ContextualHelp>
                </div>
            </Playground>

            <Playground title="Info Variant + Controlled" code_signal=controlled_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=toggle_controlled>
                            "Toggle controlled help"
                        </Button>
                        <span class="ui-muted">"open: " {move || controlled_open_raw.get()}</span>
                    </div>

                    <ContextualHelp
                        variant=ContextualHelpVariant::Info
                        open=controlled_open
                        on_open_change=on_controlled_open_change
                        aria_label="More info".to_string()
                        class_name="docs-contextual-help-custom".to_string()
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Controlled mode keeps parent state as the source of truth."</div>
                            <div class="ui-muted">"No heading path falls back to aria-label on panel."</div>
                        </div>
                    </ContextualHelp>
                </div>
            </Playground>

            <section class="docs-card docs-prose" attr:data-slot="contextual-help-api-matrix">
                <h3>"API Matrix"</h3>
                <ul attr:data-slot="contextual-help-api-rows">
                    <li>
                        <code>"variant: ContextualHelpVariant"</code>
                        " "
                        {format!(
                            "default = ContextualHelpVariant::{:?} ({})",
                            ContextualHelpVariant::default(),
                            ContextualHelpVariant::default().class_name()
                        )}
                    </li>
                    <li>
                        <code>"placement: PopoverPlacement"</code>
                        " "
                        {format!(
                            "default = PopoverPlacement::{:?} ({})",
                            ui_headless::PopoverPlacement::default(),
                            ui_headless::PopoverPlacement::default().as_str()
                        )}
                    </li>
                    <li>
                        <code>"open + on_open_change + default_open"</code>
                        " default path = uncontrolled (open absent); `default_open` omitted => internal false"
                    </li>
                    <li>
                        <code>"is_disabled: Option<bool>"</code>
                        " default = false"
                    </li>
                    <li>
                        <code>"disabled: Option<bool>"</code>
                        " compatibility alias for `is_disabled`; precedence = is_disabled -> disabled -> false"
                    </li>
                    <li>
                        <code>"heading/footer/class_name/aria_label/lang/dir/id"</code>
                        " default = None (id auto-generated from IdProvider; fallback = \"ui-contextual-help-0\")"
                    </li>
                    <li>
                        <code>"motion: ContextualHelpMotion"</code>
                        " default = ContextualHelpMotion::default()"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" attr:data-slot="contextual-help-state-matrix">
                <h3>"State Matrix"</h3>
                <ul attr:data-slot="contextual-help-state-rows">
                    <li>
                        <code>"data-open-mode"</code>
                        " = controlled | uncontrolled"
                    </li>
                    <li>
                        <code>"data-state"</code>
                        " = enabled | disabled"
                    </li>
                    <li>
                        <code>"data-variant"</code>
                        " = help | info"
                    </li>
                    <li>
                        <code>"data-placement"</code>
                        " = bottom-start | bottom-end | top-start | top-end"
                    </li>
                    <li>
                        <code>"data-open-source / data-default-open-source / data-open-change-source"</code>
                        " = custom|default / provided|implicit / provided|none"
                    </li>
                    <li>
                        <code>"size axis"</code>
                        " = N/A (ContextualHelp trigger is fixed ButtonSize::IconSm)"
                    </li>
                </ul>
            </section>

            <Playground title="Streaming/Snapshot Display" code_signal=output_mode_code>
                <div class="docs-stack docs-stack--tight">
                    <ContextualHelp
                        heading="LLM output contract".to_string()
                        footer=move || view! { "Streaming Optional; fallback=snapshot." }
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Snapshot is the baseline rendering mode for ContextualHelp."</div>
                            <div class="ui-muted">
                                "Mode contract stays machine-readable via data-ui-output-mode=snapshot|streaming."
                            </div>
                        </div>
                    </ContextualHelp>
                </div>
            </Playground>

            <section class="docs-card docs-prose" attr:data-slot="contextual-help-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground already supports "
                    <code>"Show code"</code>
                    " with copy action. The copied snippet is import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::*;\n\n<ContextualHelp heading=\"Contextual help\".to_string()>\n  <div>\"Need help?\"</div>\n</ContextualHelp>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-contextual-help-source-copy".to_string()
                />
                <ul attr:data-slot="contextual-help-source-paths">
                    <li><code>"components/contextual-help/src/mod.rs"</code></li>
                    <li><code>"components/contextual-help/src/logic.rs"</code></li>
                    <li><code>"components/contextual-help/src/view.rs"</code></li>
                    <li><code>"components/contextual-help/src/styles.rs"</code></li>
                    <li><code>"components/contextual-help/src/motion.rs"</code></li>
                </ul>
                <ul attr:data-slot="contextual-help-source-prerequisites">
                    <li><code>"component-contextual_help"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
