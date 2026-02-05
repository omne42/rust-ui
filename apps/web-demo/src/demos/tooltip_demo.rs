use leptos::prelude::*;
use ui_components::{Button, Tooltip};

#[component]
pub fn TooltipDemo() -> impl IntoView {
    view! {
        <section id="tooltip" class="demo-card">
            <h2>"Tooltip"</h2>
            <p>"Hover or Tab-focus the trigger. Tooltip enter/exit uses spring motion."</p>

            <div class="demo-row">
                <Tooltip content=|| view! { "Hello from tooltip" }>
                    <Button on_press=Callback::new(|_| {})>
                        "Hover / Focus me"
                    </Button>
                </Tooltip>
                <Tooltip content=|| view! { "Press Tab to focus, then Shift+Tab back." }>
                    <Button on_press=Callback::new(|_| {})>
                        "Keyboard accessible"
                    </Button>
                </Tooltip>
            </div>
        </section>
    }
}
