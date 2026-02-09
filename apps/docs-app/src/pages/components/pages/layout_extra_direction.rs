use crate::pages::components::{ComponentDoc, ComponentPage};

pub(super) const DIRECTION_PROVIDER_DOC: ComponentDoc = ComponentDoc {
    name: "DirectionProvider",
    slug: "direction-provider",
    group: "Layout",
    page: direction_provider,
};
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{DirectionMode, DirectionProvider};

pub(super) fn direction_provider() -> AnyView {
    let ltr_code = r#"<DirectionProvider direction=DirectionMode::Ltr>
  <div class="docs-direction-demo">"Name → Value"</div>
</DirectionProvider>"#;

    let rtl_code = r##"<DirectionProvider direction=DirectionMode::Rtl class_name="docs-direction-rtl".to_string()>
  <div class="docs-direction-demo">"الاسم ← القيمة"</div>
</DirectionProvider>"##;

    view! {
        <ComponentPage
            title="DirectionProvider"
            slug="direction-provider"
            group="Layout"
            description="Shadcn/Radix-compatible direction context wrapper with normalized `direction`/`dir` props and stable slot + data-direction contracts."
        >
            <Playground title="LTR Direction" code=ltr_code>
                <DirectionProvider direction=DirectionMode::Ltr>
                    <div class="docs-direction-demo">
                        "Name → Value"
                    </div>
                </DirectionProvider>
            </Playground>

            <Playground title="RTL Direction + Class" code=rtl_code>
                <DirectionProvider
                    direction=DirectionMode::Rtl
                    class_name="docs-direction-rtl".to_string()
                >
                    <div class="docs-direction-demo">
                        "الاسم ← القيمة"
                    </div>
                </DirectionProvider>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
