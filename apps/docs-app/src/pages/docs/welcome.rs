use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant};
use ui_layout::{Card, Heading, HeadingLevel};

#[component]
pub fn Welcome() -> impl IntoView {
    let code = r#"use ui_components::{UiRoot, Theme, Button, ButtonVariant};

<UiRoot theme=Theme::light() safe_area=true inject_components_css=true>
  <Button variant=ButtonVariant::Default>"Hello"</Button>
</UiRoot>"#;

    view! {
        <Card class_name="docs-prose".to_string()>
            <Heading level=HeadingLevel::H2>"Welcome"</Heading>
            <p>
                "This repo builds a layered Leptos UI system (baseline-style): "
                <code>"ui-state-primitives"</code> ", " <code>"ui-headless"</code> ", " <code>"ui-theme"</code> ", "
                <code>"ui-motion"</code> ", " <code>"ui-components"</code> "."
            </p>
            <p>"Use the left nav to explore docs and component playgrounds."</p>
        </Card>

        <Playground title="Hello Button" description="Basic usage" code=code>
            <Button variant=ButtonVariant::Default>"Hello"</Button>
        </Playground>
    }
}
