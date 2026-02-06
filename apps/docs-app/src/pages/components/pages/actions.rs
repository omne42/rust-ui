use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    ActionButton, ActionButtonGroup, ActionButtonGroupDensity, ActionButtonGroupOrientation,
    ActionButtonSize, ActionMenu, Button, ButtonCopy, ButtonGroup, ButtonGroupOrientation,
    ButtonSize, ButtonVariant, FlipButton, FlipDirection, IconButton, LinkButton, MenuItemKind,
    OnPress, SearchInputButton, SegmentedControl, SegmentedControlSize, ShareButton, SharePlatform,
    Switch, ThemeMode, ThemeToggleButton, ToggleButton, ToggleButtonGroup,
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
        "Small".to_string(),
        "Medium".to_string(),
        "Large".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(1_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(1) {
        0 => ButtonSize::Sm,
        2 => ButtonSize::Lg,
        _ => ButtonSize::Default,
    });

    let (disabled, set_disabled) = signal(false);
    let (loading, set_loading) = signal(false);

    let code = r#"<Button variant=ButtonVariant::Default>"Primary"</Button>
<Button variant=ButtonVariant::Outline>"Outline"</Button>
<Button variant=ButtonVariant::Ghost>"Ghost"</Button>"#;

    view! {
        <ComponentPage
            title="Button"
            slug="button"
            group="Actions"
            description="Variants + sizes with spring hover/tap motion."
        >
            <Playground
                title="Variants & sizes"
                code=code
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
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let size = size.get();
                    let disabled = disabled.get();
                    let is_loading = loading.get();

                    view! {
                        <div class="docs-row">
                            <Button
                                variant=variant
                                size=size
                                disabled=disabled
                                is_loading=is_loading
                            >
                                "Button"
                            </Button>
                            <span class="ui-muted">
                                {format!("{variant:?} · {size:?}")}
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
    let code = r#"<ActionButton>"Action"</ActionButton>
<ActionButton is_quiet=true>"Quiet"</ActionButton>
<ActionButton is_loading=true loading_placement=ActionButtonLoadingPlacement::Center>
  "Loading"
</ActionButton>"#;

    view! {
        <ComponentPage
            title="ActionButton"
            slug="action-button"
            group="Actions"
            description="Spectrum-style action button (quiet/filled) with loading state."
        >
            <Playground title="Basics" code=code>
                <div class="docs-row">
                    <ActionButton>"Action"</ActionButton>
                    <ActionButton is_quiet=true>"Quiet"</ActionButton>
                    <ActionButton is_loading=true>"Loading"</ActionButton>
                    <ActionButton is_icon_only=true aria_label="Settings".to_string()>
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
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_button_group() -> AnyView {
    let code = r#"<ActionButtonGroup size=ActionButtonSize::Sm is_quiet=true>
  <ActionButton>"One"</ActionButton>
  <ActionButton>"Two"</ActionButton>
  <ActionButton>"Three"</ActionButton>
</ActionButtonGroup>"#;

    view! {
        <ComponentPage
            title="ActionButtonGroup"
            slug="action-button-group"
            group="Actions"
            description="Groups ActionButtons with toolbar semantics."
        >
            <Playground title="Group" code=code>
                <div class="docs-stack">
                    <ActionButtonGroup
                        size=ActionButtonSize::S
                        density=ActionButtonGroupDensity::Compact
                        orientation=ActionButtonGroupOrientation::Horizontal
                        is_quiet=true
                    >
                        <ActionButton>"One"</ActionButton>
                        <ActionButton>"Two"</ActionButton>
                        <ActionButton>"Three"</ActionButton>
                    </ActionButtonGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn button_group() -> AnyView {
    let code = r#"<ButtonGroup attached=true>
  <Button variant=ButtonVariant::Secondary>"Left"</Button>
  <Button variant=ButtonVariant::Secondary>"Middle"</Button>
  <Button variant=ButtonVariant::Secondary>"Right"</Button>
</ButtonGroup>"#;

    view! {
        <ComponentPage
            title="ButtonGroup"
            slug="button-group"
            group="Actions"
            description="Groups Buttons with attached styling."
        >
            <Playground title="Attached group" code=code>
                <div class="docs-stack">
                    <ButtonGroup attached=true orientation=ButtonGroupOrientation::Horizontal>
                        <Button variant=ButtonVariant::Secondary>"Left"</Button>
                        <Button variant=ButtonVariant::Secondary>"Middle"</Button>
                        <Button variant=ButtonVariant::Secondary>"Right"</Button>
                    </ButtonGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn icon_button() -> AnyView {
    let code = r#"<IconButton aria_label="Close".to_string() variant=ButtonVariant::Ghost>
  <svg ... />
</IconButton>"#;

    view! {
        <ComponentPage
            title="IconButton"
            slug="icon-button"
            group="Actions"
            description="A Button wrapper that enforces aria-label and icon sizing."
        >
            <Playground title="Icon-only" code=code>
                <div class="docs-row">
                    <IconButton aria_label="Close".to_string() variant=ButtonVariant::Ghost>
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
                    <IconButton aria_label="Search".to_string() variant=ButtonVariant::Secondary>
                        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                            <circle cx="9" cy="9" r="6" stroke="currentColor" stroke_width="1.5" />
                            <path
                                d="M13.5 13.5l3 3"
                                stroke="currentColor"
                                stroke_width="1.5"
                                stroke_linecap="round"
                            />
                        </svg>
                    </IconButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn link_button() -> AnyView {
    let code = r#"<LinkButton href="https://example.com".to_string()>
  "External link"
</LinkButton>"#;

    view! {
        <ComponentPage
            title="LinkButton"
            slug="link-button"
            group="Actions"
            description="Button styling on an anchor element."
        >
            <Playground title="Anchor button" code=code>
                <div class="docs-row">
                    <LinkButton href="https://example.com".to_string() target="_blank">
                        "External link"
                    </LinkButton>
                    <LinkButton href="#".to_string() disabled=true variant=ButtonVariant::Secondary>
                        "Disabled"
                    </LinkButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toggle_button() -> AnyView {
    let (selected, set_selected) = signal(false);
    let code = r#"let (selected, set_selected) = signal(false);
<ToggleButton selected=selected set_selected=set_selected>
  "Toggle"
</ToggleButton>"#;

    view! {
        <ComponentPage
            title="ToggleButton"
            slug="toggle-button"
            group="Actions"
            description="Pressable toggle state with aria-pressed."
        >
            <Playground title="Toggle" code=code>
                <div class="docs-row">
                    <ToggleButton
                        selected=selected
                        set_selected=set_selected
                        variant=ui_components::ToggleButtonVariant::Default
                    >
                        "Toggle"
                    </ToggleButton>
                    <span class="ui-muted">
                        "selected: " {move || selected.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toggle_button_group() -> AnyView {
    let (a, set_a) = signal(false);
    let (b, set_b) = signal(true);
    let code = r#"<ToggleButtonGroup attached=true>
  <ToggleButton selected=a set_selected=set_a>"A"</ToggleButton>
  <ToggleButton selected=b set_selected=set_b>"B"</ToggleButton>
</ToggleButtonGroup>"#;

    view! {
        <ComponentPage
            title="ToggleButtonGroup"
            slug="toggle-button-group"
            group="Actions"
            description="Layout wrapper for grouping ToggleButtons."
        >
            <Playground title="Attached" code=code>
                <ToggleButtonGroup attached=true>
                    <ToggleButton selected=a set_selected=set_a>"A"</ToggleButton>
                    <ToggleButton selected=b set_selected=set_b>"B"</ToggleButton>
                </ToggleButtonGroup>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn theme_toggle_button() -> AnyView {
    let (mode, set_mode) = signal(ThemeMode::Light);
    let code = r#"let (mode, set_mode) = signal(ThemeMode::Light);
<ThemeToggleButton mode=mode set_mode=set_mode />"#;

    view! {
        <ComponentPage
            title="ThemeToggleButton"
            slug="theme-toggle-button"
            group="Actions"
            description="Icon-only theme toggle button (Light/Dark/OLED)."
        >
            <Playground title="Theme toggle" code=code>
                <div class="docs-row">
                    <ThemeToggleButton mode=mode set_mode=set_mode />
                    <span class="ui-muted">"mode: " {move || format!("{:?}", mode.get())}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn search_input_button() -> AnyView {
    let on_press: OnPress = Callback::new(|_| {});
    let code = r#"<SearchInputButton on_press=Some(on_press) />"#;

    view! {
        <ComponentPage
            title="SearchInputButton"
            slug="search-input-button"
            group="Actions"
            description="HeroUI-style search trigger button with shortcut hint."
        >
            <Playground title="Default" code=code>
                <div class="docs-stack">
                    <SearchInputButton on_press=on_press />
                    <SearchInputButton disabled=true />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn button_copy() -> AnyView {
    let code = r#"<ButtonCopy text="hello".to_string() />"#;

    view! {
        <ComponentPage
            title="ButtonCopy"
            slug="button-copy"
            group="Actions"
            description="Copy-to-clipboard button using the Snippet clipboard logic."
        >
            <Playground title="Copy" code=code>
                <div class="docs-row">
                    <ButtonCopy text="cargo add ui-components".to_string() />
                    <ButtonCopy text="https://github.com/openai".to_string() variant=ButtonVariant::Outline />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn flip_button() -> AnyView {
    let code = r#"<FlipButton front=... back=... />"#;

    view! {
        <ComponentPage
            title="FlipButton"
            slug="flip-button"
            group="Actions"
            description="Hover/focus flips between two faces (animate-ui-style)."
        >
            <Playground title="Flip on hover" code=code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Top
                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
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
    let code = r#"let on_icon_press = Callback::new(|platform: SharePlatform| { /* ... */ });
<ShareButton on_icon_press=Some(on_icon_press) />"#;

    view! {
        <ComponentPage
            title="ShareButton"
            slug="share-button"
            group="Actions"
            description="Flip-based share menu with platform icons."
        >
            <Playground title="Share" code=code>
                <div class="docs-row">
                    <ShareButton on_icon_press=on_icon_press />
                    <span class="ui-muted">
                        "last: " {move || last.get().map(|v| format!("{v:?}")).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_menu() -> AnyView {
    let items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Log out".to_string(),
    ];
    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));
    let code = r#"let items = vec!["Profile".to_string(), "Settings".to_string()];
<ActionMenu id_base="demo".to_string() items=items on_action=on_action />"#;

    view! {
        <ComponentPage
            title="ActionMenu"
            slug="action-menu"
            group="Actions"
            description="An ActionButton trigger that opens a Menu in a Popover."
        >
            <Playground title="Menu trigger" code=code>
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu".to_string()
                        items=items
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    />
                    <span class="ui-muted">
                        "last action: " {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
