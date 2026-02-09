use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::Textfield;

pub(super) fn textfield() -> AnyView {
    let (value, set_value) = signal("Omne".to_string());
    let (invalid, set_invalid) = signal(false);

    let basic_code = r#"let (value, set_value) = signal("Omne".to_string());
<Textfield
  id="name".to_string()
  label="Name".to_string()
  value=value
  set_value=set_value
  placeholder="Enter your name".to_string()
/>"#;

    let state_code = r#"let (value, set_value) = signal("owner@example.com".to_string());
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
/>"#;

    view! {
        <ComponentPage
            title="Textfield"
            slug="textfield"
            group="Forms"
            description="Spectrum-compatible text field alias for upstream naming parity, preserving TextField accessibility/state contracts."
        >
            <Playground title="Basic Input" code=basic_code>
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

            <Playground title="Required + Invalid" code=state_code>
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
        </ComponentPage>
    }
    .into_any()
}
