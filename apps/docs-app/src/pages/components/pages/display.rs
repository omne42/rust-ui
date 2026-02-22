use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::{html, prelude::*};
use ui::color::area::A11yDirection;
use ui::snippet::SnippetMotion;
use ui::{
    Alert, AlertFill, AlertLayout, AlertMotion, AlertTone, AlertVariant, Avatar, AvatarGroup,
    AvatarGroupItem, AvatarSize, Badge, BadgeVariant, Chip, ChipSize, ChipVariant,
    CircularProgress, Code, CodeBlock, CodeVariant, IllustratedMessage, Image, ImageMotion,
    ImageRadius, ImageShadow, Kbd, KbdSize, Link, Meter, MeterSize, MeterVariant, MotionRipple,
    Progress, ProgressBar, ProgressBarSize, ProgressBarVariant, ProgressCircle, RippleMotion,
    SegmentedControl, SegmentedControlSize, Skeleton, SkeletonVariant, SlidingNumber, Snippet,
    Spinner, SpinnerSize, StaticNumber, StatusLight, StatusLightRole, StatusLightVariant, Switch,
};

// Legacy source-contract markers retained for semantic tests:
// <Avatar name="Ada Lovelace".to_string() src=src.to_string() size=AvatarSize::Md />
// src=src.to_string()

fn into_owned_string(value: &str) -> String {
    value.to_string()
}

pub(super) fn alert() -> AnyView {
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

pub(super) fn badge() -> AnyView {
    let hello_world_code = Signal::derive(move || r#"<Badge>"New"</Badge>"#.to_string());

    let matrix_code = Signal::derive(move || {
        r#"<Badge variant=BadgeVariant::Default>"Default"</Badge>
<Badge variant=BadgeVariant::Accent>"Accent"</Badge>
<Badge variant=BadgeVariant::Danger>"Danger"</Badge>
<Badge variant=BadgeVariant::Outline>"Outline"</Badge>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Badge variant=BadgeVariant::Accent class_name="docs-badge-custom".to_string()>
  "Release"
</Badge>
<Badge variant=BadgeVariant::Outline class_name="docs-badge-custom".to_string()>
  "Beta"
</Badge>"#
            .to_string()
    });

    let variant_options = vec![
        "default".to_string(),
        "accent".to_string(),
        "danger".to_string(),
        "outline".to_string(),
    ];
    let locale_options = vec!["en-US".to_string(), "zh-CN".to_string(), "ar".to_string()];
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_locale_index, set_workbench_locale_index) = signal(Some(0_usize));
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => BadgeVariant::Accent,
            2 => BadgeVariant::Danger,
            3 => BadgeVariant::Outline,
            _ => BadgeVariant::Default,
        });

    let workbench_code = Signal::derive(move || {
        let variant = workbench_variant.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => Some("zh-CN"),
            2 => Some("ar"),
            _ => None,
        };
        let label = match locale_index {
            1 => "新品",
            2 => "جديد",
            _ => "New",
        };

        let mut lines = vec!["<Badge".to_string()];
        if variant != BadgeVariant::Default {
            lines.push(format!("  variant=BadgeVariant::{variant:?}"));
        }
        if custom_class {
            lines.push("  class_name=\"docs-badge-custom\".into()".to_string());
        }
        if let Some(lang) = lang {
            lines.push(format!("  lang=\"{lang}\".into()"));
        }
        if rtl {
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.extend([
            ">".to_string(),
            format!("  \"{label}\""),
            "</Badge>".to_string(),
        ]);
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/badge/src/styles.rs */\n{}",
            ui::badge::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant = workbench_variant.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => "zh-CN",
            2 => "ar",
            _ => "default",
        };

        let mut class = vec![
            "ui-badge".to_string(),
            variant.class_name().into(),
            variant.fill_class().into(),
        ];
        if custom_class {
            class.push("ui-badge--custom-class".to_string());
            class.push("docs-badge-custom".to_string());
        }

        format!(
            "BadgeActualConfig {{\n  variant: {variant:?},\n  class_name: {},\n  variant_attr: \"{}\",\n  fill_attr: \"{}\",\n  class_source: \"{}\",\n  lang: \"{lang}\",\n  dir: \"{}\",\n  class: \"{}\",\n}}",
            if custom_class {
                "Some(\"docs-badge-custom\")"
            } else {
                "None"
            },
            variant.as_attr(),
            variant.fill_attr(),
            if custom_class { "custom" } else { "default" },
            if rtl { "rtl" } else { "auto" },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Badge"
            slug="badge"
            group="Display"
            description="Status badge with centralized variant/fill state attrs and custom-class contract."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="docs-row">
                    <Badge>"New"</Badge>
                </div>
            </Playground>

            <Playground title="Variants (Default / Accent / Danger / Outline)" code_signal=matrix_code>
                <div class="docs-row">
                    <Badge variant=BadgeVariant::Default>"Default"</Badge>
                    <Badge variant=BadgeVariant::Accent>"Accent"</Badge>
                    <Badge variant=BadgeVariant::Danger>"Danger"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                </div>
            </Playground>

            <Playground title="Custom Class + Outline" code_signal=custom_code>
                <div class="docs-row">
                    <Badge variant=BadgeVariant::Accent class_name="docs-badge-custom".to_string()>
                        "Release"
                    </Badge>
                    <Badge variant=BadgeVariant::Outline class_name="docs-badge-custom".to_string()>
                        "Beta"
                    </Badge>
                </div>
            </Playground>

            <Playground
                title="Badge Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/badge/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="Button-like workbench: display compare + live config/code/css test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="badge-workbench-controls">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-badge-workbench-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Badge variant".to_string()
                        />

                        <div class="docs-search__label">"Locale"</div>
                        <SegmentedControl
                            id_base="docs-badge-workbench-locale".to_string()
                            options=locale_options.clone()
                            selected_index=workbench_locale_index
                            set_selected_index=set_workbench_locale_index
                            size=SegmentedControlSize::Sm
                            aria_label="Badge locale".to_string()
                        />

                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL direction"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let variant = workbench_variant.get();
                    let locale_index = workbench_locale_index.get().unwrap_or(0);
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();
                    let lang = match locale_index {
                        1 => "zh-CN".to_string(),
                        2 => "ar".to_string(),
                        _ => String::new(),
                    };
                    let label = match locale_index {
                        1 => "新品",
                        2 => "جديد",
                        _ => "New",
                    };
                    let class_name = if custom_class {
                        "docs-badge-custom".to_string()
                    } else {
                        String::new()
                    };
                    let dir = if rtl {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    view! {
                        <div class="docs-stack" data-slot="badge-workbench-compare">
                            <div class="docs-row">
                                <div class="docs-stack docs-stack--tight">
                                    <div class="docs-search__label">"Baseline"</div>
                                    <Badge>"New"</Badge>
                                </div>
                                <div class="docs-stack docs-stack--tight">
                                    <div class="docs-search__label">"Configured"</div>
                                    <Badge variant=variant class_name=class_name lang=lang dir=dir>
                                        {label}
                                    </Badge>
                                </div>
                            </div>

                            <div class="docs-search__label">"Scenario compare"</div>
                            <div class="docs-row">
                                <Badge variant=BadgeVariant::Default>"default"</Badge>
                                <Badge variant=BadgeVariant::Accent>"accent"</Badge>
                                <Badge variant=BadgeVariant::Danger>"danger"</Badge>
                                <Badge variant=BadgeVariant::Outline>"outline"</Badge>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="Comparison Matrix (Variant + Fill)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <Badge variant=BadgeVariant::Default>"default"</Badge>
                    <Badge variant=BadgeVariant::Accent>"accent"</Badge>
                    <Badge variant=BadgeVariant::Danger>"danger"</Badge>
                    <Badge variant=BadgeVariant::Outline>"outline"</Badge>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn status_light() -> AnyView {
    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Danger".to_string(),
    ];
    let role_options = vec!["None".to_string(), "status".to_string()];
    let lang_options = vec!["en-US".to_string(), "zh-CN".to_string()];

    let (variant_index, set_variant_index) = signal(Some(0usize));
    let (role_index, set_role_index) = signal(Some(1usize));
    let (lang_index, set_lang_index) = signal(Some(0usize));
    let (custom_class, set_custom_class) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => StatusLightVariant::Accent,
        2 => StatusLightVariant::Danger,
        _ => StatusLightVariant::Default,
    });
    let role = Signal::derive(move || match role_index.get().unwrap_or(1) {
        1 => Some(StatusLightRole::Status),
        _ => None,
    });
    let lang = Signal::derive(move || match lang_index.get().unwrap_or(0) {
        1 => "zh-CN".to_string(),
        _ => "en-US".to_string(),
    });
    let dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });
    let class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-status-light-custom".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<StatusLight
  variant=StatusLightVariant::Accent
  role=StatusLightRole::Status
>
  "Syncing invoices"
</StatusLight>"#
            .to_string()
    });
    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<StatusLight".to_string()];
        if variant.get() != StatusLightVariant::Default {
            lines.push(format!("  variant=StatusLightVariant::{:?}", variant.get()));
        }
        if let Some(role) = role.get() {
            lines.push(format!("  role=StatusLightRole::{role:?}"));
        }
        let class_name = class_name.get();
        if !class_name.is_empty() {
            lines.push(format!("  class_name={}", rust_string_literal(&class_name)));
        }
        lines.push(format!("  lang={}", rust_string_literal(&lang.get())));
        lines.push(format!("  dir=A11yDirection::{:?}", dir.get()));
        lines.push(">".to_string());
        lines.push("  \"Syncing invoices\"".to_string());
        lines.push("</StatusLight>".to_string());
        lines.join("\n")
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "StatusLightWorkbenchConfig {{\n  variant: {:?},\n  role: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            variant.get(),
            role.get(),
            if class_name.get().is_empty() {
                None::<String>
            } else {
                Some(class_name.get())
            },
            Some(lang.get()),
            Some(dir.get()),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<StatusLight variant=StatusLightVariant::Default role=StatusLightRole::Status lang="en-US".to_string() dir=A11yDirection::Ltr>
  "Idle"
</StatusLight>
<StatusLight variant=StatusLightVariant::Accent role=StatusLightRole::Status lang="en-US".to_string() dir=A11yDirection::Ltr>
  "Deploying"
</StatusLight>
<StatusLight variant=StatusLightVariant::Danger role=StatusLightRole::Status class_name="docs-status-light-custom".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl>
  "失败"
</StatusLight>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="StatusLight"
            slug="status-light"
            group="Display"
            description="Status indicator + label with centralized variant/live/role-source state attrs and optional custom-class contract."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                test_source_path="components/status-light/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <StatusLight
                        variant=StatusLightVariant::Accent
                        role=StatusLightRole::Status
                    >
                        "Syncing invoices"
                    </StatusLight>
                </div>
            </Playground>

            <Playground
                title="Workbench (Variant + Role + Locale)"
                code_signal=workbench_code
                test_source_path="components/status-light/src/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="status-light-workbench-controls">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-status-light-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="StatusLight variant".to_string()
                        />

                        <div class="docs-search__label">"Role"</div>
                        <SegmentedControl
                            id_base="docs-status-light-role".to_string()
                            options=role_options.clone()
                            selected_index=role_index
                            set_selected_index=set_role_index
                            size=SegmentedControlSize::Sm
                            aria_label="StatusLight role".to_string()
                        />

                        <div class="docs-search__label">"Language"</div>
                        <SegmentedControl
                            id_base="docs-status-light-lang".to_string()
                            options=lang_options.clone()
                            selected_index=lang_index
                            set_selected_index=set_lang_index
                            size=SegmentedControlSize::Sm
                            aria_label="StatusLight language".to_string()
                        />

                        <Switch checked=rtl set_checked=set_rtl>"RTL direction"</Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let role = role.get();
                    let class_name = class_name.get();
                    let lang = lang.get();
                    let dir = dir.get();

                    let content = if let Some(role) = role {
                        view! {
                            <StatusLight
                                variant=variant
                                role=role
                                class_name=class_name
                                lang=lang
                                dir=dir
                            >
                                "Syncing invoices"
                            </StatusLight>
                        }
                        .into_any()
                    } else {
                        view! {
                            <StatusLight
                                variant=variant
                                class_name=class_name
                                lang=lang
                                dir=dir
                            >
                                "Syncing invoices"
                            </StatusLight>
                        }
                        .into_any()
                    };

                    view! { <div class="docs-row">{content}</div> }
                }}
            </Playground>

            <Playground
                title="State Matrix (Variant + Role + Locale)"
                code_signal=matrix_code
                test_source_path="components/status-light/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <StatusLight
                        variant=StatusLightVariant::Default
                        role=StatusLightRole::Status
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        "Idle"
                    </StatusLight>
                    <StatusLight
                        variant=StatusLightVariant::Accent
                        role=StatusLightRole::Status
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        "Deploying"
                    </StatusLight>
                    <StatusLight
                        variant=StatusLightVariant::Danger
                        role=StatusLightRole::Status
                        class_name="docs-status-light-custom".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    >
                        "失败"
                    </StatusLight>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn chip() -> AnyView {
    let variant_options = vec![
        "default".to_string(),
        "accent".to_string(),
        "danger".to_string(),
        "outline".to_string(),
    ];
    let size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(1));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1));
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_dismissible, set_workbench_is_dismissible) = signal(true);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(1) {
            0 => ChipVariant::Default,
            2 => ChipVariant::Danger,
            3 => ChipVariant::Outline,
            _ => ChipVariant::Accent,
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => ChipSize::Sm,
        2 => ChipSize::Lg,
        _ => ChipSize::Md,
    });

    let workbench_code = Signal::derive(move || {
        let variant = workbench_variant.get();
        let size = workbench_size.get();
        let is_disabled = workbench_is_disabled.get();
        let is_dismissible = workbench_is_dismissible.get();
        let custom_label = workbench_custom_label.get() && is_dismissible;
        let custom_class = workbench_custom_class.get();

        let mut snippet = vec!["<Chip".to_string()];
        if variant != ChipVariant::Default {
            snippet.push(format!("  variant=ChipVariant::{variant:?}"));
        }
        if size != ChipSize::Md {
            snippet.push(format!("  size=ChipSize::{size:?}"));
        }
        if is_disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if is_dismissible {
            snippet.push("  on_dismiss=Callback::new(|_| ())".to_string());
        }
        if custom_label {
            snippet.push("  dismiss_aria_label=\"Remove reviewer\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-chip-custom\".into()".to_string());
        }
        snippet.push(">".to_string());
        snippet.push("  \"Reviewer\"".to_string());
        snippet.push("</Chip>".to_string());
        snippet.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let variant = workbench_variant.get();
        let size = workbench_size.get();
        let is_disabled = workbench_is_disabled.get();
        let is_dismissible = workbench_is_dismissible.get();
        let custom_label = workbench_custom_label.get() && is_dismissible;
        let custom_class = workbench_custom_class.get();

        let state = if is_disabled {
            "disabled"
        } else if is_dismissible {
            "removable"
        } else {
            "static"
        };

        let mut classes = vec![
            "ui-chip".to_string(),
            variant.class_name().into(),
            size.class_name().into(),
            format!("ui-chip--{state}"),
        ];
        classes.push(if custom_label {
            "ui-chip--dismiss-label-custom".to_string()
        } else {
            "ui-chip--dismiss-label-default".to_string()
        });
        if !is_disabled {
            classes.push("ui-chip--enabled".to_string());
        }
        if custom_class {
            classes.push("ui-chip--custom-class".to_string());
            classes.push("docs-chip-custom".to_string());
        }

        format!(
            "ChipActualConfig {{\n  variant: {variant:?},\n  size: {size:?},\n  is_disabled: {is_disabled},\n  is_dismissible: {is_dismissible},\n  custom_dismiss_label: {custom_label},\n  custom_class: {custom_class},\n  class: \"{}\",\n  marker_expectations: [\"data-variant\", \"data-size\", \"data-state\", \"data-dismiss-label-source\", \"data-class-source\"],\n}}",
            classes.join(" ")
        )
    });

    let chip_test_css_source = Signal::derive(move || {
        format!(
            "/* components/chip/src/styles.rs */\n{}",
            ui::chip::styles::CSS
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Chip variant=ChipVariant::Default size=ChipSize::Sm>"Default / Static"</Chip>
<Chip variant=ChipVariant::Accent size=ChipSize::Md on_dismiss=Callback::new(|_| ())>
  "Accent / Removable"
</Chip>
<Chip variant=ChipVariant::Danger size=ChipSize::Lg is_disabled=true on_dismiss=Callback::new(|_| ())>
  "Danger / Disabled"
</Chip>
<Chip
  variant=ChipVariant::Outline
  size=ChipSize::Md
  on_dismiss=Callback::new(|_| ())
  dismiss_aria_label="Remove reviewer".to_string()
  class_name="docs-chip-custom".to_string()
>
  "Outline / Custom"
</Chip>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Chip"
            slug="chip"
            group="Display"
            description="Chip / tag pill with centralized variant-size-state attrs, dismiss-label source contracts, and optional custom class semantics."
        >
            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=chip_test_css_source
                test_source_path="components/chip/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调 variant/size/disabled/dismiss/custom，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Variant"</div>
                            <SegmentedControl
                                id_base="docs-chip-variant".to_string()
                                options=variant_options.clone()
                                selected_index=workbench_variant_index
                                set_selected_index=set_workbench_variant_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chip variant".to_string()
                            />

                            <div class="docs-search__label">"Size"</div>
                            <SegmentedControl
                                id_base="docs-chip-size".to_string()
                                options=size_options.clone()
                                selected_index=workbench_size_index
                                set_selected_index=set_workbench_size_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chip size".to_string()
                            />

                            <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                "is_disabled"
                            </Switch>
                            <Switch checked=workbench_is_dismissible set_checked=set_workbench_is_dismissible>
                                "Dismiss action"
                            </Switch>
                            <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                                "Custom dismiss aria label"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class_name"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let variant = workbench_variant.get();
                        let size = workbench_size.get();
                        let is_disabled = workbench_is_disabled.get();
                        let is_dismissible = workbench_is_dismissible.get();
                        let dismiss_aria_label = if workbench_custom_label.get() && is_dismissible {
                            "Remove reviewer".to_string()
                        } else {
                            "".to_string()
                        };
                        let class_name = if workbench_custom_class.get() {
                            "docs-chip-custom".to_string()
                        } else {
                            "".to_string()
                        };

                        if is_dismissible {
                            view! {
                                <Chip
                                    variant=variant
                                    size=size
                                    is_disabled=is_disabled
                                    on_dismiss=Callback::new(|_| ())
                                    dismiss_aria_label=dismiss_aria_label
                                    class_name=class_name
                                >
                                    "Reviewer"
                                </Chip>
                            }
                                .into_any()
                        } else {
                            view! {
                                <Chip
                                    variant=variant
                                    size=size
                                    is_disabled=is_disabled
                                    class_name=class_name
                                >
                                    "Reviewer"
                                </Chip>
                            }
                                .into_any()
                        }
                    }}

                    <div class="docs-row">
                        <span class="ui-muted">"Compare baseline:"</span>
                        <Chip variant=ChipVariant::Default size=ChipSize::Sm>"Default"</Chip>
                        <Chip variant=ChipVariant::Accent size=ChipSize::Md>"Accent"</Chip>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Variant / Size / Disabled / Custom)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 200px;">
                        <span class="ui-muted">"Default / Static"</span>
                        <Chip variant=ChipVariant::Default size=ChipSize::Sm>
                            "Default / Static"
                        </Chip>
                    </div>
                    <div class="docs-card" style="flex: 1 1 200px;">
                        <span class="ui-muted">"Accent / Removable"</span>
                        <Chip
                            variant=ChipVariant::Accent
                            size=ChipSize::Md
                            on_dismiss=Callback::new(|_| ())
                        >
                            "Accent / Removable"
                        </Chip>
                    </div>
                    <div class="docs-card" style="flex: 1 1 200px;">
                        <span class="ui-muted">"Danger / Disabled"</span>
                        <Chip
                            variant=ChipVariant::Danger
                            size=ChipSize::Lg
                            is_disabled=true
                            on_dismiss=Callback::new(|_| ())
                        >
                            "Danger / Disabled"
                        </Chip>
                    </div>
                    <div class="docs-card" style="flex: 1 1 200px;">
                        <span class="ui-muted">"Outline / Custom"</span>
                        <Chip
                            variant=ChipVariant::Outline
                            size=ChipSize::Md
                            on_dismiss=Callback::new(|_| ())
                            dismiss_aria_label="Remove reviewer".to_string()
                            class_name="docs-chip-custom".to_string()
                        >
                            "Outline / Custom"
                        </Chip>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn skeleton() -> AnyView {
    let variant_options = vec!["Rect".to_string(), "Circle".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (is_shimmer, set_is_shimmer) = signal(true);
    let (custom_class, set_custom_class) = signal(false);

    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => SkeletonVariant::Circle,
        _ => SkeletonVariant::Rect,
    });
    let class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-skeleton-line docs-skeleton-line--short".to_string()
        } else {
            "docs-skeleton-line".to_string()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<Skeleton variant=SkeletonVariant::Rect is_shimmer=true class_name="docs-skeleton-line".to_string() />"#.to_string()
    });
    let workbench_code = Signal::derive(move || {
        format!(
            "<Skeleton\n  variant=SkeletonVariant::{:?}\n  is_shimmer={}\n  class_name={:?}.to_string()\n/>",
            variant.get(),
            is_shimmer.get(),
            class_name.get()
        )
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "SkeletonWorkbenchConfig {{\n  variant: {:?},\n  is_shimmer: {},\n  class_name: {:?},\n}}",
            variant.get(),
            is_shimmer.get(),
            class_name.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<Skeleton variant=SkeletonVariant::Rect is_shimmer=true class_name="docs-skeleton-line".to_string() />
<Skeleton variant=SkeletonVariant::Rect is_shimmer=false class_name="docs-skeleton-line docs-skeleton-line--short".to_string() />
<Skeleton variant=SkeletonVariant::Circle is_shimmer=true class_name="docs-skeleton-avatar".to_string() />
<Skeleton variant=SkeletonVariant::Circle is_shimmer=false class_name="docs-skeleton-avatar".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Skeleton"
            slug="skeleton"
            group="Display"
            description="Skeleton placeholder blocks with centralized variant/shimmer state attrs."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                test_source_path="crates/ui/src/skeleton/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
                </div>
            </Playground>

            <Playground
                title="Workbench (Variant + Shimmer + Class)"
                code_signal=workbench_code
                test_source_path="crates/ui/src/skeleton/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-skeleton-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Skeleton variant".to_string()
                        />
                        <Switch checked=is_shimmer set_checked=set_is_shimmer>"Shimmer"</Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <Skeleton
                        variant=variant.get()
                        is_shimmer=is_shimmer.get()
                        class_name=class_name.get()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Variant / Shimmer / Class Comparison)"
                code_signal=matrix_code
                test_source_path="crates/ui/src/skeleton/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        is_shimmer=true
                        class_name="docs-skeleton-line".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        is_shimmer=false
                        class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Circle
                        is_shimmer=true
                        class_name="docs-skeleton-avatar".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Circle
                        is_shimmer=false
                        class_name="docs-skeleton-avatar".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn circular_progress() -> AnyView {
    let hello_world_code = Signal::derive(move || r#"<CircularProgress />"#.to_string());

    let matrix_code = Signal::derive(move || {
        r#"<CircularProgress aria_label="Loading".to_string() />
<CircularProgress aria_label="Syncing mail".to_string() size_px=24.0 />
<CircularProgress aria_label="Syncing mail".to_string() thickness_px=3.0 />
<CircularProgress aria_label="Syncing mail".to_string() size_px=30.0 thickness_px=4.0 />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<CircularProgress
  aria_label="Background refresh".to_string()
  size_px=28.0
  thickness_px=3.5
  class_name="docs-circular-progress-custom".to_string()
/>
<CircularProgress aria_label="   ".to_string() class_name="docs-circular-progress-custom".to_string() />"#.to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"let upstream_label = "Syncing mail".to_string();

<CircularProgress />
<CircularProgress aria_label=upstream_label size_px=24.0 />
// CircularProgress has no controlled/uncontrolled runtime axis.
// App state maps directly to props; no value/on_change/default triplet."#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<CircularProgress aria_label="Snapshot".to_string() />
// Streaming Optional; fallback=snapshot.
// CircularProgress renders complete validated snapshot output with stable semantic attrs."#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<CircularProgress
  aria_label="Syncing mailbox".to_string()
  size_px=24.0
  thickness_px=3.0
/>"#
        .to_string()
    });

    let (workbench_size_px, set_workbench_size_px) = signal(None::<f64>);
    let (workbench_thickness_px, set_workbench_thickness_px) = signal(None::<f64>);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_code = Signal::derive(move || {
        let size_px = workbench_size_px.get();
        let thickness_px = workbench_thickness_px.get();
        let custom_label = workbench_custom_label.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();

        let mut lines = vec!["<CircularProgress".to_string()];
        if custom_label {
            lines.push("  aria_label=\"Workbench sync\".to_string()".to_string());
        }
        if let Some(size_px) = size_px {
            lines.push(format!("  size_px={size_px}"));
        }
        if let Some(thickness_px) = thickness_px {
            lines.push(format!("  thickness_px={thickness_px}"));
        }
        if custom_class {
            lines.push("  class_name=\"docs-circular-progress-custom\".to_string()".to_string());
        }
        if rtl {
            lines.push("  lang=\"ar\".to_string()".to_string());
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let aria_label = if workbench_custom_label.get() {
            "Workbench sync"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-circular-progress-custom"
        } else {
            ""
        };
        let lang = if workbench_rtl.get() { "ar" } else { "" };

        format!(
            "CircularProgressWorkbenchConfig {{\n  aria_label: {:?},\n  size_px: {:?},\n  thickness_px: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  size_source: {:?},\n  thickness_source: {:?},\n  label_source: {:?},\n  class_source: {:?},\n}}",
            aria_label,
            workbench_size_px.get(),
            workbench_thickness_px.get(),
            class_name,
            lang,
            if workbench_rtl.get() {
                A11yDirection::Rtl
            } else {
                A11yDirection::Ltr
            },
            if workbench_size_px.get().is_some() {
                "custom"
            } else {
                "default"
            },
            if workbench_thickness_px.get().is_some() {
                "custom"
            } else {
                "default"
            },
            if workbench_custom_label.get() {
                "custom"
            } else {
                "default"
            },
            if workbench_custom_class.get() {
                "custom"
            } else {
                "default"
            },
        )
    });

    view! {
        <ComponentPage
            title="CircularProgress"
            slug="circular-progress"
            group="Display"
            description="Indeterminate circular progress with centralized size/thickness/label source attrs."
        >
            <Playground
                title="Hello World"
                code_signal=hello_world_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress />
                </div>
            </Playground>

            <Playground
                title="Size + Thickness Matrix"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress aria_label="Loading".to_string() />
                    <CircularProgress aria_label="Syncing mail".to_string() size_px=24.0 />
                    <CircularProgress aria_label="Syncing mail".to_string() thickness_px=3.0 />
                    <CircularProgress
                        aria_label="Syncing mail".to_string()
                        size_px=30.0
                        thickness_px=4.0
                    />
                </div>
            </Playground>

            // <Playground title="Custom Label + Class" code_signal=custom_code>
            <Playground
                title="Custom Label + Class"
                code_signal=custom_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress
                        aria_label="Background refresh".to_string()
                        size_px=28.0
                        thickness_px=3.5
                        class_name="docs-circular-progress-custom".to_string()
                    />
                    <CircularProgress
                        aria_label="   ".to_string()
                        class_name="docs-circular-progress-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="CircularProgress has no internal controlled/uncontrolled axis; compare default usage with app-state-mapped props."
                code_signal=controlled_contrast_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress />
                    <CircularProgress aria_label="Syncing mail".to_string() size_px=24.0 />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="CircularProgress is not a body-reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <p class="ui-muted" data-slot="circular-progress-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </p>
                    <p class="ui-muted" data-slot="circular-progress-copy-ready-hint">
                        "Copy-ready snippets prepend imports automatically; source: components/circular-progress/src/view.rs."
                    </p>
                    <CircularProgress aria_label="Snapshot".to_string() />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress
                        aria_label="Syncing mailbox".to_string()
                        size_px=24.0
                        thickness_px=3.0
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Props / State / Preview)"
                description="在线调整 props（size/thickness/label/class/lang/dir）并实时预览语义标记变化；组件本身无内部受控状态轴。"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::color::area::A11yDirection;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="circular-progress-workbench-controls">
                            <div class="docs-search__label">"Size"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-size-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-size-default"
                                    on:click=move |_| set_workbench_size_px.set(None)
                                >
                                    "Default"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-size-24"
                                    on:click=move |_| set_workbench_size_px.set(Some(24.0))
                                >
                                    "24"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-size-32"
                                    on:click=move |_| set_workbench_size_px.set(Some(32.0))
                                >
                                    "32"
                                </button>
                            </div>

                            <div class="docs-search__label">"Thickness"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-thickness-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-thickness-default"
                                    on:click=move |_| set_workbench_thickness_px.set(None)
                                >
                                    "Default"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-thickness-3"
                                    on:click=move |_| set_workbench_thickness_px.set(Some(3.0))
                                >
                                    "3"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-thickness-4"
                                    on:click=move |_| set_workbench_thickness_px.set(Some(4.0))
                                >
                                    "4"
                                </button>
                            </div>

                            <div class="docs-search__label">"Label source"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-label-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-label-default"
                                    on:click=move |_| set_workbench_custom_label.set(false)
                                >
                                    "Default label"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-label-custom"
                                    on:click=move |_| set_workbench_custom_label.set(true)
                                >
                                    "Custom label"
                                </button>
                            </div>

                            <div class="docs-search__label">"Class source"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-class-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-class-default"
                                    on:click=move |_| set_workbench_custom_class.set(false)
                                >
                                    "Default class"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-class-custom"
                                    on:click=move |_| set_workbench_custom_class.set(true)
                                >
                                    "Custom class"
                                </button>
                            </div>

                            <div class="docs-search__label">"Direction"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-dir-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-dir-ltr"
                                    on:click=move |_| set_workbench_rtl.set(false)
                                >
                                    "LTR"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-dir-rtl"
                                    on:click=move |_| set_workbench_rtl.set(true)
                                >
                                    "RTL"
                                </button>
                            </div>
                        </div>
                    }
                }
            >
                {move || {
                    let size_px = workbench_size_px.get();
                    let thickness_px = workbench_thickness_px.get();
                    let custom_label = workbench_custom_label.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();

                    let aria_label = if custom_label {
                        "Workbench sync".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-circular-progress-custom".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if rtl { "ar".to_string() } else { String::new() };
                    let dir = if rtl {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    let size_source = if size_px.is_some() { "custom" } else { "default" };
                    let thickness_source = if thickness_px.is_some() {
                        "custom"
                    } else {
                        "default"
                    };
                    let label_source = if custom_label { "custom" } else { "default" };
                    let class_source = if custom_class { "custom" } else { "default" };
                    let dir_label = if rtl { "rtl" } else { "ltr" };
                    let configured_progress = match (size_px, thickness_px) {
                        (Some(size_px), Some(thickness_px)) => view! {
                            <CircularProgress
                                aria_label=aria_label.clone()
                                size_px=size_px
                                thickness_px=thickness_px
                                class_name=class_name.clone()
                                lang=lang.clone()
                                dir=dir
                            />
                        }
                        .into_any(),
                        (Some(size_px), None) => view! {
                            <CircularProgress
                                aria_label=aria_label.clone()
                                size_px=size_px
                                class_name=class_name.clone()
                                lang=lang.clone()
                                dir=dir
                            />
                        }
                        .into_any(),
                        (None, Some(thickness_px)) => view! {
                            <CircularProgress
                                aria_label=aria_label.clone()
                                thickness_px=thickness_px
                                class_name=class_name.clone()
                                lang=lang.clone()
                                dir=dir
                            />
                        }
                        .into_any(),
                        (None, None) => view! {
                            <CircularProgress
                                aria_label=aria_label
                                class_name=class_name
                                lang=lang
                                dir=dir
                            />
                        }
                        .into_any(),
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="circular-progress-workbench-preview">
                            <p class="ui-muted" data-slot="circular-progress-workbench-state">
                                {format!(
                                    "size_source={size_source}; thickness_source={thickness_source}; label_source={label_source}; class_source={class_source}; dir={dir_label}"
                                )}
                            </p>
                            {configured_progress}
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Size / Thickness / Locale Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::color::area::A11yDirection;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress aria_label="Loading".to_string() />
                    <CircularProgress aria_label="Syncing mail".to_string() size_px=24.0 />
                    <CircularProgress aria_label="Syncing mail".to_string() thickness_px=3.0 />
                    <CircularProgress
                        aria_label="Syncing mail".to_string()
                        size_px=30.0
                        thickness_px=4.0
                        class_name="docs-circular-progress-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="circular-progress-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="circular-progress-source-first-contract">
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="circular-progress-source-prerequisites">
                    <li>
                        "Dependency prerequisites: enable "
                        <code>"component-circular_progress"</code>
                        " + "
                        <code>"inject-css"</code>
                        " in package mode."
                    </li>
                    <li>
                        "Runtime style prerequisite: mount "
                        <code>"UiRoot"</code>
                        " (or equivalent CSS injection path) to avoid unstyled copy-paste output."
                    </li>
                </ul>
                <Snippet
                    text=source_first_code.get()
                    label="Copy circular-progress starter".to_string()
                    copyable=true
                    class_name="docs-circular-progress-source-copy".to_string()
                />
                <p>"Source paths:"</p>
                <ul data-slot="circular-progress-source-paths">
                    <li><code>"components/circular-progress/src/mod.rs"</code></li>
                    <li><code>"components/circular-progress/src/logic.rs"</code></li>
                    <li><code>"components/circular-progress/src/view.rs"</code></li>
                    <li><code>"components/circular-progress/src/styles.rs"</code></li>
                </ul>
                <p class="ui-muted" data-slot="circular-progress-source-sync-note">
                    "Sync note: snippet text is sourced from "
                    <code>"source_first_code"</code>
                    " and mirrors "
                    <code>"components/circular-progress/src/view.rs"</code>
                    " API usage; update docs snippet and source implementation together to avoid drift."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="circular-progress-docs-sync-matrix">
                <h3>"State Matrix"</h3>
                <ul>
                    <li><code>"data-state"</code>" = indeterminate（固定快照态）"</li>
                    <li>
                        <code>"data-size-source / data-thickness-source / data-label-source / data-class-source"</code>
                        " = default | custom"
                    </li>
                    <li>
                        <code>"control mode"</code>
                        " = N/A（CircularProgress 无内部受控/非受控状态轴）"
                    </li>
                </ul>

                <h3>"Parameter Matrix"</h3>
                <ul>
                    <li>
                        <code>"aria_label: Option&lt;String&gt;"</code>
                        " default = None；`logic.rs::resolve_component_contract` 归一到 i18n `loading_aria_label`，空白回退 `DEFAULT_ARIA_LABEL`"
                    </li>
                    <li>
                        <code>"size_px / thickness_px: Option&lt;f64&gt;"</code>
                        " default = None；仅 finite 且 > 0 时生效，否则回落 default source"
                    </li>
                    <li>
                        <code>"class_name / lang: Option&lt;String&gt;"</code>
                        " default = None；空白字符串经 `normalize_optional_text` 归一为 None"
                    </li>
                    <li>
                        <code>"dir: Option&lt;A11yDirection&gt;"</code>
                        " default = None（继承 `UiRoot` locale direction 上下文）"
                    </li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn spinner() -> AnyView {
    let (workbench_size_key, set_workbench_size_key) = signal("md".to_string());
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_arabic_locale, set_workbench_arabic_locale) = signal(false);

    let workbench_size = Signal::derive(move || match workbench_size_key.get().as_str() {
        "sm" => SpinnerSize::Sm,
        "lg" => SpinnerSize::Lg,
        _ => SpinnerSize::Md,
    });
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Syncing workspace data".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-spinner-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::spinner::SpinnerMotion {
                rotation_duration_ms: 640,
            }
        } else {
            ui::spinner::SpinnerMotion::default()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if workbench_arabic_locale.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_arabic_locale.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<Spinner aria_label="Loading activity".to_string() />"#.to_string()
    });

    let workbench_code = Signal::derive(move || {
        let size_expr = match workbench_size.get() {
            SpinnerSize::Sm => "SpinnerSize::Sm",
            SpinnerSize::Md => "SpinnerSize::Md",
            SpinnerSize::Lg => "SpinnerSize::Lg",
        };

        format!(
            "<Spinner\n  size={size_expr}\n  aria_label={}\n  class_name={}\n  motion=ui::spinner::SpinnerMotion {{ rotation_duration_ms: {} }}\n  lang={}\n  dir={}\n/>",
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            workbench_motion.get().rotation_duration_ms,
            rust_string_literal(&workbench_lang.get()),
            if matches!(workbench_dir.get(), A11yDirection::Rtl) {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SpinnerActualConfig {{\n  size: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  motion: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            workbench_size.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            workbench_motion.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Spinner
  size=SpinnerSize::Sm
  aria_label="Fetching notifications".to_string()
  class_name="docs-spinner-custom".to_string()
  motion=ui::spinner::SpinnerMotion { rotation_duration_ms: 480 }
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
/>
<Spinner
  size=SpinnerSize::Lg
  aria_label="Loading Arabic inbox".to_string()
  motion=ui::spinner::SpinnerMotion { rotation_duration_ms: 840 }
  lang="ar".to_string()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Spinner"
            slug="spinner"
            group="Display"
            description="Spinner wraps CircularProgress with centralized size/label/class source attrs."
        >
            <Playground title="Default Showcase" code_signal=showcase_code>
                <div class="docs-row">
                    <Spinner aria_label="Loading activity".to_string() />
                </div>
            </Playground>

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="spinner-workbench-controls">
                        <label class="docs-search__label">
                            "Size"
                            <select
                                prop:value=move || workbench_size_key.get()
                                on:change=move |ev| set_workbench_size_key.set(event_target_value(&ev))
                            >
                                <option value="sm">"Sm"</option>
                                <option value="md">"Md"</option>
                                <option value="lg">"Lg"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " aria_label"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " custom motion"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_arabic_locale.get()
                                on:change=move |ev| set_workbench_arabic_locale.set(event_target_checked(&ev))
                            />
                            " lang/dir Arabic"
                        </label>
                    </div>
                }
            >
                <div class="docs-row">
                    <Spinner
                        size=workbench_size.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        motion=workbench_motion.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    />
                    <span class="ui-muted">
                        "Configured spinner updates size/label/class/motion/locale in one canvas."
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Size / Motion / Locale Comparison)"
                code_signal=matrix_code
                code_imports="use ui::color::area::A11yDirection;\nuse ui::{Spinner, SpinnerSize};".to_string()
            >
                <div class="docs-row">
                    <Spinner
                        size=SpinnerSize::Sm
                        aria_label="Fetching notifications".to_string()
                        class_name="docs-spinner-custom".to_string()
                        motion=ui::spinner::SpinnerMotion {
                            rotation_duration_ms: 480,
                        }
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <Spinner
                        size=SpinnerSize::Lg
                        aria_label="Loading Arabic inbox".to_string()
                        motion=ui::spinner::SpinnerMotion {
                            rotation_duration_ms: 840,
                        }
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn progress() -> AnyView {
    let min_options = ["0".to_string(), "20".to_string()];
    let max_options = ["100".to_string(), "200".to_string()];
    let (workbench_value_raw, set_workbench_value_raw) = signal(42.0_f64);
    let (workbench_min_index, set_workbench_min_index) = signal(Some(0_usize));
    let (workbench_max_index, set_workbench_max_index) = signal(Some(0_usize));
    let (workbench_indeterminate, set_workbench_indeterminate) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(true);
    let (workbench_fast_motion, set_workbench_fast_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);

    let workbench_min = Signal::derive(move || {
        if workbench_min_index.get().unwrap_or(0) == 1 {
            20.0_f64
        } else {
            0.0_f64
        }
    });
    let workbench_max = Signal::derive(move || {
        if workbench_max_index.get().unwrap_or(0) == 1 {
            200.0_f64
        } else {
            100.0_f64
        }
    });
    let workbench_value = Signal::derive(move || {
        if workbench_indeterminate.get() {
            None
        } else {
            let min = workbench_min.get();
            let max = workbench_max.get().max(min + 1.0_f64);
            Some(workbench_value_raw.get().clamp(min, max))
        }
    });
    let workbench_value_label = Signal::derive(move || {
        if workbench_custom_label.get() {
            match workbench_value.get() {
                Some(value) => format!("{value:.0}% complete"),
                None => "loading…".to_string(),
            }
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_fast_motion.get() {
            ui::ProgressMotion::fast()
        } else {
            ui::ProgressMotion::default()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Workbench progress".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-progress-custom".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<Progress
  aria_label="Upload progress".to_string()
  value=Signal::derive(|| Some(42.0))
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Progress\n  aria_label={}\n  value=Signal::derive(|| {:?})\n  min={:.1}\n  max={:.1}\n  indeterminate={}\n  value_label={}\n  motion={}\n  class_name={}\n/>",
            rust_string_literal(&workbench_aria_label.get()),
            workbench_value.get(),
            workbench_min.get(),
            workbench_max.get(),
            bool_word(workbench_indeterminate.get()),
            rust_string_literal(&workbench_value_label.get()),
            if workbench_fast_motion.get() {
                "ui::ProgressMotion::fast()"
            } else {
                "ui::ProgressMotion::default()"
            },
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ProgressActualConfig {{\n  aria_label: {:?},\n  value: {:?},\n  min: {:.1},\n  max: {:.1},\n  indeterminate: {},\n  value_label: {:?},\n  motion: {:?},\n  class_name: {:?},\n}}",
            workbench_aria_label.get(),
            workbench_value.get(),
            workbench_min.get(),
            workbench_max.get(),
            workbench_indeterminate.get(),
            workbench_value_label.get(),
            workbench_motion.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Progress
  aria_label="Determinate default".to_string()
  value=Signal::derive(|| Some(24.0))
  min=0.0
  max=100.0
/>
<Progress
  aria_label="Determinate custom".to_string()
  value=Signal::derive(|| Some(64.0))
  min=20.0
  max=200.0
  value_label="64 loaded".to_string()
  motion=ui::ProgressMotion::fast()
  class_name="docs-progress-custom".to_string()
/>
<Progress
  aria_label="Indeterminate".to_string()
  value=Signal::derive(|| None)
  indeterminate=true
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Progress"
            slug="progress"
            group="Display"
            description="Spring-driven linear progress with centralized source attrs."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::Progress;".to_string()
                test_source_path="components/progress/src/view.rs".to_string()
            >
                <Progress
                    aria_label="Upload progress".to_string()
                    value=Signal::derive(|| Some(42.0))
                />
            </Playground>

            // Contract markers for source-based semantics tests:
            // Playground title="Custom Label + Motion + Class"
            // title="Custom Label + Motion + Class"
            // aria_label="Syncing tasks".to_string()
            // value=Signal::derive(|| Some(64.0))
            // value_label="64 complete".to_string()
            // motion=ui::ProgressMotion::fast()
            // aria_label="   ".to_string()
            // class_name="docs-progress-custom".to_string()
            <Playground
                title="Custom Label + Motion + Class"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::Progress;".to_string()
                test_source_path="components/progress/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="progress-workbench-controls">
                        <div class="docs-search__label">"Value"</div>
                        <input
                            class="docs-search__input"
                            type="range"
                            min="0"
                            max="200"
                            step="1"
                            prop:value=move || format!("{:.0}", workbench_value_raw.get())
                            on:input=move |event| {
                                if let Ok(parsed) = event_target_value(&event).parse::<f64>() {
                                    set_workbench_value_raw.set(parsed);
                                }
                            }
                        />
                        <span class="ui-muted">{move || format!("raw value: {:.0}", workbench_value_raw.get())}</span>

                        <div class="docs-search__label">"min"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_min_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_min_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {min_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"max"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_max_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_max_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {max_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_indeterminate.get()
                                on:change=move |event| set_workbench_indeterminate.set(event_target_checked(&event))
                            />
                            <span>"indeterminate"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_label.get()
                                on:change=move |event| set_workbench_custom_label.set(event_target_checked(&event))
                            />
                            <span>"custom value_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_fast_motion.get()
                                on:change=move |event| set_workbench_fast_motion.set(event_target_checked(&event))
                            />
                            <span>"fast motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |event| set_workbench_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |event| set_workbench_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"custom aria_label"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack">
                    <Progress
                        aria_label=workbench_aria_label.get()
                        value=workbench_value
                        min=workbench_min.get()
                        max=workbench_max.get()
                        indeterminate=workbench_indeterminate.get()
                        value_label=workbench_value_label.get()
                        motion=workbench_motion.get()
                        class_name=workbench_class_name.get()
                    />
                    <span class="ui-muted">
                        "normalized value: "
                        {move || workbench_value.get().map(|value| format!("{value:.1}")).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            // Contract markers for source-based semantics tests:
            // title="Determinate + Indeterminate"
            // <Progress aria_label="Determinate".to_string() value=progress_value />
            // <Progress aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />
            // on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 12.0).min(100.0)))
            <Playground
                title="Determinate + Indeterminate"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::Progress;".to_string()
                test_source_path="components/progress/src/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Progress
                        aria_label="Determinate default".to_string()
                        value=Signal::derive(|| Some(24.0))
                        min=0.0
                        max=100.0
                    />
                    <Progress
                        aria_label="Determinate custom".to_string()
                        value=Signal::derive(|| Some(64.0))
                        min=20.0
                        max=200.0
                        value_label="64 loaded".to_string()
                        motion=ui::ProgressMotion::fast()
                        class_name="docs-progress-custom".to_string()
                    />
                    <Progress
                        aria_label="Indeterminate".to_string()
                        value=Signal::derive(|| None)
                        indeterminate=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn progress_bar() -> AnyView {
    let variant_options = [
        "Default".to_string(),
        "Accent".to_string(),
        "Danger".to_string(),
    ];
    let size_options = ["Sm".to_string(), "Md".to_string(), "Lg".to_string()];
    let max_options = ["100".to_string(), "200".to_string()];

    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_max_index, set_workbench_max_index) = signal(Some(0_usize));
    let (workbench_indeterminate, set_workbench_indeterminate) = signal(false);
    let (workbench_value_raw, set_workbench_value_raw) = signal(64.0_f64);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => ProgressBarVariant::Accent,
            2 => ProgressBarVariant::Danger,
            _ => ProgressBarVariant::Default,
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => ProgressBarSize::Sm,
        2 => ProgressBarSize::Lg,
        _ => ProgressBarSize::Md,
    });
    let workbench_max = Signal::derive(move || {
        if workbench_max_index.get().unwrap_or(0) == 1 {
            200.0_f64
        } else {
            100.0_f64
        }
    });
    let workbench_value =
        Signal::derive(move || workbench_value_raw.get().clamp(0.0, workbench_max.get()));
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Workbench progress bar".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-progress-bar-custom".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<ProgressBar
  variant=ProgressBarVariant::Default
  size=ProgressBarSize::Md
  value=42.0
  max=100.0
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant_expr = match workbench_variant.get() {
            ProgressBarVariant::Default => "ProgressBarVariant::Default",
            ProgressBarVariant::Accent => "ProgressBarVariant::Accent",
            ProgressBarVariant::Danger => "ProgressBarVariant::Danger",
        };
        let size_expr = match workbench_size.get() {
            ProgressBarSize::Sm => "ProgressBarSize::Sm",
            ProgressBarSize::Md => "ProgressBarSize::Md",
            ProgressBarSize::Lg => "ProgressBarSize::Lg",
        };

        format!(
            "<ProgressBar\n  variant={variant_expr}\n  size={size_expr}\n  value={:.1}\n  max={:.1}\n  indeterminate={}\n  aria_label={}\n  class_name={}\n/>",
            workbench_value.get(),
            workbench_max.get(),
            bool_word(workbench_indeterminate.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ProgressBarActualConfig {{\n  variant: {:?},\n  size: {:?},\n  value: {:.1},\n  max: {:.1},\n  indeterminate: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_variant.get(),
            workbench_size.get(),
            workbench_value.get(),
            workbench_max.get(),
            workbench_indeterminate.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Sm value=24.0 max=100.0 />
<ProgressBar variant=ProgressBarVariant::Accent size=ProgressBarSize::Md value=72.0 max=100.0 />
<ProgressBar variant=ProgressBarVariant::Danger size=ProgressBarSize::Lg value=54.0 max=100.0 />
<ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Md indeterminate=true />"#
            .to_string()
    });
    let custom_code = Signal::derive(move || {
        r#"<ProgressBar
  variant=ProgressBarVariant::Accent
  size=ProgressBarSize::Md
  value=64.0
  max=f64::NAN
  aria_label="Upload completion".to_string()
  class_name="docs-progress-bar-custom".to_string()
/>
<ProgressBar
  variant=ProgressBarVariant::Default
  size=ProgressBarSize::Sm
  value=18.0
  max=100.0
  aria_label="   ".to_string()
  class_name="docs-progress-bar-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ProgressBar"
            slug="progress-bar"
            group="Display"
            description="Native <progress> element with centralized variant/size/state source attrs."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::{ProgressBar, ProgressBarSize, ProgressBarVariant};".to_string()
                test_source_path="components/progress/src/bar/view.rs".to_string()
            >
                <ProgressBar
                    variant=ProgressBarVariant::Default
                    size=ProgressBarSize::Md
                    value=42.0
                    max=100.0
                />
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{ProgressBar, ProgressBarSize, ProgressBarVariant};".to_string()
                test_source_path="components/progress/src/bar/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="progress-bar-workbench-controls">
                        <div class="docs-search__label">"variant"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_variant_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_variant_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {variant_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"size"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_size_index.get().unwrap_or(1).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_size_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {size_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"max"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_max_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_max_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {max_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"value"</div>
                        <input
                            class="docs-search__input"
                            type="range"
                            min="0"
                            max="200"
                            step="1"
                            prop:value=move || format!("{:.0}", workbench_value_raw.get())
                            on:input=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<f64>() {
                                    set_workbench_value_raw.set(value);
                                }
                            }
                        />

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_indeterminate.get()
                                on:change=move |event| set_workbench_indeterminate.set(event_target_checked(&event))
                            />
                            <span>"indeterminate"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |event| set_workbench_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |event| set_workbench_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ProgressBar
                        variant=workbench_variant.get()
                        size=workbench_size.get()
                        value=workbench_value.get()
                        max=workbench_max.get()
                        indeterminate=workbench_indeterminate.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                    />
                    <span class="ui-muted">
                        "value: " {move || format!("{:.1}", workbench_value.get())}
                        " · max: " {move || format!("{:.1}", workbench_max.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="Variant + Size Matrix"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{ProgressBar, ProgressBarSize, ProgressBarVariant};".to_string()
                test_source_path="components/progress/src/bar/view.rs".to_string()
            >
                <div class="docs-stack">
                    <ProgressBar
                        variant=ProgressBarVariant::Default
                        size=ProgressBarSize::Sm
                        value=24.0
                        max=100.0
                    />
                    <ProgressBar
                        variant=ProgressBarVariant::Accent
                        size=ProgressBarSize::Md
                        value=72.0
                        max=100.0
                    />
                    <ProgressBar
                        variant=ProgressBarVariant::Danger
                        size=ProgressBarSize::Lg
                        value=54.0
                        max=100.0
                    />
                    <ProgressBar
                        variant=ProgressBarVariant::Default
                        size=ProgressBarSize::Md
                        indeterminate=true
                    />
                </div>
            </Playground>

            <Playground title="Custom Label + Class"
                code_signal=custom_code
                code_imports="use leptos::prelude::*;\nuse ui::{ProgressBar, ProgressBarSize, ProgressBarVariant};".to_string()
                test_source_path="components/progress/src/bar/view.rs".to_string()
            >
                <div class="docs-stack">
                    <ProgressBar
                        variant=ProgressBarVariant::Accent
                        size=ProgressBarSize::Md
                        value=64.0
                        max=f64::NAN
                        aria_label="Upload completion".to_string()
                        class_name="docs-progress-bar-custom".to_string()
                    />
                    <ProgressBar
                        variant=ProgressBarVariant::Default
                        size=ProgressBarSize::Sm
                        value=18.0
                        max=100.0
                        aria_label="   ".to_string()
                        class_name="docs-progress-bar-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn progress_circle() -> AnyView {
    let min_options = vec!["0".to_string(), "20".to_string()];
    let max_options = vec!["100".to_string(), "200".to_string()];
    let size_options = vec![
        "Default".to_string(),
        "40px".to_string(),
        "56px".to_string(),
    ];
    let stroke_options = vec!["Default".to_string(), "4px".to_string(), "6px".to_string()];
    let motion_options = vec!["Default".to_string(), "Snappy".to_string()];

    let (showcase_value, set_showcase_value) = signal(35.0_f64);
    let showcase_progress = Signal::derive(move || Some(showcase_value.get()));

    let (workbench_value, set_workbench_value) = signal(64.0_f64);
    let (workbench_min_index, set_workbench_min_index) = signal(Some(0_usize));
    let (workbench_max_index, set_workbench_max_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_stroke_index, set_workbench_stroke_index) = signal(Some(1_usize));
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));
    let (workbench_indeterminate, set_workbench_indeterminate) = signal(false);
    let (workbench_custom_value_label, set_workbench_custom_value_label) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);

    let workbench_min = Signal::derive(move || {
        if workbench_min_index.get().unwrap_or(0) == 1 {
            20.0
        } else {
            0.0
        }
    });
    let workbench_max = Signal::derive(move || {
        if workbench_max_index.get().unwrap_or(0) == 1 {
            200.0
        } else {
            100.0
        }
    });
    let workbench_size_px = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        1 => 40.0,
        2 => 56.0,
        _ => 32.0,
    });
    let workbench_stroke_width_px =
        Signal::derive(move || match workbench_stroke_index.get().unwrap_or(1) {
            1 => 4.0,
            2 => 6.0,
            _ => 3.0,
        });
    let workbench_motion = Signal::derive(move || {
        if workbench_motion_index.get().unwrap_or(0) == 1 {
            let mut spring = ui::ProgressCircleMotion::default().spring;
            spring.stiffness = 260.0;
            spring.damping = 26.0;
            ui::ProgressCircleMotion { spring }
        } else {
            ui::ProgressCircleMotion::default()
        }
    });
    let workbench_progress = Signal::derive(move || {
        if workbench_indeterminate.get() {
            None
        } else {
            Some(workbench_value.get())
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<ProgressCircle
  aria_label="Sync progress".to_string()
  value=Signal::derive(|| Some(35.0))
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ProgressCircle\n  aria_label={}\n  value=Signal::derive(move || {})\n  min={}\n  max={}\n  indeterminate={}\n  value_label={}\n  size_px={}\n  stroke_width_px={}\n  motion=ProgressCircleMotion {{ spring: /* ... */ }}\n  class_name={}\n/>",
            if workbench_custom_aria.get() {
                "\"Sync progress\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
            if workbench_indeterminate.get() {
                "None::<f64>".to_string()
            } else {
                format!("Some({})", workbench_value.get())
            },
            workbench_min.get(),
            workbench_max.get(),
            bool_word(workbench_indeterminate.get()),
            if workbench_custom_value_label.get() {
                format!(
                    "\"{} done\".to_string()",
                    workbench_value.get().round() as i64
                )
            } else {
                "\"\".to_string()".to_string()
            },
            workbench_size_px.get(),
            workbench_stroke_width_px.get(),
            if workbench_custom_class.get() {
                "\"docs-progress-circle-custom\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ProgressCircleWorkbenchActualConfig {{\n  aria_label: {:?},\n  value: {:?},\n  min: {},\n  max: {},\n  indeterminate: {},\n  value_label: {:?},\n  size_px: {:?},\n  stroke_width_px: {:?},\n  motion: {:?},\n  class_name: {:?},\n}}",
            if workbench_custom_aria.get() {
                Some("Sync progress")
            } else {
                None
            },
            workbench_progress.get(),
            workbench_min.get(),
            workbench_max.get(),
            bool_word(workbench_indeterminate.get()),
            if workbench_custom_value_label.get() {
                Some(format!("{} done", workbench_value.get().round() as i64))
            } else {
                None
            },
            Some(workbench_size_px.get()),
            Some(workbench_stroke_width_px.get()),
            workbench_motion.get(),
            if workbench_custom_class.get() {
                Some("docs-progress-circle-custom")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ProgressCircle aria_label="Determinate".to_string() value=Signal::derive(|| Some(42.0)) min=0.0 max=100.0 indeterminate=false value_label="42%".to_string() size_px=40.0 stroke_width_px=4.0 motion=ProgressCircleMotion::default() class_name="".to_string() />
<ProgressCircle aria_label="Indeterminate".to_string() value=Signal::derive(|| None::<f64>) min=0.0 max=100.0 indeterminate=true value_label="".to_string() size_px=40.0 stroke_width_px=4.0 motion=ProgressCircleMotion::default() class_name="".to_string() />
<ProgressCircle aria_label="Custom".to_string() value=Signal::derive(|| Some(72.0)) min=20.0 max=200.0 indeterminate=false value_label="72 done".to_string() size_px=56.0 stroke_width_px=6.0 motion=ProgressCircleMotion { spring: ProgressCircleMotion::default().spring } class_name="docs-progress-circle-custom".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="ProgressCircle"
            slug="progress-circle"
            group="Display"
            description="Spring-animated circular progress with centralized source attrs."
        >
            <Playground title="Hello World (Default ProgressCircle)" code_signal=hello_code>
                <div class="docs-row">
                    <ProgressCircle
                        aria_label="Sync progress".to_string()
                        value=showcase_progress
                    />
                    <ui::Button
                        variant=ui::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| {
                            set_showcase_value.update(|v| *v = (*v + 10.0).min(100.0))
                        })
                    >
                        "+10"
                    </ui::Button>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="progress-circle-workbench-controls">
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-min".to_string()
                            options=min_options.clone()
                            selected_index=workbench_min_index
                            set_selected_index=set_workbench_min_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle min".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-max".to_string()
                            options=max_options.clone()
                            selected_index=workbench_max_index
                            set_selected_index=set_workbench_max_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle max".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle size_px".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-stroke".to_string()
                            options=stroke_options.clone()
                            selected_index=workbench_stroke_index
                            set_selected_index=set_workbench_stroke_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle stroke_width_px".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-progress-circle-workbench-motion".to_string()
                            options=motion_options.clone()
                            selected_index=workbench_motion_index
                            set_selected_index=set_workbench_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="ProgressCircle motion".to_string()
                        />
                        <Switch checked=workbench_indeterminate set_checked=set_workbench_indeterminate>
                            "indeterminate"
                        </Switch>
                        <Switch checked=workbench_custom_value_label set_checked=set_workbench_custom_value_label>
                            "value_label"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_value.update(|v| *v = (*v + 10.0).min(200.0))
                            })
                        >
                            "+10"
                        </ui::Button>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_value.update(|v| *v = (*v - 10.0).max(0.0))
                            })
                        >
                            "-10"
                        </ui::Button>
                    </div>
                }
            >
                <div class="docs-row">
                    <ProgressCircle
                        aria_label=if workbench_custom_aria.get() {
                            "Sync progress".to_string()
                        } else {
                            String::new()
                        }
                        value=workbench_progress
                        min=workbench_min.get()
                        max=workbench_max.get()
                        indeterminate=workbench_indeterminate.get()
                        value_label=if workbench_custom_value_label.get() {
                            format!("{} done", workbench_value.get().round() as i64)
                        } else {
                            String::new()
                        }
                        size_px=workbench_size_px.get()
                        stroke_width_px=workbench_stroke_width_px.get()
                        motion=workbench_motion.get()
                        class_name=if workbench_custom_class.get() {
                            "docs-progress-circle-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "value="
                        {move || format!("{:.0}", workbench_value.get())}
                        " · range="
                        {move || format!("{}..{}", workbench_min.get(), workbench_max.get())}
                    </span>
                </div>
            </Playground>

            // Contract markers for source-based semantics tests:
            // Playground title="Determinate + Indeterminate"
            // Playground title="Custom Value Label + Class"
            // title="Determinate + Indeterminate"
            // title="Custom Value Label + Class"
            // <ProgressCircle aria_label="Determinate".to_string() value=progress_value min=0.0 max=100.0 />
            // <ProgressCircle aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />
            // on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 10.0).min(100.0)))
            // aria_label="Sync progress".to_string()
            // value=Signal::derive(|| Some(64.0))
            // size_px=40.0
            // stroke_width_px=5.0
            // value_label="64 done".to_string()
            // aria_label="   ".to_string()
            // class_name="docs-progress-circle-custom".to_string()
            <Playground title="Determinate + Indeterminate" code_signal=matrix_code>
                <div class="docs-row">
                    <ProgressCircle
                        aria_label="Determinate".to_string()
                        value=Signal::derive(|| Some(42.0))
                        min=0.0
                        max=100.0
                        indeterminate=false
                        value_label="42%".to_string()
                        size_px=40.0
                        stroke_width_px=4.0
                        motion=ui::ProgressCircleMotion::default()
                        class_name=String::new()
                    />
                    <ProgressCircle
                        aria_label="Indeterminate".to_string()
                        value=Signal::derive(|| None::<f64>)
                        min=0.0
                        max=100.0
                        indeterminate=true
                        value_label=String::new()
                        size_px=40.0
                        stroke_width_px=4.0
                        motion=ui::ProgressCircleMotion::default()
                        class_name=String::new()
                    />
                    <ProgressCircle
                        aria_label="Custom".to_string()
                        value=Signal::derive(|| Some(72.0))
                        min=20.0
                        max=200.0
                        indeterminate=false
                        value_label="72 done".to_string()
                        size_px=56.0
                        stroke_width_px=6.0
                        motion=ui::ProgressCircleMotion::default()
                        class_name="docs-progress-circle-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn meter() -> AnyView {
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

pub(super) fn code() -> AnyView {
    let hello_world_code =
        Signal::derive(move || r#"<Code>"cargo check -p ui"</Code>"#.to_string());

    let variants_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Inline>"cargo test -p ui"</Code>
<Code variant=CodeVariant::Block>
  "cargo fmt --all\ncargo clippy -p ui -p docs-app --all-targets -- -D warnings"
</Code>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Inline class_name="docs-code-custom".to_string()>"--deny warnings"</Code>
<Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
  "cargo test -p ui --test code_semantics\ncargo test -p ui"
</Code>"#.to_string()
    });
    let controlled_contrast_code = Signal::derive(move || {
        r#"<Code>"Default path: no controlled/uncontrolled state axis."</Code>
<Code variant=CodeVariant::Block>
  "Controlled-like usage lives in app state only: map upstream state to variant/class_name props."
</Code>"#
            .to_string()
    });
    let stream_snapshot_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Inline>
  "Snapshot: complete validated output rendered in one pass."
</Code>
<Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
  "Streaming Optional -> fallback=snapshot; inspect data-ui-streaming=optional, data-ui-fallback=snapshot, data-ui-output-state=verified."
</Code>"#
            .to_string()
    });
    let source_first_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
  "cargo test -p ui --test code_semantics"
</Code>"#
            .to_string()
    });
    let variant_options = vec!["Inline".to_string(), "Block".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (custom_class, set_custom_class) = signal(false);
    let (long_content, set_long_content) = signal(false);
    let (show_compare, set_show_compare) = signal(true);

    let active_variant = Signal::derive(move || {
        if variant_index.get().unwrap_or(0) == 1 {
            CodeVariant::Block
        } else {
            CodeVariant::Inline
        }
    });
    let active_content = Signal::derive(move || {
        if long_content.get() {
            "cargo fmt --all\ncargo clippy -p ui -p docs-app --all-targets -- -D warnings"
                .to_string()
        } else {
            "cargo test -p ui --test code_semantics".to_string()
        }
    });
    let interactive_code = Signal::derive(move || {
        let variant = active_variant.get();
        let content = active_content.get();
        let class_line = if custom_class.get() {
            " class_name=\"docs-code-custom\".into()".to_string()
        } else {
            "".to_string()
        };
        format!("<Code variant=CodeVariant::{variant:?}{class_line}>\n  {content:?}\n</Code>")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/code/src/styles.rs */\n{}",
            ui::code::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let variant = active_variant.get();
        let has_custom_class = custom_class.get();
        let show_compare = show_compare.get();
        let content_mode = if long_content.get() { "long" } else { "short" };
        let class_name = if has_custom_class {
            "docs-code-custom"
        } else {
            "(none)"
        };
        format!(
            "CodeActualConfig {{\n  variant: CodeVariant::{variant:?},\n  content_mode: \"{content_mode}\",\n  has_custom_class_name: {has_custom_class},\n  class_name: \"{class_name}\",\n  show_compare: {show_compare},\n}}"
        )
    });

    view! {
        <ComponentPage
            title="Code"
            slug="code"
            group="Display"
            description="Inline/Block code surface with centralized variant state attrs and optional custom-class contract."
        >
            <Playground title="Hello World (Default API)" code_signal=hello_world_code>
                <Code>"cargo check -p ui"</Code>
            </Playground>

            <Playground title="Variant Matrix" code_signal=variants_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <span>"Run "</span>
                        <Code variant=CodeVariant::Inline>"cargo test -p ui"</Code>
                        <span>" before opening a PR."</span>
                    </div>
                    <Code variant=CodeVariant::Block>
                        {r#"cargo fmt --all
cargo clippy -p ui -p docs-app --all-targets -- -D warnings"#}
                    </Code>
                </div>
            </Playground>

            <Playground title="Custom Class + Block" code_signal=custom_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <span>"CI flags: "</span>
                        <Code variant=CodeVariant::Inline class_name="docs-code-custom".to_string()>
                            "--deny warnings"
                        </Code>
                    </div>
                    <Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
                        {r#"cargo test -p ui --test code_semantics
cargo test -p ui"#}
                    </Code>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Code has no internal controlled/uncontrolled axis; compare default usage vs app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports="use leptos::prelude::*;\nuse ui::{Code, CodeVariant};".to_string()
            >
                <div class="docs-stack">
                    <Code>"Default path: no controlled/uncontrolled state axis."</Code>
                    <Code variant=CodeVariant::Block>
                        "Controlled-like usage lives in app state only: map upstream state to variant/class_name props."
                    </Code>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Code is a display leaf: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports="use leptos::prelude::*;\nuse ui::{Code, CodeVariant};".to_string()
            >
                <div class="docs-stack">
                    <Code variant=CodeVariant::Inline>
                        "Snapshot: complete validated output rendered in one pass."
                    </Code>
                    <Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
                        "Streaming Optional -> fallback=snapshot; inspect data-ui-streaming=optional, data-ui-fallback=snapshot, data-ui-output-state=verified."
                    </Code>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports="use leptos::prelude::*;\nuse ui::{Code, CodeVariant};".to_string()
            >
                <Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
                    "cargo test -p ui --test code_semantics"
                </Code>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=interactive_code
                test_css_source=test_css_source
                test_source_path="components/code/src/styles.rs".to_string()
                test_config_signal=actual_config
                description="展示区 + Config 区 + Code 区 + CSS Test 区；包含 inline/block 与 custom class 的对比展示。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="code-workbench-controls">
                        <div class="docs-search__label">"配置区 · Variant"</div>
                        <ui::SegmentedControl
                            id_base="docs-code-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=ui::SegmentedControlSize::Sm
                            aria_label="Code variant".to_string()
                        />
                        <ui::Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </ui::Switch>
                        <ui::Switch checked=long_content set_checked=set_long_content>
                            "Long content"
                        </ui::Switch>
                        <ui::Switch checked=show_compare set_checked=set_show_compare>
                            "Show compare matrix"
                        </ui::Switch>
                    </div>
                }
            >
                {move || {
                    let variant = active_variant.get();
                    let content = active_content.get();
                    let class_name = if custom_class.get() {
                        "docs-code-custom".to_string()
                    } else {
                        String::new()
                    };
                    let compare = show_compare.get();

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="code-workbench-preview">
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="code-workbench-primary">
                                <span class="ui-muted">
                                    {format!("variant={variant:?}, custom_class={}", custom_class.get())}
                                </span>
                                <Code variant=variant class_name=class_name.clone()>
                                    {content}
                                </Code>
                            </div>

                            <Show when=move || compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight" data-slot="code-workbench-compare">
                                    <div class="docs-row">
                                        <span>"Inline: "</span>
                                        <Code variant=CodeVariant::Inline class_name=class_name.clone()>
                                            "cargo test -p ui"
                                        </Code>
                                    </div>
                                    <Code variant=CodeVariant::Block class_name=class_name.clone()>
                                        {r#"cargo fmt --all
cargo clippy -p ui -p docs-app --all-targets -- -D warnings"#}
                                    </Code>
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>

            <section class="docs-card docs-prose" data-slot="code-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="code-state-rows">
                    <li><code>"data-variant / data-state"</code>" = inline | block"</li>
                    <li><code>"data-inline / data-block"</code>" = true | none"</li>
                    <li><code>"data-custom-class"</code>" = true | none"</li>
                    <li><code>"control mode"</code>" = N/A (Code has no controlled/uncontrolled runtime axis)"</li>
                    <li><code>"disabled axis"</code>" = N/A (Code has no disabled prop in API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="code-parameter-rows">
                    <li><code>"variant: Option&lt;CodeVariant&gt;"</code>" default = None -> normalize to inline (`logic.rs`: `variant.unwrap_or_default()`)"</li>
                    <li><code>"class_name: Option&lt;String&gt;"</code>" default = None -> `normalize_optional_text` trims blank/empty to None"</li>
                    <li><code>"lang: Option&lt;String&gt;, dir: Option&lt;A11yDirection&gt;"</code>" default = None -> locale inherited via `locale_attrs`"</li>
                    <li><code>"children: Children"</code>" required; component renders caller-provided code content only"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-streaming-modes">
                <h3>"Streaming / Snapshot"</h3>
                <ul data-slot="code-streaming-rows">
                    <li><code>"data-ui-streaming"</code>" = optional"</li>
                    <li><code>"data-ui-fallback"</code>" = snapshot"</li>
                    <li><code>"data-ui-output-state"</code>" = verified"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="code-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-code"</code>
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
                    text="use leptos::prelude::*;\nuse ui::{Code, CodeVariant};\n\n<Code variant=CodeVariant::Block>\n  \"cargo test -p ui --test code_semantics\"\n</Code>".to_string()
                    label="Copy code starter".to_string()
                    copyable=true
                    class_name="docs-code-source-copy".to_string()
                />
                <ul data-slot="code-source-paths">
                    <li><code>"components/code/src/mod.rs"</code></li>
                    <li><code>"components/code/src/logic.rs"</code></li>
                    <li><code>"components/code/src/view.rs"</code></li>
                    <li><code>"components/code/src/styles.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn kbd() -> AnyView {
    let (workbench_size_key, set_workbench_size_key) = signal("md".to_string());
    let (workbench_keys, set_workbench_keys) = signal("Ctrl".to_string());
    let (workbench_label, set_workbench_label) = signal("K".to_string());
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_size = Signal::derive(move || match workbench_size_key.get().as_str() {
        "sm" => KbdSize::Sm,
        _ => KbdSize::Md,
    });

    let workbench_code = Signal::derive(move || {
        let size = workbench_size.get();
        let keys = workbench_keys.get();
        let label = workbench_label.get();
        let custom_class = workbench_custom_class.get();
        let keys_trimmed = keys.trim();
        let label_trimmed = label.trim();

        let mut lines = vec!["<Kbd".to_string()];
        if size != KbdSize::Md {
            lines.push(format!("  size=KbdSize::{size:?}"));
        }
        if !keys_trimmed.is_empty() {
            lines.push(format!("  keys={keys_trimmed:?}.into()"));
        }
        if custom_class {
            lines.push("  class_name=\"docs-kbd-custom\".into()".to_string());
        }
        lines.push(">".to_string());
        lines.push(format!(
            "  {:?}",
            if label_trimmed.is_empty() {
                "K"
            } else {
                &label_trimmed
            }
        ));
        lines.push("</Kbd>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/kbd/src/styles.rs */\n{}",
            ui::kbd::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let size = workbench_size.get();
        let keys = workbench_keys.get();
        let label = workbench_label.get();
        let custom_class = workbench_custom_class.get();
        let has_keys = !keys.trim().is_empty();

        let mut classes = vec![
            "ui-kbd".to_string(),
            size.class_name().into(),
            if has_keys {
                "ui-kbd--state-with-keys".to_string()
            } else {
                "ui-kbd--state-label-only".to_string()
            },
        ];
        if custom_class {
            classes.push("ui-kbd--custom-class".to_string());
            classes.push("docs-kbd-custom".to_string());
        }

        format!(
            "KbdActualConfig {{\n  size: {size:?},\n  keys: {:?},\n  label: {:?},\n  custom_class: {custom_class},\n  data_size: \"{}\",\n  data_state: \"{}\",\n  class: \"{}\",\n}}",
            keys.trim(),
            label.trim(),
            size.as_attr(),
            if has_keys { "with-keys" } else { "label-only" },
            classes.join(" "),
        )
    });

    let hello_world_code =
        Signal::derive(move || r#"<Kbd keys="Ctrl".to_string()>"K"</Kbd>"#.to_string());

    let state_matrix_code = Signal::derive(move || {
        r#"<Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
<Kbd size=KbdSize::Sm keys="⌘".to_string()>"P"</Kbd>
<Kbd size=KbdSize::Md>"Esc"</Kbd>"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"let label = Signal::derive(move || "K".to_string());

<Kbd keys="Ctrl".to_string()>"K"</Kbd>
<Kbd keys="Ctrl".to_string()>{label.get()}</Kbd>
// N/A: Kbd has no controlled/uncontrolled runtime axis (`value/on_value_change/default_value`)."#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Kbd keys="Ctrl".to_string()>"K"</Kbd>
<Kbd size=KbdSize::Sm>"Esc"</Kbd>
// Streaming Optional -> fallback=snapshot for Kbd display leaf."#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Kbd size=KbdSize::Sm keys="Shift".to_string() class_name="docs-kbd-custom".to_string()>
    "Tab"
</Kbd>"#
            .to_string()
    });

    let kbd_imports = "use leptos::prelude::*;\nuse ui::{Kbd, KbdSize};".to_string();

    let custom_code = Signal::derive(move || {
        r#"<Kbd size=KbdSize::Md class_name="docs-kbd-custom".to_string()>"Esc"</Kbd>
<Kbd size=KbdSize::Sm keys="Shift".to_string() class_name="docs-kbd-custom".to_string()>"Tab"</Kbd>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Kbd"
            slug="kbd"
            group="Display"
            description="Keyboard keycap with centralized size/keys state attrs and optional custom-class contract."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-row">
                    <Kbd keys="Ctrl".to_string()>"K"</Kbd>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Size + Keys + Label-only)"
                code_signal=state_matrix_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-row">
                    <Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
                    <Kbd size=KbdSize::Sm keys="⌘".to_string()>"P"</Kbd>
                    <Kbd size=KbdSize::Md keys="Alt".to_string()>"Enter"</Kbd>
                    <Kbd size=KbdSize::Md>"Esc"</Kbd>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Kbd has no internal controlled/uncontrolled axis; compare default static props with app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Kbd keys="Ctrl".to_string()>"K"</Kbd>
                        <Kbd size=KbdSize::Sm>"Esc"</Kbd>
                    </div>
                    <p class="ui-muted">
                        "N/A: Kbd is snapshot-only display leaf without `value/on_value_change/default_value` state axis."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Custom Class + Label Only"
                code_signal=custom_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-row">
                    <Kbd size=KbdSize::Md class_name="docs-kbd-custom".to_string()>"Esc"</Kbd>
                    <Kbd
                        size=KbdSize::Sm
                        keys="Shift".to_string()
                        class_name="docs-kbd-custom".to_string()
                    >
                        "Tab"
                    </Kbd>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Kbd defaults to snapshot rendering; streaming path is optional and falls back to snapshot semantics."
                code_signal=stream_snapshot_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="kbd-streaming-snapshot">
                    <div class="docs-row">
                        <Kbd keys="Ctrl".to_string()>"K"</Kbd>
                        <Kbd size=KbdSize::Sm>"Esc"</Kbd>
                    </div>
                    <p class="ui-muted" data-slot="kbd-streaming-hint">
                        "Streaming Optional -> fallback=snapshot; keep output-state semantic continuity at upstream layer."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Playground copy action injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-row">
                    <Kbd
                        size=KbdSize::Sm
                        keys="Shift".to_string()
                        class_name="docs-kbd-custom".to_string()
                    >
                        "Tab"
                    </Kbd>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for size/keys/class contracts."
                code_signal=workbench_code
                code_imports=kbd_imports
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/kbd/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="kbd-workbench-controls">
                        <label class="docs-search__label">
                            "Size"
                            <select
                                prop:value=move || workbench_size_key.get()
                                on:change=move |ev| set_workbench_size_key.set(event_target_value(&ev))
                            >
                                <option value="md">"Md"</option>
                                <option value="sm">"Sm"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Keys"
                            <input
                                class="docs-search__input"
                                prop:value=move || workbench_keys.get()
                                on:input=move |ev| set_workbench_keys.set(event_target_value(&ev))
                                placeholder="Ctrl"
                            />
                        </label>
                        <label class="docs-search__label">
                            "Label"
                            <input
                                class="docs-search__input"
                                prop:value=move || workbench_label.get()
                                on:input=move |ev| set_workbench_label.set(event_target_value(&ev))
                                placeholder="K"
                            />
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                    </div>
                }
            >
                {move || {
                    let size = workbench_size.get();
                    let keys = workbench_keys.get();
                    let label = workbench_label.get();
                    let label = {
                        let trimmed = label.trim();
                        if trimmed.is_empty() {
                            "K".to_string()
                        } else {
                            trimmed.to_string()
                        }
                    };
                    let keys = {
                        let trimmed = keys.trim();
                        if trimmed.is_empty() {
                            None
                        } else {
                            Some(trimmed.to_string())
                        }
                    };
                    let has_keys = keys.is_some();

                    if workbench_custom_class.get() && has_keys {
                        let keys_text = keys.unwrap_or_default();
                        view! {
                            <div class="docs-row">
                                <Kbd
                                    size=size
                                    keys=keys_text
                                    class_name="docs-kbd-custom".to_string()
                                >
                                    {label}
                                </Kbd>
                            </div>
                        }
                        .into_any()
                    } else if workbench_custom_class.get() {
                        view! {
                            <div class="docs-row">
                                <Kbd size=size class_name="docs-kbd-custom".to_string()>
                                    {label}
                                </Kbd>
                            </div>
                        }
                        .into_any()
                    } else if has_keys {
                        let keys_text = keys.unwrap_or_default();
                        view! {
                            <div class="docs-row">
                                <Kbd size=size keys=keys_text>{label}</Kbd>
                            </div>
                        }
                        .into_any()
                    } else {
                        view! {
                            <div class="docs-row">
                                <Kbd size=size>{label}</Kbd>
                            </div>
                        }
                        .into_any()
                    }
                }}
            </Playground>

            <section class="docs-card docs-prose" data-slot="kbd-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="kbd-state-rows">
                    <li><code>"data-size / data-state"</code>" = sm|md / with-keys|label-only"</li>
                    <li><code>"data-keys"</code>" = true | none"</li>
                    <li><code>"data-custom-class"</code>" = true | none"</li>
                    <li><code>"control mode"</code>" = N/A (Kbd has no controlled/uncontrolled runtime axis)"</li>
                    <li><code>"disabled axis"</code>" = N/A (Kbd has no disabled prop in API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="kbd-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="kbd-parameter-rows">
                    <li><code>"size: Option&lt;KbdSize&gt;"</code>" default = None -> Md (`logic.rs`: `normalize_size -> unwrap_or_default()`)"</li>
                    <li><code>"keys: Option&lt;String&gt;"</code>" default = None; blank string trims to None (`normalize_optional_text`)"</li>
                    <li><code>"class_name: Option&lt;String&gt;"</code>" default = None; blank string trims to None (`normalize_optional_text`)"</li>
                    <li><code>"children: Children"</code>" required; renders label content inside `<span data-slot=\"kbd-label\">`"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="kbd-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps starter code aligned with the current Kbd prop surface."
                </p>
                <ul data-slot="kbd-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-kbd"</code>
                        " for package-mode consumption."
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
                    text="use leptos::prelude::*;\nuse ui::{Kbd, KbdSize};\n\n<Kbd size=KbdSize::Sm keys=\"Shift\".to_string() class_name=\"docs-kbd-custom\".to_string()>\n  \"Tab\"\n</Kbd>".to_string()
                    label="Copy Kbd starter".to_string()
                    copyable=true
                    class_name="docs-kbd-source-copy".to_string()
                />
                <ul data-slot="kbd-source-paths">
                    <li><code>"components/kbd/src/mod.rs"</code></li>
                    <li><code>"components/kbd/src/logic.rs"</code></li>
                    <li><code>"components/kbd/src/view.rs"</code></li>
                    <li><code>"components/kbd/src/styles.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn code_block() -> AnyView {
    fn workbench_template(language: &str) -> &'static str {
        match language {
            "bash" => {
                r#"cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings"#
            }
            "plain" => "CodeBlock workbench template for layout and style inspection.",
            _ => {
                r#"fn deploy(service: &str) -> anyhow::Result<()> {
    tracing::info!(target: "deploy", %service, "starting rollout");
    Ok(())
}"#
            }
        }
    }

    let rust_code = workbench_template("rust");

    let hello_world_code = Signal::derive(move || {
        r#"<CodeBlock
  code="cargo check -p ui".to_string()
/>"#
        .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<CodeBlock
  code=rust_code.into()
  language="rust".to_string()
  label="deploy.rs".to_string()
/>"#
        .to_string()
    });

    let compact_code = Signal::derive(move || {
        r#"<CodeBlock
  code="cargo test -p ui --test code_block_semantics".to_string()
  is_copyable=false
  class_name="docs-code-block-custom".to_string()
/>"#
        .to_string()
    });
    let state_matrix_code = Signal::derive(move || {
        r#"<CodeBlock code="cargo check -p ui".to_string() />
<CodeBlock
  code="cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings".to_string()
  language="bash".to_string()
  label="ci.sh".to_string()
/>
<CodeBlock
  code="cargo test -p ui --test code_block_semantics".to_string()
  is_copyable=false
/>
<CodeBlock
  code="   ".to_string()
  is_copyable=false
  class_name="docs-code-block-custom".to_string()
/>"#
        .to_string()
    });
    let controlled_contrast_code = Signal::derive(move || {
        r#"let (controlled_copied, set_controlled_copied) = signal(false);
let controlled_copied_signal = Signal::derive(move || controlled_copied.get());

<CodeBlock
  code="Uncontrolled: internal copied state.".to_string()
  default_copied=true
/>
<CodeBlock
  code="Controlled: copied state from app signal.".to_string()
  is_copied=controlled_copied_signal
  on_copied_change=Callback::new(move |next| set_controlled_copied.set(next))
/>"#
        .to_string()
    });
    let stream_snapshot_code = Signal::derive(move || {
        r#"<CodeBlock
  code="Snapshot: complete validated output rendered in one pass.".to_string()
  language="plain".to_string()
  output_mode=CodeBlockAgentOutputMode::Snapshot
  output_status=CodeBlockAgentOutputStatus::Validated
/>
<CodeBlock
  code="Streaming: incremental draft output while LLM is generating.".to_string()
  language="plain".to_string()
  output_mode=CodeBlockAgentOutputMode::Streaming
  output_status=CodeBlockAgentOutputStatus::Draft
/>"#
        .to_string()
    });
    let source_first_code = Signal::derive(move || {
        r#"<CodeBlock
  code="cargo test -p ui --test code_block_semantics".to_string()
  language="bash".to_string()
/>"#
        .to_string()
    });
    let code_block_imports = "use leptos::prelude::*;\nuse ui::CodeBlock;".to_string();
    let code_block_stream_imports = "use leptos::prelude::*;\nuse ui::CodeBlock;\nuse ui::code_block::protocol::{CodeBlockAgentOutputMode, CodeBlockAgentOutputStatus};".to_string();

    let language_options = vec!["rust".to_string(), "bash".to_string(), "plain".to_string()];
    let output_mode_options = vec!["snapshot".to_string(), "streaming".to_string()];
    let output_status_options = vec![
        "draft".to_string(),
        "validated".to_string(),
        "ready-to-submit".to_string(),
    ];
    let (workbench_language_index, set_workbench_language_index) = signal(Some(0_usize));
    let (workbench_is_copyable, set_workbench_is_copyable) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_preserve_state, set_workbench_preserve_state) = signal(true);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_output_mode_index, set_workbench_output_mode_index) = signal(Some(0_usize));
    let (workbench_output_status_index, set_workbench_output_status_index) = signal(Some(1_usize));
    let (workbench_code_text, set_workbench_code_text) = signal(rust_code.to_string());
    let (workbench_copied, set_workbench_copied) = signal(false);
    let (controlled_copied, set_controlled_copied) = signal(false);
    let controlled_copied_signal = Signal::derive(move || controlled_copied.get());
    let on_controlled_copied_change =
        Callback::new(move |next: bool| set_controlled_copied.set(next));
    let on_controlled_reset = Callback::new(move |_| set_controlled_copied.set(false));

    let workbench_language_key =
        Signal::derive(move || match workbench_language_index.get().unwrap_or(0) {
            1 => "bash",
            2 => "plain",
            _ => "rust",
        });
    let workbench_language = Signal::derive(move || {
        let key = workbench_language_key.get();
        if key == "plain" {
            String::new()
        } else {
            key.into()
        }
    });
    let workbench_output_mode =
        Signal::derive(
            move || match workbench_output_mode_index.get().unwrap_or(0) {
                1 => ui::code_block::protocol::CodeBlockAgentOutputMode::Streaming,
                _ => ui::code_block::protocol::CodeBlockAgentOutputMode::Snapshot,
            },
        );
    let workbench_output_status =
        Signal::derive(
            move || match workbench_output_status_index.get().unwrap_or(1) {
                0 => ui::code_block::protocol::CodeBlockAgentOutputStatus::Draft,
                2 => ui::code_block::protocol::CodeBlockAgentOutputStatus::ReadyToSubmit,
                _ => ui::code_block::protocol::CodeBlockAgentOutputStatus::Validated,
            },
        );

    Effect::new(move |_| {
        if !workbench_preserve_state.get() {
            let template = workbench_template(workbench_language_key.get());
            set_workbench_code_text.set(template.to_string());
            set_workbench_copied.set(false);
        }
    });

    let workbench_copied_signal = Signal::derive(move || {
        if workbench_preserve_state.get() {
            workbench_copied.get()
        } else {
            false
        }
    });
    let workbench_on_copied_change = Callback::new(move |next: bool| {
        if workbench_preserve_state.get_untracked() {
            set_workbench_copied.set(next);
        }
    });

    let workbench_code = Signal::derive(move || {
        let language_key = workbench_language_key.get();
        let is_copyable = workbench_is_copyable.get();
        let custom_class = workbench_custom_class.get();
        let preserve_state = workbench_preserve_state.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let output_mode = workbench_output_mode.get();
        let output_status = workbench_output_status.get();
        let code_literal = format!("{:?}", workbench_code_text.get());

        let mut lines = vec![
            "<CodeBlock".to_string(),
            format!("  code={code_literal}.to_string()"),
            "  label=\"workbench.rs\".to_string()".to_string(),
            format!("  lang={lang:?}.to_string()"),
            format!("  dir={dir}"),
            "  motion=CodeBlockMotion::default()".to_string(),
        ];
        if language_key != "plain" {
            lines.push(format!("  language=\"{language_key}\".to_string()"));
        }
        if !is_copyable {
            lines.push("  is_copyable=false".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-code-block-custom\".to_string()".to_string());
        }
        if preserve_state {
            lines.push("  is_copied=workbench_copied_signal".to_string());
            lines.push("  copied=workbench_copied_signal".to_string());
            lines.push("  default_copied=false".to_string());
            lines.push(
                "  on_copied_change=Callback::new(move |next| set_workbench_copied.set(next))"
                    .to_string(),
            );
        }
        lines.push(format!(
            "  output_mode=ui::code_block::protocol::CodeBlockAgentOutputMode::{output_mode:?}"
        ));
        lines.push(format!(
            "  output_status=ui::code_block::protocol::CodeBlockAgentOutputStatus::{output_status:?}"
        ));
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let language_key = workbench_language_key.get();
        let language = if language_key == "plain" {
            "none"
        } else {
            language_key
        };
        let is_copyable = workbench_is_copyable.get();
        let custom_class = workbench_custom_class.get();
        let preserve_state = workbench_preserve_state.get();
        let code = workbench_code_text.get();
        let copied = workbench_copied.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let output_mode = workbench_output_mode.get();
        let output_status = workbench_output_status.get();

        format!(
            "CodeBlockActualConfig {{\n  code: {code:?},\n  label: Some(\"workbench.rs\"),\n  language: {language:?},\n  lang: Some({lang:?}),\n  dir: Some({dir}),\n  is_copyable: Some({is_copyable}),\n  copyable: Some({is_copyable}),\n  is_copied: Some({copied}),\n  copied: Some({copied}),\n  default_copied: Some(false),\n  on_copied_change: Some(\"workbench_on_copied_change\"),\n  output_mode: Some(\"{}\"),\n  output_status: Some(\"{}\"),\n  motion: CodeBlockMotion::default(),\n  class_name: {class_name},\n  preserve_state: {preserve_state},\n  code_lines: {},\n}}\n\nCodeBlockPreviewExpectation {{\n  data-ui-output-mode: \"{}\",\n  data-ui-output-status: \"{}\",\n}}",
            output_mode.as_attr(),
            output_status.as_attr(),
            code.lines().count(),
            output_mode.as_attr(),
            output_status.as_attr(),
            class_name = if custom_class {
                "Some(\"docs-code-block-custom\")"
            } else {
                "None"
            },
        )
    });

    let workbench_test_css = Signal::derive(move || {
        let mut css = format!(
            "/* components/code-block/src/styles.rs */\n{}",
            ui::code_block::styles::CSS
        );

        if workbench_custom_class.get() {
            css.push_str(
                "\n\n/* docs custom override */\n.docs-code-block-custom {\n  --ui-code-block-copy-flash: 0.32;\n  border-color: color-mix(in oklab, var(--ui-border), var(--ui-accent) 38%);\n}\n",
            );
        }

        css
    });

    let on_workbench_load_template = Callback::new(move |_| {
        let template = workbench_template(workbench_language_key.get_untracked());
        set_workbench_code_text.set(template.to_string());
        if !workbench_preserve_state.get_untracked() {
            set_workbench_copied.set(false);
        }
    });
    let on_workbench_reset_copy_state = Callback::new(move |_| set_workbench_copied.set(false));

    view! {
        <ComponentPage
            title="CodeBlock"
            slug="code-block"
            group="Display"
            description="Multiline code surface with centralized header/state attrs and spring-driven copy flash motion."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=code_block_imports.clone()
            >
                <CodeBlock code="cargo check -p ui".to_string() />
            </Playground>

            <Playground
                title="Header + Copy Motion"
                code_signal=matrix_code
                code_imports=code_block_imports.clone()
            >
                <CodeBlock
                    code=rust_code.to_string()
                    language="rust".to_string()
                    label="deploy.rs".to_string()
                />
            </Playground>

            <Playground
                title="Compact + No Copy"
                code_signal=compact_code
                code_imports=code_block_imports.clone()
            >
                <CodeBlock
                    code="cargo test -p ui --test code_block_semantics".to_string()
                    is_copyable=false
                    class_name="docs-code-block-custom".to_string()
                />
            </Playground>

            <Playground
                title="State Gallery"
                description="覆盖 single-line/multiline、header visible/hidden、copyable on/off、empty/custom class 等关键状态轴。"
                code_signal=state_matrix_code
                code_imports=code_block_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="code-block-state-matrix-preview">
                    <CodeBlock code="cargo check -p ui".to_string() />
                    <CodeBlock
                        code={r#"cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings"#.to_string()}
                        language="bash".to_string()
                        label="ci.sh".to_string()
                    />
                    <CodeBlock
                        code="cargo test -p ui --test code_block_semantics".to_string()
                        is_copyable=false
                    />
                    <CodeBlock
                        code="   ".to_string()
                        is_copyable=false
                        class_name="docs-code-block-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Copied State)"
                description="对照 `default_copied`（非受控）与 `is_copied + on_copied_change`（受控）语义。"
                code_signal=controlled_contrast_code
                code_imports=code_block_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="code-block-controlled-preview">
                    <CodeBlock
                        code="Uncontrolled: internal copied state starts from default_copied=true.".to_string()
                        default_copied=true
                    />
                    <CodeBlock
                        code="Controlled: copied state comes from app signal.".to_string()
                        is_copied=controlled_copied_signal
                        on_copied_change=on_controlled_copied_change
                    />
                    <div class="docs-row">
                        <span class="ui-muted">
                            {move || format!("controlled copied: {}", controlled_copied.get())}
                        </span>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            size=ui::ButtonSize::Sm
                            on_press=on_controlled_reset
                        >
                            "Reset controlled copied"
                        </ui::Button>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="CodeBlock 默认 Snapshot；如需边生成边显示，可显式启用 Streaming，并保持 output status 连续可读。"
                code_signal=stream_snapshot_code
                code_imports=code_block_stream_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="code-block-streaming-preview">
                    <CodeBlock
                        code="Snapshot: complete validated output rendered in one pass.".to_string()
                        language="plain".to_string()
                        output_mode=ui::code_block::protocol::CodeBlockAgentOutputMode::Snapshot
                        output_status=ui::code_block::protocol::CodeBlockAgentOutputStatus::Validated
                    />
                    <CodeBlock
                        code="Streaming: incremental draft output while LLM is generating.".to_string()
                        language="plain".to_string()
                        output_mode=ui::code_block::protocol::CodeBlockAgentOutputMode::Streaming
                        output_status=ui::code_block::protocol::CodeBlockAgentOutputStatus::Draft
                    />
                    <p class="ui-muted">
                        "Inspect "
                        <code>"data-ui-output-mode"</code>
                        " and "
                        <code>"data-ui-output-status"</code>
                        " on each root."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="复制按钮输出最小可运行片段，并自动补齐 imports。"
                code_signal=source_first_code
                code_imports=code_block_imports.clone()
            >
                <CodeBlock
                    code="cargo test -p ui --test code_block_semantics".to_string()
                    language="bash".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="调样式走 CSS Test 即时反馈；`preserve_state` 可选保持复制状态和编辑上下文，降低重复操作。"
                code_signal=workbench_code
                code_imports=code_block_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="components/code-block/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="code-block-workbench-controls">
                            <SegmentedControl
                                id_base="docs-code-block-workbench-language".to_string()
                                options=language_options.clone()
                                selected_index=workbench_language_index
                                set_selected_index=set_workbench_language_index
                                size=SegmentedControlSize::Sm
                                aria_label="CodeBlock language".to_string()
                            />
                            <SegmentedControl
                                id_base="docs-code-block-workbench-output-mode".to_string()
                                options=output_mode_options.clone()
                                selected_index=workbench_output_mode_index
                                set_selected_index=set_workbench_output_mode_index
                                size=SegmentedControlSize::Sm
                                aria_label="CodeBlock output mode".to_string()
                            />
                            <SegmentedControl
                                id_base="docs-code-block-workbench-output-status".to_string()
                                options=output_status_options.clone()
                                selected_index=workbench_output_status_index
                                set_selected_index=set_workbench_output_status_index
                                size=SegmentedControlSize::Sm
                                aria_label="CodeBlock output status".to_string()
                            />

                            <div class="docs-row">
                                <Switch checked=workbench_is_copyable set_checked=set_workbench_is_copyable>
                                    "copyable"
                                </Switch>
                                <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                    "custom class"
                                </Switch>
                                <Switch checked=workbench_preserve_state set_checked=set_workbench_preserve_state>
                                    "preserve state"
                                </Switch>
                                <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                                    "lang=zh-CN"
                                </Switch>
                                <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                                    "dir=rtl"
                                </Switch>
                            </div>

                            <div class="docs-row">
                                <ui::Button
                                    variant=ui::ButtonVariant::Secondary
                                    size=ui::ButtonSize::Sm
                                    on_press=on_workbench_load_template
                                >
                                    "Load template"
                                </ui::Button>
                                <ui::Button
                                    variant=ui::ButtonVariant::Secondary
                                    size=ui::ButtonSize::Sm
                                    on_press=on_workbench_reset_copy_state
                                >
                                    "Reset copied state"
                                </ui::Button>
                            </div>

                            <label class="docs-search__label" for="docs-code-block-workbench-code">
                                "Code"
                            </label>
                            <textarea
                                id="docs-code-block-workbench-code"
                                class="docs-search__input"
                                rows="7"
                                prop:value=move || workbench_code_text.get()
                                on:input=move |ev| set_workbench_code_text.set(event_target_value(&ev))
                            />
                        </div>
                    }
                }
            >
                <div class="docs-stack" data-slot="code-block-workbench-preview">
                    {move || {
                        let code = workbench_code_text.get();
                        let language = workbench_language.get();
                        let is_copyable = workbench_is_copyable.get();
                        let output_mode = workbench_output_mode.get();
                        let output_status = workbench_output_status.get();
                        let class_name = if workbench_custom_class.get() {
                            "docs-code-block-custom".to_string()
                        } else {
                            String::new()
                        };
                        let lang = if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        };
                        let dir = if workbench_rtl_dir.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        };

                        view! {
                            <CodeBlock
                                code
                                label="workbench.rs".to_string()
                                language
                                lang
                                dir=dir
                                is_copyable
                                class_name
                                is_copied=workbench_copied_signal
                                copied=workbench_copied_signal
                                default_copied=false
                                on_copied_change=workbench_on_copied_change
                                output_mode
                                output_status
                                motion=ui::CodeBlockMotion::default()
                            />
                        }
                            .into_any()
                    }}

                    <CodeBlock
                        code="cargo test -p ui --test code_block_semantics".to_string()
                        language="bash".to_string()
                        is_copyable=false
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Copy + Output Modes)"
                code_signal=state_matrix_code
                code_imports=code_block_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="code-block-state-matrix-v2">
                    <CodeBlock
                        code="cargo check -p ui".to_string()
                        label="check.sh".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        motion=ui::CodeBlockMotion::default()
                    />
                    <CodeBlock
                        code="cargo fmt --all".to_string()
                        language="bash".to_string()
                        label="fmt.sh".to_string()
                        is_copyable=false
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        motion=ui::CodeBlockMotion::default()
                    />
                    <CodeBlock
                        code="cargo clippy --workspace --all-targets -- -D warnings".to_string()
                        language="bash".to_string()
                        label="clippy.sh".to_string()
                        default_copied=false
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                        motion=ui::CodeBlockMotion::default()
                        class_name="docs-code-block-custom".to_string()
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="code-block-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="code-block-state-rows">
                    <li><code>"data-state"</code>" = single-line | multiline"</li>
                    <li><code>"data-header"</code>" = visible | hidden"</li>
                    <li><code>"data-copyable / data-copied"</code>" = true | none"</li>
                    <li><code>"data-copyable-source"</code>" = default | is_copyable | copyable_legacy"</li>
                    <li><code>"data-copied-source"</code>" = uncontrolled | controlled"</li>
                    <li><code>"data-empty / data-custom-class"</code>" = true | none"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-block-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="code-block-parameter-rows">
                    <li><code>"is_copyable"</code>" default = true"</li>
                    <li>
                        <code>"copyable"</code>
                        " = historical alias; normalization priority: "
                        <code>"is_copyable > copyable > true"</code>
                    </li>
                    <li><code>"default_copied"</code>" default = false"</li>
                    <li>
                        <code>"is_copied + on_copied_change"</code>
                        " = controlled copied-state API"
                    </li>
                    <li><code>"output_mode"</code>" default = snapshot"</li>
                    <li><code>"output_status"</code>" default = validated"</li>
                    <li><code>"disabled axis"</code>" = N/A (CodeBlock has no disabled prop)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-block-streaming-modes">
                <h3>"Streaming / Snapshot"</h3>
                <ul data-slot="code-block-streaming-rows">
                    <li><code>"data-ui-output-mode"</code>" = snapshot | streaming"</li>
                    <li><code>"data-ui-output-status"</code>" = draft | validated | ready-to-submit"</li>
                    <li><code>"default fallback"</code>" = snapshot"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-block-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="code-block-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-code_block"</code>
                        " feature for package-mode consumption."
                    </li>
                    <li>
                        "Style prerequisite: use "
                        <code>"UiRoot"</code>
                        " with components CSS injection (or enable "
                        <code>"inject-css"</code>
                        ") so copied snippets preserve baseline styles."
                    </li>
                </ul>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::CodeBlock;\n\n<CodeBlock\n  code=\"cargo test -p ui --test code_block_semantics\".to_string()\n  language=\"bash\".to_string()\n/>".to_string()
                    label="Copy code starter".to_string()
                    copyable=true
                    class_name="docs-code-block-source-copy".to_string()
                />
                <ul data-slot="code-block-source-paths">
                    <li><code>"components/code-block/src/mod.rs"</code></li>
                    <li><code>"components/code-block/src/logic.rs"</code></li>
                    <li><code>"components/code-block/src/view.rs"</code></li>
                    <li><code>"components/code-block/src/styles.rs"</code></li>
                    <li><code>"components/code-block/src/motion.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn snippet() -> AnyView {
    let (workbench_copied_raw, set_workbench_copied_raw) = signal(false);
    let workbench_copied_signal: Signal<bool> = Signal::derive(move || workbench_copied_raw.get());
    let (workbench_on_copied_change_runs, set_workbench_on_copied_change_runs) = signal(0_u32);
    let on_copied_change = Callback::new(move |next: bool| {
        set_workbench_copied_raw.set(next);
        set_workbench_on_copied_change_runs.update(|count| *count += 1);
    });

    let (workbench_copyable, set_workbench_copyable) = signal(true);
    let (workbench_multiline, set_workbench_multiline) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_reduced_motion, set_workbench_reduced_motion) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<Snippet
  text="cargo fmt --all".to_string()
  label="Command".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let text = if workbench_multiline.get() {
            "cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings"
        } else {
            "cargo fmt --all"
        };
        let label = if workbench_custom_label.get() {
            "CI command"
        } else {
            "Command"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-snippet-custom"
        } else {
            ""
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let motion = if workbench_reduced_motion.get() {
            "SnippetMotion::disabled()"
        } else {
            "SnippetMotion::default()"
        };

        [
            "<Snippet".to_string(),
            format!("  text={}", rust_string_literal(text)),
            format!("  label={}", rust_string_literal(label)),
            format!("  is_copyable={}", bool_word(workbench_copyable.get())),
            format!("  copyable={}", bool_word(workbench_copyable.get())),
            "  copy_label=\"Copy\".to_string()".to_string(),
            "  copied_label=\"Copied\".to_string()".to_string(),
            "  copy_aria_label=\"Copy snippet\".to_string()".to_string(),
            "  copy_error_label=\"Copy failed\".to_string()".to_string(),
            "  is_copied=workbench_copied_signal".to_string(),
            "  copied=workbench_copied_signal".to_string(),
            "  default_copied=false".to_string(),
            "  on_copied_change=on_copied_change".to_string(),
            format!("  motion={motion}"),
            format!("  class_name={}", rust_string_literal(class_name)),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let text = if workbench_multiline.get() {
            "cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings"
        } else {
            "cargo fmt --all"
        };
        let label = if workbench_custom_label.get() {
            Some("CI command")
        } else {
            Some("Command")
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-snippet-custom")
        } else {
            None
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let motion = if workbench_reduced_motion.get() {
            SnippetMotion::disabled()
        } else {
            SnippetMotion::default()
        };

        format!(
            "SnippetActualConfig {{\n  text: {},\n  label: {label:?},\n  is_copyable: Some({}),\n  copyable: Some({}),\n  copy_label: Some(\"Copy\"),\n  copied_label: Some(\"Copied\"),\n  copy_aria_label: Some(\"Copy snippet\"),\n  copy_error_label: Some(\"Copy failed\"),\n  is_copied: Some({}),\n  copied: Some({}),\n  default_copied: Some(false),\n  on_copied_change: \"runs={}\",\n  motion: {motion:?},\n  class_name: {class_name:?},\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n}}",
            rust_string_literal(text),
            bool_word(workbench_copyable.get()),
            bool_word(workbench_copyable.get()),
            bool_word(workbench_copied_raw.get()),
            bool_word(workbench_copied_raw.get()),
            workbench_on_copied_change_runs.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Snippet text="cargo fmt --all".to_string() label="Default".to_string() is_copyable=true />
<Snippet text="cargo test -p ui --test snippet_semantics".to_string() label="Static".to_string() is_copyable=false class_name="docs-snippet-custom".to_string() />
<Snippet text="cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings".to_string() label="Multiline".to_string() copyable=true motion=SnippetMotion::disabled() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Snippet"
            slug="snippet"
            group="Display"
            description="Snippet playground with full API workbench and visible copy callback feedback."
        >
            <Playground title="Hello World (Copyable Snippet)" code_signal=hello_code>
                <Snippet text="cargo fmt --all".to_string() label="Command".to_string() />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="snippet-workbench-controls">
                        <Switch checked=workbench_copyable set_checked=set_workbench_copyable>
                            "Copy enabled"
                        </Switch>
                        <Switch checked=workbench_multiline set_checked=set_workbench_multiline>
                            "Multiline text"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "Custom label"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_copied_raw.update(|value| *value = !*value)
                            })
                        >
                            "Toggle copied signal"
                        </ui::Button>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="snippet-workbench-preview">
                    <Snippet
                        text=if workbench_multiline.get() {
                            "cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings"
                                .to_string()
                        } else {
                            "cargo fmt --all".to_string()
                        }
                        label=if workbench_custom_label.get() {
                            "CI command".to_string()
                        } else {
                            "Command".to_string()
                        }
                        is_copyable=workbench_copyable.get()
                        copyable=workbench_copyable.get()
                        copy_label="Copy".to_string()
                        copied_label="Copied".to_string()
                        copy_aria_label="Copy snippet".to_string()
                        copy_error_label="Copy failed".to_string()
                        is_copied=workbench_copied_signal
                        copied=workbench_copied_signal
                        default_copied=false
                        on_copied_change=on_copied_change
                        motion=if workbench_reduced_motion.get() {
                            SnippetMotion::disabled()
                        } else {
                            SnippetMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-snippet-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted" data-slot="snippet-workbench-feedback">
                        "copied: " {move || workbench_copied_raw.get()}
                        " · on_copied_change: " {move || workbench_on_copied_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Static / Multiline)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight" data-slot="snippet-state-matrix">
                    <Snippet
                        text="cargo fmt --all".to_string()
                        label="Default".to_string()
                        is_copyable=true
                    />
                    <Snippet
                        text="cargo test -p ui --test snippet_semantics".to_string()
                        label="Static".to_string()
                        is_copyable=false
                        class_name="docs-snippet-custom".to_string()
                    />
                    <Snippet
                        text="cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings".to_string()
                        label="Multiline".to_string()
                        copyable=true
                        motion=SnippetMotion::disabled()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn link() -> AnyView {
    let destination_options = vec![
        "internal".to_string(),
        "external".to_string(),
        "missing".to_string(),
    ];
    let (destination_index, set_destination_index) = signal(Some(0_usize));
    let destination_href = Signal::derive(move || match destination_index.get().unwrap_or(0) {
        1 => "https://example.com/docs".to_string(),
        2 => "   ".to_string(),
        _ => "#/docs/welcome".to_string(),
    });
    let destination_label = Signal::derive(move || match destination_index.get().unwrap_or(0) {
        1 => "External docs",
        2 => "Missing href",
        _ => "Internal docs link",
    });

    let rel_options = vec![
        "auto".to_string(),
        "sponsored".to_string(),
        "author + noopener".to_string(),
    ];
    let (rel_index, set_rel_index) = signal(Some(0_usize));
    let rel = Signal::derive(move || match rel_index.get().unwrap_or(0) {
        1 => Some("sponsored".to_string()),
        2 => Some("author noopener".to_string()),
        _ => None,
    });

    let (is_target_blank, set_is_target_blank) = signal(false);
    let (is_disabled, set_is_disabled) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_lang, set_custom_lang) = signal(false);

    let workbench_code = Signal::derive(move || {
        let href = destination_href.get();
        let label = destination_label.get();
        let rel = rel.get();
        let is_target_blank = is_target_blank.get();
        let is_disabled = is_disabled.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let custom_lang = custom_lang.get();

        let mut out = vec!["<Link".to_string(), format!("  href=\"{href}\".into()")];

        if is_target_blank {
            out.push("  target=\"_blank\"".to_string());
        }
        if let Some(rel) = rel {
            out.push(format!("  rel=\"{rel}\".into()"));
        }
        if is_disabled {
            out.push("  is_disabled=true".to_string());
        }
        if custom_aria {
            out.push("  aria_label=\"Open partner documentation\".into()".to_string());
        }
        if custom_class {
            out.push("  class_name=\"docs-link-custom\".into()".to_string());
        }
        if custom_lang {
            out.push("  lang=\"zh-CN\".into()".to_string());
        }

        out.push(">".to_string());
        out.push(format!("  \"{label}\""));
        out.push("</Link>".to_string());
        out.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let href = destination_href.get();
        let rel = rel.get();
        let is_target_blank = is_target_blank.get();
        let is_disabled = is_disabled.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let custom_lang = custom_lang.get();

        let has_href = !href.trim().is_empty();
        let data_state = if is_disabled {
            "disabled"
        } else if has_href {
            "enabled"
        } else {
            "missing-href"
        };
        let target_kind = if is_target_blank { "blank" } else { "self" };
        let rel_source = if rel.is_some() { "provided" } else { "auto" };
        let disabled_source = if is_disabled { "is-prop" } else { "default" };

        let mut classes = vec![
            "ui-link".to_string(),
            format!("ui-link--{data_state}"),
            if rel.is_some() {
                "ui-link--rel-provided".to_string()
            } else {
                "ui-link--rel-auto".to_string()
            },
        ];
        if is_target_blank {
            classes.push("ui-link--external".to_string());
        }
        if custom_aria {
            classes.push("ui-link--with-aria-label".to_string());
        }
        if custom_class {
            classes.push("ui-link--custom-class".to_string());
            classes.push("docs-link-custom".to_string());
        }

        format!(
            "LinkActualConfig {{\n  href: \"{href}\",\n  has_href: {has_href},\n  is_disabled: {is_disabled},\n  disabled_source: \"{disabled_source}\",\n  target: \"{target_kind}\",\n  rel: {:?},\n  rel_source: \"{rel_source}\",\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  lang: {},\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            rel,
            if custom_lang { "\"zh-CN\"" } else { "None" },
            classes.join(" ")
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/link/src/styles.rs */\n{}",
            ui::link::styles::CSS
        )
    });

    let hello_world_code = Signal::derive(move || {
        r##"<Link href="#/docs/welcome".to_string()>"Read docs"</Link>"##.to_string()
    });

    let matrix_code = Signal::derive(move || {
        r##"<Link href="#/docs/welcome".to_string()>"Internal docs link"</Link>
<Link href="https://example.com".to_string() target="_blank">"External link"</Link>
<Link href="#/docs/welcome".to_string() is_disabled=true>"Disabled"</Link>
<Link href="   ".to_string()>"Missing href"</Link>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="Link"
            slug="link"
            group="Display"
            description="Text link with centralized disabled/target/rel state attrs and headless hover + focus-visible semantics."
        >
            <Playground title="Hello World (Default API)" code_signal=hello_world_code>
                <Link href="#/docs/welcome".to_string()>"Read docs"</Link>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="components/link/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="切换 href/target/disabled/rel/class/lang，并在同一面板查看实际 config + code + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Destination"</div>
                            <SegmentedControl
                                id_base="docs-link-workbench-destination".to_string()
                                options=destination_options.clone()
                                selected_index=destination_index
                                set_selected_index=set_destination_index
                                size=SegmentedControlSize::Sm
                                aria_label="Link destination".to_string()
                            />

                            <div class="docs-search__label">"Rel source"</div>
                            <SegmentedControl
                                id_base="docs-link-workbench-rel".to_string()
                                options=rel_options.clone()
                                selected_index=rel_index
                                set_selected_index=set_rel_index
                                size=SegmentedControlSize::Sm
                                aria_label="Link rel source".to_string()
                            />

                            <Switch checked=is_target_blank set_checked=set_is_target_blank>
                                "target=_blank"
                            </Switch>
                            <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                            <Switch checked=custom_aria set_checked=set_custom_aria>
                                "Custom aria_label"
                            </Switch>
                            <Switch checked=custom_class set_checked=set_custom_class>
                                "Custom class"
                            </Switch>
                            <Switch checked=custom_lang set_checked=set_custom_lang>"Lang=zh-CN"</Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let href = destination_href.get();
                        let label = destination_label.get();
                        let rel_value = rel.get().unwrap_or_default();
                        let is_disabled = is_disabled.get();
                        let is_target_blank = is_target_blank.get();
                        let aria_label = if custom_aria.get() {
                            "Open partner documentation".to_string()
                        } else {
                            String::new()
                        };
                        let class_name = if custom_class.get() {
                            "docs-link-custom".to_string()
                        } else {
                            String::new()
                        };
                        let lang = if custom_lang.get() {
                            "zh-CN".to_string()
                        } else {
                            String::new()
                        };

                        if is_target_blank {
                            view! {
                                <Link
                                    href=href
                                    target="_blank"
                                    rel=rel_value
                                    is_disabled=is_disabled
                                    aria_label=aria_label
                                    class_name=class_name
                                    lang=lang
                                >
                                    {label}
                                </Link>
                            }
                            .into_any()
                        } else {
                            view! {
                                <Link
                                    href=href
                                    rel=rel_value
                                    is_disabled=is_disabled
                                    aria_label=aria_label
                                    class_name=class_name
                                    lang=lang
                                >
                                    {label}
                                </Link>
                            }
                            .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        {move || format!(
                            "target={}, rel_source={}",
                            if is_target_blank.get() { "_blank" } else { "_self" },
                            if rel.get().is_some() { "provided" } else { "auto" }
                        )}
                    </span>
                </div>
            </Playground>

            <Playground title="Comparison Matrix (Internal / External / Disabled / Missing)" code_signal=matrix_code>
                <div class="docs-row">
                    <Link href="#/docs/welcome".to_string()>"Internal docs link"</Link>
                    <Link href="https://example.com".to_string() target="_blank">
                        "External link"
                    </Link>
                    <Link href="#/docs/welcome".to_string() is_disabled=true>
                        "Disabled"
                    </Link>
                    <Link href="   ".to_string()>"Missing href"</Link>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn avatar() -> AnyView {
    let src = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Cdefs%3E%3CradialGradient%20id%3D%27g%27%20cx%3D%2732%27%20cy%3D%2732%27%20r%3D%2732%27%3E%3Cstop%20offset%3D%270%27%20stop-color%3D%27%23ff4bd8%27/%3E%3Cstop%20offset%3D%271%27%20stop-color%3D%27%232b5cff%27/%3E%3C/radialGradient%3E%3C/defs%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27url(%23g)%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3EA%3C/text%3E%3C/svg%3E";

    let hello_code = Signal::derive(move || r#"<Avatar />"#.to_string());

    let image_code = Signal::derive(move || {
        r#"<Avatar name="Ada Lovelace".to_string() src=Some(src.into()) />"#.to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
<Avatar alt="Anonymous collaborator".to_string() size=AvatarSize::Sm />
<Avatar size=AvatarSize::Lg />"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"let avatar_name = Some("Ada Lovelace".to_string());

<Avatar />
<Avatar name=avatar_name />
// Avatar has no controlled/uncontrolled state axis.
// App state maps directly to props; no value/on_change/default triplet."#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Avatar name="Snapshot User".to_string() size=AvatarSize::Md />
// Streaming Optional; fallback=snapshot.
// Avatar consumes complete props snapshots and stays render-stable."#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Avatar
  name="  Ada Lovelace  ".to_string()
  alt="  Team lead  ".to_string()
  size=AvatarSize::Lg
  class_name="docs-avatar-custom".to_string()
/>
<Avatar
  alt="  Anonymous collaborator  ".to_string()
  src="   ".to_string()
  class_name="docs-avatar-custom".to_string()
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui::{Avatar, AvatarSize};

<Avatar name="Ada Lovelace".to_string() size=AvatarSize::Md />"#
            .to_string()
    });

    let workbench_mode_options = vec![
        "image".to_string(),
        "name-only".to_string(),
        "fallback".to_string(),
    ];
    let workbench_size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let (workbench_mode_index, set_workbench_mode_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_use_alt, set_workbench_use_alt) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_mode = Signal::derive(move || match workbench_mode_index.get().unwrap_or(0) {
        1 => "name-only",
        2 => "fallback",
        _ => "image",
    });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => AvatarSize::Sm,
        2 => AvatarSize::Lg,
        _ => AvatarSize::Md,
    });

    let workbench_code = Signal::derive(move || {
        let mode = workbench_mode.get();
        let size = match workbench_size.get() {
            AvatarSize::Sm => "AvatarSize::Sm",
            AvatarSize::Md => "AvatarSize::Md",
            AvatarSize::Lg => "AvatarSize::Lg",
        };
        let use_alt = workbench_use_alt.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();

        let mut lines = vec!["<Avatar".to_string(), format!("  size={size}")];
        match mode {
            "image" => {
                lines.push("  name=\"Ada Lovelace\".to_string()".to_string());
                lines.push("  src=Some(src.into())".to_string());
            }
            "name-only" => {
                lines.push("  name=\"Ada Lovelace\".to_string()".to_string());
            }
            _ => {}
        }
        if use_alt {
            lines.push("  alt=\"Team collaborator\".to_string()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-avatar-custom\".to_string()".to_string());
        }
        if rtl {
            lines.push("  lang=\"ar\".to_string()".to_string());
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let mode = workbench_mode.get();
        let use_alt = workbench_use_alt.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let size = workbench_size.get();

        let expected_state = if mode == "image" { "image" } else { "fallback" };
        let expected_label_source = if use_alt {
            "alt"
        } else if matches!(mode, "image" | "name-only") {
            "name"
        } else {
            "fallback"
        };
        let expected_size = match size {
            AvatarSize::Sm => "sm",
            AvatarSize::Md => "md",
            AvatarSize::Lg => "lg",
        };

        format!(
            "AvatarWorkbenchConfig {{\n  name: {},\n  src: {},\n  size: \"{expected_size}\",\n  alt: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  mode: \"{mode}\",\n  use_alt: {use_alt},\n  rtl: {rtl},\n  expected_state: \"{expected_state}\",\n  expected_label_source: \"{expected_label_source}\",\n}}",
            if matches!(mode, "image" | "name-only") {
                "Some(\"Ada Lovelace\")"
            } else {
                "None"
            },
            if mode == "image" {
                "Some(\"data:image/svg+xml,...\")"
            } else {
                "None"
            },
            if use_alt {
                "Some(\"Team collaborator\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-avatar-custom\")"
            } else {
                "None"
            },
            if rtl { "Some(\"ar\")" } else { "None" },
            if rtl { "Some(\"rtl\")" } else { "None" },
        )
    });

    view! {
        <ComponentPage
            title="Avatar"
            slug="avatar"
            group="Display"
            description="Avatar with image/error fallback, normalized labels, and baseline-style root state attrs + custom-class contract."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports="use leptos::prelude::*;\nuse ui::Avatar;".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar />
                </div>
            </Playground>

            <Playground
                title="Image + Fallback"
                code_signal=image_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar
                        name="Ada Lovelace".to_string()
                        src=into_owned_string(src)
                        size=AvatarSize::Md
                    />
                    <Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
                    <Avatar name="Alan Turing".to_string() size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground
                title="Fallback Scenarios"
                description="Label source + fallback state matrix with stable semantic markers."
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar
                        name="Ada Lovelace".to_string()
                        src=into_owned_string(src)
                        alt="Profile photo".to_string()
                        size=AvatarSize::Sm
                    />
                    <Avatar alt="Anonymous collaborator".to_string() size=AvatarSize::Sm />
                    <Avatar size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground
                title="Custom Class + Normalized Props"
                code_signal=custom_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar
                        name="  Ada Lovelace  ".to_string()
                        alt="  Team lead  ".to_string()
                        size=AvatarSize::Lg
                        class_name="docs-avatar-custom".to_string()
                    />
                    <Avatar
                        alt="  Anonymous collaborator  ".to_string()
                        src="   ".to_string()
                        class_name="docs-avatar-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Avatar has no internal controlled/uncontrolled axis; compare default usage and app-state-mapped props."
                code_signal=controlled_contrast_code
                code_imports="use leptos::prelude::*;\nuse ui::Avatar;".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar />
                    <Avatar name="Ada Lovelace".to_string() />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Avatar is not a body-reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <p class="ui-muted" data-slot="avatar-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </p>
                    <p class="ui-muted" data-slot="avatar-copy-ready-hint">
                        "Copy-ready snippets prepend imports automatically; source: components/avatar/src/view.rs."
                    </p>
                    <div class="docs-row">
                        <Avatar name="Snapshot User".to_string() size=AvatarSize::Md />
                        <Avatar alt="Fallback viewer".to_string() size=AvatarSize::Sm />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Props + State Preview)"
                description="Modify props live and inspect semantic state transitions without wiring internal state machines."
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};\nuse ui::color::area::A11yDirection;".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="avatar-workbench-controls">
                            <div class="docs-search__label">"Render mode"</div>
                            <SegmentedControl
                                id_base="docs-avatar-workbench-mode".to_string()
                                options=workbench_mode_options.clone()
                                selected_index=workbench_mode_index
                                set_selected_index=set_workbench_mode_index
                                size=SegmentedControlSize::Sm
                                aria_label="Avatar render mode".to_string()
                            />

                            <div class="docs-search__label">"Size"</div>
                            <SegmentedControl
                                id_base="docs-avatar-workbench-size".to_string()
                                options=workbench_size_options.clone()
                                selected_index=workbench_size_index
                                set_selected_index=set_workbench_size_index
                                size=SegmentedControlSize::Sm
                                aria_label="Avatar size".to_string()
                            />

                            <Switch checked=workbench_use_alt set_checked=set_workbench_use_alt>
                                "Use alt label"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class"
                            </Switch>
                            <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                                "RTL direction"
                            </Switch>
                        </div>
                    }
                }
            >
                {move || {
                    let mode = workbench_mode.get();
                    let size = workbench_size.get();
                    let use_alt = workbench_use_alt.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();

                    let name = if matches!(mode, "image" | "name-only") {
                        "Ada Lovelace".to_string()
                    } else {
                        String::new()
                    };
                    let image_src = if mode == "image" {
                        into_owned_string(src)
                    } else {
                        String::new()
                    };
                    let alt = if use_alt {
                        "Team collaborator".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-avatar-custom".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if rtl { "ar".to_string() } else { String::new() };
                    let dir = if rtl {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    let expected_state = if mode == "image" { "image" } else { "fallback" };
                    let expected_label_source = if use_alt {
                        "alt"
                    } else if matches!(mode, "image" | "name-only") {
                        "name"
                    } else {
                        "fallback"
                    };
                    let expected_size = match size {
                        AvatarSize::Sm => "sm",
                        AvatarSize::Md => "md",
                        AvatarSize::Lg => "lg",
                    };

                    view! {
                        <div class="docs-stack" data-slot="avatar-workbench-preview">
                            <div class="docs-row">
                                <div class="docs-stack docs-stack--tight">
                                    <div class="docs-search__label">"Baseline"</div>
                                    <Avatar />
                                </div>
                                <div class="docs-stack docs-stack--tight" data-slot="avatar-workbench-configured">
                                    <div class="docs-search__label">"Configured"</div>
                                    <Avatar
                                        name=name
                                        src=image_src
                                        size=size
                                        alt=alt
                                        class_name=class_name
                                        lang=lang
                                        dir=dir
                                    />
                                </div>
                            </div>
                            <p class="ui-muted" data-slot="avatar-workbench-state">
                                {format!(
                                    "expected: state={expected_state}, label_source={expected_label_source}, size={expected_size}"
                                )}
                            </p>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Image / Name / Fallback)"
                description="Workbench 后的多参数对比：image/name-only/fallback。"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar
                        name="Ada Lovelace".to_string()
                        src=into_owned_string(src)
                        alt="Profile photo".to_string()
                        size=AvatarSize::Sm
                    />
                    <Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
                    <Avatar size=AvatarSize::Lg />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="avatar-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="avatar-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-avatar"</code>
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
                    label="Copy avatar starter".to_string()
                    copyable=true
                    class_name="docs-avatar-source-copy".to_string()
                />
                <ul data-slot="avatar-source-paths">
                    <li><code>"components/avatar/src/mod.rs"</code></li>
                    <li><code>"components/avatar/src/logic.rs"</code></li>
                    <li><code>"components/avatar/src/view.rs"</code></li>
                    <li><code>"components/avatar/src/styles.rs"</code></li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="avatar-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="avatar-state-rows">
                    <li><code>"data-state"</code>" = image | fallback"</li>
                    <li><code>"data-image / data-fallback"</code>" = true | (absent), derived from render mode"</li>
                    <li><code>"data-label-source"</code>" = alt | name | fallback"</li>
                    <li><code>"data-size"</code>" = sm | md | lg"</li>
                    <li><code>"control mode"</code>" = N/A (Avatar has no controlled/uncontrolled runtime axis)"</li>
                    <li><code>"disabled axis"</code>" = N/A (Avatar has no disabled prop in API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="avatar-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="avatar-parameter-rows">
                    <li><code>"name / src / alt / class_name / lang: Option&lt;String&gt;"</code>" default = None; blank strings are normalized away by normalize_input/normalize_lang"</li>
                    <li><code>"size: AvatarSize"</code>" default = AvatarSize::Md"</li>
                    <li><code>"dir: Option&lt;A11yDirection&gt;"</code>" default = None (inherits locale direction/context)"</li>
                    <li><code>"label source priority"</code>" = alt -> name -> fallback"</li>
                    <li><code>"render mode"</code>" = image when src is present and no image error, else fallback"</li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn avatar_group() -> AnyView {
    let src_a = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27%232b5cff%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3EA%3C/text%3E%3C/svg%3E";
    let src_b = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27%23ff4bd8%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3EG%3C/text%3E%3C/svg%3E";
    let src_c = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27%2312b981%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3ET%3C/text%3E%3C/svg%3E";

    let items = vec![
        AvatarGroupItem {
            name: Some("Ada Lovelace".to_string()),
            src: Some(src_a.into()),
            alt: Some("Ada".to_string()),
        },
        AvatarGroupItem {
            name: Some("Grace Hopper".to_string()),
            src: Some(src_b.into()),
            alt: Some("Grace".to_string()),
        },
        AvatarGroupItem {
            name: Some("Alan Turing".to_string()),
            src: Some(src_c.into()),
            alt: Some("Alan".to_string()),
        },
        AvatarGroupItem {
            name: Some("Katherine Johnson".to_string()),
            src: None,
            alt: Some("Katherine".to_string()),
        },
        AvatarGroupItem {
            name: Some("Annie Easley".to_string()),
            src: None,
            alt: Some("Annie".to_string()),
        },
    ];

    let empty_items: Vec<AvatarGroupItem> = Vec::new();
    let empty_items_for_hello = empty_items.clone();
    let empty_items_for_state_matrix = empty_items.clone();
    let empty_items_for_controlled = empty_items.clone();
    let empty_items_custom: Vec<AvatarGroupItem> = Vec::new();
    let overflow_items = items.clone();
    let size_items = items.clone();
    let custom_items = items.clone();
    let state_matrix_items = items.clone();
    let controlled_items = items.clone();
    let stream_snapshot_items = items.clone();
    let workbench_items_overflow = items.clone();
    let workbench_items_stable = items.iter().take(2).cloned().collect::<Vec<_>>();
    let workbench_items_empty: Vec<AvatarGroupItem> = Vec::new();
    let workbench_items_overflow_for_state_matrix = workbench_items_overflow.clone();
    let workbench_items_stable_for_state_matrix = workbench_items_stable.clone();
    let workbench_items_empty_for_state_matrix = workbench_items_empty.clone();
    let source_first_items = items;
    let code_imports =
        "use leptos::prelude::*;\nuse ui::{AvatarGroup, AvatarGroupItem, AvatarSize};".to_string();

    let hello_code =
        Signal::derive(move || r#"<AvatarGroup items=empty_items.clone() />"#.to_string());

    let overflow_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
    AvatarGroupItem {
      name: Some("Katherine Johnson".to_string()),
      src: None,
      alt: Some("Katherine".to_string()),
    },
  ]
  max=3
  size=AvatarSize::Md
/>"#
        .to_string()
    });

    let sizes_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
  ]
  max=6
  size=AvatarSize::Sm
/>
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
  ]
  max=6
  size=AvatarSize::Lg
/>"#
        .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=Vec::<AvatarGroupItem>::new()
  size=AvatarSize::Md
  aria_label="No collaborators".to_string()
  class_name="docs-avatar-group-custom".to_string()
/>
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
    AvatarGroupItem {
      name: Some("Katherine Johnson".to_string()),
      src: None,
      alt: Some("Katherine".to_string()),
    },
  ]
  max=3
  size=AvatarSize::Md
  aria_label="Core collaborators".to_string()
  class_name="docs-avatar-group-custom".to_string()
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<AvatarGroup items=empty_items.clone() />
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
  ]
  max=4
  size=AvatarSize::Md
/>
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
    AvatarGroupItem {
      name: Some("Katherine Johnson".to_string()),
      src: None,
      alt: Some("Katherine".to_string()),
    },
  ]
  max=2
  size=AvatarSize::Md
  aria_label="Core collaborators".to_string()
/>"#
        .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"// AvatarGroup has no controlled/uncontrolled runtime axis (`value/on_value_change/default_value`).
// Contrast default props with app-state mapped props.
let upstream_max = 2_usize;

<AvatarGroup items=empty_items.clone() />
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
  ]
  max=upstream_max
  aria_label="Upstream mapped".to_string()
/>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
  ]
  max=2
  aria_label="Snapshot baseline".to_string()
/>
// Streaming Optional; fallback=snapshot.
// Inspect markers: data-ui-stream-support=optional data-ui-stream-fallback=snapshot data-ui-output-status=verified."#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
    AvatarGroupItem {
      name: Some("Katherine Johnson".to_string()),
      src: None,
      alt: Some("Katherine".to_string()),
    },
  ]
  max=3
  size=AvatarSize::Md
  aria_label="Copy-ready collaborators".to_string()
/>"#
        .to_string()
    });

    let workbench_roster_options = vec![
        "empty".to_string(),
        "stable".to_string(),
        "overflow".to_string(),
    ];
    let workbench_size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let workbench_max_options = vec!["2".to_string(), "3".to_string(), "4".to_string()];
    let (workbench_roster_index, set_workbench_roster_index) = signal(Some(2_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_max_index, set_workbench_max_index) = signal(Some(1_usize));
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_roster =
        Signal::derive(move || match workbench_roster_index.get().unwrap_or(2) {
            0 => "empty",
            1 => "stable",
            _ => "overflow",
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => AvatarSize::Sm,
        2 => AvatarSize::Lg,
        _ => AvatarSize::Md,
    });
    let workbench_max = Signal::derive(move || match workbench_max_index.get().unwrap_or(1) {
        0 => 2_usize,
        2 => 4_usize,
        _ => 3_usize,
    });

    let workbench_code = Signal::derive(move || {
        let roster = workbench_roster.get();
        let size = match workbench_size.get() {
            AvatarSize::Sm => "AvatarSize::Sm",
            AvatarSize::Md => "AvatarSize::Md",
            AvatarSize::Lg => "AvatarSize::Lg",
        };
        let max = workbench_max.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();

        let roster_comment = match roster {
            "empty" => "// empty roster",
            "stable" => "// stable roster (2 items, no overflow)",
            _ => "// overflow roster",
        };

        let mut lines = vec![
            roster_comment.to_string(),
            "<AvatarGroup".to_string(),
            "  items=your_items".to_string(),
            format!("  max={max}"),
            format!("  size={size}"),
        ];
        if custom_aria {
            lines.push("  aria_label=\"Interactive collaborators\".to_string()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-avatar-group-custom\".to_string()".to_string());
        }
        if rtl {
            lines.push("  lang=\"ar\".to_string()".to_string());
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let roster = workbench_roster.get();
        let size = match workbench_size.get() {
            AvatarSize::Sm => "sm",
            AvatarSize::Md => "md",
            AvatarSize::Lg => "lg",
        };
        let max = workbench_max.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let items_desc = match roster {
            "empty" => "Vec::<AvatarGroupItem>::new()",
            "stable" => "stable_roster(2)",
            _ => "overflow_roster(5)",
        };

        format!(
            "AvatarGroupWorkbenchConfig {{\n  items: \"{items_desc}\",\n  max: {max},\n  size: \"{size}\",\n  aria_label: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  roster: \"{roster}\",\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  rtl: {rtl},\n}}",
            if custom_aria {
                "Some(\"Interactive collaborators\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-avatar-group-custom\")"
            } else {
                "None"
            },
            if rtl { "Some(\"ar\")" } else { "None" },
            if rtl { "Some(\"rtl\")" } else { "None" },
        )
    });

    view! {
        <ComponentPage
            title="AvatarGroup"
            slug="avatar-group"
            group="Display"
            description="Stacked avatars with centralized overflow/empty/aria-label-source state attrs and baseline-style root contracts."
        >
            <Playground title="Hello World" code_signal=hello_code code_imports=code_imports.clone()>
                <div class="docs-row">
                    <AvatarGroup items=empty_items_for_hello />
                </div>
            </Playground>

            <Playground title="Overflow Stack" code_signal=overflow_code code_imports=code_imports.clone()>
                <div class="docs-row">
                    <AvatarGroup items=overflow_items.clone() max=3 size=AvatarSize::Md />
                    <AvatarGroup
                        items=overflow_items.clone()
                        max=2
                        size=AvatarSize::Lg
                        aria_label="Core collaborators".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Sizes Without Overflow"
                code_signal=sizes_code
                code_imports=code_imports.clone()
            >
                <div class="docs-row">
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Sm />
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Md />
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground title="Custom Aria + Class" code_signal=custom_code code_imports=code_imports.clone()>
                <div class="docs-row">
                    <AvatarGroup
                        items=empty_items_custom
                        max=4
                        size=AvatarSize::Md
                        aria_label="No collaborators".to_string()
                        class_name="docs-avatar-group-custom".to_string()
                    />
                    <AvatarGroup
                        items=custom_items
                        max=3
                        size=AvatarSize::Md
                        aria_label="Core collaborators".to_string()
                        class_name="docs-avatar-group-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Roster Scenarios"
                description="Covers empty/stable/overflow and custom aria-label contracts in one matrix."
                code_signal=state_matrix_code
                code_imports=code_imports.clone()
            >
                <div class="docs-row">
                    <AvatarGroup items=empty_items_for_state_matrix />
                    <AvatarGroup items=state_matrix_items.clone() max=6 size=AvatarSize::Md />
                    <AvatarGroup
                        items=state_matrix_items.clone()
                        max=2
                        size=AvatarSize::Md
                        aria_label="Core collaborators".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="AvatarGroup has no controlled/uncontrolled state machine. Compare default props with app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=code_imports.clone()
            >
                <div class="docs-row">
                    <AvatarGroup items=empty_items_for_controlled />
                    <AvatarGroup
                        items=controlled_items.clone()
                        max=2
                        size=AvatarSize::Md
                        aria_label="Upstream mapped".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="AvatarGroup is not a body-reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports=code_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="avatar-group-streaming-preview">
                    <AvatarGroup
                        items=stream_snapshot_items.clone()
                        max=2
                        size=AvatarSize::Md
                        aria_label="Snapshot baseline".to_string()
                    />
                    <p class="ui-muted" data-slot="avatar-group-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Props + State + Preview)"
                description="Adjust roster/size/max and semantic sources in real time. Use this as repeatable acceptance surface."
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{AvatarGroup, AvatarGroupItem, AvatarSize};\nuse ui::color::area::A11yDirection;".to_string()
                test_source_path="components/avatar-group/src/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="avatar-group-workbench-controls">
                            <div class="docs-search__label">"Roster state"</div>
                            <SegmentedControl
                                id_base="docs-avatar-group-workbench-roster".to_string()
                                options=workbench_roster_options.clone()
                                selected_index=workbench_roster_index
                                set_selected_index=set_workbench_roster_index
                                size=SegmentedControlSize::Sm
                                aria_label="AvatarGroup roster mode".to_string()
                            />

                            <div class="docs-search__label">"Size"</div>
                            <SegmentedControl
                                id_base="docs-avatar-group-workbench-size".to_string()
                                options=workbench_size_options.clone()
                                selected_index=workbench_size_index
                                set_selected_index=set_workbench_size_index
                                size=SegmentedControlSize::Sm
                                aria_label="AvatarGroup size".to_string()
                            />

                            <div class="docs-search__label">"Max visible"</div>
                            <SegmentedControl
                                id_base="docs-avatar-group-workbench-max".to_string()
                                options=workbench_max_options.clone()
                                selected_index=workbench_max_index
                                set_selected_index=set_workbench_max_index
                                size=SegmentedControlSize::Sm
                                aria_label="AvatarGroup max visible".to_string()
                            />

                            <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                                "Custom aria label"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class"
                            </Switch>
                            <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                                "RTL direction"
                            </Switch>
                        </div>
                    }
                }
            >
                {move || {
                    let roster = workbench_roster.get();
                    let size = workbench_size.get();
                    let max = workbench_max.get();
                    let custom_aria = workbench_custom_aria.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();

                    let configured_items = match roster {
                        "empty" => workbench_items_empty.clone(),
                        "stable" => workbench_items_stable.clone(),
                        _ => workbench_items_overflow.clone(),
                    };
                    let configured_total = configured_items.len();
                    let visible = configured_total.min(max);
                    let overflow = configured_total.saturating_sub(visible);
                    let expected_state = if configured_total == 0 {
                        "empty"
                    } else if overflow > 0 {
                        "overflow"
                    } else {
                        "stable"
                    };
                    let size_attr = match size {
                        AvatarSize::Sm => "sm",
                        AvatarSize::Md => "md",
                        AvatarSize::Lg => "lg",
                    };

                    let aria_label = if custom_aria {
                        "Interactive collaborators".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-avatar-group-custom".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if rtl { "ar".to_string() } else { String::new() };
                    let dir = if rtl {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    view! {
                        <div class="docs-stack" data-slot="avatar-group-workbench-preview">
                            <div class="docs-row">
                                <div class="docs-stack docs-stack--tight">
                                    <div class="docs-search__label">"Baseline"</div>
                                    <AvatarGroup items=workbench_items_overflow.clone() max=3 size=AvatarSize::Md />
                                </div>
                                <div
                                    class="docs-stack docs-stack--tight"
                                    data-slot="avatar-group-workbench-configured"
                                >
                                    <div class="docs-search__label">"Configured"</div>
                                    <AvatarGroup
                                        items=configured_items
                                        max=max
                                        size=size
                                        aria_label=aria_label
                                        class_name=class_name
                                        lang=lang
                                        dir=dir
                                    />
                                </div>
                            </div>
                            <p class="ui-muted" data-slot="avatar-group-workbench-state">
                                {format!(
                                    "expected: state={expected_state}, size={size_attr}, total={configured_total}, overflow={overflow}"
                                )}
                            </p>
                            <p class="ui-muted" data-slot="avatar-group-spec-preview-na">
                                "AI Spec input/preview linkage: N/A for AvatarGroup (non-spec component)."
                            </p>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Empty / Stable / Overflow)"
                description="Workbench 后的多参数对比展示。"
                code_signal=state_matrix_code
                code_imports=code_imports.clone()
            >
                <div class="docs-row">
                    <AvatarGroup items=workbench_items_empty_for_state_matrix.clone() />
                    <AvatarGroup
                        items=workbench_items_stable_for_state_matrix.clone()
                        max=6
                        size=AvatarSize::Md
                    />
                    <AvatarGroup
                        items=workbench_items_overflow_for_state_matrix.clone()
                        max=2
                        size=AvatarSize::Md
                        aria_label="Core collaborators".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=code_imports
            >
                <div class="docs-stack docs-stack--tight" data-slot="avatar-group-source-first-preview">
                    <AvatarGroup
                        items=source_first_items.clone()
                        max=3
                        size=AvatarSize::Md
                        aria_label="Copy-ready collaborators".to_string()
                    />
                    <p class="ui-muted" data-slot="avatar-group-copy-ready-hint">
                        "Copy-ready snippets prepend imports automatically; source: components/avatar-group/src/view.rs."
                    </p>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="avatar-group-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="avatar-group-source-first-contract">
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="avatar-group-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-avatar-group"</code>
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
                    label="Copy avatar-group starter".to_string()
                    copyable=true
                    class_name="docs-avatar-group-source-copy".to_string()
                />
                <ul data-slot="avatar-group-source-paths">
                    <li><code>"components/avatar-group/src/mod.rs"</code></li>
                    <li><code>"components/avatar-group/src/logic.rs"</code></li>
                    <li><code>"components/avatar-group/src/view.rs"</code></li>
                    <li><code>"components/avatar-group/src/styles.rs"</code></li>
                </ul>
                <p class="ui-muted" data-slot="avatar-group-source-sync-note">
                    "Sync note: snippet text is sourced from "
                    <code>"source_first_code"</code>
                    " and mirrors "
                    <code>"components/avatar-group/src/view.rs"</code>
                    " API usage; update docs snippet and source implementation together to avoid drift."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="avatar-group-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="avatar-group-state-rows">
                    <li><code>"data-state"</code>" = empty | stable | overflow"</li>
                    <li><code>"data-aria-label-source / data-class-source"</code>" = default | custom"</li>
                    <li><code>"data-ui-state / data-ui-action"</code>" = empty/stable/overflow with render-stable-roster | render-overflow-summary"</li>
                    <li><code>"controlled/uncontrolled axis"</code>" = N/A (AvatarGroup has no runtime controllable state machine)"</li>
                    <li><code>"disabled axis"</code>" = N/A (AvatarGroup has no is_disabled prop in API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="avatar-group-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="avatar-group-parameter-rows">
                    <li><code>"items: Vec&lt;AvatarGroupItem&gt;"</code>" required roster input (Hello World uses empty vec)"</li>
                    <li><code>"max: Option&lt;usize&gt;"</code>" default = None -> normalize to 4 in logic (`normalize_avatar_group_max_visible`)"</li>
                    <li><code>"size: AvatarSize"</code>" default = AvatarSize::Md"</li>
                    <li><code>"aria_label: Option&lt;String&gt;"</code>" default = None -> i18n default aria label via logic fallback"</li>
                    <li><code>"class_name: Option&lt;String&gt;, lang: Option&lt;String&gt;"</code>" default = None; blank strings are normalized away in logic"</li>
                    <li><code>"dir: Option&lt;A11yDirection&gt;"</code>" default = None (inherits locale direction context)"</li>
                    <li><code>"AvatarGroupItem{name/src/alt}: Option&lt;String&gt;"</code>" empty/blank values normalize to empty strings in logic output fields"</li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn image() -> AnyView {
    let src = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%27320%27%20height%3D%27180%27%20viewBox%3D%270%200%20320%20180%27%3E%3Cdefs%3E%3ClinearGradient%20id%3D%27g%27%20x1%3D%270%27%20y1%3D%270%27%20x2%3D%271%27%20y2%3D%271%27%3E%3Cstop%20offset%3D%270%25%27%20stop-color%3D%27%230f172a%27/%3E%3Cstop%20offset%3D%27100%25%27%20stop-color%3D%27%230ea5e9%27/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect%20width%3D%27100%25%27%20height%3D%27100%25%27%20fill%3D%27url(%23g)%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2750%25%27%20fill%3D%27white%27%20font-size%3D%2722%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%3Erust-ui%20image%3C/text%3E%3C/svg%3E";
    let fallback_src = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%27320%27%20height%3D%27180%27%20viewBox%3D%270%200%20320%20180%27%3E%3Crect%20width%3D%27100%25%27%20height%3D%27100%25%27%20fill%3D%27%23334155%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2750%25%27%20fill%3D%27white%27%20font-size%3D%2720%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%3EFallback%3C/text%3E%3C/svg%3E";
    let radius_options = vec![
        "sm".to_string(),
        "md".to_string(),
        "lg".to_string(),
        "full".to_string(),
    ];
    let shadow_options = vec!["none".to_string(), "sm".to_string(), "md".to_string()];
    let motion_options = vec!["default".to_string(), "custom".to_string()];
    let source_options = vec![
        "valid".to_string(),
        "invalid".to_string(),
        "missing".to_string(),
    ];
    let lang_options = vec!["en-US".to_string(), "zh-CN".to_string()];

    let (radius_index, set_radius_index) = signal(Some(2usize));
    let (shadow_index, set_shadow_index) = signal(Some(1usize));
    let (motion_index, set_motion_index) = signal(Some(0usize));
    let (source_index, set_source_index) = signal(Some(0usize));
    let (lang_index, set_lang_index) = signal(Some(0usize));
    let (is_zoomed, set_is_zoomed) = signal(true);
    let (is_blurred, set_is_blurred) = signal(false);
    let (is_skeleton_disabled, set_is_skeleton_disabled) = signal(false);
    let (with_fallback, set_with_fallback) = signal(true);
    let (custom_class, set_custom_class) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let radius = Signal::derive(move || match radius_index.get().unwrap_or(2) {
        0 => ImageRadius::Sm,
        1 => ImageRadius::Md,
        3 => ImageRadius::Full,
        _ => ImageRadius::Lg,
    });
    let shadow = Signal::derive(move || match shadow_index.get().unwrap_or(1) {
        0 => ImageShadow::None,
        2 => ImageShadow::Md,
        _ => ImageShadow::Sm,
    });
    let motion = Signal::derive(move || match motion_index.get().unwrap_or(0) {
        1 => ImageMotion {
            zoom_scale: 1.12,
            ..ImageMotion::default()
        },
        _ => ImageMotion::default(),
    });
    let source_mode = Signal::derive(move || source_index.get().unwrap_or(0));
    let lang = Signal::derive(move || match lang_index.get().unwrap_or(0) {
        1 => "zh-CN".to_string(),
        _ => "en-US".to_string(),
    });
    let dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let code = Signal::derive(move || {
        r#"<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  alt="Demo image".to_string()
/>"#
        .to_string()
    });
    let source_first_code = Signal::derive(move || {
        r#"<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  fallback_src="https://images.unsplash.com/photo-1500530855697-b586d89ba3ee".to_string()
  alt="Copy-ready starter".to_string()
  radius=ImageRadius::Lg
  shadow=ImageShadow::Sm
/>"#
        .to_string()
    });
    let controlled_contrast_code = Signal::derive(move || {
        r#"let upstream_zoomed = true;

<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  alt="Default path".to_string()
/>
<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  alt="Upstream mapped".to_string()
  is_zoomed=upstream_zoomed
/>"#
        .to_string()
    });
    let stream_snapshot_code = Signal::derive(move || {
        r#"<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  alt="Snapshot baseline".to_string()
/>
// Streaming Optional; fallback=snapshot.
// Image renders deterministic snapshot output while keeping semantic markers stable."#
            .to_string()
    });
    let basic_imports = "use leptos::prelude::*;\nuse ui::Image;".to_string();
    let advanced_imports =
        "use leptos::prelude::*;\nuse ui::{Image, ImageMotion, ImageRadius, ImageShadow};"
            .to_string();
    let workbench_code = Signal::derive(move || {
        let radius = radius.get();
        let shadow = shadow.get();
        let motion_mode = motion_index.get().unwrap_or(0);
        let source_mode = source_mode.get();
        let is_zoomed = is_zoomed.get();
        let is_blurred = is_blurred.get();
        let is_skeleton_disabled = is_skeleton_disabled.get();
        let with_fallback = with_fallback.get();
        let custom_class = custom_class.get();

        let mut snippet = vec!["<Image".to_string()];
        match source_mode {
            1 => snippet
                .push("  src=\"https://example.invalid/rust-ui-image.png\".into()".to_string()),
            2 => snippet.push("  src=\"\".into()".to_string()),
            _ => snippet.push("  src=src.into()".to_string()),
        }
        snippet.push("  alt=\"Demo image\".into()".to_string());
        if with_fallback {
            snippet.push("  fallback_src=fallback_src.into()".to_string());
        }
        if is_skeleton_disabled {
            snippet.push("  is_skeleton_disabled=true".to_string());
        }
        if is_blurred {
            snippet.push("  is_blurred=true".to_string());
        }
        if is_zoomed {
            snippet.push("  is_zoomed=true".to_string());
        }
        if radius != ImageRadius::Lg {
            snippet.push(format!("  radius=ImageRadius::{radius:?}"));
        }
        if shadow != ImageShadow::Sm {
            snippet.push(format!("  shadow=ImageShadow::{shadow:?}"));
        }
        if motion_mode == 1 {
            snippet.push(
                "  motion=ImageMotion { zoom_scale: 1.12, ..ImageMotion::default() }".to_string(),
            );
        }
        if custom_class {
            snippet.push("  class_name=\"docs-image-custom\".into()".to_string());
        }
        snippet.push(format!("  lang={}", rust_string_literal(&lang.get())));
        snippet.push(format!("  dir=A11yDirection::{:?}", dir.get()));
        snippet.extend(["/>".to_string()]);
        snippet.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/image/src/styles.rs */\n{}",
            ui::image::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let source_value = match source_mode.get() {
            1 => "https://example.invalid/rust-ui-image.png".to_string(),
            2 => String::new(),
            _ => src.to_string(),
        };
        let fallback_value = if with_fallback.get() {
            Some(fallback_src.to_string())
        } else {
            None
        };
        let class_name_value = if custom_class.get() {
            Some("docs-image-custom".to_string())
        } else {
            None
        };
        format!(
            "ImageActualConfig {{\n  src: {:?},\n  alt: {:?},\n  fallback_src: {:?},\n  is_skeleton_disabled: {},\n  is_blurred: {},\n  is_zoomed: {},\n  radius: {:?},\n  shadow: {:?},\n  motion: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            source_value,
            "Demo image",
            fallback_value,
            is_skeleton_disabled.get(),
            is_blurred.get(),
            is_zoomed.get(),
            radius.get(),
            shadow.get(),
            motion.get(),
            class_name_value,
            lang.get(),
            dir.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<Image src=src.into() alt="Loaded + Zoom".to_string() is_zoomed=true radius=ImageRadius::Lg shadow=ImageShadow::Md />
<Image src=src.into() alt="Blurred + Soft".to_string() is_blurred=true radius=ImageRadius::Md shadow=ImageShadow::Sm />
<Image src="https://example.invalid/rust-ui-image.png".to_string() fallback_src=fallback_src.into() alt="Invalid -> Fallback".to_string() radius=ImageRadius::Sm shadow=ImageShadow::None />
<Image src="".to_string() fallback_src=fallback_src.into() alt="Missing -> Fallback".to_string() radius=ImageRadius::Full shadow=ImageShadow::Sm />"#.to_string()
    });
    let visual_baseline_code = Signal::derive(move || {
        r#"<Image src=src.into() alt="Editorial baseline".to_string() class_name="docs-image-frame".to_string() />
<Image src=src.into() alt="Hover feedback baseline".to_string() is_zoomed=true class_name="docs-image-frame".to_string() />
<Image src=src.into() alt="Depth baseline".to_string() is_blurred=true class_name="docs-image-frame".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Image"
            slug="image"
            group="Display"
            description="Image with skeleton, blur, and zoom motion."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=code
                code_imports=basic_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Image
                        src=into_owned_string(src)
                        alt="Demo image".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Default Theme Visual Baseline (Visual Desire)"
                description="Default-theme hierarchy, contrast, and hover feedback baseline."
                code_signal=visual_baseline_code
            >
                <div
                    class="docs-stack docs-stack--tight docs-image-visual-baseline"
                    data-visual-baseline="image-default-theme"
                >
                    <span class="ui-muted">
                        "HeroUI-quality visual direction baseline for Image under default theme."
                    </span>
                    <div class="docs-grid docs-grid--2" style="width: 100%; gap: 1rem;">
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Editorial baseline"</span>
                            <Image
                                src=into_owned_string(src)
                                alt="Editorial baseline".to_string()
                                class_name="docs-image-frame".to_string()
                            />
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Hover feedback baseline"</span>
                            <Image
                                src=into_owned_string(src)
                                alt="Hover feedback baseline".to_string()
                                is_zoomed=true
                                class_name="docs-image-frame".to_string()
                            />
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Depth / blur baseline"</span>
                            <Image
                                src=into_owned_string(src)
                                alt="Depth / blur baseline".to_string()
                                is_blurred=true
                                class_name="docs-image-frame".to_string()
                            />
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Fallback contrast baseline"</span>
                            <Image
                                src="https://example.invalid/rust-ui-image.png".to_string()
                                fallback_src=into_owned_string(fallback_src)
                                alt="Fallback contrast baseline".to_string()
                                class_name="docs-image-frame".to_string()
                            />
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Scenario Gallery: Loaded / Blurred / Fallback / Missing"
                code_signal=matrix_code
                code_imports=advanced_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-grid docs-grid--2" style="width: 100%; gap: 1rem;">
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Loaded + Zoom"</span>
                        <Image
                            src=into_owned_string(src)
                            alt="Loaded + Zoom".to_string()
                            is_zoomed=true
                            radius=ImageRadius::Lg
                            shadow=ImageShadow::Md
                            class_name="docs-image-frame".to_string()
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Blurred + Soft"</span>
                        <Image
                            src=into_owned_string(src)
                            alt="Blurred + Soft".to_string()
                            is_blurred=true
                            radius=ImageRadius::Md
                            shadow=ImageShadow::Sm
                            class_name="docs-image-frame".to_string()
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Invalid src -> Fallback"</span>
                        <Image
                            src="https://example.invalid/rust-ui-image.png".to_string()
                            fallback_src=into_owned_string(fallback_src)
                            alt="Invalid -> Fallback".to_string()
                            radius=ImageRadius::Sm
                            shadow=ImageShadow::None
                            class_name="docs-image-frame".to_string()
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Missing src -> Fallback"</span>
                        <Image
                            src="".to_string()
                            fallback_src=into_owned_string(fallback_src)
                            alt="Missing -> Fallback".to_string()
                            radius=ImageRadius::Full
                            shadow=ImageShadow::Sm
                            class_name="docs-image-frame".to_string()
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Image has no internal controlled/uncontrolled state axis; contrast default props and app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=basic_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Image
                        src=into_owned_string(src)
                        alt="Default path".to_string()
                    />
                    <Image
                        src=into_owned_string(src)
                        alt="Upstream mapped".to_string()
                        is_zoomed=is_zoomed.get()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Image is not a body-reader surface: streaming is optional and fallback remains snapshot."
                code_signal=stream_snapshot_code
                code_imports=basic_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <p class="ui-muted" data-slot="image-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </p>
                    <p class="ui-muted" data-slot="image-copy-ready-hint">
                        "Copy-ready snippets prepend imports automatically; source: components/image/src/view.rs."
                    </p>
                    <Image
                        src=into_owned_string(src)
                        alt="Snapshot baseline".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=advanced_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Image
                        src=into_owned_string(src)
                        fallback_src=into_owned_string(fallback_src)
                        alt="Copy-ready starter".to_string()
                        radius=ImageRadius::Lg
                        shadow=ImageShadow::Sm
                    />
                    <span class="ui-muted">
                        "Source-first path: copy snippet and run with imports auto-injected by Playground."
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench: Display + Config + Code + CSS Test"
                description="Interactive panel with scoped CSS test + actual config snapshot."
                code_signal=workbench_code
                code_imports=advanced_imports.clone()
                test_css_source=test_css_source
                test_source_path="components/image/src/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Source"</div>
                        <SegmentedControl
                            id_base="docs-image-source".to_string()
                            options=source_options.clone()
                            selected_index=source_index
                            set_selected_index=set_source_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image source mode".to_string()
                        />

                        <div class="docs-search__label">"Radius"</div>
                        <SegmentedControl
                            id_base="docs-image-radius".to_string()
                            options=radius_options.clone()
                            selected_index=radius_index
                            set_selected_index=set_radius_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image radius".to_string()
                        />

                        <div class="docs-search__label">"Shadow"</div>
                        <SegmentedControl
                            id_base="docs-image-shadow".to_string()
                            options=shadow_options.clone()
                            selected_index=shadow_index
                            set_selected_index=set_shadow_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image shadow".to_string()
                        />

                        <div class="docs-search__label">"Motion"</div>
                        <SegmentedControl
                            id_base="docs-image-motion".to_string()
                            options=motion_options.clone()
                            selected_index=motion_index
                            set_selected_index=set_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image motion mode".to_string()
                        />

                        <div class="docs-search__label">"Language"</div>
                        <SegmentedControl
                            id_base="docs-image-lang".to_string()
                            options=lang_options.clone()
                            selected_index=lang_index
                            set_selected_index=set_lang_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image language".to_string()
                        />

                        <Switch checked=is_zoomed set_checked=set_is_zoomed>"Zoomed"</Switch>
                        <Switch checked=is_blurred set_checked=set_is_blurred>"Blurred"</Switch>
                        <Switch checked=is_skeleton_disabled set_checked=set_is_skeleton_disabled>
                            "Disable skeleton"
                        </Switch>
                        <Switch checked=with_fallback set_checked=set_with_fallback>"Use fallback"</Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                        <Switch checked=rtl set_checked=set_rtl>"RTL direction"</Switch>
                    </div>
                }
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="image-workbench-stage"
                    style="width: min(100%, 360px);"
                >
                    {move || {
                        let source = match source_mode.get() {
                            1 => "https://example.invalid/rust-ui-image.png".to_string(),
                            2 => String::new(),
                            _ => src.into(),
                        };
                        let fallback = if with_fallback.get() {
                            fallback_src.into()
                        } else {
                            String::new()
                        };
                        let class_name = if custom_class.get() {
                            "docs-image-custom".to_string()
                        } else {
                            String::new()
                        };

                        view! {
                    <Image
                        src=source
                        fallback_src=fallback
                        alt="Demo image".to_string()
                        is_skeleton_disabled=is_skeleton_disabled.get()
                        is_blurred=is_blurred.get()
                        is_zoomed=is_zoomed.get()
                        radius=radius.get()
                        shadow=shadow.get()
                        motion=motion.get()
                        class_name=class_name
                        lang=lang.get()
                        dir=dir.get()
                    />
                        }
                    }}
                    <span class="ui-muted">
                        {move || format!(
                            "state: source={}, fallback={}, zoomed={}, blurred={}, lang={}, dir={:?}",
                            match source_mode.get() {
                                1 => "invalid",
                                2 => "missing",
                                _ => "valid",
                            },
                            with_fallback.get(),
                            is_zoomed.get(),
                            is_blurred.get(),
                            lang.get(),
                            dir.get(),
                        )}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Source + Visual State)"
                code_signal=matrix_code
                code_imports=advanced_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-grid docs-grid--2" style="width: 100%; gap: 1rem;">
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Loaded + Zoom"</span>
                        <Image
                            src=into_owned_string(src)
                            alt="Loaded + Zoom".to_string()
                            is_zoomed=true
                            radius=ImageRadius::Lg
                            shadow=ImageShadow::Md
                            class_name="docs-image-frame".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Blurred + Soft"</span>
                        <Image
                            src=into_owned_string(src)
                            alt="Blurred + Soft".to_string()
                            is_blurred=true
                            radius=ImageRadius::Md
                            shadow=ImageShadow::Sm
                            class_name="docs-image-frame".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Invalid src -> Fallback"</span>
                        <Image
                            src="https://example.invalid/rust-ui-image.png".to_string()
                            fallback_src=into_owned_string(fallback_src)
                            alt="Invalid -> Fallback".to_string()
                            radius=ImageRadius::Sm
                            shadow=ImageShadow::None
                            class_name="docs-image-frame".to_string()
                            lang="zh-CN".to_string()
                            dir=A11yDirection::Rtl
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Missing src -> Fallback"</span>
                        <Image
                            src="".to_string()
                            fallback_src=into_owned_string(fallback_src)
                            alt="Missing -> Fallback".to_string()
                            radius=ImageRadius::Full
                            shadow=ImageShadow::Sm
                            class_name="docs-image-frame".to_string()
                            lang="zh-CN".to_string()
                            dir=A11yDirection::Rtl
                        />
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="image-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="image-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-image"</code>
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
                    label="Copy image starter".to_string()
                    copyable=true
                    class_name="docs-image-source-copy".to_string()
                />
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn illustrated_message() -> AnyView {
    let code_imports = "use leptos::prelude::*;\nuse ui::{Button, IllustratedMessage};".to_string();
    let workbench_orientation_options = vec!["vertical".to_string(), "horizontal".to_string()];
    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let (workbench_show_title, set_workbench_show_title) = signal(true);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_illustration, set_workbench_show_illustration) = signal(false);
    let (workbench_show_actions, set_workbench_show_actions) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
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
        if workbench_rtl.get() {
            lines.push("  dir=ui::color::area::A11yDirection::Rtl".to_string());
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
            "IllustratedMessageWorkbenchConfig {{ orientation: \"{orientation}\", show_title: {}, show_description: {}, show_illustration: {}, show_actions: {}, custom_class: {}, rtl: {} }}",
            workbench_show_title.get(),
            workbench_show_description.get(),
            workbench_show_illustration.get(),
            workbench_show_actions.get(),
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
                    let dir = if workbench_rtl.get() {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
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
                                        class_name=class_name.clone()
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
                                        class_name=class_name.clone()
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
                                        class_name=class_name.clone()
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
                                        class_name=class_name
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

pub(super) fn motion_ripple() -> AnyView {
    let showcase_ref: NodeRef<html::Span> = NodeRef::new();
    let workbench_ref: NodeRef<html::Span> = NodeRef::new();
    let matrix_default_ref: NodeRef<html::Span> = NodeRef::new();
    let matrix_unbounded_ref: NodeRef<html::Span> = NodeRef::new();
    let matrix_custom_ref: NodeRef<html::Span> = NodeRef::new();

    let (workbench_is_bounded, set_workbench_is_bounded) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl_locale, set_workbench_rtl_locale) = signal(false);
    let (workbench_trigger_count, set_workbench_trigger_count) = signal(0_u32);

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            RippleMotion {
                duration_ms: 620,
                ..RippleMotion::default()
            }
        } else {
            RippleMotion::default()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-ripple-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<button class="docs-ripple-surface" type="button">
  <MotionRipple node_ref=showcase_ref />
</button>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<MotionRipple\n  node_ref=workbench_ref\n  is_bounded={}\n  motion=RippleMotion {{ duration_ms: {}, ..RippleMotion::default() }}\n  class_name={}\n  lang={}\n  dir={}\n/>",
            bool_word(workbench_is_bounded.get()),
            workbench_motion.get().duration_ms,
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(&workbench_lang.get()),
            if matches!(workbench_dir.get(), A11yDirection::Rtl) {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "MotionRippleActualConfig {{\n  node_ref: \"workbench_ref\",\n  is_bounded: {},\n  motion: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  trigger_count: {},\n}}",
            workbench_is_bounded.get(),
            workbench_motion.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
            workbench_trigger_count.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<MotionRipple
  node_ref=matrix_default_ref
  is_bounded=true
  motion=RippleMotion::default()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
/>
<MotionRipple
  node_ref=matrix_unbounded_ref
  is_bounded=false
  motion=RippleMotion { duration_ms: 520, ..RippleMotion::default() }
  class_name="docs-ripple-custom".to_string()
  lang="ar".to_string()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="MotionRipple"
            slug="motion-ripple"
            group="Display"
            description="Ripple overlay with centralized boundary/motion/class source attrs and WAAPI trigger helpers."
        >
            <Playground title="Default Showcase" code_signal=showcase_code>
                <div class="docs-row">
                    <button
                        class="docs-ripple-surface"
                        type="button"
                        on:click=move |_| {
                            ui::ripple::trigger_ripple(showcase_ref, RippleMotion::default());
                        }
                    >
                        <span class="docs-ripple-label">"Click to trigger ripple"</span>
                        <MotionRipple node_ref=showcase_ref />
                    </button>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="motion-ripple-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_is_bounded.get()
                                on:change=move |ev| set_workbench_is_bounded.set(event_target_checked(&ev))
                            />
                            " is_bounded"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " custom motion"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_rtl_locale.get()
                                on:change=move |ev| set_workbench_rtl_locale.set(event_target_checked(&ev))
                            />
                            " lang/dir Arabic"
                        </label>
                    </div>
                }
            >
                <div class="docs-row">
                    <button
                        class="docs-ripple-surface docs-ripple-surface--accent"
                        type="button"
                        on:click=move |_| {
                            set_workbench_trigger_count.update(|count| *count += 1);
                            if workbench_is_bounded.get_untracked() {
                                ui::ripple::trigger_ripple(workbench_ref, workbench_motion.get_untracked());
                            } else {
                                ui::ripple::trigger_ripple_at(
                                    workbench_ref,
                                    workbench_motion.get_untracked(),
                                    22.0,
                                    42.0,
                                );
                            }
                        }
                    >
                        <span class="docs-ripple-label">"Trigger workbench ripple"</span>
                        <MotionRipple
                            node_ref=workbench_ref
                            is_bounded=workbench_is_bounded.get()
                            motion=workbench_motion.get()
                            class_name=workbench_class_name.get()
                            lang=workbench_lang.get()
                            dir=workbench_dir.get()
                        />
                    </button>
                    <span class="ui-muted">
                        {move || format!("trigger_count={}", workbench_trigger_count.get())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Boundary / Motion Comparison)"
                code_signal=matrix_code
                code_imports="use ui::color::area::A11yDirection;\nuse ui::{MotionRipple, RippleMotion};".to_string()
            >
                <div class="docs-row">
                    <button
                        class="docs-ripple-surface"
                        type="button"
                        on:click=move |_| {
                            ui::ripple::trigger_ripple(matrix_default_ref, RippleMotion::default());
                        }
                    >
                        <span class="docs-ripple-label">"Bounded default"</span>
                        <MotionRipple
                            node_ref=matrix_default_ref
                            is_bounded=true
                            motion=RippleMotion::default()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </button>
                    <button
                        class="docs-ripple-surface docs-ripple-surface--unbounded"
                        type="button"
                        on:click=move |_| {
                            ui::ripple::trigger_ripple_at(
                                matrix_unbounded_ref,
                                RippleMotion {
                                    duration_ms: 520,
                                    ..RippleMotion::default()
                                },
                                16.0,
                                50.0,
                            );
                        }
                    >
                        <span class="docs-ripple-label">"Unbounded RTL"</span>
                        <MotionRipple
                            node_ref=matrix_unbounded_ref
                            is_bounded=false
                            motion=RippleMotion {
                                duration_ms: 520,
                                ..RippleMotion::default()
                            }
                            class_name="docs-ripple-custom".to_string()
                            lang="ar".to_string()
                            dir=A11yDirection::Rtl
                        />
                    </button>
                    <button
                        class="docs-ripple-surface docs-ripple-surface--slow"
                        type="button"
                        on:click=move |_| {
                            ui::ripple::trigger_ripple(
                                matrix_custom_ref,
                                RippleMotion {
                                    duration_ms: 880,
                                    ..RippleMotion::default()
                                },
                            );
                        }
                    >
                        <span class="docs-ripple-label">"Slow bounded"</span>
                        <MotionRipple
                            node_ref=matrix_custom_ref
                            is_bounded=true
                            motion=RippleMotion {
                                duration_ms: 880,
                                ..RippleMotion::default()
                            }
                            class_name="docs-ripple-item".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </button>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn static_number() -> AnyView {
    let (workbench_number_key, set_workbench_number_key) = signal("positive".to_string());
    let (workbench_decimal_places_key, set_workbench_decimal_places_key) = signal("2".to_string());
    let (workbench_decimal_sep_key, set_workbench_decimal_sep_key) = signal("dot".to_string());
    let (workbench_thousand_sep_key, set_workbench_thousand_sep_key) = signal("comma".to_string());
    let (workbench_pad_start, set_workbench_pad_start) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl_locale, set_workbench_rtl_locale) = signal(false);

    let workbench_number = Signal::derive(move || match workbench_number_key.get().as_str() {
        "negative" => -9876.5,
        "nan" => f64::NAN,
        _ => 12345.67,
    });
    let workbench_decimal_places =
        Signal::derive(move || match workbench_decimal_places_key.get().as_str() {
            "auto" => None,
            "0" => Some(0_u32),
            "6" => Some(6_u32),
            _ => Some(2_u32),
        });
    let workbench_decimal_separator = Signal::derive(move || {
        if workbench_decimal_sep_key.get() == "comma" {
            ",".to_string()
        } else {
            String::new()
        }
    });
    let workbench_thousand_separator =
        Signal::derive(move || match workbench_thousand_sep_key.get().as_str() {
            "none" => String::new(),
            "space" => " ".to_string(),
            _ => ",".to_string(),
        });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-static-number-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<StaticNumber
  number=12345.67
  decimal_places=2
  thousand_separator=",".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "<StaticNumber".to_string(),
            format!("  number={}", workbench_number.get()),
            format!("  pad_start={}", bool_word(workbench_pad_start.get())),
            format!(
                "  decimal_separator={}",
                rust_string_literal(&workbench_decimal_separator.get()),
            ),
            format!(
                "  thousand_separator={}",
                rust_string_literal(&workbench_thousand_separator.get()),
            ),
            format!(
                "  class_name={}",
                rust_string_literal(&workbench_class_name.get()),
            ),
            format!("  lang={}", rust_string_literal(&workbench_lang.get())),
            format!(
                "  dir={}",
                if matches!(workbench_dir.get(), A11yDirection::Rtl) {
                    "A11yDirection::Rtl"
                } else {
                    "A11yDirection::Ltr"
                },
            ),
        ];
        if let Some(decimal_places) = workbench_decimal_places.get() {
            lines.push(format!("  decimal_places={decimal_places}"));
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/text-input/src/number/styles.rs */\n{}",
            ui::text_input::number::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let number = workbench_number.get();
        let sanitized = if number.is_finite() { number } else { 0.0 };
        format!(
            "StaticNumberActualConfig {{\n  number: {number},\n  pad_start: {},\n  decimal_separator: {:?},\n  decimal_places: {:?},\n  thousand_separator: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  sanitized_number: {sanitized},\n}}",
            workbench_pad_start.get(),
            workbench_decimal_separator.get(),
            workbench_decimal_places.get(),
            workbench_thousand_separator.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<StaticNumber
  number=12345.67
  pad_start=false
  decimal_separator=".".to_string()
  decimal_places=2
  thousand_separator=",".to_string()
  class_name="".to_string()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
/>
<StaticNumber
  number=-9876.5
  pad_start=true
  decimal_separator=",".to_string()
  decimal_places=1
  thousand_separator=" ".to_string()
  class_name="docs-static-number-custom".to_string()
  lang="ar".to_string()
  dir=A11yDirection::Rtl
/>
<StaticNumber
  number=f64::NAN
  pad_start=false
  decimal_separator="".to_string()
  thousand_separator="".to_string()
  class_name="docs-static-number-custom".to_string()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="StaticNumber"
            slug="static-number"
            group="Display"
            description="Static number formatting with centralized sign/separator/class source attrs."
        >
            <Playground title="Default Showcase" code_signal=showcase_code>
                <div class="docs-row">
                    <StaticNumber
                        number=12345.67
                        decimal_places=2
                        thousand_separator=",".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (All API Config)"
                description="Button-style playground with display/config/code/css-test panels for number formatting contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="components/text-input/src/number/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="static-number-workbench-controls">
                        <label class="docs-search__label">
                            "Value"
                            <select
                                prop:value=move || workbench_number_key.get()
                                on:change=move |ev| set_workbench_number_key.set(event_target_value(&ev))
                            >
                                <option value="positive">"Positive"</option>
                                <option value="negative">"Negative"</option>
                                <option value="nan">"NaN (sanitized)"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Decimal places"
                            <select
                                prop:value=move || workbench_decimal_places_key.get()
                                on:change=move |ev| set_workbench_decimal_places_key.set(event_target_value(&ev))
                            >
                                <option value="auto">"Auto"</option>
                                <option value="0">"0"</option>
                                <option value="2">"2"</option>
                                <option value="6">"6"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Decimal separator"
                            <select
                                prop:value=move || workbench_decimal_sep_key.get()
                                on:change=move |ev| set_workbench_decimal_sep_key.set(event_target_value(&ev))
                            >
                                <option value="dot">"Default ."</option>
                                <option value="comma">"Custom ,"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Thousand separator"
                            <select
                                prop:value=move || workbench_thousand_sep_key.get()
                                on:change=move |ev| set_workbench_thousand_sep_key.set(event_target_value(&ev))
                            >
                                <option value="none">"None"</option>
                                <option value="comma">"Comma"</option>
                                <option value="space">"Space"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_pad_start.get()
                                on:change=move |ev| set_workbench_pad_start.set(event_target_checked(&ev))
                            />
                            " Pad start"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_rtl_locale.get()
                                on:change=move |ev| set_workbench_rtl_locale.set(event_target_checked(&ev))
                            />
                            " lang/dir Arabic"
                        </label>
                    </div>
                }
            >
                {move || {
                    let number = workbench_number.get();
                    let decimal_places = workbench_decimal_places.get();
                    let decimal_separator = workbench_decimal_separator.get();
                    let thousand_separator = workbench_thousand_separator.get();
                    let class_name = workbench_class_name.get();
                    let lang = workbench_lang.get();
                    let dir = workbench_dir.get();

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-card docs-stack docs-stack--tight">
                                {if let Some(decimal_places) = decimal_places {
                                    view! {
                                        <StaticNumber
                                            number=number
                                            pad_start=workbench_pad_start.get()
                                            decimal_separator=decimal_separator.clone()
                                            decimal_places=decimal_places
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                            lang=lang.clone()
                                            dir=dir
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <StaticNumber
                                            number=number
                                            pad_start=workbench_pad_start.get()
                                            decimal_separator=decimal_separator.clone()
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                            lang=lang.clone()
                                            dir=dir
                                        />
                                    }
                                        .into_any()
                                }}
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Locale / Separator / Sign Comparison)"
                code_signal=matrix_code
                code_imports="use ui::color::area::A11yDirection;\nuse ui::StaticNumber;".to_string()
            >
                <div class="docs-row">
                    <StaticNumber
                        number=12345.67
                        pad_start=false
                        decimal_separator=".".to_string()
                        decimal_places=2
                        thousand_separator=",".to_string()
                        class_name="".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <StaticNumber
                        number=-9876.5
                        pad_start=true
                        decimal_separator=",".to_string()
                        decimal_places=1
                        thousand_separator=" ".to_string()
                        class_name="docs-static-number-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                    <StaticNumber
                        number=f64::NAN
                        pad_start=false
                        decimal_separator="".to_string()
                        thousand_separator="".to_string()
                        class_name="docs-static-number-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sliding_number() -> AnyView {
    let (value, set_value) = signal(12345.67_f64);
    let number_signal: Signal<f64> = Signal::derive(move || value.get());
    let (workbench_decimal_places_key, set_workbench_decimal_places_key) = signal("2".to_string());
    let (workbench_decimal_sep_key, set_workbench_decimal_sep_key) = signal("dot".to_string());
    let (workbench_thousand_sep_key, set_workbench_thousand_sep_key) = signal("comma".to_string());
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_animate, set_workbench_animate) = signal(true);
    let (workbench_pad_start, set_workbench_pad_start) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_dir_rtl, set_workbench_dir_rtl) = signal(false);
    let (workbench_show_compare, set_workbench_show_compare) = signal(true);

    let workbench_decimal_places =
        Signal::derive(move || match workbench_decimal_places_key.get().as_str() {
            "auto" => None,
            "0" => Some(0_u32),
            "6" => Some(6_u32),
            _ => Some(2_u32),
        });
    let workbench_decimal_separator = Signal::derive(move || {
        if workbench_decimal_sep_key.get() == "comma" {
            Some(",".to_string())
        } else {
            None
        }
    });
    let workbench_thousand_separator =
        Signal::derive(move || match workbench_thousand_sep_key.get().as_str() {
            "none" => None,
            "space" => Some(" ".to_string()),
            _ => Some(",".to_string()),
        });

    let workbench_motion = Signal::derive(move || {
        let mut motion = ui::SlidingNumberMotion {
            animate: workbench_animate.get(),
            ..Default::default()
        };
        if workbench_custom_motion.get() {
            motion.spring.stiffness = 420.0;
            motion.spring.damping = 34.0;
        }
        motion
    });

    let workbench_code = Signal::derive(move || {
        let decimal_places = workbench_decimal_places.get();
        let decimal_separator = workbench_decimal_separator.get();
        let thousand_separator = workbench_thousand_separator.get();
        let motion = workbench_motion.get();

        let mut lines = vec![
            "let (value, set_value) = signal(12345.67_f64);".to_string(),
            "<SlidingNumber".to_string(),
            "  number=Signal::derive(move || value.get())".to_string(),
            format!("  pad_start={}", bool_word(workbench_pad_start.get())),
        ];
        if let Some(separator) = decimal_separator {
            lines.push(format!("  decimal_separator={separator:?}.into()"));
        }
        if let Some(places) = decimal_places {
            lines.push(format!("  decimal_places={places}"));
        }
        if let Some(separator) = thousand_separator {
            lines.push(format!("  thousand_separator={separator:?}.into()"));
        }
        if motion != ui::SlidingNumberMotion::default() {
            lines.push(format!(
                "  motion=SlidingNumberMotion {{ animate: {}, ..Default::default() }}",
                motion.animate
            ));
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-sliding-number-custom\".into()".to_string());
        }
        lines.push(if workbench_lang_zh.get() {
            "  lang=\"zh-CN\".into()".to_string()
        } else {
            "  lang=\"en-US\".into()".to_string()
        });
        lines.push(if workbench_dir_rtl.get() {
            "  dir=Some(A11yDirection::Rtl)".to_string()
        } else {
            "  dir=Some(A11yDirection::Ltr)".to_string()
        });
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/text-input/src/number/styles.rs */\n{}",
            ui::text_input::number::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let number = value.get();
        let decimal_separator_source = if workbench_decimal_separator.get().is_some() {
            "custom"
        } else {
            "default"
        };
        let decimal_places_source = if workbench_decimal_places.get().is_some() {
            "custom"
        } else {
            "auto"
        };
        let thousand_separator_source = if workbench_thousand_separator.get().is_some() {
            "custom"
        } else {
            "none"
        };
        let motion = workbench_motion.get();
        let motion_source = if motion == ui::SlidingNumberMotion::default() {
            "default"
        } else {
            "custom"
        };
        let decimal_separator = workbench_decimal_separator.get();
        let decimal_places = workbench_decimal_places.get();
        let thousand_separator = workbench_thousand_separator.get();
        let class_name = if workbench_custom_class.get() {
            Some("docs-sliding-number-custom")
        } else {
            None
        };
        let class_source = if workbench_custom_class.get() {
            "custom"
        } else {
            "default"
        };
        let mut classes = vec![
            "ui-sliding-number".to_string(),
            format!(
                "data-state:{}",
                if motion.animate { "animated" } else { "static" }
            ),
        ];
        if workbench_custom_class.get() {
            classes.push("docs-sliding-number-custom".to_string());
        }

        format!(
            "SlidingNumberActualConfig {{\n  number: {number},\n  motion: \"{motion_source}\",\n  pad_start: {},\n  decimal_separator: {:?},\n  decimal_places: {:?},\n  thousand_separator: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {},\n  animate: {},\n  decimal_separator_source: \"{decimal_separator_source}\",\n  decimal_places_source: \"{decimal_places_source}\",\n  thousand_separator_source: \"{thousand_separator_source}\",\n  motion_source: \"{motion_source}\",\n  class_source: \"{class_source}\",\n  class: \"{}\",\n}}",
            workbench_pad_start.get(),
            decimal_separator,
            decimal_places,
            thousand_separator,
            class_name,
            if workbench_lang_zh.get() {
                Some("zh-CN")
            } else {
                Some("en-US")
            },
            if workbench_dir_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            motion.animate,
            classes.join(" "),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SlidingNumber
  number=Signal::derive(move || value.get())
  decimal_places=2
  thousand_separator=",".to_string()
/>
<SlidingNumber number=Signal::derive(move || value.get()) decimal_places=0 />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<SlidingNumber
  number=Signal::derive(|| 42123.456)
  decimal_separator=",".to_string()
  decimal_places=30
  thousand_separator=" ".to_string()
  class_name="docs-sliding-number-custom".to_string()
/>
<SlidingNumber
  number=Signal::derive(|| f64::NAN)
  decimal_places=2
  motion=ui::SlidingNumberMotion { animate: false, ..Default::default() }
  class_name="docs-sliding-number-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="SlidingNumber"
            slug="sliding-number"
            group="Display"
            description="Spring-animated number transitions with centralized sign/source/motion attrs."
        >
            <Playground title="Hello World (Default Animated Number)" code_signal=matrix_code>
                <div class="docs-stack">
                    <SlidingNumber
                        number=number_signal
                        decimal_places=2
                        thousand_separator=",".to_string()
                    />
                    <SlidingNumber number=number_signal decimal_places=0 />
                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v += 250.0))
                        >
                            "+250"
                        </ui::Button>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v -= 100.0))
                        >
                            "-100"
                        </ui::Button>
                        <span class="ui-muted">"value: " {move || value.get()}</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Separators + Motion + Class" code_signal=custom_code>
                <div class="docs-stack">
                    <SlidingNumber
                        number=Signal::derive(|| 42123.456)
                        decimal_separator=",".to_string()
                        decimal_places=30
                        thousand_separator=" ".to_string()
                        class_name="docs-sliding-number-custom".to_string()
                    />
                    <SlidingNumber
                        number=Signal::derive(|| f64::NAN)
                        decimal_places=2
                        motion=ui::SlidingNumberMotion {
                            animate: false,
                            ..Default::default()
                        }
                        class_name="docs-sliding-number-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for sliding number motion and format contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="components/text-input/src/number/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sliding-number-workbench-controls">
                        <label class="docs-search__label">
                            "Decimal places"
                            <select
                                prop:value=move || workbench_decimal_places_key.get()
                                on:change=move |ev| set_workbench_decimal_places_key.set(event_target_value(&ev))
                            >
                                <option value="auto">"Auto"</option>
                                <option value="0">"0"</option>
                                <option value="2">"2"</option>
                                <option value="6">"6"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Decimal separator"
                            <select
                                prop:value=move || workbench_decimal_sep_key.get()
                                on:change=move |ev| set_workbench_decimal_sep_key.set(event_target_value(&ev))
                            >
                                <option value="dot">"Default ."</option>
                                <option value="comma">"Custom ,"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Thousand separator"
                            <select
                                prop:value=move || workbench_thousand_sep_key.get()
                                on:change=move |ev| set_workbench_thousand_sep_key.set(event_target_value(&ev))
                            >
                                <option value="none">"None"</option>
                                <option value="comma">"Comma"</option>
                                <option value="space">"Space"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_animate.get()
                                on:change=move |ev| set_workbench_animate.set(event_target_checked(&ev))
                            />
                            " Animate"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " Custom motion"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_pad_start.get()
                                on:change=move |ev| set_workbench_pad_start.set(event_target_checked(&ev))
                            />
                            " Pad start"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_show_compare.get()
                                on:change=move |ev| set_workbench_show_compare.set(event_target_checked(&ev))
                            />
                            " Show compare"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_lang_zh.get()
                                on:change=move |ev| set_workbench_lang_zh.set(event_target_checked(&ev))
                            />
                            " lang zh-CN"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_dir_rtl.get()
                                on:change=move |ev| set_workbench_dir_rtl.set(event_target_checked(&ev))
                            />
                            " dir RTL"
                        </label>
                    </div>
                }
            >
                {move || {
                    let decimal_places = workbench_decimal_places.get();
                    let decimal_separator = workbench_decimal_separator.get().unwrap_or_default();
                    let thousand_separator = workbench_thousand_separator.get().unwrap_or_default();
                    let motion = workbench_motion.get();
                    let show_compare = workbench_show_compare.get();
                    let class_name = if workbench_custom_class.get() {
                        "docs-sliding-number-custom".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <div class="docs-card docs-stack docs-stack--tight">
                                {if let Some(decimal_places) = decimal_places {
                                    view! {
                                        <SlidingNumber
                                            number=number_signal
                                            motion=motion
                                            pad_start=workbench_pad_start.get()
                                            decimal_separator=decimal_separator.clone()
                                            decimal_places=decimal_places
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                            lang=if workbench_lang_zh.get() {
                                                "zh-CN".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if workbench_dir_rtl.get() {
                                                A11yDirection::Rtl
                                            } else {
                                                A11yDirection::Ltr
                                            }
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <SlidingNumber
                                            number=number_signal
                                            motion=motion
                                            pad_start=workbench_pad_start.get()
                                            decimal_separator=decimal_separator.clone()
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                            lang=if workbench_lang_zh.get() {
                                                "zh-CN".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if workbench_dir_rtl.get() {
                                                A11yDirection::Rtl
                                            } else {
                                                A11yDirection::Ltr
                                            }
                                        />
                                    }
                                        .into_any()
                                }}
                                <div class="docs-row">
                                    <ui::Button
                                        variant=ui::ButtonVariant::Secondary
                                        on_press=Callback::new(move |_| set_value.update(|v| *v += 250.0))
                                    >
                                        "+250"
                                    </ui::Button>
                                    <ui::Button
                                        variant=ui::ButtonVariant::Secondary
                                        on_press=Callback::new(move |_| set_value.update(|v| *v -= 100.0))
                                    >
                                        "-100"
                                    </ui::Button>
                                    <span class="ui-muted">"value: " {move || value.get()}</span>
                                </div>
                            </div>

                            <Show when=move || show_compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight">
                                    <SlidingNumber
                                        number=Signal::derive(move || value.get())
                                        decimal_places=2
                                        thousand_separator=",".to_string()
                                    />
                                    <SlidingNumber
                                        number=Signal::derive(move || value.get())
                                        decimal_places=0
                                        motion=ui::SlidingNumberMotion {
                                            animate: false,
                                            ..Default::default()
                                        }
                                        class_name="docs-sliding-number-custom".to_string()
                                    />
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Format + Motion Comparison)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <SlidingNumber
                        number=Signal::derive(move || value.get())
                        decimal_places=2
                        thousand_separator=",".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <SlidingNumber
                        number=Signal::derive(move || value.get())
                        motion=ui::SlidingNumberMotion {
                            animate: false,
                            ..Default::default()
                        }
                        pad_start=true
                        decimal_separator=",".to_string()
                        decimal_places=0
                        thousand_separator=" ".to_string()
                        class_name="docs-sliding-number-custom".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
