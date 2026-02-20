pub fn CheckboxGroup(
    id: String,
    label: String,
    description: Option<String>,
    error: Option<String>,
    invalid: leptos::prelude::Signal<bool>,
    required: leptos::prelude::Signal<bool>,
    disabled: bool,
    aria_describedby: leptos::prelude::Signal<Option<String>>,
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
