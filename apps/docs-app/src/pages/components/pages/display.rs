use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::{html, prelude::*};
use ui_components::{
    Alert, AlertVariant, Avatar, AvatarGroup, AvatarGroupItem, AvatarSize, Badge, BadgeVariant,
    Chip, ChipSize, ChipVariant, CircularProgress, Code, CodeBlock, CodeVariant,
    IllustratedMessage, Image, ImageRadius, ImageShadow, InlineAlert, InlineAlertFill,
    InlineAlertTone, Kbd, KbdSize, Link, Meter, MeterSize, MeterVariant, MotionRipple, Progress,
    ProgressBar, ProgressBarSize, ProgressBarVariant, ProgressCircle, RippleMotion, Skeleton,
    SkeletonVariant, SlidingNumber, Snippet, Spinner, SpinnerSize, StaticNumber, StatusLight,
    StatusLightRole, StatusLightVariant,
};

pub(super) fn alert() -> AnyView {
    let variants_code = r#"<Alert variant=AlertVariant::Default title="Notice".to_string() description="Something happened.".to_string()>
  <Button variant=ButtonVariant::Secondary>"Undo"</Button>
</Alert>
<Alert variant=AlertVariant::Accent title="Syncing".to_string() description="Deployment is in progress.".to_string()>
  <Button variant=ButtonVariant::Secondary>"View logs"</Button>
</Alert>
<Alert variant=AlertVariant::Danger title="Failed".to_string() description="Publishing failed.".to_string()>
  <Button variant=ButtonVariant::Secondary>"Retry"</Button>
</Alert>"#;

    let compact_code = r#"<Alert
  variant=AlertVariant::Accent
  description="Custom class without title".to_string()
  class_name="docs-alert-custom".to_string()
>
  <Button variant=ButtonVariant::Secondary>"Review"</Button>
</Alert>
<Alert variant=AlertVariant::Default title="Heads up".to_string()>
  <Button variant=ButtonVariant::Secondary>"Dismiss"</Button>
</Alert>"#;

    view! {
        <ComponentPage
            title="Alert"
            slug="alert"
            group="Display"
            description="Inline alert surface with centralized variant/content state attrs and action slot semantics."
        >
            <Playground title="Variants + Live Region" code=variants_code>
                <div class="docs-stack">
                    <Alert
                        variant=AlertVariant::Default
                        title="Notice".to_string()
                        description="Something happened.".to_string()
                    >
                        <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                            "Undo"
                        </ui_components::Button>
                    </Alert>
                    <Alert
                        variant=AlertVariant::Accent
                        title="Syncing".to_string()
                        description="Deployment is in progress.".to_string()
                    >
                        <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                            "View logs"
                        </ui_components::Button>
                    </Alert>
                    <Alert
                        variant=AlertVariant::Danger
                        title="Failed".to_string()
                        description="Publishing failed.".to_string()
                    >
                        <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                            "Retry"
                        </ui_components::Button>
                    </Alert>
                </div>
            </Playground>

            <Playground title="Custom Class + Compact" code=compact_code>
                <div class="docs-stack">
                    <Alert
                        variant=AlertVariant::Accent
                        description="Custom class without title".to_string()
                        class_name="docs-alert-custom".to_string()
                    >
                        <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                            "Review"
                        </ui_components::Button>
                    </Alert>
                    <Alert variant=AlertVariant::Default title="Heads up".to_string()>
                        <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                            "Dismiss"
                        </ui_components::Button>
                    </Alert>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn inline_alert() -> AnyView {
    let code = r#"<InlineAlert tone=InlineAlertTone::Info title="Info".to_string()>
  "Message"
</InlineAlert>"#;

    view! {
        <ComponentPage
            title="InlineAlert"
            slug="inline-alert"
            group="Display"
            description="Compact alert with tone/fill variants and optional icon."
        >
            <Playground title="Inline alerts" code=code>
                <div class="docs-stack">
                    <InlineAlert
                        tone=InlineAlertTone::Info
                        fill=InlineAlertFill::Subtle
                        title="Info".to_string()
                        description="Subtle fill".to_string()
                    >
                        "This is an inline alert."
                    </InlineAlert>
                    <InlineAlert
                        tone=InlineAlertTone::Negative
                        fill=InlineAlertFill::Border
                        title="Error".to_string()
                        description="Border fill".to_string()
                    >
                        "Something went wrong."
                    </InlineAlert>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn badge() -> AnyView {
    let matrix_code = r#"<Badge variant=BadgeVariant::Default>"Default"</Badge>
<Badge variant=BadgeVariant::Accent>"Accent"</Badge>
<Badge variant=BadgeVariant::Danger>"Danger"</Badge>
<Badge variant=BadgeVariant::Outline>"Outline"</Badge>"#;

    let custom_code = r#"<Badge variant=BadgeVariant::Accent class_name="docs-badge-custom".to_string()>
  "Release"
</Badge>
<Badge variant=BadgeVariant::Outline class_name="docs-badge-custom".to_string()>
  "Beta"
</Badge>"#;

    view! {
        <ComponentPage
            title="Badge"
            slug="badge"
            group="Display"
            description="Status badge with centralized variant/fill state attrs and custom-class contract."
        >
            <Playground title="Variant Matrix" code=matrix_code>
                <div class="docs-row">
                    <Badge variant=BadgeVariant::Default>"Default"</Badge>
                    <Badge variant=BadgeVariant::Accent>"Accent"</Badge>
                    <Badge variant=BadgeVariant::Danger>"Danger"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                </div>
            </Playground>

            <Playground title="Custom Class + Outline" code=custom_code>
                <div class="docs-row">
                    <Badge variant=BadgeVariant::Accent class_name="docs-badge-custom".to_string()>
                        "Release"
                    </Badge>
                    <Badge variant=BadgeVariant::Outline class_name="docs-badge-custom".to_string()>
                        "Beta"
                    </Badge>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn status_light() -> AnyView {
    let variants_code = r#"<StatusLight variant=StatusLightVariant::Default>"Idle"</StatusLight>
<StatusLight variant=StatusLightVariant::Accent>"Deploying"</StatusLight>
<StatusLight variant=StatusLightVariant::Danger>"Failed"</StatusLight>"#;

    let role_code =
        r#"<StatusLight role=StatusLightRole::Status>"Background sync complete"</StatusLight>"#;

    let custom_code = r#"<StatusLight class_name="docs-status-light-custom".to_string()>"Queued"</StatusLight>
<StatusLight
  role=StatusLightRole::Status
  variant=StatusLightVariant::Accent
  class_name="docs-status-light-custom".to_string()
>
  "Deploy started"
</StatusLight>"#;

    view! {
        <ComponentPage
            title="StatusLight"
            slug="status-light"
            group="Display"
            description="Status indicator + label with centralized variant/live/role-source state attrs and optional custom-class contract."
        >
            <Playground title="Variants" code=variants_code>
                <div class="docs-row">
                    <StatusLight variant=StatusLightVariant::Default>"Idle"</StatusLight>
                    <StatusLight variant=StatusLightVariant::Accent>"Deploying"</StatusLight>
                    <StatusLight variant=StatusLightVariant::Danger>"Failed"</StatusLight>
                </div>
            </Playground>

            <Playground title="Live Region Role" code=role_code>
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

            <Playground title="Custom Class + Static" code=custom_code>
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
    let (removed, set_removed) = signal(false);
    let on_dismiss = Callback::new(move |_| set_removed.set(true));

    let removable_code = r#"<Chip
  variant=ChipVariant::Accent
  size=ChipSize::Md
  on_dismiss=Some(on_dismiss)
  dismiss_aria_label="Remove reviewer".to_string()
>
  "Reviewer"
</Chip>"#;

    let variants_code = r#"<Chip variant=ChipVariant::Default size=ChipSize::Sm>"Default"</Chip>
<Chip variant=ChipVariant::Accent size=ChipSize::Md>"Accent"</Chip>
<Chip variant=ChipVariant::Danger size=ChipSize::Lg>"Danger"</Chip>
<Chip variant=ChipVariant::Outline size=ChipSize::Md>"Outline"</Chip>"#;

    let custom_code = r#"<Chip
  variant=ChipVariant::Accent
  on_dismiss=Some(Callback::new(|_| ()))
  dismiss_aria_label="  Remove reviewer  ".to_string()
  class_name="docs-chip-custom".to_string()
>
  "Reviewer"
</Chip>
<Chip variant=ChipVariant::Outline class_name="docs-chip-custom".to_string()>
  "Read only"
</Chip>"#;

    let disabled_code = r#"<Chip disabled=true on_dismiss=Some(on_dismiss)>"Locked"</Chip>
<Chip disabled=true>"Read only"</Chip>"#;

    view! {
        <ComponentPage
            title="Chip"
            slug="chip"
            group="Display"
            description="Chip / tag pill with centralized variant-size-state attrs, dismiss-label source contracts, and optional custom class semantics."
        >
            <Playground title="Removable" code=removable_code>
                <div class="docs-row">
                    <Show
                        when=move || !removed.get()
                        fallback=move || view! { <span class="ui-muted">"Removed"</span> }
                    >
                        <Chip
                            variant=ChipVariant::Accent
                            size=ChipSize::Md
                            on_dismiss=on_dismiss
                            dismiss_aria_label="Remove reviewer".to_string()
                        >
                            "Reviewer"
                        </Chip>
                    </Show>
                </div>
            </Playground>

            <Playground title="Variants + Sizes" code=variants_code>
                <div class="docs-row">
                    <Chip variant=ChipVariant::Default size=ChipSize::Sm>"Default"</Chip>
                    <Chip variant=ChipVariant::Accent size=ChipSize::Md>"Accent"</Chip>
                    <Chip variant=ChipVariant::Danger size=ChipSize::Lg>"Danger"</Chip>
                    <Chip variant=ChipVariant::Outline size=ChipSize::Md>"Outline"</Chip>
                </div>
            </Playground>

            <Playground title="Custom Label + Class" code=custom_code>
                <div class="docs-row">
                    <Chip
                        variant=ChipVariant::Accent
                        on_dismiss=Callback::new(|_| ())
                        dismiss_aria_label="  Remove reviewer  ".to_string()
                        class_name="docs-chip-custom".to_string()
                    >
                        "Reviewer"
                    </Chip>
                    <Chip variant=ChipVariant::Outline class_name="docs-chip-custom".to_string()>
                        "Read only"
                    </Chip>
                </div>
            </Playground>

            <Playground title="Disabled + Static" code=disabled_code>
                <div class="docs-row">
                    <Chip
                        disabled=true
                        on_dismiss=Callback::new(|_| ())
                        dismiss_aria_label="Cannot remove".to_string()
                    >
                        "Locked"
                    </Chip>
                    <Chip disabled=true variant=ChipVariant::Outline>
                        "Read only"
                    </Chip>
                    <Chip variant=ChipVariant::Default size=ChipSize::Sm>
                        "Static"
                    </Chip>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn skeleton() -> AnyView {
    let shimmer_code = r#"<Skeleton variant=SkeletonVariant::Rect shimmer=true class_name="docs-skeleton-line".to_string() />
<Skeleton variant=SkeletonVariant::Circle shimmer=true class_name="docs-skeleton-avatar".to_string() />"#;

    let still_code = r#"<Skeleton variant=SkeletonVariant::Rect shimmer=false class_name="docs-skeleton-line".to_string() />
<Skeleton variant=SkeletonVariant::Circle shimmer=false class_name="docs-skeleton-avatar".to_string() />"#;

    view! {
        <ComponentPage
            title="Skeleton"
            slug="skeleton"
            group="Display"
            description="Skeleton placeholder blocks with centralized variant/shimmer state attrs."
        >
            <Playground title="Shimmer" code=shimmer_code>
                <div class="docs-stack">
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line docs-skeleton-line--short".to_string() />
                    <Skeleton variant=SkeletonVariant::Circle class_name="docs-skeleton-avatar".to_string() />
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-card".to_string() />
                </div>
            </Playground>

            <Playground title="Still" code=still_code>
                <div class="docs-stack">
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        shimmer=false
                        class_name="docs-skeleton-line".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        shimmer=false
                        class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Circle
                        shimmer=false
                        class_name="docs-skeleton-avatar".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn circular_progress() -> AnyView {
    let matrix_code = r#"<CircularProgress aria_label="Loading".to_string() />
<CircularProgress aria_label="Syncing mail".to_string() size_px=24.0 />
<CircularProgress aria_label="Syncing mail".to_string() thickness_px=3.0 />
<CircularProgress aria_label="Syncing mail".to_string() size_px=30.0 thickness_px=4.0 />"#;

    let custom_code = r#"<CircularProgress
  aria_label="Background refresh".to_string()
  size_px=28.0
  thickness_px=3.5
  class_name="docs-circular-progress-custom".to_string()
/>
<CircularProgress aria_label="   ".to_string() class_name="docs-circular-progress-custom".to_string() />"#;

    view! {
        <ComponentPage
            title="CircularProgress"
            slug="circular-progress"
            group="Display"
            description="Indeterminate circular progress with centralized size/thickness/label source attrs."
        >
            <Playground title="Size + Thickness Matrix" code=matrix_code>
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

            <Playground title="Custom Label + Class" code=custom_code>
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
    let sizes_code = r#"<Spinner size=SpinnerSize::Sm />
<Spinner size=SpinnerSize::Md />
<Spinner size=SpinnerSize::Lg />"#;

    let labels_code = r#"<Spinner aria_label="Fetching notifications".to_string() />
<Spinner class_name="docs-spinner-custom".to_string() />"#;

    view! {
        <ComponentPage
            title="Spinner"
            slug="spinner"
            group="Display"
            description="Spinner wraps CircularProgress and now exposes Spectrum-style size/label state attrs."
        >
            <Playground title="Sizes" code=sizes_code>
                <div class="docs-row">
                    <Spinner size=SpinnerSize::Sm />
                    <Spinner size=SpinnerSize::Md />
                    <Spinner size=SpinnerSize::Lg />
                </div>
            </Playground>

            <Playground title="Label + Custom Class" code=labels_code>
                <div class="docs-row">
                    <Spinner aria_label="Fetching notifications".to_string() />
                    <Spinner class_name="docs-spinner-custom".to_string() />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn progress() -> AnyView {
    let (value, set_value) = signal(42.0_f64);
    let progress_value = Signal::derive(move || Some(value.get()));

    let matrix_code = r#"let progress_value = Signal::derive(move || Some(value.get()));
<Progress aria_label="Determinate".to_string() value=progress_value />
<Progress aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />"#;

    let custom_code = r#"<Progress
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
/>"#;

    view! {
        <ComponentPage
            title="Progress"
            slug="progress"
            group="Display"
            description="Spring-driven linear progress with centralized source attrs."
        >
            <Playground title="Determinate + Indeterminate" code=matrix_code>
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
                        <span class="ui-muted">"value: " {move || value.get().to_string()}</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Label + Motion + Class" code=custom_code>
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
    let code = r#"<ProgressBar value=Some(30.0) max=100.0 />"#;

    view! {
        <ComponentPage
            title="ProgressBar"
            slug="progress-bar"
            group="Display"
            description="Native <progress> element styling."
        >
            <Playground title="ProgressBar" code=code>
                <div class="docs-stack">
                    <ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Md value=30.0 />
                    <ProgressBar variant=ProgressBarVariant::Accent size=ProgressBarSize::Md value=72.0 />
                    <ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Md indeterminate=true />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn progress_circle() -> AnyView {
    let (value, set_value) = signal(35.0_f64);
    let progress_value = Signal::derive(move || Some(value.get()));

    let matrix_code = r#"<ProgressCircle aria_label="Determinate".to_string() value=progress_value min=0.0 max=100.0 />
<ProgressCircle aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />"#;

    let custom_code = r#"<ProgressCircle
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
/>"#;

    view! {
        <ComponentPage
            title="ProgressCircle"
            slug="progress-circle"
            group="Display"
            description="Spring-animated circular progress with centralized source attrs."
        >
            <Playground title="Determinate + Indeterminate" code=matrix_code>
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

            <Playground title="Custom Value Label + Class" code=custom_code>
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

    let code = r#"let meter_value = Signal::derive(move || Some(value.get() as f64));
<Meter id="m".to_string() label="Completion".to_string() value=meter_value />"#;

    view! {
        <ComponentPage
            title="Meter"
            slug="meter"
            group="Display"
            description="Meter/progressbar with label and optional value label."
        >
            <Playground title="Meter" code=code>
                <div class="docs-stack">
                    <Meter
                        id="docs-meter".to_string()
                        label="Completion".to_string()
                        value=meter_value
                        min=0.0
                        max=100.0
                        variant=MeterVariant::Default
                        size=MeterSize::Default
                    />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 10).min(100)))
                        >
                            "+10"
                        </ui_components::Button>
                        <span class="ui-muted">"value: " {move || value.get().to_string()}</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn code() -> AnyView {
    let variants_code = r#"<Code variant=CodeVariant::Inline>"cargo test -p ui-components"</Code>
<Code variant=CodeVariant::Block>
  "cargo fmt --all\ncargo clippy -p ui-components -p docs-app --all-targets -- -D warnings"
</Code>"#;

    let custom_code = r#"<Code variant=CodeVariant::Inline class_name="docs-code-custom".to_string()>"--deny warnings"</Code>
<Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
  "cargo test -p ui-components --test code_semantics\ncargo test -p ui-components"
</Code>"#;

    view! {
        <ComponentPage
            title="Code"
            slug="code"
            group="Display"
            description="Inline/Block code surface with centralized variant state attrs and optional custom-class contract."
        >
            <Playground title="Variant Matrix" code=variants_code>
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

            <Playground title="Custom Class + Block" code=custom_code>
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn kbd() -> AnyView {
    let matrix_code = r#"<Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
<Kbd size=KbdSize::Sm keys="⌘".to_string()>"P"</Kbd>"#;

    let custom_code = r#"<Kbd size=KbdSize::Md class_name="docs-kbd-custom".to_string()>"Esc"</Kbd>
<Kbd size=KbdSize::Sm keys="Shift".to_string() class_name="docs-kbd-custom".to_string()>"Tab"</Kbd>"#;

    view! {
        <ComponentPage
            title="Kbd"
            slug="kbd"
            group="Display"
            description="Keyboard keycap with centralized size/keys state attrs and optional custom-class contract."
        >
            <Playground title="Size + Keys Matrix" code=matrix_code>
                <div class="docs-row">
                    <Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
                    <Kbd size=KbdSize::Sm keys="⌘".to_string()>"P"</Kbd>
                    <Kbd size=KbdSize::Md keys="Alt".to_string()>"Enter"</Kbd>
                </div>
            </Playground>

            <Playground title="Custom Class + Label Only" code=custom_code>
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
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn code_block() -> AnyView {
    let rust_code = r#"fn deploy(service: &str) -> anyhow::Result<()> {
    tracing::info!(target: "deploy", %service, "starting rollout");
    Ok(())
}"#;

    let matrix_code = r#"<CodeBlock
  code=rust_code.to_string()
  language="rust".to_string()
  label="deploy.rs".to_string()
/>"#;

    let compact_code = r#"<CodeBlock
  code="cargo test -p ui-components --test code_block_semantics".to_string()
  copyable=false
  class_name="docs-code-block-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="CodeBlock"
            slug="code-block"
            group="Display"
            description="Multiline code surface with centralized header/state attrs and spring-driven copy flash motion."
        >
            <Playground title="Header + Copy Motion" code=matrix_code>
                <CodeBlock
                    code=rust_code.to_string()
                    language="rust".to_string()
                    label="deploy.rs".to_string()
                />
            </Playground>

            <Playground title="Compact + No Copy" code=compact_code>
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
    let copy_code = r#"<Snippet
  text="cargo fmt --all".to_string()
  label="Command".to_string()
  copyable=true
/>
<Snippet
  text="RUST_LOG=debug".to_string()
  copyable=true
  copied_label="Done".to_string()
/>"#;

    let custom_code = r#"<Snippet
  text="cargo test -p ui-components --test snippet_semantics".to_string()
  copyable=false
  class_name="docs-snippet-custom".to_string()
/>
<Snippet
  text="cargo fmt --all\ncargo clippy -p ui-components -p docs-app --all-targets -- -D warnings".to_string()
  label="CI".to_string()
  copyable=false
  class_name="docs-snippet-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Snippet"
            slug="snippet"
            group="Display"
            description="Text snippet with centralized multiline/copy state attrs and optional copied-label/custom-class contracts."
        >
            <Playground title="Copyable + Copied Label" code=copy_code>
                <div class="docs-stack">
                    <Snippet text="cargo fmt --all".to_string() label="Command".to_string() copyable=true />
                    <Snippet text="RUST_LOG=debug".to_string() copyable=true copied_label="Done".to_string() />
                </div>
            </Playground>

            <Playground title="Static + Multiline Custom" code=custom_code>
                <div class="docs-stack">
                    <Snippet
                        text="cargo test -p ui-components --test snippet_semantics".to_string()
                        copyable=false
                        class_name="docs-snippet-custom".to_string()
                    />
                    <Snippet
                        text="cargo fmt --all\ncargo clippy -p ui-components -p docs-app --all-targets -- -D warnings".to_string()
                        label="CI".to_string()
                        copyable=false
                        class_name="docs-snippet-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn link() -> AnyView {
    let states_code = r##"<Link href="#/docs/welcome".to_string()>"Internal docs link"</Link>
<Link href="https://example.com".to_string() target="_blank">"External link"</Link>
<Link href="#/docs/welcome".to_string() disabled=true>"Disabled"</Link>
<Link href="   ".to_string()>"Missing href"</Link>"##;

    let custom_code = r#"<Link
  href="https://example.com".to_string()
  target="_blank"
  rel="sponsored".to_string()
  aria_label="Open partner documentation".to_string()
  class_name="docs-link-custom".to_string()
>
  "Partner docs"
</Link>"#;

    view! {
        <ComponentPage
            title="Link"
            slug="link"
            group="Display"
            description="Text link with centralized disabled/target/rel state attrs and headless hover + focus-visible semantics."
        >
            <Playground title="State Matrix" code=states_code>
                <div class="docs-row">
                    <Link href="#/docs/welcome".to_string()>"Internal docs link"</Link>
                    <Link href="https://example.com".to_string() target="_blank">
                        "External link"
                    </Link>
                    <Link href="#/docs/welcome".to_string() disabled=true>"Disabled"</Link>
                    <Link href="   ".to_string()>"Missing href"</Link>
                </div>
            </Playground>

            <Playground title="Custom Rel + Class" code=custom_code>
                <div class="docs-row">
                    <Link
                        href="https://example.com".to_string()
                        target="_blank"
                        rel="sponsored".to_string()
                        aria_label="Open partner documentation".to_string()
                        class_name="docs-link-custom".to_string()
                    >
                        "Partner docs"
                    </Link>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn avatar() -> AnyView {
    let src = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Cdefs%3E%3CradialGradient%20id%3D%27g%27%20cx%3D%2732%27%20cy%3D%2732%27%20r%3D%2732%27%3E%3Cstop%20offset%3D%270%27%20stop-color%3D%27%23ff4bd8%27/%3E%3Cstop%20offset%3D%271%27%20stop-color%3D%27%232b5cff%27/%3E%3C/radialGradient%3E%3C/defs%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27url(%23g)%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3EA%3C/text%3E%3C/svg%3E";

    let image_code = r#"<Avatar name="Ada Lovelace".to_string() src=Some(src.to_string()) />"#;

    let states_code = r#"<Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
<Avatar alt="Anonymous collaborator".to_string() size=AvatarSize::Sm />
<Avatar size=AvatarSize::Lg />"#;

    let custom_code = r#"<Avatar
  name="  Ada Lovelace  ".to_string()
  alt="  Team lead  ".to_string()
  size=AvatarSize::Lg
  class_name="docs-avatar-custom".to_string()
/>
<Avatar
  alt="  Anonymous collaborator  ".to_string()
  src="   ".to_string()
  class_name="docs-avatar-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Avatar"
            slug="avatar"
            group="Display"
            description="Avatar with image/error fallback, normalized labels, and Spectrum-style root state attrs + custom-class contract."
        >
            <Playground title="Image + Fallback" code=image_code>
                <div class="docs-row">
                    <Avatar name="Ada Lovelace".to_string() src=src.to_string() size=AvatarSize::Md />
                    <Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
                    <Avatar name="Alan Turing".to_string() size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground title="Sizes + Label Sources" code=states_code>
                <div class="docs-row">
                    <Avatar
                        name="Ada Lovelace".to_string()
                        src=src.to_string()
                        alt="Profile photo".to_string()
                        size=AvatarSize::Sm
                    />
                    <Avatar alt="Anonymous collaborator".to_string() size=AvatarSize::Sm />
                    <Avatar size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground title="Custom Class + Normalized Props" code=custom_code>
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
            src: Some(src_a.to_string()),
            alt: Some("Ada".to_string()),
        },
        AvatarGroupItem {
            name: Some("Grace Hopper".to_string()),
            src: Some(src_b.to_string()),
            alt: Some("Grace".to_string()),
        },
        AvatarGroupItem {
            name: Some("Alan Turing".to_string()),
            src: Some(src_c.to_string()),
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
    let overflow_items = items.clone();
    let size_items = items.clone();
    let custom_items = items.clone();

    let overflow_code = r#"<AvatarGroup items=items.clone() max=3 size=AvatarSize::Md />"#;

    let sizes_code = r#"<AvatarGroup items=items.clone() max=6 size=AvatarSize::Sm />
<AvatarGroup items=items.clone() max=6 size=AvatarSize::Lg />"#;

    let custom_code = r#"<AvatarGroup
  items=Vec::<AvatarGroupItem>::new()
  size=AvatarSize::Md
  aria_label="No collaborators".to_string()
  class_name="docs-avatar-group-custom".to_string()
/>
<AvatarGroup
  items=items.clone()
  max=3
  size=AvatarSize::Md
  aria_label="Core collaborators".to_string()
  class_name="docs-avatar-group-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="AvatarGroup"
            slug="avatar-group"
            group="Display"
            description="Stacked avatars with centralized overflow/empty/aria-label-source state attrs and Spectrum-style root contracts."
        >
            <Playground title="Overflow Stack" code=overflow_code>
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

            <Playground title="Sizes Without Overflow" code=sizes_code>
                <div class="docs-row">
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Sm />
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Md />
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground title="Custom Aria + Class" code=custom_code>
                <div class="docs-row">
                    <AvatarGroup
                        items=empty_items.clone()
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
    let src = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%27320%27%20height%3D%27180%27%20viewBox%3D%270%200%20320%20180%27%3E%3Crect%20width%3D%27100%25%27%20height%3D%27100%25%27%20fill%3D%27%23111%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2750%25%27%20fill%3D%27%23fff%27%20font-size%3D%2720%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%3Erust-ui%3C/text%3E%3C/svg%3E";

    let code = r#"<Image src=Some(src.to_string()) alt="Demo".to_string() />"#;

    view! {
        <ComponentPage
            title="Image"
            slug="image"
            group="Display"
            description="Image with skeleton, blur, and zoom motion."
        >
            <Playground title="Image" code=code>
                <div class="docs-row">
                    <Image
                        src=src.to_string()
                        alt="Demo image".to_string()
                        radius=ImageRadius::Lg
                        shadow=ImageShadow::Md
                        is_zoomed=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn illustrated_message() -> AnyView {
    let code = r#"<IllustratedMessage title="Empty".to_string() description="Nothing here".to_string() />"#;

    view! {
        <ComponentPage
            title="IllustratedMessage"
            slug="illustrated-message"
            group="Display"
            description="Empty-state component with optional illustration and actions."
        >
            <Playground title="Empty state" code=code>
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
    let ripple_ref: NodeRef<html::Span> = NodeRef::new();
    let on_click = move |_| {
        ui_components::ripple::motion::trigger_ripple(ripple_ref, RippleMotion::default());
    };

    let code = r#"<button on:click=...>
  <MotionRipple node_ref=ripple_ref />
</button>"#;

    view! {
        <ComponentPage
            title="MotionRipple"
            slug="motion-ripple"
            group="Display"
            description="WAAPI-driven ripple effect surface (animate-ui style)."
        >
            <Playground title="Ripple" code=code>
                <button class="docs-ripple-surface" type="button" on:click=on_click>
                    <span class="docs-ripple-label">"Click for ripple"</span>
                    <MotionRipple node_ref=ripple_ref class_name="docs-ripple-item".to_string() />
                </button>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn static_number() -> AnyView {
    let code = r#"<StaticNumber number=12345.67 decimal_places=Some(2) thousand_separator=Some(",".to_string()) />"#;

    view! {
        <ComponentPage
            title="StaticNumber"
            slug="static-number"
            group="Display"
            description="Static number formatting (no animation)."
        >
            <Playground title="Static number" code=code>
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
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sliding_number() -> AnyView {
    let (value, set_value) = signal(12345.67_f64);
    let number_signal: Signal<f64> = Signal::derive(move || value.get());

    let code = r#"let number_signal: Signal<f64> = Signal::derive(move || value.get());
<SlidingNumber number=number_signal />"#;

    view! {
        <ComponentPage
            title="SlidingNumber"
            slug="sliding-number"
            group="Display"
            description="Spring-animated per-digit number transitions."
        >
            <Playground title="Animated number" code=code>
                <div class="docs-stack">
                    <SlidingNumber number=number_signal decimal_places=2 thousand_separator=",".to_string() />
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
                        <span class="ui-muted">"value: " {move || value.get().to_string()}</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
