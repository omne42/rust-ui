use leptos::{mount::mount_to_body, prelude::*};
use ui_components::{
    provide_focus_visible, provide_overlay_stack, Button, Checkbox, ListBox, OnPress, Overlay,
    Switch,
};
use ui_core::overlay_trigger::{use_overlay_trigger_state, OverlayTriggerStateOptions};

#[component]
fn App() -> impl IntoView {
    provide_focus_visible();
    provide_overlay_stack();

    let (count, set_count) = signal(0_i32);
    let on_press: OnPress = Callback::new(move |_| set_count.update(|n| *n += 1));

    let (overlay_state, set_overlay_state) = signal(use_overlay_trigger_state(
        OverlayTriggerStateOptions::default(),
    ));
    let open_overlay: OnPress = Callback::new(move |_| set_overlay_state.update(|s| s.open()));
    let close_overlay: OnPress = Callback::new(move |_| set_overlay_state.update(|s| s.close()));

    let (selected_index, set_selected_index) = signal(None::<usize>);

    let (checkbox_enabled, set_checkbox_enabled) = signal(false);
    let (checkbox_checked, set_checkbox_checked) = signal(true);
    let (checkbox_disabled_off, set_checkbox_disabled_off) = signal(false);
    let (checkbox_disabled_on, set_checkbox_disabled_on) = signal(true);

    let (switch_enabled, set_switch_enabled) = signal(false);
    let (switch_checked, set_switch_checked) = signal(true);
    let (switch_disabled_off, set_switch_disabled_off) = signal(false);
    let (switch_disabled_on, set_switch_disabled_on) = signal(true);

    view! {
        <main style="padding: 24px; font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif;">
            <h1 style="margin: 0 0 12px 0; font-size: 18px;">"web-demo"</h1>
            <div style="display: flex; gap: 12px; align-items: center;">
                <Button on_press=on_press>"Press Me"</Button>
                <Button disabled=true>"Disabled"</Button>
                <Button on_press=open_overlay>"Open Overlay"</Button>
                <span>"count: " {count}</span>
            </div>

            <Show when=move || overlay_state.get().is_open()>
                <Overlay on_close=close_overlay>
                    <h2 style="margin: 0 0 8px 0; font-size: 16px;">"Overlay v1"</h2>
                    <p style="margin: 0 0 12px 0; line-height: 1.4;">
                        "Esc / click outside closes. Tab is trapped; close returns focus."
                    </p>
                    <div style="display: flex; gap: 12px; justify-content: flex-end;">
                        <Button on_press=close_overlay>"Close"</Button>
                    </div>
                </Overlay>
            </Show>

            <ListBox
                id_base="demo-listbox".to_string()
                items=vec![
                    "First".to_string(),
                    "Second".to_string(),
                    "Third".to_string(),
                ]
                selected_index=selected_index
                set_selected_index=set_selected_index
            />
            <div style="margin-top: 8px; font-size: 12px; color: #6b7280;">
                "selected_index: " {move || format!("{:?}", selected_index.get())}
            </div>

            <section style="margin-top: 24px;">
                <h2 style="margin: 0 0 8px 0; font-size: 14px;">"Checkbox / Switch"</h2>
                <div style="font-size: 12px; color: #6b7280; margin: 0 0 12px 0;">
                    "Tab to focus; Space to toggle. Focus-visible shows an outline."
                </div>

                <div style="display: flex; gap: 24px; flex-wrap: wrap; align-items: flex-start;">
                    <div style="display: flex; flex-direction: column; gap: 10px; min-width: 260px;">
                        <div style="font-size: 12px; color: #6b7280;">"Checkbox"</div>
                        <Checkbox checked=checkbox_enabled set_checked=set_checkbox_enabled>
                            "Enabled (interactive)"
                        </Checkbox>
                        <Checkbox checked=checkbox_checked set_checked=set_checkbox_checked>
                            "Enabled (checked)"
                        </Checkbox>
                        <Checkbox
                            disabled=true
                            checked=checkbox_disabled_off
                            set_checked=set_checkbox_disabled_off
                        >
                            "Disabled (unchecked)"
                        </Checkbox>
                        <Checkbox
                            disabled=true
                            checked=checkbox_disabled_on
                            set_checked=set_checkbox_disabled_on
                        >
                            "Disabled (checked)"
                        </Checkbox>
                        <div style="font-size: 12px; color: #6b7280;">
                            "enabled checked: " {move || checkbox_enabled.get().to_string()}
                        </div>
                    </div>

                    <div style="display: flex; flex-direction: column; gap: 10px; min-width: 260px;">
                        <div style="font-size: 12px; color: #6b7280;">"Switch"</div>
                        <Switch checked=switch_enabled set_checked=set_switch_enabled>
                            "Enabled (interactive)"
                        </Switch>
                        <Switch checked=switch_checked set_checked=set_switch_checked>
                            "Enabled (checked)"
                        </Switch>
                        <Switch
                            disabled=true
                            checked=switch_disabled_off
                            set_checked=set_switch_disabled_off
                        >
                            "Disabled (unchecked)"
                        </Switch>
                        <Switch
                            disabled=true
                            checked=switch_disabled_on
                            set_checked=set_switch_disabled_on
                        >
                            "Disabled (checked)"
                        </Switch>
                        <div style="font-size: 12px; color: #6b7280;">
                            "enabled checked: " {move || switch_enabled.get().to_string()}
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
