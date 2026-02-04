use crate::popover::{PopoverMotion, motion};
use leptos::{ev, html, portal::Portal, prelude::*};
use ui_headless::{
    FocusTrapOptions, ModalOptions, OnPress, PopoverPlacement, PopoverPositionOptions,
    use_focus_trap, use_modal, use_overlay_stack_registration, use_popover_position,
};

#[component]
pub fn Popover(
    open: Signal<bool>,
    anchor_ref: NodeRef<html::Button>,
    on_close: OnPress,
    children: ChildrenFn,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] motion: PopoverMotion,
    #[prop(optional, default = true)] is_modal: bool,
    /// Called after the close animation finishes (useful for presence/unmount).
    #[prop(optional)]
    on_exit_complete: Option<Callback<()>>,
) -> impl IntoView {
    let registration = use_overlay_stack_registration();
    if is_modal {
        use_modal(ModalOptions::enabled());
    }

    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    let focus_trap = use_focus_trap(FocusTrapOptions::enabled(panel_ref));

    let position = use_popover_position(PopoverPositionOptions {
        anchor_ref,
        panel_ref,
        placement,
        ..Default::default()
    });

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));
    motion::attach_motion(
        panel_ref,
        open,
        position.placement.into(),
        on_exit_complete,
        motion,
    );

    let panel_vars = move || {
        format!(
            "--ui-popover-top: {}px; --ui-popover-left: {}px; --ui-popover-anchor-width: {}px;",
            position.top_px.get(),
            position.left_px.get(),
            position.anchor_width_px.get()
        )
    };

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
                class="ui-popover"
                data-ui-overlay-portal=""
                on:click=move |_| on_close.run(())
            >
                <div
                    class="ui-popover__panel"
                    node_ref=panel_ref
                    tabindex="-1"
                    style=panel_vars
                    data-placement=move || position.placement.get().as_str()
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
