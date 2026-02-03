use leptos::prelude::*;
use ui_components::{Button, Modal, OnPress};
use ui_core::overlay_trigger::{OverlayTriggerStateOptions, use_overlay_trigger_state};

#[component]
pub fn OverlayDemo() -> impl IntoView {
    let (overlay_state, set_overlay_state) = signal(use_overlay_trigger_state(
        OverlayTriggerStateOptions::default(),
    ));
    let open_overlay: OnPress = Callback::new(move |_| set_overlay_state.update(|s| s.open()));
    let close_overlay: OnPress = Callback::new(move |_| set_overlay_state.update(|s| s.close()));

    let is_modal_open = Signal::derive(move || overlay_state.get().is_open());
    let (is_modal_present, set_modal_present) = signal(is_modal_open.get_untracked());

    Effect::new(move |_| {
        if is_modal_open.get() {
            set_modal_present.set(true);
        }
    });

    let on_modal_exit_complete: Callback<()> = Callback::new(move |_| set_modal_present.set(false));

    view! {
        <>
            <Show when=move || is_modal_present.get()>
                <Modal
                    open=is_modal_open
                    id_base="demo-modal".to_string()
                    title="Overlay v2".to_string()
                    description="Esc / click outside closes. Tab is trapped; close returns focus.".to_string()
                    on_close=close_overlay
                    on_exit_complete=on_modal_exit_complete
                >
                    <div class="demo-row" style="justify-content: flex-end;">
                        <Button on_press=close_overlay>"Close"</Button>
                    </div>
                </Modal>
            </Show>

            <section id="overlay" class="demo-card">
                <h2>"Overlay"</h2>
                <p>"Esc dismiss + click outside + focus trap (v0) + focus restore."</p>
                <div class="demo-row">
                    <Button on_press=open_overlay>"Open Modal"</Button>
                    <span class="demo-kv">
                        "open: " {move || overlay_state.get().is_open().to_string()}
                    </span>
                </div>
            </section>
        </>
    }
}
