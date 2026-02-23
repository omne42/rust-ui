use super::*;

pub(crate) fn toast_viewport() -> AnyView {
    let showcase_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 3 }));
    let workbench_default_store =
        StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 3 }));
    let workbench_custom_store =
        StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 5 }));
    let matrix_portal_store =
        StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 2 }));
    let matrix_inline_store =
        StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 2 }));
    let matrix_motion_store =
        StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));

    let push_showcase: OnPress = Callback::new(move |_| {
        showcase_store.get_value().push_simple("Saved");
    });
    let push_showcase_danger: OnPress = Callback::new(move |_| {
        showcase_store.get_value().push.run(ToastOptions {
            title: "Failed".to_string(),
            description: Some("Something went wrong.".to_string()),
            variant: ToastVariant::Danger,
            duration_ms: Some(6000),
        });
    });

    let (workbench_portal, set_workbench_portal) = signal(true);
    let (workbench_max_toasts, set_workbench_max_toasts) = signal(3_u16);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_use_custom_store, set_workbench_use_custom_store) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_push_count, set_workbench_push_count) = signal(0_u32);
    let (workbench_clear_count, set_workbench_clear_count) = signal(0_u32);

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ToastMotion {
                initial_y_px: 16.0,
                initial_scale: 0.95,
                ..ToastMotion::default()
            }
        } else {
            ToastMotion::default()
        }
    });

    let push_workbench: OnPress = Callback::new(move |_| {
        let store = if workbench_use_custom_store.get() {
            workbench_custom_store.get_value()
        } else {
            workbench_default_store.get_value()
        };
        store.push.run(ToastOptions {
            title: "Deployment finished".to_string(),
            description: Some("Viewport config is active.".to_string()),
            variant: ToastVariant::Default,
            duration_ms: Some(4800),
        });
        set_workbench_push_count.update(|count| *count += 1);
    });

    let clear_workbench: OnPress = Callback::new(move |_| {
        let store = if workbench_use_custom_store.get() {
            workbench_custom_store.get_value()
        } else {
            workbench_default_store.get_value()
        };
        store.clear.run(());
        set_workbench_clear_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ToastViewport\n  motion={}\n  is_portal={}\n  max_toasts={}\n  class_name={}\n  lang={}\n  dir={}\n  store={}\n/>",
            if workbench_custom_motion.get() {
                "ToastMotion { initial_y_px: 16.0, initial_scale: 0.95, ..ToastMotion::default() }"
            } else {
                "ToastMotion::default()"
            },
            workbench_portal.get(),
            workbench_max_toasts.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-toast-viewport-workbench\".to_string())"
            } else {
                "None"
            },
            if workbench_zh_lang.get() {
                "Some(\"zh-CN\".to_string())"
            } else {
                "Some(\"en-US\".to_string())"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_use_custom_store.get() {
                "Some(custom_store)"
            } else {
                "Some(default_store)"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ToastViewportActualConfig {{\n  motion: {},\n  is_portal: {},\n  max_toasts: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  store: {},\n  push_count: {},\n  clear_count: {},\n}}",
            if workbench_custom_motion.get() {
                "ToastMotion::custom"
            } else {
                "ToastMotion::default"
            },
            workbench_portal.get(),
            workbench_max_toasts.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-toast-viewport-workbench\")"
            } else {
                "None"
            },
            if workbench_zh_lang.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_use_custom_store.get() {
                "custom_store"
            } else {
                "default_store"
            },
            workbench_push_count.get(),
            workbench_clear_count.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ToastViewport is_portal=true max_toasts=2 store=portal_store />
<ToastViewport is_portal=false max_toasts=2 class_name=Some("docs-toast-viewport-inline".to_string()) store=inline_store />
<ToastViewport motion=ToastMotion { initial_y_px: 20.0, initial_scale: 0.94, ..ToastMotion::default() } max_toasts=4 store=motion_store />"#
            .to_string()
    });

    let push_matrix_portal: OnPress = Callback::new(move |_| {
        matrix_portal_store.get_value().push_simple("Portal queue");
    });
    let push_matrix_inline: OnPress = Callback::new(move |_| {
        matrix_inline_store.get_value().push_simple("Inline queue");
    });
    let push_matrix_motion: OnPress = Callback::new(move |_| {
        matrix_motion_store.get_value().push.run(ToastOptions {
            title: "Motion preset".to_string(),
            description: Some("Custom motion + bigger queue.".to_string()),
            variant: ToastVariant::Accent,
            duration_ms: Some(5200),
        });
    });

    view! {
        <ComponentPage
            title="ToastViewport"
            slug="toast-viewport"
            group="Overlays"
            description="Toast viewport (portal) with per-toast spring motion and auto-dismiss."
        >
            <Playground
                title="Hello World (Default Viewport)"
                code_signal=Signal::derive(move || {
                    r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<ToastViewport store=store />"#
                        .to_string()
                })
            >
                <div class="docs-row">
                    <Button variant=ButtonVariant::Secondary on_press=push_showcase>"Push toast"</Button>
                    <Button variant=ButtonVariant::Destructive on_press=push_showcase_danger>
                        "Push danger"
                    </Button>
                </div>
                <ToastViewport store=showcase_store.get_value() />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="toast-viewport-workbench-controls">
                        <Switch checked=workbench_portal set_checked=set_workbench_portal>
                            "is_portal"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_use_custom_store set_checked=set_workbench_use_custom_store>
                            "custom store"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <label class="docs-search__label">
                            "max_toasts (" {move || workbench_max_toasts.get()} ")"
                            <input
                                type="range"
                                min="1"
                                max="6"
                                step="1"
                                prop:value=move || workbench_max_toasts.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev)
                                        .parse::<u16>()
                                        .unwrap_or(3)
                                        .clamp(1, 6);
                                    set_workbench_max_toasts.set(next);
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=push_workbench>"Push toast"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=clear_workbench>
                            "Clear"
                        </Button>
                    </div>
                    <span class="ui-muted">
                        "push: " {move || workbench_push_count.get()}
                        " · clear: " {move || workbench_clear_count.get()}
                        " · store: "
                        {move || if workbench_use_custom_store.get() { "custom" } else { "default" }}
                    </span>
                    <ToastViewport
                        motion=workbench_motion.get()
                        is_portal=workbench_portal.get()
                        max_toasts=usize::from(workbench_max_toasts.get())
                        class_name=if workbench_custom_class.get() {
                            "docs-toast-viewport-workbench".to_string()
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
                        store=if workbench_use_custom_store.get() {
                            workbench_custom_store.get_value()
                        } else {
                            workbench_default_store.get_value()
                        }
                    />
                </div>
            </Playground>

            <Playground title="State Matrix (Portal / Inline / Custom Motion)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight" data-slot="toast-viewport-state-matrix">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=push_matrix_portal>
                            "Portal sample"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=push_matrix_inline>
                            "Inline sample"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=push_matrix_motion>
                            "Motion sample"
                        </Button>
                    </div>
                    <div class="docs-row">
                        <ToastViewport
                            is_portal=true
                            max_toasts=2
                            store=matrix_portal_store.get_value()
                        />
                        <ToastViewport
                            is_portal=false
                            max_toasts=2
                            class_name="docs-toast-viewport-inline".to_string()
                            store=matrix_inline_store.get_value()
                        />
                        <ToastViewport
                            motion=ToastMotion {
                                initial_y_px: 20.0,
                                initial_scale: 0.94,
                                ..ToastMotion::default()
                            }
                            is_portal=true
                            max_toasts=4
                            store=matrix_motion_store.get_value()
                        />
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
