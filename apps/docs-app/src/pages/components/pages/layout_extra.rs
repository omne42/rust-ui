#[path = "layout_extra_sidebar_content.rs"]
mod layout_extra_sidebar_content;
#[path = "layout_extra_sidebar_footer.rs"]
mod layout_extra_sidebar_footer;
#[path = "layout_extra_sidebar_trigger.rs"]
mod layout_extra_sidebar_trigger;
#[path = "layout_extra_surface.rs"]
mod layout_extra_surface;

use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AspectRatio, AspectRatioPreset, AspectRatioRadius, Grid, GridAlign, GridColumns, GridGap,
    GridJustify, GridRows, Resizable, ResizableOrientation, ScrollArea, ScrollAreaOrientation,
    Sidebar, SidebarCollapsible, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
    SidebarMenuSubItem, SidebarSide, SidebarVariant, View, ViewBackground, ViewBorder, ViewPadding,
    ViewRadius,
};

pub(super) fn aspect_ratio() -> AnyView {
    let preset_code = r#"<AspectRatio ratio=AspectRatioPreset::Square radius=AspectRatioRadius::Sm fill=true>
  <View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::None>"1:1"</View>
</AspectRatio>
<AspectRatio ratio=AspectRatioPreset::Video radius=AspectRatioRadius::Md fill=true>
  <View background=ViewBackground::Accent border=ViewBorder::None padding=ViewPadding::Sm radius=ViewRadius::None>"16:9"</View>
</AspectRatio>
<AspectRatio ratio=AspectRatioPreset::Portrait radius=AspectRatioRadius::Md fill=true>
  <View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::None>"3:4"</View>
</AspectRatio>"#;

    let framed_code = r#"<AspectRatio
  ratio=AspectRatioPreset::UltraWide
  radius=AspectRatioRadius::Lg
  bordered=true
  fill=true
  aria_label="Release trailer preview".to_string()
  class_name="docs-aspect-ratio-custom".to_string()
>
  <View
    background=ViewBackground::Accent
    border=ViewBorder::None
    padding=ViewPadding::Md
    radius=ViewRadius::None
  >
    "21:9 framed media"
  </View>
</AspectRatio>"#;

    view! {
        <ComponentPage
            title="AspectRatio"
            slug="aspect-ratio"
            group="Layout"
            description="Shadcn/HeroUI-compatible media frame primitive with centralized ratio/radius/frame/source normalization and stable state-marker contracts."
        >
            <Playground title="Ratio Presets" code=preset_code>
                <div class="docs-stack">
                    <AspectRatio ratio=AspectRatioPreset::Square radius=AspectRatioRadius::Sm fill=true>
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::None
                        >
                            "1:1"
                        </View>
                    </AspectRatio>

                    <AspectRatio ratio=AspectRatioPreset::Video radius=AspectRatioRadius::Md fill=true>
                        <View
                            background=ViewBackground::Accent
                            border=ViewBorder::None
                            padding=ViewPadding::Sm
                            radius=ViewRadius::None
                        >
                            "16:9"
                        </View>
                    </AspectRatio>

                    <AspectRatio
                        ratio=AspectRatioPreset::Portrait
                        radius=AspectRatioRadius::Md
                        fill=true
                    >
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::None
                        >
                            "3:4"
                        </View>
                    </AspectRatio>
                </div>
            </Playground>

            <Playground title="Bordered + Fill + Custom Aria/Class" code=framed_code>
                <AspectRatio
                    ratio=AspectRatioPreset::UltraWide
                    radius=AspectRatioRadius::Lg
                    bordered=true
                    fill=true
                    aria_label="Release trailer preview".to_string()
                    class_name="docs-aspect-ratio-custom".to_string()
                >
                    <View
                        background=ViewBackground::Accent
                        border=ViewBorder::None
                        padding=ViewPadding::Md
                        radius=ViewRadius::None
                    >
                        "21:9 framed media"
                    </View>
                </AspectRatio>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn grid() -> AnyView {
    let columns_code = r#"<Grid columns=GridColumns::Three gap=GridGap::Md>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"A"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"B"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"C"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"D"</View>
</Grid>"#;

    let adaptive_code = r#"<Grid
  columns=GridColumns::AutoFit
  rows=GridRows::Equal
  gap=GridGap::Lg
  justify=GridJustify::Stretch
  align=GridAlign::Stretch
  dense=true
  class_name="docs-grid-adaptive".to_string()
>
  <View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Sm>"Revenue"</View>
  <View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Sm>"Users"</View>
  <View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Sm>"Latency"</View>
</Grid>"#;

    view! {
        <ComponentPage
            title="Grid"
            slug="grid"
            group="Layout"
            description="Spectrum-style grid layout primitive with centralized columns/rows/gap/alignment normalization and stable state-marker contracts."
        >
            <Playground title="Columns + Gap" code=columns_code>
                <Grid columns=GridColumns::Three gap=GridGap::Md aria_label="Overview cards grid".to_string()>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "A"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "B"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "C"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "D"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "E"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "F"
                    </View>
                </Grid>
            </Playground>

            <Playground title="AutoFit + Dense + Equal Rows" code=adaptive_code>
                <Grid
                    columns=GridColumns::AutoFit
                    rows=GridRows::Equal
                    gap=GridGap::Lg
                    justify=GridJustify::Stretch
                    align=GridAlign::Stretch
                    dense=true
                    class_name="docs-grid-adaptive".to_string()
                >
                    <View
                        background=ViewBackground::Subtle
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Md
                        radius=ViewRadius::Sm
                    >
                        "Revenue"
                    </View>
                    <View
                        background=ViewBackground::Subtle
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Md
                        radius=ViewRadius::Sm
                    >
                        "Users"
                    </View>
                    <View
                        background=ViewBackground::Subtle
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Md
                        radius=ViewRadius::Sm
                    >
                        "Latency"
                    </View>
                    <View
                        background=ViewBackground::Subtle
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Md
                        radius=ViewRadius::Sm
                    >
                        "Errors"
                    </View>
                </Grid>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn scroll_area() -> AnyView {
    let default_code = r#"<ScrollArea max_height_px=180>
  {rows}
</ScrollArea>"#;

    let state_code = r#"<ScrollArea
  orientation=ScrollAreaOrientation::Horizontal
  max_height_px=120
  class_name="docs-scroll-area-custom".to_string()
>
  {chips}
</ScrollArea>
<ScrollArea
  orientation=ScrollAreaOrientation::Both
  disabled=true
  max_height_px=120
  aria_label="Disabled logs".to_string()
>
  {grid}
</ScrollArea>"#;

    view! {
        <ComponentPage
            title="ScrollArea"
            slug="scroll-area"
            group="Layout"
            description="Shadcn-compatible scroll container with centralized orientation/max-height/disabled normalization and stable state-marker data contracts."
        >
            <Playground title="Vertical + Max Height" code=default_code>
                <ScrollArea max_height_px=180>
                    <div class="docs-stack docs-stack--tight">
                        {(1..=24)
                            .map(|idx| {
                                view! { <div class="docs-scroll-shadow-item">{format!("Release note {idx}")}</div> }
                            })
                            .collect_view()}
                    </div>
                </ScrollArea>
            </Playground>

            <Playground title="Horizontal + Both + Disabled" code=state_code>
                <div class="docs-stack docs-stack--tight">
                    <ScrollArea
                        orientation=ScrollAreaOrientation::Horizontal
                        max_height_px=120
                        class_name="docs-scroll-area-custom".to_string()
                    >
                        <div class="docs-row">
                            {(1..=16)
                                .map(|idx| {
                                    view! {
                                        <span class="ui-chip ui-chip--flat docs-scroll-area-chip">
                                            {format!("Tag {idx}")}
                                        </span>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </ScrollArea>

                    <ScrollArea
                        orientation=ScrollAreaOrientation::Both
                        disabled=true
                        max_height_px=120
                        aria_label="Disabled logs".to_string()
                    >
                        <div class="docs-scroll-area-grid">
                            {(1..=20)
                                .map(|idx| {
                                    view! {
                                        <div class="docs-scroll-shadow-item">
                                            {format!("Cell {idx}")}
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </ScrollArea>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn resizable() -> AnyView {
    let horizontal_code = r#"<Resizable
  orientation=ResizableOrientation::Horizontal
  default_split_percent=36.0
  with_handle=true
  first=move || view! { <div>"Sidebar"</div> }
  second=move || view! { <div>"Content"</div> }
/>"#;

    let vertical_code = r#"let (split_raw, set_split_raw) = signal(58.0_f64);
let split: Signal<f64> = Signal::derive(move || split_raw.get());

<Resizable
  orientation=ResizableOrientation::Vertical
  split_percent=split
  on_split_percent_change=Callback::new(move |next| set_split_raw.set(next))
  min_split_percent=25.0
  max_split_percent=80.0
  with_handle=true
  class_name="docs-resizable-custom".to_string()
  first=move || view! { <div>"Header"</div> }
  second=move || view! { <div>"Body"</div> }
/>"#;

    let (split_raw, set_split_raw) = signal(58.0_f64);
    let split: Signal<f64> = Signal::derive(move || split_raw.get());
    let on_split_change = Callback::new(move |next: f64| {
        set_split_raw.set(next);
    });

    view! {
        <ComponentPage
            title="Resizable"
            slug="resizable"
            group="Layout"
            description="Shadcn-compatible panel splitter with controlled/uncontrolled split state, pointer + keyboard resize semantics, and Spectrum-style state data contracts."
        >
            <Playground title="Horizontal + Handle Grip" code=horizontal_code>
                <Resizable
                    orientation=ResizableOrientation::Horizontal
                    default_split_percent=36.0
                    with_handle=true
                    first=move || {
                        view! {
                            <View
                                background=ViewBackground::Subtle
                                border=ViewBorder::Subtle
                                padding=ViewPadding::Md
                                radius=ViewRadius::None
                            >
                                <div class="docs-stack docs-stack--tight">
                                    <strong>"Sidebar"</strong>
                                    <span class="ui-muted">"Drag handle or Arrow keys to resize."</span>
                                </div>
                            </View>
                        }
                    }
                    second=move || {
                        view! {
                            <View
                                background=ViewBackground::Default
                                border=ViewBorder::None
                                padding=ViewPadding::Md
                                radius=ViewRadius::None
                            >
                                <div class="docs-stack docs-stack--tight">
                                    <strong>"Content"</strong>
                                    <span class="ui-muted">"Resizable panel body with scroll-safe overflow."</span>
                                </div>
                            </View>
                        }
                    }
                />
            </Playground>

            <Playground title="Controlled + Vertical Bounds" code=vertical_code>
                <div class="docs-stack docs-stack--tight">
                    <Resizable
                        orientation=ResizableOrientation::Vertical
                        split_percent=split
                        on_split_percent_change=on_split_change
                        min_split_percent=25.0
                        max_split_percent=80.0
                        with_handle=true
                        aria_label="Deployment regions split".to_string()
                        class_name="docs-resizable-custom".to_string()
                        first=move || {
                            view! {
                                <View
                                    background=ViewBackground::Subtle
                                    border=ViewBorder::Subtle
                                    padding=ViewPadding::Md
                                    radius=ViewRadius::None
                                >
                                    <strong>"Header"</strong>
                                </View>
                            }
                        }
                        second=move || {
                            view! {
                                <View
                                    background=ViewBackground::Default
                                    border=ViewBorder::None
                                    padding=ViewPadding::Md
                                    radius=ViewRadius::None
                                >
                                    <strong>"Body"</strong>
                                </View>
                            }
                        }
                    />
                    <span class="ui-muted">
                        "controlled split: "
                        {move || format!("{:.1}%", split_raw.get())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sidebar() -> AnyView {
    let basic_code = r#"<Sidebar
  side=SidebarSide::Left
  variant=SidebarVariant::Sidebar
  collapsible=SidebarCollapsible::Offcanvas
>
  <div class="ui-sidebar__header"><strong>"Workspace"</strong></div>
  <div class="ui-sidebar__content">
    <span>"Dashboard"</span>
    <span>"Analytics"</span>
    <span>"Settings"</span>
  </div>
  <div class="ui-sidebar__footer"><span>"Free plan"</span></div>
</Sidebar>"#;

    let controlled_code = r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());
let on_open_change = Callback::new(move |next| set_open_raw.set(next));

<button class="ui-button" on:click=move |_| set_open_raw.update(|open| *open = !*open)>
  "Toggle right sidebar"
</button>

<Sidebar
  open=open
  on_open_change=on_open_change
  side=SidebarSide::Right
  variant=SidebarVariant::Inset
  collapsible=SidebarCollapsible::Icon
  show_trigger=false
  class_name="docs-sidebar-custom".to_string()
>
  <div class="ui-sidebar__header"><strong>"Inspector"</strong></div>
  <div class="ui-sidebar__content"><span>"Layers"</span><span>"Tokens"</span></div>
  <div class="ui-sidebar__footer"><span>"Ctrl+B / Cmd+B"</span></div>
</Sidebar>"#;

    let (open_raw, set_open_raw) = signal(true);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    view! {
        <ComponentPage
            title="Sidebar"
            slug="sidebar"
            group="Layout"
            description="Shadcn-compatible sidebar primitive with controlled/uncontrolled open state, side+variant+collapsible contracts, keyboard shortcut toggle, and Spectrum-style data markers."
        >
            <Playground title="Offcanvas + Slot Markers" code=basic_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    aria_label="Project navigation sidebar".to_string()
                >
                    <div class="ui-sidebar__header">
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Workspace"</strong>
                            <span class="ui-muted">"Navigation + quick status"</span>
                        </div>
                    </div>

                    <div class="ui-sidebar__content">
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::Sm
                        >
                            "Dashboard"
                        </View>
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::Sm
                        >
                            "Analytics"
                        </View>
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::Sm
                        >
                            "Settings"
                        </View>
                    </div>

                    <div class="ui-sidebar__footer">
                        <span class="ui-muted">"Free plan · 2 seats"</span>
                    </div>
                </Sidebar>
            </Playground>

            <Playground title="Controlled + Right Inset/Icon" code=controlled_code>
                <div class="docs-stack docs-stack--tight">
                    <button
                        class="ui-button"
                        type="button"
                        on:click=move |_| set_open_raw.update(|open| *open = !*open)
                    >
                        "Toggle right sidebar"
                    </button>

                    <Sidebar
                        open=open
                        on_open_change=on_open_change
                        side=SidebarSide::Right
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        class_name="docs-sidebar-custom".to_string()
                        aria_label="Inspector sidebar".to_string()
                    >
                        <div class="ui-sidebar__header">
                            <strong>"Inspector"</strong>
                        </div>
                        <div class="ui-sidebar__content">
                            <span>"Layers"</span>
                            <span>"Tokens"</span>
                            <span>"Motion"</span>
                        </div>
                        <div class="ui-sidebar__footer">
                            <span class="ui-muted">"Ctrl+B / Cmd+B"</span>
                        </div>
                    </Sidebar>

                    <span class="ui-muted">
                        "controlled open: "
                        {move || if open_raw.get() { "true" } else { "false" }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sidebar_header() -> AnyView {
    let basic_code = r#"<SidebarHeader aria_label="Workspace header".to_string()>
  <strong>"Workspace"</strong>
  <span class="ui-muted">"5 active projects"</span>
</SidebarHeader>"#;

    let disabled_code = r#"<SidebarHeader
  disabled=true
  aria_label="Disabled inspector header".to_string()
  class_name="docs-sidebar-header-custom".to_string()
>
  <strong>"Inspector"</strong>
  <span class="ui-muted">"Read-only mode"</span>
</SidebarHeader>"#;

    view! {
        <ComponentPage
            title="SidebarHeader"
            slug="sidebar-header"
            group="Layout"
            description="Shadcn-compatible sidebar header region primitive with centralized disabled/source-state contracts and Spectrum-style data markers."
        >
            <Playground title="Default Header Region" code=basic_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar header playground".to_string()
                >
                    <SidebarHeader aria_label="Workspace header".to_string()>
                        <strong>"Workspace"</strong>
                        <span class="ui-muted">"5 active projects"</span>
                    </SidebarHeader>
                    <div class="docs-stack docs-stack--tight">
                        <span>"Dashboard"</span>
                        <span>"Projects"</span>
                        <span>"Billing"</span>
                    </div>
                </Sidebar>
            </Playground>

            <Playground title="Disabled + Custom Class" code=disabled_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Disabled header sidebar".to_string()
                >
                    <SidebarHeader
                        disabled=true
                        aria_label="Disabled inspector header".to_string()
                        class_name="docs-sidebar-header-custom".to_string()
                    >
                        <strong>"Inspector"</strong>
                        <span class="ui-muted">"Read-only mode"</span>
                    </SidebarHeader>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Tokens"</span>
                        <span class="ui-muted">"Layers"</span>
                    </div>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sidebar_trigger() -> AnyView {
    layout_extra_sidebar_trigger::sidebar_trigger()
}

pub(super) fn sidebar_content() -> AnyView {
    layout_extra_sidebar_content::sidebar_content()
}

pub(super) fn sidebar_footer() -> AnyView {
    layout_extra_sidebar_footer::sidebar_footer()
}

pub(super) fn sidebar_group() -> AnyView {
    let group_items = vec![
        SidebarMenuItem {
            id: "support".to_string(),
            label: "Support".to_string(),
            href: Some("/support".to_string()),
            badge: Some("2".to_string()),
            action_label: Some("Support item action".to_string()),
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
        SidebarMenuItem {
            id: "feedback".to_string(),
            label: "Feedback".to_string(),
            href: Some("/feedback".to_string()),
            badge: Some("1".to_string()),
            action_label: Some("Feedback item action".to_string()),
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
    ];

    let collapsible_items = vec![SidebarMenuItem {
        id: "project".to_string(),
        label: "Project docs".to_string(),
        href: None,
        badge: None,
        action_label: Some("Project item action".to_string()),
        disabled: false,
        sub_items: vec![
            SidebarMenuSubItem {
                id: "install".to_string(),
                label: "Installation".to_string(),
                href: Some("/docs/install".to_string()),
                disabled: false,
            },
            SidebarMenuSubItem {
                id: "routing".to_string(),
                label: "Routing".to_string(),
                href: Some("/docs/routing".to_string()),
                disabled: false,
            },
        ],
        default_sub_open: true,
    }];

    let (action_count, set_action_count) = signal(0_usize);
    let on_group_action = Callback::new(move |_| set_action_count.update(|count| *count += 1));

    let (group_open_raw, set_group_open_raw) = signal(true);
    let group_open: Signal<bool> = Signal::derive(move || group_open_raw.get());
    let on_group_open_change = Callback::new(move |next: bool| set_group_open_raw.set(next));

    let base_code = r#"<SidebarGroup
  label="Help".to_string()
  action_label="Add".to_string()
  on_action=Callback::new(move |_| set_action_count.update(|count| *count += 1))
>
  <SidebarMenu items=items />
</SidebarGroup>"#;

    let controlled_code = r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<SidebarGroup
  open=open
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  collapsible=true
  show_action=false
  label="Architecture".to_string()
>
  <SidebarMenu items=items allow_submenu_collapse=true show_actions=true />
</SidebarGroup>"#;

    view! {
        <ComponentPage
            title="SidebarGroup"
            slug="sidebar-group"
            group="Layout"
            description="Shadcn-compatible sidebar group primitive with label/action header regions, controlled/uncontrolled collapsible state, Spectrum-style data contracts, and motion-ready collapse behavior."
        >
            <Playground title="Label + Group Action" code=base_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar group playground".to_string()
                >
                    <SidebarGroup
                        label="Help".to_string()
                        action_label="Add".to_string()
                        on_action=on_group_action
                        collapsible=false
                        aria_label="Help group".to_string()
                    >
                        <SidebarMenu
                            id_base="docs-sidebar-group-basic".to_string()
                            items=group_items
                            show_actions=false
                            aria_label="Help menu".to_string()
                        />
                    </SidebarGroup>
                    <div class="ui-sidebar__footer">
                        <span class="ui-muted">"group action count: " {move || action_count.get().to_string()}</span>
                    </div>
                </Sidebar>
            </Playground>

            <Playground title="Controlled + Collapsible Group" code=controlled_code>
                <div class="docs-stack docs-stack--tight">
                    <button
                        class="ui-button"
                        type="button"
                        on:click=move |_| set_group_open_raw.update(|open| *open = !*open)
                    >
                        "Toggle group"
                    </button>

                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Controlled group sidebar".to_string()
                    >
                        <SidebarGroup
                            open=group_open
                            on_open_change=on_group_open_change
                            collapsible=true
                            show_action=false
                            label="Architecture".to_string()
                            aria_label="Architecture group".to_string()
                            class_name="docs-sidebar-group-custom".to_string()
                        >
                            <SidebarMenu
                                id_base="docs-sidebar-group-collapsible".to_string()
                                items=collapsible_items
                                allow_submenu_collapse=true
                                show_actions=true
                                show_badges=false
                                aria_label="Architecture menu".to_string()
                            />
                        </SidebarGroup>

                        <div class="ui-sidebar__footer">
                            <span class="ui-muted">
                                "group open: "
                                {move || if group_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                    </Sidebar>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sidebar_menu() -> AnyView {
    let badge_items = vec![
        SidebarMenuItem {
            id: "projects".to_string(),
            label: "Projects".to_string(),
            href: Some("/projects".to_string()),
            badge: Some("24".to_string()),
            action_label: Some("Project actions".to_string()),
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
        SidebarMenuItem {
            id: "support".to_string(),
            label: "Support".to_string(),
            href: Some("/support".to_string()),
            badge: Some("3".to_string()),
            action_label: Some("Support actions".to_string()),
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
        SidebarMenuItem {
            id: "billing".to_string(),
            label: "Billing".to_string(),
            href: Some("/billing".to_string()),
            badge: Some("1".to_string()),
            action_label: Some("Billing actions".to_string()),
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
    ];

    let collapsible_items = vec![
        SidebarMenuItem {
            id: "workspace".to_string(),
            label: "Workspace".to_string(),
            href: None,
            badge: None,
            action_label: Some("Workspace actions".to_string()),
            disabled: false,
            sub_items: vec![
                SidebarMenuSubItem {
                    id: "overview".to_string(),
                    label: "Overview".to_string(),
                    href: Some("/workspace/overview".to_string()),
                    disabled: false,
                },
                SidebarMenuSubItem {
                    id: "tokens".to_string(),
                    label: "Design tokens".to_string(),
                    href: Some("/workspace/tokens".to_string()),
                    disabled: false,
                },
            ],
            default_sub_open: true,
        },
        SidebarMenuItem {
            id: "releases".to_string(),
            label: "Releases".to_string(),
            href: None,
            badge: None,
            action_label: Some("Release actions".to_string()),
            disabled: false,
            sub_items: vec![SidebarMenuSubItem {
                id: "changelog".to_string(),
                label: "Changelog".to_string(),
                href: Some("/releases/changelog".to_string()),
                disabled: false,
            }],
            default_sub_open: false,
        },
    ];

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (last_item_action, set_last_item_action) = signal("none".to_string());
    let on_item_action = Callback::new(move |id: String| set_last_item_action.set(id));

    let (active_raw, set_active_raw) = signal(Some("tokens".to_string()));
    let active: Signal<Option<String>> = Signal::derive(move || active_raw.get());
    let on_active_change = Callback::new(move |next: Option<String>| set_active_raw.set(next));

    let badge_code = r#"<SidebarMenu
  items=items
  on_action=Callback::new(move |id: String| set_last_action.set(id))
  on_item_action=Callback::new(move |id: String| set_last_item_action.set(id))
/>"#;

    let controlled_code = r#"let (active_raw, set_active_raw) = signal(Some("tokens".to_string()));
let active: Signal<Option<String>> = Signal::derive(move || active_raw.get());

<SidebarMenu
  items=items
  active_id=active
  on_active_id_change=Callback::new(move |next| set_active_raw.set(next))
  allow_submenu_collapse=true
  show_badges=false
  show_actions=true
/>"#;

    view! {
        <ComponentPage
            title="SidebarMenu"
            slug="sidebar-menu"
            group="Layout"
            description="Shadcn-compatible sidebar menu primitive with badges/actions/sub-items, controlled active-id flow, collapsible submenu behavior, Spectrum-style data contracts, and HeroUI-level active-highlight motion."
        >
            <Playground title="Badge + Item Action" code=badge_code>
                <div class="docs-stack docs-stack--tight">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Sidebar
                        collapsible=SidebarCollapsible::Offcanvas
                        show_trigger=false
                        aria_label="Menu playground sidebar".to_string()
                    >
                        <div class="ui-sidebar__header">
                            <strong>"Primary navigation"</strong>
                        </div>
                        <SidebarMenu
                            id_base="docs-sidebar-menu-badge".to_string()
                            items=badge_items
                            on_action=on_action
                            on_item_action=on_item_action
                            aria_label="Primary menu".to_string()
                        />
                        <div class="ui-sidebar__footer">
                            <span class="ui-muted">"Action: " {move || last_action.get()} " · Item action: " {move || last_item_action.get()}</span>
                        </div>
                    </Sidebar>
                </div>
            </Playground>

            <Playground title="Controlled + Collapsible Submenu" code=controlled_code>
                <div class="docs-stack docs-stack--tight">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Controlled menu sidebar".to_string()
                    >
                        <div class="ui-sidebar__header">
                            <strong>"Workspace sections"</strong>
                        </div>
                        <SidebarMenu
                            id_base="docs-sidebar-menu-controlled".to_string()
                            items=collapsible_items
                            active_id=active
                            on_active_id_change=on_active_change
                            on_action=on_action
                            on_item_action=on_item_action
                            allow_submenu_collapse=true
                            show_badges=false
                            show_actions=true
                            keyboard_shortcut_key="k".to_string()
                            aria_label="Workspace menu".to_string()
                            class_name="docs-sidebar-menu-custom".to_string()
                        />
                        <div class="ui-sidebar__footer">
                            <span class="ui-muted">
                                "active: "
                                {move || active_raw.get().unwrap_or_else(|| "none".to_string())}
                            </span>
                        </div>
                    </Sidebar>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn surface() -> AnyView {
    layout_extra_surface::surface()
}
