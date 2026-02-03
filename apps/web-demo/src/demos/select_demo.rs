use leptos::prelude::*;
use ui_components::Select;

#[component]
pub fn SelectDemo() -> impl IntoView {
    let select_items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
    ];
    let (select_index, set_select_index) = signal(None::<usize>);

    view! {
        <section id="select" class="demo-card">
            <h2>"Select"</h2>
            <p>"Composition: Button → Popover → ListBox → select/close. Includes disabled item + typeahead."</p>

            <div class="demo-row">
                <Select
                    id_base="demo-select".to_string()
                    items=select_items.clone()
                    selected_index=select_index
                    set_selected_index=set_select_index
                    disabled_indices=vec![3]
                />
                <div class="demo-kv">
                    "selected_index: " {move || format!("{:?}", select_index.get())}
                </div>
            </div>
        </section>
    }
}
