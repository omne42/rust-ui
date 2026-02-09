use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AspectRatio, AspectRatioPreset, AspectRatioRadius, Grid, GridAlign, GridColumns, GridGap,
    GridJustify, GridRows, ScrollArea, ScrollAreaOrientation, Surface, SurfaceElevation,
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
