use leptos::{ev, html, portal::Portal, prelude::*};
use ui_headless::{
    use_focus_trap, use_overlay_stack_registration, use_popover_position, FocusTrapOptions,
    OnPress, PopoverPlacement, PopoverPositionOptions,
};

#[component]
pub fn Popover(
    anchor_ref: NodeRef<html::Button>,
    on_close: OnPress,
    children: ChildrenFn,
    #[prop(optional)] placement: PopoverPlacement,
) -> impl IntoView {
    let registration = use_overlay_stack_registration();

    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    let focus_trap = use_focus_trap(FocusTrapOptions::enabled(panel_ref));

    let position = use_popover_position(PopoverPositionOptions {
        anchor_ref,
        panel_ref,
        placement,
        ..Default::default()
    });

    let panel_style = move || {
        format!(
            "position: fixed; top: {}px; left: {}px; background: white; border-radius: 12px; padding: 12px; min-width: 240px; box-shadow: 0 10px 40px rgba(0,0,0,0.18);",
            position.top_px.get(),
            position.left_px.get()
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
                style="position: fixed; inset: 0;"
                on:click=move |_| on_close.run(())
            >
                <div
                    class="ui-popover__panel"
                    node_ref=panel_ref
                    tabindex="-1"
                    style=panel_style
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
