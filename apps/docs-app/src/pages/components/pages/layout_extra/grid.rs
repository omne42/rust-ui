use super::*;

pub(crate) fn grid() -> AnyView {
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
