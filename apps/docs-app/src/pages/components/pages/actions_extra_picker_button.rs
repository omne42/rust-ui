use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::PickerButton;

pub(super) fn picker_button() -> AnyView {
    let (active, set_active) = signal(false);
    let on_press = Callback::new(move |_| set_active.update(|value| *value = !*value));

    let (marker_presses, set_marker_presses) = signal(0_usize);
    let marker_press =
        Callback::new(move |_| set_marker_presses.update(|value| *value = value.saturating_add(1)));

    let basic_code = r#"<PickerButton on_press=Callback::new(move |_| { /* ... */ })>
  \"Choose item\"
</PickerButton>"#;

    let states_code = r#"<PickerButton quiet=true>"Filter"</PickerButton>
<PickerButton invalid=true>"Required"</PickerButton>
<PickerButton disabled=true>"Disabled"</PickerButton>"#;

    let markers_code = r#"let (active, set_active) = signal(false);

<PickerButton
  quiet=true
  invalid=true
  is_active=true
  aria_label="Inspect picker trigger".to_string()
  class_name="docs-picker-button-state".to_string()
  on_press=Callback::new(move |_| set_active.update(|value| *value = !*value))
>
  "Inspect markers"
</PickerButton>"#;

    view! {
        <ComponentPage
            title="PickerButton"
            slug="picker-button"
            group="Actions"
            description="Spectrum-compatible PickerButton alias for upstream naming parity, preserving FieldButton accessibility/state contracts and HeroUI-level press/focus interaction behavior."
        >
            <Playground title="Interactive" code=basic_code>
                <div class="docs-stack">
                    <PickerButton on_press=on_press>
                        "Choose item"
                    </PickerButton>
                    <span class="ui-muted">"active: " {move || active.get().to_string()}</span>
                </div>
            </Playground>

            <Playground title="State Matrix" code=states_code>
                <div class="docs-row">
                    <PickerButton quiet=true>
                        "Filter"
                    </PickerButton>
                    <PickerButton invalid=true>
                        "Required"
                    </PickerButton>
                    <PickerButton disabled=true>
                        "Disabled"
                    </PickerButton>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-quiet`, `data-invalid`, `data-disabled`, `data-active`, `data-has-handler`, `data-aria-source`, `data-class-source`, and `data-handler-source`."
                code=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <PickerButton
                        quiet=true
                        invalid=true
                        is_active=true
                        aria_label="Inspect picker trigger".to_string()
                        class_name="docs-picker-button-state".to_string()
                        on_press=marker_press
                    >
                        "Inspect markers"
                    </PickerButton>
                    <span class="ui-muted">"presses: " {move || marker_presses.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
