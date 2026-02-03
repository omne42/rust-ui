use leptos::prelude::*;

#[component]
pub fn Playground(
    title: &'static str,
    #[prop(optional)] description: &'static str,
    #[prop(optional)] code: &'static str,
    children: Children,
) -> impl IntoView {
    let description = (!description.trim().is_empty()).then_some(description);
    let code = (!code.trim().is_empty()).then_some(code);

    view! {
        <section class="docs-card playground">
            <h2>{title}</h2>
            {description.map(|description| view! { <div class="docs-subtitle">{description}</div> })}
            <div class="playground__preview">{children()}</div>
            {code.map(|code| {
                view! {
                    <pre class="playground__code"><code>{code}</code></pre>
                }
            })}
        </section>
    }
}
