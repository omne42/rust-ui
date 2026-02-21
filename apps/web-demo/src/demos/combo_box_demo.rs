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
    let controlled_items = items.clone();

    let (selected, set_selected) = signal(Some(0_usize));
    let (controlled_selected, set_controlled_selected) = signal(Some(1_usize));
    let (controlled_open, set_controlled_open) = signal(false);
    let on_open_change = Callback::new(move |next: bool| set_controlled_open.set(next));

    let label = Signal::derive(move || match selected.get() {
        Some(0) => "Rust",
        Some(1) => "TypeScript",
        Some(2) => "Go",
        Some(3) => "Python",
        Some(4) => "Swift",
        _ => "(none)",
    });
    let controlled_label = Signal::derive(move || match controlled_selected.get() {
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

            <div class="demo-divider"></div>

            <div class="demo-stack">
                <div class="demo-kv">"Controlled open"</div>
                <ComboBox
                    id_base="demo-language-controlled".to_string()
                    label="Language".to_string()
                    items=controlled_items
                    selected_index=controlled_selected
                    set_selected_index=set_controlled_selected
                    is_open=controlled_open.into()
                    on_open_change=on_open_change
                    description="This combobox is controlled via is_open/on_open_change.".to_string()
                />
                <div class="demo-kv">
                    {move || format!("open: {}, selected: {}", controlled_open.get(), controlled_label.get())}
                </div>
            </div>
        </section>
    }
}
