use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    ActionButton, ActionButtonGroup, ActionButtonGroupDensity, ActionButtonGroupOrientation,
    ActionButtonLoadingPlacement, ActionButtonSize, ActionMenu, Button, ButtonCopy, ButtonGroup,
    ButtonGroupOrientation, ButtonLoadingPlacement, ButtonSize, ButtonVariant, FlipButton,
    FlipDirection, IconButton, LinkButton, MenuItemKind, OnPress, SearchInputButton,
    SegmentedControl, SegmentedControlSize, ShareButton, ShareButtonIconPlacement, ShareButtonItem,
    SharePlatform, Switch, ThemeMode, ThemeToggleButton, ToggleButton, ToggleButtonGroup,
    ToggleButtonGroupOrientation, ToggleButtonSize, ToggleButtonVariant,
};

pub(super) fn button() -> AnyView {
    let variant_options = vec![
        "Primary".to_string(),
        "Outline".to_string(),
        "Ghost".to_string(),
        "Danger".to_string(),
        "Secondary".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Outline,
        2 => ButtonVariant::Ghost,
        3 => ButtonVariant::Destructive,
        4 => ButtonVariant::Secondary,
        _ => ButtonVariant::Default,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        2 => ButtonSize::M,
        3 => ButtonSize::L,
        _ => ButtonSize::Xl,
    });

    let (disabled, set_disabled) = signal(false);
    let (loading, set_loading) = signal(false);
    let loading_placement_options =
        vec!["Start".to_string(), "End".to_string(), "Center".to_string()];
    let (loading_placement_index, set_loading_placement_index) = signal(Some(0_usize));
    let loading_placement =
        Signal::derive(move || match loading_placement_index.get().unwrap_or(0) {
            1 => ButtonLoadingPlacement::End,
            2 => ButtonLoadingPlacement::Center,
            _ => ButtonLoadingPlacement::Start,
        });
    let (icon_only, set_icon_only) = signal(false);
    let (full_width, set_full_width) = signal(false);
    let (show_start, set_show_start) = signal(false);
    let (show_end, set_show_end) = signal(false);

    let code = Signal::derive(move || {
        let variant = variant.get();
        let size = size.get();
        let disabled = disabled.get();
        let loading = loading.get();
        let loading_placement = loading_placement.get();
        let icon_only = icon_only.get();
        let full_width = full_width.get();
        let show_start = show_start.get();
        let show_end = show_end.get();

        let mut snippet = vec!["<Button".to_string()];

        if variant != ButtonVariant::Default {
            snippet.push(format!("  variant=ButtonVariant::{variant:?}"));
        }
        if size != ButtonSize::M {
            snippet.push(format!("  size=ButtonSize::{size:?}"));
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }
        if loading {
            snippet.push("  is_loading=true".to_string());
            if loading_placement != ButtonLoadingPlacement::Start {
                snippet.push(format!(
                    "  loading_placement=ButtonLoadingPlacement::{loading_placement:?}"
                ));
            }
        }
        if icon_only {
            snippet.push("  is_icon_only=true".to_string());
            snippet.push("  aria_label=\"Button\".to_string()".to_string());
        }
        if full_width {
            snippet.push("  full_width=true".to_string());
        }
        if show_start {
            snippet.push("  start_content=move || view! { <span>\"★\"</span> }".to_string());
        }
        if show_end {
            snippet.push("  end_content=move || view! { <span>\"→\"</span> }".to_string());
        }

        snippet.extend([
            ">".to_string(),
            if icon_only {
                "  \"★\"".to_string()
            } else {
                "  \"Button\"".to_string()
            },
            "</Button>".to_string(),
        ]);

        snippet.join("\n")
    });

    view! {
        <ComponentPage
            title="Button"
            slug="button"
            group="Actions"
            description="Variants + sizes with spring hover/tap motion."
        >
            <Playground
                title="Variants & sizes"
                code_signal=code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button size".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=loading set_checked=set_loading>"Loading"</Switch>
                        <div class="docs-search__label">"Loading placement"</div>
                        <SegmentedControl
                            id_base="docs-button-loading-placement".to_string()
                            options=loading_placement_options.clone()
                            selected_index=loading_placement_index
                            set_selected_index=set_loading_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button loading placement".to_string()
                        />
                        <Switch checked=icon_only set_checked=set_icon_only>"Icon only"</Switch>
                        <Switch checked=full_width set_checked=set_full_width>"Full width"</Switch>
                        <Switch checked=show_start set_checked=set_show_start>"Start slot"</Switch>
                        <Switch checked=show_end set_checked=set_show_end>"End slot"</Switch>
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let size = size.get();
                    let disabled = disabled.get();
                    let is_loading = loading.get();
                    let loading_placement = loading_placement.get();
                    let icon_only = icon_only.get();
                    let full_width = full_width.get();
                    let show_start = show_start.get();
                    let show_end = show_end.get();

                    view! {
                        <div class="docs-stack" style="width: min(100%, 360px);">
                            <div class="docs-row" style="width: 100%;">
                                {match (show_start, show_end) {
                                    (true, true) => view! {
                                        <Button
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            is_loading=is_loading
                                            loading_placement=loading_placement
                                            is_icon_only=icon_only
                                            full_width=full_width
                                            aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            start_content=move || view! { <span>"★"</span> }
                                            end_content=move || view! { <span>"→"</span> }
                                        >
                                            {if icon_only { "★" } else { "Button" }}
                                        </Button>
                                    }
                                        .into_any(),
                                    (true, false) => view! {
                                        <Button
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            is_loading=is_loading
                                            loading_placement=loading_placement
                                            is_icon_only=icon_only
                                            full_width=full_width
                                            aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            start_content=move || view! { <span>"★"</span> }
                                        >
                                            {if icon_only { "★" } else { "Button" }}
                                        </Button>
                                    }
                                        .into_any(),
                                    (false, true) => view! {
                                        <Button
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            is_loading=is_loading
                                            loading_placement=loading_placement
                                            is_icon_only=icon_only
                                            full_width=full_width
                                            aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            end_content=move || view! { <span>"→"</span> }
                                        >
                                            {if icon_only { "★" } else { "Button" }}
                                        </Button>
                                    }
                                        .into_any(),
                                    (false, false) => view! {
                                        <Button
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            is_loading=is_loading
                                            loading_placement=loading_placement
                                            is_icon_only=icon_only
                                            full_width=full_width
                                            aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                        >
                                            {if icon_only { "★" } else { "Button" }}
                                        </Button>
                                    }
                                        .into_any(),
                                }}
                            </div>
                            <span class="ui-muted">
                                {format!(
                                    "{variant:?} · {size:?} · {loading_placement:?} · icon_only={icon_only} · full_width={full_width}"
                                )}
                            </span>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_button() -> AnyView {
    let (press_count, set_press_count) = signal(0_u32);
    let on_press: OnPress = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let code = Signal::derive(move || {
        r#"<ActionButton
  on_press=Callback::new(move |_| {})
>
  "Action"
</ActionButton>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ActionButton
  is_loading=true
  loading_placement=ActionButtonLoadingPlacement::Start
  start_content=move || view! { <span>"★"</span> }
>
  "Start"
</ActionButton>
<ActionButton
  is_loading=true
  loading_placement=ActionButtonLoadingPlacement::End
  end_content=move || view! { <span>"→"</span> }
>
  "End"
</ActionButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ActionButton"
            slug="action-button"
            group="Actions"
            description="Spectrum-style action trigger with state attrs and HeroUI-grade spring hover/press feedback."
        >
            <Playground title="Default + callback" code_signal=code>
                <div class="docs-row">
                    <ActionButton on_press=on_press>"Action"</ActionButton>
                    <ActionButton is_quiet=true on_press=on_press>"Quiet"</ActionButton>
                    <ActionButton
                        is_loading=true
                        loading_placement=ActionButtonLoadingPlacement::Center
                    >
                        "Loading"
                    </ActionButton>
                    <span class="ui-muted">
                        "pressed: "
                        {move || press_count.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Loading placement + icon-only" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ActionButton
                            size=ActionButtonSize::S
                            is_loading=true
                            loading_placement=ActionButtonLoadingPlacement::Start
                            start_content=move || view! { <span>"★"</span> }
                        >
                            "Start"
                        </ActionButton>
                        <ActionButton
                            size=ActionButtonSize::L
                            is_loading=true
                            loading_placement=ActionButtonLoadingPlacement::End
                            end_content=move || view! { <span>"→"</span> }
                        >
                            "End"
                        </ActionButton>
                        <ActionButton is_icon_only=true is_quiet=true aria_label="Settings".to_string()>
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <path
                                    d="M10 13.3a3.3 3.3 0 1 0 0-6.6a3.3 3.3 0 0 0 0 6.6Z"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M3.8 10a6.2 6.2 0 0 1 .1-1l1.6-.9.2-.5-.6-1.8a7.6 7.6 0 0 1 1.5-1.5l1.8.6.5-.2.9-1.6a6.4 6.4 0 0 1 2 0l.9 1.6.5.2 1.8-.6c.6.4 1.1.9 1.5 1.5l-.6 1.8.2.5 1.6.9a6.5 6.5 0 0 1 0 2l-1.6.9-.2.5.6 1.8a7.6 7.6 0 0 1-1.5 1.5l-1.8-.6-.5.2-.9 1.6a6.4 6.4 0 0 1-2 0l-.9-1.6-.5-.2-1.8.6a7.6 7.6 0 0 1-1.5-1.5l.6-1.8-.2-.5-1.6-.9a6.2 6.2 0 0 1-.1-1Z"
                                    stroke="currentColor"
                                    stroke_width="1.2"
                                    stroke_linecap="round"
                                    stroke_linejoin="round"
                                />
                            </svg>
                        </ActionButton>
                    </div>
                    <span class="ui-muted">
                        "Start/end slots, loading placement, and icon-only mode all expose stable data-* attrs."
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_button_group() -> AnyView {
    let (press_count, set_press_count) = signal(0_u32);
    let on_press: OnPress = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let code = Signal::derive(move || {
        r#"<ActionButtonGroup
  size=ActionButtonSize::S
  density=ActionButtonGroupDensity::Compact
  is_quiet=true
>
  <ActionButton on_press=Callback::new(move |_| {})>"One"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Two"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Three"</ActionButton>
</ActionButtonGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ActionButtonGroup
  orientation=ActionButtonGroupOrientation::Vertical
  is_justified=true
  aria_label="Vertical actions".to_string()
>
  <ActionButton>"Top"</ActionButton>
  <ActionButton>"Bottom"</ActionButton>
</ActionButtonGroup>

<ActionButtonGroup disabled=true density=ActionButtonGroupDensity::Compact>
  <ActionButton>"Disabled"</ActionButton>
  <ActionButton>"Group"</ActionButton>
</ActionButtonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ActionButtonGroup"
            slug="action-button-group"
            group="Actions"
            description="Toolbar-style action clusters with Spectrum state attrs for orientation, density, quiet/filled, and enablement."
        >
            <Playground title="Default + compact" code_signal=code>
                <div class="docs-stack">
                    <ActionButtonGroup
                        size=ActionButtonSize::S
                        density=ActionButtonGroupDensity::Compact
                        orientation=ActionButtonGroupOrientation::Horizontal
                        is_quiet=true
                    >
                        <ActionButton on_press=on_press>"One"</ActionButton>
                        <ActionButton on_press=on_press>"Two"</ActionButton>
                        <ActionButton on_press=on_press>"Three"</ActionButton>
                    </ActionButtonGroup>
                    <span class="ui-muted">
                        "pressed: "
                        {move || press_count.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Vertical + justified + disabled" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ActionButtonGroup
                            size=ActionButtonSize::M
                            orientation=ActionButtonGroupOrientation::Vertical
                            is_justified=true
                            aria_label="Vertical actions".to_string()
                        >
                            <ActionButton>"Top"</ActionButton>
                            <ActionButton>"Middle"</ActionButton>
                            <ActionButton>"Bottom"</ActionButton>
                        </ActionButtonGroup>

                        <ActionButtonGroup
                            size=ActionButtonSize::S
                            density=ActionButtonGroupDensity::Compact
                            disabled=true
                            aria_label="Disabled actions".to_string()
                        >
                            <ActionButton>"Disabled"</ActionButton>
                            <ActionButton>"Group"</ActionButton>
                        </ActionButtonGroup>
                    </div>
                    <span class="ui-muted">
                        "Vertical/compact/disabled/justified are all reflected via stable data-* attrs for Spectrum-level styling contracts."
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn button_group() -> AnyView {
    let (left_count, set_left_count) = signal(0_usize);
    let (middle_count, set_middle_count) = signal(0_usize);
    let (right_count, set_right_count) = signal(0_usize);

    let on_left: OnPress = Callback::new(move |_| {
        set_left_count.update(|count| *count += 1);
    });
    let on_middle: OnPress = Callback::new(move |_| {
        set_middle_count.update(|count| *count += 1);
    });
    let on_right: OnPress = Callback::new(move |_| {
        set_right_count.update(|count| *count += 1);
    });

    let (top_count, set_top_count) = signal(0_usize);
    let (bottom_count, set_bottom_count) = signal(0_usize);
    let on_top: OnPress = Callback::new(move |_| {
        set_top_count.update(|count| *count += 1);
    });
    let on_bottom: OnPress = Callback::new(move |_| {
        set_bottom_count.update(|count| *count += 1);
    });

    let code = Signal::derive(move || {
        r#"<ButtonGroup attached=true>
  <Button variant=ButtonVariant::Secondary>"Left"</Button>
  <Button variant=ButtonVariant::Secondary>"Middle"</Button>
  <Button variant=ButtonVariant::Secondary>"Right"</Button>
</ButtonGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ButtonGroup
  attached=false
  orientation=ButtonGroupOrientation::Vertical
  aria_label="Document actions".to_string()
>
  <Button variant=ButtonVariant::Outline>"Top"</Button>
  <Button variant=ButtonVariant::Outline disabled=true>"Disabled"</Button>
  <Button variant=ButtonVariant::Outline>"Bottom"</Button>
</ButtonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ButtonGroup"
            slug="button-group"
            group="Actions"
            description="Groups Buttons with Spectrum-style root state attrs for orientation, attachment, and accessible labeling."
        >
            <Playground title="Attached horizontal" code_signal=code>
                <div class="docs-stack">
                    <ButtonGroup attached=true orientation=ButtonGroupOrientation::Horizontal>
                        <Button variant=ButtonVariant::Secondary on_press=on_left>
                            "Left"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_middle>
                            "Middle"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_right>
                            "Right"
                        </Button>
                    </ButtonGroup>
                    <span class="ui-muted">
                        "left/middle/right clicks: "
                        {move || format!(
                            "{}/{}/{}",
                            left_count.get(),
                            middle_count.get(),
                            right_count.get()
                        )}
                    </span>
                </div>
            </Playground>

            <Playground title="Vertical + detached" code_signal=states_code>
                <div class="docs-stack">
                    <ButtonGroup
                        attached=false
                        orientation=ButtonGroupOrientation::Vertical
                        aria_label="Document actions".to_string()
                    >
                        <Button variant=ButtonVariant::Outline on_press=on_top>
                            "Top"
                        </Button>
                        <Button variant=ButtonVariant::Outline disabled=true>
                            "Disabled"
                        </Button>
                        <Button variant=ButtonVariant::Outline on_press=on_bottom>
                            "Bottom"
                        </Button>
                    </ButtonGroup>
                    <span class="ui-muted">
                        "top/bottom clicks: "
                        {move || format!("{}/{}", top_count.get(), bottom_count.get())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn icon_button() -> AnyView {
    let (close_count, set_close_count) = signal(0_usize);
    let (search_count, set_search_count) = signal(0_usize);

    let on_close: OnPress = Callback::new(move |_| {
        set_close_count.update(|count| *count += 1);
    });
    let on_search: OnPress = Callback::new(move |_| {
        set_search_count.update(|count| *count += 1);
    });

    let variant_options = vec![
        "Default".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Outline".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Secondary,
        2 => ButtonVariant::Ghost,
        3 => ButtonVariant::Outline,
        _ => ButtonVariant::Default,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::IconXs,
        1 => ButtonSize::IconS,
        2 => ButtonSize::Icon,
        3 => ButtonSize::IconL,
        _ => ButtonSize::IconXl,
    });

    let (search_disabled, set_search_disabled) = signal(false);

    let code = Signal::derive(move || {
        let variant = variant.get();
        let size = size.get();
        let disabled = search_disabled.get();

        let mut snippet = vec![
            "<IconButton".to_string(),
            "  aria_label=\"Search\".to_string()".to_string(),
        ];

        if variant != ButtonVariant::Default {
            snippet.push(format!("  variant=ButtonVariant::{variant:?}"));
        }
        if size != ButtonSize::Icon {
            snippet.push(format!("  size=ButtonSize::{size:?}"));
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }

        snippet.extend([
            ">".to_string(),
            "  <span aria-hidden=\"true\">\"⌕\"</span>".to_string(),
            "</IconButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let states_code = Signal::derive(move || {
        r#"<IconButton aria_label="Search xs".to_string() size=ButtonSize::IconXs>
  <svg ... />
</IconButton>
<IconButton aria_label="Search s".to_string() size=ButtonSize::IconS>
  <svg ... />
</IconButton>
<IconButton aria_label="Search m".to_string() size=ButtonSize::IconM>
  <svg ... />
</IconButton>
<IconButton aria_label="Search l".to_string() size=ButtonSize::IconL>
  <svg ... />
</IconButton>
<IconButton aria_label="Search xl".to_string() size=ButtonSize::IconXl disabled=true>
  <svg ... />
</IconButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="IconButton"
            slug="icon-button"
            group="Actions"
            description="A Button wrapper that enforces accessible labeling and icon sizing while preserving motion/press semantics."
        >
            <Playground
                title="on_press + variants"
                code_signal=code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-icon-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="IconButton variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-icon-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="IconButton size".to_string()
                        />

                        <Switch checked=search_disabled set_checked=set_search_disabled>
                            "Disable search button"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <IconButton
                            aria_label="Close dialog".to_string()
                            variant=ButtonVariant::Ghost
                            on_press=on_close
                        >
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <path
                                    d="M5 5l10 10M15 5L5 15"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                    stroke_linejoin="round"
                                />
                            </svg>
                        </IconButton>
                        <IconButton
                            aria_label="Search".to_string()
                            variant=variant.get()
                            size=size.get()
                            disabled=search_disabled.get()
                            on_press=on_search
                        >
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle
                                    cx="9"
                                    cy="9"
                                    r="6"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M13.5 13.5l3 3"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                            </svg>
                        </IconButton>
                    </div>
                    <span class="ui-muted">
                        "close/search presses: "
                        {move || format!("{}/{}", close_count.get(), search_count.get())}
                    </span>
                    <span class="ui-muted">{move || format!("{:?} · {:?}", variant.get(), size.get())}</span>
                </div>
            </Playground>

            <Playground title="Size + disabled matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <IconButton aria_label="Search xs".to_string() size=ButtonSize::IconXs>
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle
                                    cx="9"
                                    cy="9"
                                    r="6"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M13.5 13.5l3 3"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                            </svg>
                        </IconButton>
                        <IconButton aria_label="Search s".to_string() size=ButtonSize::IconS>
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle
                                    cx="9"
                                    cy="9"
                                    r="6"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M13.5 13.5l3 3"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                            </svg>
                        </IconButton>
                        <IconButton aria_label="Search m".to_string() size=ButtonSize::IconM>
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle
                                    cx="9"
                                    cy="9"
                                    r="6"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M13.5 13.5l3 3"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                            </svg>
                        </IconButton>
                        <IconButton aria_label="Search l".to_string() size=ButtonSize::IconL>
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle
                                    cx="9"
                                    cy="9"
                                    r="6"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M13.5 13.5l3 3"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                            </svg>
                        </IconButton>
                        <IconButton aria_label="Search xl".to_string() size=ButtonSize::IconXl>
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle
                                    cx="9"
                                    cy="9"
                                    r="6"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M13.5 13.5l3 3"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                            </svg>
                        </IconButton>
                    </div>
                    <div class="docs-row">
                        <IconButton
                            aria_label="Close disabled".to_string()
                            variant=ButtonVariant::Ghost
                            disabled=true
                        >
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <path
                                    d="M5 5l10 10M15 5L5 15"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                    stroke_linejoin="round"
                                />
                            </svg>
                        </IconButton>
                        <IconButton
                            aria_label="Search disabled".to_string()
                            variant=ButtonVariant::Secondary
                            disabled=true
                        >
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <circle
                                    cx="9"
                                    cy="9"
                                    r="6"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M13.5 13.5l3 3"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                            </svg>
                        </IconButton>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn link_button() -> AnyView {
    let variant_options = vec![
        "Default".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Outline".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Secondary,
        2 => ButtonVariant::Ghost,
        3 => ButtonVariant::Outline,
        _ => ButtonVariant::Default,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        2 => ButtonSize::M,
        3 => ButtonSize::L,
        _ => ButtonSize::Xl,
    });

    let (disabled, set_disabled) = signal(false);
    let (open_in_new_tab, set_open_in_new_tab) = signal(false);
    let (sponsored_rel, set_sponsored_rel) = signal(false);

    let code = Signal::derive(move || {
        let variant = variant.get();
        let size = size.get();
        let disabled = disabled.get();

        let mut snippet = vec![
            "<LinkButton".to_string(),
            "  href=\"https://example.com/docs\".to_string()".to_string(),
        ];

        if variant != ButtonVariant::Default {
            snippet.push(format!("  variant=ButtonVariant::{variant:?}"));
        }
        if size != ButtonSize::M {
            snippet.push(format!("  size=ButtonSize::{size:?}"));
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }
        if open_in_new_tab.get() {
            snippet.push("  target=Some(\"_blank\")".to_string());
        }
        if sponsored_rel.get() {
            snippet.push("  rel=Some(\"sponsored\".to_string())".to_string());
        }

        snippet.extend([
            ">".to_string(),
            "  \"Open docs\"".to_string(),
            "</LinkButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let states_code = Signal::derive(move || {
        r#"<LinkButton href="https://example.com/xs".to_string() size=ButtonSize::Xs>
  "xs"
</LinkButton>
<LinkButton href="https://example.com/s".to_string() size=ButtonSize::S>
  "s"
</LinkButton>
<LinkButton href="https://example.com/m".to_string() size=ButtonSize::M>
  "m"
</LinkButton>
<LinkButton
  href="https://example.com/l".to_string()
  size=ButtonSize::L
  variant=ButtonVariant::Secondary
>
  "l secondary"
</LinkButton>
<LinkButton
  href="https://example.com/xl".to_string()
  size=ButtonSize::Xl
>
  "xl"
</LinkButton>
<LinkButton href="https://example.com/disabled".to_string() disabled=true>
  "Disabled"
</LinkButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="LinkButton"
            slug="link-button"
            group="Actions"
            description="Button styling on anchors with Spectrum-style disabled semantics and secure rel handling for external targets."
        >
            <Playground
                title="External target + rel hardening"
                code_signal=code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-link-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="LinkButton variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-link-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="LinkButton size".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=open_in_new_tab set_checked=set_open_in_new_tab>
                            "Open in new tab (_blank)"
                        </Switch>
                        <Switch checked=sponsored_rel set_checked=set_sponsored_rel>
                            "Add sponsored rel"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    {move || {
                        let variant = variant.get();
                        let size = size.get();
                        let disabled = disabled.get();
                        let rel = if sponsored_rel.get() {
                            "sponsored".to_string()
                        } else {
                            String::new()
                        };

                        view! {
                            <div class="docs-row">
                                {if open_in_new_tab.get() {
                                    view! {
                                        <LinkButton
                                            href="https://example.com/docs".to_string()
                                            target="_blank"
                                            rel=rel.clone()
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            aria_label="Open docs in a new tab".to_string()
                                        >
                                            "Open docs"
                                        </LinkButton>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <LinkButton
                                            href="https://example.com/docs".to_string()
                                            rel=rel
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            aria_label="Open docs in the same tab".to_string()
                                        >
                                            "Open docs"
                                        </LinkButton>
                                    }
                                        .into_any()
                                }}
                                <LinkButton href="https://example.com/changelog".to_string()>
                                    "Same tab"
                                </LinkButton>
                                <LinkButton href="   ".to_string() variant=ButtonVariant::Ghost>
                                    "Missing href"
                                </LinkButton>
                            </div>
                        }
                    }}
                    <span class="ui-muted">
                        "_blank links auto-append noopener+noreferrer; blank href is normalized as non-navigable."
                    </span>
                </div>
            </Playground>

            <Playground title="Variant + size + disabled matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <LinkButton href="https://example.com/xs".to_string() size=ButtonSize::Xs>
                            "xs"
                        </LinkButton>
                        <LinkButton href="https://example.com/s".to_string() size=ButtonSize::S>
                            "s"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/m".to_string()
                            size=ButtonSize::M
                        >
                            "m"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/l".to_string()
                            size=ButtonSize::L
                            variant=ButtonVariant::Secondary
                        >
                            "l secondary"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/xl".to_string()
                            size=ButtonSize::Xl
                        >
                            "xl"
                        </LinkButton>
                    </div>
                    <div class="docs-row">
                        <LinkButton href="https://example.com/disabled".to_string() disabled=true>
                            "Disabled"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/disabled-ghost".to_string()
                            variant=ButtonVariant::Ghost
                            disabled=true
                        >
                            "Disabled ghost"
                        </LinkButton>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn toggle_button() -> AnyView {
    let (selected, set_selected) = signal(false);
    let (last_change, set_last_change) = signal("none".to_string());
    let on_toggle_change = Callback::new(move |next: bool| {
        set_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
    });

    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Outline".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Destructive".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Accent,
        2 => ToggleButtonVariant::Outline,
        3 => ToggleButtonVariant::Secondary,
        4 => ToggleButtonVariant::Ghost,
        5 => ToggleButtonVariant::Destructive,
        _ => ToggleButtonVariant::Default,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::S,
        2 => ToggleButtonSize::M,
        3 => ToggleButtonSize::L,
        _ => ToggleButtonSize::Xl,
    });

    let (disabled, set_disabled) = signal(false);

    let code = Signal::derive(move || {
        let variant = variant.get();
        let size = size.get();
        let disabled = disabled.get();

        let mut snippet = vec![
            "let (selected, set_selected) = signal(false);".to_string(),
            String::new(),
            "<ToggleButton".to_string(),
            "  selected=selected".to_string(),
            "  set_selected=set_selected".to_string(),
        ];

        if variant != ToggleButtonVariant::Default {
            snippet.push(format!("  variant=ToggleButtonVariant::{variant:?}"));
        }
        if size != ToggleButtonSize::M {
            snippet.push(format!("  size=ToggleButtonSize::{size:?}"));
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }

        snippet.extend([
            ">".to_string(),
            "  \"Toggle\"".to_string(),
            "</ToggleButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let (notifications, set_notifications) = signal(true);
    let (disabled_selected, set_disabled_selected) = signal(true);
    let (disabled_unselected, set_disabled_unselected) = signal(false);

    let states_code = Signal::derive(move || {
        r#"<ToggleButton
  selected=notifications
  set_selected=set_notifications
  variant=ToggleButtonVariant::Accent
  size=ToggleButtonSize::L
>
  "Notifications"
</ToggleButton>
<ToggleButton selected=disabled_selected set_selected=set_disabled_selected disabled=true>
  "Disabled on"
</ToggleButton>
<ToggleButton selected=disabled_unselected set_selected=set_disabled_unselected disabled=true>
  "Disabled off"
</ToggleButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ToggleButton"
            slug="toggle-button"
            group="Actions"
            description="Pressable toggle state with HeroUI-level spring motion and Spectrum-style root state attrs."
        >
            <Playground
                title="Controlled + on_change"
                code_signal=code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButton variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButton size".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let size = size.get();
                    let disabled = disabled.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                <ToggleButton
                                    selected=selected
                                    set_selected=set_selected
                                    on_change=on_toggle_change
                                    variant=variant
                                    size=size
                                    disabled=disabled
                                >
                                    "Toggle"
                                </ToggleButton>
                                <span class="ui-muted">
                                    "selected: "
                                    {move || selected.get().to_string()}
                                </span>
                            </div>
                            <span class="ui-muted">"last on_change: " {move || last_change.get()}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Variant + size + disabled matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ToggleButton
                            selected=notifications
                            set_selected=set_notifications
                            variant=ToggleButtonVariant::Accent
                            size=ToggleButtonSize::L
                        >
                            "Notifications"
                        </ToggleButton>
                        <span class="ui-muted">
                            "notifications: "
                            {move || notifications.get().to_string()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <ToggleButton
                            selected=disabled_selected
                            set_selected=set_disabled_selected
                            disabled=true
                        >
                            "Disabled on"
                        </ToggleButton>
                        <ToggleButton
                            selected=disabled_unselected
                            set_selected=set_disabled_unselected
                            disabled=true
                        >
                            "Disabled off"
                        </ToggleButton>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn toggle_button_group() -> AnyView {
    let (a, set_a) = signal(false);
    let (b, set_b) = signal(true);
    let (c, set_c) = signal(false);
    let attached_selected_count =
        Signal::derive(move || usize::from(a.get()) + usize::from(b.get()) + usize::from(c.get()));

    let orientation_options = vec!["Horizontal".to_string(), "Vertical".to_string()];
    let (orientation_index, set_orientation_index) = signal(Some(0_usize));
    let orientation = Signal::derive(move || match orientation_index.get().unwrap_or(0) {
        1 => ToggleButtonGroupOrientation::Vertical,
        _ => ToggleButtonGroupOrientation::Horizontal,
    });

    let (attached, set_attached) = signal(false);

    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Outline".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Destructive".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Accent,
        2 => ToggleButtonVariant::Outline,
        3 => ToggleButtonVariant::Secondary,
        4 => ToggleButtonVariant::Ghost,
        5 => ToggleButtonVariant::Destructive,
        _ => ToggleButtonVariant::Default,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::S,
        2 => ToggleButtonSize::M,
        3 => ToggleButtonSize::L,
        _ => ToggleButtonSize::Xl,
    });

    let code = Signal::derive(move || {
        let orientation = orientation.get();
        let attached = attached.get();
        let variant = variant.get();
        let size = size.get();

        let mut toggle_props = String::new();
        if variant != ToggleButtonVariant::Default {
            toggle_props.push_str(&format!(" variant=ToggleButtonVariant::{variant:?}"));
        }
        if size != ToggleButtonSize::M {
            toggle_props.push_str(&format!(" size=ToggleButtonSize::{size:?}"));
        }

        let mut snippet = vec![
            "let (bold, set_bold) = signal(false);".to_string(),
            "let (italic, set_italic) = signal(true);".to_string(),
            "let (underline, set_underline) = signal(false);".to_string(),
            String::new(),
            "<ToggleButtonGroup".to_string(),
        ];

        if orientation != ToggleButtonGroupOrientation::Horizontal {
            snippet.push(format!(
                "  orientation=ToggleButtonGroupOrientation::{orientation:?}"
            ));
        }
        if attached {
            snippet.push("  attached=true".to_string());
        }

        snippet.extend([
            ">".to_string(),
            format!(
                "  <ToggleButton selected=bold set_selected=set_bold{toggle_props}>\"Bold\"</ToggleButton>"
            ),
            format!(
                "  <ToggleButton selected=italic set_selected=set_italic{toggle_props}>\"Italic\"</ToggleButton>"
            ),
            format!(
                "  <ToggleButton selected=underline set_selected=set_underline{toggle_props}>\"Underline\"</ToggleButton>"
            ),
            "</ToggleButtonGroup>".to_string(),
        ]);

        snippet.join("\n")
    });

    let (left, set_left) = signal(true);
    let (center, set_center) = signal(false);
    let (right, set_right) = signal(true);
    let detached_selected_count = Signal::derive(move || {
        usize::from(left.get()) + usize::from(center.get()) + usize::from(right.get())
    });

    let states_code = Signal::derive(move || {
        r#"<ToggleButtonGroup
  orientation=ToggleButtonGroupOrientation::Vertical
  attached=false
  aria_label="Alignment controls".to_string()
>
  <ToggleButton selected=left set_selected=set_left>"Left"</ToggleButton>
  <ToggleButton selected=center set_selected=set_center>"Center"</ToggleButton>
  <ToggleButton selected=right set_selected=set_right>"Right"</ToggleButton>
</ToggleButtonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ToggleButtonGroup"
            slug="toggle-button-group"
            group="Actions"
            description="Layout wrapper with Spectrum-style root state attrs for orientation, attachment, and accessible labeling."
        >
            <Playground
                title="Attached horizontal"
                code_signal=code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Orientation"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=orientation_index
                            set_selected_index=set_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup orientation".to_string()
                        />

                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup size".to_string()
                        />

                        <Switch checked=attached set_checked=set_attached>
                            "Attached layout"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let orientation = orientation.get();
                    let attached = attached.get();
                    let variant = variant.get();
                    let size = size.get();

                    view! {
                        <div class="docs-stack">
                            <ToggleButtonGroup
                                orientation=orientation
                                attached=attached
                                aria_label="Formatting controls".to_string()
                            >
                                <ToggleButton
                                    selected=a
                                    set_selected=set_a
                                    variant=variant
                                    size=size
                                >
                                    "Bold"
                                </ToggleButton>
                                <ToggleButton
                                    selected=b
                                    set_selected=set_b
                                    variant=variant
                                    size=size
                                >
                                    "Italic"
                                </ToggleButton>
                                <ToggleButton
                                    selected=c
                                    set_selected=set_c
                                    variant=variant
                                    size=size
                                >
                                    "Underline"
                                </ToggleButton>
                            </ToggleButtonGroup>
                            <span class="ui-muted">
                                "attached selected count: "
                                {move || attached_selected_count.get().to_string()}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Vertical + detached" code_signal=states_code>
                <div class="docs-stack">
                    <ToggleButtonGroup
                        orientation=ToggleButtonGroupOrientation::Vertical
                        attached=false
                        aria_label="Alignment controls".to_string()
                    >
                        <ToggleButton
                            selected=left
                            set_selected=set_left
                            variant=ToggleButtonVariant::Secondary
                        >
                            "Left"
                        </ToggleButton>
                        <ToggleButton
                            selected=center
                            set_selected=set_center
                            variant=ToggleButtonVariant::Secondary
                        >
                            "Center"
                        </ToggleButton>
                        <ToggleButton
                            selected=right
                            set_selected=set_right
                            variant=ToggleButtonVariant::Secondary
                        >
                            "Right"
                        </ToggleButton>
                    </ToggleButtonGroup>
                    <span class="ui-muted">
                        "detached selected count: "
                        {move || detached_selected_count.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn theme_toggle_button() -> AnyView {
    let (mode, set_mode) = signal(ThemeMode::Light);

    let mode_options = vec!["Light".to_string(), "Dark".to_string(), "OLED".to_string()];
    let (mode_index, set_mode_index) = signal(Some(0_usize));
    Effect::new(move |_| {
        let mode = match mode_index.get().unwrap_or(0) {
            1 => ThemeMode::Dark,
            2 => ThemeMode::Oled,
            _ => ThemeMode::Light,
        };
        set_mode.set(mode);
    });

    let (disabled, set_disabled) = signal(false);
    let (two_mode_cycle, set_two_mode_cycle) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);

    let code = Signal::derive(move || {
        let mode = match mode_index.get().unwrap_or(0) {
            1 => ThemeMode::Dark,
            2 => ThemeMode::Oled,
            _ => ThemeMode::Light,
        };
        let disabled = disabled.get();
        let two_mode_cycle = two_mode_cycle.get();
        let custom_aria_label = custom_aria_label.get();

        let mut snippet = vec![
            format!("let (mode, set_mode) = signal(ThemeMode::{mode:?});"),
            String::new(),
            "<ThemeToggleButton".to_string(),
            "  mode=mode".to_string(),
            "  set_mode=set_mode".to_string(),
        ];

        if disabled {
            snippet.push("  disabled=true".to_string());
        }
        if two_mode_cycle {
            snippet.push("  modes=vec![ThemeMode::Dark, ThemeMode::Light]".to_string());
        }
        if custom_aria_label {
            snippet.push("  aria_label=\"Switch UI mode\".to_string()".to_string());
        }

        snippet.push("/>".to_string());

        snippet.join("\n")
    });

    let (custom_mode, set_custom_mode) = signal(ThemeMode::Dark);
    let custom_modes = vec![ThemeMode::Dark, ThemeMode::Light];

    let states_code = Signal::derive(move || {
        r#"let (custom_mode, set_custom_mode) = signal(ThemeMode::Dark);
let (mode, set_mode) = signal(ThemeMode::System);

<ThemeToggleButton
  mode=custom_mode
  set_mode=set_custom_mode
  modes=vec![ThemeMode::Dark, ThemeMode::Light]
  aria_label="Switch UI mode".to_string()
/>
<ThemeToggleButton mode=mode set_mode=set_mode disabled=true />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ThemeToggleButton"
            slug="theme-toggle-button"
            group="Actions"
            description="Icon-only theme toggle with HeroUI-level spring motion and Spectrum-style mode state attrs."
        >
            <Playground
                title="Default cycle"
                code_signal=code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Start mode"</div>
                        <SegmentedControl
                            id_base="docs-theme-toggle-mode".to_string()
                            options=mode_options.clone()
                            selected_index=mode_index
                            set_selected_index=set_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="ThemeToggle start mode".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=two_mode_cycle set_checked=set_two_mode_cycle>
                            "Two-mode cycle (dark/light)"
                        </Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let disabled = disabled.get();
                    let two_mode_cycle = two_mode_cycle.get();
                    let custom_aria_label = custom_aria_label.get();
                    let modes = if two_mode_cycle {
                        vec![ThemeMode::Dark, ThemeMode::Light]
                    } else {
                        vec![ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled]
                    };

                    view! {
                        <div class="docs-row">
                            {if custom_aria_label {
                                view! {
                                    <ThemeToggleButton
                                        mode=mode
                                        set_mode=set_mode
                                        disabled=disabled
                                        modes=modes.clone()
                                        aria_label="Switch UI mode".to_string()
                                    />
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ThemeToggleButton
                                        mode=mode
                                        set_mode=set_mode
                                        disabled=disabled
                                        modes=modes
                                    />
                                }
                                    .into_any()
                            }}
                            <span class="ui-muted">"mode: " {move || format!("{:?}", mode.get())}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Custom modes + disabled" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ThemeToggleButton
                            mode=custom_mode
                            set_mode=set_custom_mode
                            modes=custom_modes.clone()
                            aria_label="Switch UI mode".to_string()
                        />
                        <span class="ui-muted">
                            "custom mode: " {move || format!("{:?}", custom_mode.get())}
                        </span>
                    </div>
                    <div class="docs-row">
                        <ThemeToggleButton mode=mode set_mode=set_mode disabled=true />
                        <span class="ui-muted">"disabled toggle should remain inert"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn search_input_button() -> AnyView {
    let (press_count, set_press_count) = signal(0_usize);
    let on_press: OnPress = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let preset_options = vec![
        "Default".to_string(),
        "Docs".to_string(),
        "Command".to_string(),
        "Components".to_string(),
    ];
    let (preset_index, set_preset_index) = signal(Some(0_usize));
    let placeholder = Signal::derive(move || match preset_index.get().unwrap_or(0) {
        1 => "Search docs".to_string(),
        2 => "Command menu".to_string(),
        3 => "Find components".to_string(),
        _ => "Search".to_string(),
    });
    let compact_placeholder = Signal::derive(move || match preset_index.get().unwrap_or(0) {
        1 => "Search".to_string(),
        2 => "Cmd".to_string(),
        3 => "Find".to_string(),
        _ => "Search".to_string(),
    });

    let meta_key_options = vec![
        "None".to_string(),
        "⌘".to_string(),
        "Ctrl".to_string(),
        "Alt".to_string(),
    ];
    let (meta_key_index, set_meta_key_index) = signal(Some(0_usize));
    let meta_key_label = Signal::derive(move || match meta_key_index.get().unwrap_or(0) {
        1 => "⌘".to_string(),
        2 => "Ctrl".to_string(),
        3 => "Alt".to_string(),
        _ => String::new(),
    });

    let key_label_options = vec!["None".to_string(), "K".to_string(), "F".to_string()];
    let (key_label_index, set_key_label_index) = signal(Some(0_usize));
    let key_label = Signal::derive(move || match key_label_index.get().unwrap_or(0) {
        1 => "K".to_string(),
        2 => "F".to_string(),
        _ => String::new(),
    });

    let (disabled, set_disabled) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);

    let code = Signal::derive(move || {
        let placeholder = placeholder.get();
        let compact_placeholder = compact_placeholder.get();
        let meta_key_label = meta_key_label.get();
        let key_label = key_label.get();
        let disabled = disabled.get();
        let custom_aria_label = custom_aria_label.get();

        let mut snippet = vec!["<SearchInputButton".to_string()];

        if placeholder != "Search" {
            snippet.push(format!("  placeholder=\"{placeholder}\".to_string()"));
        }
        if compact_placeholder != placeholder {
            snippet.push(format!(
                "  compact_placeholder=\"{compact_placeholder}\".to_string()"
            ));
        }
        if !meta_key_label.is_empty() {
            snippet.push(format!("  meta_key_label=\"{meta_key_label}\".to_string()"));
        }
        if !key_label.is_empty() {
            snippet.push(format!("  key_label=\"{key_label}\".to_string()"));
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }
        if custom_aria_label {
            snippet.push("  aria_label=\"Open command menu\".to_string()".to_string());
        }

        snippet.push("/>".to_string());

        snippet.join("\n")
    });

    let states_code = Signal::derive(move || {
        r#"<SearchInputButton placeholder="Find components".to_string() />
<SearchInputButton
  placeholder="Find components".to_string()
  compact_placeholder="Find".to_string()
/>
<SearchInputButton placeholder="Disabled search".to_string() disabled=true />
<SearchInputButton placeholder="Forced disabled".to_string() is_disabled=true />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<SearchInputButton
  placeholder="Browse components".to_string()
  compact_placeholder="Browse".to_string()
  aria_label="Open component search".to_string()
  class_name="docs-search-input-button-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="SearchInputButton"
            slug="search-input-button"
            group="Actions"
            description="HeroUI-level spring search trigger button with centralized placeholder/shortcut/aria-label state attrs."
        >
            <Playground
                title="Interactive + shortcut"
                code_signal=code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Preset"</div>
                        <SegmentedControl
                            id_base="docs-search-input-preset".to_string()
                            options=preset_options.clone()
                            selected_index=preset_index
                            set_selected_index=set_preset_index
                            size=SegmentedControlSize::Sm
                            aria_label="Search input preset".to_string()
                        />

                        <div class="docs-search__label">"Meta key"</div>
                        <SegmentedControl
                            id_base="docs-search-input-meta-key".to_string()
                            options=meta_key_options.clone()
                            selected_index=meta_key_index
                            set_selected_index=set_meta_key_index
                            size=SegmentedControlSize::Sm
                            aria_label="Search input meta key".to_string()
                        />

                        <div class="docs-search__label">"Shortcut key"</div>
                        <SegmentedControl
                            id_base="docs-search-input-key".to_string()
                            options=key_label_options.clone()
                            selected_index=key_label_index
                            set_selected_index=set_key_label_index
                            size=SegmentedControlSize::Sm
                            aria_label="Search input shortcut key".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let placeholder = placeholder.get();
                    let compact_placeholder = compact_placeholder.get();
                    let meta_key_label = meta_key_label.get();
                    let key_label = key_label.get();
                    let disabled = disabled.get();
                    let custom_aria_label = custom_aria_label.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                {if custom_aria_label {
                                    view! {
                                        <SearchInputButton
                                            placeholder=placeholder.clone()
                                            compact_placeholder=compact_placeholder.clone()
                                            meta_key_label=meta_key_label.clone()
                                            key_label=key_label.clone()
                                            aria_label="Open command menu".to_string()
                                            disabled=disabled
                                            on_press=on_press
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <SearchInputButton
                                            placeholder=placeholder
                                            compact_placeholder=compact_placeholder
                                            meta_key_label=meta_key_label
                                            key_label=key_label
                                            disabled=disabled
                                            on_press=on_press
                                        />
                                    }
                                        .into_any()
                                }}
                            </div>
                            <span class="ui-muted">"presses: " {move || press_count.get().to_string()}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Placeholder + disabled matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <SearchInputButton placeholder="Find components".to_string() />
                        <SearchInputButton
                            placeholder="Find components".to_string()
                            compact_placeholder="Find".to_string()
                        />
                    </div>
                    <div class="docs-row">
                        <SearchInputButton placeholder="Disabled search".to_string() disabled=true />
                        <SearchInputButton
                            placeholder="Forced disabled".to_string()
                            is_disabled=true
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class + Aria Label" code_signal=custom_code>
                <div class="docs-row">
                    <SearchInputButton
                        placeholder="Browse components".to_string()
                        compact_placeholder="Browse".to_string()
                        aria_label="Open component search".to_string()
                        class_name="docs-search-input-button-custom".to_string()
                    />
                    <SearchInputButton
                        placeholder="Search by keyword".to_string()
                        class_name="docs-search-input-button-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn button_copy() -> AnyView {
    let code = Signal::derive(move || {
        r#"<ButtonCopy
  text="cargo add ui-components".to_string()
  label="Copy install command".to_string()
  copied_label="Copied!".to_string()
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ButtonCopy text="https://example.com/docs".to_string() variant=ButtonVariant::Outline />
<ButtonCopy text="   ".to_string() label="Nothing to copy".to_string() />
<ButtonCopy text="token".to_string() disabled=true />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ButtonCopy"
            slug="button-copy"
            group="Actions"
            description="Copy-to-clipboard button with Spectrum-style disabled/empty semantics and live copied announcements."
        >
            <Playground title="Label + variant" code_signal=code>
                <div class="docs-row">
                    <ButtonCopy
                        text="cargo add ui-components".to_string()
                        label="Copy install command".to_string()
                        copied_label="Copied!".to_string()
                    />
                    <ButtonCopy
                        text="https://github.com/openai".to_string()
                        variant=ButtonVariant::Outline
                        label="Copy URL".to_string()
                        copied_label="URL copied".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Disabled + empty matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ButtonCopy
                            text="https://example.com/docs".to_string()
                            variant=ButtonVariant::Outline
                        />
                        <ButtonCopy text="   ".to_string() label="Nothing to copy".to_string() />
                        <ButtonCopy text="token".to_string() disabled=true />
                    </div>
                    <span class="ui-muted">
                        "Blank text and explicit disabled state both force non-copyable semantics."
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn flip_button() -> AnyView {
    let code = Signal::derive(move || {
        r#"<FlipButton
  from=FlipDirection::Top
  front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
  back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<FlipButton from=FlipDirection::Top front=... back=... />
<FlipButton from=FlipDirection::Bottom front=... back=... />
<FlipButton from=FlipDirection::Left front=... back=... />
<FlipButton from=FlipDirection::Right front=... back=... />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<FlipButton
  from=FlipDirection::Left
  class_name="docs-flip-button-custom".to_string()
  front=move || view! { <Button variant=ButtonVariant::Outline>"Inspect"</Button> }
  back=move || view! { <Button variant=ButtonVariant::Accent>"Inspecting"</Button> }
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="FlipButton"
            slug="flip-button"
            group="Actions"
            description="HeroUI-level spring flip surface with centralized direction/interaction/class-source state attrs."
        >
            <Playground title="Top flip" code_signal=code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Top
                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                    />
                </div>
            </Playground>

            <Playground title="Direction matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <FlipButton
                            from=FlipDirection::Bottom
                            front=move || view! { <Button variant=ButtonVariant::Secondary>"Bottom"</Button> }
                            back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                        />
                        <FlipButton
                            from=FlipDirection::Left
                            front=move || view! { <Button variant=ButtonVariant::Secondary>"Left"</Button> }
                            back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                        />
                        <FlipButton
                            from=FlipDirection::Right
                            front=move || view! { <Button variant=ButtonVariant::Secondary>"Right"</Button> }
                            back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class" code_signal=custom_code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Left
                        class_name="docs-flip-button-custom".to_string()
                        front=move || view! { <Button variant=ButtonVariant::Outline>"Inspect"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Inspecting"</Button> }
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn share_button() -> AnyView {
    let (last, set_last) = signal(None::<SharePlatform>);
    let on_icon_press = Callback::new(move |platform: SharePlatform| set_last.set(Some(platform)));

    let custom_items = vec![
        ShareButtonItem::new(SharePlatform::Github, "Repository"),
        ShareButtonItem::new(SharePlatform::X, "Post"),
        ShareButtonItem::new(SharePlatform::Facebook, "   "),
    ];

    let custom_items_for_matrix = custom_items.clone();
    let custom_items_for_custom = custom_items.clone();

    let code = Signal::derive(move || {
        r#"let on_icon_press = Callback::new(|platform: SharePlatform| {
  logging::log!("pressed: {platform:?}");
});
<ShareButton on_icon_press=Some(on_icon_press) />"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ShareButton
  icon=ShareButtonIconPlacement::Prefix
  from=FlipDirection::Left
  label="Share now".to_string()
  items=custom_items_for_matrix.clone()
/>
<ShareButton icon=ShareButtonIconPlacement::None label="Iconless".to_string() />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<ShareButton
  class_name="docs-share-button-custom".to_string()
  icon=ShareButtonIconPlacement::Prefix
  from=FlipDirection::Right
  label="Share docs".to_string()
  items=custom_items.clone()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ShareButton"
            slug="share-button"
            group="Actions"
            description="Flip-based share surface with centralized item/icon/handler state attrs and HeroUI-grade spring motion."
        >
            <Playground title="Default + callback" code_signal=code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ShareButton on_icon_press=on_icon_press />
                        <span class="ui-muted">
                            "last: "
                            {move || {
                                last.get()
                                    .map(|v| format!("{v:?}"))
                                    .unwrap_or_else(|| "None".to_string())
                            }}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground title="Icon placement + custom items" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ShareButton
                            icon=ShareButtonIconPlacement::Prefix
                            from=FlipDirection::Left
                            label="Share now".to_string()
                            items=custom_items_for_matrix.clone()
                            on_icon_press=on_icon_press
                        />
                        <ShareButton
                            icon=ShareButtonIconPlacement::None
                            label="Iconless".to_string()
                            items=custom_items_for_matrix.clone()
                        />
                    </div>
                    <span class="ui-muted">
                        "Blank custom item labels fall back to platform defaults; missing handlers stay safe."
                    </span>
                </div>
            </Playground>

            <Playground title="Custom Class + Direction" code_signal=custom_code>
                <div class="docs-row">
                    <ShareButton
                        class_name="docs-share-button-custom".to_string()
                        icon=ShareButtonIconPlacement::Prefix
                        from=FlipDirection::Right
                        label="Share docs".to_string()
                        items=custom_items_for_custom.clone()
                    />
                    <ShareButton
                        class_name="docs-share-button-custom".to_string()
                        label="Share defaults".to_string()
                        icon=ShareButtonIconPlacement::Suffix
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_menu() -> AnyView {
    let default_items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Log out".to_string(),
    ];
    let controlled_items = vec![
        "Rename".to_string(),
        "Duplicate".to_string(),
        "Archive".to_string(),
    ];
    let disabled_items = vec!["Copy".to_string(), "Move".to_string()];
    let empty_items: Vec<String> = Vec::new();
    let marker_items = vec![
        "Open dashboard".to_string(),
        "Duplicate project".to_string(),
        "Archive workspace".to_string(),
    ];

    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let (marker_open_raw, set_marker_open_raw) = signal(true);
    let marker_open: Signal<bool> = Signal::derive(move || marker_open_raw.get());
    let on_marker_open_change = Callback::new(move |next: bool| set_marker_open_raw.set(next));

    let (last_marker_action, set_last_marker_action) = signal(None::<usize>);
    let on_marker_action =
        Callback::new(move |index: usize| set_last_marker_action.set(Some(index)));

    let code = Signal::derive(move || {
        r#"<ActionMenu
  id_base="demo".to_string()
  items=vec![
    "Open dashboard".to_string(),
    "Duplicate project".to_string(),
    "Archive workspace".to_string(),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
/>"#
        .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<ActionMenu
  id_base="action-controlled".to_string()
  items=vec![
    "Open dashboard".to_string(),
    "Duplicate project".to_string(),
    "Archive workspace".to_string(),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
  close_on_action=false
  disabled_indices=vec![1]
  open=open
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<ActionMenu
  id_base="docs-action-menu-markers".to_string()
  items=vec![
    "Open dashboard".to_string(),
    "Duplicate project".to_string(),
    "Archive workspace".to_string(),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
  disabled_indices=vec![2]
  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]
  close_on_action=false
  open=open
  default_open=true
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  aria_label="Workspace actions".to_string()
  class_name="docs-action-menu-custom".to_string()
  motion=ui_components::ActionMenuMotion {
    popover: ui_components::PopoverMotion {
      initial_scale: 0.93,
      offset_y_px: 8.0,
      ..ui_components::PopoverMotion::default()
    },
  }
/>"#
        .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<ActionMenu
  id_base="action-disabled".to_string()
  items=vec![
    "Open dashboard".to_string(),
    "Duplicate project".to_string(),
    "Archive workspace".to_string(),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
  disabled=true
/>
<ActionMenu
  id_base="action-empty".to_string()
  items=Vec::<String>::new()
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
/>"#
        .to_string()
    });

    let marker_motion = ui_components::ActionMenuMotion {
        popover: ui_components::PopoverMotion {
            initial_scale: 0.93,
            offset_y_px: 8.0,
            ..ui_components::PopoverMotion::default()
        },
    };

    view! {
        <ComponentPage
            title="ActionMenu"
            slug="action-menu"
            group="Actions"
            description="ActionButton-triggered menu surface with Spectrum state/source data attrs and HeroUI-grade popover spring motion (controlled/uncontrolled + close strategy)."
        >
            <Playground title="Default" code_signal=code>
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu".to_string()
                        items=default_items
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + persistent open" code_signal=controlled_code>
                <div class="docs-stack">
                    <ActionMenu
                        id_base="docs-action-menu-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        close_on_action=false
                        disabled_indices=vec![1]
                        open=controlled_open
                        on_open_change=on_open_change
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_marker_open_raw.set(true)>
                            "Open"
                        </button>
                        <button type="button" on:click=move |_| set_marker_open_raw.set(false)>
                            "Close"
                        </button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-aria-label-source / data-disabled-indices-source / data-item-kinds-source / data-open-source / data-open-change-source / data-motion-source in DevTools."
                    </div>
                    <ActionMenu
                        id_base="docs-action-menu-markers".to_string()
                        items=marker_items
                        on_action=on_marker_action
                        disabled_indices=vec![2]
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                        close_on_action=false
                        open=marker_open
                        default_open=true
                        on_open_change=on_marker_open_change
                        aria_label="Workspace actions".to_string()
                        class_name="docs-action-menu-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || marker_open_raw.get().to_string()}
                        " · last action: "
                        {move || {
                            last_marker_action
                                .get()
                                .map(|value| value.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_code>
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu-disabled".to_string()
                        items=disabled_items
                        on_action=on_action
                        disabled=true
                        item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                    />

                    <ActionMenu
                        id_base="docs-action-menu-empty".to_string()
                        items=empty_items
                        on_action=on_action
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
