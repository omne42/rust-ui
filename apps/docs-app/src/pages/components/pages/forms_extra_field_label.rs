use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{FieldLabel, FieldLabelTone};

pub(super) fn field_label() -> AnyView {
    let tone_code = r#"<FieldLabel text=\"Email\".to_string() for_id=\"email\".to_string() required=true />
<FieldLabel text=\"Helper\".to_string() tone=FieldLabelTone::Muted />
<FieldLabel text=\"Critical\".to_string() tone=FieldLabelTone::Strong required=true />"#;

    let custom_code = r#"<FieldLabel
  text=\"Assignee\".to_string()
  for_id=\"assignee\".to_string()
  required=true
  required_indicator=\"(required)\".to_string()
  aria_label=\"Assignee field label\".to_string()
  class_name=\"docs-field-label-custom\".to_string()
/>"#;

    view! {
        <ComponentPage
            title="FieldLabel"
            slug="field-label"
            group="Forms"
            description="Spectrum-compatible field label primitive with centralized tone/required/source-state modeling and stable data contracts."
        >
            <Playground title="Tone + Required" code=tone_code>
                <div class="docs-stack">
                    <FieldLabel
                        text="Email".to_string()
                        for_id="docs-field-label-email".to_string()
                        required=true
                    />
                    <input
                        id="docs-field-label-email"
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />

                    <FieldLabel text="Helper".to_string() tone=FieldLabelTone::Muted />
                    <FieldLabel
                        text="Critical".to_string()
                        tone=FieldLabelTone::Strong
                        required=true
                    />
                </div>
            </Playground>

            <Playground title="Custom Indicator + Aria + Class" code=custom_code>
                <div class="docs-stack">
                    <FieldLabel
                        text="Assignee".to_string()
                        for_id="docs-field-label-assignee".to_string()
                        required=true
                        required_indicator="(required)".to_string()
                        aria_label="Assignee field label".to_string()
                        class_name="docs-field-label-custom".to_string()
                    />
                    <input
                        id="docs-field-label-assignee"
                        class="docs-search__input"
                        type="text"
                        placeholder="Owner"
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
