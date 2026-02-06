use leptos::prelude::*;
use ui_components::{Button, ButtonSize, ButtonVariant, CodeBlock, OnPress};

#[component]
pub fn Playground(
    title: &'static str,
    #[prop(optional)] description: &'static str,
    #[prop(optional)] code: &'static str,
    children: Children,
) -> impl IntoView {
    let anchor_id = crate::toc::use_docs_toc()
        .map(|toc| toc.register(title, 2))
        .unwrap_or_default();
    let anchor_id = (!anchor_id.trim().is_empty()).then_some(anchor_id);

    let description = (!description.trim().is_empty()).then_some(description);
    let code = (!code.trim().is_empty()).then_some(code);
    let (show_code, set_show_code) = signal(code.is_some());

    let on_toggle_code: OnPress = Callback::new(move |_| set_show_code.update(|v| *v = !*v));

    view! {
        <section class="docs-card playground" id=anchor_id>
            <div class="playground__header">
                <div class="playground__title">
                    <h2>{title}</h2>
                    {description.map(|description| view! { <div class="docs-subtitle">{description}</div> })}
                </div>

                {code.map(|_| {
                    view! {
                        <div class="playground__actions">
                            <Button
                                variant=ButtonVariant::Secondary
                                size=ButtonSize::Sm
                                on_press=on_toggle_code
                            >
                                {move || if show_code.get() { "Hide code" } else { "Show code" }}
                            </Button>
                        </div>
                    }
                })}
            </div>

            <div class="playground__preview">{children()}</div>
            {code.map(|code| {
                view! {
                    <Show when=move || show_code.get()>
                        <CodeBlock code=code.to_string() />
                    </Show>
                }
            })}
        </section>
    }
}
