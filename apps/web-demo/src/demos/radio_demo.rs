use leptos::prelude::*;
use ui_components::{RadioGroup, RadioGroupOrientation};

#[component]
pub fn RadioDemo() -> impl IntoView {
    let options = vec![
        "Apple".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Grape".to_string(),
    ];

    let (selected, set_selected) = signal(Some(1_usize));

    view! {
        <section id="radio" class="demo-card">
            <h2>"RadioGroup"</h2>
            <p>"Roving tabindex + arrow navigation; role=radio / role=radiogroup semantics."</p>

            <div class="demo-grid-2">
                <div class="demo-stack">
                    <div class="demo-kv">"Vertical"</div>
                    <RadioGroup
                        id_base="demo-fruit-vertical".to_string()
                        label="Pick a fruit".to_string()
                        options=options.clone()
                        selected_index=selected
                        set_selected_index=set_selected
                        orientation=RadioGroupOrientation::Vertical
                    />
                </div>

                <div class="demo-stack">
                    <div class="demo-kv">"Horizontal (with disabled)"</div>
                    <RadioGroup
                        id_base="demo-fruit-horizontal".to_string()
                        options=options
                        selected_index=selected
                        set_selected_index=set_selected
                        disabled_indices=vec![2]
                        orientation=RadioGroupOrientation::Horizontal
                    />
                    <div class="demo-kv">"selected index: " {move || selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}</div>
                </div>
            </div>
        </section>
    }
}
