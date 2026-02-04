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

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
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

    view! {
        <span
            class=class
            data-slot="tooltip"
        >
            <button
                type="button"
                class="ui-tooltip__trigger"
                data-slot="tooltip-trigger"
                node_ref=anchor_ref
                disabled=disabled
                aria-describedby=move || {
                    open.get().then(|| tooltip_id.with_value(|id| id.clone()))
                }
                on:pointerenter=move |_| trigger.handlers.on_pointer_enter.run(())
                on:pointerleave=move |_| trigger.handlers.on_pointer_leave.run(())
                on:focus=move |_| trigger.handlers.on_focus.run(())
                on:blur=move |_| trigger.handlers.on_blur.run(())
                on:pointerdown=move |_| trigger.handlers.on_pointer_down.run(())
                on:keydown=move |ev: ev::KeyboardEvent| trigger.handlers.on_key_down.run(ev.key())
            >
                {children()}
            </button>
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
