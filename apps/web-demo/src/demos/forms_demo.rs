use leptos::prelude::*;
use ui_components::{Checkbox, CheckboxSize, CheckboxVariant, Switch};

#[component]
pub fn FormsDemo() -> impl IntoView {
    let (checkbox_enabled, set_checkbox_enabled) = signal(false);
    let (checkbox_checked, set_checkbox_checked) = signal(true);
    let (checkbox_disabled_off, set_checkbox_disabled_off) = signal(false);
    let (checkbox_disabled_on, set_checkbox_disabled_on) = signal(true);

    let (switch_enabled, set_switch_enabled) = signal(false);
    let (switch_checked, set_switch_checked) = signal(true);
    let (switch_disabled_off, set_switch_disabled_off) = signal(false);
    let (switch_disabled_on, set_switch_disabled_on) = signal(true);

    view! {
        <section id="forms" class="demo-card">
            <h2>"Checkbox / Switch"</h2>
            <p>"Tab to focus; Space to toggle. Focus-visible shows an outline."</p>

            <div class="demo-grid-2">
                <div class="demo-stack">
                    <div class="demo-kv">"Checkbox"</div>
                    <Checkbox size=CheckboxSize::Sm checked=checkbox_enabled set_checked=set_checkbox_enabled>
                        "Enabled (interactive)"
                    </Checkbox>
                    <Checkbox
                        variant=CheckboxVariant::Accent
                        checked=checkbox_checked
                        set_checked=set_checkbox_checked
                    >
                        "Enabled (checked)"
                    </Checkbox>
                    <Checkbox
                        disabled=true
                        size=CheckboxSize::Lg
                        checked=checkbox_disabled_off
                        set_checked=set_checkbox_disabled_off
                    >
                        "Disabled (unchecked)"
                    </Checkbox>
                    <Checkbox
                        disabled=true
                        size=CheckboxSize::Lg
                        variant=CheckboxVariant::Accent
                        checked=checkbox_disabled_on
                        set_checked=set_checkbox_disabled_on
                    >
                        "Disabled (checked)"
                    </Checkbox>
                    <div class="demo-kv">
                        "enabled checked: " {move || checkbox_enabled.get().to_string()}
                    </div>
                </div>

                <div class="demo-stack">
                    <div class="demo-kv">"Switch"</div>
                    <Switch checked=switch_enabled set_checked=set_switch_enabled>
                        "Enabled (interactive)"
                    </Switch>
                    <Switch pressed_width_px=22.0 checked=switch_checked set_checked=set_switch_checked>
                        "Enabled (checked)"
                    </Switch>
                    <Switch disabled=true checked=switch_disabled_off set_checked=set_switch_disabled_off>
                        "Disabled (unchecked)"
                    </Switch>
                    <Switch disabled=true checked=switch_disabled_on set_checked=set_switch_disabled_on>
                        "Disabled (checked)"
                    </Switch>
                    <div class="demo-kv">
                        "enabled checked: " {move || switch_enabled.get().to_string()}
                    </div>
                </div>
            </div>
        </section>
    }
}
