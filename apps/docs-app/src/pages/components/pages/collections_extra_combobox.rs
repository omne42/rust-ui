use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::Combobox;

pub(super) fn combobox() -> AnyView {
    let items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
    ];

    let items_basic = items.clone();
    let items_state = items.clone();

    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let basic_code = r#"let (selected, set_selected) = signal(Some(1_usize));
<Combobox
  id_base="lang".to_string()
  label="Language".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  description="Pick one runtime language".to_string()
/>"#;

    let state_code = r#"let (selected, set_selected) = signal(Some(2_usize));
let (invalid, set_invalid) = signal(false);
<Combobox
  id_base="lang-state".to_string()
  label="Stateful language".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![3]
  invalid=Signal::derive(move || invalid.get())
  error="Language is required".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Combobox"
            slug="combobox"
            group="Collections"
            description="Spectrum-compatible combobox alias for upstream naming parity, preserving ComboBox accessibility, state contracts, and HeroUI-level panel/highlight motion."
        >
            <Playground title="Basic Selection" code=basic_code>
                <div class="docs-stack">
                    <Combobox
                        id_base="docs-combobox-basic".to_string()
                        label="Language".to_string()
                        items=items_basic
                        selected_index=selected
                        set_selected_index=set_selected
                        description="Pick one runtime language".to_string()
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Invalid + Disabled Option" code=state_code>
                <div class="docs-stack">
                    <Combobox
                        id_base="docs-combobox-state".to_string()
                        label="Stateful language".to_string()
                        items=items_state
                        selected_index=selected
                        set_selected_index=set_selected
                        disabled_indices=vec![3]
                        invalid=Signal::derive(move || invalid.get())
                        error="Language is required".to_string()
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
