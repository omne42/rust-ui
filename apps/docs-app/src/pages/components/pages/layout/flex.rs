use super::*;

pub(crate) fn flex() -> AnyView {
    let matrix_code = Signal::derive(move || {
        r#"<Flex direction=FlexDirection::Row wrap=FlexWrap::Wrap gap=FlexGap::Md align=FlexAlign::Center>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"Alpha"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"Beta"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"Gamma"</View>
</Flex>
<Flex direction=FlexDirection::Column gap=FlexGap::Sm align=FlexAlign::Stretch>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"Line 1"</View>
  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>"Line 2"</View>
</Flex>"#.to_string()
    });

    let inline_code = Signal::derive(move || {
        r#"<Flex
  inline=true
  justify=FlexJustify::SpaceBetween
  align=FlexAlign::Baseline
  gap=FlexGap::Lg
  class_name="docs-flex-inline".to_string()
>
  <Heading level=HeadingLevel::H5>"Inline Flex"</Heading>
  <Content tone=ContentTone::Muted>"Baseline aligned helper text."</Content>
  <Footer tone=FooterTone::Muted>"Updated now"</Footer>
</Flex>"#
            .to_string()
    });
    let (interactive_column, set_interactive_column) = signal(false);
    let (interactive_wrap, set_interactive_wrap) = signal(true);
    let (interactive_inline, set_interactive_inline) = signal(false);
    let (interactive_spread, set_interactive_spread) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let interactive_code = Signal::derive(move || {
        let direction = if interactive_column.get() {
            FlexDirection::Column
        } else {
            FlexDirection::Row
        };
        let wrap = if interactive_wrap.get() {
            FlexWrap::Wrap
        } else {
            FlexWrap::NoWrap
        };
        let justify = if interactive_spread.get() {
            FlexJustify::SpaceBetween
        } else {
            FlexJustify::Start
        };
        let align = if interactive_spread.get() {
            FlexAlign::Baseline
        } else {
            FlexAlign::Stretch
        };
        let gap = if interactive_spread.get() {
            FlexGap::Lg
        } else {
            FlexGap::Sm
        };
        let inline = interactive_inline.get();
        let custom_class = interactive_custom_class.get();

        let mut snippet = vec![
            "<Flex".to_string(),
            format!("  direction=FlexDirection::{direction:?}"),
            format!("  wrap=FlexWrap::{wrap:?}"),
            format!("  justify=FlexJustify::{justify:?}"),
            format!("  align=FlexAlign::{align:?}"),
            format!("  gap=FlexGap::{gap:?}"),
        ];
        if inline {
            snippet.push("  inline=true".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-flex-workbench\".into()".to_string());
        }
        snippet.extend([
            ">".to_string(),
            "  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>\"Alpha\"</View>".to_string(),
            "  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>\"Beta\"</View>".to_string(),
            "  <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>\"Gamma\"</View>".to_string(),
            "</Flex>".to_string(),
        ]);
        snippet.join("\n")
    });
    let interactive_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/flex/styles.rs */\n{}",
            ui_layout::flex::styles::CSS
        )
    });
    let interactive_actual_config = Signal::derive(move || {
        let direction = if interactive_column.get() {
            FlexDirection::Column
        } else {
            FlexDirection::Row
        };
        let wrap = if interactive_wrap.get() {
            FlexWrap::Wrap
        } else {
            FlexWrap::NoWrap
        };
        let justify = if interactive_spread.get() {
            FlexJustify::SpaceBetween
        } else {
            FlexJustify::Start
        };
        let align = if interactive_spread.get() {
            FlexAlign::Baseline
        } else {
            FlexAlign::Stretch
        };
        let gap = if interactive_spread.get() {
            FlexGap::Lg
        } else {
            FlexGap::Sm
        };
        let inline = interactive_inline.get();
        let custom_class = interactive_custom_class.get();

        format!(
            "FlexActualConfig {{\n  direction: FlexDirection::{direction:?},\n  wrap: FlexWrap::{wrap:?},\n  justify: FlexJustify::{justify:?},\n  align: FlexAlign::{align:?},\n  gap: FlexGap::{gap:?},\n  inline: {inline},\n  motion: FlexMotion::default(),\n  aria_label: Some(\"Flex interactive current\"),\n  class_name: {},\n}}",
            if custom_class {
                "\"docs-flex-workbench\""
            } else {
                "\"<none>\""
            }
        )
    });

    view! {
        <ComponentPage
            title="Flex"
            slug="flex"
            group="Layout"
            description="baseline-style flex layout primitive with centralized direction/wrap/alignment/gap normalization and stable data-state contracts."
        >
            <Playground title="Direction + Wrap + Gap" code_signal=matrix_code>
                <div class="docs-stack">
                    <Flex
                        direction=FlexDirection::Row
                        wrap=FlexWrap::Wrap
                        gap=FlexGap::Md
                        align=FlexAlign::Center
                        aria_label="Tag cloud layout".to_string()
                    >
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Alpha"
                        </View>
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Beta"
                        </View>
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Gamma"
                        </View>
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Delta"
                        </View>
                    </Flex>

                    <Flex
                        direction=FlexDirection::Column
                        gap=FlexGap::Sm
                        align=FlexAlign::Stretch
                        class_name="docs-flex-column".to_string()
                    >
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Line 1"
                        </View>
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Line 2"
                        </View>
                    </Flex>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display / Config / Code / CSS Test)"
                code_signal=interactive_code
                test_css_source=interactive_test_css_source
                test_source_path="crates/ui-layout/src/flex/styles.rs".to_string()
                test_config_signal=interactive_actual_config
                description="展示区用于当前配置与 baseline 对比；Config/Code/CSS Test 区用于调参与契约验证。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="flex-config-controls">
                        <button
                            type="button"
                            on:click=move |_| set_interactive_column.update(|value| *value = !*value)
                        >
                            "Toggle direction (row/column)"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_wrap.update(|value| *value = !*value)
                        >
                            "Toggle wrap"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_inline.update(|value| *value = !*value)
                        >
                            "Toggle inline"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_spread.update(|value| *value = !*value)
                        >
                            "Toggle distribution (start/stretch vs between/baseline)"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_custom_class.update(|value| *value = !*value)
                        >
                            "Toggle custom class"
                        </button>
                        <p class="ui-muted" data-slot="flex-config-summary">
                            {move || {
                                format!(
                                    "config: direction={} wrap={} inline={} spread={} custom_class={}",
                                    if interactive_column.get() { "column" } else { "row" },
                                    if interactive_wrap.get() { "wrap" } else { "nowrap" },
                                    interactive_inline.get(),
                                    interactive_spread.get(),
                                    interactive_custom_class.get()
                                )
                            }}
                        </p>
                    </div>
                }
            >
                {move || {
                    let direction = if interactive_column.get() {
                        FlexDirection::Column
                    } else {
                        FlexDirection::Row
                    };
                    let wrap = if interactive_wrap.get() {
                        FlexWrap::Wrap
                    } else {
                        FlexWrap::NoWrap
                    };
                    let justify = if interactive_spread.get() {
                        FlexJustify::SpaceBetween
                    } else {
                        FlexJustify::Start
                    };
                    let align = if interactive_spread.get() {
                        FlexAlign::Baseline
                    } else {
                        FlexAlign::Stretch
                    };
                    let gap = if interactive_spread.get() {
                        FlexGap::Lg
                    } else {
                        FlexGap::Sm
                    };
                    let inline = interactive_inline.get();
                    let custom_class = if interactive_custom_class.get() {
                        "docs-flex-workbench".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack" data-slot="flex-workbench-display">
                            <span class="ui-muted">
                                "display: current config vs baseline"
                            </span>
                            <div class="docs-row">
                                <View border=ViewBorder::Subtle radius=ViewRadius::Md padding=ViewPadding::Sm>
                                    <div class="ui-muted">"Current"</div>
                                    <Flex
                                        direction=direction
                                        wrap=wrap
                                        justify=justify
                                        align=align
                                        gap=gap
                                        inline=inline
                                        motion=FlexMotion::default()
                                        class_name=custom_class
                                        aria_label="Flex interactive current".to_string()
                                    >
                                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                                            "Alpha"
                                        </View>
                                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                                            "Beta"
                                        </View>
                                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                                            "Gamma"
                                        </View>
                                    </Flex>
                                </View>

                                <View border=ViewBorder::Subtle radius=ViewRadius::Md padding=ViewPadding::Sm>
                                    <div class="ui-muted">"Baseline"</div>
                                    <Flex
                                        direction=FlexDirection::Row
                                        wrap=FlexWrap::Wrap
                                        justify=FlexJustify::Start
                                        align=FlexAlign::Stretch
                                        gap=FlexGap::Sm
                                        motion=FlexMotion::default()
                                        aria_label="Flex interactive baseline".to_string()
                                    >
                                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                                            "Alpha"
                                        </View>
                                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                                            "Beta"
                                        </View>
                                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                                            "Gamma"
                                        </View>
                                    </Flex>
                                </View>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Direction / Distribution / Inline Comparison)"
                code_signal=inline_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Flex
                        direction=FlexDirection::Row
                        wrap=FlexWrap::Wrap
                        gap=FlexGap::Sm
                        motion=FlexMotion::default()
                        aria_label="Row matrix".to_string()
                    >
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Row"
                        </View>
                        <View border=ViewBorder::Subtle padding=ViewPadding::Sm radius=ViewRadius::Sm>
                            "Wrap"
                        </View>
                    </Flex>
                    <Flex
                        direction=FlexDirection::Column
                        justify=FlexJustify::SpaceBetween
                        align=FlexAlign::Baseline
                        gap=FlexGap::Lg
                        inline=true
                        motion=FlexMotion::default()
                        aria_label="Inline matrix".to_string()
                        class_name="docs-flex-inline".to_string()
                    >
                        <span>"Inline"</span>
                        <span>"Baseline"</span>
                    </Flex>
                </div>
            </Playground>

            <Playground title="Inline + Distribution" code_signal=inline_code>
                <div class="docs-stack">
                    <Flex
                        inline=true
                        justify=FlexJustify::SpaceBetween
                        align=FlexAlign::Baseline
                        gap=FlexGap::Lg
                        class_name="docs-flex-inline".to_string()
                    >
                        <Heading level=HeadingLevel::H5 tone=HeadingTone::Strong>
                            "Inline Flex"
                        </Heading>
                        <Content tone=ContentTone::Muted>
                            "Baseline aligned helper text."
                        </Content>
                        <Footer tone=FooterTone::Muted bordered=true>
                            "Updated now"
                        </Footer>
                    </Flex>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
