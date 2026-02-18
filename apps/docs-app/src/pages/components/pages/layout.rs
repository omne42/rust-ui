use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AutoHeight, AutoHeightMotion, Card, CardVariant, Content, ContentTone, Divider,
    DividerOrientation, Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexWrap, Footer,
    FooterTone, Header, HeaderTone, Heading, HeadingLevel, HeadingTone, ScrollShadow, Separator,
    SeparatorElementType, SeparatorOrientation, Snippet, Spacer, SpacerAxis, SpacerSize, View,
    ViewBackground, ViewBorder, ViewElement, ViewPadding, ViewRadius, ViewShadow, Well,
    WellDensity, WellTone,
};

pub(super) fn card() -> AnyView {
    let (workbench_variant_key, set_workbench_variant_key) = signal("default".to_string());
    let (workbench_padded, set_workbench_padded) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_variant = Signal::derive(move || match workbench_variant_key.get().as_str() {
        "muted" => CardVariant::Muted,
        "outline" => CardVariant::Outline,
        _ => CardVariant::Default,
    });

    let workbench_code = Signal::derive(move || {
        let variant_key = workbench_variant_key.get();
        let padded = workbench_padded.get();
        let custom_class = workbench_custom_class.get();

        let variant_line = match variant_key.as_str() {
            "muted" => "  variant=CardVariant::Muted\n",
            "outline" => "  variant=CardVariant::Outline\n",
            _ => "",
        };
        let padded_line = if padded { "" } else { "  padded=false\n" };
        let class_line = if custom_class {
            "  class_name=\"docs-card-custom\".to_string()\n"
        } else {
            ""
        };

        format!(
            "<Card\n{variant_line}{padded_line}{class_line}>\n  <div>\"Workbench content\"</div>\n</Card>"
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/card/styles.rs */\n{}",
            ui_components::card::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant = workbench_variant.get();
        let padded = workbench_padded.get();
        let custom_class = workbench_custom_class.get();

        let mut classes = vec![
            "ui-card".to_string(),
            variant.class_name().to_string(),
            if padded {
                "ui-card--padded".to_string()
            } else {
                "ui-card--no-padding".to_string()
            },
        ];
        if custom_class {
            classes.push("docs-card-custom".to_string());
        }

        format!(
            "CardActualConfig {{\n  variant: {variant:?},\n  padded: {padded},\n  custom_class: {custom_class},\n  data_variant: \"{}\",\n  data_state: \"{}\",\n  class: \"{}\",\n}}",
            variant.as_str(),
            if padded { "padded" } else { "flush" },
            classes.join(" "),
        )
    });

    let variants_code = Signal::derive(move || {
        r#"<Card variant=CardVariant::Default>"Default"</Card>
<Card variant=CardVariant::Muted>"Muted"</Card>
<Card variant=CardVariant::Outline>"Outline"</Card>"#
            .to_string()
    });

    let padding_code = Signal::derive(move || {
        r#"<Card padded=true>
  <div>"Padded content"</div>
</Card>
<Card padded=false>
  <div>"Flush content"</div>
</Card>"#
            .to_string()
    });

    let custom_class_code = Signal::derive(move || {
        r#"<Card class_name="docs-card-custom".to_string()>
  <div>"Custom class marker"</div>
</Card>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Card"
            slug="card"
            group="Layout"
            description="A token-styled surface with centralized variant/padding state attrs."
        >
            <Playground title="Variants" code_signal=variants_code>
                <div class="docs-row">
                    <Card variant=CardVariant::Default>
                        <div class="docs-stack">
                            <div>"Default"</div>
                            <div class="ui-muted">"Uses tokens for bg/border/shadow."</div>
                        </div>
                    </Card>
                    <Card variant=CardVariant::Muted>
                        <div class="docs-stack">
                            <div>"Muted"</div>
                            <div class="ui-muted">"Lower-contrast surface."</div>
                        </div>
                    </Card>
                    <Card variant=CardVariant::Outline>
                        <div class="docs-stack">
                            <div>"Outline"</div>
                            <div class="ui-muted">"Border-forward style."</div>
                        </div>
                    </Card>
                </div>
            </Playground>

            <Playground title="Padding States" code_signal=padding_code>
                <div class="docs-row">
                    <Card padded=true>
                        <div class="docs-stack">
                            <div>"Padded"</div>
                            <div class="ui-muted">"Default spacing"</div>
                        </div>
                    </Card>
                    <Card padded=false>
                        <div class="docs-stack">
                            <div class="docs-row docs-row--tight">
                                <span>"Flush"</span>
                                <span class="ui-muted">"No internal padding"</span>
                            </div>
                        </div>
                    </Card>
                </div>
            </Playground>

            <Playground title="Custom Class" code_signal=custom_class_code>
                <div class="docs-row">
                    <Card class_name="docs-card-custom".to_string()>
                        <div class="docs-stack">
                            <div>"Custom class marker"</div>
                            <div class="ui-muted">"Verifies `data-custom-class` + class merge."</div>
                        </div>
                    </Card>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for variant, padding, and class-source contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/card/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="card-workbench-controls">
                        <label class="docs-search__label">
                            "Variant"
                            <select
                                prop:value=move || workbench_variant_key.get()
                                on:change=move |ev| set_workbench_variant_key.set(event_target_value(&ev))
                            >
                                <option value="default">"Default"</option>
                                <option value="muted">"Muted"</option>
                                <option value="outline">"Outline"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_padded.get()
                                on:change=move |ev| set_workbench_padded.set(event_target_checked(&ev))
                            />
                            " Padded"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| {
                                    set_workbench_custom_class.set(event_target_checked(&ev))
                                }
                            />
                            " Custom class"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="card-workbench-preview">
                    {move || {
                        let variant = workbench_variant.get();
                        let padded = workbench_padded.get();
                        if workbench_custom_class.get() {
                            view! {
                                <Card
                                    variant=variant
                                    padded=padded
                                    class_name="docs-card-custom".to_string()
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"Workbench card"</div>
                                        <div class="ui-muted">
                                            "Compare variant/padding/class-source markers in one canvas."
                                        </div>
                                    </div>
                                </Card>
                            }
                            .into_any()
                        } else {
                            view! {
                                <Card variant=variant padded=padded>
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"Workbench card"</div>
                                        <div class="ui-muted">
                                            "Compare variant/padding/class-source markers in one canvas."
                                        </div>
                                    </div>
                                </Card>
                            }
                            .into_any()
                        }
                    }}
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn view() -> AnyView {
    let surface_code = Signal::derive(move || {
        r#"<View border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md>
  <div>"Subtle surface"</div>
</View>
<View
  background=ViewBackground::Accent
  border=ViewBorder::Strong
  padding=ViewPadding::Lg
  radius=ViewRadius::Lg
  shadow=ViewShadow::Md
>
  <div>"Accent emphasis surface"</div>
</View>"#
            .to_string()
    });

    let element_code = Signal::derive(move || {
        r#"<View
  element=ViewElement::Section
  background=ViewBackground::Subtle
  border=ViewBorder::Subtle
  padding=ViewPadding::Sm
  radius=ViewRadius::Sm
  fluid=true
  class_name="docs-view-custom".to_string()
  aria_label="Release notes".to_string()
>
  <div>"Section container"</div>
</View>
<View element=ViewElement::Span padding=ViewPadding::Sm border=ViewBorder::Subtle>
  <span>"Inline view"</span>
</View>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="View"
            slug="view"
            group="Layout"
            description="General-purpose baseline-style container with centralized surface token state and stable data markers."
        >
            <Playground title="Surface Tokens" code_signal=surface_code>
                <div class="docs-stack">
                    <View border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Subtle surface"</strong>
                            <span class="ui-muted">"Border + radius + padding from tokenized state attrs."</span>
                        </div>
                    </View>

                    <View
                        background=ViewBackground::Accent
                        border=ViewBorder::Strong
                        padding=ViewPadding::Lg
                        radius=ViewRadius::Lg
                        shadow=ViewShadow::Md
                    >
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Accent emphasis"</strong>
                            <span class="ui-muted">"Accent background with strong border and stronger elevation."</span>
                        </div>
                    </View>
                </div>
            </Playground>

            <Playground title="Element + Fluid + Custom Class" code_signal=element_code>
                <div class="docs-stack">
                    <View
                        element=ViewElement::Section
                        background=ViewBackground::Subtle
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Sm
                        radius=ViewRadius::Sm
                        fluid=true
                        class_name="docs-view-custom".to_string()
                        aria_label="Release notes".to_string()
                    >
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Section container"</strong>
                            <span class="ui-muted">"Verifies section element + fluid width + custom class marker."</span>
                        </div>
                    </View>

                    <View
                        element=ViewElement::Span
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Sm
                        radius=ViewRadius::Sm
                    >
                        <span>"Inline view"</span>
                    </View>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn flex() -> AnyView {
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
            snippet.push("  class_name=\"docs-flex-workbench\".to_string()".to_string());
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
            "/* crates/ui-components/src/flex/styles.rs */\n{}",
            ui_components::flex::styles::CSS
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
            "FlexActualConfig {{\n  direction: FlexDirection::{direction:?},\n  wrap: FlexWrap::{wrap:?},\n  justify: FlexJustify::{justify:?},\n  align: FlexAlign::{align:?},\n  gap: FlexGap::{gap:?},\n  inline: {inline},\n  class_name: {},\n}}",
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

            <Playground
                title="Interactive Playground (Display / Config / Code / CSS Test)"
                code_signal=interactive_code
                test_css_source=interactive_test_css_source
                test_source_path="crates/ui-components/src/flex/styles.rs".to_string()
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
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn content() -> AnyView {
    let semantic_code = Signal::derive(move || {
        r#"<Content>
  <p>"Primary body content"</p>
</Content>
<Content tone=ContentTone::Muted>
  <p>"Secondary muted content"</p>
</Content>"#
            .to_string()
    });

    let padded_code = Signal::derive(move || {
        r#"<View border=ViewBorder::Subtle radius=ViewRadius::Md>
  <Content
    padded=true
    aria_label="Dialog content".to_string()
    class_name="docs-content-custom".to_string()
  >
    <p>"Padded container content"</p>
  </Content>
</View>"#
            .to_string()
    });
    let (interactive_muted, set_interactive_muted) = signal(false);
    let (interactive_padded, set_interactive_padded) = signal(false);
    let (interactive_custom_aria, set_interactive_custom_aria) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let interactive_code = Signal::derive(move || {
        let muted = interactive_muted.get();
        let padded = interactive_padded.get();
        let custom_aria = interactive_custom_aria.get();
        let custom_class = interactive_custom_class.get();

        let mut snippet = vec!["<Content".to_string()];
        if muted {
            snippet.push("  tone=ContentTone::Muted".to_string());
        }
        if padded {
            snippet.push("  padded=true".to_string());
        }
        if custom_aria {
            snippet.push("  aria_label=\"Docs content area\".to_string()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-content-workbench\".to_string()".to_string());
        }
        snippet.extend([
            ">".to_string(),
            "  <p>\"Interactive content region\"</p>".to_string(),
            "</Content>".to_string(),
        ]);
        snippet.join("\n")
    });
    let interactive_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/content/styles.rs */\n{}",
            ui_components::content::styles::CSS
        )
    });
    let interactive_actual_config = Signal::derive(move || {
        let tone = if interactive_muted.get() {
            ContentTone::Muted
        } else {
            ContentTone::Default
        };
        let padded = interactive_padded.get();
        let custom_aria = interactive_custom_aria.get();
        let custom_class = interactive_custom_class.get();
        let state = if padded && tone == ContentTone::Muted {
            "muted-padded"
        } else if padded {
            "padded"
        } else if tone == ContentTone::Muted {
            "muted"
        } else {
            "default"
        };

        format!(
            "ContentActualConfig {{\n  tone: ContentTone::{tone:?},\n  padded: {padded},\n  aria_source: {},\n  class_source: {},\n  data_state: \"{state}\",\n}}",
            if custom_aria {
                "\"custom\""
            } else {
                "\"default\""
            },
            if custom_class {
                "\"custom\""
            } else {
                "\"default\""
            }
        )
    });

    view! {
        <ComponentPage
            title="Content"
            slug="content"
            group="Layout"
            description="Semantic primary-content region (`<section>`) with centralized tone/padding/source state contracts."
        >
            <Playground title="Semantic Section + Tone" code_signal=semantic_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <div class="docs-stack">
                        <Content>
                            <p>"Primary body content for a container region."</p>
                        </Content>
                        <Content tone=ContentTone::Muted>
                            <p>"Secondary muted notes that still stay in the same semantic content slot."</p>
                        </Content>
                    </div>
                </View>
            </Playground>

            <Playground title="Padded + Custom Aria/Class" code_signal=padded_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <Content
                        padded=true
                        aria_label="Dialog content".to_string()
                        class_name="docs-content-custom".to_string()
                    >
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Padded content region"</strong>
                            <span class="ui-muted">"Verifies padding marker + custom class source contract."</span>
                        </div>
                    </Content>
                </View>
            </Playground>

            <Playground
                title="Interactive Playground (Display / Config / Code / CSS Test)"
                code_signal=interactive_code
                test_css_source=interactive_test_css_source
                test_source_path="crates/ui-components/src/content/styles.rs".to_string()
                test_config_signal=interactive_actual_config
                description="展示区用于当前配置与 baseline 对比；Config/Code/CSS Test 区用于快速验证语义与样式契约。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="content-config-controls">
                        <button
                            type="button"
                            on:click=move |_| set_interactive_muted.update(|value| *value = !*value)
                        >
                            "Toggle tone (default/muted)"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_padded.update(|value| *value = !*value)
                        >
                            "Toggle padded"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_custom_aria.update(|value| *value = !*value)
                        >
                            "Toggle custom aria label"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_custom_class.update(|value| *value = !*value)
                        >
                            "Toggle custom class"
                        </button>
                        <p class="ui-muted" data-slot="content-config-summary">
                            {move || {
                                format!(
                                    "config: tone={} padded={} custom_aria={} custom_class={}",
                                    if interactive_muted.get() { "muted" } else { "default" },
                                    interactive_padded.get(),
                                    interactive_custom_aria.get(),
                                    interactive_custom_class.get()
                                )
                            }}
                        </p>
                    </div>
                }
            >
                {move || {
                    let tone = if interactive_muted.get() {
                        ContentTone::Muted
                    } else {
                        ContentTone::Default
                    };
                    let padded = interactive_padded.get();
                    let aria_label = if interactive_custom_aria.get() {
                        "Docs content area".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if interactive_custom_class.get() {
                        "docs-content-workbench".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack" data-slot="content-workbench-display">
                            <span class="ui-muted">"display: current config vs baseline"</span>
                            <div class="docs-row">
                                <View border=ViewBorder::Subtle radius=ViewRadius::Md padding=ViewPadding::Sm>
                                    <div class="ui-muted">"Current"</div>
                                    <Content
                                        tone=tone
                                        padded=padded
                                        aria_label=aria_label
                                        class_name=class_name
                                    >
                                        <p>"Interactive content region."</p>
                                    </Content>
                                </View>

                                <View border=ViewBorder::Subtle radius=ViewRadius::Md padding=ViewPadding::Sm>
                                    <div class="ui-muted">"Baseline"</div>
                                    <Content>
                                        <p>"Baseline content region."</p>
                                    </Content>
                                </View>
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn header() -> AnyView {
    let (interactive_strong_tone, set_interactive_strong_tone) = signal(false);
    let (interactive_bordered, set_interactive_bordered) = signal(false);

    let semantic_code = Signal::derive(move || {
        r#"<Header>
  <h3>"Dialog title"</h3>
</Header>
<Header tone=HeaderTone::Strong>
  <h3>"Strong header"</h3>
</Header>"#
            .to_string()
    });

    let bordered_code = Signal::derive(move || {
        r#"<View border=ViewBorder::Subtle radius=ViewRadius::Md>
  <Header
    tone=HeaderTone::Strong
    bordered=true
    aria_label="Settings header".to_string()
    class_name="docs-header-custom".to_string()
  >
    <h3>"Settings"</h3>
  </Header>
  <Content padded=true>
    <p>"Header above content, matching baseline container semantics."</p>
  </Content>
</View>"#
            .to_string()
    });

    let interactive_code = Signal::derive(move || {
        r#"let (strong_tone, set_strong_tone) = signal(false);
let (bordered, set_bordered) = signal(false);

<Header
  tone=if strong_tone.get() { HeaderTone::Strong } else { HeaderTone::Default }
  bordered=bordered.get()
>
  <h3>"Interactive header"</h3>
</Header>"#
            .to_string()
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/header/styles.rs */\n{}",
            ui_components::header::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let strong_tone = interactive_strong_tone.get();
        let bordered = interactive_bordered.get();

        let mut classes = vec![
            "ui-header".to_string(),
            if strong_tone {
                "ui-header--tone-strong".to_string()
            } else {
                "ui-header--tone-default".to_string()
            },
        ];
        if bordered {
            classes.push("ui-header--bordered".to_string());
        }
        classes.push("ui-header--custom-class".to_string());
        classes.push("docs-header-interactive".to_string());

        format!(
            "HeaderActualConfig {{\n  tone: {},\n  bordered: {},\n  aria_label: \"Interactive docs header\",\n  class_name: \"docs-header-interactive\",\n  class: \"{}\",\n}}",
            if strong_tone { "Strong" } else { "Default" },
            bordered,
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Header"
            slug="header"
            group="Layout"
            description="Semantic container header (`<header>`) with centralized tone/border/source state contracts."
        >
            <Playground title="Semantic Header + Tone" code_signal=semantic_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <div class="docs-stack">
                        <Header>
                            <h3>"Dialog title"</h3>
                        </Header>
                        <Header tone=HeaderTone::Strong>
                            <h3>"Strong header"</h3>
                        </Header>
                    </div>
                </View>
            </Playground>

            <Playground title="Bordered + Custom Aria/Class" code_signal=bordered_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <Header
                        tone=HeaderTone::Strong
                        bordered=true
                        aria_label="Settings header".to_string()
                        class_name="docs-header-custom".to_string()
                    >
                        <h3>"Settings"</h3>
                    </Header>
                    <Content padded=true>
                        <p>"Header above content, matching baseline container semantics."</p>
                    </Content>
                </View>
            </Playground>

            <Playground
                title="Interactive Playground (State + Source Markers)"
                code_signal=interactive_code
                test_css_source=test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui-components/src/header/styles.rs".to_string()
                test_config_signal=actual_config
                description="Workbench canvas: 展示区负责状态对比；Config/Code/CSS Test 区用于快速验证契约。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="header-config-controls">
                        <div class="docs-search__label">"Tone"</div>
                        <button
                            type="button"
                            data-action="toggle-tone-config"
                            on:click=move |_| {
                                set_interactive_strong_tone.update(|value| *value = !*value);
                            }
                        >
                            "Toggle strong tone"
                        </button>
                        <div class="docs-search__label">"Border"</div>
                        <button
                            type="button"
                            data-action="toggle-bordered-config"
                            on:click=move |_| {
                                set_interactive_bordered.update(|value| *value = !*value);
                            }
                        >
                            "Toggle bordered"
                        </button>
                        <p class="ui-muted" data-slot="header-config-summary">
                            {move || {
                                format!(
                                    "config: tone={} bordered={}",
                                    if interactive_strong_tone.get() {
                                        "strong"
                                    } else {
                                        "default"
                                    },
                                    if interactive_bordered.get() {
                                        "true"
                                    } else {
                                        "false"
                                    }
                                )
                            }}
                        </p>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="header-interactive-controls">
                    <div class="docs-row" data-slot="header-actions">
                        <button
                            type="button"
                            data-action="toggle-tone"
                            on:click=move |_| {
                                set_interactive_strong_tone.update(|value| *value = !*value);
                            }
                        >
                            "Toggle tone"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-bordered"
                            on:click=move |_| {
                                set_interactive_bordered.update(|value| *value = !*value);
                            }
                        >
                            "Toggle bordered"
                        </button>
                    </div>

                    <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                        <Header
                            tone=if interactive_strong_tone.get() {
                                HeaderTone::Strong
                            } else {
                                HeaderTone::Default
                            }
                            bordered=interactive_bordered.get()
                            class_name="docs-header-interactive".to_string()
                            aria_label="Interactive docs header".to_string()
                        >
                            <h3>"Interactive header"</h3>
                        </Header>
                        <Content padded=true>
                            <p data-slot="header-interactive-summary">
                                {move || {
                                    format!(
                                        "tone={} bordered={}",
                                        if interactive_strong_tone.get() {
                                            "strong"
                                        } else {
                                            "default"
                                        },
                                        if interactive_bordered.get() {
                                            "true"
                                        } else {
                                            "false"
                                        }
                                    )
                                }}
                            </p>
                        </Content>
                    </View>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="header-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each header playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\n<Header>\n  <h3>\"Settings\"</h3>\n</Header>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-header-source-copy".to_string()
                />
                <ul data-slot="header-source-paths">
                    <li><code>"crates/ui-components/src/header/mod.rs"</code></li>
                    <li><code>"crates/ui-components/src/header/logic.rs"</code></li>
                    <li><code>"crates/ui-components/src/header/view.rs"</code></li>
                    <li><code>"crates/ui-components/src/header/styles.rs"</code></li>
                    <li><code>"crates/ui-components/src/header/motion.rs"</code></li>
                </ul>
                <ul data-slot="header-source-prerequisites">
                    <li><code>"component-header"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn footer() -> AnyView {
    let (workbench_muted, set_workbench_muted) = signal(true);
    let (workbench_bordered, set_workbench_bordered) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let semantic_code = Signal::derive(move || {
        r#"<Footer>
  <p>"Cancel · Save"</p>
</Footer>
<Footer tone=FooterTone::Muted>
  <p>"Secondary action hint"</p>
</Footer>"#
            .to_string()
    });

    let bordered_code = Signal::derive(move || {
        r#"<View border=ViewBorder::Subtle radius=ViewRadius::Md>
  <Header bordered=true>
    <h3>"Profile settings"</h3>
  </Header>
  <Content padded=true>
    <p>"Main settings body"</p>
  </Content>
  <Footer
    tone=FooterTone::Muted
    bordered=true
    aria_label="Settings footer".to_string()
    class_name="docs-footer-custom".to_string()
  >
    <p>"Cancel · Save"</p>
  </Footer>
</View>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone = if workbench_muted.get() {
            "FooterTone::Muted"
        } else {
            "FooterTone::Default"
        };
        let bordered = workbench_bordered.get();
        let aria = workbench_custom_aria.get();
        let class_name = workbench_custom_class.get();

        let mut lines = vec!["<Footer".to_string()];
        lines.push(format!("  tone={tone}"));
        if bordered {
            lines.push("  bordered=true".to_string());
        }
        if aria {
            lines.push("  aria_label=\"Workbench footer\".to_string()".to_string());
        }
        if class_name {
            lines.push("  class_name=\"docs-footer-workbench\".to_string()".to_string());
        }
        lines.extend([
            ">".to_string(),
            "  <p>\"Cancel · Save\"</p>".to_string(),
            "</Footer>".to_string(),
        ]);
        lines.join("\n")
    });

    let footer_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/footer/styles.rs */\n{}",
            ui_components::footer::styles::CSS
        )
    });

    let footer_actual_config = Signal::derive(move || {
        let tone = if workbench_muted.get() {
            FooterTone::Muted
        } else {
            FooterTone::Default
        };
        let bordered = workbench_bordered.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let aria_label = if custom_aria {
            "Workbench footer"
        } else {
            "Footer"
        };

        let mut classes = vec!["ui-footer".to_string(), tone.class_name().to_string()];
        if bordered {
            classes.push("ui-footer--bordered".to_string());
        }
        if custom_class {
            classes.push("ui-footer--custom-class".to_string());
            classes.push("docs-footer-workbench".to_string());
        }

        format!(
            "FooterActualConfig {{\n  tone: {tone:?},\n  bordered: {bordered},\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  aria_label: \"{aria_label}\",\n  data_state: \"{}\",\n  class: \"{}\",\n}}",
            if bordered && matches!(tone, FooterTone::Muted) {
                "muted-bordered"
            } else if bordered {
                "bordered"
            } else if matches!(tone, FooterTone::Muted) {
                "muted"
            } else {
                "default"
            },
            classes.join(" "),
        )
    });

    view! {
        <ComponentPage
            title="Footer"
            slug="footer"
            group="Layout"
            description="Semantic container footer (`<footer>`) with centralized tone/border/source state contracts."
        >
            <Playground title="Semantic Footer + Tone" code_signal=semantic_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <div class="docs-stack">
                        <Footer>
                            <p>"Cancel · Save"</p>
                        </Footer>
                        <Footer tone=FooterTone::Muted>
                            <p>"Secondary action hint"</p>
                        </Footer>
                    </div>
                </View>
            </Playground>

            <Playground title="Bordered + Custom Aria/Class" code_signal=bordered_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <Header bordered=true>
                        <h3>"Profile settings"</h3>
                    </Header>
                    <Content padded=true>
                        <p>"Main settings body"</p>
                    </Content>
                    <Footer
                        tone=FooterTone::Muted
                        bordered=true
                        aria_label="Settings footer".to_string()
                        class_name="docs-footer-custom".to_string()
                    >
                        <p>"Cancel · Save"</p>
                    </Footer>
                </View>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=footer_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/footer/styles.rs".to_string()
                test_config_signal=footer_actual_config
                description="Footer workbench: 对比展示 + config 快照 + copy-ready code + scoped CSS test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <ui_components::Switch checked=workbench_muted set_checked=set_workbench_muted>
                            "Muted tone"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_bordered set_checked=set_workbench_bordered>
                            "Bordered"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria_label"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </ui_components::Switch>
                    </div>
                }
            >
                {move || {
                    let tone = if workbench_muted.get() {
                        FooterTone::Muted
                    } else {
                        FooterTone::Default
                    };
                    let bordered = workbench_bordered.get();
                    let custom_aria = workbench_custom_aria.get();
                    let custom_class = workbench_custom_class.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                <View border=ViewBorder::Subtle radius=ViewRadius::Md class_name="docs-footer-workbench-card".to_string()>
                                    <Header bordered=true>
                                        <h3>"Configured Footer"</h3>
                                    </Header>
                                    <Content padded=true>
                                        <p>"State toggles apply to this footer."</p>
                                    </Content>
                                    <Footer
                                        tone=tone
                                        bordered=bordered
                                        aria_label=if custom_aria {
                                            "Workbench footer".to_string()
                                        } else {
                                            String::new()
                                        }
                                        class_name=if custom_class {
                                            "docs-footer-workbench".to_string()
                                        } else {
                                            String::new()
                                        }
                                    >
                                        <p>"Cancel · Save"</p>
                                    </Footer>
                                </View>

                                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                                    <Header bordered=true>
                                        <h3>"Reference Footer"</h3>
                                    </Header>
                                    <Content padded=true>
                                        <p>"Baseline default state for comparison."</p>
                                    </Content>
                                    <Footer>
                                        <p>"Cancel · Save"</p>
                                    </Footer>
                                </View>
                            </div>

                            <div class="ui-muted">
                                {format!(
                                    "comparison: configured(tone={}, bordered={}, custom_aria={}, custom_class={}) vs reference(default)",
                                    if matches!(tone, FooterTone::Muted) { "muted" } else { "default" },
                                    bordered,
                                    custom_aria,
                                    custom_class,
                                )}
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn heading() -> AnyView {
    let levels_code = Signal::derive(move || {
        r#"<Heading level=HeadingLevel::H1>"Display title"</Heading>
<Heading level=HeadingLevel::H3>"Section title"</Heading>
<Heading level=HeadingLevel::H5 tone=HeadingTone::Muted>"Meta heading"</Heading>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Heading
  level=HeadingLevel::H4
  tone=HeadingTone::Strong
  truncate=true
  class_name="docs-heading-custom".to_string()
  aria_label="Truncated heading".to_string()
>
  "Long heading title that intentionally exceeds the available inline width to verify truncation"
</Heading>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Heading"
            slug="heading"
            group="Layout"
            description="baseline-style semantic heading (`<h1>`..`<h6>`) with centralized level/tone/truncate contracts."
        >
            <Playground title="Heading Levels + Tone" code_signal=levels_code>
                <div class="docs-stack">
                    <Heading level=HeadingLevel::H1>"Display title"</Heading>
                    <Heading level=HeadingLevel::H3>"Section title"</Heading>
                    <Heading level=HeadingLevel::H5 tone=HeadingTone::Muted>
                        "Meta heading"
                    </Heading>
                </div>
            </Playground>

            <Playground title="Strong + Truncate + Custom Aria/Class" code_signal=states_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <Heading
                        level=HeadingLevel::H4
                        tone=HeadingTone::Strong
                        truncate=true
                        class_name="docs-heading-custom".to_string()
                        aria_label="Truncated heading".to_string()
                    >
                        "Long heading title that intentionally exceeds the available inline width to verify truncation"
                    </Heading>
                </View>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn divider() -> AnyView {
    let orientations_code = Signal::derive(move || {
        r#"<Divider />
<Divider orientation=DividerOrientation::Vertical class_name="docs-divider-rail".to_string() />"#
            .to_string()
    });

    let custom_class_code = Signal::derive(move || {
        r#"<Divider class_name="docs-divider-custom".to_string() />
<Divider
  orientation=DividerOrientation::Vertical
  class_name="docs-divider-custom docs-divider-rail".to_string()
/>"#
        .to_string()
    });

    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let workbench_code = Signal::derive(move || {
        format!(
            "<Divider\n  orientation=DividerOrientation::{}\n  motion=DividerMotion {{ animate_in: {} }}\n  class_name={}\n/>",
            if workbench_vertical.get() {
                "Vertical"
            } else {
                "Horizontal"
            },
            workbench_custom_motion.get(),
            if workbench_custom_class.get() {
                "\"docs-divider-custom docs-divider-rail\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/divider/styles.rs */\n{}",
            ui_components::divider::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let orientation = if workbench_vertical.get() {
            DividerOrientation::Vertical
        } else {
            DividerOrientation::Horizontal
        };
        let custom_motion = workbench_custom_motion.get();
        let custom_class = workbench_custom_class.get();

        let mut classes = vec![
            "ui-divider".to_string(),
            orientation.class_name().to_string(),
        ];
        if custom_class {
            classes.push("docs-divider-custom".to_string());
            if matches!(orientation, DividerOrientation::Vertical) {
                classes.push("docs-divider-rail".to_string());
            }
        }

        format!(
            "DividerActualConfig {{\n  orientation: {orientation:?},\n  aria_orientation: {:?},\n  custom_class: {custom_class},\n  custom_motion: {custom_motion},\n  data_motion_source: \"{}\",\n  class: \"{}\",\n}}",
            orientation.aria_orientation(),
            if custom_motion { "custom" } else { "default" },
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Divider"
            slug="divider"
            group="Layout"
            description="A separator primitive with centralized orientation state attrs and baseline-style styling markers."
        >
            <Playground title="Orientation" code_signal=orientations_code>
                <div class="docs-stack">
                    <div>"Above"</div>
                    <Divider />
                    <div>"Below"</div>
                    <div class="docs-row">
                        <span>"Left"</span>
                        <Divider
                            orientation=DividerOrientation::Vertical
                            class_name="docs-divider-rail".to_string()
                        />
                        <span>"Right"</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class Marker" code_signal=custom_class_code>
                <div class="docs-stack">
                    <span>"Custom horizontal divider"</span>
                    <Divider class_name="docs-divider-custom".to_string() />
                    <div class="docs-row">
                        <span>"Start"</span>
                        <Divider
                            orientation=DividerOrientation::Vertical
                            class_name="docs-divider-custom docs-divider-rail".to_string()
                        />
                        <span>"End"</span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/divider/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="展示区对比 default/workbench；Config 控制 orientation/motion/class，Code 与 CSS Test 用于契约回归。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="divider-config-controls">
                        <button
                            type="button"
                            data-action="toggle-orientation-config"
                            on:click=move |_| {
                                set_workbench_vertical.update(|value| *value = !*value);
                            }
                        >
                            "Toggle orientation"
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
                            data-action="toggle-motion-config"
                            on:click=move |_| {
                                set_workbench_custom_motion.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom motion"
                        </button>
                        <p class="ui-muted" data-slot="divider-config-summary">
                            {move || {
                                format!(
                                    "config: orientation={} class={} motion={}",
                                    if workbench_vertical.get() {
                                        "vertical"
                                    } else {
                                        "horizontal"
                                    },
                                    if workbench_custom_class.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    },
                                    if workbench_custom_motion.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    },
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
                            <span>"Above"</span>
                            <Divider />
                            <span>"Below"</span>
                        </div>

                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"workbench"</span>
                            {move || {
                                if workbench_vertical.get() {
                                    view! {
                                        <div class="docs-row">
                                            <span>"Start"</span>
                                            <Divider
                                                orientation=DividerOrientation::Vertical
                                                motion=if workbench_custom_motion.get() {
                                                    ui_components::divider::motion::DividerMotion {
                                                        animate_in: true,
                                                    }
                                                } else {
                                                    ui_components::divider::motion::DividerMotion::default()
                                                }
                                                class_name=if workbench_custom_class.get() {
                                                    "docs-divider-custom docs-divider-rail".to_string()
                                                } else {
                                                    "".to_string()
                                                }
                                            />
                                            <span>"End"</span>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <div class="docs-stack docs-stack--tight">
                                            <span>"Start"</span>
                                            <Divider
                                                orientation=DividerOrientation::Horizontal
                                                motion=if workbench_custom_motion.get() {
                                                    ui_components::divider::motion::DividerMotion {
                                                        animate_in: true,
                                                    }
                                                } else {
                                                    ui_components::divider::motion::DividerMotion::default()
                                                }
                                                class_name=if workbench_custom_class.get() {
                                                    "docs-divider-custom".to_string()
                                                } else {
                                                    "".to_string()
                                                }
                                            />
                                            <span>"End"</span>
                                        </div>
                                    }
                                        .into_any()
                                }
                            }}
                        </div>
                    </div>
                    <span class="ui-muted" data-slot="divider-workbench-summary">
                        {move || {
                            format!(
                                "orientation={} class={} motion={}",
                                if workbench_vertical.get() {
                                    "vertical"
                                } else {
                                    "horizontal"
                                },
                                if workbench_custom_class.get() {
                                    "custom"
                                } else {
                                    "default"
                                },
                                if workbench_custom_motion.get() {
                                    "custom"
                                } else {
                                    "default"
                                }
                            )
                        }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn separator() -> AnyView {
    let semantic_code = Signal::derive(move || {
        r#"<Separator />
<Separator element_type=SeparatorElementType::Hr />
<Separator orientation=SeparatorOrientation::Vertical class_name="docs-separator-rail".to_string() />"#.to_string()
    });

    let decorative_code = Signal::derive(move || {
        r#"<Separator is_decorative=true />
<Separator
  is_decorative=true
  orientation=SeparatorOrientation::Vertical
  class_name="docs-separator-rail docs-separator-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Separator"
            slug="separator"
            group="Layout"
            description="Spring-enabled separator with centralized orientation/element/decorative state attrs."
        >
            <Playground title="Semantic + Element Type" code_signal=semantic_code>
                <div class="docs-stack">
                    <div class="docs-stack docs-stack--tight">
                        <span>"Above"</span>
                        <Separator />
                        <span>"Below"</span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <span>"HR element path"</span>
                        <Separator element_type=SeparatorElementType::Hr />
                        <span class="ui-muted">"Uses `<hr>` with the same motion/state contract."</span>
                    </div>

                    <div class="docs-row">
                        <span>"Left"</span>
                        <Separator
                            orientation=SeparatorOrientation::Vertical
                            class_name="docs-separator-rail".to_string()
                        />
                        <span>"Right"</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Decorative + Custom Class" code_signal=decorative_code>
                <div class="docs-stack">
                    <span>"Decorative separator (aria-hidden)"</span>
                    <Separator is_decorative=true class_name="docs-separator-custom".to_string() />

                    <div class="docs-row">
                        <span>"Start"</span>
                        <Separator
                            is_decorative=true
                            orientation=SeparatorOrientation::Vertical
                            class_name="docs-separator-rail docs-separator-custom".to_string()
                        />
                        <span>"End"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn spacer() -> AnyView {
    let hello_code = Signal::derive(move || r#"<Spacer />"#.to_string());

    let axis_and_size_code = Signal::derive(move || {
        r#"<Spacer axis=SpacerAxis::Vertical size=SpacerSize::Sm />
<Spacer axis=SpacerAxis::Vertical size=SpacerSize::Lg />
<Spacer axis=SpacerAxis::Horizontal size=SpacerSize::Md />"#
            .to_string()
    });

    let custom_class_code = Signal::derive(move || {
        r#"<Spacer
  axis=SpacerAxis::Vertical
  size=SpacerSize::Md
  class_name="docs-spacer-guide".to_string()
/>
<Spacer
  axis=SpacerAxis::Horizontal
  size=SpacerSize::Lg
  class_name="docs-spacer-guide".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Spacer"
            slug="spacer"
            group="Layout"
            description="A pure spacing primitive with centralized axis/size state attrs for baseline-style styling contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-stack">
                    <span class="docs-spacer-box">"Before"</span>
                    <Spacer />
                    <span class="docs-spacer-box">"After"</span>
                </div>
            </Playground>

            <Playground title="Axis + Size" code_signal=axis_and_size_code>
                <div class="docs-stack">
                    <div class="docs-stack">
                        <span class="docs-spacer-box">"Top"</span>
                        <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Sm />
                        <span class="docs-spacer-box">"Small gap"</span>
                        <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Lg />
                        <span class="docs-spacer-box">"Large gap"</span>
                    </div>

                    <div class="docs-row">
                        <span class="docs-spacer-box">"Left"</span>
                        <Spacer axis=SpacerAxis::Horizontal size=SpacerSize::Md />
                        <span class="docs-spacer-box">"Right"</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class Marker" code_signal=custom_class_code>
                <div class="docs-stack">
                    <span class="docs-spacer-box">"Custom vertical spacer"</span>
                    <Spacer
                        axis=SpacerAxis::Vertical
                        size=SpacerSize::Md
                        class_name="docs-spacer-guide".to_string()
                    />
                    <span class="docs-spacer-box">"Marker visible via custom class"</span>

                    <div class="docs-row">
                        <span class="docs-spacer-box">"Start"</span>
                        <Spacer
                            axis=SpacerAxis::Horizontal
                            size=SpacerSize::Lg
                            class_name="docs-spacer-guide".to_string()
                        />
                        <span class="docs-spacer-box">"End"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn well() -> AnyView {
    let hello_code = Signal::derive(move || {
        r#"<Well>
  <div>"Default well"</div>
</Well>"#
            .to_string()
    });

    let tone_code = Signal::derive(move || {
        r#"<Well tone=WellTone::Default>
  <div>"Default well"</div>
</Well>
<Well tone=WellTone::Quiet density=WellDensity::Compact>
  <div>"Quiet compact well"</div>
</Well>
<Well tone=WellTone::Strong is_inset=true>
  <div>"Strong inset well"</div>
</Well>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Well
  tone=WellTone::Strong
  is_inset=true
  aria_label="Selection summary".to_string()
  class_name="docs-well-custom".to_string()
>
  <div>"Custom class + label"</div>
</Well>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Well"
            slug="well"
            group="Layout"
            description="Inset container surface for grouped content with centralized tone/density/label state contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <Well>
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Default well"</strong>
                        <span class="ui-muted">"Minimal usage with default tone/density and non-inset state."</span>
                    </div>
                </Well>
            </Playground>

            <Playground title="Tone + Density + Inset" code_signal=tone_code>
                <div class="docs-stack">
                    <Well tone=WellTone::Default>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Default"</strong>
                            <span class="ui-muted">"Balanced neutral container for grouped content."</span>
                        </div>
                    </Well>

                    <Well tone=WellTone::Quiet density=WellDensity::Compact>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Quiet compact"</strong>
                            <span class="ui-muted">"Lower-contrast surface with tighter spacing."</span>
                        </div>
                    </Well>

                    <Well tone=WellTone::Strong is_inset=true>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Strong inset"</strong>
                            <span class="ui-muted">"Emphasized background with inset ring contract."</span>
                        </div>
                    </Well>
                </div>
            </Playground>

            <Playground title="Custom Label + Class" code_signal=custom_code>
                <Well
                    tone=WellTone::Strong
                    is_inset=true
                    aria_label="Selection summary".to_string()
                    class_name="docs-well-custom".to_string()
                >
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Selection summary"</strong>
                        <span class="ui-muted">
                            "Verifies aria label fallback/custom source and class merge contract."
                        </span>
                    </div>
                </Well>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn scroll_shadow() -> AnyView {
    let hello_code = Signal::derive(move || {
        r#"<ScrollShadow>
  <div class="docs-scroll-shadow-item">Activity</div>
</ScrollShadow>"#
            .to_string()
    });

    let default_code = Signal::derive(move || {
        r#"<ScrollShadow max_height_px=160>
  <div class="docs-stack docs-stack--tight">
    {(1..=20)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Activity {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollShadow>"#
            .to_string()
    });

    let custom_class_code = Signal::derive(move || {
        r#"<ScrollShadow max_height_px=120 class_name="docs-scroll-shadow-custom".to_string()>
  <div class="docs-stack docs-stack--tight">
    {(1..=16)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Notification {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollShadow>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ScrollShadow"
            slug="scroll-shadow"
            group="Layout"
            description="Adds top/bottom shadow indicators with centralized edge/max-height state attrs."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ScrollShadow>
                    <div class="docs-scroll-shadow-item">"Activity"</div>
                </ScrollShadow>
            </Playground>

            <Playground title="Default Scrollable" code_signal=default_code>
                <ScrollShadow max_height_px=160>
                    <div class="docs-stack docs-stack--tight">
                        {(1..=20)
                            .map(|idx| {
                                view! { <div class="docs-scroll-shadow-item">{format!("Activity {idx}")}</div> }
                            })
                            .collect_view()}
                    </div>
                </ScrollShadow>
            </Playground>

            <Playground title="Custom Height + Class" code_signal=custom_class_code>
                <ScrollShadow max_height_px=120 class_name="docs-scroll-shadow-custom".to_string()>
                    <div class="docs-stack docs-stack--tight">
                        {(1..=16)
                            .map(|idx| {
                                view! {
                                    <div class="docs-scroll-shadow-item">
                                        {format!("Notification {idx}")}
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </ScrollShadow>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn auto_height() -> AnyView {
    let (animated_open, set_animated_open) = signal(false);
    let (static_open, set_static_open) = signal(false);
    let (workbench_open, set_workbench_open) = signal(true);
    let (workbench_animate, set_workbench_animate) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let animated_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);
<Button on_press=...>"Toggle"</Button>
<AutoHeight class_name="docs-auto-height".to_string()>
  <Show when=open>...</Show>
</AutoHeight>"#
            .to_string()
    });

    let static_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);
let motion = AutoHeightMotion {
  animate_height: false,
  ..AutoHeightMotion::default()
};
<AutoHeight motion=motion class_name="docs-auto-height docs-auto-height--static-demo".to_string()>
  <Show when=open>...</Show>
</AutoHeight>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let animate = workbench_animate.get();
        let custom_class = workbench_custom_class.get();
        let mut lines = vec!["let (open, set_open) = signal(true);".to_string()];
        lines.push("<AutoHeight".to_string());
        if !animate {
            lines.push("  motion=AutoHeightMotion { animate_height: false, ..AutoHeightMotion::default() }".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-auto-height-workbench\".to_string()".to_string());
        }
        lines.extend([
            ">".to_string(),
            "  <Show when=move || open.get()>".to_string(),
            "    <div>\"Workbench content\"</div>".to_string(),
            "  </Show>".to_string(),
            "</AutoHeight>".to_string(),
        ]);
        lines.join("\n")
    });

    let auto_height_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/auto_height/styles.rs */\n{}",
            ui_components::auto_height::styles::CSS
        )
    });

    let auto_height_actual_config = Signal::derive(move || {
        let animate_height = workbench_animate.get();
        let custom_class = workbench_custom_class.get();
        let is_open = workbench_open.get();
        let mut classes = vec!["ui-auto-height".to_string()];

        if animate_height {
            classes.push("ui-auto-height--animated".to_string());
        } else {
            classes.push("ui-auto-height--static".to_string());
            classes.push("ui-auto-height--custom-motion".to_string());
        }

        if custom_class {
            classes.push("ui-auto-height--custom-class".to_string());
            classes.push("docs-auto-height-workbench".to_string());
        }

        format!(
            "AutoHeightActualConfig {{\n  open: {is_open},\n  animate_height: {animate_height},\n  custom_class: {custom_class},\n  data_state: \"{}\",\n  data_motion_source: \"{}\",\n  class: \"{}\",\n}}",
            if animate_height { "animated" } else { "static" },
            if animate_height { "default" } else { "custom" },
            classes.join(" "),
        )
    });

    view! {
        <ComponentPage
            title="AutoHeight"
            slug="auto-height"
            group="Layout"
            description="Animates (or snaps) height changes via spring-driven CSS variables with centralized motion/class state attrs."
        >
            <Playground title="Animated Height" code_signal=animated_code>
                <div class="docs-stack">
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_animated_open.update(|v| *v = !*v))
                    >
                        {move || if animated_open.get() { "Collapse" } else { "Expand" }}
                    </ui_components::Button>

                    <AutoHeight class_name="docs-auto-height".to_string()>
                        <Show when=move || animated_open.get()>
                            <div class="docs-stack">
                                <div>"AutoHeight content"</div>
                                <div class="ui-muted">"ResizeObserver + ui-motion spring."</div>
                                <div class="ui-muted">"Toggle quickly to verify stable interpolation."</div>
                            </div>
                        </Show>
                    </AutoHeight>
                </div>
            </Playground>

            <Playground title="Static Motion + Custom Class" code_signal=static_code>
                <div class="docs-stack">
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_static_open.update(|v| *v = !*v))
                    >
                        {move || if static_open.get() { "Hide Static" } else { "Show Static" }}
                    </ui_components::Button>

                    <AutoHeight
                        motion=AutoHeightMotion {
                            animate_height: false,
                            ..AutoHeightMotion::default()
                        }
                        class_name="docs-auto-height docs-auto-height--static-demo".to_string()
                    >
                        <Show when=move || static_open.get()>
                            <div class="docs-stack">
                                <div>"Static mode content"</div>
                                <div class="ui-muted">"Uses custom motion contract (`animate_height=false`)."</div>
                                <div class="ui-muted">"Useful for reduced-motion or deterministic layout jumps."</div>
                            </div>
                        </Show>
                    </AutoHeight>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=auto_height_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/auto_height/styles.rs".to_string()
                test_config_signal=auto_height_actual_config
                description="AutoHeight workbench: 展示区 + config 快照 + code + scoped CSS test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <ui_components::Switch checked=workbench_open set_checked=set_workbench_open>
                            "Open content"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_animate set_checked=set_workbench_animate>
                            "Animate height"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </ui_components::Switch>
                    </div>
                }
            >
                {move || {
                    let animate_height = workbench_animate.get();
                    let custom_class = workbench_custom_class.get();
                    let is_open = workbench_open.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                                    <Header bordered=true>
                                        <h3>"Configured AutoHeight"</h3>
                                    </Header>
                                    <AutoHeight
                                        motion=AutoHeightMotion {
                                            animate_height,
                                            ..AutoHeightMotion::default()
                                        }
                                        class_name=if custom_class {
                                            "docs-auto-height-workbench".to_string()
                                        } else {
                                            String::new()
                                        }
                                    >
                                        <Show when=move || is_open>
                                            <div class="docs-stack">
                                                <div>"Configured workbench content"</div>
                                                <div class="ui-muted">"Toggle open/animate/class to compare state markers."</div>
                                            </div>
                                        </Show>
                                    </AutoHeight>
                                </View>

                                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                                    <Header bordered=true>
                                        <h3>"Reference AutoHeight"</h3>
                                    </Header>
                                    <AutoHeight class_name="docs-auto-height".to_string()>
                                        <div class="docs-stack">
                                            <div>"Reference content (always shown)"</div>
                                            <div class="ui-muted">"Baseline animated/default contract."</div>
                                        </div>
                                    </AutoHeight>
                                </View>
                            </div>

                            <div class="ui-muted">
                                {format!(
                                    "comparison: configured(open={}, animate_height={}, custom_class={}) vs reference(default)",
                                    is_open,
                                    animate_height,
                                    custom_class,
                                )}
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
