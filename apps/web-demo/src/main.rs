use leptos::{mount::mount_to_body, prelude::*};
use ui_components::{
    provide_focus_visible, provide_overlay_stack, Button, ListBox, OnPress, Overlay,
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
        </main>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
