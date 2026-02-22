use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::Snippet;
use ui_headless::A11yDirection;
use ui_layout::{
    AutoHeight, AutoHeightMotion, Card, CardVariant, Content, ContentTone, Divider, DividerMotion,
    DividerOrientation, Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexMotion, FlexWrap,
    Footer, FooterTone, Header, HeaderTone, Heading, HeadingLevel, HeadingTone, ScrollShadow,
    Separator, SeparatorElementType, SeparatorMotion, SeparatorOrientation, Spacer, SpacerAxis,
    SpacerMotion, SpacerSize, View, ViewBackground, ViewBorder, ViewElement, ViewPadding,
    ViewRadius, ViewShadow, Well, WellDensity, WellTone,
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
            "  class_name=\"docs-card-custom\".into()\n"
        } else {
            ""
        };

        format!(
            "<Card\n{variant_line}{padded_line}{class_line}>\n  <div>\"Workbench content\"</div>\n</Card>"
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/card/styles.rs */\n{}",
            ui_layout::card::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant = workbench_variant.get();
        let padded = workbench_padded.get();
        let custom_class = workbench_custom_class.get();
        let class_name = if custom_class {
            Some("docs-card-custom")
        } else {
            None
        };

        let mut classes = vec![
            "ui-card".to_string(),
            variant.class_name().into(),
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
            "CardActualConfig {{\n  variant: {variant:?},\n  padded: {padded},\n  custom_class: {custom_class},\n  class_name: {class_name:?},\n  data_variant: \"{}\",\n  data_state: \"{}\",\n  class: \"{}\",\n}}",
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
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/card/styles.rs".to_string()
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

            <Playground
                title="State Matrix (Variant / Padding / Class Comparison)"
                code_signal=variants_code
            >
                <div class="docs-row">
                    <Card variant=CardVariant::Default>
                        <div>"Default"</div>
                    </Card>
                    <Card variant=CardVariant::Muted padded=false>
                        <div>"Muted + flush"</div>
                    </Card>
                    <Card variant=CardVariant::Outline class_name="docs-card-custom".to_string()>
                        <div>"Outline + custom class"</div>
                    </Card>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn view() -> AnyView {
    let (workbench_accent, set_workbench_accent) = signal(false);
    let (workbench_strong_border, set_workbench_strong_border) = signal(false);
    let (workbench_large_padding, set_workbench_large_padding) = signal(false);
    let (workbench_large_radius, set_workbench_large_radius) = signal(false);
    let (workbench_shadow_enabled, set_workbench_shadow_enabled) = signal(false);
    let (workbench_section, set_workbench_section) = signal(false);
    let (workbench_fluid, set_workbench_fluid) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_background = Signal::derive(move || {
        if workbench_accent.get() {
            ViewBackground::Accent
        } else {
            ViewBackground::Subtle
        }
    });
    let workbench_border = Signal::derive(move || {
        if workbench_strong_border.get() {
            ViewBorder::Strong
        } else {
            ViewBorder::Subtle
        }
    });
    let workbench_padding = Signal::derive(move || {
        if workbench_large_padding.get() {
            ViewPadding::Lg
        } else {
            ViewPadding::Md
        }
    });
    let workbench_radius = Signal::derive(move || {
        if workbench_large_radius.get() {
            ViewRadius::Lg
        } else {
            ViewRadius::Md
        }
    });
    let workbench_shadow = Signal::derive(move || {
        if workbench_shadow_enabled.get() {
            ViewShadow::Md
        } else {
            ViewShadow::None
        }
    });
    let workbench_element = Signal::derive(move || {
        if workbench_section.get() {
            ViewElement::Section
        } else {
            ViewElement::Div
        }
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<View\n  background=ViewBackground::{:?}\n  border=ViewBorder::{:?}\n  padding=ViewPadding::{:?}\n  radius=ViewRadius::{:?}\n  shadow=ViewShadow::{:?}\n  element=ViewElement::{:?}\n  fluid={}\n  aria_label={}\n  class_name={}\n>\n  <div>\"Workbench content\"</div>\n</View>",
            workbench_background.get(),
            workbench_border.get(),
            workbench_padding.get(),
            workbench_radius.get(),
            workbench_shadow.get(),
            workbench_element.get(),
            workbench_fluid.get(),
            if workbench_custom_aria.get() {
                "\"Release notes\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_custom_class.get() {
                "\"docs-view-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ViewActualConfig {{\n  background: ViewBackground::{:?},\n  border: ViewBorder::{:?},\n  padding: ViewPadding::{:?},\n  radius: ViewRadius::{:?},\n  shadow: ViewShadow::{:?},\n  element: ViewElement::{:?},\n  fluid: {},\n  aria_label: {},\n  class_name: {},\n}}",
            workbench_background.get(),
            workbench_border.get(),
            workbench_padding.get(),
            workbench_radius.get(),
            workbench_shadow.get(),
            workbench_element.get(),
            workbench_fluid.get(),
            if workbench_custom_aria.get() {
                "Some(\"Release notes\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-view-custom\")"
            } else {
                "None"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md />
<View background=ViewBackground::Accent border=ViewBorder::Strong padding=ViewPadding::Lg radius=ViewRadius::Lg shadow=ViewShadow::Md />
<View element=ViewElement::Section fluid=true aria_label="Release notes".to_string() class_name="docs-view-custom".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="View"
            slug="view"
            group="Layout"
            description="General-purpose baseline-style container with centralized surface token state and stable data markers."
        >
            <Playground
                title="Hello World (Default View)"
                code_signal=Signal::derive(move || {
                    r#"<View border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md>
  <div>"Default container"</div>
</View>"#
                        .to_string()
                })
            >
                <View border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md>
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Default container"</strong>
                        <span class="ui-muted">"Baseline layout surface for content blocks."</span>
                    </div>
                </View>
            </Playground>

            <Playground title="Element + Fluid + Custom Class"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="view-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_accent.get()
                                on:change=move |ev| set_workbench_accent.set(event_target_checked(&ev))
                            />
                            " background accent"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_strong_border.get()
                                on:change=move |ev| set_workbench_strong_border.set(event_target_checked(&ev))
                            />
                            " border strong"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_large_padding.get()
                                on:change=move |ev| set_workbench_large_padding.set(event_target_checked(&ev))
                            />
                            " padding large"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_large_radius.get()
                                on:change=move |ev| set_workbench_large_radius.set(event_target_checked(&ev))
                            />
                            " radius large"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_shadow_enabled.get()
                                on:change=move |ev| set_workbench_shadow_enabled.set(event_target_checked(&ev))
                            />
                            " shadow md"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_section.get()
                                on:change=move |ev| set_workbench_section.set(event_target_checked(&ev))
                            />
                            " element section"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_fluid.get()
                                on:change=move |ev| set_workbench_fluid.set(event_target_checked(&ev))
                            />
                            " fluid"
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
                <View
                    background=workbench_background.get()
                    border=workbench_border.get()
                    padding=workbench_padding.get()
                    radius=workbench_radius.get()
                    shadow=workbench_shadow.get()
                    element=workbench_element.get()
                    fluid=workbench_fluid.get()
                    aria_label=if workbench_custom_aria.get() {
                        "Release notes".to_string()
                    } else {
                        String::new()
                    }
                    class_name=if workbench_custom_class.get() {
                        "docs-view-custom".to_string()
                    } else {
                        String::new()
                    }
                >
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Workbench view"</strong>
                        <span class="ui-muted">"Adjust all View props and inspect actual config."</span>
                    </div>
                </View>
            </Playground>

            <Playground title="Surface Tokens" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <View border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md>
                        <div>"Default"</div>
                    </View>
                    <View
                        background=ViewBackground::Accent
                        border=ViewBorder::Strong
                        padding=ViewPadding::Lg
                        radius=ViewRadius::Lg
                        shadow=ViewShadow::Md
                    >
                        <div>"Accent + elevated"</div>
                    </View>
                    <View
                        element=ViewElement::Section
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Sm
                        radius=ViewRadius::Sm
                        fluid=true
                        aria_label="Release notes".to_string()
                        class_name="docs-view-custom".to_string()
                    >
                        <div>"Section + fluid"</div>
                    </View>
                    <View
                        element=ViewElement::Span
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Sm
                        radius=ViewRadius::Sm
                    >
                        <span>"Span element"</span>
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
            snippet.push("  aria_label=\"Docs content area\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-content-workbench\".into()".to_string());
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
            "/* crates/ui-layout/src/content/styles.rs */\n{}",
            ui_layout::content::styles::CSS
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
            "ContentActualConfig {{\n  tone: ContentTone::{tone:?},\n  padded: {padded},\n  aria_label: {},\n  class_name: {},\n  aria_source: {},\n  class_source: {},\n  data_state: \"{state}\",\n}}",
            if custom_aria {
                "Some(\"Docs content area\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-content-workbench\")"
            } else {
                "None"
            },
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
                test_source_path="crates/ui-layout/src/content/styles.rs".to_string()
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

            <Playground
                title="State Matrix (Tone / Padding / Source Comparison)"
                code_signal=padded_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Content>
                        <p>"Default content"</p>
                    </Content>
                    <Content tone=ContentTone::Muted padded=true>
                        <p>"Muted + padded"</p>
                    </Content>
                    <Content
                        padded=true
                        aria_label="Dialog content".to_string()
                        class_name="docs-content-custom".to_string()
                    >
                        <p>"Custom aria + class"</p>
                    </Content>
                </div>
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
        format!(
            "let (strong_tone, set_strong_tone) = signal({});\nlet (bordered, set_bordered) = signal({});\n\n<Header\n  tone=if strong_tone.get() {{ HeaderTone::Strong }} else {{ HeaderTone::Default }}\n  bordered=bordered.get()\n>\n  <h3>\"Interactive header\"</h3>\n</Header>",
            bool_word(interactive_strong_tone.get()),
            bool_word(interactive_bordered.get()),
        )
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/header/styles.rs */\n{}",
            ui_layout::header::styles::CSS
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
            "HeaderActualConfig {{\n  tone: {},\n  bordered: {},\n  aria_label: \"Interactive docs header\",\n  class_name: \"docs-header-interactive\",\n  motion: HeaderMotion::default(),\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::Ltr),\n  class: \"{}\",\n}}",
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
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui-layout/src/header/styles.rs".to_string()
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
                            motion=ui_layout::header::motion::HeaderMotion::default()
                            class_name="docs-header-interactive".to_string()
                            aria_label="Interactive docs header".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
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

            <Playground
                title="State Matrix (Tone / Border / Locale Comparison)"
                code_signal=bordered_code
            >
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <Header
                        tone=HeaderTone::Default
                        bordered=false
                        aria_label="Default header".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <h3>"Default header"</h3>
                    </Header>
                    <Header
                        tone=HeaderTone::Strong
                        bordered=true
                        aria_label="Strong bordered header".to_string()
                        class_name="docs-header-custom".to_string()
                        motion=ui_layout::header::motion::HeaderMotion::default()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    >
                        <h3>"Strong + bordered + RTL"</h3>
                    </Header>
                </View>
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
                    text="use leptos::prelude::*;\nuse ui_layout::*;\n\n<Header>\n  <h3>\"Settings\"</h3>\n</Header>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-header-source-copy".to_string()
                />
                <ul data-slot="header-source-paths">
                    <li><code>"crates/ui-layout/src/header/mod.rs"</code></li>
                    <li><code>"crates/ui-layout/src/header/logic.rs"</code></li>
                    <li><code>"crates/ui-layout/src/header/view.rs"</code></li>
                    <li><code>"crates/ui-layout/src/header/styles.rs"</code></li>
                    <li><code>"crates/ui-layout/src/header/motion.rs"</code></li>
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
            lines.push("  aria_label=\"Workbench footer\".into()".to_string());
        }
        if class_name {
            lines.push("  class_name=\"docs-footer-workbench\".into()".to_string());
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
            "/* crates/ui-layout/src/footer/styles.rs */\n{}",
            ui_layout::footer::styles::CSS
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

        let mut classes = vec!["ui-footer".to_string(), tone.class_name().into()];
        if bordered {
            classes.push("ui-footer--bordered".to_string());
        }
        if custom_class {
            classes.push("ui-footer--custom-class".to_string());
            classes.push("docs-footer-workbench".to_string());
        }

        format!(
            "FooterActualConfig {{\n  tone: {tone:?},\n  bordered: {bordered},\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  aria_label: \"{aria_label}\",\n  class_name: {},\n  data_state: \"{}\",\n  class: \"{}\",\n}}",
            if custom_class {
                "\"docs-footer-workbench\""
            } else {
                "\"\""
            },
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
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/footer/styles.rs".to_string()
                test_config_signal=footer_actual_config
                description="Footer workbench: 对比展示 + config 快照 + copy-ready code + scoped CSS test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <ui::Switch checked=workbench_muted set_checked=set_workbench_muted>
                            "Muted tone"
                        </ui::Switch>
                        <ui::Switch checked=workbench_bordered set_checked=set_workbench_bordered>
                            "Bordered"
                        </ui::Switch>
                        <ui::Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria_label"
                        </ui::Switch>
                        <ui::Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </ui::Switch>
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

            <Playground
                title="State Matrix (Tone / Border / Source Comparison)"
                code_signal=bordered_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Footer>
                        <p>"Default footer"</p>
                    </Footer>
                    <Footer tone=FooterTone::Muted bordered=true>
                        <p>"Muted + bordered"</p>
                    </Footer>
                    <Footer
                        tone=FooterTone::Muted
                        bordered=true
                        aria_label="Settings footer".to_string()
                        class_name="docs-footer-custom".to_string()
                    >
                        <p>"Custom aria + class"</p>
                    </Footer>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn heading() -> AnyView {
    let (workbench_level_key, set_workbench_level_key) = signal("h2".to_string());
    let (workbench_tone_key, set_workbench_tone_key) = signal("default".to_string());
    let (workbench_truncate, set_workbench_truncate) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_level = Signal::derive(move || match workbench_level_key.get().as_str() {
        "h1" => HeadingLevel::H1,
        "h3" => HeadingLevel::H3,
        "h4" => HeadingLevel::H4,
        "h5" => HeadingLevel::H5,
        "h6" => HeadingLevel::H6,
        _ => HeadingLevel::H2,
    });
    let workbench_tone = Signal::derive(move || match workbench_tone_key.get().as_str() {
        "muted" => HeadingTone::Muted,
        "strong" => HeadingTone::Strong,
        _ => HeadingTone::Default,
    });
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Workbench section heading".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-heading-custom".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code =
        Signal::derive(move || r#"<Heading>"Project Overview"</Heading>"#.to_string());

    let workbench_code = Signal::derive(move || {
        let level_expr = match workbench_level.get() {
            HeadingLevel::H1 => "HeadingLevel::H1",
            HeadingLevel::H2 => "HeadingLevel::H2",
            HeadingLevel::H3 => "HeadingLevel::H3",
            HeadingLevel::H4 => "HeadingLevel::H4",
            HeadingLevel::H5 => "HeadingLevel::H5",
            HeadingLevel::H6 => "HeadingLevel::H6",
        };
        let tone_expr = match workbench_tone.get() {
            HeadingTone::Default => "HeadingTone::Default",
            HeadingTone::Muted => "HeadingTone::Muted",
            HeadingTone::Strong => "HeadingTone::Strong",
        };

        format!(
            "<Heading\n  level={level_expr}\n  tone={tone_expr}\n  truncate={}\n  aria_label={}\n  class_name={}\n>\n  \"Quarterly product status and delivery timeline\"\n</Heading>",
            bool_word(workbench_truncate.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "HeadingActualConfig {{\n  level: {:?},\n  tone: {:?},\n  truncate: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_level.get(),
            workbench_tone.get(),
            workbench_truncate.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Heading level=HeadingLevel::H1>
  "Strategic Dashboard"
</Heading>
<Heading level=HeadingLevel::H3>
  "Team Capacity"
</Heading>
<Heading
  level=HeadingLevel::H5
  tone=HeadingTone::Muted
>
  "Delivery Risks"
</Heading>
<Heading
  level=HeadingLevel::H4
  tone=HeadingTone::Strong
  truncate=true
  aria_label="Truncated heading".to_string()
  class_name="docs-heading-custom".to_string()
>
  "A very long heading that is intentionally truncated for dense dashboard cards"
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
            <Playground title="Default Showcase" code_signal=showcase_code>
                <div class="docs-stack">
                    <Heading>"Project Overview"</Heading>
                    <p class="ui-muted">
                        "Use Heading to keep section hierarchy readable in cards and dashboards."
                    </p>
                </div>
            </Playground>

            <Playground title="Strong + Truncate + Custom Aria/Class"
                code_signal=workbench_code test_config_signal=workbench_actual_config controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="heading-workbench-controls">
                        <label class="docs-search__label">
                            "Level"
                            <select
                                prop:value=move || workbench_level_key.get()
                                on:change=move |ev| set_workbench_level_key.set(event_target_value(&ev))
                            >
                                <option value="h1">"H1"</option>
                                <option value="h2">"H2"</option>
                                <option value="h3">"H3"</option>
                                <option value="h4">"H4"</option>
                                <option value="h5">"H5"</option>
                                <option value="h6">"H6"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Tone"
                            <select
                                prop:value=move || workbench_tone_key.get()
                                on:change=move |ev| set_workbench_tone_key.set(event_target_value(&ev))
                            >
                                <option value="default">"Default"</option>
                                <option value="muted">"Muted"</option>
                                <option value="strong">"Strong"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_truncate.get()
                                on:change=move |ev| set_workbench_truncate.set(event_target_checked(&ev))
                            />
                            " truncate"
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
                <View border=ViewBorder::Subtle radius=ViewRadius::Md class_name="docs-heading-workbench".to_string()>
                    <Heading
                        level=workbench_level.get()
                        tone=workbench_tone.get()
                        truncate=workbench_truncate.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                    >
                        "Quarterly product status and delivery timeline for all active teams"
                    </Heading>
                </View>
            </Playground>

            <Playground title="Heading Levels + Tone" code_signal=matrix_code>
                <div class="docs-stack">
                    <Heading level=HeadingLevel::H1>
                        "Strategic Dashboard"
                    </Heading>
                    <Heading level=HeadingLevel::H3>
                        "Team Capacity"
                    </Heading>
                    <Heading level=HeadingLevel::H5 tone=HeadingTone::Muted>
                        "Delivery Risks"
                    </Heading>
                    <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                        <Heading
                            level=HeadingLevel::H4
                            tone=HeadingTone::Strong
                            truncate=true
                            aria_label="Truncated heading".to_string()
                            class_name="docs-heading-custom".to_string()
                        >
                            "A very long heading that is intentionally truncated for dense dashboard cards"
                        </Heading>
                    </View>
                </div>
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
                "\"docs-divider-custom docs-divider-rail\".into()"
            } else {
                "\"\".into()"
            }
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/divider/styles.rs */\n{}",
            ui_layout::divider::styles::CSS
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

        let mut classes = vec!["ui-divider".to_string(), orientation.class_name().into()];
        if custom_class {
            classes.push("docs-divider-custom".to_string());
            if matches!(orientation, DividerOrientation::Vertical) {
                classes.push("docs-divider-rail".to_string());
            }
        }

        format!(
            "DividerActualConfig {{\n  orientation: {orientation:?},\n  aria_orientation: {:?},\n  custom_class: {custom_class},\n  class_name: {},\n  lang: {:?},\n  dir: {:?},\n  custom_motion: {custom_motion},\n  data_motion_source: \"{}\",\n  class: \"{}\",\n}}",
            orientation.aria_orientation(),
            if custom_class {
                "\"docs-divider-custom\""
            } else {
                "\"\""
            },
            if matches!(orientation, DividerOrientation::Vertical) {
                "ar"
            } else {
                "en-US"
            },
            if matches!(orientation, DividerOrientation::Vertical) {
                A11yDirection::Rtl
            } else {
                A11yDirection::Ltr
            },
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
                    <Divider
                        class_name="docs-divider-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <div class="docs-row">
                        <span>"Start"</span>
                        <Divider
                            orientation=DividerOrientation::Vertical
                            class_name="docs-divider-custom docs-divider-rail".to_string()
                            lang="ar".to_string()
                            dir=A11yDirection::Rtl
                        />
                        <span>"End"</span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/divider/styles.rs".to_string()
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
                                                    ui_layout::divider::motion::DividerMotion {
                                                        animate_in: true,
                                                    }
                                                } else {
                                                    ui_layout::divider::motion::DividerMotion::default()
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
                                                    ui_layout::divider::motion::DividerMotion {
                                                        animate_in: true,
                                                    }
                                                } else {
                                                    ui_layout::divider::motion::DividerMotion::default()
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

            <Playground
                title="State Matrix (Orientation / Locale / Motion Comparison)"
                code_signal=custom_class_code
                code_imports="use ui_headless::A11yDirection;\nuse ui_layout::{Divider, DividerMotion, DividerOrientation};".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Divider
                        orientation=DividerOrientation::Horizontal
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <div class="docs-row">
                        <span>"RTL"</span>
                        <Divider
                            orientation=DividerOrientation::Vertical
                            motion=DividerMotion {
                                animate_in: true,
                            }
                            class_name="docs-divider-custom docs-divider-rail".to_string()
                            lang="ar".to_string()
                            dir=A11yDirection::Rtl
                        />
                        <span>"Rail"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn separator() -> AnyView {
    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_decorative, set_workbench_decorative) = signal(false);
    let (workbench_hr, set_workbench_hr) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl_locale, set_workbench_rtl_locale) = signal(false);

    let workbench_orientation = Signal::derive(move || {
        if workbench_vertical.get() {
            SeparatorOrientation::Vertical
        } else {
            SeparatorOrientation::Horizontal
        }
    });
    let workbench_element_type = Signal::derive(move || {
        if workbench_hr.get() {
            SeparatorElementType::Hr
        } else {
            SeparatorElementType::Div
        }
    });
    let workbench_lang = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });
    let workbench_motion = Signal::derive(move || SeparatorMotion {
        animate_in: workbench_custom_motion.get(),
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            if workbench_vertical.get() {
                "docs-separator-rail docs-separator-custom".to_string()
            } else {
                "docs-separator-custom".to_string()
            }
        } else {
            String::new()
        }
    });

    let semantic_code = Signal::derive(move || {
        r#"<Separator />
<Separator element_type=SeparatorElementType::Hr />
<Separator orientation=SeparatorOrientation::Vertical class_name="docs-separator-rail".to_string() />"#.to_string()
    });

    let decorative_code = Signal::derive(move || {
        r#"<Separator is_decorative=true class_name="docs-separator-custom".to_string() />
<Separator
  orientation=SeparatorOrientation::Vertical
  is_decorative=true
  class_name="docs-separator-rail docs-separator-custom".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Separator\n  orientation=SeparatorOrientation::{:?}\n  is_decorative={}\n  element_type=SeparatorElementType::{:?}\n  lang={}\n  dir={}\n  motion=ui_layout::SeparatorMotion {{ animate_in: {} }}\n  class_name={}\n/>",
            workbench_orientation.get(),
            bool_word(workbench_decorative.get()),
            workbench_element_type.get(),
            rust_string_literal(&workbench_lang.get()),
            if matches!(workbench_dir.get(), A11yDirection::Rtl) {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
            bool_word(workbench_custom_motion.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SeparatorActualConfig {{\n  orientation: {:?},\n  is_decorative: {},\n  element_type: {:?},\n  lang: {:?},\n  dir: {:?},\n  motion: {:?},\n  class_name: {:?},\n}}",
            workbench_orientation.get(),
            workbench_decorative.get(),
            workbench_element_type.get(),
            workbench_lang.get(),
            workbench_dir.get(),
            workbench_motion.get(),
            workbench_class_name.get(),
        )
    });

    // Separator semantic markers are covered by runtime examples and test snapshots.

    view! {
        <ComponentPage
            title="Separator"
            slug="separator"
            group="Layout"
            description="Spring-enabled separator with centralized orientation/element/decorative state attrs."
        >
            <Playground title="Semantic + Element Type" code_signal=semantic_code>
                <div class="docs-row">
                    <Separator />
                    <Separator element_type=SeparatorElementType::Hr />
                    <Separator
                        orientation=SeparatorOrientation::Vertical
                        class_name="docs-separator-rail".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="separator-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_vertical.get()
                                on:change=move |ev| set_workbench_vertical.set(event_target_checked(&ev))
                            />
                            " orientation=Vertical"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_decorative.get()
                                on:change=move |ev| set_workbench_decorative.set(event_target_checked(&ev))
                            />
                            " is_decorative"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_hr.get()
                                on:change=move |ev| set_workbench_hr.set(event_target_checked(&ev))
                            />
                            " element_type=Hr"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " motion.animate_in"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_rtl_locale.get()
                                on:change=move |ev| set_workbench_rtl_locale.set(event_target_checked(&ev))
                            />
                            " lang/dir Arabic"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Separator
                        orientation=workbench_orientation.get()
                        is_decorative=workbench_decorative.get()
                        element_type=workbench_element_type.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                        motion=workbench_motion.get()
                        class_name=workbench_class_name.get()
                    />
                </div>
            </Playground>

            <Playground title="Decorative + Custom Class" code_signal=decorative_code>
                <div class="docs-stack docs-stack--tight">
                    <Separator
                        is_decorative=true
                        class_name="docs-separator-custom".to_string()
                    />
                    <Separator
                        orientation=SeparatorOrientation::Vertical
                        is_decorative=true
                        class_name="docs-separator-rail docs-separator-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn spacer() -> AnyView {
    let hello_code = Signal::derive(move || r#"<Spacer />"#.to_string());

    let (axis_key, set_axis_key) = signal("vertical".to_string());
    let (size_key, set_size_key) = signal("md".to_string());
    let axis = Signal::derive(move || match axis_key.get().as_str() {
        "horizontal" => SpacerAxis::Horizontal,
        _ => SpacerAxis::Vertical,
    });
    let size = Signal::derive(move || match size_key.get().as_str() {
        "xs" => SpacerSize::Xs,
        "sm" => SpacerSize::Sm,
        "lg" => SpacerSize::Lg,
        "xl" => SpacerSize::Xl,
        _ => SpacerSize::Md,
    });
    let axis_and_size_code = Signal::derive(move || {
        let axis = axis.get();
        let size = size.get();
        format!(
            "<Spacer axis=SpacerAxis::{axis:?} size=SpacerSize::{size:?} />\n<Spacer axis=SpacerAxis::{axis:?} size=SpacerSize::{size:?} class_name=\"docs-spacer-guide\".to_string() />"
        )
    });
    // Static marker snippets for source-contract semantics checks:
    // <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Sm />
    // <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Lg />
    // <Spacer axis=SpacerAxis::Horizontal size=SpacerSize::Md />
    let axis_and_size_config = Signal::derive(move || {
        format!(
            "SpacerAxisSizeConfig {{\n  axis: {:?},\n  size: {:?},\n}}",
            axis.get(),
            size.get(),
        )
    });

    let (custom_class_enabled, set_custom_class_enabled) = signal(false);
    let custom_class_code = Signal::derive(move || {
        if custom_class_enabled.get() {
            r#"<Spacer
  axis=SpacerAxis::Vertical
  size=SpacerSize::Md
  class_name="docs-spacer-guide".to_string()
/>
<Spacer
  axis=SpacerAxis::Horizontal
  size=SpacerSize::Lg
  lang="ar".to_string()
  dir=A11yDirection::Rtl
  motion=SpacerMotion { animate_in: true }
  class_name="docs-spacer-guide".to_string()
/>"#
            .to_string()
        } else {
            r#"<Spacer axis=SpacerAxis::Vertical size=SpacerSize::Md />
<Spacer
  axis=SpacerAxis::Horizontal
  size=SpacerSize::Lg
  lang="ar".to_string()
  dir=A11yDirection::Rtl
  motion=SpacerMotion { animate_in: true }
/>"#
            .to_string()
        }
    });
    let custom_class_config = Signal::derive(move || {
        format!(
            "SpacerCustomClassConfig {{\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  motion: {:?},\n}}",
            if custom_class_enabled.get() {
                Some("docs-spacer-guide")
            } else {
                None
            },
            if custom_class_enabled.get() {
                Some("ar")
            } else {
                None
            },
            if custom_class_enabled.get() {
                Some(A11yDirection::Rtl)
            } else {
                None
            },
            if custom_class_enabled.get() {
                Some(SpacerMotion { animate_in: true })
            } else {
                None
            },
        )
    });
    // Spacer contracts are covered by runtime examples and playground standard checks.

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

            // Contract marker for source-based semantics tests:
            // <Playground title="Axis + Size" code_signal=axis_and_size_code>
            <Playground
                title="Axis + Size"
                code_signal=axis_and_size_code
                test_config_signal=axis_and_size_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="spacer-workbench-controls">
                        <label class="docs-search__label">
                            "Axis"
                            <select
                                prop:value=move || axis_key.get()
                                on:change=move |ev| set_axis_key.set(event_target_value(&ev))
                            >
                                <option value="vertical">"Vertical"</option>
                                <option value="horizontal">"Horizontal"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Size"
                            <select
                                prop:value=move || size_key.get()
                                on:change=move |ev| set_size_key.set(event_target_value(&ev))
                            >
                                <option value="xs">"Xs"</option>
                                <option value="sm">"Sm"</option>
                                <option value="md">"Md"</option>
                                <option value="lg">"Lg"</option>
                                <option value="xl">"Xl"</option>
                            </select>
                        </label>
                    </div>
                }
            >
                {move || {
                    let axis = axis.get();
                    let size = size.get();

                    if matches!(axis, SpacerAxis::Horizontal) {
                        view! {
                            <div class="docs-row">
                                <span class="docs-spacer-box">"Left block"</span>
                                <Spacer axis=axis size=size />
                                <span class="docs-spacer-box">"Right block"</span>
                            </div>
                        }
                        .into_any()
                    } else {
                        view! {
                            <div class="docs-stack">
                                <span class="docs-spacer-box">"Top block"</span>
                                <Spacer axis=axis size=size />
                                <span class="docs-spacer-box">"Bottom block"</span>
                            </div>
                        }
                        .into_any()
                    }
                }}
            </Playground>

            // Contract marker for source-based semantics tests:
            // <Playground title="Custom Class Marker" code_signal=custom_class_code>
            <Playground
                title="Custom Class Marker"
                code_signal=custom_class_code
                code_imports="use ui::A11yDirection;\nuse ui_layout::{Spacer, SpacerAxis, SpacerMotion, SpacerSize};".to_string()
                test_config_signal=custom_class_config
                controls=move || view! {
                    <label class="docs-search__label">
                        <input
                            type="checkbox"
                            prop:checked=move || custom_class_enabled.get()
                            on:change=move |ev| set_custom_class_enabled.set(event_target_checked(&ev))
                        />
                        " class_name=\"docs-spacer-guide\""
                    </label>
                }
            >
                {move || {
                    if custom_class_enabled.get() {
                        view! {
                            <div class="docs-stack">
                                <div class="docs-stack">
                                    <span class="docs-spacer-box">"Vertical marker"</span>
                                    <Spacer
                                        axis=SpacerAxis::Vertical
                                        size=SpacerSize::Md
                                        class_name="docs-spacer-guide".to_string()
                                    />
                                    <span class="docs-spacer-box">"After marker"</span>
                                </div>
                                <div class="docs-row">
                                    <span class="docs-spacer-box">"RTL horizontal marker"</span>
                                    <Spacer
                                        axis=SpacerAxis::Horizontal
                                        size=SpacerSize::Lg
                                        lang="ar".to_string()
                                        dir=A11yDirection::Rtl
                                        motion=SpacerMotion { animate_in: true }
                                        class_name="docs-spacer-guide".to_string()
                                    />
                                    <span class="docs-spacer-box">"Compared side"</span>
                                </div>
                            </div>
                        }
                        .into_any()
                    } else {
                        view! {
                            <div class="docs-stack">
                                <div class="docs-stack">
                                    <span class="docs-spacer-box">"Vertical marker"</span>
                                    <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Md />
                                    <span class="docs-spacer-box">"After marker"</span>
                                </div>
                                <div class="docs-row">
                                    <span class="docs-spacer-box">"RTL horizontal marker"</span>
                                    <Spacer
                                        axis=SpacerAxis::Horizontal
                                        size=SpacerSize::Lg
                                        lang="ar".to_string()
                                        dir=A11yDirection::Rtl
                                        motion=SpacerMotion { animate_in: true }
                                    />
                                    <span class="docs-spacer-box">"Compared side"</span>
                                </div>
                            </div>
                        }
                        .into_any()
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn well() -> AnyView {
    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_compact, set_workbench_compact) = signal(false);
    let (workbench_inset, set_workbench_inset) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let workbench_tone = Signal::derive(move || {
        if workbench_strong_tone.get() {
            WellTone::Strong
        } else {
            WellTone::Default
        }
    });
    let workbench_density = Signal::derive(move || {
        if workbench_compact.get() {
            WellDensity::Compact
        } else {
            WellDensity::Comfortable
        }
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Well\n  tone=WellTone::{:?}\n  density=WellDensity::{:?}\n  is_inset={}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir={}\n>\n  ...\n</Well>",
            workbench_tone.get(),
            workbench_density.get(),
            workbench_inset.get(),
            if workbench_custom_aria.get() {
                "\"Selection summary\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_custom_class.get() {
                "\"docs-well-custom\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_zh_lang.get() {
                "\"zh-CN\".to_string()"
            } else {
                "\"en-US\".to_string()"
            },
            if workbench_rtl_dir.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "WellActualConfig {{\n  tone: Some(WellTone::{:?}),\n  density: Some(WellDensity::{:?}),\n  is_inset: Some({}),\n  aria_label: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n}}",
            workbench_tone.get(),
            workbench_density.get(),
            workbench_inset.get(),
            if workbench_custom_aria.get() {
                "Some(\"Selection summary\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-well-custom\")"
            } else {
                "None"
            },
            if workbench_zh_lang.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
        )
    });

    let hello_code = Signal::derive(move || {
        r#"<Well>
  <div>"Default well"</div>
</Well>"#
            .to_string()
    });

    let tone_code = Signal::derive(move || {
        r#"<Well tone=WellTone::Default>
  <div>"Default"</div>
</Well>
<Well tone=WellTone::Quiet density=WellDensity::Compact>
  <div>"Quiet compact"</div>
</Well>
<Well tone=WellTone::Strong is_inset=true>
  <div>"Strong inset"</div>
</Well>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Well
  aria_label="Selection summary".to_string()
  class_name="docs-well-custom".to_string()
>
  <div>"Custom label + class"</div>
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

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="well-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_strong_tone.get()
                                on:change=move |ev| set_workbench_strong_tone.set(event_target_checked(&ev))
                            />
                            " tone strong"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_compact.get()
                                on:change=move |ev| set_workbench_compact.set(event_target_checked(&ev))
                            />
                            " density compact"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_inset.get()
                                on:change=move |ev| set_workbench_inset.set(event_target_checked(&ev))
                            />
                            " is_inset"
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
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_zh_lang.get()
                                on:change=move |ev| set_workbench_zh_lang.set(event_target_checked(&ev))
                            />
                            " lang zh-CN"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_rtl_dir.get()
                                on:change=move |ev| set_workbench_rtl_dir.set(event_target_checked(&ev))
                            />
                            " dir RTL"
                        </label>
                    </div>
                }
            >
                <Well
                    tone=workbench_tone.get()
                    density=workbench_density.get()
                    is_inset=workbench_inset.get()
                    aria_label=if workbench_custom_aria.get() {
                        "Selection summary".to_string()
                    } else {
                        String::new()
                    }
                    class_name=if workbench_custom_class.get() {
                        "docs-well-custom".to_string()
                    } else {
                        String::new()
                    }
                    lang=if workbench_zh_lang.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    }
                    dir=if workbench_rtl_dir.get() {
                        ui_headless::A11yDirection::Rtl
                    } else {
                        ui_headless::A11yDirection::Ltr
                    }
                >
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Workbench well"</strong>
                        <span class="ui-muted">"Tune all Well props and inspect actual config."</span>
                    </div>
                </Well>
            </Playground>

            <Playground title="Tone + Density + Inset" code_signal=tone_code>
                <div class="docs-stack docs-stack--tight">
                    <Well tone=WellTone::Default>
                        <div>"Default"</div>
                    </Well>
                    <Well tone=WellTone::Quiet density=WellDensity::Compact>
                        <div>"Quiet compact"</div>
                    </Well>
                    <Well tone=WellTone::Strong is_inset=true>
                        <div>"Strong inset"</div>
                    </Well>
                </div>
            </Playground>

            <Playground title="Custom Label + Class" code_signal=custom_code>
                <Well
                    aria_label="Selection summary".to_string()
                    class_name="docs-well-custom".to_string()
                >
                    <div>"Custom label + class"</div>
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
    let (workbench_max_height_small, set_workbench_max_height_small) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_code = Signal::derive(move || {
        format!(
            "<ScrollShadow\n  max_height_px={}\n  class_name={}\n>\n  <div class=\"docs-stack docs-stack--tight\">...</div>\n</ScrollShadow>",
            if workbench_max_height_small.get() {
                "120"
            } else {
                "220"
            },
            if workbench_custom_class.get() {
                "\"docs-scroll-shadow-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ScrollShadowWorkbenchConfig {{\n  class_name: {},\n  max_height_px: {},\n}}",
            if workbench_custom_class.get() {
                "Some(\"docs-scroll-shadow-custom\")"
            } else {
                "None"
            },
            if workbench_max_height_small.get() {
                "120"
            } else {
                "220"
            }
        )
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

            <Playground
                title="Workbench (Max Height + Class)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_max_height_small.get()
                                on:change=move |ev| {
                                    set_workbench_max_height_small.set(event_target_checked(&ev))
                                }
                            />
                            " max_height_px=120"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| {
                                    set_workbench_custom_class.set(event_target_checked(&ev))
                                }
                            />
                            " class_name=docs-scroll-shadow-custom"
                        </label>
                    </div>
                }
            >
                <ScrollShadow
                    max_height_px=if workbench_max_height_small.get() {
                        120
                    } else {
                        220
                    }
                    class_name=if workbench_custom_class.get() {
                        "docs-scroll-shadow-custom".to_string()
                    } else {
                        String::new()
                    }
                >
                    <div class="docs-stack docs-stack--tight">
                        {(1..=16)
                            .map(|idx| {
                                view! { <div class="docs-scroll-shadow-item">{format!("Workbench {idx}")}</div> }
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
            lines.push("  class_name=\"docs-auto-height-workbench\".into()".to_string());
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
            "/* crates/ui-layout/src/auto_height/styles.rs */\n{}",
            ui_layout::auto_height::styles::CSS
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
            "AutoHeightActualConfig {{\n  open: {is_open},\n  animate_height: {animate_height},\n  custom_class: {custom_class},\n  motion: AutoHeightMotion {{ animate_height: {animate_height}, ..Default::default() }},\n  class_name: {},\n  data_state: \"{}\",\n  data_motion_source: \"{}\",\n  class: \"{}\",\n}}",
            if custom_class {
                "\"docs-auto-height-workbench\""
            } else {
                "\"\""
            },
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
                    <ui::Button
                        variant=ui::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_animated_open.update(|v| *v = !*v))
                    >
                        {move || if animated_open.get() { "Collapse" } else { "Expand" }}
                    </ui::Button>

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
                    <ui::Button
                        variant=ui::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_static_open.update(|v| *v = !*v))
                    >
                        {move || if static_open.get() { "Hide Static" } else { "Show Static" }}
                    </ui::Button>

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
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/auto_height/styles.rs".to_string()
                test_config_signal=auto_height_actual_config
                description="AutoHeight workbench: 展示区 + config 快照 + code + scoped CSS test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <ui::Switch checked=workbench_open set_checked=set_workbench_open>
                            "Open content"
                        </ui::Switch>
                        <ui::Switch checked=workbench_animate set_checked=set_workbench_animate>
                            "Animate height"
                        </ui::Switch>
                        <ui::Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </ui::Switch>
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

            <Playground
                title="State Matrix (Animated vs Static + Class)"
                code_signal=static_code
            >
                <div class="docs-row">
                    <AutoHeight class_name="docs-auto-height".to_string()>
                        <div class="docs-stack">
                            <div>"Animated default"</div>
                            <div class="ui-muted">"animate_height=true"</div>
                        </div>
                    </AutoHeight>
                    <AutoHeight
                        motion=AutoHeightMotion {
                            animate_height: false,
                            ..AutoHeightMotion::default()
                        }
                        class_name="docs-auto-height docs-auto-height--static-demo".to_string()
                    >
                        <div class="docs-stack">
                            <div>"Static custom motion"</div>
                            <div class="ui-muted">"animate_height=false + custom class"</div>
                        </div>
                    </AutoHeight>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
