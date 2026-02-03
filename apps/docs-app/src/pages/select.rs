use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::Select;

#[component]
pub fn SelectPage() -> impl IntoView {
    let items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
    ];
    let (selected, set_selected) = signal(None::<usize>);
    let code = r#"let items = vec!["Apple".to_string(), "Banana".to_string()];
let (selected, set_selected) = signal(None::<usize>);
<Select id_base="fruit".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
/>"#;

    view! {
        <Playground title="Select" description="Popover + listbox" code=code>
            <Select
                id_base="fruit".to_string()
                items=items
                selected_index=selected
                set_selected_index=set_selected
            />
        </Playground>
    }
}
