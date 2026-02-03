use leptos::prelude::*;
use ui_components::ComboBox;

#[component]
pub fn ComboBoxDemo() -> impl IntoView {
    let items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Swift".to_string(),
    ];

    let (selected, set_selected) = signal(Some(0_usize));
    let label = Signal::derive(move || match selected.get() {
        Some(0) => "Rust",
        Some(1) => "TypeScript",
        Some(2) => "Go",
        Some(3) => "Python",
        Some(4) => "Swift",
        _ => "(none)",
    });

    view! {
        <section id="combo-box" class="demo-card">
            <h2>"ComboBox"</h2>
            <p>"Combobox input + listbox popup with keyboard navigation and strict selection."</p>

            <div class="demo-stack">
                <ComboBox
                    id_base="demo-language".to_string()
                    label="Language".to_string()
                    items=items
                    selected_index=selected
                    set_selected_index=set_selected
                    disabled_indices=vec![3]
                    description="Type to filter; ArrowDown to open; Enter to commit.".to_string()
                />
                <div class="demo-kv">"selected: " {move || label.get()}</div>
            </div>
        </section>
    }
}
