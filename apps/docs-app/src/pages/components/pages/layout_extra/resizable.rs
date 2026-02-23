use super::*;

pub(crate) fn resizable() -> AnyView {
    let (workbench_split_raw, set_workbench_split_raw) = signal(58.0_f64);
    let workbench_value: Signal<f64> = Signal::derive(move || workbench_split_raw.get());
    let workbench_split_percent: Signal<f64> = Signal::derive(move || workbench_split_raw.get());
    let (last_value_change, set_last_value_change) = signal("58.0".to_string());
    let (last_split_change, set_last_split_change) = signal("58.0".to_string());
    let on_value_change = Callback::new(move |next: f64| {
        set_last_value_change.set(format!("{next:.1}"));
        set_workbench_split_raw.set(next);
    });
    let on_split_percent_change = Callback::new(move |next: f64| {
        set_last_split_change.set(format!("{next:.1}"));
        set_workbench_split_raw.set(next);
    });

    let (workbench_orientation_key, set_workbench_orientation_key) =
        signal("horizontal".to_string());
    let workbench_orientation = Signal::derive(move || {
        if workbench_orientation_key.get() == "vertical" {
            ResizableOrientation::Vertical
        } else {
            ResizableOrientation::Horizontal
        }
    });
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_with_handle, set_workbench_with_handle) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_min, set_workbench_min) = signal(25.0_f64);
    let (workbench_max, set_workbench_max) = signal(80.0_f64);

    let (split_raw, set_split_raw) = signal(58.0_f64);
    let split: Signal<f64> = Signal::derive(move || split_raw.get());
    let on_split_change = Callback::new(move |next: f64| set_split_raw.set(next));

    let horizontal_code = Signal::derive(move || {
        r#"<Resizable
  orientation=ResizableOrientation::Horizontal
  default_value=40.0
  first=move || view! { <div>"Sidebar"</div> }
  second=move || view! { <div>"Content"</div> }
/>"#
        .to_string()
    });

    let vertical_code = Signal::derive(move || {
        r#"<Resizable
  orientation=ResizableOrientation::Vertical
  value=split
  on_value_change=on_split_change
  min_split_percent=25.0
  max_split_percent=80.0
  is_with_handle=true
  aria_label="Deployment regions split".to_string()
  class_name="docs-resizable-custom".to_string()
  first=move || view! { <div>\"Left\"</div> }
  second=move || view! { <div>\"Right\"</div> }
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Resizable\n  orientation={}\n  value=value\n  split_percent=split_percent\n  default_value=58.0\n  default_split_percent=58.0\n  on_value_change=on_value_change\n  on_split_percent_change=on_split_percent_change\n  min_split_percent={:.1}\n  max_split_percent={:.1}\n  is_disabled={}\n  disabled={}\n  is_with_handle={}\n  with_handle={}\n  aria_label=\"Workspace split\".to_string()\n  class_name={}\n  lang={}\n  dir={}\n  motion={}\n  first=move || view! {{ <div>\"Primary panel\"</div> }}\n  second=move || view! {{ <div>\"Secondary panel\"</div> }}\n/>",
            if workbench_orientation.get() == ResizableOrientation::Vertical {
                "ResizableOrientation::Vertical"
            } else {
                "ResizableOrientation::Horizontal"
            },
            workbench_min.get(),
            workbench_max.get(),
            workbench_disabled.get(),
            workbench_disabled.get(),
            workbench_with_handle.get(),
            workbench_with_handle.get(),
            if workbench_custom_class.get() {
                "\"docs-resizable-workbench\".to_string()"
            } else {
                "String::new()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en\".to_string()"
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
            if workbench_custom_motion.get() {
                "ResizableMotion { enabled: true, panel_duration_ms: 120, handle_duration_ms: 120 }"
            } else {
                "ResizableMotion::default()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ResizableWorkbenchConfig {{\n  orientation: {},\n  value: {:.1},\n  split_percent: {:.1},\n  default_value: Some(58.0),\n  default_split_percent: Some(58.0),\n  on_value_change: Some(\"Callback<f64>\"),\n  on_split_percent_change: Some(\"Callback<f64>\"),\n  min_split_percent: {:.1},\n  max_split_percent: {:.1},\n  is_disabled: Some({}),\n  disabled: {},\n  is_with_handle: Some({}),\n  with_handle: {},\n  aria_label: Some(\"Workspace split\"),\n  class_name: {},\n  lang: {},\n  dir: {},\n  motion: {},\n  first: \"ViewFn(primary)\",\n  second: \"ViewFn(secondary)\",\n}}",
            if workbench_orientation.get() == ResizableOrientation::Vertical {
                "Vertical"
            } else {
                "Horizontal"
            },
            workbench_split_raw.get(),
            workbench_split_raw.get(),
            workbench_min.get(),
            workbench_max.get(),
            workbench_disabled.get(),
            workbench_disabled.get(),
            workbench_with_handle.get(),
            workbench_with_handle.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-resizable-workbench\")"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_motion.get() {
                "ResizableMotion::custom"
            } else {
                "ResizableMotion::default"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Resizable orientation=ResizableOrientation::Horizontal default_value=40.0 first=move || view! { <div>"Sidebar"</div> } second=move || view! { <div>"Content"</div> } />
<Resizable orientation=ResizableOrientation::Vertical default_value=60.0 is_with_handle=true first=move || view! { <div>"Header"</div> } second=move || view! { <div>"Body"</div> } />
<Resizable orientation=ResizableOrientation::Horizontal default_value=35.0 is_disabled=true first=move || view! { <div>"Disabled left"</div> } second=move || view! { <div>"Disabled right"</div> } />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Resizable"
            slug="resizable"
            group="Layout"
            description="baseline-compatible panel splitter with controlled/uncontrolled split state, pointer + keyboard resize semantics, and baseline-style state data contracts."
        >
            <Playground title="Horizontal + Handle Grip" code_signal=horizontal_code>
                <Resizable
                    orientation=ResizableOrientation::Horizontal
                    default_value=36.0
                    is_with_handle=true
                    first=move || {
                        view! {
                            <View
                                background=ViewBackground::Subtle
                                border=ViewBorder::Subtle
                                padding=ViewPadding::Md
                                radius=ViewRadius::None
                            >
                                <strong>"Sidebar"</strong>
                            </View>
                        }
                    }
                    second=move || {
                        view! {
                            <View
                                background=ViewBackground::Default
                                border=ViewBorder::None
                                padding=ViewPadding::Md
                                radius=ViewRadius::None
                            >
                                <strong>"Content"</strong>
                            </View>
                        }
                    }
                />
            </Playground>

            <Playground
                title="Config Workbench"
                description="Covers full Resizable API with callback feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="resizable-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Orientation"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| set_workbench_orientation_key.set(event_target_value(&ev))
                            >
                                <option value="horizontal" selected=move || workbench_orientation_key.get() == "horizontal">"Horizontal"</option>
                                <option value="vertical" selected=move || workbench_orientation_key.get() == "vertical">"Vertical"</option>
                            </select>
                        </label>
                        <label class="docs-choice-row">
                            <span>"Min"</span>
                            <input
                                type="number"
                                prop:value=move || workbench_min.get().to_string()
                                on:change=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().ok().unwrap_or(25.0);
                                    set_workbench_min.set(next);
                                }
                            />
                        </label>
                        <label class="docs-choice-row">
                            <span>"Max"</span>
                            <input
                                type="number"
                                prop:value=move || workbench_max.get().to_string()
                                on:change=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().ok().unwrap_or(80.0);
                                    set_workbench_max.set(next);
                                }
                            />
                        </label>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_with_handle set_checked=set_workbench_with_handle>"With handle"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Custom motion"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="resizable-workbench-preview">
                    <Resizable
                        orientation=workbench_orientation.get()
                        value=workbench_value
                        split_percent=workbench_split_percent
                        default_value=58.0
                        default_split_percent=58.0
                        on_value_change=on_value_change
                        on_split_percent_change=on_split_percent_change
                        min_split_percent=workbench_min.get()
                        max_split_percent=workbench_max.get()
                        is_disabled=workbench_disabled.get()
                        disabled=workbench_disabled.get()
                        is_with_handle=workbench_with_handle.get()
                        with_handle=workbench_with_handle.get()
                        aria_label="Workspace split".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-resizable-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        motion=if workbench_custom_motion.get() {
                            ResizableMotion {
                                enabled: true,
                                panel_duration_ms: 120,
                                handle_duration_ms: 120,
                            }
                        } else {
                            ResizableMotion::default()
                        }
                        first=move || {
                            view! {
                                <View
                                    background=ViewBackground::Subtle
                                    border=ViewBorder::Subtle
                                    padding=ViewPadding::Md
                                    radius=ViewRadius::None
                                >
                                    <strong>"Primary panel"</strong>
                                </View>
                            }
                        }
                        second=move || {
                            view! {
                                <View
                                    background=ViewBackground::Default
                                    border=ViewBorder::None
                                    padding=ViewPadding::Md
                                    radius=ViewRadius::None
                                >
                                    <strong>"Secondary panel"</strong>
                                </View>
                            }
                        }
                    />
                    <span class="ui-muted">
                        "split=" {move || format!("{:.1}", workbench_split_raw.get())}
                        " · on_value_change=" {move || last_value_change.get()}
                        " · on_split_percent_change=" {move || last_split_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <Resizable
                        orientation=ResizableOrientation::Horizontal
                        default_value=40.0
                        first=move || view! { <div>"Sidebar"</div> }
                        second=move || view! { <div>"Content"</div> }
                    />
                    <Resizable
                        orientation=ResizableOrientation::Vertical
                        default_value=60.0
                        is_with_handle=true
                        first=move || view! { <div>"Header"</div> }
                        second=move || view! { <div>"Body"</div> }
                    />
                    <Resizable
                        orientation=ResizableOrientation::Horizontal
                        default_value=35.0
                        is_disabled=true
                        first=move || view! { <div>"Disabled left"</div> }
                        second=move || view! { <div>"Disabled right"</div> }
                    />
                </div>
            </Playground>

            <Playground title="Controlled + Vertical Bounds" code_signal=vertical_code>
                <div class="docs-stack docs-stack--tight">
                    <Resizable
                        orientation=ResizableOrientation::Vertical
                        value=split
                        on_value_change=on_split_change
                        min_split_percent=25.0
                        max_split_percent=80.0
                        is_with_handle=true
                        aria_label="Deployment regions split".to_string()
                        class_name="docs-resizable-custom".to_string()
                        first=move || view! { <div>"Header"</div> }
                        second=move || view! { <div>"Body"</div> }
                    />
                    <span class="ui-muted">
                        "controlled split: "
                        {move || format!("{:.1}%", split_raw.get())}
                    </span>
                </div>
            </Playground>

            <section class="docs-stack docs-stack--tight" data-slot="resizable-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="resizable-api-rows">
                    <li><code>"orientation / value / on_value_change / min_split_percent / max_split_percent"</code></li>
                    <li><code>"is_with_handle / is_disabled / aria_label / class_name"</code></li>
                </ul>
            </section>

            <section class="docs-stack docs-stack--tight" data-slot="resizable-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="resizable-state-rows">
                    <li><code>"idle / dragging / disabled"</code></li>
                    <li><code>"controlled / uncontrolled split state"</code></li>
                </ul>
            </section>

            <section class="docs-stack docs-stack--tight" data-slot="resizable-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>"Copy starter"</p>
                <ul data-slot="resizable-source-paths">
                    <li><code>"component-resizable"</code></li>
                    <li><code>"crates/ui-layout/src/resizable/view.rs"</code></li>
                </ul>
                <ul data-slot="resizable-source-prerequisites">
                    <li><code>"compose_copy_ready_code"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
