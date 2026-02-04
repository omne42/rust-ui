use crate::overlay::{OverlayMotion, motion};
use leptos::{ev, html, portal::Portal, prelude::*};
use ui_headless::{
    FocusTrapOptions, ModalOptions, OnPress, use_focus_trap, use_modal,
    use_overlay_stack_registration,
};

#[component]
pub fn Overlay(
    open: Signal<bool>,
    on_close: OnPress,
    children: ChildrenFn,
    #[prop(optional)] aria_labelledby: Option<String>,
    #[prop(optional)] aria_describedby: Option<String>,
    #[prop(optional, default = "dialog")] role: &'static str,
    #[prop(optional)] motion: OverlayMotion,
    /// Called after the close animation finishes (useful for presence/unmount).
    #[prop(optional)]
    on_exit_complete: Option<Callback<()>>,
) -> impl IntoView {
    let registration = use_overlay_stack_registration();
    use_modal(ModalOptions::enabled());

    let aria_labelledby: Signal<Option<String>> = aria_labelledby.into();
    let aria_describedby: Signal<Option<String>> = aria_describedby.into();

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));
    motion::attach_motion(root_ref, open, on_exit_complete, motion);

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
            <div class="ui-overlay" data-ui-overlay-portal="" node_ref=root_ref on:keydown=on_key_down>
                <div class="ui-overlay__backdrop" on:click=move |_| on_close.run(())></div>
                <div
                    class="ui-overlay__panel"
                    role=role
                    aria-modal="true"
                    aria-labelledby=move || aria_labelledby.get()
                    aria-describedby=move || aria_describedby.get()
                    tabindex="-1"
                    node_ref=panel_ref
                    on:click=move |ev| ev.stop_propagation()
                    on:pointerdown=move |ev| ev.stop_propagation()
                >
                    {children()}
                </div>
            </div>
        </Portal>
    }
}
