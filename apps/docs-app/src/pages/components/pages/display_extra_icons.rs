use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Icons, IconsGlyph, IconsScale, IconsSet, IconsTone};

pub(super) fn icons() -> AnyView {
    let default_code = r#"<Icons name="check".to_string() set=IconsSet::Ui scale=IconsScale::Medium tone=IconsTone::Accent decorative=false />
<Icons name="workflow:warning".to_string() scale=IconsScale::Large tone=IconsTone::Danger decorative=false />"#;

    let custom_code = r#"<Icons
  name="workflow:deploy".to_string()
  set=IconsSet::Workflow
  scale=IconsScale::Large
  tone=IconsTone::Default
  glyphs=vec![IconsGlyph::new("workflow:deploy", "🚀").with_aria_label("Workflow Deploy")]
  decorative=false
  class_name="docs-icons-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Icons"
            slug="icons"
            group="Display"
            description="Spectrum-compatible `icons` package wrapper that maps medium/large scale and ui/workflow set selection onto IconsUi/IconsWorkflow with stable source-state contracts."
        >
            <Playground title="Medium + Large Set Selection" code=default_code>
                <div class="docs-row">
                    <Icons
                        name="check".to_string()
                        set=IconsSet::Ui
                        scale=IconsScale::Medium
                        tone=IconsTone::Accent
                        decorative=false
                    />
                    <Icons
                        name="workflow:warning".to_string()
                        scale=IconsScale::Large
                        tone=IconsTone::Danger
                        decorative=false
                    />
                </div>
            </Playground>

            <Playground title="Custom Workflow Glyph Extension" code=custom_code>
                <div class="docs-row">
                    <Icons
                        name="workflow:deploy".to_string()
                        set=IconsSet::Workflow
                        scale=IconsScale::Large
                        tone=IconsTone::Default
                        glyphs=vec![IconsGlyph::new("workflow:deploy", "🚀").with_aria_label("Workflow Deploy")]
                        decorative=false
                        class_name="docs-icons-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
