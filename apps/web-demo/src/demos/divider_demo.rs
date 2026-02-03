use leptos::prelude::*;
use ui_components::{Divider, DividerOrientation};

#[component]
pub fn DividerDemo() -> impl IntoView {
    view! {
        <section id="divider" class="demo-card">
            <h2>"Divider"</h2>
            <p>"Theme-driven separators (horizontal/vertical) with `role=separator`."</p>

            <div class="demo-stack">
                <div class="demo-row">
                    <span>"Above"</span>
                </div>
                <Divider />
                <div class="demo-row">
                    <span>"Below"</span>
                </div>
            </div>

            <div class="demo-divider"></div>

            <div class="demo-row demo-row--tall">
                <span>"Left"</span>
                <Divider orientation=DividerOrientation::Vertical />
                <span>"Right"</span>
            </div>
        </section>
    }
}
