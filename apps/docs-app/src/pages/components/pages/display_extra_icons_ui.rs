use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{IconsUi, IconsUiSize, IconsUiTone, IconsetGlyph};

pub(super) fn icons_ui() -> AnyView {
    let defaults_code = Signal::derive(move || {
        r#"<IconsUi icon="check".to_string() size=IconsUiSize::Md tone=IconsUiTone::Accent decorative=false />
<IconsUi icon="close".to_string() size=IconsUiSize::Md tone=IconsUiTone::Danger decorative=false />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<IconsUi
  icon="ui:save".to_string()
  glyphs=vec![IconsetGlyph::new("ui:save", "💾").with_aria_label("UI Save")]
  size=IconsUiSize::Lg
  tone=IconsUiTone::Default
  decorative=false
  class_name="docs-icons-ui-custom".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<IconsUi
  icon="help".to_string()
  glyphs=vec![IconsetGlyph::new("ui:help", "?").with_aria_label("UI Help")]
  size=IconsUiSize::Lg
  tone=IconsUiTone::Muted
  decorative=false
  aria_label="Explicit UI help icon".to_string()
  class_name="docs-icons-ui-state".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="IconsUi"
            slug="icons-ui"
            group="Display"
            description="Spectrum-compatible icons-ui wrapper with built-in UI icon registry defaults, namespace normalization, and Iconset accessibility/source-state contracts."
        >
            <Playground title="Built-in UI Glyphs" code_signal=defaults_code>
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

            <Playground title="Custom Registry Extension" code_signal=custom_code>
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

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-icon-reference-source`, `data-aria-source`, `data-class-source`, `data-glyph-source`, `data-size-source`, and `data-tone-source`."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <IconsUi
                        icon="help".to_string()
                        glyphs=vec![
                            IconsetGlyph::new("ui:help", "?").with_aria_label("UI Help"),
                        ]
                        size=IconsUiSize::Lg
                        tone=IconsUiTone::Muted
                        decorative=false
                        aria_label="Explicit UI help icon".to_string()
                        class_name="docs-icons-ui-state".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
