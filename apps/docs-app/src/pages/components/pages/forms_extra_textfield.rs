use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::Textfield;

pub(super) fn textfield() -> AnyView {
    let (value, set_value) = signal("Omne".to_string());
    let (invalid, set_invalid) = signal(false);

    let (marker_value, set_marker_value) = signal("owner@example.com".to_string());
    let (marker_invalid, set_marker_invalid) = signal(false);

    let basic_code = Signal::derive(move || {
        r#"let (value, set_value) = signal("Omne".to_string());
<Textfield
  id="name".to_string()
  label="Name".to_string()
  value=value
  set_value=set_value
  placeholder="Enter your name".to_string()
/>"#
        .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"let (value, set_value) = signal("owner@example.com".to_string());
let (invalid, set_invalid) = signal(false);
<Textfield
  id="email".to_string()
  label="Email".to_string()
  value=value
  set_value=set_value
  input_type="email"
  required=true
  invalid=Signal::derive(move || invalid.get())
  error="Valid email is required".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"let (value, set_value) = signal("owner@example.com".to_string());
let (invalid, set_invalid) = signal(false);

<Textfield
  id="docs-textfield-markers".to_string()
  label="Account email".to_string()
  value=value
  set_value=set_value
  required=true
  invalid=Signal::derive(move || invalid.get())
  description="Inspect source/state marker contracts".to_string()
  error="Valid email is required".to_string()
  placeholder="name@example.com".to_string()
  input_type="email"
  class_name="docs-textfield-state".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Textfield"
            slug="textfield"
            group="Forms"
            description="Spectrum-compatible text field alias for upstream naming parity, preserving TextField accessibility/state contracts."
        >
            <Playground title="Basic Input" code_signal=basic_code>
                <div class="docs-stack">
                    <Textfield
                        id="docs-textfield-name".to_string()
                        label="Name".to_string()
                        value=value
                        set_value=set_value
                        placeholder="Enter your name".to_string()
                    />
                    <span class="ui-muted">{move || format!("value: {}", value.get())}</span>
                </div>
            </Playground>

            <Playground title="Required + Invalid" code_signal=state_code>
                <div class="docs-stack">
                    <Textfield
                        id="docs-textfield-email".to_string()
                        label="Email".to_string()
                        value=value
                        set_value=set_value
                        input_type="email"
                        required=true
                        invalid=Signal::derive(move || invalid.get())
                        error="Valid email is required".to_string()
                    />
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                    >
                        {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                    </ui_components::Button>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, and `data-type-source`."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Textfield
                        id="docs-textfield-markers".to_string()
                        label="Account email".to_string()
                        value=marker_value
                        set_value=set_marker_value
                        required=true
                        invalid=Signal::derive(move || marker_invalid.get())
                        description="Inspect source/state marker contracts".to_string()
                        error="Valid email is required".to_string()
                        placeholder="name@example.com".to_string()
                        input_type="email"
                        class_name="docs-textfield-state".to_string()
                    />
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| {
                            set_marker_invalid.update(|value| *value = !*value)
                        })
                    >
                        {move || {
                            if marker_invalid.get() {
                                "Clear marker invalid"
                            } else {
                                "Mark marker invalid"
                            }
                        }}
                    </ui_components::Button>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
