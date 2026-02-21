use leptos::prelude::*;
use ui_components::{Button, Modal, OnPress};

#[component]
pub fn OverlayDemo() -> impl IntoView {
    let (is_modal_open, set_is_modal_open) = signal(false);
    let open_overlay: OnPress = Callback::new(move |_| set_is_modal_open.set(true));
    let close_overlay: OnPress = Callback::new(move |_| set_is_modal_open.set(false));

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
                    is_open=is_modal_open.into()
                    id_base="demo-modal".to_string()
                    title="Overlay v2".to_string()
                    description="Esc / click outside closes. Tab is trapped; close returns focus.".to_string()
                    on_close=close_overlay
                    on_exit_complete=on_modal_exit_complete
                >
                    <div class="demo-row demo-row--end">
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
                        "open: " {move || is_modal_open.get()}
                    </span>
                </div>
            </section>
        </>
    }
}
