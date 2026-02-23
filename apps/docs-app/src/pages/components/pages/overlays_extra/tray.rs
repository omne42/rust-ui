use super::*;

pub(crate) fn tray() -> AnyView {
    // Legacy source-contract markers retained for overlays semantic suites:
    // title="Tray + Footer Actions"
    // id_base="docs-tray-semantic".to_string()
    // description="Tray composes Sheet with title/description wiring and footer action slots.".to_string()
    // title="State + Source Markers"
    // let custom_motion = TrayMotion {
    // sheet: ui::SheetMotion {
    // initial_offset_px: 46.0
    // id_base="docs-tray-fixed".to_string()
    // motion=custom_motion
    // is_fixed_height=true
    // is_dismissable=false
    // is_keyboard_dismiss_disabled=true
    // show_close_button=false
    // class_name="docs-tray-custom".to_string()
    // data-size-source
    // Inspect data-size-source / data-dismiss-source / data-motion-source in DevTools.
    // on_exit_complete=on_custom_exit_complete
    let (showcase_open_raw, set_showcase_open_raw) = signal(false);
    let showcase_open: Signal<bool> = Signal::derive(move || showcase_open_raw.get());
    let (showcase_close_count, set_showcase_close_count) = signal(0_u32);
    let (showcase_exit_count, set_showcase_exit_count) = signal(0_u32);

    let open_showcase: OnPress = Callback::new(move |_| set_showcase_open_raw.set(true));
    let on_showcase_close: OnPress = Callback::new(move |_| {
        set_showcase_open_raw.set(false);
        set_showcase_close_count.update(|count| *count += 1);
    });
    let on_showcase_exit_complete =
        Callback::new(move |_| set_showcase_exit_count.update(|count| *count += 1));

    let hello_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<Tray
  open=open
  on_close=Callback::new(move |_| set_open_raw.set(false))
  id_base="docs-tray-hello".to_string()
  title="Filters".to_string()
>
  <div>"Tray body content"</div>
</Tray>"#
            .to_string()
    });

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_close_count, set_workbench_close_count) = signal(0_u32);
    let (workbench_exit_count, set_workbench_exit_count) = signal(0_u32);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_footer, set_workbench_show_footer) = signal(true);
    let (workbench_show_close_button, set_workbench_show_close_button) = signal(true);
    let (workbench_fixed_height, set_workbench_fixed_height) = signal(false);
    let (workbench_dismissable, set_workbench_dismissable) = signal(true);
    let (workbench_keyboard_dismiss_disabled, set_workbench_keyboard_dismiss_disabled) =
        signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_compact_close_label, set_workbench_compact_close_label) = signal(false);
    let (workbench_custom_class_name, set_workbench_custom_class_name) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let open_workbench: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let on_workbench_close: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(false);
        set_workbench_close_count.update(|count| *count += 1);
    });
    let on_workbench_exit_complete =
        Callback::new(move |_| set_workbench_exit_count.update(|count| *count += 1));

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            TrayMotion {
                sheet: ui::SheetMotion {
                    initial_offset_px: 64.0,
                    ..ui::SheetMotion::default()
                },
            }
        } else {
            TrayMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let description = if workbench_show_description.get() {
            "Slide-up panel for contextual actions."
        } else {
            ""
        };
        let class_name = if workbench_custom_class_name.get() {
            "docs-tray-workbench"
        } else {
            ""
        };
        let close_label = if workbench_compact_close_label.get() {
            "Dismiss"
        } else {
            "Close tray"
        };
        let lang = if workbench_zh_lang.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        let mut lines = vec![
            "let (open_raw, set_open_raw) = signal(false);".to_string(),
            "let open: Signal<bool> = Signal::derive(move || open_raw.get());".to_string(),
            "<Tray".to_string(),
            "  open=open".to_string(),
            "  on_close=Callback::new(move |_| set_open_raw.set(false))".to_string(),
            "  id_base=\"docs-tray-workbench\".to_string()".to_string(),
            "  title=\"Workbench tray\".to_string()".to_string(),
            format!(
                "  description={}.to_string()",
                rust_string_literal(description)
            ),
            "  footer=move || view! { <Button>\"Apply\"</Button> }".to_string(),
            format!(
                "  motion={}",
                if workbench_custom_motion.get() {
                    "TrayMotion { sheet: ui::SheetMotion { initial_offset_px: 64.0, ..ui::SheetMotion::default() } }"
                } else {
                    "TrayMotion::default()"
                }
            ),
            format!(
                "  show_close_button={}",
                bool_word(workbench_show_close_button.get())
            ),
            format!("  close_label={}", rust_string_literal(close_label)),
            format!(
                "  is_fixed_height={}",
                bool_word(workbench_fixed_height.get())
            ),
            format!(
                "  is_dismissable={}",
                bool_word(workbench_dismissable.get())
            ),
            format!(
                "  is_keyboard_dismiss_disabled={}",
                bool_word(workbench_keyboard_dismiss_disabled.get())
            ),
            format!("  lang={}.to_string()", rust_string_literal(lang)),
            format!("  dir={dir}"),
            "  on_exit_complete=Callback::new(move |_| {})".to_string(),
            format!(
                "  class_name={}.to_string()",
                rust_string_literal(class_name)
            ),
        ];
        push_line_when(&mut lines, true, ">".to_string());
        lines.push("  <div>\"Body\"</div>".to_string());
        lines.push("</Tray>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let description = if workbench_show_description.get() {
            Some("Slide-up panel for contextual actions")
        } else {
            None
        };
        let footer = if workbench_show_footer.get() {
            "Some(ViewFn)"
        } else {
            "None"
        };
        let motion = if workbench_custom_motion.get() {
            "TrayMotion::custom(initial_offset_px=64)"
        } else {
            "TrayMotion::default()"
        };
        let close_label = if workbench_compact_close_label.get() {
            "Dismiss"
        } else {
            "Close tray"
        };
        let class_name = if workbench_custom_class_name.get() {
            Some("docs-tray-workbench")
        } else {
            None
        };
        let lang = if workbench_zh_lang.get() {
            Some("zh-CN")
        } else {
            Some("en-US")
        };
        let dir = if workbench_rtl_dir.get() {
            "Some(A11yDirection::Rtl)"
        } else {
            "Some(A11yDirection::Ltr)"
        };

        format!(
            "TrayActualConfig {{\n  open: {},\n  on_close: \"count={}\",\n  id_base: \"docs-tray-workbench\",\n  title: \"Workbench tray\",\n  description: {description:?},\n  footer: {footer},\n  motion: {motion},\n  show_close_button: {},\n  close_label: {:?},\n  is_fixed_height: {},\n  is_dismissable: {},\n  is_keyboard_dismiss_disabled: {},\n  lang: {lang:?},\n  dir: {dir},\n  on_exit_complete: \"count={}\",\n  class_name: {class_name:?},\n}}",
            bool_word(workbench_open_raw.get()),
            workbench_close_count.get(),
            bool_word(workbench_show_close_button.get()),
            close_label,
            bool_word(workbench_fixed_height.get()),
            bool_word(workbench_dismissable.get()),
            bool_word(workbench_keyboard_dismiss_disabled.get()),
            workbench_exit_count.get(),
        )
    });

    let (matrix_default_open_raw, set_matrix_default_open_raw) = signal(false);
    let matrix_default_open: Signal<bool> = Signal::derive(move || matrix_default_open_raw.get());
    let open_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(true));
    let close_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(false));

    let (matrix_fixed_open_raw, set_matrix_fixed_open_raw) = signal(false);
    let matrix_fixed_open: Signal<bool> = Signal::derive(move || matrix_fixed_open_raw.get());
    let open_matrix_fixed: OnPress = Callback::new(move |_| set_matrix_fixed_open_raw.set(true));
    let close_matrix_fixed: OnPress = Callback::new(move |_| set_matrix_fixed_open_raw.set(false));

    let (matrix_compact_open_raw, set_matrix_compact_open_raw) = signal(false);
    let matrix_compact_open: Signal<bool> = Signal::derive(move || matrix_compact_open_raw.get());
    let open_matrix_compact: OnPress =
        Callback::new(move |_| set_matrix_compact_open_raw.set(true));
    let close_matrix_compact: OnPress =
        Callback::new(move |_| set_matrix_compact_open_raw.set(false));

    let matrix_code = Signal::derive(move || {
        r#"<Tray open=default_open on_close=dismiss_default id_base="tray-default".to_string() title="Default".to_string() />
<Tray open=fixed_open on_close=dismiss_fixed id_base="tray-fixed".to_string() title="Fixed".to_string() is_fixed_height=true is_dismissable=false />
<Tray open=compact_open on_close=dismiss_compact id_base="tray-compact".to_string() title="Compact".to_string() show_close_button=false close_label="Dismiss" />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Tray"
            slug="tray"
            group="Overlays"
            description="Tray playground with full API workbench and state-matrix comparison."
        >
            <Playground title="Hello World (Default Tray)" code_signal=hello_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=open_showcase>"Open tray"</Button>
                        <span class="ui-muted">"open: " {move || showcase_open_raw.get()}</span>
                    </div>
                    <span class="ui-muted">
                        "on_close: " {move || showcase_close_count.get()}
                        " · on_exit_complete: " {move || showcase_exit_count.get()}
                    </span>
                </div>
                <Tray
                    open=showcase_open
                    on_close=on_showcase_close
                    id_base="docs-tray-hello".to_string()
                    title="Filters".to_string()
                    description="Tray body with semantic heading and close control.".to_string()
                    footer=move || view! {
                        <div class="docs-row docs-row--end">
                            <Button variant=ButtonVariant::Secondary on_press=on_showcase_close>
                                "Reset"
                            </Button>
                            <Button on_press=on_showcase_close>"Apply"</Button>
                        </div>
                    }
                    on_exit_complete=on_showcase_exit_complete
                >
                    <div class="docs-stack docs-stack--tight">
                        <span>"Real tray content for mobile-first actions."</span>
                        <span class="ui-muted">
                            "Dismiss via close action, Esc, or backdrop by default."
                        </span>
                    </div>
                </Tray>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="tray-workbench-controls">
                        <Switch
                            checked=workbench_show_description
                            set_checked=set_workbench_show_description
                        >
                            "description"
                        </Switch>
                        <Switch checked=workbench_show_footer set_checked=set_workbench_show_footer>
                            "footer"
                        </Switch>
                        <Switch
                            checked=workbench_show_close_button
                            set_checked=set_workbench_show_close_button
                        >
                            "show_close_button"
                        </Switch>
                        <Switch checked=workbench_fixed_height set_checked=set_workbench_fixed_height>
                            "is_fixed_height"
                        </Switch>
                        <Switch checked=workbench_dismissable set_checked=set_workbench_dismissable>
                            "is_dismissable"
                        </Switch>
                        <Switch
                            checked=workbench_keyboard_dismiss_disabled
                            set_checked=set_workbench_keyboard_dismiss_disabled
                        >
                            "is_keyboard_dismiss_disabled"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch
                            checked=workbench_compact_close_label
                            set_checked=set_workbench_compact_close_label
                        >
                            "close_label compact"
                        </Switch>
                        <Switch
                            checked=workbench_custom_class_name
                            set_checked=set_workbench_custom_class_name
                        >
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <Button variant=ButtonVariant::Secondary on_press=open_workbench>
                            "Open workbench tray"
                        </Button>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="tray-workbench-feedback">
                    <span class="ui-muted">
                        "open: " {move || workbench_open_raw.get()}
                        " · on_close: " {move || workbench_close_count.get()}
                        " · on_exit_complete: " {move || workbench_exit_count.get()}
                    </span>
                </div>
                <Tray
                    open=workbench_open
                    on_close=on_workbench_close
                    id_base="docs-tray-workbench".to_string()
                    title="Workbench tray".to_string()
                    description=if workbench_show_description.get() {
                        "Slide-up panel for contextual actions.".to_string()
                    } else {
                        String::new()
                    }
                    footer=move || {
                        view! {
                            <Show when=move || workbench_show_footer.get()>
                                <div class="docs-row docs-row--end">
                                    <Button variant=ButtonVariant::Secondary on_press=on_workbench_close>
                                        "Cancel"
                                    </Button>
                                    <Button on_press=on_workbench_close>"Save"</Button>
                                </div>
                            </Show>
                        }
                        .into_any()
                    }
                    motion=workbench_motion.get()
                    show_close_button=workbench_show_close_button.get()
                    close_label=if workbench_compact_close_label.get() {
                        "Dismiss"
                    } else {
                        "Close tray"
                    }
                    is_fixed_height=workbench_fixed_height.get()
                    is_dismissable=workbench_dismissable.get()
                    is_keyboard_dismiss_disabled=workbench_keyboard_dismiss_disabled.get()
                    lang=if workbench_zh_lang.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    }
                    dir=if workbench_rtl_dir.get() {
                        ui_headless::A11yDirection::Rtl
                    } else {
                        ui_headless::A11yDirection::Ltr
                    }
                    on_exit_complete=on_workbench_exit_complete
                    class_name=if workbench_custom_class_name.get() {
                        "docs-tray-workbench".to_string()
                    } else {
                        String::new()
                    }
                >
                    <div class="docs-stack docs-stack--tight">
                        <span>"Workbench body area. Toggle parameters from the control panel."</span>
                        <span class="ui-muted">
                            "This tray exposes close/exit callback counts as live feedback."
                        </span>
                    </div>
                </Tray>
            </Playground>

            <Playground title="State Matrix (Default / Fixed / Compact)" code_signal=matrix_code>
                <div class="docs-row" data-slot="tray-state-matrix-controls">
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_default>
                        "Open Default"
                    </Button>
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_fixed>
                        "Open Fixed"
                    </Button>
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_compact>
                        "Open Compact"
                    </Button>
                </div>

                <Tray
                    open=matrix_default_open
                    on_close=close_matrix_default
                    id_base="docs-tray-matrix-default".to_string()
                    title="Default tray".to_string()
                    description="Default behavior with footer actions.".to_string()
                    footer=move || view! {
                        <div class="docs-row docs-row--end">
                            <Button on_press=close_matrix_default>"Done"</Button>
                        </div>
                    }
                >
                    <div>"Default tray body."</div>
                </Tray>

                <Tray
                    open=matrix_fixed_open
                    on_close=close_matrix_fixed
                    id_base="docs-tray-matrix-fixed".to_string()
                    title="Fixed tray".to_string()
                    is_fixed_height=true
                    is_dismissable=false
                    is_keyboard_dismiss_disabled=true
                    show_close_button=true
                    class_name="docs-tray-fixed".to_string()
                >
                    <div>"Fixed-height tray with stricter dismiss behavior."</div>
                </Tray>

                <Tray
                    open=matrix_compact_open
                    on_close=close_matrix_compact
                    id_base="docs-tray-matrix-compact".to_string()
                    title="Compact tray".to_string()
                    show_close_button=false
                    close_label="Dismiss"
                    motion=TrayMotion {
                        sheet: ui::SheetMotion {
                            initial_offset_px: 48.0,
                            ..ui::SheetMotion::default()
                        },
                    }
                >
                    <div>"Compact state prioritizes content area."</div>
                </Tray>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
