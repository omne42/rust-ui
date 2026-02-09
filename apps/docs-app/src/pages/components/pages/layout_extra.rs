use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AspectRatio, AspectRatioPreset, AspectRatioRadius, Grid, GridAlign, GridColumns, GridGap,
    GridJustify, GridRows, Resizable, ResizableOrientation, ScrollArea, ScrollAreaOrientation,
    Sidebar, SidebarCollapsible, SidebarSide, SidebarVariant, Surface, SurfaceElevation,
    SurfaceTone, View, ViewBackground, ViewBorder, ViewPadding, ViewRadius,
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

pub(super) fn surface() -> AnyView {
    let tone_code = r#"<Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>
  <div>"Default raised surface"</div>
</Surface>
<Surface tone=SurfaceTone::Subtle elevation=SurfaceElevation::Flat bordered=true>
  <div>"Subtle flat bordered surface"</div>
</Surface>
<Surface tone=SurfaceTone::Strong elevation=SurfaceElevation::Floating padded=false>
  <div>"Strong floating compact surface"</div>
</Surface>"#;

    let custom_code = r#"<Surface
  tone=SurfaceTone::Strong
  elevation=SurfaceElevation::Floating
  bordered=true
  aria_label="Deployment summary".to_string()
  class_name="docs-surface-custom".to_string()
>
  <div>"Custom class + aria source marker"</div>
</Surface>"#;

    view! {
        <ComponentPage
            title="Surface"
            slug="surface"
            group="Layout"
            description="Spectrum/HeroUI-style foundational container surface with centralized tone/elevation/frame/source contracts and stable data markers."
        >
            <Playground title="Tone + Elevation + Frame" code=tone_code>
                <div class="docs-stack">
                    <Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Default raised"</strong>
                            <span class="ui-muted">"Primary neutral container for page-level composition."</span>
                        </div>
                    </Surface>

                    <Surface
                        tone=SurfaceTone::Subtle
                        elevation=SurfaceElevation::Flat
                        bordered=true
                    >
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Subtle flat bordered"</strong>
                            <span class="ui-muted">"Low-emphasis container using only border contrast."</span>
                        </div>
                    </Surface>

                    <Surface
                        tone=SurfaceTone::Strong
                        elevation=SurfaceElevation::Floating
                        padded=false
                    >
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Strong floating compact"</strong>
                            <span class="ui-muted">"Higher emphasis with floating elevation and explicit tight content."</span>
                        </div>
                    </Surface>
                </div>
            </Playground>

            <Playground title="Custom Aria + Class" code=custom_code>
                <Surface
                    tone=SurfaceTone::Strong
                    elevation=SurfaceElevation::Floating
                    bordered=true
                    aria_label="Deployment summary".to_string()
                    class_name="docs-surface-custom".to_string()
                >
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Deployment summary"</strong>
                        <span class="ui-muted">
                            "Verifies custom aria source + custom class merge contracts."
                        </span>
                    </div>
                </Surface>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
