use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Surface, SurfaceElevation, SurfaceTone};

pub(super) fn surface() -> AnyView {
    let tone_code = r#"<Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>
  <div>"Default raised surface"</div>
</Surface>
<Surface tone=SurfaceTone::Subtle elevation=SurfaceElevation::Flat bordered=true>
  <div>"Subtle flat bordered surface"</div>
</Surface>
<Surface tone=SurfaceTone::Strong elevation=SurfaceElevation::Floating padded=false>
  <div>"Strong floating compact surface"</div>
</Surface>"#;

    let custom_code = r#"<Surface
  tone=SurfaceTone::Strong
  elevation=SurfaceElevation::Floating
  bordered=true
  aria_label="Deployment summary".to_string()
  class_name="docs-surface-custom".to_string()
>
  <div>"Custom class + aria source marker"</div>
</Surface>"#;

    view! {
        <ComponentPage
            title="Surface"
            slug="surface"
            group="Layout"
            description="Spectrum/HeroUI-style foundational container surface with centralized tone/elevation/frame/source contracts and stable data markers."
        >
            <Playground title="Tone + Elevation + Frame" code=tone_code>
                <div class="docs-stack">
                    <Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Default raised"</strong>
                            <span class="ui-muted">"Primary neutral container for page-level composition."</span>
                        </div>
                    </Surface>

                    <Surface
                        tone=SurfaceTone::Subtle
                        elevation=SurfaceElevation::Flat
                        bordered=true
                    >
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Subtle flat bordered"</strong>
                            <span class="ui-muted">"Low-emphasis container using only border contrast."</span>
                        </div>
                    </Surface>

                    <Surface
                        tone=SurfaceTone::Strong
                        elevation=SurfaceElevation::Floating
                        padded=false
                    >
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Strong floating compact"</strong>
                            <span class="ui-muted">"Higher emphasis with floating elevation and explicit tight content."</span>
                        </div>
                    </Surface>
                </div>
            </Playground>

            <Playground title="Custom Aria + Class" code=custom_code>
                <Surface
                    tone=SurfaceTone::Strong
                    elevation=SurfaceElevation::Floating
                    bordered=true
                    aria_label="Deployment summary".to_string()
                    class_name="docs-surface-custom".to_string()
                >
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Deployment summary"</strong>
                        <span class="ui-muted">
                            "Verifies custom aria source + custom class merge contracts."
                        </span>
                    </div>
                </Surface>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
