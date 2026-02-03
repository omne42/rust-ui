use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::TextField;

#[component]
pub fn TextFieldPage() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    let code = r#"let (value, set_value) = signal(String::new());
<TextField id="name".to_string()
  label="Name".to_string()
  value=value
  set_value=set_value
  placeholder=Some("Jane".to_string())
/>"#;

    view! {
        <Playground title="TextField" description="Label + placeholder + validation" code=code>
            <TextField
                id="name".to_string()
                label="Name".to_string()
                value=value
                set_value=set_value
                placeholder="Jane".to_string()
            />
        </Playground>
    }
}
