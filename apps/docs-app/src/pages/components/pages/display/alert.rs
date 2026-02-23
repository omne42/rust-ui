use super::*;

pub(crate) fn alert() -> AnyView {
    let hello_world_code = Signal::derive(move || {
        r#"<Alert>
  "Install now to keep your workspace secure."
</Alert>"#
            .to_string()
    });

    let tone_fill_code = Signal::derive(move || {
        r#"<Alert
  tone=AlertTone::Info
  fill=AlertFill::Border
  title="Updates available".to_string()
  description="A new version is ready to install.".to_string()
>
  "Install now to keep your workspace secure."
</Alert>
<Alert
  tone=AlertTone::Negative
  fill=AlertFill::Subtle
  title="Deployment failed".to_string()
  description="Rollback completed. Review incident timeline.".to_string()
>
  "Check logs before retrying."
</Alert>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Alert
  tone=AlertTone::Notice
  fill=AlertFill::Bold
  is_hide_icon=true
  title="Maintenance window".to_string()
  description="Service may be degraded during migration.".to_string()
  start_content=move || view! { <span>"↳"</span> }
  end_content=move || view! { <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm>"Details"</Button> }
  class_name="docs-alert-custom".to_string()
>
  "Follow status page for live updates."
</Alert>"#
            .to_string()
    });

    let compatibility_code = Signal::derive(move || {
        r#"<Alert
  variant=AlertVariant::Danger
  fill=AlertFill::Border
  title="Legacy variant mapping".to_string()
  description="`variant` now maps into tone semantics.".to_string()
  motion=AlertMotion { spring: Default::default() }
>
  "Use tone/fill for new code; variant remains a compatibility input."
</Alert>"#
            .to_string()
    });

    let inline_layout_code = Signal::derive(move || {
        r#"<Alert
  layout=AlertLayout::Inline
  tone=AlertTone::Info
  fill=AlertFill::Subtle
  title="Inline mode".to_string()
  description="Use layout=Inline to replace old InlineAlert.".to_string()
>
  "Compact inline content."
</Alert>"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"let service_degraded = true;

<Alert>
  "Default path: no controlled/uncontrolled state axis."
</Alert>
<Alert
  tone=if service_degraded { Some(AlertTone::Negative) } else { Some(AlertTone::Info) }
  fill=AlertFill::Subtle
  title="App-state mapping".to_string()
  description="Alert is stateless; upstream state only maps to props.".to_string()
>
  "Controlled-like usage lives in app state, not inside Alert."
</Alert>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Alert
  tone=AlertTone::Info
  fill=AlertFill::Border
  title="Snapshot".to_string()
  description="Complete validated output rendered in one pass.".to_string()
>
  "Stable snapshot result."
</Alert>
<Alert
  tone=AlertTone::Notice
  fill=AlertFill::Subtle
  title="Streaming Optional -> Snapshot".to_string()
  description="Alert exposes optional streaming metadata, while rendering snapshot output.".to_string()
>
  "Inspect data-ui-streaming=optional, data-ui-fallback=snapshot, data-ui-state=snapshot."
</Alert>"#
            .to_string()
    });

    let workbench_tone_options = vec![
        "neutral".to_string(),
        "info".to_string(),
        "positive".to_string(),
        "notice".to_string(),
        "negative".to_string(),
    ];
    let workbench_fill_options = vec![
        "border".to_string(),
        "subtle".to_string(),
        "bold".to_string(),
    ];
    let workbench_layout_options = vec!["banner".to_string(), "inline".to_string()];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(1_usize));
    let (workbench_fill_index, set_workbench_fill_index) = signal(Some(0_usize));
    let (workbench_layout_index, set_workbench_layout_index) = signal(Some(0_usize));
    let (workbench_hide_icon, set_workbench_hide_icon) = signal(false);
    let (workbench_show_title, set_workbench_show_title) = signal(true);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(1) {
        0 => AlertTone::Neutral,
        2 => AlertTone::Positive,
        3 => AlertTone::Notice,
        4 => AlertTone::Negative,
        _ => AlertTone::Info,
    });
    let workbench_fill = Signal::derive(move || match workbench_fill_index.get().unwrap_or(0) {
        1 => AlertFill::Subtle,
        2 => AlertFill::Bold,
        _ => AlertFill::Border,
    });
    let workbench_layout =
        Signal::derive(move || match workbench_layout_index.get().unwrap_or(0) {
            1 => AlertLayout::Inline,
            _ => AlertLayout::Banner,
        });

    let workbench_code = Signal::derive(move || {
        let tone = workbench_tone.get();
        let fill = workbench_fill.get();
        let layout = workbench_layout.get();
        let hide_icon = workbench_hide_icon.get();
        let show_title = workbench_show_title.get();
        let show_description = workbench_show_description.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();

        let tone_expr = match tone {
            AlertTone::Neutral => "AlertTone::Neutral",
            AlertTone::Info => "AlertTone::Info",
            AlertTone::Positive => "AlertTone::Positive",
            AlertTone::Notice => "AlertTone::Notice",
            AlertTone::Negative => "AlertTone::Negative",
        };
        let fill_expr = match fill {
            AlertFill::Border => "AlertFill::Border",
            AlertFill::Subtle => "AlertFill::Subtle",
            AlertFill::Bold => "AlertFill::Bold",
        };
        let layout_expr = match layout {
            AlertLayout::Banner => "AlertLayout::Banner",
            AlertLayout::Inline => "AlertLayout::Inline",
        };

        let mut lines = vec![
            "<Alert".to_string(),
            format!("  tone={tone_expr}"),
            format!("  fill={fill_expr}"),
            format!("  layout={layout_expr}"),
        ];
        if hide_icon {
            lines.push("  is_hide_icon=true".to_string());
        }
        if show_title {
            lines.push("  title=\"Interactive status\".to_string()".to_string());
        }
        if show_description {
            lines.push(
                "  description=\"Props update in real time; inspect data-* markers.\".to_string()"
                    .to_string(),
            );
        }
        if custom_class {
            lines.push("  class_name=\"docs-alert-custom\".to_string()".to_string());
        }
        if rtl {
            lines.push("  lang=\"ar\".to_string()".to_string());
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.extend([
            ">".to_string(),
            "  \"Observe semantic markers update in real time.\"".to_string(),
            "</Alert>".to_string(),
        ]);
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/alert/src/styles.rs */\n{}",
            ui::alert::styles::CSS
        )
    });

    let workbench_config = Signal::derive(move || {
        let tone = workbench_tone.get();
        let fill = workbench_fill.get();
        let layout = workbench_layout.get();
        let hide_icon = workbench_hide_icon.get();
        let show_title = workbench_show_title.get();
        let show_description = workbench_show_description.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();

        let tone_attr = match tone {
            AlertTone::Neutral => "neutral",
            AlertTone::Info => "info",
            AlertTone::Positive => "positive",
            AlertTone::Notice => "notice",
            AlertTone::Negative => "negative",
        };
        let fill_attr = fill.attr_value();
        let layout_attr = match layout {
            AlertLayout::Banner => "banner",
            AlertLayout::Inline => "inline",
        };

        format!(
            "AlertWorkbenchConfig {{\n  tone: {tone:?},\n  variant: None,\n  layout: {layout:?},\n  fill: {fill:?},\n  title: {},\n  description: {},\n  is_hide_icon: Some({hide_icon}),\n  hide_icon: Some({hide_icon}),\n  icon_label: {},\n  start_content: None,\n  end_content: None,\n  class_name: {},\n  lang: {},\n  dir: {},\n  motion: AlertMotion::default(),\n  marker_expectations: [\"data-tone={tone_attr}\", \"data-fill={fill_attr}\", \"data-layout={layout_attr}\", \"data-hide-icon-source\", \"data-ui-state=snapshot\"],\n}}",
            if show_title {
                "Some(\"Interactive status\")"
            } else {
                "None"
            },
            if show_description {
                "Some(\"Props update in real time; inspect data-* markers.\")"
            } else {
                "None"
            },
            if hide_icon {
                "Some(\"Status icon\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-alert-custom\")"
            } else {
                "None"
            },
            if rtl { "Some(\"ar\")" } else { "None" },
            if rtl { "Some(\"rtl\")" } else { "None" }
        )
    });

    view! {
        <ComponentPage
            title="Alert"
            slug="alert"
            group="Display"
            description="Unified notification surface with tone/fill, optional icon + slots, and motion contracts."
        >
            <Playground
                title="Hello World"
                code_signal=hello_world_code
                code_imports="use leptos::prelude::*;\nuse ui::Alert;".to_string()
                test_source_path="components/alert/src/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Alert>
                        "Install now to keep your workspace secure."
                    </Alert>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::color::area::A11yDirection;\nuse ui::{Alert, AlertFill, AlertLayout, AlertTone};".to_string()
                test_css_source=workbench_test_css
                test_source_path="components/alert/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="在线调整 tone/fill/layout 与 title/description/icon/lang/dir/class，实时预览并复现键盘路径。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="alert-workbench-controls">
                            <div class="docs-search__label">"Tone"</div>
                            <SegmentedControl
                                id_base="docs-alert-workbench-tone".to_string()
                                options=workbench_tone_options.clone()
                                selected_index=workbench_tone_index
                                set_selected_index=set_workbench_tone_index
                                size=SegmentedControlSize::Sm
                                aria_label="Alert tone".to_string()
                            />

                            <div class="docs-search__label">"Fill"</div>
                            <SegmentedControl
                                id_base="docs-alert-workbench-fill".to_string()
                                options=workbench_fill_options.clone()
                                selected_index=workbench_fill_index
                                set_selected_index=set_workbench_fill_index
                                size=SegmentedControlSize::Sm
                                aria_label="Alert fill".to_string()
                            />

                            <div class="docs-search__label">"Layout"</div>
                            <SegmentedControl
                                id_base="docs-alert-workbench-layout".to_string()
                                options=workbench_layout_options.clone()
                                selected_index=workbench_layout_index
                                set_selected_index=set_workbench_layout_index
                                size=SegmentedControlSize::Sm
                                aria_label="Alert layout".to_string()
                            />

                            <Switch checked=workbench_hide_icon set_checked=set_workbench_hide_icon>
                                "is_hide_icon"
                            </Switch>
                            <Switch checked=workbench_show_title set_checked=set_workbench_show_title>
                                "Show title"
                            </Switch>
                            <Switch checked=workbench_show_description set_checked=set_workbench_show_description>
                                "Show description"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class_name"
                            </Switch>
                            <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                                "RTL direction"
                            </Switch>
                        </div>
                    }
                }
            >
                {move || {
                    let tone = workbench_tone.get();
                    let fill = workbench_fill.get();
                    let layout = workbench_layout.get();
                    let hide_icon = workbench_hide_icon.get();
                    let show_title = workbench_show_title.get();
                    let show_description = workbench_show_description.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();

                    let title = if show_title {
                        "Interactive status".to_string()
                    } else {
                        String::new()
                    };
                    let description = if show_description {
                        "Props update in real time; inspect data-* markers.".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-alert-custom".to_string()
                    } else {
                        String::new()
                    };
                    let dir = if rtl {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };
                    let lang = if rtl {
                        "ar".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="alert-workbench-preview">
                            <div class="docs-search__label">"Configured"</div>
                            <Alert
                                tone=tone
                                fill=fill
                                layout=layout
                                is_hide_icon=hide_icon
                                title=title
                                description=description
                                class_name=class_name
                                lang=lang
                                dir=dir
                                end_content=move || view! {
                                    <ui::Button
                                        variant=ui::ButtonVariant::Secondary
                                        size=ui::ButtonSize::Sm
                                    >
                                        "Acknowledge"
                                    </ui::Button>
                                }
                            >
                                "Observe semantic markers update in real time."
                            </Alert>

                            <div class="docs-search__label">"Baseline"</div>
                            <Alert>
                                "Install now to keep your workspace secure."
                            </Alert>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix"
                description="Tone + fill + variant-source matrix with stable semantic markers."
                code_signal=tone_fill_code
                code_imports="use leptos::prelude::*;\nuse ui::{Alert, AlertFill, AlertTone};".to_string()
            >
                <div class="docs-stack">
                    <Alert
                        tone=AlertTone::Info
                        fill=AlertFill::Border
                        title="Updates available".to_string()
                        description="A new version is ready to install.".to_string()
                    >
                        "Install now to keep your workspace secure."
                    </Alert>
                    <Alert
                        tone=AlertTone::Negative
                        fill=AlertFill::Subtle
                        title="Deployment failed".to_string()
                        description="Rollback completed. Review incident timeline.".to_string()
                    >
                        "Check logs before retrying."
                    </Alert>
                </div>
            </Playground>

            <Playground title="Slots + Hidden Icon + Custom Class" code_signal=custom_code>
                <Alert
                    tone=AlertTone::Notice
                    fill=AlertFill::Bold
                    is_hide_icon=true
                    title="Maintenance window".to_string()
                    description="Service may be degraded during migration.".to_string()
                    start_content=move || view! { <span>"↳"</span> }
                    end_content=move || view! {
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            size=ui::ButtonSize::Sm
                        >
                            "Details"
                        </ui::Button>
                    }
                    class_name="docs-alert-custom".to_string()
                >
                    "Follow status page for live updates."
                </Alert>
            </Playground>

            <Playground title="Variant Compatibility + Motion" code_signal=compatibility_code>
                <Alert
                    variant=AlertVariant::Danger
                    fill=AlertFill::Border
                    title="Legacy variant mapping".to_string()
                    description="`variant` now maps into tone semantics.".to_string()
                    motion=AlertMotion {
                        spring: Default::default(),
                    }
                >
                    "Use tone/fill for new code; variant remains a compatibility input."
                </Alert>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Alert has no internal controlled/uncontrolled axis; compare default usage vs app-state-mapped props."
                code_signal=controlled_contrast_code
                code_imports="use leptos::prelude::*;\nuse ui::{Alert, AlertFill, AlertTone};".to_string()
            >
                <div class="docs-stack">
                    <Alert>"Default path: no controlled/uncontrolled state axis."</Alert>
                    <Alert
                        tone=AlertTone::Negative
                        fill=AlertFill::Subtle
                        title="App-state mapping".to_string()
                        description="Alert is stateless; upstream state only maps to props.".to_string()
                    >
                        "Controlled-like usage lives in app state, not inside Alert."
                    </Alert>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Alert is not a body-reader surface: streaming stays optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports="use leptos::prelude::*;\nuse ui::{Alert, AlertFill, AlertTone};".to_string()
            >
                <div class="docs-stack">
                    <Alert
                        tone=AlertTone::Info
                        fill=AlertFill::Border
                        title="Snapshot".to_string()
                        description="Complete validated output rendered in one pass.".to_string()
                    >
                        "Stable snapshot result."
                    </Alert>
                    <Alert
                        tone=AlertTone::Notice
                        fill=AlertFill::Subtle
                        title="Streaming Optional -> Snapshot".to_string()
                        description="Alert exposes optional streaming metadata, while rendering snapshot output.".to_string()
                    >
                        "Inspect data-ui-streaming=optional, data-ui-fallback=snapshot, data-ui-state=snapshot."
                    </Alert>
                </div>
            </Playground>

            <Playground title="Inline Layout" code_signal=inline_layout_code>
                <Alert
                    layout=AlertLayout::Inline
                    tone=AlertTone::Info
                    fill=AlertFill::Subtle
                    title="Inline mode".to_string()
                    description="Use layout=Inline to replace old InlineAlert.".to_string()
                >
                    "Compact inline content."
                </Alert>
            </Playground>

            <section class="docs-card docs-prose" data-slot="alert-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="alert-state-rows">
                    <li><code>"data-tone"</code>" = neutral | info | positive | notice | negative"</li>
                    <li><code>"data-fill"</code>" = border | subtle | bold"</li>
                    <li><code>"data-layout"</code>" = banner | inline"</li>
                    <li><code>"data-variant-source"</code>" = default | tone | variant"</li>
                    <li><code>"data-hide-icon-source / data-icon-label-source / data-motion-source"</code>" = closed enum set"</li>
                    <li><code>"control mode"</code>" = N/A (Alert has no controlled/uncontrolled runtime axis)"</li>
                    <li><code>"disabled axis"</code>" = N/A (Alert has no disabled prop in API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="alert-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="alert-parameter-rows">
                    <li><code>"tone: Option&lt;AlertTone&gt;"</code>" default = None -> normalize to tone=neutral (source=default)"</li>
                    <li><code>"variant: Option&lt;AlertVariant&gt;"</code>" default = None -> source=default (tone mapping only when variant is set)"</li>
                    <li><code>"layout: Option&lt;AlertLayout&gt;"</code>" default = None -> normalize to banner"</li>
                    <li><code>"fill: Option&lt;AlertFill&gt;"</code>" default = None -> normalize to border"</li>
                    <li><code>"is_hide_icon / hide_icon: Option&lt;bool&gt;"</code>" default = None/None -> hide_icon=false (source=default)"</li>
                    <li><code>"title / description / class_name: Option&lt;String&gt;"</code>" default = None; empty/blank strings are normalized away"</li>
                    <li><code>"icon_label: Option&lt;String&gt;"</code>" default = None -> tone-default label when tone has icon, else empty"</li>
                    <li><code>"lang: Option&lt;String&gt;, dir: Option&lt;A11yDirection&gt;"</code>" default = None (inherits locale context)"</li>
                    <li><code>"motion: AlertMotion"</code>" default = AlertMotion::default()"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="alert-streaming-modes">
                <h3>"Streaming / Snapshot"</h3>
                <ul data-slot="alert-streaming-rows">
                    <li><code>"data-ui-streaming"</code>" = optional"</li>
                    <li><code>"data-ui-fallback"</code>" = snapshot"</li>
                    <li><code>"data-ui-state"</code>" = snapshot"</li>
                    <li><code>"data-ui-output-status"</code>" = verified"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="alert-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="alert-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-alert"</code>
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
                    text="use leptos::prelude::*;\nuse ui::{Alert, AlertFill, AlertTone};\n\n<Alert tone=AlertTone::Info fill=AlertFill::Border>\n  \"Install now to keep your workspace secure.\"\n</Alert>".to_string()
                    label="Copy alert starter".to_string()
                    copyable=true
                    class_name="docs-alert-source-copy".to_string()
                />
                <ul data-slot="alert-source-paths">
                    <li><code>"components/alert/src/mod.rs"</code></li>
                    <li><code>"components/alert/src/logic.rs"</code></li>
                    <li><code>"components/alert/src/view.rs"</code></li>
                    <li><code>"components/alert/src/styles.rs"</code></li>
                    <li><code>"components/alert/src/motion.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
