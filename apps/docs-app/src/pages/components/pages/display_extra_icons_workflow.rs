use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{IconsWorkflow, IconsWorkflowSize, IconsWorkflowTone, IconsetGlyph};

pub(super) fn icons_workflow() -> AnyView {
    let defaults_code = r#"<IconsWorkflow icon="success".to_string() size=IconsWorkflowSize::Md tone=IconsWorkflowTone::Accent decorative=false />
<IconsWorkflow icon="warning".to_string() size=IconsWorkflowSize::Md tone=IconsWorkflowTone::Danger decorative=false />"#;

    let custom_code = r#"<IconsWorkflow
  icon="workflow:deploy".to_string()
  glyphs=vec![IconsetGlyph::new("workflow:deploy", "🚀").with_aria_label("Workflow Deploy")]
  size=IconsWorkflowSize::Lg
  tone=IconsWorkflowTone::Default
  decorative=false
  class_name="docs-icons-workflow-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="IconsWorkflow"
            slug="icons-workflow"
            group="Display"
            description="Spectrum-compatible icons-workflow wrapper with workflow namespace normalization, built-in workflow glyph defaults, and Iconset accessibility/source-state contracts."
        >
            <Playground title="Built-in Workflow Glyphs" code=defaults_code>
                <div class="docs-row">
                    <IconsWorkflow
                        icon="success".to_string()
                        size=IconsWorkflowSize::Md
                        tone=IconsWorkflowTone::Accent
                        decorative=false
                    />
                    <IconsWorkflow
                        icon="warning".to_string()
                        size=IconsWorkflowSize::Md
                        tone=IconsWorkflowTone::Danger
                        decorative=false
                    />
                </div>
            </Playground>

            <Playground title="Custom Workflow Extension" code=custom_code>
                <div class="docs-row">
                    <IconsWorkflow
                        icon="workflow:deploy".to_string()
                        glyphs=vec![IconsetGlyph::new("workflow:deploy", "🚀").with_aria_label("Workflow Deploy")]
                        size=IconsWorkflowSize::Lg
                        tone=IconsWorkflowTone::Default
                        decorative=false
                        class_name="docs-icons-workflow-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
