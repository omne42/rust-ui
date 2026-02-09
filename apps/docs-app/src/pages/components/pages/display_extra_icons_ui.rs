use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{IconsUi, IconsUiSize, IconsUiTone, IconsetGlyph};

pub(super) fn icons_ui() -> AnyView {
    let defaults_code = r#"<IconsUi icon="check".to_string() size=IconsUiSize::Md tone=IconsUiTone::Accent decorative=false />
<IconsUi icon="close".to_string() size=IconsUiSize::Md tone=IconsUiTone::Danger decorative=false />"#;

    let custom_code = r#"<IconsUi
  icon="ui:save".to_string()
  glyphs=vec![IconsetGlyph::new("ui:save", "💾").with_aria_label("UI Save")]
  size=IconsUiSize::Lg
  tone=IconsUiTone::Default
  decorative=false
  class_name="docs-icons-ui-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="IconsUi"
            slug="icons-ui"
            group="Display"
            description="Spectrum-compatible icons-ui wrapper with built-in UI icon registry defaults, namespace normalization, and Iconset accessibility/source-state contracts."
        >
            <Playground title="Built-in UI Glyphs" code=defaults_code>
                <div class="docs-row">
                    <IconsUi
                        icon="check".to_string()
                        size=IconsUiSize::Md
                        tone=IconsUiTone::Accent
                        decorative=false
                    />
                    <IconsUi
                        icon="close".to_string()
                        size=IconsUiSize::Md
                        tone=IconsUiTone::Danger
                        decorative=false
                    />
                </div>
            </Playground>

            <Playground title="Custom Registry Extension" code=custom_code>
                <div class="docs-row">
                    <IconsUi
                        icon="ui:save".to_string()
                        glyphs=vec![IconsetGlyph::new("ui:save", "💾").with_aria_label("UI Save")]
                        size=IconsUiSize::Lg
                        tone=IconsUiTone::Default
                        decorative=false
                        class_name="docs-icons-ui-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
