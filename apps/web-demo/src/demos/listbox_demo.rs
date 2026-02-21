use leptos::prelude::*;
use ui_components::List;

#[component]
pub fn ListBoxDemo() -> impl IntoView {
    let (selected_index, set_selected_index) = signal(None::<usize>);

    view! {
        <section id="list" class="demo-card">
            <h2>"List"</h2>
            <p>"Roving focus + aria-activedescendant semantics. Includes disabled item."</p>
            <List
                id_base="demo-listbox".to_string()
                items=vec![
                    "First".to_string(),
                    "Second (disabled)".to_string(),
                    "Third".to_string(),
                ]
                selected_index=selected_index.into()
                on_selected_index_change=Callback::new(move |next| set_selected_index.set(next))
                disabled_indices=vec![1]
            />
            <div class="demo-kv">
                "selected_index: " {move || format!("{:?}", selected_index.get())}
            </div>
        </section>
    }
}
