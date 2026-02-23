use super::*;

pub(crate) fn toaster() -> AnyView {
    let portal_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 3 }));
    let inline_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 2 }));
    let source_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));
    let workbench_store =
        StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));

    let hello_world_code = Signal::derive(move || r#"<Toaster />"#.to_string());

    let push_saved: OnPress = Callback::new(move |_| {
        portal_store.get_value().push_simple("Synced");
    });
    let push_warning: OnPress = Callback::new(move |_| {
        portal_store.get_value().push.run(ToastOptions {
            title: "Action required".to_string(),
            description: Some("Session expires soon.".to_string()),
            variant: ToastVariant::Accent,
            duration_ms: Some(5000),
        });
    });

    let push_inline: OnPress = Callback::new(move |_| {
        inline_store.get_value().push.run(ToastOptions {
            title: "Draft restored".to_string(),
            description: Some("Recovered from autosave.".to_string()),
            variant: ToastVariant::Default,
            duration_ms: Some(4500),
        });
    });
    let clear_inline: OnPress = Callback::new(move |_| inline_store.get_value().clear.run(()));

    let push_source: OnPress = Callback::new(move |_| {
        source_store.get_value().push.run(ToastOptions {
            title: "Promotion deployed".to_string(),
            description: Some("Store + source markers stay inspectable.".to_string()),
            variant: ToastVariant::Danger,
            duration_ms: Some(5200),
        });
    });
    let clear_source: OnPress = Callback::new(move |_| source_store.get_value().clear.run(()));

    let basic_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<Toaster store=store.clone() />
store.push_simple("Synced");"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Toaster
  store=store.clone()
  portal=false
  position=ToasterPosition::TopCenter
  max_toasts=2
  class_name="docs-toaster-inline".to_string()
/>"#
        .to_string()
    });

    let source_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Toaster
  store=store.clone()
  portal=false
  position=ToasterPosition::TopLeft
  max_toasts=4
  aria_label="Alert stream".to_string()
  class_name="docs-toaster-source".to_string()
  motion=ToastMotion {
    initial_y_px: 20.0,
    initial_scale: 0.95,
    ..ToastMotion::default()
  }
/>"#
        .to_string()
    });

    let custom_motion = ToastMotion {
        initial_y_px: 20.0,
        initial_scale: 0.95,
        ..ToastMotion::default()
    };

    let (workbench_top_left, set_workbench_top_left) = signal(false);
    let (workbench_portal, set_workbench_portal) = signal(true);
    let (workbench_max_toasts, set_workbench_max_toasts) = signal(3_u16);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_push_count, set_workbench_push_count) = signal(0_u32);
    let (workbench_clear_count, set_workbench_clear_count) = signal(0_u32);

    let push_workbench: OnPress = Callback::new(move |_| {
        workbench_store.get_value().push.run(ToastOptions {
            title: "Workbench alert".to_string(),
            description: Some("Toaster workbench is active.".to_string()),
            variant: ToastVariant::Accent,
            duration_ms: Some(5200),
        });
        set_workbench_push_count.update(|count| *count += 1);
    });
    let clear_workbench: OnPress = Callback::new(move |_| {
        workbench_store.get_value().clear.run(());
        set_workbench_clear_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Toaster\n  position={}\n  portal={}\n  max_toasts={}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir={}\n  motion={}\n  store=Some(store)\n/>",
            if workbench_top_left.get() {
                "ToasterPosition::TopLeft"
            } else {
                "ToasterPosition::BottomRight"
            },
            workbench_portal.get(),
            workbench_max_toasts.get(),
            if workbench_custom_aria.get() {
                "Some(\"Alert stream\".to_string())"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-toaster-workbench\".to_string())"
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
            if workbench_custom_motion.get() {
                "ToastMotion { initial_y_px: 20.0, initial_scale: 0.95, ..ToastMotion::default() }"
            } else {
                "ToastMotion::default()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ToasterActualConfig {{\n  position: {},\n  portal: {},\n  max_toasts: {},\n  aria_label: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  motion: {},\n  store: Some(workbench_store),\n  push_count: {},\n  clear_count: {},\n}}",
            if workbench_top_left.get() {
                "ToasterPosition::TopLeft"
            } else {
                "ToasterPosition::BottomRight"
            },
            workbench_portal.get(),
            workbench_max_toasts.get(),
            if workbench_custom_aria.get() {
                "Some(\"Alert stream\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-toaster-workbench\")"
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
            if workbench_custom_motion.get() {
                "ToastMotion::custom"
            } else {
                "ToastMotion::default"
            },
            workbench_push_count.get(),
            workbench_clear_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="Toaster"
            slug="toaster"
            group="Overlays"
            description="baseline-compatible toast host that composes Sonner/ToastViewport with centralized slot/queue/position/store source-state contracts and baseline-level spring motion handoff."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="ui-muted">
                    "Default path only mounts host; no state primitive wiring or custom store binding required."
                </div>
                <Toaster />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="toaster-workbench-controls">
                        <Switch checked=workbench_top_left set_checked=set_workbench_top_left>
                            "position TopLeft"
                        </Switch>
                        <Switch checked=workbench_portal set_checked=set_workbench_portal>
                            "portal"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "motion"
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
                        <Button on_press=push_workbench>"Push workbench toast"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=clear_workbench>
                            "Clear"
                        </Button>
                    </div>
                    <span class="ui-muted">
                        "push: " {move || workbench_push_count.get()}
                        " · clear: " {move || workbench_clear_count.get()}
                    </span>
                    <Toaster
                        position=if workbench_top_left.get() {
                            ToasterPosition::TopLeft
                        } else {
                            ToasterPosition::BottomRight
                        }
                        portal=workbench_portal.get()
                        max_toasts=usize::from(workbench_max_toasts.get())
                        aria_label=if workbench_custom_aria.get() {
                            "Alert stream".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-toaster-workbench".to_string()
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
                        motion=if workbench_custom_motion.get() {
                            ToastMotion {
                                initial_y_px: 20.0,
                                initial_scale: 0.95,
                                ..ToastMotion::default()
                            }
                        } else {
                            ToastMotion::default()
                        }
                        store=workbench_store.get_value()
                    />
                </div>
            </Playground>

            <Playground title="Portal Queue Host" code_signal=basic_code>
                <div class="docs-row" data-slot="toaster-portal-controls">
                    <span data-slot="toaster-portal-push-success">
                        <Button variant=ButtonVariant::Secondary on_press=push_saved>
                            "Push success"
                        </Button>
                    </span>
                    <span data-slot="toaster-portal-push-accent">
                        <Button on_press=push_warning>
                            "Push accent"
                        </Button>
                    </span>
                </div>
                <Toaster store=portal_store.get_value() />
            </Playground>

            <Playground title="Inline Top-Center Host" code_signal=state_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="toaster-inline-controls">
                        <span data-slot="toaster-inline-push">
                            <Button on_press=push_inline>"Push inline toast"</Button>
                        </span>
                        <span data-slot="toaster-inline-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_inline>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <Toaster
                        store=inline_store.get_value()
                        portal=false
                        position=ToasterPosition::TopCenter
                        max_toasts=2
                        class_name="docs-toaster-inline".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Source Markers)"
                description="Inspect `data-state`, `data-queue`, `data-position-source`, `data-portal-source`, `data-max-toasts-source`, `data-store-source`, and `data-motion-source` contracts."
                code_signal=source_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="toaster-source-controls">
                        <span data-slot="toaster-source-push">
                            <Button on_press=push_source>"Push source toast"</Button>
                        </span>
                        <span data-slot="toaster-source-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_source>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools."
                    </div>
                    <Toaster
                        store=source_store.get_value()
                        portal=false
                        position=ToasterPosition::TopLeft
                        max_toasts=4
                        aria_label="Alert stream".to_string()
                        class_name="docs-toaster-source".to_string()
                        motion=custom_motion
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="toaster-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="toaster-api-rows">
                    <li>
                        <code>"position: ToasterPosition"</code>
                        " "
                        {format!(
                            "default = ToasterPosition::{:?} ({})",
                            ToasterPosition::default(),
                            ToasterPosition::default().as_attr()
                        )}
                    </li>
                    <li>
                        <code>"portal: bool"</code>
                        " "
                        {format!("default = {}", ui::toaster::DEFAULT_PORTAL)}
                    </li>
                    <li>
                        <code>"max_toasts: usize"</code>
                        " "
                        {format!(
                            "default = {}",
                            ui::toaster::DEFAULT_MAX_TOASTS
                        )}
                    </li>
                    <li>
                        <code>"aria_label: Option<String>"</code>
                        " "
                        {format!(
                            "default label = {:?}",
                            ui::toaster::DEFAULT_ARIA_LABEL
                        )}
                    </li>
                    <li>
                        <code>"class_name: Option<String>"</code>
                        " default = None"
                    </li>
                    <li>
                        <code>"lang: Option<String>, dir: Option<A11yDirection>"</code>
                        " default = None (inherits app locale context)"
                    </li>
                    <li>
                        <code>"motion: ToastMotion"</code>
                        " default = ToastMotion::default()"
                    </li>
                    <li>
                        <code>"store: Option<ToastStore>"</code>
                        " default path = provided -> context -> local"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="toaster-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="toaster-state-rows">
                    <li>
                        <code>"data-state"</code>
                        " = portal | inline"
                    </li>
                    <li>
                        <code>"data-queue"</code>
                        " = single | bounded | extended"
                    </li>
                    <li>
                        <code>"data-position"</code>
                        " = top-left | top-center | top-right | bottom-left | bottom-center | bottom-right"
                    </li>
                    <li>
                        <code>"data-store-source"</code>
                        " = provided | context | local"
                    </li>
                    <li>
                        <code>"data-position-source / data-portal-source / data-max-toasts-source / data-motion-source"</code>
                        " = default | custom"
                    </li>
                    <li>
                        <code>"control mode"</code>
                        " = N/A (Toaster is host config, no controlled/uncontrolled runtime axis)"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="toaster-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground already supports "
                    <code>"Show code"</code>
                    " with copy action. The copied snippet is import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::*;\n\n<Toaster />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-toaster-source-copy".to_string()
                />
                <ul data-slot="toaster-source-paths">
                    <li><code>"components/toaster/src/mod.rs"</code></li>
                    <li><code>"components/toaster/src/logic.rs"</code></li>
                    <li><code>"components/toaster/src/view.rs"</code></li>
                    <li><code>"components/toaster/src/styles.rs"</code></li>
                    <li><code>"components/toaster/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="toaster-source-prerequisites">
                    <li><code>"component-toaster"</code></li>
                    <li><code>"component-toast"</code></li>
                    <li><code>"component-sonner"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
