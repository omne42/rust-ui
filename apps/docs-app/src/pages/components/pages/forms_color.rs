use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{ColorArea, ColorField};

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

pub(super) fn color_area() -> AnyView {
    let (value, set_value) = signal((0.6_f32, 0.4_f32));
    let on_value_change = Callback::new(move |next: (f32, f32)| set_value.set(next));

    let basic_code = r##"let (value, set_value) = signal((0.6_f32, 0.4_f32));
let on_value_change = Callback::new(move |next: (f32, f32)| set_value.set(next));

<ColorArea
  id_base="docs-color-area-basic".to_string()
  label="Saturation / Lightness".to_string()
  value=value.into()
  on_value_change=on_value_change
  preview_color="#7c3aed".to_string()
/>"##;

    let states_code = r##"<ColorArea
  id_base="docs-color-area-disabled".to_string()
  label="Accent area".to_string()
  default_value=(0.25, 0.85)
  grid_size=15
  step=0.05
  disabled=true
  class_name="docs-color-area-custom".to_string()
/>"##;

    view! {
        <ComponentPage
            title="ColorArea"
            slug="color-area"
            group="Forms"
            description="Spectrum-compatible two-axis color selection primitive with centralized step/grid normalization, keyboard navigation, and stable slot/data-state contracts."
        >
            <Playground title="Controlled Grid Selection" code=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorArea
                        id_base="docs-color-area-basic".to_string()
                        label="Saturation / Lightness".to_string()
                        value=value.into()
                        on_value_change=on_value_change
                        preview_color="#7c3aed".to_string()
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || {
                            let (x, y) = value.get();
                            format!("({:.2}, {:.2})", x, y)
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Custom Grid + Custom Class" code=states_code>
                <ColorArea
                    id_base="docs-color-area-disabled".to_string()
                    label="Accent area".to_string()
                    default_value=(0.25, 0.85)
                    grid_size=15
                    step=0.05
                    disabled=true
                    class_name="docs-color-area-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
