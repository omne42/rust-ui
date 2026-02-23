use super::*;

pub(crate) fn preview_card() -> AnyView {
    let hello_code = Signal::derive(move || {
        r##"<PreviewCard
  trigger=move || view! { <Button variant=ButtonVariant::Secondary>"Open preview"</Button> }
/>"##
            .to_string()
    });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_top_end, set_workbench_top_end) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_open_delay_ms, set_workbench_open_delay_ms) = signal(220_u64);
    let (workbench_close_delay_ms, set_workbench_close_delay_ms) = signal(200_u64);

    let workbench_code = Signal::derive(move || {
        format!(
            "<PreviewCard\n  trigger=move || view! {{ <Button variant=ButtonVariant::Secondary>\"Workbench trigger\"</Button> }}\n  title=Some(\"Workbench title\".to_string())\n  description=Some(\"Workbench description\".to_string())\n  url=Some(\"https://ui-baseline.adobe.com\".to_string())\n  site_label=Some(\"ui-baseline.adobe.com\".to_string())\n  image_src=Some(\"https://ui-baseline.adobe.com/static/logo.png\".to_string())\n  disabled={}\n  placement={}\n  open_delay_ms={}\n  close_delay_ms={}\n  motion={}\n  class_name={}\n  id={}\n/>",
            workbench_disabled.get(),
            if workbench_top_end.get() {
                "ui_headless::PopoverPlacement::TopEnd"
            } else {
                "ui_headless::PopoverPlacement::BottomStart"
            },
            workbench_open_delay_ms.get(),
            workbench_close_delay_ms.get(),
            if workbench_custom_motion.get() {
                "PreviewCardMotion { initial_scale: 0.95, offset_y_px: 12.0, ..PreviewCardMotion::default() }"
            } else {
                "PreviewCardMotion::default()"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-preview-card-workbench\".to_string())"
            } else {
                "None"
            },
            if workbench_custom_id.get() {
                "Some(\"docs-preview-card-workbench\".to_string())"
            } else {
                "None"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "PreviewCardActualConfig {{\n  trigger: \"Button(ViewFn)\",\n  title: Some(\"Workbench title\"),\n  description: Some(\"Workbench description\"),\n  url: Some(\"https://ui-baseline.adobe.com\"),\n  site_label: Some(\"ui-baseline.adobe.com\"),\n  image_src: Some(\"https://ui-baseline.adobe.com/static/logo.png\"),\n  disabled: {},\n  placement: {},\n  open_delay_ms: {},\n  close_delay_ms: {},\n  motion: {},\n  class_name: {},\n  id: {},\n}}",
            workbench_disabled.get(),
            if workbench_top_end.get() {
                "PopoverPlacement::TopEnd"
            } else {
                "PopoverPlacement::BottomStart"
            },
            workbench_open_delay_ms.get(),
            workbench_close_delay_ms.get(),
            if workbench_custom_motion.get() {
                "PreviewCardMotion::custom"
            } else {
                "PreviewCardMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-preview-card-workbench\")"
            } else {
                "None"
            },
            if workbench_custom_id.get() {
                "Some(\"docs-preview-card-workbench\")"
            } else {
                "None"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r##"<PreviewCard trigger=move || view! { <Button variant=ButtonVariant::Secondary>"Default"</Button> } />
<PreviewCard disabled=true placement=ui_headless::PopoverPlacement::TopEnd trigger=move || view! { <Button variant=ButtonVariant::Secondary>"Disabled"</Button> } />
<PreviewCard class_name="docs-preview-card-custom".to_string() trigger=move || view! { <Button variant=ButtonVariant::Secondary>"Custom class"</Button> } />"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="PreviewCard"
            slug="preview-card"
            group="Overlays"
            description="PreviewCard playground with full API workbench and matrix comparison."
        >
            <Playground title="Hello World (Default PreviewCard)" code_signal=hello_code>
                <PreviewCard
                    trigger=move || {
                        view! { <Button variant=ButtonVariant::Secondary>"Open preview"</Button> }
                    }
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_top_end set_checked=set_workbench_top_end>
                            "placement top-end"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                            "id"
                        </Switch>
                        <label class="docs-search__label">
                            "open_delay_ms: " {move || workbench_open_delay_ms.get()}
                            <input
                                type="range"
                                min="0"
                                max="1000"
                                step="20"
                                prop:value=move || workbench_open_delay_ms.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<u64>().unwrap_or(220);
                                    set_workbench_open_delay_ms.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "close_delay_ms: " {move || workbench_close_delay_ms.get()}
                            <input
                                type="range"
                                min="0"
                                max="1000"
                                step="20"
                                prop:value=move || workbench_close_delay_ms.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<u64>().unwrap_or(200);
                                    set_workbench_close_delay_ms.set(next);
                                }
                            />
                        </label>
                    </div>
                }
            >
                <PreviewCard
                    trigger=move || {
                        view! { <Button variant=ButtonVariant::Secondary>"Workbench trigger"</Button> }
                    }
                    title="Workbench title".to_string()
                    description="Workbench description".to_string()
                    url="https://ui-baseline.adobe.com".to_string()
                    site_label="ui-baseline.adobe.com".to_string()
                    image_src="https://ui-baseline.adobe.com/static/logo.png".to_string()
                    disabled=workbench_disabled.get()
                    placement=if workbench_top_end.get() {
                        ui_headless::PopoverPlacement::TopEnd
                    } else {
                        ui_headless::PopoverPlacement::BottomStart
                    }
                    open_delay_ms=workbench_open_delay_ms.get()
                    close_delay_ms=workbench_close_delay_ms.get()
                    motion=if workbench_custom_motion.get() {
                        PreviewCardMotion {
                            initial_scale: 0.95,
                            offset_y_px: 12.0,
                            ..PreviewCardMotion::default()
                        }
                    } else {
                        PreviewCardMotion::default()
                    }
                    class_name=if workbench_custom_class.get() {
                        "docs-preview-card-workbench".to_string()
                    } else {
                        String::new()
                    }
                    id=if workbench_custom_id.get() {
                        "docs-preview-card-workbench".to_string()
                    } else {
                        String::new()
                    }
                />
            </Playground>

            <Playground title="State Matrix (Default / Disabled / Custom)" code_signal=matrix_code>
                <div class="docs-row">
                    <PreviewCard
                        trigger=move || {
                            view! { <Button variant=ButtonVariant::Secondary>"Default"</Button> }
                        }
                    />
                    <PreviewCard
                        disabled=true
                        placement=ui_headless::PopoverPlacement::TopEnd
                        trigger=move || {
                            view! { <Button variant=ButtonVariant::Secondary>"Disabled"</Button> }
                        }
                    />
                    <PreviewCard
                        class_name="docs-preview-card-custom".to_string()
                        trigger=move || {
                            view! { <Button variant=ButtonVariant::Secondary>"Custom class"</Button> }
                        }
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
