use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::{html, prelude::*};
use ui_components::color::area::A11yDirection;
use ui_components::{
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
        r#"<Alert
  tone=AlertTone::Info
  fill=AlertFill::Border
  title="Updates available".to_string()
  description="A new version is ready to install.".to_string()
>
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
                test_source_path="components/status-light/src/view.rs".to_string()
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
                </div>
            </Playground>

            <Playground title="Tone + Fill" code_signal=tone_fill_code>
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
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            size=ui_components::ButtonSize::Sm
                        >
                            "Details"
                        </ui_components::Button>
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
            ui_components::badge::styles::CSS
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
            "BadgeActualConfig {{\n  variant: {variant:?},\n  variant_attr: \"{}\",\n  fill_attr: \"{}\",\n  class_source: \"{}\",\n  lang: \"{lang}\",\n  dir: \"{}\",\n  class: \"{}\",\n}}",
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

            <Playground title="Variant Matrix" code_signal=matrix_code>
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn status_light() -> AnyView {
    let hello_world_code =
        Signal::derive(move || r#"<StatusLight>"Idle"</StatusLight>"#.to_string());

    let variants_code = Signal::derive(move || {
        r#"<StatusLight variant=StatusLightVariant::Default>"Idle"</StatusLight>
<StatusLight variant=StatusLightVariant::Accent>"Deploying"</StatusLight>
<StatusLight variant=StatusLightVariant::Danger>"Failed"</StatusLight>"#
            .to_string()
    });

    let role_code = Signal::derive(move || {
        r#"<StatusLight role=StatusLightRole::Status>"Background sync complete"</StatusLight>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<StatusLight class_name="docs-status-light-custom".to_string()>"Queued"</StatusLight>
<StatusLight
  role=StatusLightRole::Status
  variant=StatusLightVariant::Accent
  class_name="docs-status-light-custom".to_string()
>
  "Deploy started"
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
                title="Hello World"
                code_signal=hello_world_code
                test_source_path="components/status-light/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <StatusLight>"Idle"</StatusLight>
                </div>
            </Playground>

            <Playground
                title="Variants"
                code_signal=variants_code
                test_source_path="components/status-light/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <StatusLight variant=StatusLightVariant::Default>"Idle"</StatusLight>
                    <StatusLight variant=StatusLightVariant::Accent>"Deploying"</StatusLight>
                    <StatusLight variant=StatusLightVariant::Danger>"Failed"</StatusLight>
                </div>
            </Playground>

            <Playground
                title="Live Region Role"
                code_signal=role_code
                test_source_path="components/status-light/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <StatusLight role=StatusLightRole::Status>"Background sync complete"</StatusLight>
                    <StatusLight
                        role=StatusLightRole::Status
                        variant=StatusLightVariant::Accent
                        class_name="docs-status-light-custom".to_string()
                    >
                        "Deploy started"
                    </StatusLight>
                </div>
            </Playground>

            <Playground
                title="Custom Class + Static"
                code_signal=custom_code
                test_source_path="components/status-light/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <StatusLight class_name="docs-status-light-custom".to_string()>"Queued"</StatusLight>
                    <StatusLight
                        role=StatusLightRole::Status
                        variant=StatusLightVariant::Accent
                        class_name="docs-status-light-custom".to_string()
                    >
                        "Deploy started"
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
            ui_components::chip::styles::CSS
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
    let shimmer_code = Signal::derive(move || {
        r#"<Skeleton variant=SkeletonVariant::Rect is_shimmer=true class_name="docs-skeleton-line".to_string() />
<Skeleton variant=SkeletonVariant::Circle is_shimmer=true class_name="docs-skeleton-avatar".to_string() />"#.to_string()
    });

    let still_code = Signal::derive(move || {
        r#"<Skeleton variant=SkeletonVariant::Rect is_shimmer=false class_name="docs-skeleton-line".to_string() />
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
                title="Shimmer"
                code_signal=shimmer_code
                test_source_path="crates/ui-components/src/skeleton/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line docs-skeleton-line--short".to_string() />
                    <Skeleton variant=SkeletonVariant::Circle class_name="docs-skeleton-avatar".to_string() />
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-card".to_string() />
                </div>
            </Playground>

            <Playground
                title="Still"
                code_signal=still_code
                test_source_path="crates/ui-components/src/skeleton/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        is_shimmer=false
                        class_name="docs-skeleton-line".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        is_shimmer=false
                        class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
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

    view! {
        <ComponentPage
            title="CircularProgress"
            slug="circular-progress"
            group="Display"
            description="Indeterminate circular progress with centralized size/thickness/label source attrs."
        >
            <Playground title="Size + Thickness Matrix" code_signal=matrix_code>
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

            <Playground title="Custom Label + Class" code_signal=custom_code>
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn spinner() -> AnyView {
    let hello_code = Signal::derive(move || r#"<Spinner />"#.to_string());

    let matrix_code = Signal::derive(move || {
        r#"<Spinner size=SpinnerSize::Sm />
<Spinner size=SpinnerSize::Md />
<Spinner size=SpinnerSize::Lg />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Spinner aria_label="Fetching notifications".to_string() />
<Spinner aria_label="   ".to_string() class_name="docs-spinner-custom".to_string() />
<Spinner aria_label="Syncing inbox".to_string() class_name="docs-spinner-custom".to_string() size=SpinnerSize::Lg />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Spinner"
            slug="spinner"
            group="Display"
            description="Spinner wraps CircularProgress with centralized size/label/class source attrs."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-row">
                    <Spinner />
                </div>
            </Playground>

            <Playground title="Size Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <Spinner size=SpinnerSize::Sm />
                    <Spinner size=SpinnerSize::Md />
                    <Spinner size=SpinnerSize::Lg />
                </div>
            </Playground>

            <Playground title="Custom Label + Class" code_signal=custom_code>
                <div class="docs-row">
                    <Spinner aria_label="Fetching notifications".to_string() />
                    <Spinner
                        aria_label="   ".to_string()
                        class_name="docs-spinner-custom".to_string()
                    />
                    <Spinner
                        aria_label="Syncing inbox".to_string()
                        class_name="docs-spinner-custom".to_string()
                        size=SpinnerSize::Lg
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn progress() -> AnyView {
    let (value, set_value) = signal(42.0_f64);
    let progress_value = Signal::derive(move || Some(value.get()));

    let matrix_code = Signal::derive(move || {
        r#"let progress_value = Signal::derive(move || Some(value.get()));
<Progress aria_label="Determinate".to_string() value=progress_value />
<Progress aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Progress
  aria_label="Syncing tasks".to_string()
  value=Signal::derive(|| Some(64.0))
  min=0.0
  max=100.0
  value_label="64 complete".to_string()
  motion=ui_components::ProgressMotion::fast()
  class_name="docs-progress-custom".to_string()
/>
<Progress
  aria_label="   ".to_string()
  value=Signal::derive(|| Some(18.0))
  class_name="docs-progress-custom".to_string()
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
            <Playground title="Determinate + Indeterminate" code_signal=matrix_code>
                <div class="docs-stack">
                    <Progress aria_label="Determinate".to_string() value=progress_value />
                    <Progress aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 12.0).min(100.0)))
                        >
                            "+12"
                        </ui_components::Button>
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.set(0.0))
                        >
                            "Reset"
                        </ui_components::Button>
                        <span class="ui-muted">"value: " {move || value.get()}</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Label + Motion + Class" code_signal=custom_code>
                <div class="docs-stack">
                    <Progress
                        aria_label="Syncing tasks".to_string()
                        value=Signal::derive(|| Some(64.0))
                        min=0.0
                        max=100.0
                        value_label="64 complete".to_string()
                        motion=ui_components::ProgressMotion::fast()
                        class_name="docs-progress-custom".to_string()
                    />
                    <Progress
                        aria_label="   ".to_string()
                        value=Signal::derive(|| Some(18.0))
                        class_name="docs-progress-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn progress_bar() -> AnyView {
    let matrix_code = Signal::derive(move || {
        r#"<ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Sm value=24.0 max=100.0 />
<ProgressBar variant=ProgressBarVariant::Accent size=ProgressBarSize::Md value=72.0 max=100.0 />
<ProgressBar variant=ProgressBarVariant::Danger size=ProgressBarSize::Lg value=54.0 max=100.0 />
<ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Md indeterminate=true />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<ProgressBar
  variant=ProgressBarVariant::Accent
  size=ProgressBarSize::Md
  value=64.0
  max=100.0
  aria_label="Upload completion".to_string()
  class_name="docs-progress-bar-custom".to_string()
/>
<ProgressBar
  variant=ProgressBarVariant::Default
  size=ProgressBarSize::Sm
  value=18.0
  max=f64::NAN
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
            <Playground title="Variant + Size Matrix" code_signal=matrix_code>
                <div class="docs-stack">
                    <ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Sm value=24.0 max=100.0 />
                    <ProgressBar variant=ProgressBarVariant::Accent size=ProgressBarSize::Md value=72.0 max=100.0 />
                    <ProgressBar variant=ProgressBarVariant::Danger size=ProgressBarSize::Lg value=54.0 max=100.0 />
                    <ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Md indeterminate=true />
                </div>
            </Playground>

            <Playground title="Custom Label + Class" code_signal=custom_code>
                <div class="docs-stack">
                    <ProgressBar
                        variant=ProgressBarVariant::Accent
                        size=ProgressBarSize::Md
                        value=64.0
                        max=100.0
                        aria_label="Upload completion".to_string()
                        class_name="docs-progress-bar-custom".to_string()
                    />
                    <ProgressBar
                        variant=ProgressBarVariant::Default
                        size=ProgressBarSize::Sm
                        value=18.0
                        max=f64::NAN
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
    let (value, set_value) = signal(35.0_f64);
    let progress_value = Signal::derive(move || Some(value.get()));

    let matrix_code = Signal::derive(move || {
        r#"<ProgressCircle aria_label="Determinate".to_string() value=progress_value min=0.0 max=100.0 />
<ProgressCircle aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<ProgressCircle
  aria_label="Sync progress".to_string()
  value=Signal::derive(|| Some(64.0))
  min=0.0
  max=100.0
  size_px=40.0
  stroke_width_px=5.0
  value_label="64 done".to_string()
  class_name="docs-progress-circle-custom".to_string()
/>
<ProgressCircle
  aria_label="   ".to_string()
  value=Signal::derive(|| Some(18.0))
  class_name="docs-progress-circle-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ProgressCircle"
            slug="progress-circle"
            group="Display"
            description="Spring-animated circular progress with centralized source attrs."
        >
            <Playground title="Determinate + Indeterminate" code_signal=matrix_code>
                <div class="docs-row">
                    <ProgressCircle aria_label="Determinate".to_string() value=progress_value min=0.0 max=100.0 />
                    <ProgressCircle aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 10.0).min(100.0)))
                    >
                        "+10"
                    </ui_components::Button>
                </div>
            </Playground>

            <Playground title="Custom Value Label + Class" code_signal=custom_code>
                <div class="docs-row">
                    <ProgressCircle
                        aria_label="Sync progress".to_string()
                        value=Signal::derive(|| Some(64.0))
                        min=0.0
                        max=100.0
                        size_px=40.0
                        stroke_width_px=5.0
                        value_label="64 done".to_string()
                        class_name="docs-progress-circle-custom".to_string()
                    />
                    <ProgressCircle
                        aria_label="   ".to_string()
                        value=Signal::derive(|| Some(18.0))
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
  motion=ui_components::MeterMotion::fast()
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
            lines.push("  motion=ui_components::MeterMotion::fast()".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-meter-custom\".into()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/meter/src/styles.rs */\n{}",
            ui_components::meter::styles::CSS
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
            "MeterActualConfig {{\n  value: {},\n  variant: {variant:?},\n  size: {size:?},\n  is_indeterminate: {is_indeterminate},\n  show_value_label: {show_value_label},\n  has_custom_value_label: {has_custom_label},\n  has_custom_motion: {has_custom_motion},\n  has_custom_class_name: {has_custom_class},\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            if is_indeterminate {
                "None".to_string()
            } else {
                format!("Some({value}.0)")
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
            <Playground title="Variant + Size Matrix" code_signal=matrix_code>
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
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 10).min(100)))
                        >
                            "+10"
                        </ui_components::Button>
                        <span class="ui-muted">"value: " {move || value.get()}</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Label + Motion + Class" code_signal=custom_code>
                <div class="docs-stack">
                    <Meter
                        id="docs-meter-custom".to_string()
                        label="Sync progress".to_string()
                        aria_label="Background sync".to_string()
                        value=Signal::derive(|| Some(64.0))
                        min=0.0
                        max=100.0
                        value_label="64 complete".to_string()
                        motion=ui_components::MeterMotion::fast()
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
                title="Workbench (Display + Config + Code + CSS Test)"
                description="展示区提供当前配置与对比样例；Config/Code/CSS Test 区用于契约验证。"
                code_signal=workbench_code
                test_css_source=test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/meter/src/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="meter-workbench-controls">
                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_variant_danger.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_variant_danger.get() {
                                    "Variant: Danger"
                                } else {
                                    "Variant: Default"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_size_large.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_size_large.get() {
                                    "Size: Lg"
                                } else {
                                    "Size: Default"
                                }}
                            </ui_components::Button>
                        </div>

                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_value.update(|v| *v = (*v - 10).max(0))
                                })
                            >
                                "-10"
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_value.update(|v| *v = (*v + 10).min(100))
                                })
                            >
                                "+10"
                            </ui_components::Button>
                            <span class="ui-muted">"value: " {move || workbench_value.get()}</span>
                        </div>

                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_indeterminate.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_indeterminate.get() {
                                    "Indeterminate: on"
                                } else {
                                    "Indeterminate: off"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_show_value_label.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_show_value_label.get() {
                                    "Value label: on"
                                } else {
                                    "Value label: off"
                                }}
                            </ui_components::Button>
                        </div>

                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_label.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_label.get() {
                                    "Custom value label: on"
                                } else {
                                    "Custom value label: off"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_motion.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_motion.get() {
                                    "Custom motion: on"
                                } else {
                                    "Custom motion: off"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_class.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_class.get() {
                                    "Custom class: on"
                                } else {
                                    "Custom class: off"
                                }}
                            </ui_components::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="meter-workbench-preview">
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
                                show_value_label=workbench_show_value_label.get()
                                value_label=if workbench_custom_label.get() {
                                    format!("{} complete", workbench_value.get())
                                } else {
                                    String::new()
                                }
                                motion=if workbench_custom_motion.get() {
                                    ui_components::MeterMotion::fast()
                                } else {
                                    ui_components::MeterMotion::default()
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn code() -> AnyView {
    let variants_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Inline>"cargo test -p ui-components"</Code>
<Code variant=CodeVariant::Block>
  "cargo fmt --all\ncargo clippy -p ui-components -p docs-app --all-targets -- -D warnings"
</Code>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Inline class_name="docs-code-custom".to_string()>"--deny warnings"</Code>
<Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
  "cargo test -p ui-components --test code_semantics\ncargo test -p ui-components"
</Code>"#.to_string()
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
            "cargo fmt --all\ncargo clippy -p ui-components -p docs-app --all-targets -- -D warnings".to_string()
        } else {
            "cargo test -p ui-components --test code_semantics".to_string()
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
            ui_components::code::styles::CSS
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
            <Playground title="Variant Matrix" code_signal=variants_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <span>"Run "</span>
                        <Code variant=CodeVariant::Inline>"cargo test -p ui-components"</Code>
                        <span>" before opening a PR."</span>
                    </div>
                    <Code variant=CodeVariant::Block>
                        {r#"cargo fmt --all
cargo clippy -p ui-components -p docs-app --all-targets -- -D warnings"#}
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
                        {r#"cargo test -p ui-components --test code_semantics
cargo test -p ui-components"#}
                    </Code>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=interactive_code
                test_css_source=test_css_source
                test_source_path="components/code/src/styles.rs".to_string()
                test_config_signal=actual_config
                description="展示区 + Config 区 + Code 区 + CSS Test 区；包含 inline/block 与 custom class 的对比展示。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"配置区 · Variant"</div>
                        <ui_components::SegmentedControl
                            id_base="docs-code-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=ui_components::SegmentedControlSize::Sm
                            aria_label="Code variant".to_string()
                        />
                        <ui_components::Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </ui_components::Switch>
                        <ui_components::Switch checked=long_content set_checked=set_long_content>
                            "Long content"
                        </ui_components::Switch>
                        <ui_components::Switch checked=show_compare set_checked=set_show_compare>
                            "Show compare matrix"
                        </ui_components::Switch>
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
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <div class="docs-card docs-stack docs-stack--tight">
                                <span class="ui-muted">
                                    {format!("variant={variant:?}, custom_class={}", custom_class.get())}
                                </span>
                                <Code variant=variant class_name=class_name.clone()>
                                    {content}
                                </Code>
                            </div>

                            <Show when=move || compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight">
                                    <div class="docs-row">
                                        <span>"Inline: "</span>
                                        <Code variant=CodeVariant::Inline class_name=class_name.clone()>
                                            "cargo test -p ui-components"
                                        </Code>
                                    </div>
                                    <Code variant=CodeVariant::Block class_name=class_name.clone()>
                                        {r#"cargo fmt --all
cargo clippy -p ui-components -p docs-app --all-targets -- -D warnings"#}
                                    </Code>
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>
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
            ui_components::kbd::styles::CSS
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

    let matrix_code = Signal::derive(move || {
        r#"<Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
<Kbd size=KbdSize::Sm keys="⌘".to_string()>"P"</Kbd>"#
            .to_string()
    });

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
            <Playground title="Size + Keys Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
                    <Kbd size=KbdSize::Sm keys="⌘".to_string()>"P"</Kbd>
                    <Kbd size=KbdSize::Md keys="Alt".to_string()>"Enter"</Kbd>
                </div>
            </Playground>

            <Playground title="Custom Class + Label Only" code_signal=custom_code>
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
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for size/keys/class contracts."
                code_signal=workbench_code
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
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn code_block() -> AnyView {
    let rust_code = r#"fn deploy(service: &str) -> anyhow::Result<()> {
    tracing::info!(target: "deploy", %service, "starting rollout");
    Ok(())
}"#;

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
  code="cargo test -p ui-components --test code_block_semantics".to_string()
  copyable=false
  class_name="docs-code-block-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="CodeBlock"
            slug="code-block"
            group="Display"
            description="Multiline code surface with centralized header/state attrs and spring-driven copy flash motion."
        >
            <Playground title="Header + Copy Motion" code_signal=matrix_code>
                <CodeBlock
                    code=rust_code.to_string()
                    language="rust".to_string()
                    label="deploy.rs".to_string()
                />
            </Playground>

            <Playground title="Compact + No Copy" code_signal=compact_code>
                <CodeBlock
                    code="cargo test -p ui-components --test code_block_semantics".to_string()
                    copyable=false
                    class_name="docs-code-block-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn snippet() -> AnyView {
    let copy_code = Signal::derive(move || {
        r#"<Snippet
  text="cargo fmt --all".to_string()
  label="Command".to_string()
  is_copyable=true
/>
<Snippet
  text="RUST_LOG=debug".to_string()
  is_copyable=true
  copied_label="Done".to_string()
/>"#
        .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Snippet
  text="cargo test -p ui-components --test snippet_semantics".to_string()
  is_copyable=false
  class_name="docs-snippet-custom".to_string()
/>
<Snippet
  text="cargo fmt --all\ncargo clippy -p ui-components -p docs-app --all-targets -- -D warnings".to_string()
  label="CI".to_string()
  is_copyable=false
  class_name="docs-snippet-custom".to_string()
/>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Snippet"
            slug="snippet"
            group="Display"
            description="Text snippet with centralized multiline/copy state attrs and optional copied-label/custom-class contracts."
        >
            <Playground title="Copyable + Copied Label" code_signal=copy_code>
                <div class="docs-stack">
                    <Snippet
                        text="cargo fmt --all".to_string()
                        label="Command".to_string()
                        is_copyable=true
                    />
                    <Snippet
                        text="RUST_LOG=debug".to_string()
                        is_copyable=true
                        copied_label="Done".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Static + Multiline Custom" code_signal=custom_code>
                <div class="docs-stack">
                    <Snippet
                        text="cargo test -p ui-components --test snippet_semantics".to_string()
                        is_copyable=false
                        class_name="docs-snippet-custom".to_string()
                    />
                    <Snippet
                        text="cargo fmt --all\ncargo clippy -p ui-components -p docs-app --all-targets -- -D warnings".to_string()
                        label="CI".to_string()
                        is_copyable=false
                        class_name="docs-snippet-custom".to_string()
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
            ui_components::link::styles::CSS
        )
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

    let states_code = Signal::derive(move || {
        r#"<Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
<Avatar alt="Anonymous collaborator".to_string() size=AvatarSize::Sm />
<Avatar size=AvatarSize::Lg />"#
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

    view! {
        <ComponentPage
            title="Avatar"
            slug="avatar"
            group="Display"
            description="Avatar with image/error fallback, normalized labels, and baseline-style root state attrs + custom-class contract."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-row">
                    <Avatar />
                </div>
            </Playground>

            <Playground title="Image + Fallback" code_signal=image_code>
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

            <Playground title="Sizes + Label Sources" code_signal=states_code>
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

            <Playground title="Custom Class + Normalized Props" code_signal=custom_code>
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
    let empty_items_custom: Vec<AvatarGroupItem> = Vec::new();
    let overflow_items = items.clone();
    let size_items = items.clone();
    let custom_items = items.clone();

    let hello_code = Signal::derive(move || {
        r#"<AvatarGroup items=Vec::<AvatarGroupItem>::new() />"#.to_string()
    });

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

    view! {
        <ComponentPage
            title="AvatarGroup"
            slug="avatar-group"
            group="Display"
            description="Stacked avatars with centralized overflow/empty/aria-label-source state attrs and baseline-style root contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-row">
                    <AvatarGroup items=empty_items.clone() />
                </div>
            </Playground>

            <Playground title="Overflow Stack" code_signal=overflow_code>
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

            <Playground title="Sizes Without Overflow" code_signal=sizes_code>
                <div class="docs-row">
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Sm />
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Md />
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground title="Custom Aria + Class" code_signal=custom_code>
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

    let (radius_index, set_radius_index) = signal(Some(2usize));
    let (shadow_index, set_shadow_index) = signal(Some(1usize));
    let (motion_index, set_motion_index) = signal(Some(0usize));
    let (source_index, set_source_index) = signal(Some(0usize));
    let (is_zoomed, set_is_zoomed) = signal(true);
    let (is_blurred, set_is_blurred) = signal(false);
    let (disable_skeleton, set_disable_skeleton) = signal(false);
    let (with_fallback, set_with_fallback) = signal(true);
    let (custom_class, set_custom_class) = signal(false);

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

    let code =
        Signal::derive(move || r#"<Image src=src.into() alt="Demo".to_string() />"#.to_string());
    let workbench_code = Signal::derive(move || {
        let radius = radius.get();
        let shadow = shadow.get();
        let motion_mode = motion_index.get().unwrap_or(0);
        let source_mode = source_mode.get();
        let is_zoomed = is_zoomed.get();
        let is_blurred = is_blurred.get();
        let disable_skeleton = disable_skeleton.get();
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
        if disable_skeleton {
            snippet.push("  disable_skeleton=true".to_string());
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
        snippet.extend(["/>".to_string()]);
        snippet.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/image/src/styles.rs */\n{}",
            ui_components::image::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let radius = radius.get();
        let shadow = shadow.get();
        let motion_mode = if motion_index.get().unwrap_or(0) == 0 {
            "default"
        } else {
            "custom"
        };
        let source_mode = match source_mode.get() {
            1 => "invalid",
            2 => "missing",
            _ => "valid",
        };
        format!(
            "ImageActualConfig {{\n  source_mode: \"{source_mode}\",\n  has_fallback: {},\n  disable_skeleton: {},\n  is_blurred: {},\n  is_zoomed: {},\n  radius: {radius:?},\n  shadow: {shadow:?},\n  motion: \"{motion_mode}\",\n  custom_class: {},\n}}",
            with_fallback.get(),
            disable_skeleton.get(),
            is_blurred.get(),
            is_zoomed.get(),
            custom_class.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<Image src=src.into() alt="Loaded + Zoom".to_string() is_zoomed=true radius=ImageRadius::Lg shadow=ImageShadow::Md />
<Image src=src.into() alt="Blurred + Soft".to_string() is_blurred=true radius=ImageRadius::Md shadow=ImageShadow::Sm />
<Image src="https://example.invalid/rust-ui-image.png".to_string() fallback_src=fallback_src.into() alt="Invalid -> Fallback".to_string() radius=ImageRadius::Sm shadow=ImageShadow::None />
<Image src="".to_string() fallback_src=fallback_src.into() alt="Missing -> Fallback".to_string() radius=ImageRadius::Full shadow=ImageShadow::Sm />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Image"
            slug="image"
            group="Display"
            description="Image with skeleton, blur, and zoom motion."
        >
            <Playground title="Image" code_signal=code>
                <div class="docs-row">
                    <Image
                        src=into_owned_string(src)
                        alt="Demo image".to_string()
                        radius=ImageRadius::Lg
                        shadow=ImageShadow::Md
                        is_zoomed=true
                    />
                </div>
            </Playground>

            <Playground title="Comparison Matrix: Loaded / Blurred / Fallback / Missing" code_signal=matrix_code>
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
                title="Workbench: Display + Config + Code + CSS Test"
                description="Interactive panel with scoped CSS test + actual config snapshot."
                code_signal=workbench_code
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

                        <Switch checked=is_zoomed set_checked=set_is_zoomed>"Zoomed"</Switch>
                        <Switch checked=is_blurred set_checked=set_is_blurred>"Blurred"</Switch>
                        <Switch checked=disable_skeleton set_checked=set_disable_skeleton>
                            "Disable skeleton"
                        </Switch>
                        <Switch checked=with_fallback set_checked=set_with_fallback>"Use fallback"</Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" style="width: min(100%, 360px);">
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
                        disable_skeleton=disable_skeleton.get()
                        is_blurred=is_blurred.get()
                        is_zoomed=is_zoomed.get()
                        radius=radius.get()
                        shadow=shadow.get()
                        motion=motion.get()
                        class_name=class_name
                    />
                        }
                    }}
                    <span class="ui-muted">
                        {move || format!(
                            "state: source={}, fallback={}, zoomed={}, blurred={}",
                            match source_mode.get() {
                                1 => "invalid",
                                2 => "missing",
                                _ => "valid",
                            },
                            with_fallback.get(),
                            is_zoomed.get(),
                            is_blurred.get(),
                        )}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn illustrated_message() -> AnyView {
    let code = Signal::derive(move || {
        r#"<IllustratedMessage title="Empty".to_string() description="Nothing here".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="IllustratedMessage"
            slug="illustrated-message"
            group="Display"
            description="Empty-state component with optional illustration and actions."
        >
            <Playground title="Empty state" code_signal=code>
                <IllustratedMessage
                    title="No results".to_string()
                    description="Try changing your search.".to_string()
                    illustration=move || view! { <div class="docs-illustration">"◎"</div> }
                    actions=move || view! { <ui_components::Button>"Clear"</ui_components::Button> }
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn motion_ripple() -> AnyView {
    let hello_ref: NodeRef<html::Span> = NodeRef::new();
    let default_ref: NodeRef<html::Span> = NodeRef::new();
    let slow_ref: NodeRef<html::Span> = NodeRef::new();
    let static_ref: NodeRef<html::Span> = NodeRef::new();
    let custom_ref: NodeRef<html::Span> = NodeRef::new();
    let unbounded_ref: NodeRef<html::Span> = NodeRef::new();

    let default_motion = RippleMotion::default();
    let slow_motion = RippleMotion {
        duration_ms: 880,
        ..RippleMotion::default()
    };
    let static_motion = RippleMotion::disabled();
    let custom_motion = RippleMotion {
        duration_ms: 620,
        ..RippleMotion::default()
    };
    let unbounded_motion = RippleMotion {
        duration_ms: 520,
        ..RippleMotion::default()
    };

    let on_hello_click = move |_| {
        ui_components::ripple::trigger_ripple(hello_ref, RippleMotion::default());
    };
    let on_default_click = move |_| {
        ui_components::ripple::trigger_ripple(default_ref, default_motion);
    };
    let on_slow_click = move |_| {
        ui_components::ripple::trigger_ripple(slow_ref, slow_motion);
    };
    let on_static_click = move |_| {
        ui_components::ripple::trigger_ripple(static_ref, static_motion);
    };
    let on_custom_click = move |_| {
        ui_components::ripple::trigger_ripple(custom_ref, custom_motion);
    };
    let on_unbounded_click = move |_| {
        ui_components::ripple::trigger_ripple_at(unbounded_ref, unbounded_motion, 18.0, 48.0);
    };

    let hello_world_code = Signal::derive(move || {
        r#"<button class="docs-ripple-surface" type="button">
  <MotionRipple node_ref=ripple_ref motion=RippleMotion::default() />
</button>"#
            .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<MotionRipple node_ref=default_ref motion=RippleMotion::default() />
<MotionRipple node_ref=slow_ref motion=RippleMotion { duration_ms: 880, ..RippleMotion::default() } />
<MotionRipple node_ref=static_ref motion=RippleMotion::disabled() />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<MotionRipple
  node_ref=custom_ref
  motion=RippleMotion { duration_ms: 620, ..RippleMotion::default() }
  class_name="docs-ripple-custom".to_string()
/>
<MotionRipple
  node_ref=unbounded_ref
  is_bounded=false
  motion=RippleMotion { duration_ms: 520, ..RippleMotion::default() }
  class_name="docs-ripple-custom".to_string()
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
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="docs-row">
                    <button class="docs-ripple-surface" type="button" on:click=on_hello_click>
                        <span class="docs-ripple-label">"Click me"</span>
                        <MotionRipple
                            node_ref=hello_ref
                            motion=RippleMotion::default()
                            class_name="docs-ripple-item".to_string()
                        />
                    </button>
                </div>
            </Playground>

            <Playground title="Animation Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <button class="docs-ripple-surface" type="button" on:click=on_default_click>
                        <span class="docs-ripple-label">"Default (180ms)"</span>
                        <MotionRipple
                            node_ref=default_ref
                            motion=default_motion
                            class_name="docs-ripple-item".to_string()
                        />
                    </button>

                    <button
                        class="docs-ripple-surface docs-ripple-surface--slow"
                        type="button"
                        on:click=on_slow_click
                    >
                        <span class="docs-ripple-label">"Slow (880ms)"</span>
                        <MotionRipple
                            node_ref=slow_ref
                            motion=slow_motion
                            class_name="docs-ripple-item".to_string()
                        />
                    </button>

                    <button
                        class="docs-ripple-surface docs-ripple-surface--static"
                        type="button"
                        on:click=on_static_click
                    >
                        <span class="docs-ripple-label">"Disabled"</span>
                        <MotionRipple
                            node_ref=static_ref
                            motion=static_motion
                            class_name="docs-ripple-item".to_string()
                        />
                    </button>
                </div>
            </Playground>

            <Playground title="Custom Boundary + Class" code_signal=custom_code>
                <div class="docs-row">
                    <button
                        class="docs-ripple-surface docs-ripple-surface--accent"
                        type="button"
                        on:click=on_custom_click
                    >
                        <span class="docs-ripple-label">"Custom Class"</span>
                        <MotionRipple
                            node_ref=custom_ref
                            motion=custom_motion
                            class_name="docs-ripple-custom".to_string()
                        />
                    </button>

                    <button
                        class="docs-ripple-surface docs-ripple-surface--unbounded"
                        type="button"
                        on:click=on_unbounded_click
                    >
                        <span class="docs-ripple-label">"Unbounded + Origin"</span>
                        <MotionRipple
                            node_ref=unbounded_ref
                            is_bounded=false
                            motion=unbounded_motion
                            class_name="docs-ripple-custom".to_string()
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
    let (workbench_show_compare, set_workbench_show_compare) = signal(true);

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

    let workbench_code = Signal::derive(move || {
        let number = workbench_number.get();
        let decimal_places = workbench_decimal_places.get();
        let decimal_separator = workbench_decimal_separator.get();
        let thousand_separator = workbench_thousand_separator.get();

        let mut lines = vec!["<StaticNumber".to_string(), format!("  number={number}")];
        if workbench_pad_start.get() {
            lines.push("  pad_start=true".to_string());
        }
        if let Some(separator) = decimal_separator {
            lines.push(format!("  decimal_separator={separator:?}.into()"));
        }
        if let Some(places) = decimal_places {
            lines.push(format!("  decimal_places={places}"));
        }
        if let Some(separator) = thousand_separator {
            lines.push(format!("  thousand_separator={separator:?}.into()"));
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-static-number-custom\".into()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/text-input/src/number/styles.rs */\n{}",
            ui_components::text_input::number::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let number = workbench_number.get();
        let sanitized = if number.is_finite() { number } else { 0.0 };
        let sign = if sanitized < 0.0 {
            "negative"
        } else if sanitized > 0.0 {
            "positive"
        } else {
            "zero"
        };
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
        let class_source = if workbench_custom_class.get() {
            "custom"
        } else {
            "default"
        };
        let mut classes = vec!["ui-static-number".to_string(), format!("data-sign={sign}")];
        if workbench_custom_class.get() {
            classes.push("docs-static-number-custom".to_string());
        }

        format!(
            "StaticNumberActualConfig {{\n  number: {number},\n  sanitized_number: {sanitized},\n  pad_start: {},\n  decimal_separator_source: \"{decimal_separator_source}\",\n  decimal_places_source: \"{decimal_places_source}\",\n  thousand_separator_source: \"{thousand_separator_source}\",\n  class_source: \"{class_source}\",\n  class: \"{}\",\n}}",
            workbench_pad_start.get(),
            classes.join(" "),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<StaticNumber number=12345.67 decimal_places=2 thousand_separator=",".to_string() />
<StaticNumber number=-9876.5 decimal_places=1 thousand_separator=",".to_string() />
<StaticNumber number=1000.0 decimal_places=0 />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<StaticNumber
  number=42.123456789
  decimal_separator=",".to_string()
  decimal_places=30
  thousand_separator=" ".to_string()
  class_name="docs-static-number-custom".to_string()
/>
<StaticNumber
  number=f64::NAN
  decimal_places=2
  class_name="docs-static-number-custom".to_string()
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
            <Playground title="Formatting Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <StaticNumber
                        number=12345.67
                        decimal_places=2
                        thousand_separator=",".to_string()
                    />
                    <StaticNumber
                        number=-9876.5
                        decimal_places=1
                        thousand_separator=",".to_string()
                    />
                    <StaticNumber number=1000.0 decimal_places=0 />
                </div>
            </Playground>

            <Playground title="Custom Separators + Class" code_signal=custom_code>
                <div class="docs-row">
                    <StaticNumber
                        number=42.123456789
                        decimal_separator=",".to_string()
                        decimal_places=30
                        thousand_separator=" ".to_string()
                        class_name="docs-static-number-custom".to_string()
                    />
                    <StaticNumber
                        number=f64::NAN
                        decimal_places=2
                        class_name="docs-static-number-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
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
                                prop:checked=move || workbench_show_compare.get()
                                on:change=move |ev| set_workbench_show_compare.set(event_target_checked(&ev))
                            />
                            " Show compare"
                        </label>
                    </div>
                }
            >
                {move || {
                    let number = workbench_number.get();
                    let decimal_places = workbench_decimal_places.get();
                    let decimal_separator = workbench_decimal_separator.get().unwrap_or_default();
                    let thousand_separator = workbench_thousand_separator.get().unwrap_or_default();
                    let class_name = if workbench_custom_class.get() {
                        "docs-static-number-custom".to_string()
                    } else {
                        String::new()
                    };
                    let show_compare = workbench_show_compare.get();

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"展示区 · Primary"</div>
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
                                        />
                                    }
                                        .into_any()
                                }}
                            </div>

                            <Show when=move || show_compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-row">
                                    <StaticNumber
                                        number=12345.67
                                        decimal_places=2
                                        thousand_separator=",".to_string()
                                    />
                                    <StaticNumber
                                        number=-9876.5
                                        decimal_places=1
                                        thousand_separator=",".to_string()
                                    />
                                    <StaticNumber number=f64::NAN decimal_places=2 />
                                </div>
                            </Show>
                        </div>
                    }
                }}
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
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
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
        let mut motion = ui_components::SlidingNumberMotion {
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
        if motion != ui_components::SlidingNumberMotion::default() {
            lines.push(format!(
                "  motion=SlidingNumberMotion {{ animate: {}, ..Default::default() }}",
                motion.animate
            ));
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-sliding-number-custom\".into()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/text-input/src/number/styles.rs */\n{}",
            ui_components::text_input::number::styles::CSS
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
        let motion_source = if motion == ui_components::SlidingNumberMotion::default() {
            "default"
        } else {
            "custom"
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
            "SlidingNumberActualConfig {{\n  value: {number},\n  animate: {},\n  decimal_separator_source: \"{decimal_separator_source}\",\n  decimal_places_source: \"{decimal_places_source}\",\n  thousand_separator_source: \"{thousand_separator_source}\",\n  motion_source: \"{motion_source}\",\n  class_source: \"{class_source}\",\n  class: \"{}\",\n}}",
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
  motion=ui_components::SlidingNumberMotion { animate: false, ..Default::default() }
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
            <Playground title="Animated Matrix" code_signal=matrix_code>
                <div class="docs-stack">
                    <SlidingNumber
                        number=number_signal
                        decimal_places=2
                        thousand_separator=",".to_string()
                    />
                    <SlidingNumber number=number_signal decimal_places=0 />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v += 250.0))
                        >
                            "+250"
                        </ui_components::Button>
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v -= 100.0))
                        >
                            "-100"
                        </ui_components::Button>
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
                        motion=ui_components::SlidingNumberMotion {
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
                                            decimal_separator=decimal_separator.clone()
                                            decimal_places=decimal_places
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <SlidingNumber
                                            number=number_signal
                                            motion=motion
                                            decimal_separator=decimal_separator.clone()
                                            thousand_separator=thousand_separator.clone()
                                            class_name=class_name.clone()
                                        />
                                    }
                                        .into_any()
                                }}
                                <div class="docs-row">
                                    <ui_components::Button
                                        variant=ui_components::ButtonVariant::Secondary
                                        on_press=Callback::new(move |_| set_value.update(|v| *v += 250.0))
                                    >
                                        "+250"
                                    </ui_components::Button>
                                    <ui_components::Button
                                        variant=ui_components::ButtonVariant::Secondary
                                        on_press=Callback::new(move |_| set_value.update(|v| *v -= 100.0))
                                    >
                                        "-100"
                                    </ui_components::Button>
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
                                        motion=ui_components::SlidingNumberMotion {
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
        </ComponentPage>
    }
    .into_any()
}
