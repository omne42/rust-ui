use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AutoHeight, AutoHeightMotion, Card, CardVariant, Content, ContentTone, Divider,
    DividerOrientation, Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexWrap, Footer,
    FooterTone, Header, HeaderTone, Heading, HeadingLevel, HeadingTone, ScrollShadow, Separator,
    SeparatorElementType, SeparatorOrientation, Spacer, SpacerAxis, SpacerSize, View,
    ViewBackground, ViewBorder, ViewElement, ViewPadding, ViewRadius, ViewShadow, Well,
    WellDensity, WellTone,
};

pub(super) fn card() -> AnyView {
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
            description="General-purpose Spectrum-style container with centralized surface token state and stable data markers."
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

    view! {
        <ComponentPage
            title="Flex"
            slug="flex"
            group="Layout"
            description="Spectrum-style flex layout primitive with centralized direction/wrap/alignment/gap normalization and stable data-state contracts."
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn header() -> AnyView {
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
    <p>"Header above content, matching Spectrum container semantics."</p>
  </Content>
</View>"#
            .to_string()
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
                        <p>"Header above content, matching Spectrum container semantics."</p>
                    </Content>
                </View>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn footer() -> AnyView {
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
            description="Spectrum-style semantic heading (`<h1>`..`<h6>`) with centralized level/tone/truncate contracts."
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

    view! {
        <ComponentPage
            title="Divider"
            slug="divider"
            group="Layout"
            description="A separator primitive with centralized orientation state attrs and Spectrum-style styling markers."
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
        r#"<Separator decorative=true />
<Separator
  decorative=true
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
                    <Separator decorative=true class_name="docs-separator-custom".to_string() />

                    <div class="docs-row">
                        <span>"Start"</span>
                        <Separator
                            decorative=true
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
            description="A pure spacing primitive with centralized axis/size state attrs for Spectrum-style styling contracts."
        >
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
    let tone_code = Signal::derive(move || {
        r#"<Well tone=WellTone::Default>
  <div>"Default well"</div>
</Well>
<Well tone=WellTone::Quiet density=WellDensity::Compact>
  <div>"Quiet compact well"</div>
</Well>
<Well tone=WellTone::Strong inset=true>
  <div>"Strong inset well"</div>
</Well>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Well
  tone=WellTone::Strong
  inset=true
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

                    <Well tone=WellTone::Strong inset=true>
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
                    inset=true
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn ui_root() -> AnyView {
    let usage_code = Signal::derive(move || {
        r#"use ui_components::{UiRoot, Theme};

let theme = Signal::derive(|| Theme::dark());

<UiRoot theme=theme safe_area=true>
  // your app
</UiRoot>"#
            .to_string()
    });

    let contract_code = Signal::derive(move || {
        r#"<UiRoot ...>
  // wrapper attrs:
  // data-slot="ui-root"
  // data-theme-scheme="light|dark"
  // data-state="default|safe-area"
  // data-safe-area="true" (optional)
</UiRoot>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="UiRoot"
            slug="ui-root"
            group="Layout"
            description="Provider that injects theme tokens + layered component CSS and exposes stable root state attrs."
        >
            <Playground title="Usage" code_signal=usage_code>
                <div class="docs-stack">
                    <div class="docs-ui-root-note">
                        "This docs app already mounts a global UiRoot at startup."
                    </div>
                    <div class="docs-ui-root-note">
                        "UiRoot injects BASE_CSS + theme CSS variables + component CSS in one place."
                    </div>
                    <div class="ui-muted">
                        "safe_area=true adds the safe-area inset contract used on mobile/WebView shells."
                    </div>
                </div>
            </Playground>

            <Playground title="State Contract" code_signal=contract_code>
                <div class="docs-stack">
                    <div class="docs-ui-root-note">"`data-slot=ui-root` for stable root targeting."</div>
                    <div class="docs-ui-root-note">"`data-theme-scheme` mirrors `Theme::scheme` (`light`/`dark`)."</div>
                    <div class="docs-ui-root-note">"`data-state` + `data-safe-area` describe safe-area mode."</div>
                    <div class="ui-muted">"Use these attrs to write app-level overrides without coupling to internal implementation details."</div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
