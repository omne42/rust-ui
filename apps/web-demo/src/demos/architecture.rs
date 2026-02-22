use leptos::prelude::*;

#[component]
pub fn ArchitectureDemo() -> impl IntoView {
    view! {
        <section id="architecture" class="demo-card demo-featured">
            <h2>"Architecture (token-layered layering)"</h2>
            <p>
                "Rule of thumb: keep state pure, keep DOM behavior in headless hooks, keep visuals in components."
            </p>
            <ul>
                <li><code>"ui-state-primitives"</code> " — state primitives (controlled/uncontrolled), no DOM."</li>
                <li><code>"ui-headless"</code> " — interaction + a11y (press/focus-visible/roving tabindex), feature-gated."</li>
                <li><code>"ui-theme"</code> " — tokens → CSS variables."</li>
                <li><code>"ui"</code> " — composed Leptos components; no direct " <code>"web-sys"</code> " usage."</li>
            </ul>
            <div class="demo-divider"></div>
            <div class="demo-kv">
                "Try keyboard: " <code>"Tab"</code> ", " <code>"Enter"</code> "/" <code>"Space"</code> ", " <code>"Esc"</code> ". Focus ring should only appear for keyboard navigation."
            </div>
        </section>
    }
}
