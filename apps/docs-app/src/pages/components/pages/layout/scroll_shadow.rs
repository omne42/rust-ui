use super::*;

pub(crate) fn scroll_shadow() -> AnyView {
    let hello_code = Signal::derive(move || {
        r#"<ScrollShadow>
  <div class="docs-scroll-shadow-item">Activity</div>
</ScrollShadow>"#
            .to_string()
    });

    let default_code = Signal::derive(move || {
        r#"<ScrollShadow max_height_px=160>
  <div class="docs-stack docs-stack--tight">
    {(1..=20)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Activity {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollShadow>"#
            .to_string()
    });

    let custom_class_code = Signal::derive(move || {
        r#"<ScrollShadow max_height_px=120 class_name="docs-scroll-shadow-custom".to_string()>
  <div class="docs-stack docs-stack--tight">
    {(1..=16)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Notification {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollShadow>"#
            .to_string()
    });
    let (workbench_max_height_small, set_workbench_max_height_small) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_code = Signal::derive(move || {
        format!(
            "<ScrollShadow\n  max_height_px={}\n  class_name={}\n>\n  <div class=\"docs-stack docs-stack--tight\">...</div>\n</ScrollShadow>",
            if workbench_max_height_small.get() {
                "120"
            } else {
                "220"
            },
            if workbench_custom_class.get() {
                "\"docs-scroll-shadow-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ScrollShadowWorkbenchConfig {{\n  class_name: {},\n  max_height_px: {},\n}}",
            if workbench_custom_class.get() {
                "Some(\"docs-scroll-shadow-custom\")"
            } else {
                "None"
            },
            if workbench_max_height_small.get() {
                "120"
            } else {
                "220"
            }
        )
    });

    view! {
        <ComponentPage
            title="ScrollShadow"
            slug="scroll-shadow"
            group="Layout"
            description="Adds top/bottom shadow indicators with centralized edge/max-height state attrs."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ScrollShadow>
                    <div class="docs-scroll-shadow-item">"Activity"</div>
                </ScrollShadow>
            </Playground>

            <Playground
                title="Workbench (Max Height + Class)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_max_height_small.get()
                                on:change=move |ev| {
                                    set_workbench_max_height_small.set(event_target_checked(&ev))
                                }
                            />
                            " max_height_px=120"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| {
                                    set_workbench_custom_class.set(event_target_checked(&ev))
                                }
                            />
                            " class_name=docs-scroll-shadow-custom"
                        </label>
                    </div>
                }
            >
                <ScrollShadow
                    max_height_px=if workbench_max_height_small.get() {
                        120
                    } else {
                        220
                    }
                    class_name=if workbench_custom_class.get() {
                        "docs-scroll-shadow-custom".to_string()
                    } else {
                        String::new()
                    }
                >
                    <div class="docs-stack docs-stack--tight">
                        {(1..=16)
                            .map(|idx| {
                                view! { <div class="docs-scroll-shadow-item">{format!("Workbench {idx}")}</div> }
                            })
                            .collect_view()}
                    </div>
                </ScrollShadow>
            </Playground>

            <Playground title="Default Scrollable" code_signal=default_code>
                <ScrollShadow max_height_px=160>
                    <div class="docs-stack docs-stack--tight">
                        {(1..=20)
                            .map(|idx| {
                                view! { <div class="docs-scroll-shadow-item">{format!("Activity {idx}")}</div> }
                            })
                            .collect_view()}
                    </div>
                </ScrollShadow>
            </Playground>

            <Playground title="Custom Height + Class" code_signal=custom_class_code>
                <ScrollShadow max_height_px=120 class_name="docs-scroll-shadow-custom".to_string()>
                    <div class="docs-stack docs-stack--tight">
                        {(1..=16)
                            .map(|idx| {
                                view! {
                                    <div class="docs-scroll-shadow-item">
                                        {format!("Notification {idx}")}
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </ScrollShadow>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
