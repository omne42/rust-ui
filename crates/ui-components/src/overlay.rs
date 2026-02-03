use leptos::{ev, html, portal::Portal, prelude::*};
use ui_headless::{
    use_focus_trap, use_modal, use_overlay_stack_registration, FocusTrapOptions, ModalOptions,
    OnPress,
};

#[component]
pub fn Overlay(on_close: OnPress, children: ChildrenFn) -> impl IntoView {
    let registration = use_overlay_stack_registration();
    use_modal(ModalOptions::enabled());

    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    let focus_trap = use_focus_trap(FocusTrapOptions::enabled(panel_ref));

    let on_key_down = {
        let is_topmost = registration.is_topmost;
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();
            if focus_trap.on_key_down.run((key.clone(), ev.shift_key())) {
                ev.prevent_default();
            }
            if key == "Escape" && is_topmost.get() {
                on_close.run(());
            }
        }
    };

    view! {
        <Portal>
            <div
                class="ui-overlay"
                data-ui-overlay-portal=""
                style="position: fixed; inset: 0; display: flex; align-items: center; justify-content: center; padding: 24px; background: rgba(0, 0, 0, 0.35);"
                on:click=move |_| on_close.run(())
            >
                <div
                    class="ui-overlay__panel"
                    role="dialog"
                    aria-modal="true"
                    tabindex="-1"
                    node_ref=panel_ref
                    style="background: white; border-radius: 12px; padding: 16px; min-width: 280px; max-width: 640px; box-shadow: 0 10px 40px rgba(0,0,0,0.25);"
                    on:click=move |ev| ev.stop_propagation()
                    on:pointerdown=move |ev| ev.stop_propagation()
                    on:keydown=on_key_down
                >
                    {children()}
                </div>
            </div>
        </Portal>
    }
}
