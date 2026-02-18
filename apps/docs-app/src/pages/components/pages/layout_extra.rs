#[path = "layout_extra_sidebar_content.rs"]
mod layout_extra_sidebar_content;
#[path = "layout_extra_sidebar_footer.rs"]
mod layout_extra_sidebar_footer;
#[path = "layout_extra_sidebar_group.rs"]
mod layout_extra_sidebar_group;
#[path = "layout_extra_sidebar_inset.rs"]
mod layout_extra_sidebar_inset;
#[path = "layout_extra_sidebar_menu_action.rs"]
mod layout_extra_sidebar_menu_action;
#[path = "layout_extra_sidebar_menu_badge.rs"]
mod layout_extra_sidebar_menu_badge;
#[path = "layout_extra_sidebar_rail.rs"]
mod layout_extra_sidebar_rail;
#[path = "layout_extra_sidebar_trigger.rs"]
mod layout_extra_sidebar_trigger;
#[path = "layout_extra_surface.rs"]
mod layout_extra_surface;

use crate::pages::components::{ComponentDoc, ComponentPage};

pub(super) const SCROLL_AREA_DOC: ComponentDoc = ComponentDoc {
    name: "ScrollArea",
    slug: "scroll-area",
    group: "Layout",
    page: scroll_area,
};
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AspectRatio, AspectRatioPreset, AspectRatioRadius, Grid, GridAlign, GridColumns, GridGap,
    GridJustify, GridRows, Resizable, ResizableOrientation, ScrollArea, ScrollAreaOrientation,
    Sidebar, SidebarCollapsible, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarMenuSubItem,
    SidebarSide, SidebarVariant, Snippet, View, ViewBackground, ViewBorder, ViewPadding,
    ViewRadius,
};

pub(super) fn aspect_ratio() -> AnyView {
    let preset_code = Signal::derive(move || {
        r#"<AspectRatio ratio=AspectRatioPreset::Square radius=AspectRatioRadius::Sm fill=true>
  <View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::None>"1:1"</View>
</AspectRatio>
<AspectRatio ratio=AspectRatioPreset::Video radius=AspectRatioRadius::Md fill=true>
  <View background=ViewBackground::Accent border=ViewBorder::None padding=ViewPadding::Sm radius=ViewRadius::None>"16:9"</View>
</AspectRatio>
<AspectRatio ratio=AspectRatioPreset::Portrait radius=AspectRatioRadius::Md fill=true>
  <View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::None>"3:4"</View>
</AspectRatio>"#.to_string()
    });

    let framed_code = Signal::derive(move || {
        r#"<AspectRatio
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
</AspectRatio>"#
            .to_string()
    });

    let (workbench_ratio_index, set_workbench_ratio_index) = signal(Some(2_usize));
    let workbench_ratio = Signal::derive(move || match workbench_ratio_index.get().unwrap_or(2) {
        0 => AspectRatioPreset::Square,
        1 => AspectRatioPreset::Standard,
        3 => AspectRatioPreset::Portrait,
        4 => AspectRatioPreset::UltraWide,
        _ => AspectRatioPreset::Video,
    });
    let (workbench_radius_index, set_workbench_radius_index) = signal(Some(2_usize));
    let workbench_radius =
        Signal::derive(move || match workbench_radius_index.get().unwrap_or(2) {
            0 => AspectRatioRadius::None,
            1 => AspectRatioRadius::Sm,
            3 => AspectRatioRadius::Lg,
            4 => AspectRatioRadius::Full,
            _ => AspectRatioRadius::Md,
        });
    let (workbench_bordered, set_workbench_bordered) = signal(false);
    let (workbench_fill, set_workbench_fill) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        let ratio = workbench_ratio.get();
        let radius = workbench_radius.get();
        let bordered = workbench_bordered.get();
        let fill = workbench_fill.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        format!(
            "<AspectRatio\n  ratio=AspectRatioPreset::{ratio:?}\n  radius=AspectRatioRadius::{radius:?}\n  bordered={bordered}\n  fill={fill}\n  aria_label={}\n  class_name={}\n>\n  <View background=ViewBackground::Accent border=ViewBorder::None padding=ViewPadding::Sm radius=ViewRadius::None>\n    \"Workbench preview\"\n  </View>\n</AspectRatio>",
            if custom_aria {
                "\"Workbench media region\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if custom_class {
                "\"docs-aspect-ratio-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/aspect_ratio/styles.rs */\n{}",
            ui_components::aspect_ratio::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let ratio = workbench_ratio.get();
        let radius = workbench_radius.get();
        let bordered = workbench_bordered.get();
        let fill = workbench_fill.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        let mut classes = vec![
            "ui-aspect-ratio".to_string(),
            ratio.class_name().to_string(),
            radius.class_name().to_string(),
        ];
        if bordered {
            classes.push("ui-aspect-ratio--bordered".to_string());
        }
        if fill {
            classes.push("ui-aspect-ratio--fill".to_string());
        }
        if custom_class {
            classes.push("ui-aspect-ratio--custom-class".to_string());
            classes.push("docs-aspect-ratio-custom".to_string());
        }

        let state_attr = if bordered && fill {
            "media"
        } else if bordered {
            "framed"
        } else if fill {
            "fill"
        } else {
            "plain"
        };

        format!(
            "AspectRatioActualConfig {{\n  ratio: {ratio:?},\n  radius: {radius:?},\n  bordered: {bordered},\n  fill: {fill},\n  aria_source: \"{}\",\n  class_source: \"{}\",\n  data_state: \"{state_attr}\",\n  class: \"{}\",\n}}",
            if custom_aria { "custom" } else { "default" },
            if custom_class { "custom" } else { "default" },
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="AspectRatio"
            slug="aspect-ratio"
            group="Layout"
            description="baseline-compatible media frame primitive with centralized ratio/radius/frame/source normalization and stable state-marker contracts."
        >
            <Playground title="Ratio Presets" code_signal=preset_code>
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

            <Playground title="Bordered + Fill + Custom Aria/Class" code_signal=framed_code>
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

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/aspect_ratio/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="展示区对比 default/workbench 两组；Config 调参，Code 查看当前调用，CSS Test 验证样式契约。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="aspect-ratio-config-controls">
                        <div class="docs-search__label">"Ratio"</div>
                        <button
                            type="button"
                            data-action="toggle-ratio-config"
                            on:click=move |_| {
                                set_workbench_ratio_index.update(|value| {
                                    *value = Some((value.unwrap_or(2) + 1) % 5);
                                });
                            }
                        >
                            "Cycle ratio"
                        </button>
                        <div class="docs-search__label">"Radius"</div>
                        <button
                            type="button"
                            data-action="toggle-radius-config"
                            on:click=move |_| {
                                set_workbench_radius_index.update(|value| {
                                    *value = Some((value.unwrap_or(2) + 1) % 5);
                                });
                            }
                        >
                            "Cycle radius"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-bordered-config"
                            on:click=move |_| {
                                set_workbench_bordered.update(|value| *value = !*value);
                            }
                        >
                            "Toggle bordered"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-fill-config"
                            on:click=move |_| {
                                set_workbench_fill.update(|value| *value = !*value);
                            }
                        >
                            "Toggle fill"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-aria-config"
                            on:click=move |_| {
                                set_workbench_custom_aria.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom aria"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-class-config"
                            on:click=move |_| {
                                set_workbench_custom_class.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom class"
                        </button>
                        <p class="ui-muted" data-slot="aspect-ratio-config-summary">
                            {move || {
                                format!(
                                    "config: ratio={:?} radius={:?} bordered={} fill={} aria={} class={}",
                                    workbench_ratio.get(),
                                    workbench_radius.get(),
                                    workbench_bordered.get(),
                                    workbench_fill.get(),
                                    if workbench_custom_aria.get() { "custom" } else { "default" },
                                    if workbench_custom_class.get() { "custom" } else { "default" },
                                )
                            }}
                        </p>
                    </div>
                }
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"default"</span>
                            <AspectRatio ratio=AspectRatioPreset::Video radius=AspectRatioRadius::Md fill=true>
                                <View
                                    background=ViewBackground::Subtle
                                    border=ViewBorder::Subtle
                                    padding=ViewPadding::Sm
                                    radius=ViewRadius::None
                                >
                                    "Default preview"
                                </View>
                            </AspectRatio>
                        </div>

                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"workbench"</span>
                            <AspectRatio
                                ratio=workbench_ratio.get()
                                radius=workbench_radius.get()
                                bordered=workbench_bordered.get()
                                fill=workbench_fill.get()
                                aria_label=if workbench_custom_aria.get() {
                                    "Workbench media region".to_string()
                                } else {
                                    "".to_string()
                                }
                                class_name=if workbench_custom_class.get() {
                                    "docs-aspect-ratio-custom".to_string()
                                } else {
                                    "".to_string()
                                }
                            >
                                <View
                                    background=ViewBackground::Accent
                                    border=ViewBorder::None
                                    padding=ViewPadding::Sm
                                    radius=ViewRadius::None
                                >
                                    "Workbench preview"
                                </View>
                            </AspectRatio>
                        </div>
                    </div>

                    <span class="ui-muted" data-slot="aspect-ratio-workbench-summary">
                        {move || {
                            format!(
                                "ratio={:?} radius={:?} bordered={} fill={} aria={} class={}",
                                workbench_ratio.get(),
                                workbench_radius.get(),
                                workbench_bordered.get(),
                                workbench_fill.get(),
                                if workbench_custom_aria.get() { "custom" } else { "default" },
                                if workbench_custom_class.get() { "custom" } else { "default" },
                            )
                        }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn grid() -> AnyView {
    let columns_code = Signal::derive(move || {
        r#"<Grid columns=GridColumns::Three gap=GridGap::Md>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"A"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"B"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"C"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"D"</View>
</Grid>"#
            .to_string()
    });

    let adaptive_code = Signal::derive(move || {
        r#"<Grid
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
</Grid>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Grid"
            slug="grid"
            group="Layout"
            description="baseline-style grid layout primitive with centralized columns/rows/gap/alignment normalization and stable state-marker contracts."
        >
            <Playground title="Columns + Gap" code_signal=columns_code>
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

            <Playground title="AutoFit + Dense + Equal Rows" code_signal=adaptive_code>
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
    let (marker_orientation, set_marker_orientation) = signal(ScrollAreaOrientation::Vertical);
    let (marker_is_disabled, set_marker_is_disabled) = signal(false);
    let (marker_has_custom_max_height, set_marker_has_custom_max_height) = signal(true);
    let (marker_has_custom_class, set_marker_has_custom_class) = signal(false);
    let (marker_has_custom_aria, set_marker_has_custom_aria) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<ScrollArea>
  <div>"Activity feed"</div>
</ScrollArea>"#
            .to_string()
    });

    let default_code = Signal::derive(move || {
        r#"<ScrollArea max_height_px=180>
  <div class="docs-stack docs-stack--tight">
    {(1..=24)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Release note {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollArea>"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<ScrollArea
  orientation=ScrollAreaOrientation::Horizontal
  max_height_px=120
  class_name="docs-scroll-area-custom".to_string()
>
  <div class="docs-row">
    {(1..=16)
      .map(|idx| {
        view! { <span class="ui-chip ui-chip--flat docs-scroll-area-chip">{format!("Tag {idx}")}</span> }
      })
      .collect_view()}
  </div>
</ScrollArea>

<ScrollArea
  orientation=ScrollAreaOrientation::Both
  is_disabled=Some(true)
  max_height_px=120
  aria_label="Disabled logs".to_string()
>
  <div class="docs-scroll-area-grid">
    {(1..=20)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Cell {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollArea>"#
            .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (marker_orientation, set_marker_orientation) = signal(ScrollAreaOrientation::Vertical);
let (marker_is_disabled, set_marker_is_disabled) = signal(false);
let (marker_has_custom_max_height, set_marker_has_custom_max_height) = signal(true);
let (marker_has_custom_class, set_marker_has_custom_class) = signal(false);
let (marker_has_custom_aria, set_marker_has_custom_aria) = signal(false);

<ScrollArea
  orientation=marker_orientation.get()
  is_disabled=Some(marker_is_disabled.get())
  max_height_px=if marker_has_custom_max_height.get() { Some(140) } else { None }
  class_name=if marker_has_custom_class.get() { "docs-scroll-area-custom".to_string() } else { "".to_string() }
  aria_label=if marker_has_custom_aria.get() { "Marker logs".to_string() } else { "".to_string() }
>
  <div class="docs-stack docs-stack--tight">
    {(1..=20)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Marker row {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollArea>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ScrollArea"
            slug="scroll-area"
            group="Layout"
            description="baseline-compatible scroll container with centralized orientation/max-height/disabled normalization and stable state-marker data contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ScrollArea>
                    <div>"Activity feed"</div>
                </ScrollArea>
            </Playground>

            <Playground title="Vertical + Max Height" code_signal=default_code>
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

            <Playground title="Horizontal + Both + Disabled" code_signal=state_code>
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
                        is_disabled=true
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

            <Playground
                title="Interactive Playground (State + Source Markers)"
                description="Toggle orientation/disabled/max-height/class/aria inputs and inspect live `data-*` + `aria-*` contracts."
                code_signal=marker_code
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        if marker_has_custom_max_height.get() {
                            view! {
                                <ScrollArea
                                    orientation=marker_orientation.get()
                                    is_disabled=marker_is_disabled.get()
                                    max_height_px=140
                                    class_name=if marker_has_custom_class.get() {
                                        "docs-scroll-area-custom".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                    aria_label=if marker_has_custom_aria.get() {
                                        "Marker logs".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        {(1..=20)
                                            .map(|idx| {
                                                view! {
                                                    <div class="docs-scroll-shadow-item">{format!("Marker row {idx}")}</div>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </ScrollArea>
                            }
                                .into_any()
                        } else {
                            view! {
                                <ScrollArea
                                    orientation=marker_orientation.get()
                                    is_disabled=marker_is_disabled.get()
                                    class_name=if marker_has_custom_class.get() {
                                        "docs-scroll-area-custom".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                    aria_label=if marker_has_custom_aria.get() {
                                        "Marker logs".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        {(1..=20)
                                            .map(|idx| {
                                                view! {
                                                    <div class="docs-scroll-shadow-item">{format!("Marker row {idx}")}</div>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </ScrollArea>
                            }
                                .into_any()
                        }
                    }}

                    <div class="docs-row" data-slot="scroll-area-marker-controls">
                        <div data-slot="scroll-area-toggle-orientation">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_orientation.update(|value| {
                                        *value = match *value {
                                            ScrollAreaOrientation::Vertical =>
                                                ScrollAreaOrientation::Horizontal,
                                            ScrollAreaOrientation::Horizontal =>
                                                ScrollAreaOrientation::Both,
                                            ScrollAreaOrientation::Both =>
                                                ScrollAreaOrientation::Vertical,
                                        };
                                    })
                                })
                            >
                                {move || format!("Orientation: {}", match marker_orientation.get() {
                                    ScrollAreaOrientation::Vertical => "vertical",
                                    ScrollAreaOrientation::Horizontal => "horizontal",
                                    ScrollAreaOrientation::Both => "both",
                                })}
                            </ui_components::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-disabled">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_is_disabled.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_is_disabled.get() {
                                    "Set enabled"
                                } else {
                                    "Set disabled"
                                }}
                            </ui_components::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-max-height">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_has_custom_max_height.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_has_custom_max_height.get() {
                                    "Use default max height"
                                } else {
                                    "Use custom max height"
                                }}
                            </ui_components::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-class">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_has_custom_class.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_has_custom_class.get() {
                                    "Clear custom class"
                                } else {
                                    "Set custom class"
                                }}
                            </ui_components::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-aria">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_has_custom_aria.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_has_custom_aria.get() {
                                    "Use default aria label"
                                } else {
                                    "Use custom aria label"
                                }}
                            </ui_components::Button>
                        </div>
                    </div>

                    <span class="ui-muted" data-slot="scroll-area-marker-summary">
                        "orientation="
                        {move || match marker_orientation.get() {
                            ScrollAreaOrientation::Vertical => "vertical",
                            ScrollAreaOrientation::Horizontal => "horizontal",
                            ScrollAreaOrientation::Both => "both",
                        }}
                        " · disabled="
                        {move || if marker_is_disabled.get() { "true" } else { "false" }}
                        " · max-height="
                        {move || if marker_has_custom_max_height.get() {
                            "custom"
                        } else {
                            "default"
                        }}
                        " · class="
                        {move || if marker_has_custom_class.get() {
                            "custom"
                        } else {
                            "default"
                        }}
                        " · aria="
                        {move || if marker_has_custom_aria.get() {
                            "custom"
                        } else {
                            "default"
                        }}
                    </span>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="scroll-area-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " with one-click copy. Snippets are import-ready through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::{ScrollArea, ScrollAreaOrientation};\n\n<ScrollArea\n  orientation=ScrollAreaOrientation::Vertical\n  max_height_px=180\n  aria_label=\"Release feed\".to_string()\n>\n  <div class=\"docs-stack docs-stack--tight\">\n    <div>\"Release note 1\"</div>\n    <div>\"Release note 2\"</div>\n  </div>\n</ScrollArea>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-scroll-area-source-copy".to_string()
                />
                <ul data-slot="scroll-area-source-paths">
                    <li><code>"crates/ui-components/src/scroll_area/mod.rs"</code></li>
                    <li><code>"crates/ui-components/src/scroll_area/logic.rs"</code></li>
                    <li><code>"crates/ui-components/src/scroll_area/view.rs"</code></li>
                    <li><code>"crates/ui-components/src/scroll_area/styles.rs"</code></li>
                    <li><code>"crates/ui-components/src/scroll_area/motion.rs"</code></li>
                </ul>
                <ul data-slot="scroll-area-source-prerequisites">
                    <li><code>"component-scroll_area"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="scroll-area-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="scroll-area-api-rows">
                    <li>
                        <code>"orientation: ScrollAreaOrientation"</code>
                        " default = ScrollAreaOrientation::Vertical"
                    </li>
                    <li>
                        <code>"max_height_px: Option<u32>"</code>
                        " default = None (no custom max-height marker)"
                    </li>
                    <li>
                        <code>"is_disabled: Option<bool> + disabled: bool"</code>
                        " `is_disabled` has priority; `disabled` is legacy compatibility path"
                    </li>
                    <li>
                        <code>"aria_label: Option<String>"</code>
                        " fallback = ui_components::scroll_area::DEFAULT_ARIA_LABEL"
                    </li>
                    <li>
                        <code>"class_name / lang / dir / motion"</code>
                        " optional style + locale + motion contract inputs"
                    </li>
                    <li>
                        <code>"children: Children"</code>
                        " explicit composition payload"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="scroll-area-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="scroll-area-state-rows">
                    <li>
                        <code>"data-orientation"</code>
                        " = vertical | horizontal | both"
                    </li>
                    <li>
                        <code>"data-disabled / data-disabled-source"</code>
                        " = true? and is-prop | legacy-prop source provenance"
                    </li>
                    <li>
                        <code>"data-max-height / data-aria-source / data-class-source / data-custom-class"</code>
                        " = default | custom marker set"
                    </li>
                    <li>
                        <code>"data-ui-schema / data-ui-intent / data-ui-action / data-ui-state / data-ui-source / data-ui-output-status"</code>
                        " machine-readable agent contract + snapshot status markers"
                    </li>
                    <li>
                        <code>"tabindex / aria-disabled"</code>
                        " viewport accessibility path = enabled(0,None) | disabled(-1,true)"
                    </li>
                    <li>
                        <code>"controlled/uncontrolled value axis"</code>
                        " N/A for ScrollArea (no value/open state machine)"
                    </li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn resizable() -> AnyView {
    let horizontal_code = Signal::derive(move || {
        r#"<Resizable
  orientation=ResizableOrientation::Horizontal
  default_value=36.0
  is_with_handle=true
  first=move || view! { <div>"Sidebar"</div> }
  second=move || view! { <div>"Content"</div> }
/>"#
        .to_string()
    });

    let vertical_code = Signal::derive(move || {
        r#"let (split_raw, set_split_raw) = signal(58.0_f64);
let split: Signal<f64> = Signal::derive(move || split_raw.get());

<Resizable
  orientation=ResizableOrientation::Vertical
  value=split
  on_value_change=Callback::new(move |next| set_split_raw.set(next))
  min_split_percent=25.0
  max_split_percent=80.0
  is_with_handle=true
  class_name="docs-resizable-custom".to_string()
  first=move || view! { <div>"Header"</div> }
  second=move || view! { <div>"Body"</div> }
/>"#
        .to_string()
    });

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
            description="baseline-compatible panel splitter with controlled/uncontrolled split state, pointer + keyboard resize semantics, and baseline-style state data contracts."
        >
            <Playground title="Horizontal + Handle Grip" code_signal=horizontal_code>
                <Resizable
                    orientation=ResizableOrientation::Horizontal
                    default_value=36.0
                    is_with_handle=true
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

            <Playground title="Controlled + Vertical Bounds" code_signal=vertical_code>
                <div class="docs-stack docs-stack--tight">
                    <Resizable
                        orientation=ResizableOrientation::Vertical
                        value=split
                        on_value_change=on_split_change
                        min_split_percent=25.0
                        max_split_percent=80.0
                        is_with_handle=true
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

            <section class="docs-card docs-prose" data-slot="resizable-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="resizable-api-rows">
                    <li>
                        <code>"orientation: ResizableOrientation"</code>
                        " horizontal | vertical"
                    </li>
                    <li>
                        <code>"value + on_value_change + default_value"</code>
                        " canonical controlled/uncontrolled split axis"
                    </li>
                    <li>
                        <code>"split_percent + on_split_percent_change + default_split_percent"</code>
                        " legacy aliases supported for migration"
                    </li>
                    <li>
                        <code>"min_split_percent / max_split_percent"</code>
                        " bounded normalization in logic/primitive layer"
                    </li>
                    <li>
                        <code>"is_disabled / is_with_handle"</code>
                        " prefixed accessibility + presentation switches"
                    </li>
                    <li>
                        <code>"lang / dir / aria_label / class_name / motion"</code>
                        " locale + semantics + visual contract inputs"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="resizable-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="resizable-state-rows">
                    <li>
                        <code>"data-state"</code>
                        " = idle | dragging | disabled"
                    </li>
                    <li>
                        <code>"data-orientation"</code>
                        " = horizontal | vertical"
                    </li>
                    <li>
                        <code>"data-control-mode / data-value-source / data-default-value-source"</code>
                        " controlled/uncontrolled and source provenance markers"
                    </li>
                    <li>
                        <code>"data-value-change-source / data-disabled-source / data-handle-source"</code>
                        " source markers for update + disabled + handle decision paths"
                    </li>
                    <li>
                        <code>"data-ui-schema / data-ui-intent / data-ui-stream-* / data-ui-output-status"</code>
                        " machine-readable agent contract + snapshot fallback policy"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="resizable-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " and copy action. Snippets are import-ready through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::{Resizable, ResizableOrientation};\n\n<Resizable\n  orientation=ResizableOrientation::Horizontal\n  default_value=40.0\n  first=move || view! { <div>\"Left\"</div> }\n  second=move || view! { <div>\"Right\"</div> }\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-resizable-source-copy".to_string()
                />
                <ul data-slot="resizable-source-paths">
                    <li><code>"crates/ui-components/src/resizable/mod.rs"</code></li>
                    <li><code>"crates/ui-components/src/resizable/logic.rs"</code></li>
                    <li><code>"crates/ui-components/src/resizable/view.rs"</code></li>
                    <li><code>"crates/ui-components/src/resizable/styles.rs"</code></li>
                    <li><code>"crates/ui-components/src/resizable/motion.rs"</code></li>
                </ul>
                <ul data-slot="resizable-source-prerequisites">
                    <li><code>"component-resizable"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sidebar() -> AnyView {
    let basic_code = Signal::derive(move || {
        r#"<Sidebar
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
</Sidebar>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
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
</Sidebar>"#
            .to_string()
    });

    let (open_raw, set_open_raw) = signal(true);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    view! {
        <ComponentPage
            title="Sidebar"
            slug="sidebar"
            group="Layout"
            description="baseline-compatible sidebar primitive with controlled/uncontrolled open state, side+variant+collapsible contracts, keyboard shortcut toggle, and baseline-style data markers."
        >
            <Playground title="Offcanvas + Slot Markers" code_signal=basic_code>
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

            <Playground title="Controlled + Right Inset/Icon" code_signal=controlled_code>
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
    let basic_code = Signal::derive(move || {
        r#"<SidebarHeader aria_label="Workspace header".to_string()>
  <strong>"Workspace"</strong>
  <span class="ui-muted">"5 active projects"</span>
</SidebarHeader>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<SidebarHeader
  disabled=true
  aria_label="Disabled inspector header".to_string()
  class_name="docs-sidebar-header-custom".to_string()
>
  <strong>"Inspector"</strong>
  <span class="ui-muted">"Read-only mode"</span>
</SidebarHeader>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarHeader"
            slug="sidebar-header"
            group="Layout"
            description="baseline-compatible sidebar header region primitive with centralized disabled/source-state contracts and baseline-style data markers."
        >
            <Playground title="Default Header Region" code_signal=basic_code>
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

            <Playground title="Disabled + Custom Class" code_signal=disabled_code>
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

pub(super) fn sidebar_rail() -> AnyView {
    layout_extra_sidebar_rail::sidebar_rail()
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

pub(super) fn sidebar_inset() -> AnyView {
    layout_extra_sidebar_inset::sidebar_inset()
}

pub(super) fn sidebar_group() -> AnyView {
    layout_extra_sidebar_group::sidebar_group()
}

pub(super) fn sidebar_menu_action() -> AnyView {
    layout_extra_sidebar_menu_action::sidebar_menu_action()
}

pub(super) fn sidebar_menu_badge() -> AnyView {
    layout_extra_sidebar_menu_badge::sidebar_menu_badge()
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

    let badge_code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal("none".to_string());
let (last_item_action, set_last_item_action) = signal("none".to_string());

<SidebarMenu
  items=vec![
    SidebarMenuItem {
      id: "activity".to_string(),
      label: "Activity".to_string(),
      href: Some("/activity".to_string()),
      badge: Some("4".to_string()),
      action_label: Some("Activity actions".to_string()),
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
  ]
  on_action=Callback::new(move |id: String| set_last_action.set(id))
  on_item_action=Callback::new(move |id: String| set_last_item_action.set(id))
/>
<span class="ui-muted">"Action: " {move || last_action.get()} " · Item action: " {move || last_item_action.get()}</span>"#
        .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (active_raw, set_active_raw) = signal(Some("tokens".to_string()));

<SidebarMenu
  items=vec![
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
  ]
  active_id=Signal::derive(move || active_raw.get())
  on_active_id_change=Callback::new(move |next| set_active_raw.set(next))
  allow_submenu_collapse=true
  show_badges=false
  show_actions=true
/>
<span class="ui-muted">"active: " {move || active_raw.get().unwrap_or_else(|| "none".to_string())}</span>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarMenu"
            slug="sidebar-menu"
            group="Layout"
            description="baseline-compatible sidebar menu primitive with badges/actions/sub-items, controlled active-id flow, collapsible submenu behavior, baseline-style data contracts, and baseline-level active-highlight motion."
        >
            <Playground title="Badge + Item Action" code_signal=badge_code>
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

            <Playground title="Controlled + Collapsible Submenu" code_signal=controlled_code>
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
