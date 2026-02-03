use leptos::prelude::*;
use ui_components::Tabs;

#[component]
pub fn TabsDemo() -> impl IntoView {
    view! {
        <section id="tabs" class="demo-card">
            <h2>"Tabs"</h2>
            <p>"Arrow/Home/End to move; focus follows selection (automatic activation)."</p>

            <Tabs labels=vec!["Account", "Billing", "Team"] id_base="demo-tabs".to_string()>
                <div class="demo-stack">
                    <div class="demo-kv">"Account"</div>
                    <div>"Profile, security, sessions…"</div>
                </div>
                <div class="demo-stack">
                    <div class="demo-kv">"Billing"</div>
                    <div>"Plans, invoices, payment methods…"</div>
                </div>
                <div class="demo-stack">
                    <div class="demo-kv">"Team"</div>
                    <div>"Members, roles, invites…"</div>
                </div>
            </Tabs>
        </section>
    }
}
