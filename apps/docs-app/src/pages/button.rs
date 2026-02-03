use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn ButtonPage() -> impl IntoView {
    let code = r#"<Button variant=ButtonVariant::Default>"Primary"</Button>
<Button variant=ButtonVariant::Outline>"Outline"</Button>
<Button variant=ButtonVariant::Ghost>"Ghost"</Button>"#;

    view! {
        <Playground title="Button" description="Variants and sizes" code=code>
            <div style="display:flex; gap:12px; flex-wrap:wrap;">
                <Button variant=ButtonVariant::Default>"Primary"</Button>
                <Button variant=ButtonVariant::Outline>"Outline"</Button>
                <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                <Button variant=ButtonVariant::Destructive>"Danger"</Button>
                <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm>"Small"</Button>
                <Button variant=ButtonVariant::Secondary size=ButtonSize::Lg>"Large"</Button>
            </div>
        </Playground>
    }
}
