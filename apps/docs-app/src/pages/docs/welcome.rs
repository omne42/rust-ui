use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant};

#[component]
pub fn Welcome() -> impl IntoView {
    let code = r#"use ui_components::{UiRoot, Theme, Button, ButtonVariant};

<UiRoot theme=Theme::light() safe_area=true inject_components_css=true>
  <Button variant=ButtonVariant::Default>"Hello"</Button>
</UiRoot>"#;

    view! {
        <section class="docs-card docs-prose">
            <h2>"Welcome"</h2>
            <p>
                "This repo builds a layered Leptos UI system (Spectrum-style): "
                <code>"ui-state-primitives"</code> ", " <code>"ui-headless"</code> ", " <code>"ui-theme"</code> ", "
                <code>"ui-motion"</code> ", " <code>"ui-components"</code> "."
            </p>
            <p>"Use the left nav to explore docs and component playgrounds."</p>
        </section>

        <Playground title="Hello Button" description="Basic usage" code=code>
            <Button variant=ButtonVariant::Default>"Hello"</Button>
        </Playground>
    }
}
