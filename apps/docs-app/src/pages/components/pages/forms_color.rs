use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::ColorField;

pub(super) fn color_field() -> AnyView {
    let (value, set_value) = signal(Some("#4f46e5".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));

    let basic_code = r##"let (value, set_value) = signal(Some("#4f46e5".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));

<ColorField
  id_base="docs-color-field-basic".to_string()
  label="Fill color".to_string()
  value=value.into()
  on_value_change=on_value_change
/>"##;

    let states_code = r##"<ColorField
  id_base="docs-color-field-invalid".to_string()
  label="Brand color".to_string()
  default_value="javascript:alert(1)".to_string()
  class_name="docs-color-field-custom".to_string()
/>
<ColorField
  id_base="docs-color-field-disabled".to_string()
  label="Accent color".to_string()
  default_value="#0ea5e9".to_string()
  disabled=true
/>"##;

    view! {
        <ComponentPage
            title="ColorField"
            slug="color-field"
            group="Forms"
            description="Spectrum-compatible color text field with centralized label/placeholder/aria/state normalization, sanitized preview rendering, and stable slot/data contracts."
        >
            <Playground title="Controlled Value" code=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorField
                        id_base="docs-color-field-basic".to_string()
                        label="Fill color".to_string()
                        value=value.into()
                        on_value_change=on_value_change
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Invalid + Disabled + Custom Class" code=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorField
                        id_base="docs-color-field-invalid".to_string()
                        label="Brand color".to_string()
                        default_value="javascript:alert(1)".to_string()
                        class_name="docs-color-field-custom".to_string()
                    />
                    <ColorField
                        id_base="docs-color-field-disabled".to_string()
                        label="Accent color".to_string()
                        default_value="#0ea5e9".to_string()
                        disabled=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
