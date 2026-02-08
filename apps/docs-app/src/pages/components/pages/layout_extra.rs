use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Grid, GridAlign, GridColumns, GridGap, GridJustify, GridRows, View, ViewBackground, ViewBorder,
    ViewPadding, ViewRadius,
};

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
