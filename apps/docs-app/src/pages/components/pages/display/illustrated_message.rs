use super::*;

pub(crate) fn illustrated_message() -> AnyView {
    let code_imports = "use leptos::prelude::*;\nuse ui::{Button, IllustratedMessage};".to_string();
    let workbench_orientation_options = vec!["vertical".to_string(), "horizontal".to_string()];
    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let (workbench_show_title, set_workbench_show_title) = signal(true);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_illustration, set_workbench_show_illustration) = signal(false);
    let (workbench_show_actions, set_workbench_show_actions) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_lang, set_workbench_custom_lang) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let hello_world_code = Signal::derive(move || {
        r#"<IllustratedMessage title="Empty".to_string() description="Nothing here".to_string() />"#
            .to_string()
    });
    let state_matrix_code = Signal::derive(move || {
        r#"<div class="docs-stack docs-stack--tight">
  <IllustratedMessage
    title="Empty".to_string()
    description="Nothing here".to_string()
  />
  <IllustratedMessage
    title="No results".to_string()
    description="Try changing your search.".to_string()
    illustration=move || view! { <div class="docs-illustration">"◎"</div> }
    actions=move || view! { <Button>"Clear"</Button> }
  />
  <IllustratedMessage
    description="Only description provided.".to_string()
  />
</div>"#
            .to_string()
    });
    let controlled_contrast_code = Signal::derive(move || {
        r#"let is_filtered_empty = true;

<IllustratedMessage
  title="Default path".to_string()
  description="No controlled axis; props-only snapshot render.".to_string()
/>
<IllustratedMessage
  title=if is_filtered_empty { "Filtered empty".to_string() } else { "Results ready".to_string() }
  description="N/A: no value/on_value_change/default_value axis.".to_string()
/>"#
        .to_string()
    });
    let stream_snapshot_code = Signal::derive(move || {
        r#"<IllustratedMessage
  title="Snapshot result".to_string()
  description="Complete validated output rendered in one pass.".to_string()
/>
<IllustratedMessage
  title="Streaming Optional -> Snapshot".to_string()
  description="Display leaf consumes snapshot output; upstream keeps streaming lifecycle.".to_string()
/>"#
            .to_string()
    });
    let source_first_code = Signal::derive(move || {
        r#"<IllustratedMessage
  title="No results".to_string()
  description="Try changing your search.".to_string()
  illustration=move || view! { <div class="docs-illustration">"◎"</div> }
  actions=move || view! { <Button>"Clear"</Button> }
/>"#
        .to_string()
    });
    let workbench_code = Signal::derive(move || {
        let orientation_expr = match workbench_orientation_index.get().unwrap_or(0) {
            1 => "ui::IllustratedMessageOrientation::Horizontal",
            _ => "ui::IllustratedMessageOrientation::Vertical",
        };

        let mut lines = vec!["<IllustratedMessage".to_string()];
        if workbench_show_title.get() {
            lines.push("  title=\"Workbench empty\".to_string()".to_string());
        }
        if workbench_show_description.get() {
            lines.push(
                "  description=\"Preview updates as you toggle props.\".to_string()".to_string(),
            );
        }
        if workbench_show_illustration.get() {
            lines.push(
                "  illustration=move || view! { <div class=\"docs-illustration\">\"◎\"</div> }"
                    .to_string(),
            );
        }
        if workbench_show_actions.get() {
            lines.push(
                "  actions=move || view! { <Button variant=ui::ButtonVariant::Secondary size=ui::ButtonSize::Sm>\"Retry\"</Button> }".to_string(),
            );
        }
        lines.push(format!("  orientation={orientation_expr}"));
        if workbench_custom_class.get() {
            lines.push(
                "  class_name=\"docs-illustrated-message-workbench\".to_string()".to_string(),
            );
        }
        if workbench_custom_lang.get() {
            lines.push("  lang=\"zh-CN\".to_string()".to_string());
        }
        if workbench_rtl.get() {
            lines.push("  dir=ui::color::area::A11yDirection::Rtl".to_string());
        } else {
            lines.push("  dir=ui::color::area::A11yDirection::Ltr".to_string());
        }
        if workbench_custom_motion.get() {
            lines.push(
                "  motion=ui::IllustratedMessageMotion { spring: ui::IllustratedMessageMotion::default().spring }"
                    .to_string(),
            );
        } else {
            lines.push("  motion=ui::IllustratedMessageMotion::default()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });
    let workbench_config = Signal::derive(move || {
        let orientation = match workbench_orientation_index.get().unwrap_or(0) {
            1 => "horizontal",
            _ => "vertical",
        };
        format!(
            "IllustratedMessageWorkbenchConfig {{ orientation: \"{orientation}\", show_title: {}, show_description: {}, show_illustration: {}, show_actions: {}, class_name: {:?}, lang: {:?}, dir: {:?}, motion: {:?}, custom_class: {}, rtl: {} }}",
            workbench_show_title.get(),
            workbench_show_description.get(),
            workbench_show_illustration.get(),
            workbench_show_actions.get(),
            if workbench_custom_class.get() {
                Some("docs-illustrated-message-workbench")
            } else {
                None
            },
            if workbench_custom_lang.get() {
                Some("zh-CN")
            } else {
                None
            },
            if workbench_rtl.get() {
                A11yDirection::Rtl
            } else {
                A11yDirection::Ltr
            },
            if workbench_custom_motion.get() {
                ui::IllustratedMessageMotion {
                    spring: ui::IllustratedMessageMotion::default().spring,
                    initial_y_px: ui::IllustratedMessageMotion::default().initial_y_px,
                }
            } else {
                ui::IllustratedMessageMotion::default()
            },
            workbench_custom_class.get(),
            workbench_rtl.get(),
        )
    });

    view! {
        <ComponentPage
            title="IllustratedMessage"
            slug="illustrated-message"
            group="Display"
            description="Empty-state component with optional illustration and actions."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=code_imports.clone()
                test_source_path="components/illustrated-message/src/view.rs".to_string()
            >
                <IllustratedMessage
                    title="Empty".to_string()
                    description="Nothing here".to_string()
                />
            </Playground>

            <Playground
                title="Interactive Playground (Props + State + Preview)"
                description="Live controls for slot toggles and orientation/dir mapping; preview state markers update in real time."
                code_signal=workbench_code
                code_imports=code_imports.clone()
                test_source_path="components/illustrated-message/src/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="illustrated-message-workbench-controls">
                        <div data-slot="illustrated-message-workbench-orientation">
                            <SegmentedControl
                                id_base="docs-illustrated-message-workbench-orientation".to_string()
                                options=workbench_orientation_options.clone()
                                selected_index=workbench_orientation_index
                                set_selected_index=set_workbench_orientation_index
                                size=SegmentedControlSize::Sm
                            />
                        </div>
                        <div class="docs-row">
                            <div data-slot="illustrated-message-workbench-toggle-title">
                                <Switch checked=workbench_show_title set_checked=set_workbench_show_title>
                                    "Show title"
                                </Switch>
                            </div>
                            <div data-slot="illustrated-message-workbench-toggle-description">
                                <Switch checked=workbench_show_description set_checked=set_workbench_show_description>
                                    "Show description"
                                </Switch>
                            </div>
                            <div data-slot="illustrated-message-workbench-toggle-illustration">
                                <Switch checked=workbench_show_illustration set_checked=set_workbench_show_illustration>
                                    "Show illustration"
                                </Switch>
                            </div>
                            <div data-slot="illustrated-message-workbench-toggle-actions">
                                <Switch checked=workbench_show_actions set_checked=set_workbench_show_actions>
                                    "Show actions"
                                </Switch>
                            </div>
                        </div>
                            <div class="docs-row">
                                <div data-slot="illustrated-message-workbench-toggle-custom-class">
                                    <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                        "Custom class"
                                    </Switch>
                                </div>
                                <div data-slot="illustrated-message-workbench-toggle-custom-lang">
                                    <Switch checked=workbench_custom_lang set_checked=set_workbench_custom_lang>
                                        "Lang=zh-CN"
                                    </Switch>
                                </div>
                                <div data-slot="illustrated-message-workbench-toggle-custom-motion">
                                    <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                                        "Custom motion"
                                    </Switch>
                                </div>
                                <div data-slot="illustrated-message-workbench-toggle-rtl">
                                    <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                                        "RTL"
                                </Switch>
                            </div>
                        </div>
                    </div>
                }
            >
                {move || {
                    let orientation = match workbench_orientation_index.get().unwrap_or(0) {
                        1 => ui::IllustratedMessageOrientation::Horizontal,
                        _ => ui::IllustratedMessageOrientation::Vertical,
                    };
                    let orientation_label = match orientation {
                        ui::IllustratedMessageOrientation::Horizontal => "horizontal",
                        ui::IllustratedMessageOrientation::Vertical => "vertical",
                    };
                    let title = if workbench_show_title.get() {
                        "Workbench empty".to_string()
                    } else {
                        String::new()
                    };
                    let description = if workbench_show_description.get() {
                        "Preview updates as you toggle props.".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if workbench_custom_class.get() {
                        "docs-illustrated-message-workbench".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if workbench_custom_lang.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    };
                    let dir = if workbench_rtl.get() {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };
                    let motion = if workbench_custom_motion.get() {
                        ui::IllustratedMessageMotion {
                            spring: ui::IllustratedMessageMotion::default().spring,
                            initial_y_px: ui::IllustratedMessageMotion::default().initial_y_px,
                        }
                    } else {
                        ui::IllustratedMessageMotion::default()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="illustrated-message-workbench-preview">
                            {if workbench_show_illustration.get() && workbench_show_actions.get() {
                                view! {
                                    <IllustratedMessage
                                        title=title.clone()
                                        description=description.clone()
                                        illustration=move || view! { <div class="docs-illustration">"◎"</div> }
                                        actions=move || {
                                            view! {
                                                <ui::Button
                                                    variant=ui::ButtonVariant::Secondary
                                                    size=ui::ButtonSize::Sm
                                                >
                                                    "Retry"
                                                </ui::Button>
                                            }
                                        }
                                        orientation=orientation
                                        motion=motion
                                        class_name=class_name.clone()
                                        lang=lang.clone()
                                        dir=dir
                                    />
                                }
                                    .into_any()
                            } else if workbench_show_illustration.get() {
                                view! {
                                    <IllustratedMessage
                                        title=title.clone()
                                        description=description.clone()
                                        illustration=move || view! { <div class="docs-illustration">"◎"</div> }
                                        orientation=orientation
                                        motion=motion
                                        class_name=class_name.clone()
                                        lang=lang.clone()
                                        dir=dir
                                    />
                                }
                                    .into_any()
                            } else if workbench_show_actions.get() {
                                view! {
                                    <IllustratedMessage
                                        title=title.clone()
                                        description=description.clone()
                                        actions=move || {
                                            view! {
                                                <ui::Button
                                                    variant=ui::ButtonVariant::Secondary
                                                    size=ui::ButtonSize::Sm
                                                >
                                                    "Retry"
                                                </ui::Button>
                                            }
                                        }
                                        orientation=orientation
                                        motion=motion
                                        class_name=class_name.clone()
                                        lang=lang.clone()
                                        dir=dir
                                    />
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <IllustratedMessage
                                        title=title
                                        description=description
                                        orientation=orientation
                                        motion=motion
                                        class_name=class_name
                                        lang=lang
                                        dir=dir
                                    />
                                }
                                    .into_any()
                            }}
                            <span class="ui-muted" data-slot="illustrated-message-workbench-state">
                                {format!(
                                    "orientation={orientation_label}, title={}, description={}, illustration={}, actions={}, rtl={}, custom_class={}",
                                    workbench_show_title.get(),
                                    workbench_show_description.get(),
                                    workbench_show_illustration.get(),
                                    workbench_show_actions.get(),
                                    workbench_rtl.get(),
                                    workbench_custom_class.get(),
                                )}
                            </span>
                        </div>
                    }
                    .into_any()
                }}
            </Playground>

            <Playground
                title="State Matrix"
                description="Covers default, rich slots, and partial-content states with stable aria/data markers."
                code_signal=state_matrix_code
                code_imports=code_imports.clone()
                test_source_path="components/illustrated-message/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <IllustratedMessage
                        title="Empty".to_string()
                        description="Nothing here".to_string()
                    />
                    <IllustratedMessage
                        title="No results".to_string()
                        description="Try changing your search.".to_string()
                        illustration=move || view! { <div class="docs-illustration">"◎"</div> }
                        actions=move || view! { <ui::Button>"Clear"</ui::Button> }
                    />
                    <IllustratedMessage description="Only description provided.".to_string() />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="IllustratedMessage is display-only; compare default props and app-state-mapped props without internal state axis."
                code_signal=controlled_contrast_code
                code_imports=code_imports.clone()
                test_source_path="components/illustrated-message/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <IllustratedMessage
                        title="Default path".to_string()
                        description="No controlled axis; props-only snapshot render.".to_string()
                    />
                    <IllustratedMessage
                        title="Filtered empty".to_string()
                        description="Mapped from external app state.".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Display leaf keeps snapshot rendering; streaming lifecycle stays in upstream orchestration."
                code_signal=stream_snapshot_code
                code_imports=code_imports.clone()
                test_source_path="components/illustrated-message/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="illustrated-message-streaming-preview">
                    <p class="ui-muted" data-slot="illustrated-message-streaming-policy">
                        "Streaming Optional -> fallback=snapshot."
                    </p>
                    <p class="ui-muted" data-slot="illustrated-message-copy-ready-hint">
                        "Copy-ready snippets prepend missing imports automatically."
                    </p>
                    <IllustratedMessage
                        title="Snapshot result".to_string()
                        description="Complete validated output rendered in one pass.".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Playground copy action injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=code_imports.clone()
                test_source_path="components/illustrated-message/src/view.rs".to_string()
            >
                <IllustratedMessage
                    title="No results".to_string()
                    description="Try changing your search.".to_string()
                    illustration=move || view! { <div class="docs-illustration">"◎"</div> }
                    actions=move || view! { <ui::Button>"Clear"</ui::Button> }
                />
            </Playground>

            <Playground
                title="State Matrix (Default / Rich / Locale Comparison)"
                code_signal=state_matrix_code
                code_imports=code_imports.clone()
                test_source_path="components/illustrated-message/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <IllustratedMessage
                        title="Empty".to_string()
                        description="Nothing here".to_string()
                        motion=ui::IllustratedMessageMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <IllustratedMessage
                        title="No results".to_string()
                        description="Try changing your search.".to_string()
                        illustration=move || view! { <div class="docs-illustration">"◎"</div> }
                        actions=move || view! { <ui::Button>"Clear"</ui::Button> }
                        orientation=ui::IllustratedMessageOrientation::Horizontal
                        motion=ui::IllustratedMessageMotion::default()
                        class_name="docs-illustrated-message-workbench".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="illustrated-message-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="illustrated-message-source-first-contract">
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="illustrated-message-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-illustrated_message"</code>
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
                    text="use leptos::prelude::*;\nuse ui::{Button, IllustratedMessage};\n\n<IllustratedMessage\n  title=\"No results\".to_string()\n  description=\"Try changing your search.\".to_string()\n  illustration=move || view! { <div class=\"docs-illustration\">\"◎\"</div> }\n  actions=move || view! { <Button>\"Clear\"</Button> }\n/>".to_string()
                    label="Copy illustrated-message starter".to_string()
                    copyable=true
                    class_name="docs-illustrated-message-source-copy".to_string()
                />
                <ul data-slot="illustrated-message-source-paths">
                    <li><code>"components/illustrated-message/src/mod.rs"</code></li>
                    <li><code>"components/illustrated-message/src/logic.rs"</code></li>
                    <li><code>"components/illustrated-message/src/view.rs"</code></li>
                    <li><code>"components/illustrated-message/src/styles.rs"</code></li>
                    <li><code>"components/illustrated-message/src/motion.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
