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
    StatusLightVariant,
};

pub(super) fn alert() -> AnyView {
    let code = r#"<Alert title="Notice".to_string() description="Something happened".to_string()>
  <Button>"Action"</Button>
</Alert>"#;

    view! {
        <ComponentPage
            title="Alert"
            slug="alert"
            group="Display"
            description="Inline alert surface with optional title/description and action slot."
        >
            <Playground title="Alert" code=code>
                <Alert
                    variant=AlertVariant::Default
                    title="Notice".to_string()
                    description="Something happened.".to_string()
                >
                    <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                        "Undo"
                    </ui_components::Button>
                </Alert>
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
    let code = r#"<Badge variant=BadgeVariant::Accent>"New"</Badge>"#;

    view! {
        <ComponentPage
            title="Badge"
            slug="badge"
            group="Display"
            description="Small status badge with variants."
        >
            <Playground title="Badges" code=code>
                <div class="docs-row">
                    <Badge variant=BadgeVariant::Default>"Default"</Badge>
                    <Badge variant=BadgeVariant::Accent>"Accent"</Badge>
                    <Badge variant=BadgeVariant::Danger>"Danger"</Badge>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn status_light() -> AnyView {
    let code = r#"<StatusLight variant=StatusLightVariant::Positive>"Online"</StatusLight>"#;

    view! {
        <ComponentPage
            title="StatusLight"
            slug="status-light"
            group="Display"
            description="Status indicator + label (Spectrum-style)."
        >
            <Playground title="Statuses" code=code>
                <div class="docs-row">
                    <StatusLight variant=StatusLightVariant::Default>"Default"</StatusLight>
                    <StatusLight variant=StatusLightVariant::Accent>"Accent"</StatusLight>
                    <StatusLight variant=StatusLightVariant::Danger>"Danger"</StatusLight>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn chip() -> AnyView {
    let (removed, set_removed) = signal(false);
    let on_dismiss = Callback::new(move |_| set_removed.set(true));

    let code = r#"<Chip on_dismiss=Some(on_dismiss)>"Tag"</Chip>"#;

    view! {
        <ComponentPage
            title="Chip"
            slug="chip"
            group="Display"
            description="Chip / tag pill with optional dismiss button."
        >
            <Playground title="Dismissable" code=code>
                <div class="docs-row">
                    <Show when=move || !removed.get() fallback=move || view! { <span class="ui-muted">"Removed"</span> }>
                        <Chip
                            variant=ChipVariant::Default
                            size=ChipSize::Md
                            on_dismiss=on_dismiss
                        >
                            "Tag"
                        </Chip>
                    </Show>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn skeleton() -> AnyView {
    let code = r#"<Skeleton variant=SkeletonVariant::Rect class_name="...".to_string() />"#;

    view! {
        <ComponentPage
            title="Skeleton"
            slug="skeleton"
            group="Display"
            description="Skeleton placeholder blocks."
        >
            <Playground title="Skeletons" code=code>
                <div class="docs-stack">
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line docs-skeleton-line--short".to_string() />
                    <Skeleton variant=SkeletonVariant::Circle class_name="docs-skeleton-avatar".to_string() />
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-card".to_string() />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn circular_progress() -> AnyView {
    let code = r#"<CircularProgress aria_label="Loading".to_string() size_px=Some(24.0) />"#;

    view! {
        <ComponentPage
            title="CircularProgress"
            slug="circular-progress"
            group="Display"
            description="Indeterminate circular progress (CSS vars)."
        >
            <Playground title="Circular progress" code=code>
                <div class="docs-row">
                    <CircularProgress aria_label="Loading".to_string() size_px=18.0 />
                    <CircularProgress aria_label="Loading".to_string() size_px=26.0 thickness_px=3.0 />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn spinner() -> AnyView {
    let code = r#"<Spinner size=SpinnerSize::Md aria_label="Loading".to_string() />"#;

    view! {
        <ComponentPage
            title="Spinner"
            slug="spinner"
            group="Display"
            description="Spinner is a wrapper over CircularProgress with size presets."
        >
            <Playground title="Spinner" code=code>
                <div class="docs-row">
                    <Spinner size=SpinnerSize::Sm aria_label="Loading".to_string() />
                    <Spinner size=SpinnerSize::Md aria_label="Loading".to_string() />
                    <Spinner size=SpinnerSize::Lg aria_label="Loading".to_string() />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn progress() -> AnyView {
    let (value, set_value) = signal(42.0_f64);
    let progress_value = Signal::derive(move || Some(value.get()));

    let code = r#"let progress_value = Signal::derive(move || Some(value.get()));
<Progress aria_label="Progress".to_string() value=progress_value />"#;

    view! {
        <ComponentPage
            title="Progress"
            slug="progress"
            group="Display"
            description="Spring-driven linear progress indicator."
        >
            <Playground title="Determinate / indeterminate" code=code>
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

    let code = r#"<ProgressCircle value=progress_value min=0.0 max=100.0 />"#;

    view! {
        <ComponentPage
            title="ProgressCircle"
            slug="progress-circle"
            group="Display"
            description="Spring-animated circular progress indicator."
        >
            <Playground title="ProgressCircle" code=code>
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
    let code = r#"<Code>"inline code"</Code>"#;

    view! {
        <ComponentPage
            title="Code"
            slug="code"
            group="Display"
            description="Inline code styling."
        >
            <Playground title="Inline code" code=code>
                <div class="docs-row">
                    <span>"Use "</span>
                    <Code variant=CodeVariant::Inline>"cargo test"</Code>
                    <span>" to run tests."</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn kbd() -> AnyView {
    let code = r#"<Kbd keys="Ctrl+K".to_string() />"#;

    view! {
        <ComponentPage
            title="Kbd"
            slug="kbd"
            group="Display"
            description="Keyboard key hint styling."
        >
            <Playground title="Keyboard hint" code=code>
                <div class="docs-row">
                    <Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
                    <Kbd size=KbdSize::Sm keys="⌘".to_string()>"K"</Kbd>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn code_block() -> AnyView {
    let code_str = r#"fn main() {
    println!("hello");
}"#;
    let code = r#"<CodeBlock code=code.to_string() language=Some("rust".to_string()) />"#;

    view! {
        <ComponentPage
            title="CodeBlock"
            slug="code-block"
            group="Display"
            description="Multiline code block with copy button and copy-flash motion."
        >
            <Playground title="Code block" code=code>
                <CodeBlock code=code_str.to_string() language="rust".to_string() label="main.rs".to_string() />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn snippet() -> AnyView {
    let code = r#"<Snippet text="cargo fmt".to_string() copyable=true />"#;

    view! {
        <ComponentPage
            title="Snippet"
            slug="snippet"
            group="Display"
            description="Text snippet with optional copy-to-clipboard."
        >
            <Playground title="Snippet" code=code>
                <div class="docs-stack">
                    <Snippet text="cargo fmt --all".to_string() label="Command".to_string() copyable=true />
                    <Snippet text="RUST_LOG=debug".to_string() copyable=true />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn link() -> AnyView {
    let code = r#"<Link href="https://example.com".to_string()>"Visit"</Link>"#;

    view! {
        <ComponentPage
            title="Link"
            slug="link"
            group="Display"
            description="Text link with hover/focus-visible styling."
        >
            <Playground title="Links" code=code>
                <div class="docs-row">
                    <Link href="#/docs/welcome".to_string()>"Internal docs link"</Link>
                    <Link href="https://example.com".to_string() target="_blank" rel="noreferrer".to_string()>
                        "External link"
                    </Link>
                    <Link href="#/docs/welcome".to_string() disabled=true>"Disabled"</Link>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn avatar() -> AnyView {
    let src = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Cdefs%3E%3CradialGradient%20id%3D%27g%27%20cx%3D%2732%27%20cy%3D%2732%27%20r%3D%2732%27%3E%3Cstop%20offset%3D%270%27%20stop-color%3D%27%23ff4bd8%27/%3E%3Cstop%20offset%3D%271%27%20stop-color%3D%27%232b5cff%27/%3E%3C/radialGradient%3E%3C/defs%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27url(%23g)%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3EA%3C/text%3E%3C/svg%3E";

    let code = r#"<Avatar name="Ada Lovelace".to_string() src=Some(src.to_string()) />"#;

    let states_code = r#"<Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
<Avatar alt="Anonymous collaborator".to_string() size=AvatarSize::Sm />
<Avatar size=AvatarSize::Lg />"#;

    view! {
        <ComponentPage
            title="Avatar"
            slug="avatar"
            group="Display"
            description="Avatar with image/error fallback, normalized labels, and Spectrum-style root state attrs."
        >
            <Playground title="Image + Fallback" code=code>
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
    ];

    let code = r#"<AvatarGroup items=items max=Some(3) />"#;

    view! {
        <ComponentPage
            title="AvatarGroup"
            slug="avatar-group"
            group="Display"
            description="Overlapping avatar stack with overflow indicator."
        >
            <Playground title="AvatarGroup" code=code>
                <div class="docs-row">
                    <AvatarGroup items=items.clone() max=3 size=AvatarSize::Md />
                    <AvatarGroup items=items max=4 size=AvatarSize::Sm />
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
