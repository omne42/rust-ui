use super::playground_workbench::{bool_word, rust_string_literal};
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
use ui::{
    A11yDirection, SegmentedControl, SegmentedControlSize, Sidebar, SidebarCollapsible,
    SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarMenuMotion, SidebarMenuSubItem,
    SidebarSide, SidebarVariant, Snippet, Switch,
};
use ui_layout::{
    AspectRatio, AspectRatioPreset, AspectRatioRadius, Grid, GridAlign, GridColumns, GridGap,
    GridJustify, GridRows, Resizable, ResizableMotion, ResizableOrientation, ScrollArea,
    ScrollAreaOrientation, View, ViewBackground, ViewBorder, ViewPadding, ViewRadius,
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
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_code = Signal::derive(move || {
        let ratio = workbench_ratio.get();
        let radius = workbench_radius.get();
        let bordered = workbench_bordered.get();
        let fill = workbench_fill.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        format!(
            "<AspectRatio\n  ratio=AspectRatioPreset::{ratio:?}\n  radius=AspectRatioRadius::{radius:?}\n  bordered={bordered}\n  fill={fill}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir={}\n>\n  <View background=ViewBackground::Accent border=ViewBorder::None padding=ViewPadding::Sm radius=ViewRadius::None>\n    \"Workbench preview\"\n  </View>\n</AspectRatio>",
            if custom_aria {
                "\"Workbench media region\".into()"
            } else {
                "\"\".into()"
            },
            if custom_class {
                "\"docs-aspect-ratio-custom\".into()"
            } else {
                "\"\".into()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en-US\".to_string()"
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/aspect_ratio/styles.rs */\n{}",
            ui_layout::aspect_ratio::styles::CSS
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
            ratio.class_name().into(),
            radius.class_name().into(),
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
            "AspectRatioActualConfig {{\n  ratio: {ratio:?},\n  radius: {radius:?},\n  bordered: {bordered},\n  fill: {fill},\n  aria_label: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  aria_source: \"{}\",\n  class_source: \"{}\",\n  data_state: \"{state_attr}\",\n  class: \"{}\",\n}}",
            if custom_aria {
                "Workbench media region"
            } else {
                ""
            },
            if custom_class {
                "docs-aspect-ratio-custom"
            } else {
                ""
            },
            if workbench_rtl.get() { "ar" } else { "en-US" },
            if workbench_rtl.get() {
                A11yDirection::Rtl
            } else {
                A11yDirection::Ltr
            },
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
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/aspect_ratio/styles.rs".to_string()
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
                        <button
                            type="button"
                            data-action="toggle-rtl-config"
                            on:click=move |_| {
                                set_workbench_rtl.update(|value| *value = !*value);
                            }
                        >
                            "Toggle RTL locale"
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
                                lang=if workbench_rtl.get() {
                                    "ar".to_string()
                                } else {
                                    "en-US".to_string()
                                }
                                dir=if workbench_rtl.get() {
                                    A11yDirection::Rtl
                                } else {
                                    A11yDirection::Ltr
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

            <Playground title="State Matrix (Preset / Border / Locale Comparison)" code_signal=framed_code>
                <div class="docs-stack">
                    <AspectRatio
                        ratio=AspectRatioPreset::Square
                        radius=AspectRatioRadius::Sm
                        fill=true
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <View
                            background=ViewBackground::Subtle
                            border=ViewBorder::Subtle
                            padding=ViewPadding::Sm
                            radius=ViewRadius::None
                        >
                            "1:1"
                        </View>
                    </AspectRatio>
                    <AspectRatio
                        ratio=AspectRatioPreset::UltraWide
                        radius=AspectRatioRadius::Lg
                        bordered=true
                        fill=true
                        aria_label="Release trailer preview".to_string()
                        class_name="docs-aspect-ratio-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
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
                    <AspectRatio
                        ratio=AspectRatioPreset::Portrait
                        radius=AspectRatioRadius::Md
                        bordered=false
                        fill=true
                        aria_label="Arabic preview".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn grid() -> AnyView {
    let (workbench_auto_fit, set_workbench_auto_fit) = signal(false);
    let (workbench_equal_rows, set_workbench_equal_rows) = signal(false);
    let (workbench_dense, set_workbench_dense) = signal(false);
    let (workbench_inline, set_workbench_inline) = signal(false);
    let (workbench_stretch, set_workbench_stretch) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_columns = Signal::derive(move || {
        if workbench_auto_fit.get() {
            GridColumns::AutoFit
        } else {
            GridColumns::Three
        }
    });
    let workbench_rows = Signal::derive(move || {
        if workbench_equal_rows.get() {
            GridRows::Equal
        } else {
            GridRows::Auto
        }
    });
    let workbench_gap = Signal::derive(move || {
        if workbench_stretch.get() {
            GridGap::Lg
        } else {
            GridGap::Md
        }
    });
    let workbench_justify = Signal::derive(move || {
        if workbench_stretch.get() {
            GridJustify::Stretch
        } else {
            GridJustify::Start
        }
    });
    let workbench_align = Signal::derive(move || {
        if workbench_stretch.get() {
            GridAlign::Stretch
        } else {
            GridAlign::Start
        }
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Grid\n  columns=GridColumns::{:?}\n  rows=GridRows::{:?}\n  gap=GridGap::{:?}\n  justify=GridJustify::{:?}\n  align=GridAlign::{:?}\n  dense={}\n  inline={}\n  aria_label={}\n  class_name={}\n>\n  ...\n</Grid>",
            workbench_columns.get(),
            workbench_rows.get(),
            workbench_gap.get(),
            workbench_justify.get(),
            workbench_align.get(),
            workbench_dense.get(),
            workbench_inline.get(),
            if workbench_custom_aria.get() {
                "\"Overview cards grid\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_custom_class.get() {
                "\"docs-grid-adaptive\".to_string()"
            } else {
                "\"\".to_string()"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "GridActualConfig {{\n  columns: GridColumns::{:?},\n  rows: GridRows::{:?},\n  gap: GridGap::{:?},\n  justify: GridJustify::{:?},\n  align: GridAlign::{:?},\n  dense: {},\n  inline: {},\n  aria_label: {},\n  class_name: {},\n}}",
            workbench_columns.get(),
            workbench_rows.get(),
            workbench_gap.get(),
            workbench_justify.get(),
            workbench_align.get(),
            workbench_dense.get(),
            workbench_inline.get(),
            if workbench_custom_aria.get() {
                "Some(\"Overview cards grid\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-grid-adaptive\")"
            } else {
                "None"
            },
        )
    });

    let columns_code = Signal::derive(move || {
        r#"<Grid columns=GridColumns::Three gap=GridGap::Md>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"A"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"B"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"C"</View>
</Grid>"#
            .to_string()
    });

    let adaptive_code = Signal::derive(move || {
        r#"<Grid columns=GridColumns::AutoFit rows=GridRows::Equal gap=GridGap::Lg justify=GridJustify::Stretch align=GridAlign::Stretch dense=true aria_label="Overview cards grid".to_string()>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"Dense"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"Equal rows"</View>
</Grid>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Grid"
            slug="grid"
            group="Layout"
            description="baseline-style grid layout primitive with centralized columns/rows/gap/alignment normalization and stable state-marker contracts."
        >
            <Playground title="Columns + Gap" code_signal=columns_code>
                <Grid columns=GridColumns::Three gap=GridGap::Md>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "A"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "B"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "C"
                    </View>
                </Grid>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="grid-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_auto_fit.get()
                                on:change=move |ev| set_workbench_auto_fit.set(event_target_checked(&ev))
                            />
                            " columns auto-fit"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_equal_rows.get()
                                on:change=move |ev| set_workbench_equal_rows.set(event_target_checked(&ev))
                            />
                            " rows equal"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_stretch.get()
                                on:change=move |ev| set_workbench_stretch.set(event_target_checked(&ev))
                            />
                            " justify/align stretch"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_dense.get()
                                on:change=move |ev| set_workbench_dense.set(event_target_checked(&ev))
                            />
                            " dense"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_inline.get()
                                on:change=move |ev| set_workbench_inline.set(event_target_checked(&ev))
                            />
                            " inline"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " aria_label"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                    </div>
                }
            >
                <Grid
                    columns=workbench_columns.get()
                    rows=workbench_rows.get()
                    gap=workbench_gap.get()
                    justify=workbench_justify.get()
                    align=workbench_align.get()
                    dense=workbench_dense.get()
                    inline=workbench_inline.get()
                    aria_label=if workbench_custom_aria.get() {
                        "Overview cards grid".to_string()
                    } else {
                        String::new()
                    }
                    class_name=if workbench_custom_class.get() {
                        "docs-grid-adaptive".to_string()
                    } else {
                        String::new()
                    }
                >
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "Revenue"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "Users"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "Latency"
                    </View>
                    <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                        "Errors"
                    </View>
                </Grid>
            </Playground>

            <Playground title="AutoFit + Dense + Equal Rows" code_signal=adaptive_code>
                <div class="docs-stack docs-stack--tight">
                    <Grid columns=GridColumns::Three rows=GridRows::Auto gap=GridGap::Md>
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Basic"
                        </View>
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Three"
                        </View>
                    </Grid>
                    <Grid
                        columns=GridColumns::AutoFit
                        rows=GridRows::Equal
                        gap=GridGap::Lg
                        justify=GridJustify::Stretch
                        align=GridAlign::Stretch
                        dense=true
                    >
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Dense"
                        </View>
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Equal rows"
                        </View>
                    </Grid>
                    <Grid
                        columns=GridColumns::Three
                        rows=GridRows::Auto
                        gap=GridGap::Md
                        inline=true
                        class_name="docs-grid-adaptive".to_string()
                    >
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Inline"
                        </View>
                    </Grid>
                </div>
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
</ScrollArea>"#.to_string()
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
</ScrollArea>"#.to_string()
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
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
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
                            </ui::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-disabled">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_is_disabled.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_is_disabled.get() {
                                    "Set enabled"
                                } else {
                                    "Set disabled"
                                }}
                            </ui::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-max-height">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_has_custom_max_height.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_has_custom_max_height.get() {
                                    "Use default max height"
                                } else {
                                    "Use custom max height"
                                }}
                            </ui::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-class">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_has_custom_class.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_has_custom_class.get() {
                                    "Clear custom class"
                                } else {
                                    "Set custom class"
                                }}
                            </ui::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-aria">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_has_custom_aria.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_has_custom_aria.get() {
                                    "Use default aria label"
                                } else {
                                    "Use custom aria label"
                                }}
                            </ui::Button>
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
                    text="use leptos::prelude::*;\nuse ui_layout::{ScrollArea, ScrollAreaOrientation};\n\n<ScrollArea\n  orientation=ScrollAreaOrientation::Vertical\n  max_height_px=180\n  aria_label=\"Release feed\".into()\n>\n  <div class=\"docs-stack docs-stack--tight\">\n    <div>\"Release note 1\"</div>\n    <div>\"Release note 2\"</div>\n  </div>\n</ScrollArea>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-scroll-area-source-copy".to_string()
                />
                <ul data-slot="scroll-area-source-paths">
                    <li><code>"crates/ui-layout/src/scroll_area/mod.rs"</code></li>
                    <li><code>"crates/ui-layout/src/scroll_area/logic.rs"</code></li>
                    <li><code>"crates/ui-layout/src/scroll_area/view.rs"</code></li>
                    <li><code>"crates/ui-layout/src/scroll_area/styles.rs"</code></li>
                    <li><code>"crates/ui-layout/src/scroll_area/motion.rs"</code></li>
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
                        <code>"is_disabled: Option<bool>"</code>
                        " None -> default(false), Some(v) -> is-prop"
                    </li>
                    <li>
                        <code>"aria_label: Option<String>"</code>
                        " fallback = ui_layout::scroll_area::DEFAULT_ARIA_LABEL"
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
                        " = true? and is-prop | default source provenance"
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
    let (workbench_split_raw, set_workbench_split_raw) = signal(58.0_f64);
    let workbench_value: Signal<f64> = Signal::derive(move || workbench_split_raw.get());
    let workbench_split_percent: Signal<f64> = Signal::derive(move || workbench_split_raw.get());
    let (last_value_change, set_last_value_change) = signal("58.0".to_string());
    let (last_split_change, set_last_split_change) = signal("58.0".to_string());
    let on_value_change = Callback::new(move |next: f64| {
        set_last_value_change.set(format!("{next:.1}"));
        set_workbench_split_raw.set(next);
    });
    let on_split_percent_change = Callback::new(move |next: f64| {
        set_last_split_change.set(format!("{next:.1}"));
        set_workbench_split_raw.set(next);
    });

    let (workbench_orientation_key, set_workbench_orientation_key) =
        signal("horizontal".to_string());
    let workbench_orientation = Signal::derive(move || {
        if workbench_orientation_key.get() == "vertical" {
            ResizableOrientation::Vertical
        } else {
            ResizableOrientation::Horizontal
        }
    });
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_with_handle, set_workbench_with_handle) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_min, set_workbench_min) = signal(25.0_f64);
    let (workbench_max, set_workbench_max) = signal(80.0_f64);

    let (split_raw, set_split_raw) = signal(58.0_f64);
    let split: Signal<f64> = Signal::derive(move || split_raw.get());
    let on_split_change = Callback::new(move |next: f64| set_split_raw.set(next));

    let horizontal_code = Signal::derive(move || {
        r#"<Resizable
  orientation=ResizableOrientation::Horizontal
  default_value=40.0
  first=move || view! { <div>"Sidebar"</div> }
  second=move || view! { <div>"Content"</div> }
/>"#
        .to_string()
    });

    let vertical_code = Signal::derive(move || {
        r#"<Resizable
  orientation=ResizableOrientation::Vertical
  value=split
  on_value_change=on_split_change
  min_split_percent=25.0
  max_split_percent=80.0
  is_with_handle=true
  aria_label="Deployment regions split".to_string()
  class_name="docs-resizable-custom".to_string()
  first=move || view! { <div>\"Left\"</div> }
  second=move || view! { <div>\"Right\"</div> }
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Resizable\n  orientation={}\n  value=value\n  split_percent=split_percent\n  default_value=58.0\n  default_split_percent=58.0\n  on_value_change=on_value_change\n  on_split_percent_change=on_split_percent_change\n  min_split_percent={:.1}\n  max_split_percent={:.1}\n  is_disabled={}\n  disabled={}\n  is_with_handle={}\n  with_handle={}\n  aria_label=\"Workspace split\".to_string()\n  class_name={}\n  lang={}\n  dir={}\n  motion={}\n  first=move || view! {{ <div>\"Primary panel\"</div> }}\n  second=move || view! {{ <div>\"Secondary panel\"</div> }}\n/>",
            if workbench_orientation.get() == ResizableOrientation::Vertical {
                "ResizableOrientation::Vertical"
            } else {
                "ResizableOrientation::Horizontal"
            },
            workbench_min.get(),
            workbench_max.get(),
            workbench_disabled.get(),
            workbench_disabled.get(),
            workbench_with_handle.get(),
            workbench_with_handle.get(),
            if workbench_custom_class.get() {
                "\"docs-resizable-workbench\".to_string()"
            } else {
                "String::new()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en\".to_string()"
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
            if workbench_custom_motion.get() {
                "ResizableMotion { enabled: true, panel_duration_ms: 120, handle_duration_ms: 120 }"
            } else {
                "ResizableMotion::default()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ResizableWorkbenchConfig {{\n  orientation: {},\n  value: {:.1},\n  split_percent: {:.1},\n  default_value: Some(58.0),\n  default_split_percent: Some(58.0),\n  on_value_change: Some(\"Callback<f64>\"),\n  on_split_percent_change: Some(\"Callback<f64>\"),\n  min_split_percent: {:.1},\n  max_split_percent: {:.1},\n  is_disabled: Some({}),\n  disabled: {},\n  is_with_handle: Some({}),\n  with_handle: {},\n  aria_label: Some(\"Workspace split\"),\n  class_name: {},\n  lang: {},\n  dir: {},\n  motion: {},\n  first: \"ViewFn(primary)\",\n  second: \"ViewFn(secondary)\",\n}}",
            if workbench_orientation.get() == ResizableOrientation::Vertical {
                "Vertical"
            } else {
                "Horizontal"
            },
            workbench_split_raw.get(),
            workbench_split_raw.get(),
            workbench_min.get(),
            workbench_max.get(),
            workbench_disabled.get(),
            workbench_disabled.get(),
            workbench_with_handle.get(),
            workbench_with_handle.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-resizable-workbench\")"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_motion.get() {
                "ResizableMotion::custom"
            } else {
                "ResizableMotion::default"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Resizable orientation=ResizableOrientation::Horizontal default_value=40.0 first=move || view! { <div>"Sidebar"</div> } second=move || view! { <div>"Content"</div> } />
<Resizable orientation=ResizableOrientation::Vertical default_value=60.0 is_with_handle=true first=move || view! { <div>"Header"</div> } second=move || view! { <div>"Body"</div> } />
<Resizable orientation=ResizableOrientation::Horizontal default_value=35.0 is_disabled=true first=move || view! { <div>"Disabled left"</div> } second=move || view! { <div>"Disabled right"</div> } />"#
            .to_string()
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
                                <strong>"Sidebar"</strong>
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
                                <strong>"Content"</strong>
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
                        first=move || view! { <div>"Header"</div> }
                        second=move || view! { <div>"Body"</div> }
                    />
                    <span class="ui-muted">
                        "controlled split: "
                        {move || format!("{:.1}%", split_raw.get())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Config Workbench"
                description="Covers full Resizable API with callback feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="resizable-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Orientation"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| set_workbench_orientation_key.set(event_target_value(&ev))
                            >
                                <option value="horizontal" selected=move || workbench_orientation_key.get() == "horizontal">"Horizontal"</option>
                                <option value="vertical" selected=move || workbench_orientation_key.get() == "vertical">"Vertical"</option>
                            </select>
                        </label>
                        <label class="docs-choice-row">
                            <span>"Min"</span>
                            <input
                                type="number"
                                prop:value=move || workbench_min.get().to_string()
                                on:change=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().ok().unwrap_or(25.0);
                                    set_workbench_min.set(next);
                                }
                            />
                        </label>
                        <label class="docs-choice-row">
                            <span>"Max"</span>
                            <input
                                type="number"
                                prop:value=move || workbench_max.get().to_string()
                                on:change=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().ok().unwrap_or(80.0);
                                    set_workbench_max.set(next);
                                }
                            />
                        </label>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_with_handle set_checked=set_workbench_with_handle>"With handle"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Custom motion"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="resizable-workbench-preview">
                    <Resizable
                        orientation=workbench_orientation.get()
                        value=workbench_value
                        split_percent=workbench_split_percent
                        default_value=58.0
                        default_split_percent=58.0
                        on_value_change=on_value_change
                        on_split_percent_change=on_split_percent_change
                        min_split_percent=workbench_min.get()
                        max_split_percent=workbench_max.get()
                        is_disabled=workbench_disabled.get()
                        disabled=workbench_disabled.get()
                        is_with_handle=workbench_with_handle.get()
                        with_handle=workbench_with_handle.get()
                        aria_label="Workspace split".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-resizable-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        motion=if workbench_custom_motion.get() {
                            ResizableMotion {
                                enabled: true,
                                panel_duration_ms: 120,
                                handle_duration_ms: 120,
                            }
                        } else {
                            ResizableMotion::default()
                        }
                        first=move || {
                            view! {
                                <View
                                    background=ViewBackground::Subtle
                                    border=ViewBorder::Subtle
                                    padding=ViewPadding::Md
                                    radius=ViewRadius::None
                                >
                                    <strong>"Primary panel"</strong>
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
                                    <strong>"Secondary panel"</strong>
                                </View>
                            }
                        }
                    />
                    <span class="ui-muted">
                        "split=" {move || format!("{:.1}", workbench_split_raw.get())}
                        " · on_value_change=" {move || last_value_change.get()}
                        " · on_split_percent_change=" {move || last_split_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <Resizable
                        orientation=ResizableOrientation::Horizontal
                        default_value=40.0
                        first=move || view! { <div>"Sidebar"</div> }
                        second=move || view! { <div>"Content"</div> }
                    />
                    <Resizable
                        orientation=ResizableOrientation::Vertical
                        default_value=60.0
                        is_with_handle=true
                        first=move || view! { <div>"Header"</div> }
                        second=move || view! { <div>"Body"</div> }
                    />
                    <Resizable
                        orientation=ResizableOrientation::Horizontal
                        default_value=35.0
                        is_disabled=true
                        first=move || view! { <div>"Disabled left"</div> }
                        second=move || view! { <div>"Disabled right"</div> }
                    />
                </div>
            </Playground>

            <section class="docs-stack docs-stack--tight" data-slot="resizable-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="resizable-api-rows">
                    <li><code>"orientation / value / on_value_change / min_split_percent / max_split_percent"</code></li>
                    <li><code>"is_with_handle / is_disabled / aria_label / class_name"</code></li>
                </ul>
            </section>

            <section class="docs-stack docs-stack--tight" data-slot="resizable-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="resizable-state-rows">
                    <li><code>"idle / dragging / disabled"</code></li>
                    <li><code>"controlled / uncontrolled split state"</code></li>
                </ul>
            </section>

            <section class="docs-stack docs-stack--tight" data-slot="resizable-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>"Copy starter"</p>
                <ul data-slot="resizable-source-paths">
                    <li><code>"component-resizable"</code></li>
                    <li><code>"crates/ui-layout/src/resizable/view.rs"</code></li>
                </ul>
                <ul data-slot="resizable-source-prerequisites">
                    <li><code>"compose_copy_ready_code"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sidebar() -> AnyView {
    let showcase_code = Signal::derive(move || {
        r#"<Sidebar
  side=SidebarSide::Left
  variant=SidebarVariant::Sidebar
  collapsible=SidebarCollapsible::Offcanvas
  aria_label="Project navigation".to_string()
>
  <div class="ui-sidebar__header"><strong>"Workspace"</strong></div>
  <div class="ui-sidebar__content"><span>"Dashboard"</span><span>"Analytics"</span><span>"Settings"</span></div>
</Sidebar>"#
            .to_string()
    });

    let side_options = vec!["Left".to_string(), "Right".to_string()];
    let variant_options = vec![
        "Sidebar".to_string(),
        "Floating".to_string(),
        "Inset".to_string(),
    ];
    let collapsible_options = vec![
        "Offcanvas".to_string(),
        "Icon".to_string(),
        "None".to_string(),
    ];

    let (workbench_open_raw, set_workbench_open_raw) = signal(true);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);
    let (workbench_last_open, set_workbench_last_open) = signal(true);
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_last_open.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });

    let (workbench_side_index, set_workbench_side_index) = signal(Some(0_usize));
    let workbench_side = Signal::derive(move || match workbench_side_index.get().unwrap_or(0) {
        1 => SidebarSide::Right,
        _ => SidebarSide::Left,
    });
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => SidebarVariant::Floating,
            2 => SidebarVariant::Inset,
            _ => SidebarVariant::Sidebar,
        });
    let (workbench_collapsible_index, set_workbench_collapsible_index) = signal(Some(0_usize));
    let workbench_collapsible =
        Signal::derive(
            move || match workbench_collapsible_index.get().unwrap_or(0) {
                1 => SidebarCollapsible::Icon,
                2 => SidebarCollapsible::None,
                _ => SidebarCollapsible::Offcanvas,
            },
        );
    let (workbench_default_open, set_workbench_default_open) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_show_trigger, set_workbench_show_trigger) = signal(true);
    let (workbench_enable_shortcut, set_workbench_enable_shortcut) = signal(true);
    let (workbench_custom_shortcut, set_workbench_custom_shortcut) = signal(false);
    let (workbench_custom_trigger_label, set_workbench_custom_trigger_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        let side = match workbench_side.get() {
            SidebarSide::Right => "SidebarSide::Right",
            SidebarSide::Left => "SidebarSide::Left",
        };
        let variant = match workbench_variant.get() {
            SidebarVariant::Floating => "SidebarVariant::Floating",
            SidebarVariant::Inset => "SidebarVariant::Inset",
            SidebarVariant::Sidebar => "SidebarVariant::Sidebar",
        };
        let collapsible = match workbench_collapsible.get() {
            SidebarCollapsible::Icon => "SidebarCollapsible::Icon",
            SidebarCollapsible::None => "SidebarCollapsible::None",
            SidebarCollapsible::Offcanvas => "SidebarCollapsible::Offcanvas",
        };

        format!(
            "let (open_raw, set_open_raw) = signal({});\nlet open = Signal::derive(move || open_raw.get());\nlet on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));\n\n<Sidebar\n  open=open\n  default_open={}\n  on_open_change=on_open_change\n  side={side}\n  variant={variant}\n  collapsible={collapsible}\n  disabled={}\n  show_trigger={}\n  enable_shortcut={}\n  shortcut_key={}.to_string()\n  trigger_label={}.to_string()\n  aria_label=\"Project navigation sidebar\".to_string()\n  class_name={}\n/>",
            bool_word(workbench_open_raw.get()),
            bool_word(workbench_default_open.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_show_trigger.get()),
            bool_word(workbench_enable_shortcut.get()),
            rust_string_literal(if workbench_custom_shortcut.get() {
                "j"
            } else {
                "b"
            }),
            rust_string_literal(if workbench_custom_trigger_label.get() {
                "Toggle nav panel"
            } else {
                "Toggle sidebar"
            }),
            if workbench_custom_class.get() {
                "\"docs-sidebar-workbench\".to_string()"
            } else {
                "String::new()"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SidebarWorkbenchActualConfig {{\n  open: Some({}),\n  default_open: Some({}),\n  on_open_change: \"count={}, last={}\",\n  side: {:?},\n  variant: {:?},\n  collapsible: {:?},\n  disabled: {},\n  show_trigger: {},\n  enable_shortcut: {},\n  shortcut_key: Some({:?}),\n  trigger_label: Some({:?}),\n  aria_label: Some(\"Project navigation sidebar\"),\n  class_name: {:?},\n}}",
            bool_word(workbench_open_raw.get()),
            bool_word(workbench_default_open.get()),
            workbench_open_change_count.get(),
            bool_word(workbench_last_open.get()),
            workbench_side.get(),
            workbench_variant.get(),
            workbench_collapsible.get(),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_show_trigger.get()),
            bool_word(workbench_enable_shortcut.get()),
            if workbench_custom_shortcut.get() {
                "j"
            } else {
                "b"
            },
            if workbench_custom_trigger_label.get() {
                "Toggle nav panel"
            } else {
                "Toggle sidebar"
            },
            if workbench_custom_class.get() {
                Some("docs-sidebar-workbench")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Sidebar side=SidebarSide::Left variant=SidebarVariant::Sidebar collapsible=SidebarCollapsible::Offcanvas aria_label="Left default".to_string() />
<Sidebar side=SidebarSide::Right variant=SidebarVariant::Floating collapsible=SidebarCollapsible::Icon show_trigger=false aria_label="Right floating".to_string() />
<Sidebar side=SidebarSide::Left variant=SidebarVariant::Inset collapsible=SidebarCollapsible::None disabled=true enable_shortcut=false trigger_label="Disabled".to_string() aria_label="Disabled sidebar".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Sidebar"
            slug="sidebar"
            group="Layout"
            description="baseline-compatible sidebar primitive with controlled/uncontrolled open state, side+variant+collapsible contracts, keyboard shortcut toggle, and baseline-style data markers."
        >
            <Playground title="Hello World (Default Sidebar)" code_signal=showcase_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    aria_label="Project navigation".to_string()
                >
                    <div class="ui-sidebar__header">
                        <strong>"Workspace"</strong>
                    </div>
                    <div class="ui-sidebar__content">
                        <span>"Dashboard"</span>
                        <span>"Analytics"</span>
                        <span>"Settings"</span>
                    </div>
                </Sidebar>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sidebar-workbench-controls">
                        <SegmentedControl
                            id_base="docs-sidebar-workbench-side".to_string()
                            options=side_options.clone()
                            selected_index=workbench_side_index
                            set_selected_index=set_workbench_side_index
                            size=SegmentedControlSize::Sm
                            aria_label="Sidebar side".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-sidebar-workbench-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Sidebar variant".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-sidebar-workbench-collapsible".to_string()
                            options=collapsible_options.clone()
                            selected_index=workbench_collapsible_index
                            set_selected_index=set_workbench_collapsible_index
                            size=SegmentedControlSize::Sm
                            aria_label="Sidebar collapsible".to_string()
                        />
                        <Switch checked=workbench_default_open set_checked=set_workbench_default_open>
                            "default_open"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_show_trigger set_checked=set_workbench_show_trigger>
                            "show_trigger"
                        </Switch>
                        <Switch checked=workbench_enable_shortcut set_checked=set_workbench_enable_shortcut>
                            "enable_shortcut"
                        </Switch>
                        <Switch checked=workbench_custom_shortcut set_checked=set_workbench_custom_shortcut>
                            "custom shortcut_key"
                        </Switch>
                        <Switch checked=workbench_custom_trigger_label set_checked=set_workbench_custom_trigger_label>
                            "custom trigger_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_open_raw.update(|open| *open = !*open)
                        >
                            "Toggle open"
                        </button>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Sidebar
                        open=workbench_open
                        default_open=workbench_default_open.get()
                        on_open_change=on_workbench_open_change
                        side=workbench_side.get()
                        variant=workbench_variant.get()
                        collapsible=workbench_collapsible.get()
                        disabled=workbench_disabled.get()
                        show_trigger=workbench_show_trigger.get()
                        enable_shortcut=workbench_enable_shortcut.get()
                        shortcut_key=if workbench_custom_shortcut.get() {
                            "j".to_string()
                        } else {
                            "b".to_string()
                        }
                        trigger_label=if workbench_custom_trigger_label.get() {
                            "Toggle nav panel".to_string()
                        } else {
                            "Toggle sidebar".to_string()
                        }
                        aria_label="Project navigation sidebar".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-sidebar-workbench".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <div class="ui-sidebar__header">
                            <strong>"Workbench"</strong>
                        </div>
                        <div class="ui-sidebar__content">
                            <span>"Inbox"</span>
                            <span>"Projects"</span>
                            <span>"Reports"</span>
                        </div>
                    </Sidebar>
                    <span class="ui-muted">
                        "open: " {move || bool_word(workbench_open_raw.get())}
                        " · on_open_change count: " {move || workbench_open_change_count.get()}
                        " · last: " {move || bool_word(workbench_last_open.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Left / Right / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Sidebar
                        collapsible=SidebarCollapsible::Offcanvas
                        aria_label="Left default".to_string()
                    >
                        <div class="ui-sidebar__content"><span>"Default"</span></div>
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Right
                        variant=SidebarVariant::Floating
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Right floating".to_string()
                    >
                        <div class="ui-sidebar__content"><span>"Right/Floating"</span></div>
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::None
                        disabled=true
                        enable_shortcut=false
                        trigger_label="Disabled".to_string()
                        aria_label="Disabled sidebar".to_string()
                    >
                        <div class="ui-sidebar__content"><span>"Disabled"</span></div>
                    </Sidebar>
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

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_code = Signal::derive(move || {
        format!(
            "<SidebarHeader\n  disabled={}\n  aria_label={}\n  class_name={}\n>\n  <strong>\"Inspector\"</strong>\n  <span class=\"ui-muted\">\"Read-only mode\"</span>\n</SidebarHeader>",
            bool_word(workbench_disabled.get()),
            rust_string_literal(if workbench_custom_aria.get() {
                "Workbench inspector header"
            } else {
                ""
            }),
            rust_string_literal(if workbench_custom_class.get() {
                "docs-sidebar-header-custom"
            } else {
                ""
            }),
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SidebarHeaderActualConfig {{\n  disabled: {},\n  aria_label: {},\n  class_name: {},\n}}",
            bool_word(workbench_disabled.get()),
            if workbench_custom_aria.get() {
                "Some(\"Workbench inspector header\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-sidebar-header-custom\")"
            } else {
                "None"
            },
        )
    });

    view! {
        <ComponentPage
            title="SidebarHeader"
            slug="sidebar-header"
            group="Layout"
            description="baseline-compatible sidebar header region primitive with centralized disabled/source-state contracts and baseline-style data markers."
        >
            <Playground title="Hello World (Default Header Region)" code_signal=basic_code>
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

            <Playground
                title="Workbench (Disabled + Aria + Class)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " aria_label"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                    </div>
                }
            >
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Workbench header sidebar".to_string()
                >
                    <SidebarHeader
                        disabled=workbench_disabled.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Workbench inspector header".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-sidebar-header-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <strong>"Inspector"</strong>
                        <span class="ui-muted">"Read-only mode"</span>
                    </SidebarHeader>
                </Sidebar>
            </Playground>

            <Playground title="State Matrix (Disabled + Custom Class)" code_signal=disabled_code>
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
    let items = vec![
        SidebarMenuItem {
            id: "workspace".to_string(),
            label: "Workspace".to_string(),
            href: None,
            badge: Some("6".to_string()),
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
            badge: Some("2".to_string()),
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
    let showcase_items = items.clone();
    let workbench_items = items.clone();
    let matrix_items_first = items.clone();
    let matrix_items_second = items.clone();
    let matrix_items_third = items;

    let (active_id_raw, set_active_id_raw) = signal(Some("tokens".to_string()));
    let active_id: Signal<Option<String>> = Signal::derive(move || active_id_raw.get());
    let (on_active_id_change_runs, set_on_active_id_change_runs) = signal(0_u32);
    let on_active_id_change = Callback::new(move |next: Option<String>| {
        set_active_id_raw.set(next);
        set_on_active_id_change_runs.update(|count| *count += 1);
    });

    let (last_action, set_last_action) = signal("none".to_string());
    let (on_action_runs, set_on_action_runs) = signal(0_u32);
    let on_action = Callback::new(move |id: String| {
        set_last_action.set(id);
        set_on_action_runs.update(|count| *count += 1);
    });

    let (last_item_action, set_last_item_action) = signal("none".to_string());
    let (on_item_action_runs, set_on_item_action_runs) = signal(0_u32);
    let on_item_action = Callback::new(move |id: String| {
        set_last_item_action.set(id);
        set_on_item_action_runs.update(|count| *count += 1);
    });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_show_badges, set_workbench_show_badges) = signal(true);
    let (workbench_show_actions, set_workbench_show_actions) = signal(true);
    let (workbench_allow_submenu_collapse, set_workbench_allow_submenu_collapse) = signal(true);
    let (workbench_enable_shortcut, set_workbench_enable_shortcut) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<SidebarMenu
  items=vec![SidebarMenuItem::new("workspace", "Workspace"), SidebarMenuItem::new("releases", "Releases")]
  id_base="docs-sidebar-menu-hello".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            "docs-sidebar-menu-custom"
        } else {
            ""
        };
        let motion = "SidebarMenuMotion::default()";
        [
            "<SidebarMenu".to_string(),
            "  items=vec![SidebarMenuItem::new(\"workspace\", \"Workspace\"), SidebarMenuItem::new(\"releases\", \"Releases\")]".to_string(),
            "  id_base=\"docs-sidebar-menu-workbench\".to_string()".to_string(),
            "  active_id=active_id".to_string(),
            "  default_active_id=\"tokens\".to_string()".to_string(),
            "  on_active_id_change=on_active_id_change".to_string(),
            "  on_action=on_action".to_string(),
            "  on_item_action=on_item_action".to_string(),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            format!("  show_badges={}", bool_word(workbench_show_badges.get())),
            format!("  show_actions={}", bool_word(workbench_show_actions.get())),
            format!(
                "  allow_submenu_collapse={}",
                bool_word(workbench_allow_submenu_collapse.get())
            ),
            format!(
                "  enable_keyboard_shortcut={}",
                bool_word(workbench_enable_shortcut.get())
            ),
            "  keyboard_shortcut_key=\"k\".to_string()".to_string(),
            format!("  motion={motion}"),
            "  aria_label=\"Workspace menu\".to_string()".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            Some("docs-sidebar-menu-custom")
        } else {
            None
        };
        let motion = SidebarMenuMotion::default();

        format!(
            "SidebarMenuActualConfig {{\n  items: \"sample_items(len=2)\",\n  id_base: Some(\"docs-sidebar-menu-workbench\"),\n  active_id: {:?},\n  default_active_id: Some(\"tokens\"),\n  on_active_id_change: \"runs={}\",\n  on_action: \"runs={},last={:?}\",\n  on_item_action: \"runs={},last={:?}\",\n  disabled: {},\n  show_badges: {},\n  show_actions: {},\n  allow_submenu_collapse: {},\n  enable_keyboard_shortcut: {},\n  keyboard_shortcut_key: Some(\"k\"),\n  motion: {motion:?},\n  aria_label: Some(\"Workspace menu\"),\n  class_name: {class_name:?},\n}}",
            active_id_raw.get(),
            on_active_id_change_runs.get(),
            on_action_runs.get(),
            last_action.get(),
            on_item_action_runs.get(),
            last_item_action.get(),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_show_badges.get()),
            bool_word(workbench_show_actions.get()),
            bool_word(workbench_allow_submenu_collapse.get()),
            bool_word(workbench_enable_shortcut.get()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SidebarMenu items=vec![SidebarMenuItem::new("workspace", "Workspace"), SidebarMenuItem::new("releases", "Releases")] id_base="menu-default".to_string() default_active_id="tokens".to_string() />
<SidebarMenu items=vec![SidebarMenuItem::new("workspace", "Workspace"), SidebarMenuItem::new("releases", "Releases")] id_base="menu-badges-off".to_string() show_badges=false show_actions=true allow_submenu_collapse=true />
<SidebarMenu items=vec![SidebarMenuItem::new("workspace", "Workspace"), SidebarMenuItem::new("releases", "Releases")] id_base="menu-disabled".to_string() disabled=true enable_keyboard_shortcut=false motion=SidebarMenuMotion::default() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="SidebarMenu"
            slug="sidebar-menu"
            group="Layout"
            description="SidebarMenu playground with full API workbench and callback feedback."
        >
            <Playground title="Hello World (Default Sidebar Menu)" code_signal=hello_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar menu hello".to_string()
                >
                    <SidebarMenu
                        items=showcase_items
                        id_base="docs-sidebar-menu-hello".to_string()
                    />
                </Sidebar>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sidebar-menu-workbench-controls">
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_show_badges set_checked=set_workbench_show_badges>
                            "show_badges"
                        </Switch>
                        <Switch checked=workbench_show_actions set_checked=set_workbench_show_actions>
                            "show_actions"
                        </Switch>
                        <Switch
                            checked=workbench_allow_submenu_collapse
                            set_checked=set_workbench_allow_submenu_collapse
                        >
                            "allow_submenu_collapse"
                        </Switch>
                        <Switch checked=workbench_enable_shortcut set_checked=set_workbench_enable_shortcut>
                            "enable_keyboard_shortcut"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="sidebar-menu-workbench-preview">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar menu workbench".to_string()
                    >
                        <SidebarMenu
                            items=workbench_items
                            id_base="docs-sidebar-menu-workbench".to_string()
                            active_id=active_id
                            default_active_id="tokens".to_string()
                            on_active_id_change=on_active_id_change
                            on_action=on_action
                            on_item_action=on_item_action
                            disabled=workbench_disabled.get()
                            show_badges=workbench_show_badges.get()
                            show_actions=workbench_show_actions.get()
                            allow_submenu_collapse=workbench_allow_submenu_collapse.get()
                            enable_keyboard_shortcut=workbench_enable_shortcut.get()
                            keyboard_shortcut_key="k".to_string()
                            motion=SidebarMenuMotion::default()
                            aria_label="Workspace menu".to_string()
                            class_name=if workbench_custom_class.get() {
                                "docs-sidebar-menu-custom".to_string()
                            } else {
                                String::new()
                            }
                        />
                    </Sidebar>
                    <span class="ui-muted" data-slot="sidebar-menu-workbench-feedback">
                        "active_id: "
                        {move || active_id_raw.get().unwrap_or_else(|| "none".to_string())}
                        " · on_active_id_change: " {move || on_active_id_change_runs.get()}
                        " · on_action: " {move || on_action_runs.get()}
                        " · on_item_action: " {move || on_item_action_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Badges Off / Disabled)"
                code_signal=matrix_code
            >
                <div class="docs-row" data-slot="sidebar-menu-state-matrix">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar default".to_string()
                    >
                        <SidebarMenu
                            items=matrix_items_first
                            id_base="docs-sidebar-menu-matrix-default".to_string()
                            default_active_id="tokens".to_string()
                        />
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar no badges".to_string()
                    >
                        <SidebarMenu
                            items=matrix_items_second
                            id_base="docs-sidebar-menu-matrix-badges-off".to_string()
                            show_badges=false
                            show_actions=true
                            allow_submenu_collapse=true
                        />
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar disabled".to_string()
                    >
                        <SidebarMenu
                            items=matrix_items_third
                            id_base="docs-sidebar-menu-matrix-disabled".to_string()
                            disabled=true
                            enable_keyboard_shortcut=false
                            motion=SidebarMenuMotion::default()
                        />
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
