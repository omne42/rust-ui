use super::*;

pub(crate) fn toast() -> AnyView {
    let hello_code = Signal::derive(move || {
        r#"<Toast
  title="Saved".to_string()
  default_open=true
/>"#
        .to_string()
    });

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);
    let (workbench_close_count, set_workbench_close_count) = signal(0_u32);
    let (workbench_exit_count, set_workbench_exit_count) = signal(0_u32);
    let (workbench_danger_variant, set_workbench_danger_variant) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class_name, set_workbench_custom_class_name) = signal(false);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_custom_close_label, set_workbench_custom_close_label) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let open_workbench_toast: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench_toast: OnPress = Callback::new(move |_| set_workbench_open_raw.set(false));
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });
    let on_workbench_close: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(false);
        set_workbench_close_count.update(|count| *count += 1);
    });
    let on_workbench_exit_complete =
        Callback::new(move |_| set_workbench_exit_count.update(|count| *count += 1));

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ToastMotion {
                initial_y_px: 18.0,
                initial_scale: 0.96,
                ..ToastMotion::default()
            }
        } else {
            ToastMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let variant = if workbench_danger_variant.get() {
            "ToastVariant::Danger"
        } else {
            "ToastVariant::Default"
        };
        let description = if workbench_show_description.get() {
            "Some(\"Try again in a few seconds.\".to_string())"
        } else {
            "None"
        };
        let motion = if workbench_custom_motion.get() {
            "ToastMotion { initial_y_px: 18.0, initial_scale: 0.96, ..ToastMotion::default() }"
        } else {
            "ToastMotion::default()"
        };
        let class_name = if workbench_custom_class_name.get() {
            "docs-toast-workbench"
        } else {
            ""
        };
        let id = if workbench_custom_id.get() {
            "docs-toast-workbench"
        } else {
            ""
        };
        let close_aria_label = if workbench_custom_close_label.get() {
            "Dismiss notification"
        } else {
            ""
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

        vec![
            "<Toast".to_string(),
            "  title=\"Operation status\".to_string()".to_string(),
            "  is_open=Signal::derive(move || open_raw.get())".to_string(),
            "  open=Signal::derive(move || open_raw.get())".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=Callback::new(move |next| set_open_raw.set(next))".to_string(),
            format!("  id={id:?}.to_string()"),
            format!("  description={description}"),
            format!("  variant={variant}"),
            format!("  motion={motion}"),
            format!("  class_name={class_name:?}.to_string()"),
            "  on_close=Callback::new(move |_| set_open_raw.set(false))".to_string(),
            "  on_exit_complete=Callback::new(move |_| {})".to_string(),
            format!("  close_aria_label={close_aria_label:?}.to_string()"),
            format!("  lang={lang:?}.to_string()"),
            format!("  dir={dir}"),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let id = if workbench_custom_id.get() {
            Some("docs-toast-workbench")
        } else {
            None
        };
        let description = if workbench_show_description.get() {
            Some("Try again in a few seconds.")
        } else {
            None
        };
        let variant = if workbench_danger_variant.get() {
            "ToastVariant::Danger"
        } else {
            "ToastVariant::Default"
        };
        let motion = if workbench_custom_motion.get() {
            "ToastMotion::custom"
        } else {
            "ToastMotion::default"
        };
        let class_name = if workbench_custom_class_name.get() {
            Some("docs-toast-workbench")
        } else {
            None
        };
        let close_aria_label = if workbench_custom_close_label.get() {
            Some("Dismiss notification")
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
            "ToastActualConfig {{\n  title: \"Operation status\",\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: \"count={}\",\n  id: {id:?},\n  description: {description:?},\n  variant: {variant},\n  motion: {motion},\n  class_name: {class_name:?},\n  on_close: \"count={}\",\n  on_exit_complete: \"count={}\",\n  close_aria_label: {close_aria_label:?},\n  lang: {lang:?},\n  dir: {dir},\n}}",
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            workbench_open_change_count.get(),
            workbench_close_count.get(),
            workbench_exit_count.get(),
        )
    });

    let (matrix_default_open_raw, set_matrix_default_open_raw) = signal(false);
    let matrix_default_open: Signal<bool> = Signal::derive(move || matrix_default_open_raw.get());
    let open_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(true));
    let close_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(false));

    let (matrix_danger_open_raw, set_matrix_danger_open_raw) = signal(false);
    let matrix_danger_open: Signal<bool> = Signal::derive(move || matrix_danger_open_raw.get());
    let open_matrix_danger: OnPress = Callback::new(move |_| set_matrix_danger_open_raw.set(true));
    let close_matrix_danger: OnPress =
        Callback::new(move |_| set_matrix_danger_open_raw.set(false));

    let matrix_code = Signal::derive(move || {
        r#"<Toast title="Saved".to_string() open=default_open on_close=dismiss_default />
<Toast title="Publish failed".to_string() variant=ToastVariant::Danger open=danger_open on_close=dismiss_danger />
<Toast title="Muted".to_string() class_name="docs-toast-custom".to_string() default_open=true />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Toast"
            slug="toast"
            group="Overlays"
            description="Toast playground with full API workbench and matrix comparison."
        >
            <Playground title="Hello World (Default Toast)" code_signal=hello_code>
                <Toast title="Saved".to_string() default_open=true />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="toast-workbench-controls">
                        <Switch checked=workbench_danger_variant set_checked=set_workbench_danger_variant>
                            "Danger variant"
                        </Switch>
                        <Switch checked=workbench_show_description set_checked=set_workbench_show_description>
                            "description"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch
                            checked=workbench_custom_class_name
                            set_checked=set_workbench_custom_class_name
                        >
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                            "id"
                        </Switch>
                        <Switch
                            checked=workbench_custom_close_label
                            set_checked=set_workbench_custom_close_label
                        >
                            "close_aria_label"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <div class="docs-row docs-row--tight">
                            <Button variant=ButtonVariant::Secondary on_press=open_workbench_toast>
                                "Open"
                            </Button>
                            <Button variant=ButtonVariant::Secondary on_press=close_workbench_toast>
                                "Close"
                            </Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="toast-workbench-feedback">
                    <span class="ui-muted">
                        "open: " {move || workbench_open_raw.get()}
                        " · on_open_change: " {move || workbench_open_change_count.get()}
                        " · on_close: " {move || workbench_close_count.get()}
                        " · on_exit_complete: " {move || workbench_exit_count.get()}
                    </span>
                </div>
                <Toast
                    title="Operation status".to_string()
                    is_open=workbench_open
                    open=workbench_open
                    default_open=false
                    on_open_change=on_workbench_open_change
                    id=if workbench_custom_id.get() {
                        "docs-toast-workbench".to_string()
                    } else {
                        String::new()
                    }
                    description=if workbench_show_description.get() {
                        "Try again in a few seconds.".to_string()
                    } else {
                        String::new()
                    }
                    variant=if workbench_danger_variant.get() {
                        ToastVariant::Danger
                    } else {
                        ToastVariant::Default
                    }
                    motion=workbench_motion.get()
                    class_name=if workbench_custom_class_name.get() {
                        "docs-toast-workbench".to_string()
                    } else {
                        String::new()
                    }
                    on_close=on_workbench_close
                    on_exit_complete=on_workbench_exit_complete
                    close_aria_label=if workbench_custom_close_label.get() {
                        "Dismiss notification".to_string()
                    } else {
                        String::new()
                    }
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
                />
            </Playground>

            <Playground title="State Matrix (Default / Danger / Custom)" code_signal=matrix_code>
                <div class="docs-row" data-slot="toast-state-matrix-controls">
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_default>
                        "Open default"
                    </Button>
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_danger>
                        "Open danger"
                    </Button>
                </div>
                <Toast
                    title="Saved".to_string()
                    open=matrix_default_open
                    on_close=close_matrix_default
                />
                <Toast
                    title="Publish failed".to_string()
                    variant=ToastVariant::Danger
                    open=matrix_danger_open
                    on_close=close_matrix_danger
                />
                <Toast
                    title="Muted".to_string()
                    default_open=true
                    class_name="docs-toast-custom".to_string()
                    motion=ToastMotion {
                        initial_y_px: 12.0,
                        initial_scale: 0.97,
                        ..ToastMotion::default()
                    }
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
