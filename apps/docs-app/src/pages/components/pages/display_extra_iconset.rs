use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Iconset, IconsetGlyph, IconsetSize, IconsetTone};

pub(super) fn iconset() -> AnyView {
    let registry_code = r#"<Iconset
  icon="workflow:check".to_string()
  glyphs=vec![
    IconsetGlyph::new("workflow:check", "✓").with_aria_label("Workflow Check"),
    IconsetGlyph::new("workflow:alert", "⚠").with_aria_label("Workflow Alert"),
  ]
  size=IconsetSize::Md
  tone=IconsetTone::Accent
  decorative=false
/>"#;

    let fallback_code = r#"<Iconset
  icon="ui:unknown".to_string()
  iconset="ui".to_string()
  size=IconsetSize::Lg
  tone=IconsetTone::Muted
  decorative=false
  class_name="docs-iconset-custom".to_string()
/>"#;

    let workflow_glyphs = vec![
        IconsetGlyph::new("workflow:check", "✓").with_aria_label("Workflow Check"),
        IconsetGlyph::new("workflow:alert", "⚠").with_aria_label("Workflow Alert"),
    ];

    view! {
        <ComponentPage
            title="Iconset"
            slug="iconset"
            group="Display"
            description="Spectrum-compatible Iconset registry wrapper for namespace + icon-name resolution, composed on Icon accessibility contracts with stable source markers."
        >
            <Playground title="Registry Namespace Resolution" code=registry_code>
                <div class="docs-row">
                    <Iconset
                        icon="workflow:check".to_string()
                        glyphs=workflow_glyphs.clone()
                        size=IconsetSize::Md
                        tone=IconsetTone::Accent
                        decorative=false
                    />
                    <Iconset
                        icon="workflow:alert".to_string()
                        glyphs=workflow_glyphs
                        size=IconsetSize::Md
                        tone=IconsetTone::Danger
                        decorative=false
                    />
                </div>
            </Playground>

            <Playground title="Fallback + Source State" code=fallback_code>
                <div class="docs-row">
                    <Iconset
                        icon="ui:unknown".to_string()
                        iconset="ui".to_string()
                        size=IconsetSize::Lg
                        tone=IconsetTone::Muted
                        decorative=false
                        class_name="docs-iconset-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
