use crate::tooltip::{TooltipMotion, motion};
use leptos::{children::ViewFn, ev, html, portal::Portal, prelude::*};
use ui_headless::{
    TooltipPlacement, TooltipPositionOptions, TooltipTriggerMode, TooltipTriggerOptions,
    use_tooltip_position, use_tooltip_trigger,
};

#[component]
pub fn Tooltip(
    #[prop(into)] content: ViewFn,
    children: Children,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] placement: TooltipPlacement,
    #[prop(optional, default = 1500)] delay_ms: u64,
    #[prop(optional, default = 500)] close_delay_ms: u64,
    #[prop(optional)] trigger: TooltipTriggerMode,
    #[prop(optional, default = true)] should_close_on_press: bool,
    #[prop(optional)] motion: TooltipMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let trigger = use_tooltip_trigger(
        id,
        TooltipTriggerOptions {
            is_disabled: disabled,
            delay_ms,
            close_delay_ms,
            trigger,
            should_close_on_press,
        },
    );

    let tooltip_id: StoredValue<String> = StoredValue::new(trigger.state.id().to_string());

    let open: Signal<bool> = trigger.state.is_open().into();
    let presence = crate::presence::use_presence(open);

    let content = StoredValue::new(content);

    let anchor_ref: NodeRef<html::Span> = NodeRef::new();
    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    let position = use_tooltip_position(TooltipPositionOptions {
        anchor_ref,
        panel_ref,
        placement,
        ..Default::default()
    });

    motion::attach_motion(
        panel_ref,
        open,
        position.placement.into(),
        presence.finish_exit,
        motion,
    );

    let base_class = "ui-tooltip".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    #[cfg(target_arch = "wasm32")]
    let focus_target = StoredValue::new_local(None::<leptos::web_sys::Element>);

    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        if let Some(target) = focus_target.get_value() {
            let _ = target.remove_attribute("aria-describedby");
        }
    });

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let is_open = open.get();
        let Some(target) = focus_target.get_value() else {
            return;
        };

        let id = tooltip_id.with_value(|id| id.clone());
        if is_open {
            let _ = target.set_attribute("aria-describedby", &id);
        } else {
            let _ = target.remove_attribute("aria-describedby");
        }
    });

    let on_focus_in = move |_ev: ev::FocusEvent| {
        trigger.handlers.on_focus.run(());

        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            if let Some(target) = focus_target.get_value() {
                let _ = target.remove_attribute("aria-describedby");
            }

            let Some(target) = _ev.target() else {
                focus_target.set_value(None);
                return;
            };

            let Ok(target) = target.dyn_into::<leptos::web_sys::Element>() else {
                focus_target.set_value(None);
                return;
            };

            if open.get_untracked() {
                let id = tooltip_id.with_value(|id| id.clone());
                let _ = target.set_attribute("aria-describedby", &id);
            }

            focus_target.set_value(Some(target));
        }
    };

    let on_focus_out = move |_ev: ev::FocusEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            if let Some(target) = focus_target.get_value() {
                let _ = target.remove_attribute("aria-describedby");
            }
            focus_target.set_value(None);

            let leaving = match anchor_ref.get_untracked() {
                Some(anchor) => {
                    let anchor_el: leptos::web_sys::Element = anchor.unchecked_into();
                    match _ev.related_target() {
                        Some(related) => match related.dyn_into::<leptos::web_sys::Node>() {
                            Ok(node) => !anchor_el.contains(Some(&node)),
                            Err(_) => true,
                        },
                        None => true,
                    }
                }
                None => true,
            };

            if !leaving {
                return;
            }
        }

        trigger.handlers.on_blur.run(());
    };

    view! {
        <span
            class=class
            data-slot="tooltip"
            node_ref=anchor_ref
            on:pointerenter=move |_| trigger.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| trigger.handlers.on_pointer_leave.run(())
            on:focusin=on_focus_in
            on:focusout=on_focus_out
            on:pointerdown=move |_| trigger.handlers.on_pointer_down.run(())
            on:keydown=move |ev: ev::KeyboardEvent| trigger.handlers.on_key_down.run(ev.key())
        >
            {children()}
            <Show when=move || presence.is_present.get()>
                <Portal>
                    <div
                        class="ui-tooltip__panel"
                        data-ui-overlay-portal=""
                        node_ref=panel_ref
                        id=move || tooltip_id.with_value(|id| id.clone())
                        role="tooltip"
                        style=move || format!(
                            "--ui-tooltip-top: {}px; --ui-tooltip-left: {}px;",
                            position.top_px.get(),
                            position.left_px.get()
                        )
                        data-placement=move || position.placement.get().as_str()
                        data-slot="tooltip-panel"
                    >
                        {move || content.with_value(|content| content.run())}
                    </div>
                </Portal>
            </Show>
        </span>
    }
}
