use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Iconset, IconsetGlyph, IconsetSize, IconsetTone};

pub(super) fn iconset() -> AnyView {
    let registry_code = Signal::derive(move || {
        r#"<Iconset
  icon="workflow:check".to_string()
  glyphs=vec![
    IconsetGlyph::new("workflow:check", "✓").with_aria_label("Workflow Check"),
    IconsetGlyph::new("workflow:alert", "⚠").with_aria_label("Workflow Alert"),
  ]
  size=IconsetSize::Md
  tone=IconsetTone::Accent
  decorative=false
/>"#
        .to_string()
    });

    let fallback_code = Signal::derive(move || {
        r#"<Iconset
  icon="ui:unknown".to_string()
  iconset="ui".to_string()
  size=IconsetSize::Lg
  tone=IconsetTone::Muted
  decorative=false
  class_name="docs-iconset-custom".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<Iconset
  icon="workflow:check".to_string()
  iconset="workflow".to_string()
  glyphs=vec![IconsetGlyph::new("workflow:check", "✓").with_aria_label("Registry Check")]
  size=IconsetSize::Lg
  tone=IconsetTone::Danger
  decorative=false
  aria_label="Explicit workflow check".to_string()
  class_name="docs-iconset-state".to_string()
/>"#
        .to_string()
    });

    let workflow_glyphs = vec![
        IconsetGlyph::new("workflow:check", "✓").with_aria_label("Workflow Check"),
        IconsetGlyph::new("workflow:alert", "⚠").with_aria_label("Workflow Alert"),
    ];

    view! {
        <ComponentPage
            title="Iconset"
            slug="iconset"
            group="Display"
            description="baseline-compatible Iconset registry wrapper for namespace + icon-name resolution, composed on Icon accessibility contracts with stable source markers."
        >
            <Playground title="Registry Namespace Resolution" code_signal=registry_code>
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

            <Playground title="Fallback + Source State" code_signal=fallback_code>
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

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-icon-source`, `data-iconset-source`, `data-label-source`, `data-class-source`, `data-size-source`, and `data-tone-source`."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <Iconset
                        icon="workflow:check".to_string()
                        iconset="workflow".to_string()
                        glyphs=vec![
                            IconsetGlyph::new("workflow:check", "✓")
                                .with_aria_label("Registry Check"),
                        ]
                        size=IconsetSize::Lg
                        tone=IconsetTone::Danger
                        decorative=false
                        aria_label="Explicit workflow check".to_string()
                        class_name="docs-iconset-state".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
