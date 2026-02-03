use leptos::prelude::*;
use ui_components::{SearchField, TextField};

#[component]
pub fn TextFieldDemo() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (query, set_query) = signal(String::new());
    let invalid = Signal::derive(move || {
        let value = email.get();
        !value.is_empty() && !value.contains('@')
    });

    view! {
        <section id="text-field" class="demo-card">
            <h2>"TextField"</h2>
            <p>"Label + description/error wiring via aria-describedby."</p>

            <div class="demo-stack">
                <TextField
                    id="demo-email".to_string()
                    label="Email".to_string()
                    value=email
                    set_value=set_email
                    required=true
                    invalid=invalid
                    description="We’ll never share your email.".to_string()
                    error="Please enter a valid email address.".to_string()
                    placeholder="name@example.com".to_string()
                />
                <div class="demo-kv">"value: " {move || email.get()}</div>

                <div class="demo-divider"></div>

                <SearchField
                    id="demo-search".to_string()
                    label="Search".to_string()
                    value=query
                    set_value=set_query
                    placeholder="Type to search…".to_string()
                    required=false
                    invalid=false
                />
            </div>
        </section>
    }
}
