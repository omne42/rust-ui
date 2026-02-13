use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{NativeSelect, NativeSelectOption, NativeSelectSize};

pub(super) fn native_select() -> AnyView {
    let options = vec![
        NativeSelectOption::new("system", "System"),
        NativeSelectOption::new("manual", "Manual"),
        NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
    ];
    let (selected_raw, set_selected_raw) = signal(None::<usize>);
    let selected_signal: Signal<Option<usize>> = Signal::derive(move || selected_raw.get());
    let on_selected_change = Callback::new(move |next: Option<usize>| set_selected_raw.set(next));

    let required_options = vec![
        NativeSelectOption::new("staging", "Staging"),
        NativeSelectOption::new("production", "Production"),
        NativeSelectOption::new("canary", "Canary"),
    ];

    let disabled_options = vec![
        NativeSelectOption::new("legacy", "Legacy").disabled(true),
        NativeSelectOption::new("frozen", "Frozen").disabled(true),
    ];

    let code = Signal::derive(move || {
        r#"let (selected_raw, set_selected_raw) = signal(None::<usize>);
let selected: Signal<Option<usize>> = Signal::derive(move || selected_raw.get());

<NativeSelect
  id_base="mode".to_string()
  options=vec![
    NativeSelectOption::new("system", "System"),
    NativeSelectOption::new("manual", "Manual"),
    NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
  ]
  selected_index=selected
  on_selected_index_change=Callback::new(move |next| set_selected_raw.set(next))
  placeholder="Choose mode".to_string()
  name="mode".to_string()
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<NativeSelect
  id_base="deployment".to_string()
  options=vec![
    NativeSelectOption::new("staging", "Staging"),
    NativeSelectOption::new("production", "Production"),
    NativeSelectOption::new("canary", "Canary"),
  ]
  default_selected_index=1
  required=true
  invalid=true
  size=NativeSelectSize::Lg
  aria_label="Deployment strategy".to_string()
/>
<NativeSelect
  id_base="disabled".to_string()
  options=vec![
    NativeSelectOption::new("legacy", "Legacy").disabled(true),
    NativeSelectOption::new("frozen", "Frozen").disabled(true),
  ]
  disabled=true
  placeholder="Disabled select".to_string()
  size=NativeSelectSize::Sm
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="NativeSelect"
            slug="native-select"
            group="Forms"
            description="Spectrum-style native `<select>` wrapper with controllable selection, root `data-*` contracts, and stable option normalization."
        >
            <Playground title="Controlled + Placeholder" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <NativeSelect
                        id_base="docs-native-select-controlled".to_string()
                        options=options
                        selected_index=selected_signal
                        on_selected_index_change=on_selected_change
                        placeholder="Choose mode".to_string()
                        name="mode".to_string()
                        aria_label="Mode".to_string()
                    />
                    <span class="ui-muted">
                        "selected index: "
                        {move || {
                            selected_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Required + Invalid + Disabled" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <NativeSelect
                        id_base="docs-native-select-required".to_string()
                        options=required_options
                        default_selected_index=1
                        required=true
                        invalid=true
                        size=NativeSelectSize::Lg
                        aria_label="Deployment strategy".to_string()
                        class_name="docs-native-select-custom".to_string()
                    />

                    <NativeSelect
                        id_base="docs-native-select-disabled".to_string()
                        options=disabled_options
                        disabled=true
                        placeholder="Disabled select".to_string()
                        size=NativeSelectSize::Sm
                        aria_label="Disabled native select".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
