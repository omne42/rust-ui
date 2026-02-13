use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::TextField;

pub(super) fn text_field() -> AnyView {
    let (value, set_value) = signal(String::new());

    let (marker_value, set_marker_value) = signal("release@omne.rs".to_string());
    let (marker_invalid, set_marker_invalid) = signal(false);
    let (marker_read_only, set_marker_read_only) = signal(false);

    let code = Signal::derive(move || {
        r#"let (value, set_value) = signal(String::new());
<TextField id=\"name\".to_string()
  label=\"Name\".to_string()
  value=value
  set_value=set_value
  placeholder=\"Jane\".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"let (marker_value, set_marker_value) = signal(\"release@omne.rs\".to_string());
let (marker_invalid, set_marker_invalid) = signal(false);
let (marker_read_only, set_marker_read_only) = signal(false);

<TextField
  id=\"docs-text-field-markers\".to_string()
  label=\"Email\".to_string()
  value=marker_value
  set_value=set_marker_value
  required=true
  invalid=Signal::derive(move || marker_invalid.get())
  read_only=marker_read_only.get()
  description=\"Inspect source/state marker contracts\".to_string()
  error=\"Email is required\".to_string()
  placeholder=\"release@omne.rs\".to_string()
  input_type=\"email\"
  class_name=\"docs-text-field-state\".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="TextField"
            slug="text-field"
            group="Forms"
            description="A compact field wrapper built on headless text field semantics with explicit state/source marker contracts."
        >
            <Playground title="Label + placeholder" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <TextField
                        id="docs-text-field".to_string()
                        label="Name".to_string()
                        value=value
                        set_value=set_value
                        placeholder="Jane".to_string()
                    />
                    <span class="ui-muted">"value: " {move || value.get()}</span>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, and `data-type-source`."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <TextField
                        id="docs-text-field-markers".to_string()
                        label="Email".to_string()
                        value=marker_value
                        set_value=set_marker_value
                        required=true
                        invalid=Signal::derive(move || marker_invalid.get())
                        read_only=marker_read_only.get()
                        description="Inspect source/state marker contracts".to_string()
                        error="Email is required".to_string()
                        placeholder="release@omne.rs".to_string()
                        input_type="email"
                        class_name="docs-text-field-state".to_string()
                    />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_marker_invalid.update(|value| *value = !*value)
                            })
                        >
                            {move || if marker_invalid.get() {
                                "Clear marker invalid"
                            } else {
                                "Mark marker invalid"
                            }}
                        </ui_components::Button>
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_marker_read_only.update(|value| *value = !*value)
                            })
                        >
                            {move || if marker_read_only.get() {
                                "Set editable"
                            } else {
                                "Set read only"
                            }}
                        </ui_components::Button>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
