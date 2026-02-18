use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Snippet, TextField};

pub(super) fn text_field() -> AnyView {
    let (marker_value, set_marker_value) = signal("release@omne.rs".to_string());
    let (marker_invalid, set_marker_invalid) = signal(false);
    let (marker_read_only, set_marker_read_only) = signal(false);
    let (marker_disabled, set_marker_disabled) = signal(false);

    let code = Signal::derive(move || {
        r#"<TextField id=\"name\".to_string()
  label=\"Name\".to_string()
  placeholder=\"Jane\".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"let (marker_value, set_marker_value) = signal(\"release@omne.rs\".to_string());
let (marker_invalid, set_marker_invalid) = signal(false);
let (marker_read_only, set_marker_read_only) = signal(false);
let (marker_disabled, set_marker_disabled) = signal(false);

<TextField
  id=\"docs-text-field-markers\".to_string()
  label=\"Email\".to_string()
  value=marker_value
  on_value_change=Callback::new(move |next| set_marker_value.set(next))
  is_disabled=marker_disabled.get()
  is_required=Signal::derive(|| true)
  is_invalid=Signal::derive(move || marker_invalid.get())
  is_read_only=marker_read_only.get()
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
                        placeholder="Jane".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (State + Source Markers)"
                description="Inspect root markers like `data-state`, `data-value`, `data-value-control-mode`, `data-default-value-source`, `data-value-change-source`, `data-requirement`, `data-label-source`, and `data-type-source` while toggling state inputs."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <TextField
                        id="docs-text-field-markers".to_string()
                        label="Email".to_string()
                        value=marker_value
                        on_value_change=Callback::new(move |next| set_marker_value.set(next))
                        is_disabled=marker_disabled.get()
                        is_required=Signal::derive(|| true)
                        is_invalid=Signal::derive(move || marker_invalid.get())
                        is_read_only=marker_read_only.get()
                        description="Inspect source/state marker contracts".to_string()
                        error="Email is required".to_string()
                        placeholder="release@omne.rs".to_string()
                        input_type="email"
                        class_name="docs-text-field-state".to_string()
                    />
                    <div class="docs-row" data-slot="text-field-marker-controls">
                        <div data-slot="text-field-toggle-invalid">
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
                        </div>
                        <div data-slot="text-field-toggle-readonly">
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
                        <div data-slot="text-field-toggle-disabled">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_disabled.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_disabled.get() {
                                    "Set enabled"
                                } else {
                                    "Set disabled"
                                }}
                            </ui_components::Button>
                        </div>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="text-field-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="text-field-api-rows">
                    <li>
                        <code>"id: String"</code>
                        " required"
                    </li>
                    <li>
                        <code>"label: String"</code>
                        " default fallback = ui_components::text_field::DEFAULT_LABEL"
                    </li>
                    <li>
                        <code>"value + on_value_change + default_value"</code>
                        " controlled/uncontrolled value axis"
                    </li>
                    <li>
                        <code>"is_disabled / is_read_only / is_required / is_invalid"</code>
                        " unified prefixed accessibility states"
                    </li>
                    <li>
                        <code>"description / error / placeholder / input_type / class_name"</code>
                        " optional semantic + presentation inputs"
                    </li>
                    <li>
                        <code>"lang / dir"</code>
                        " locale passthrough to headless contract"
                    </li>
                    <li>
                        <code>"motion: TextFieldMotion"</code>
                        " default = TextFieldMotion::default()"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="text-field-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="text-field-state-rows">
                    <li>
                        <code>"data-value-control-mode"</code>
                        " = controlled | uncontrolled"
                    </li>
                    <li>
                        <code>"data-state"</code>
                        " = ready | invalid | readonly | disabled"
                    </li>
                    <li>
                        <code>"data-value"</code>
                        " = empty | filled"
                    </li>
                    <li>
                        <code>"data-requirement"</code>
                        " = required | optional"
                    </li>
                    <li>
                        <code>"data-default-value-source / data-value-change-source"</code>
                        " = default | custom and on_value_change | set_value | none"
                    </li>
                    <li>
                        <code>"data-label-source / data-description-source / data-error-source / data-placeholder-source / data-type-source / data-class-source"</code>
                        " = default | custom"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="text-field-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground already supports "
                    <code>"Show code"</code>
                    " with copy action. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\n<TextField\n  id=\"email\".to_string()\n  label=\"Email\".to_string()\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-text-field-source-copy".to_string()
                />
                <ul data-slot="text-field-source-paths">
                    <li><code>"crates/ui-components/src/text_field/mod.rs"</code></li>
                    <li><code>"crates/ui-components/src/text_field/logic.rs"</code></li>
                    <li><code>"crates/ui-components/src/text_field/view.rs"</code></li>
                    <li><code>"crates/ui-components/src/text_field/styles.rs"</code></li>
                    <li><code>"crates/ui-components/src/text_field/motion.rs"</code></li>
                </ul>
                <ul data-slot="text-field-source-prerequisites">
                    <li><code>"component-text_field"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
