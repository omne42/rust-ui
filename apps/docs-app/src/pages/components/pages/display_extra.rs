use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AlertBanner, AlertBannerFill, AlertBannerMotion, AlertBannerTone, Chart, ChartKind, ChartPoint,
    ColorSwatch, ColorSwatchPicker, ColorSwatchPickerItem, ColorSwatchRounding, ColorSwatchShape,
    ColorSwatchSize, EmptyState, EmptyStateAlign, EmptyStateTone, ErrorView, ErrorViewMotion,
    ErrorViewTone, Icon, IconSize, IconTone, Keyboard, KeyboardTone, LabeledValue,
    LabeledValueOrientation, LabeledValueTone, PressableFeedback, PressableFeedbackEffect,
    PressableFeedbackMotion, PressableFeedbackTone, RippleMotion, Skeleton, SkeletonGroup,
    SkeletonGroupDensity, SkeletonGroupLayout, SkeletonGroupVariant, SkeletonVariant, Text,
    TextAlign, TextElement, TextTone, TextWeight,
};

pub(super) fn alert_banner() -> AnyView {
    let tone_code = r#"<AlertBanner
  tone=AlertBannerTone::Info
  fill=AlertBannerFill::Border
  title="Updates available".to_string()
  description="A new version is ready to install.".to_string()
>
  "Install now to keep your workspace secure."
</AlertBanner>
<AlertBanner
  tone=AlertBannerTone::Negative
  fill=AlertBannerFill::Subtle
  title="Deployment failed".to_string()
  description="Rollback completed. Review incident timeline.".to_string()
>
  "Check logs before retrying."
</AlertBanner>"#;

    let custom_code = r#"<AlertBanner
  tone=AlertBannerTone::Notice
  fill=AlertBannerFill::Bold
  hide_icon=true
  title="Maintenance window".to_string()
  description="Service may be degraded during migration.".to_string()
  class_name="docs-alert-banner-custom".to_string()
>
  "Follow status page for live updates."
</AlertBanner>"#;

    let motion_code = r#"<AlertBanner
  tone=AlertBannerTone::Info
  fill=AlertBannerFill::Border
  title="Motion tuned".to_string()
  description="Custom spring contract for alert reveal.".to_string()
  motion=AlertBannerMotion {
    spring: Default::default(),
  }
>
  "Inspect data-motion-source/data-custom-motion markers."
</AlertBanner>"#;

    view! {
        <ComponentPage
            title="AlertBanner"
            slug="alert-banner"
            group="Display"
            description="Spectrum-compatible alert banner with centralized tone/fill/content contracts and HeroUI-grade spring reveal motion."
        >
            <Playground title="Tone + Fill" code=tone_code>
                <div class="docs-stack">
                    <AlertBanner
                        tone=AlertBannerTone::Info
                        fill=AlertBannerFill::Border
                        title="Updates available".to_string()
                        description="A new version is ready to install.".to_string()
                    >
                        "Install now to keep your workspace secure."
                    </AlertBanner>
                    <AlertBanner
                        tone=AlertBannerTone::Negative
                        fill=AlertBannerFill::Subtle
                        title="Deployment failed".to_string()
                        description="Rollback completed. Review incident timeline.".to_string()
                    >
                        "Check logs before retrying."
                    </AlertBanner>
                </div>
            </Playground>

            <Playground title="Bold + Hidden Icon + Custom Class" code=custom_code>
                <AlertBanner
                    tone=AlertBannerTone::Notice
                    fill=AlertBannerFill::Bold
                    hide_icon=true
                    title="Maintenance window".to_string()
                    description="Service may be degraded during migration.".to_string()
                    class_name="docs-alert-banner-custom".to_string()
                >
                    "Follow status page for live updates."
                </AlertBanner>
            </Playground>

            <Playground title="Custom motion contract" code=motion_code>
                <AlertBanner
                    tone=AlertBannerTone::Info
                    fill=AlertBannerFill::Border
                    title="Motion tuned".to_string()
                    description="Custom spring contract for alert reveal.".to_string()
                    motion=AlertBannerMotion {
                        spring: Default::default(),
                    }
                >
                    "Inspect data-motion-source/data-custom-motion markers."
                </AlertBanner>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn labeled_value() -> AnyView {
    let orientation_code = r#"<LabeledValue label="Project".to_string() value="Omne".to_string() />
<LabeledValue
  label="Status".to_string()
  value="Healthy".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>"#;

    let custom_code = r#"<LabeledValue
  label="Build".to_string()
  value="passing".to_string()
  description="Updated 2 minutes ago".to_string()
  aria_label="Build status".to_string()
  class_name="docs-labeled-value-custom".to_string()
  tone=LabeledValueTone::Strong
/>"#;

    view! {
        <ComponentPage
            title="LabeledValue"
            slug="labeled-value"
            group="Display"
            description="Label-value pair primitive with centralized orientation/tone/source state contracts and Spectrum-style data markers."
        >
            <Playground title="Orientation + Tone" code=orientation_code>
                <div class="docs-stack">
                    <LabeledValue label="Project".to_string() value="Omne".to_string() />
                    <LabeledValue
                        label="Status".to_string()
                        value="Healthy".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                    />
                </div>
            </Playground>

            <Playground title="Description + Custom Aria/Class" code=custom_code>
                <div class="docs-stack">
                    <LabeledValue
                        label="Build".to_string()
                        value="passing".to_string()
                        description="Updated 2 minutes ago".to_string()
                        aria_label="Build status".to_string()
                        class_name="docs-labeled-value-custom".to_string()
                        tone=LabeledValueTone::Strong
                    />
                    <LabeledValue
                        label="SLA".to_string()
                        value="99.95%".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Default
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn keyboard() -> AnyView {
    let tone_code = r#"<Keyboard>"⌘K"</Keyboard>
<Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>"#;

    let compact_code = r#"<Keyboard
  compact=true
  aria_label="Open command palette".to_string()
  class_name="docs-keyboard-custom".to_string()
>
  "Ctrl+Shift+P"
</Keyboard>"#;

    view! {
        <ComponentPage
            title="Keyboard"
            slug="keyboard"
            group="Display"
            description="Keyboard command primitive (`<kbd>`) with centralized tone/compact/source state contracts."
        >
            <Playground title="Tone" code=tone_code>
                <div class="docs-row">
                    <Keyboard>"⌘K"</Keyboard>
                    <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
                </div>
            </Playground>

            <Playground title="Compact + Custom Aria/Class" code=compact_code>
                <div class="docs-row">
                    <Keyboard
                        compact=true
                        aria_label="Open command palette".to_string()
                        class_name="docs-keyboard-custom".to_string()
                    >
                        "Ctrl+Shift+P"
                    </Keyboard>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn text() -> AnyView {
    let tone_code = r#"<Text text="Primary body copy".to_string() />
<Text text="Subtle metadata".to_string() tone=TextTone::Subtle />
<Text text="Strong headline".to_string() tone=TextTone::Strong weight=TextWeight::Bold />"#;

    let states_code = r#"<Text
  text="Centered label".to_string()
  align=TextAlign::Center
  element=TextElement::Div
/>
<Text
  text="Long text that truncates when width is constrained by the container around it".to_string()
  truncate=true
  class_name="docs-text-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Text"
            slug="text"
            group="Display"
            description="Typography primitive with centralized tone/alignment/weight/source state contracts and Spectrum-style data markers."
        >
            <Playground title="Tone + Weight Matrix" code=tone_code>
                <div class="docs-stack">
                    <Text text="Primary body copy".to_string() />
                    <Text text="Subtle metadata".to_string() tone=TextTone::Subtle />
                    <Text
                        text="Strong headline".to_string()
                        tone=TextTone::Strong
                        weight=TextWeight::Bold
                    />
                </div>
            </Playground>

            <Playground title="Alignment + Truncate + Element" code=states_code>
                <div class="docs-stack">
                    <Text
                        text="Centered label".to_string()
                        align=TextAlign::Center
                        weight=TextWeight::Semibold
                        element=TextElement::Div
                    />
                    <Text
                        text="Long text that truncates when width is constrained by the container around it".to_string()
                        truncate=true
                        class_name="docs-text-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn icon() -> AnyView {
    let matrix_code = r#"<Icon size=IconSize::Sm tone=IconTone::Default decorative=true>"✓"</Icon>
<Icon size=IconSize::Md tone=IconTone::Muted decorative=true>"⚙"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Accent decorative=true>"★"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Danger decorative=true>"⚠"</Icon>"#;

    let states_code = r#"<Icon
  size=IconSize::Md
  tone=IconTone::Accent
  decorative=false
  aria_label="Sync successful".to_string()
>
  "✓"
</Icon>
<Icon
  size=IconSize::Lg
  tone=IconTone::Muted
  disabled=true
  class_name="docs-icon-custom".to_string()
  decorative=true
>
  "⚙"
</Icon>"#;

    view! {
        <ComponentPage
            title="Icon"
            slug="icon"
            group="Display"
            description="Spectrum-style icon primitive with centralized size/tone/accessibility/source state contracts and stable slot/data markers."
        >
            <Playground title="Size + Tone Matrix" code=matrix_code>
                <div class="docs-row">
                    <Icon size=IconSize::Sm tone=IconTone::Default decorative=true>
                        "✓"
                    </Icon>
                    <Icon size=IconSize::Md tone=IconTone::Muted decorative=true>
                        "⚙"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Accent decorative=true>
                        "★"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Danger decorative=true>
                        "⚠"
                    </Icon>
                </div>
            </Playground>

            <Playground title="Accessible + Disabled + Custom Class" code=states_code>
                <div class="docs-row">
                    <Icon
                        size=IconSize::Md
                        tone=IconTone::Accent
                        decorative=false
                        aria_label="Sync successful".to_string()
                    >
                        "✓"
                    </Icon>
                    <Icon
                        size=IconSize::Lg
                        tone=IconTone::Muted
                        disabled=true
                        class_name="docs-icon-custom".to_string()
                        decorative=true
                    >
                        "⚙"
                    </Icon>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn empty_state() -> AnyView {
    let tone_code = r#"<EmptyState
  title="No projects yet".to_string()
  description="Create your first project to unlock dashboards and team workflows.".to_string()
  tone=EmptyStateTone::Default
  icon=move || view! { <span>"📁"</span> }
  actions=move || view! {
    <ui_components::Button>"Create project"</ui_components::Button>
  }
/>
<EmptyState
  title="Nothing matched".to_string()
  description="Try a different query or clear filters.".to_string()
  tone=EmptyStateTone::Muted
  align=EmptyStateAlign::Center
/>"#;

    let state_code = r#"<EmptyState
  title="Deployments paused".to_string()
  description="Approvals are required before resuming this environment.".to_string()
  tone=EmptyStateTone::Accent
  compact=true
  bordered=true
  class_name="docs-empty-state-custom".to_string()
  icon=move || view! { <span>"⏸"</span> }
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
      "Review approvals"
    </ui_components::Button>
  }
/>"#;

    view! {
        <ComponentPage
            title="EmptyState"
            slug="empty-state"
            group="Display"
            description="Spectrum/HeroUI-style empty-state primitive with centralized tone/align/layout/source contracts and stable slot/data markers."
        >
            <Playground title="Tone + Alignment + Actions" code=tone_code>
                <div class="docs-stack">
                    <EmptyState
                        title="No projects yet".to_string()
                        description="Create your first project to unlock dashboards and team workflows.".to_string()
                        tone=EmptyStateTone::Default
                        icon=move || view! { <span>"📁"</span> }
                        actions=move || {
                            view! {
                                <ui_components::Button>
                                    "Create project"
                                </ui_components::Button>
                            }
                        }
                    />
                    <EmptyState
                        title="Nothing matched".to_string()
                        description="Try a different query or clear filters.".to_string()
                        tone=EmptyStateTone::Muted
                        align=EmptyStateAlign::Center
                    />
                </div>
            </Playground>

            <Playground title="Compact + Bordered + Custom Class" code=state_code>
                <EmptyState
                    title="Deployments paused".to_string()
                    description="Approvals are required before resuming this environment.".to_string()
                    tone=EmptyStateTone::Accent
                    compact=true
                    bordered=true
                    class_name="docs-empty-state-custom".to_string()
                    icon=move || view! { <span>"⏸"</span> }
                    actions=move || {
                        view! {
                            <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                                "Review approvals"
                            </ui_components::Button>
                        }
                    }
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn error_view() -> AnyView {
    let basic_code = r#"<ErrorView
  is_invalid=true
  message="Please enter a valid email address".to_string()
/>
<ErrorView
  is_invalid=false
  message="This error stays hidden until the field becomes invalid.".to_string()
/>"#;

    let state_code = r#"<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  compact=true
  bordered=true
  class_name="docs-error-view-custom".to_string()
  motion=ErrorViewMotion {
    hidden_translate_px: 12.0,
    hidden_opacity: 0.0,
    hidden_scale: 0.95,
    ..ErrorViewMotion::default()
  }
  icon=move || view! {
    <Icon size=IconSize::Sm tone=IconTone::Danger decorative=true>"⚠"</Icon>
  }
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
      "Retry"
    </ui_components::Button>
  }
>
  <span>"Validation failed. Check highlighted fields and retry."</span>
</ErrorView>"#;

    view! {
        <ComponentPage
            title="ErrorView"
            slug="error-view"
            group="Display"
            description="Spectrum/HeroUI-style validation error container with centralized visibility/content/source state contracts and spring-driven motion markers."
        >
            <Playground title="Invalid Visibility" code=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ErrorView
                        is_invalid=true
                        message="Please enter a valid email address".to_string()
                    />
                    <ErrorView
                        is_invalid=false
                        message="This error stays hidden until the field becomes invalid.".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Custom Content + Motion + Actions" code=state_code>
                <ErrorView
                    is_invalid=true
                    tone=ErrorViewTone::Neutral
                    compact=true
                    bordered=true
                    class_name="docs-error-view-custom".to_string()
                    motion=ErrorViewMotion {
                        hidden_translate_px: 12.0,
                        hidden_opacity: 0.0,
                        hidden_scale: 0.95,
                        ..ErrorViewMotion::default()
                    }
                    icon=move || {
                        view! {
                            <Icon size=IconSize::Sm tone=IconTone::Danger decorative=true>
                                "⚠"
                            </Icon>
                        }
                    }
                    actions=move || {
                        view! {
                            <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                                "Retry"
                            </ui_components::Button>
                        }
                    }
                >
                    <span>
                        "Validation failed. Check highlighted fields and retry."
                    </span>
                </ErrorView>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn pressable_feedback() -> AnyView {
    let (press_count, set_press_count) = signal(0u32);
    let on_press_count = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let basic_code = r#"<PressableFeedback
  effect=PressableFeedbackEffect::Highlight
  tone=PressableFeedbackTone::Accent
  on_press=on_press_count
>
  <div class="docs-ripple-surface">"Press me"</div>
</PressableFeedback>"#;

    let custom_code = r#"<PressableFeedback
  effect=PressableFeedbackEffect::HighlightRipple
  tone=PressableFeedbackTone::Neutral
  bounded=false
  motion=PressableFeedbackMotion {
    pressed_scale: 0.94,
    highlight_opacity: 0.2,
    ripple: RippleMotion {
      duration_ms: 720,
      ..RippleMotion::default()
    },
    ..PressableFeedbackMotion::default()
  }
  class_name="docs-pressable-feedback-custom".to_string()
>
  <div class="docs-ripple-surface docs-ripple-surface--accent">"Custom feedback"</div>
</PressableFeedback>

<PressableFeedback is_disabled=true effect=PressableFeedbackEffect::Highlight>
  <div class="docs-ripple-surface docs-ripple-surface--static">"Disabled"</div>
</PressableFeedback>"#;

    view! {
        <ComponentPage
            title="PressableFeedback"
            slug="pressable-feedback"
            group="Display"
            description="HeroUI-style press feedback container with centralized effect/tone/boundary/source contracts, spring-driven scale/highlight motion, and optional ripple composition."
        >
            <Playground title="Scale + Highlight" code=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <PressableFeedback
                        effect=PressableFeedbackEffect::Highlight
                        tone=PressableFeedbackTone::Accent
                        on_press=on_press_count
                    >
                        <div class="docs-ripple-surface">
                            "Press me"
                        </div>
                    </PressableFeedback>

                    <div class="ui-muted">
                        {move || format!("Press count: {}", press_count.get())}
                    </div>
                </div>
            </Playground>

            <Playground title="Highlight + Ripple + Custom Motion" code=custom_code>
                <div class="docs-stack docs-stack--tight">
                    <PressableFeedback
                        effect=PressableFeedbackEffect::HighlightRipple
                        tone=PressableFeedbackTone::Neutral
                        bounded=false
                        motion=PressableFeedbackMotion {
                            pressed_scale: 0.94,
                            highlight_opacity: 0.2,
                            ripple: RippleMotion {
                                duration_ms: 720,
                                ..RippleMotion::default()
                            },
                            ..PressableFeedbackMotion::default()
                        }
                        class_name="docs-pressable-feedback-custom".to_string()
                    >
                        <div class="docs-ripple-surface docs-ripple-surface--accent">
                            "Custom feedback"
                        </div>
                    </PressableFeedback>

                    <PressableFeedback
                        is_disabled=true
                        effect=PressableFeedbackEffect::Highlight
                    >
                        <div class="docs-ripple-surface docs-ripple-surface--static">
                            "Disabled"
                        </div>
                    </PressableFeedback>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_swatch() -> AnyView {
    let size_code = r##"<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Xs />
<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Sm />
<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Md />
<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Lg rounding=ColorSwatchRounding::Full />"##;

    let state_code = r#"<ColorSwatch
  color="rgba(38, 99, 235, 0.35)".to_string()
  color_name="Brand blue".to_string()
  aria_label="Background color".to_string()
  shape=ColorSwatchShape::Wide
  rounding=ColorSwatchRounding::Default
  class_name="docs-color-swatch-custom".to_string()
/>
<ColorSwatch
  color="rgba(255, 0, 0, 0)".to_string()
  color_name="No fill".to_string()
  bordered=true
/>
<ColorSwatch color="".to_string() bordered=true />"#;

    view! {
        <ComponentPage
            title="ColorSwatch"
            slug="color-swatch"
            group="Display"
            description="Spectrum-compatible color preview primitive with centralized size/rounding/shape/transparency/source contracts and stable slot/data markers."
        >
            <Playground title="Size + Rounding" code=size_code>
                <div class="docs-row">
                    <ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Xs />
                    <ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Sm />
                    <ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Md />
                    <ColorSwatch
                        color="#ffcc00".to_string()
                        size=ColorSwatchSize::Lg
                        rounding=ColorSwatchRounding::Full
                    />
                </div>
            </Playground>

            <Playground title="Transparency + Accessible Label + Shape" code=state_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorSwatch
                        color="rgba(38, 99, 235, 0.35)".to_string()
                        color_name="Brand blue".to_string()
                        aria_label="Background color".to_string()
                        shape=ColorSwatchShape::Wide
                        rounding=ColorSwatchRounding::Default
                        class_name="docs-color-swatch-custom".to_string()
                    />
                    <ColorSwatch
                        color="rgba(255, 0, 0, 0)".to_string()
                        color_name="No fill".to_string()
                        bordered=true
                    />
                    <ColorSwatch color="".to_string() bordered=true />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_swatch_picker() -> AnyView {
    let swatches = vec![
        ColorSwatchPickerItem::named("#A00", "Red"),
        ColorSwatchPickerItem::named("#f80", "Orange"),
        ColorSwatchPickerItem::named("#080", "Green"),
        ColorSwatchPickerItem::named("#08f", "Blue"),
    ];

    let disabled_swatches = vec![
        ColorSwatchPickerItem::named("#A00", "Red"),
        ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
        ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
        ColorSwatchPickerItem::new("#08f"),
    ];

    let basic_code = r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
/>"##;

    let state_code = r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
    ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
    ColorSwatchPickerItem::new("#08f"),
  ]).0
  shape=ColorSwatchShape::Wide
  rounding=ColorSwatchRounding::Default
  class_name="docs-color-swatch-picker-custom".to_string()
  aria_label="Fill color".to_string()
/>"##;

    view! {
        <ComponentPage
            title="ColorSwatchPicker"
            slug="color-swatch-picker"
            group="Display"
            description="Spectrum-compatible selectable swatch group with centralized color normalization, single-selection state, keyboard roving, and stable slot/data state markers."
        >
            <Playground title="Basic Selection" code=basic_code>
                <ColorSwatchPicker
                    swatches=signal(swatches).0
                    default_selected_color="#f80".to_string()
                />
            </Playground>

            <Playground title="Transparency + Disabled + Custom Class" code=state_code>
                <ColorSwatchPicker
                    swatches=signal(disabled_swatches).0
                    shape=ColorSwatchShape::Wide
                    rounding=ColorSwatchRounding::Default
                    class_name="docs-color-swatch-picker-custom".to_string()
                    aria_label="Fill color".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn skeleton_group() -> AnyView {
    let loading_code = r#"<SkeletonGroup
  is_loading=true
  variant=SkeletonGroupVariant::Shimmer
  layout=SkeletonGroupLayout::Vertical
>
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line docs-skeleton-line--short".to_string() />
</SkeletonGroup>"#;

    let state_code = r#"<SkeletonGroup
  is_loading=false
  is_skeleton_only=false
  variant=SkeletonGroupVariant::None
>
  <div class="ui-muted">"Loaded content rendered by parent group."</div>
</SkeletonGroup>

<SkeletonGroup
  is_loading=false
  is_skeleton_only=true
  variant=SkeletonGroupVariant::Pulse
  class_name="docs-skeleton-group-custom".to_string()
>
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
</SkeletonGroup>"#;

    view! {
        <ComponentPage
            title="SkeletonGroup"
            slug="skeleton-group"
            group="Display"
            description="Spectrum/HeroUI-style skeleton coordination container with centralized loading/layout/variant visibility contracts and stable slot/data-state markers."
        >
            <Playground title="Shimmer + Pulse Layout" code=loading_code>
                <div class="docs-stack">
                    <SkeletonGroup
                        is_loading=true
                        variant=SkeletonGroupVariant::Shimmer
                        layout=SkeletonGroupLayout::Vertical
                        density=SkeletonGroupDensity::Comfortable
                    >
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                        />
                    </SkeletonGroup>

                    <SkeletonGroup
                        is_loading=true
                        variant=SkeletonGroupVariant::Pulse
                        layout=SkeletonGroupLayout::Horizontal
                        density=SkeletonGroupDensity::Compact
                        aria_label="Profile placeholders".to_string()
                        class_name="docs-skeleton-group-custom".to_string()
                    >
                        <Skeleton
                            variant=SkeletonVariant::Circle
                            shimmer=false
                            class_name="docs-skeleton-avatar".to_string()
                        />
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
                    </SkeletonGroup>
                </div>
            </Playground>

            <Playground title="Loaded + Skeleton Only" code=state_code>
                <div class="docs-stack">
                    <SkeletonGroup
                        is_loading=false
                        is_skeleton_only=false
                        variant=SkeletonGroupVariant::None
                    >
                        <div class="ui-muted">
                            "Loaded content rendered by parent group."
                        </div>
                    </SkeletonGroup>

                    <SkeletonGroup
                        is_loading=false
                        is_skeleton_only=true
                        variant=SkeletonGroupVariant::Pulse
                        class_name="docs-skeleton-group-custom".to_string()
                    >
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                        />
                    </SkeletonGroup>

                    <div class="ui-muted">
                        "When `is_skeleton_only=true` and loading is finished, the skeleton group hides itself."
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn chart() -> AnyView {
    let revenue_points = vec![
        ChartPoint::new("jan", "Jan", 12.0),
        ChartPoint::new("feb", "Feb", 18.5),
        ChartPoint::new("mar", "Mar", 17.2),
        ChartPoint::new("apr", "Apr", 24.7),
        ChartPoint::new("may", "May", 28.1),
    ];

    let line_points = vec![
        ChartPoint::new("q1", "Q1", 42.0),
        ChartPoint::new("q2", "Q2", 56.0),
        ChartPoint::new("q3", "Q3", 51.0),
        ChartPoint::new("q4", "Q4", 63.0),
    ];

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (controlled_active_raw, set_controlled_active_raw) = signal(1_usize);
    let controlled_active: Signal<usize> = Signal::derive(move || controlled_active_raw.get());
    let on_controlled_active_change =
        Callback::new(move |next: usize| set_controlled_active_raw.set(next));

    let bar_code = r#"<Chart
  id_base="docs-chart-bar".to_string()
  points=points
  kind=ChartKind::Bar
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>"#;

    let line_code = r#"let (active_raw, set_active_raw) = signal(1_usize);
let active: Signal<usize> = Signal::derive(move || active_raw.get());

<Chart
  id_base="docs-chart-line".to_string()
  points=points
  kind=ChartKind::Line
  active_index=active
  on_active_index_change=Callback::new(move |next| set_active_raw.set(next))
  class_name="docs-chart-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Chart"
            slug="chart"
            group="Display"
            description="Shadcn-compatible chart primitive with bar/line modes, controlled active-index state, Spectrum-style data contracts, and HeroUI-level spring highlight motion for legends."
        >
            <Playground title="Bar + Hover/Keyboard + Action" code=bar_code>
                <div class="docs-stack docs-stack--tight">
                    <Chart
                        id_base="docs-chart-bar".to_string()
                        points=revenue_points
                        kind=ChartKind::Bar
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled Line + Active Index" code=line_code>
                <div class="docs-stack docs-stack--tight">
                    <Chart
                        id_base="docs-chart-line".to_string()
                        points=line_points
                        kind=ChartKind::Line
                        active_index=controlled_active
                        on_active_index_change=on_controlled_active_change
                        aria_label="Quarterly growth line chart".to_string()
                        class_name="docs-chart-custom".to_string()
                    />
                    <span class="ui-muted">
                        "controlled active index: "
                        {move || controlled_active_raw.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
