use super::*;

pub(crate) fn meter() -> AnyView {
    let (value, set_value) = signal(42_i64);
    let meter_value = Signal::derive(move || Some(value.get() as f64));
    let (workbench_value, set_workbench_value) = signal(64_i64);
    let (workbench_variant_danger, set_workbench_variant_danger) = signal(false);
    let (workbench_size_large, set_workbench_size_large) = signal(false);
    let (workbench_indeterminate, set_workbench_indeterminate) = signal(false);
    let (workbench_show_value_label, set_workbench_show_value_label) = signal(true);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_preserve_state, set_workbench_preserve_state) = signal(true);

    Effect::new(move |_| {
        if !workbench_preserve_state.get() {
            set_workbench_value.set(64);
            set_workbench_variant_danger.set(false);
            set_workbench_size_large.set(false);
            set_workbench_indeterminate.set(false);
            set_workbench_show_value_label.set(true);
            set_workbench_custom_label.set(false);
            set_workbench_custom_class.set(false);
            set_workbench_custom_motion.set(false);
            set_workbench_custom_aria.set(false);
            set_workbench_rtl_dir.set(false);
        }
    });

    let on_meter_workbench_reset = Callback::new(move |_| {
        set_workbench_value.set(64);
        set_workbench_variant_danger.set(false);
        set_workbench_size_large.set(false);
        set_workbench_indeterminate.set(false);
        set_workbench_show_value_label.set(true);
        set_workbench_custom_label.set(false);
        set_workbench_custom_class.set(false);
        set_workbench_custom_motion.set(false);
        set_workbench_custom_aria.set(false);
        set_workbench_rtl_dir.set(false);
    });

    let hello_world_code = Signal::derive(move || {
        r#"<Meter
  id="docs-meter-hello".to_string()
  label="Completion".to_string()
  value=Signal::derive(|| Some(42.0))
/>"#
        .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"let meter_value = Signal::derive(move || Some(value.get() as f64));
<Meter id="docs-meter-default".to_string() label="Completion".to_string() value=meter_value min=0.0 max=100.0 />
<Meter id="docs-meter-danger".to_string() label="Risk".to_string() value=meter_value variant=MeterVariant::Danger size=MeterSize::Lg />
<Meter id="docs-meter-compact".to_string() label="Compact".to_string() value=meter_value size=MeterSize::Sm show_value_label=false />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Meter
  id="docs-meter-custom".to_string()
  label="Sync progress".to_string()
  aria_label="Background sync".to_string()
  value=Signal::derive(|| Some(64.0))
  min=0.0
  max=100.0
  value_label="64 complete".to_string()
  motion=ui::MeterMotion::fast()
  class_name="docs-meter-custom".to_string()
/>
<Meter
  id="docs-meter-fallback".to_string()
  label="   ".to_string()
  aria_label="   ".to_string()
  value=Signal::derive(|| Some(18.0))
  class_name="docs-meter-custom".to_string()
/>
<Meter
  id="docs-meter-indeterminate".to_string()
  label="Pending".to_string()
  value=Signal::derive(|| None)
  class_name="docs-meter-custom".to_string()
/>"#
        .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"let upstream_progress = 72_i64;

<Meter
  id="docs-meter-default-contrast".to_string()
  label="Default path".to_string()
  value=Signal::derive(|| Some(42.0))
/>
<Meter
  id="docs-meter-upstream-mapped".to_string()
  label="Upstream mapped".to_string()
  value=Signal::derive(move || Some(upstream_progress as f64))
/>
// Meter has no internal controlled/uncontrolled runtime axis.
// App state maps directly to props; there is no value/on_change/default triplet."#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Meter
  id="docs-meter-snapshot".to_string()
  label="Snapshot".to_string()
  value=Signal::derive(|| Some(88.0))
/>
// Streaming Optional; fallback=snapshot.
// Meter renders complete validated snapshots and keeps semantic continuity."#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui::{Meter, MeterSize, MeterVariant};

<Meter
  id="docs-meter-source-first".to_string()
  label="Completion".to_string()
  value=Signal::derive(|| Some(42.0))
  variant=MeterVariant::Default
  size=MeterSize::Default
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant = if workbench_variant_danger.get() {
            MeterVariant::Danger
        } else {
            MeterVariant::Default
        };
        let size = if workbench_size_large.get() {
            MeterSize::Lg
        } else {
            MeterSize::Default
        };
        let mut lines = vec![
            "<Meter".to_string(),
            "  id=\"docs-meter-workbench\".into()".to_string(),
            "  label=\"Workbench meter\".into()".to_string(),
            if workbench_indeterminate.get() {
                "  value=Signal::derive(|| None)".to_string()
            } else {
                format!(
                    "  value=Signal::derive(|| Some({}.0))",
                    workbench_value.get()
                )
            },
        ];

        if variant != MeterVariant::Default {
            lines.push(format!("  variant=MeterVariant::{variant:?}"));
        }
        if size != MeterSize::Default {
            lines.push(format!("  size=MeterSize::{size:?}"));
        }
        if !workbench_show_value_label.get() {
            lines.push("  show_value_label=false".to_string());
        }
        if workbench_custom_label.get() {
            lines.push(format!(
                "  value_label=\"{} complete\".into()",
                workbench_value.get()
            ));
        }
        if workbench_custom_motion.get() {
            lines.push("  motion=ui::MeterMotion::fast()".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-meter-custom\".into()".to_string());
        }
        if workbench_custom_aria.get() {
            lines.push("  aria_label=\"Background sync meter\".into()".to_string());
        }
        lines.push("  min=0.0".to_string());
        lines.push("  max=100.0".to_string());
        lines.push(format!(
            "  is_value_label_visible={}",
            bool_word(workbench_show_value_label.get())
        ));
        lines.push(if workbench_rtl_dir.get() {
            "  lang=\"ar\".into()".to_string()
        } else {
            "  lang=\"en-US\".into()".to_string()
        });
        lines.push(if workbench_rtl_dir.get() {
            "  dir=A11yDirection::Rtl".to_string()
        } else {
            "  dir=A11yDirection::Ltr".to_string()
        });
        lines.push("/>".to_string());
        lines.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/meter/src/styles.rs */\n{}",
            ui::meter::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let variant = if workbench_variant_danger.get() {
            MeterVariant::Danger
        } else {
            MeterVariant::Default
        };
        let size = if workbench_size_large.get() {
            MeterSize::Lg
        } else {
            MeterSize::Default
        };
        let is_indeterminate = workbench_indeterminate.get();
        let has_custom_label = workbench_custom_label.get();
        let has_custom_class = workbench_custom_class.get();
        let has_custom_motion = workbench_custom_motion.get();
        let preserve_state = workbench_preserve_state.get();
        let show_value_label = workbench_show_value_label.get();
        let value = workbench_value.get();
        let data_state = if is_indeterminate {
            "indeterminate"
        } else {
            "determinate"
        };

        let mut classes = vec![
            "ui-meter".to_string(),
            variant.class_name().into(),
            size.class_name().into(),
            if has_custom_label {
                "ui-meter--value-label-custom".to_string()
            } else {
                "ui-meter--value-label-auto".to_string()
            },
            if has_custom_motion {
                "ui-meter--motion-custom".to_string()
            } else {
                "ui-meter--motion-default".to_string()
            },
            if is_indeterminate {
                "ui-meter--state-indeterminate".to_string()
            } else {
                "ui-meter--state-determinate".to_string()
            },
        ];
        if has_custom_class {
            classes.push("ui-meter--custom-class".to_string());
            classes.push("docs-meter-custom".to_string());
        }

        format!(
            "MeterActualConfig {{\n  id: \"docs-meter-workbench\",\n  value: {},\n  min: 0.0,\n  max: 100.0,\n  variant: {variant:?},\n  size: {size:?},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {},\n  is_indeterminate: {is_indeterminate},\n  is_value_label_visible: {show_value_label},\n  show_value_label: {show_value_label},\n  has_custom_value_label: {has_custom_label},\n  has_custom_motion: {has_custom_motion},\n  has_custom_class_name: {has_custom_class},\n  preserve_state: {preserve_state},\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            if is_indeterminate {
                "None".to_string()
            } else {
                format!("Some({value}.0)")
            },
            if workbench_custom_aria.get() {
                Some("Background sync meter")
            } else {
                None
            },
            if workbench_rtl_dir.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Meter"
            slug="meter"
            group="Display"
            description="Spring-driven meter with centralized variant/size/phase source attrs."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports="use leptos::prelude::*;\nuse ui::Meter;".to_string()
                test_source_path="components/meter/src/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Meter
                        id="docs-meter-hello".to_string()
                        label="Completion".to_string()
                        value=Signal::derive(|| Some(42.0))
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="调样式优先走 CSS Test 即时反馈；`preserve_state` 可选保留当前配置上下文。"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{Meter, MeterSize, MeterVariant, Switch};".to_string()
                test_css_source=test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/meter/src/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="meter-workbench-controls">
                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_variant_danger.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_variant_danger.get() {
                                    "Variant: Danger"
                                } else {
                                    "Variant: Default"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_size_large.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_size_large.get() {
                                    "Size: Lg"
                                } else {
                                    "Size: Default"
                                }}
                            </ui::Button>
                        </div>

                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_value.update(|v| *v = (*v - 10).max(0))
                                })
                            >
                                "-10"
                            </ui::Button>
                            <div data-action="meter-workbench-increment">
                                <ui::Button
                                    variant=ui::ButtonVariant::Secondary
                                    on_press=Callback::new(move |_| {
                                        set_workbench_value.update(|v| *v = (*v + 10).min(100))
                                    })
                                >
                                    "+10"
                                </ui::Button>
                            </div>
                            <span class="ui-muted">"value: " {move || workbench_value.get()}</span>
                        </div>

                        <div class="docs-row">
                            <div data-action="meter-workbench-toggle-indeterminate">
                                <ui::Button
                                    variant=ui::ButtonVariant::Secondary
                                    on_press=Callback::new(move |_| {
                                        set_workbench_indeterminate.update(|v| *v = !*v)
                                    })
                                >
                                    {move || if workbench_indeterminate.get() {
                                        "Indeterminate: on"
                                    } else {
                                        "Indeterminate: off"
                                    }}
                                </ui::Button>
                            </div>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_show_value_label.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_show_value_label.get() {
                                    "Value label: on"
                                } else {
                                    "Value label: off"
                                }}
                            </ui::Button>
                        </div>

                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_label.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_label.get() {
                                    "Custom value label: on"
                                } else {
                                    "Custom value label: off"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_motion.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_motion.get() {
                                    "Custom motion: on"
                                } else {
                                    "Custom motion: off"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_class.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_class.get() {
                                    "Custom class: on"
                                } else {
                                    "Custom class: off"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_aria.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_aria.get() {
                                    "Custom aria_label: on"
                                } else {
                                    "Custom aria_label: off"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_rtl_dir.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_rtl_dir.get() {
                                    "Direction: RTL(ar)"
                                } else {
                                    "Direction: LTR(en)"
                                }}
                            </ui::Button>
                        </div>

                        <div class="docs-row">
                            <Switch checked=workbench_preserve_state set_checked=set_workbench_preserve_state>
                                "preserve state"
                            </Switch>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=on_meter_workbench_reset
                            >
                                "Reset context"
                            </ui::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="meter-workbench-preview">
                    <p class="ui-muted" data-slot="meter-spec-linkage">
                        "Spec Input -> Preview Output: controls drive `MeterActualConfig` and live preview in sync."
                    </p>
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight" style="min-width: 18rem;">
                            <span class="ui-muted">"当前配置"</span>
                            <Meter
                                id="docs-meter-workbench".to_string()
                                label="Workbench meter".to_string()
                                value=Signal::derive(move || {
                                    if workbench_indeterminate.get() {
                                        None
                                    } else {
                                        Some(workbench_value.get() as f64)
                                    }
                                })
                                variant=if workbench_variant_danger.get() {
                                    MeterVariant::Danger
                                } else {
                                    MeterVariant::Default
                                }
                                size=if workbench_size_large.get() {
                                    MeterSize::Lg
                                } else {
                                    MeterSize::Default
                                }
                                aria_label=if workbench_custom_aria.get() {
                                    "Background sync meter".to_string()
                                } else {
                                    String::new()
                                }
                                min=0.0
                                max=100.0
                                lang=if workbench_rtl_dir.get() {
                                    "ar".to_string()
                                } else {
                                    "en-US".to_string()
                                }
                                dir=if workbench_rtl_dir.get() {
                                    A11yDirection::Rtl
                                } else {
                                    A11yDirection::Ltr
                                }
                                is_value_label_visible=workbench_show_value_label.get()
                                show_value_label=workbench_show_value_label.get()
                                value_label=if workbench_custom_label.get() {
                                    format!("{} complete", workbench_value.get())
                                } else {
                                    String::new()
                                }
                                motion=if workbench_custom_motion.get() {
                                    ui::MeterMotion::fast()
                                } else {
                                    ui::MeterMotion::default()
                                }
                                class_name=if workbench_custom_class.get() {
                                    "docs-meter-custom".to_string()
                                } else {
                                    String::new()
                                }
                            />
                        </div>

                        <div class="docs-stack docs-stack--tight" style="min-width: 18rem;">
                            <span class="ui-muted">"对比：Danger + Lg（固定）"</span>
                            <Meter
                                id="docs-meter-workbench-contrast".to_string()
                                label="Contrast".to_string()
                                value=Signal::derive(move || Some(workbench_value.get() as f64))
                                variant=MeterVariant::Danger
                                size=MeterSize::Lg
                            />
                        </div>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"对比：Indeterminate（固定）"</span>
                        <Meter
                            id="docs-meter-workbench-indeterminate".to_string()
                            label="Pending".to_string()
                            value=Signal::derive(|| None)
                            class_name="docs-meter-custom".to_string()
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="State Matrix (Variant + Range Comparison)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <Meter
                        id="docs-meter-state-default".to_string()
                        label="Default".to_string()
                        value=Signal::derive(move || Some(value.get() as f64))
                        min=0.0
                        max=100.0
                        is_value_label_visible=true
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <Meter
                        id="docs-meter-state-danger".to_string()
                        label="Danger".to_string()
                        value=Signal::derive(move || Some(value.get() as f64))
                        variant=MeterVariant::Danger
                        size=MeterSize::Lg
                        min=0.0
                        max=120.0
                        aria_label="Risk progress".to_string()
                        is_value_label_visible=false
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                    <Meter
                        id="docs-meter-state-indeterminate".to_string()
                        label="Pending".to_string()
                        value=Signal::derive(|| None)
                        min=0.0
                        max=100.0
                        class_name="docs-meter-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Variant + Size Matrix"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{Meter, MeterSize, MeterVariant};".to_string()
                test_source_path="components/meter/src/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Meter
                        id="docs-meter-default".to_string()
                        label="Completion".to_string()
                        value=meter_value
                        min=0.0
                        max=100.0
                    />
                    <Meter
                        id="docs-meter-danger".to_string()
                        label="Risk".to_string()
                        value=meter_value
                        variant=MeterVariant::Danger
                        size=MeterSize::Lg
                    />
                    <Meter
                        id="docs-meter-compact".to_string()
                        label="Compact".to_string()
                        value=meter_value
                        size=MeterSize::Sm
                        show_value_label=false
                    />
                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 10).min(100)))
                        >
                            "+10"
                        </ui::Button>
                        <span class="ui-muted">"value: " {move || value.get()}</span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Custom Label + Motion + Class"
                code_signal=custom_code
                code_imports="use leptos::prelude::*;\nuse ui::{Meter, MeterMotion};".to_string()
                test_source_path="components/meter/src/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Meter
                        id="docs-meter-custom".to_string()
                        label="Sync progress".to_string()
                        aria_label="Background sync".to_string()
                        value=Signal::derive(|| Some(64.0))
                        min=0.0
                        max=100.0
                        value_label="64 complete".to_string()
                        motion=ui::MeterMotion::fast()
                        class_name="docs-meter-custom".to_string()
                    />
                    <Meter
                        id="docs-meter-fallback".to_string()
                        label="   ".to_string()
                        aria_label="   ".to_string()
                        value=Signal::derive(|| Some(18.0))
                        class_name="docs-meter-custom".to_string()
                    />
                    <Meter
                        id="docs-meter-indeterminate".to_string()
                        label="Pending".to_string()
                        value=Signal::derive(|| None)
                        class_name="docs-meter-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Meter has no internal controlled/uncontrolled axis; compare default usage and app-state-mapped props."
                code_signal=controlled_contrast_code
                code_imports="use leptos::prelude::*;\nuse ui::Meter;".to_string()
                test_source_path="components/meter/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Meter
                        id="docs-meter-controlled-na-default".to_string()
                        label="Default path".to_string()
                        value=Signal::derive(|| Some(42.0))
                    />
                    <Meter
                        id="docs-meter-controlled-na-upstream".to_string()
                        label="Upstream mapped".to_string()
                        value=Signal::derive(move || Some(workbench_value.get() as f64))
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Meter is not a body-reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports="use leptos::prelude::*;\nuse ui::Meter;".to_string()
                test_source_path="components/meter/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <p class="ui-muted" data-slot="meter-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </p>
                    <p class="ui-muted" data-slot="meter-copy-ready-hint">
                        "Copy-ready snippets prepend imports automatically; source: components/meter/src/view.rs."
                    </p>
                    <Meter
                        id="docs-meter-stream-snapshot".to_string()
                        label="Snapshot".to_string()
                        value=Signal::derive(|| Some(88.0))
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="meter-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="meter-state-rows">
                    <li><code>"data-state / data-ui-state-phase"</code>" = determinate | indeterminate"</li>
                    <li><code>"data-variant"</code>" = default | danger"</li>
                    <li><code>"data-size"</code>" = default | sm | lg"</li>
                    <li><code>"data-label-source / data-value-label-source / data-motion-source / data-class-source"</code>" = default | custom（封闭集合）"</li>
                    <li><code>"control mode"</code>" = N/A（Meter 无内部受控/非受控状态轴）"</li>
                    <li><code>"disabled axis"</code>" = N/A（Meter API 无 disabled 输入）"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="meter-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="meter-parameter-rows">
                    <li><code>"min/max: Option&lt;f64&gt;"</code>" default = None/None -> `DEFAULT_MIN=0.0`、`DEFAULT_MAX=100.0`（`logic.rs::normalize_inputs`）"</li>
                    <li><code>"is_value_label_visible/show_value_label: Option&lt;bool&gt;"</code>" default = None/None -> `DEFAULT_SHOW_VALUE_LABEL=true`，且 `is_*` 优先于历史别名 `show_value_label`"</li>
                    <li><code>"value: Signal&lt;Option&lt;f64&gt;&gt;"</code>" default = None -> `data-state=indeterminate`；Some(v) 走 clamp+progress 推导"</li>
                    <li><code>"value_label: Option&lt;String&gt;"</code>" default = None -> 可见时回退到百分比文本（`derive_render_state`）"</li>
                    <li><code>"variant/size"</code>" default = `MeterVariant::Default` / `MeterSize::Default`"</li>
                    <li><code>"aria_label/label/default_aria_label"</code>" 归一优先级：`aria_label` > `label` > i18n fallback（`resolve_aria_label_with_fallback`）"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="meter-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="meter-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-meter"</code>
                        " feature for package-mode consumption."
                    </li>
                    <li>
                        "Style prerequisite: use "
                        <code>"UiRoot"</code>
                        " with components CSS injection (or enable "
                        <code>"inject-css"</code>
                        " path) to avoid unstyled copy-paste output."
                    </li>
                </ul>
                <Snippet
                    text=source_first_code.get()
                    label="Copy meter starter".to_string()
                    copyable=true
                    class_name="docs-meter-source-copy".to_string()
                />
                <ul data-slot="meter-source-paths">
                    <li><code>"components/meter/src/mod.rs"</code></li>
                    <li><code>"components/meter/src/logic.rs"</code></li>
                    <li><code>"components/meter/src/view.rs"</code></li>
                    <li><code>"components/meter/src/styles.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
