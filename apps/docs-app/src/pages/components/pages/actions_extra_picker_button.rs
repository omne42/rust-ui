use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::PickerButton;

pub(super) fn picker_button() -> AnyView {
    let (active, set_active) = signal(false);
    let on_press = Callback::new(move |_| set_active.update(|value| *value = !*value));

    let basic_code = r#"<PickerButton on_press=Callback::new(move |_| { /* ... */ })>
  \"Choose item\"
</PickerButton>"#;

    let states_code = r#"<PickerButton quiet=true>"Filter"</PickerButton>
<PickerButton invalid=true>"Required"</PickerButton>
<PickerButton disabled=true>"Disabled"</PickerButton>"#;

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
        </ComponentPage>
    }
    .into_any()
}
