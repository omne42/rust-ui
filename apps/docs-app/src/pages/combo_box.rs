use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::ComboBox;

#[component]
pub fn ComboBoxPage() -> impl IntoView {
    let items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let (selected, set_selected) = signal(None::<usize>);
    let code = r#"let items = vec!["Rust".to_string(), "TypeScript".to_string()];
let (selected, set_selected) = signal(None::<usize>);
<ComboBox id_base="lang".to_string()
  label="Language".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
/>"#;

    view! {
        <Playground title="ComboBox" description="Searchable listbox (keyboard + pointer)" code=code>
            <ComboBox
                id_base="lang".to_string()
                label="Language".to_string()
                items=items
                selected_index=selected
                set_selected_index=set_selected
            />
        </Playground>
    }
}
