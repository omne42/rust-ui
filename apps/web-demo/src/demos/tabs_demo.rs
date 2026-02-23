use leptos::prelude::*;
use ui::{Tabs, TabsItem, TabsKeyboardActivation};

#[component]
pub fn TabsDemo() -> impl IntoView {
    let (manual_selected, set_manual_selected) = signal(0_usize);
    let on_manual_change = Callback::new(move |index: usize| set_manual_selected.set(index));

    view! {
        <section id="tabs" class="demo-card">
            <h2>"Tabs"</h2>
            <p>"Arrow/Home/End to move; focus follows selection (keyboardActivation=automatic)."</p>

            <Tabs id_base="demo-tabs".to_string()>
                <TabsItem label="Account">
                    <div class="demo-stack">
                        <div class="demo-kv">"Account"</div>
                        <div>"Profile, security, sessions…"</div>
                    </div>
                </TabsItem>
                <TabsItem label="Billing">
                    <div class="demo-stack">
                        <div class="demo-kv">"Billing"</div>
                        <div>"Plans, invoices, payment methods…"</div>
                    </div>
                </TabsItem>
                <TabsItem label="Team">
                    <div class="demo-stack">
                        <div class="demo-kv">"Team"</div>
                        <div>"Members, roles, invites…"</div>
                    </div>
                </TabsItem>
            </Tabs>

            <h3>"Manual activation"</h3>
            <p>"Arrow/Home/End to move focus; press Enter/Space/click to select (keyboardActivation=manual)."</p>

            <Tabs
                id_base="demo-tabs-manual".to_string()
                keyboard_activation=TabsKeyboardActivation::Manual
                selected_index=manual_selected
                on_selection_change=on_manual_change
                disabled_indices=vec![2]
            >
                <TabsItem label="Profile">
                    <div class="demo-stack">
                        <div class="demo-kv">"Profile"</div>
                        <div>"Selected index: " {move || manual_selected.get()}</div>
                    </div>
                </TabsItem>
                <TabsItem label="Billing">
                    <div class="demo-stack">
                        <div class="demo-kv">"Billing"</div>
                        <div>"Selected index: " {move || manual_selected.get()}</div>
                    </div>
                </TabsItem>
                <TabsItem label="Team (disabled)">
                    <div class="demo-stack">
                        <div class="demo-kv">"Team"</div>
                        <div>"This tab is disabled."</div>
                    </div>
                </TabsItem>
            </Tabs>
        </section>
    }
}
