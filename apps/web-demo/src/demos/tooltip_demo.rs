use leptos::prelude::*;
use ui_components::Tooltip;

#[component]
pub fn TooltipDemo() -> impl IntoView {
    view! {
        <section id="tooltip" class="demo-card">
            <h2>"Tooltip"</h2>
            <p>"Hover or Tab-focus the trigger. Tooltip enter/exit uses spring motion."</p>

            <div class="demo-row">
                <Tooltip content=|| view! { "Hello from tooltip" }>
                    <span class="demo-kv">"Hover / Focus me"</span>
                </Tooltip>
                <Tooltip content=|| view! { "Press Tab to focus, then Shift+Tab back." }>
                    <span class="demo-kv">"Keyboard accessible"</span>
                </Tooltip>
            </div>
        </section>
    }
}
