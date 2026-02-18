use leptos::prelude::*;
use ui_components::TextArea;

#[component]
pub fn TextAreaDemo() -> impl IntoView {
    let (bio, set_bio) = signal(String::new());
    let on_value_change = Callback::new(move |next: String| set_bio.set(next));
    let invalid = Signal::derive(move || bio.get().len() > 140);

    view! {
        <section id="text-area" class="demo-card">
            <h2>"TextArea"</h2>
            <p>"Uses the same aria-describedby/error wiring as TextField via ui-headless."</p>

            <div class="demo-stack">
                <TextArea
                    id="demo-bio".to_string()
                    label="Bio".to_string()
                    value=Signal::derive(move || bio.get())
                    on_value_change=on_value_change
                    rows=4
                    is_invalid=invalid
                    description="Keep it under 140 characters.".to_string()
                    error="Too long. Please shorten your bio.".to_string()
                    placeholder="Write something…".to_string()
                />
                <div class="demo-kv">
                    "len: " {move || bio.get().len().to_string()}
                </div>
            </div>
        </section>
    }
}
