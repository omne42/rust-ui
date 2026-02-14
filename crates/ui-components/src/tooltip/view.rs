use crate::tooltip::{TooltipMotion, TooltipPartStateInput, TooltipSlot, logic, motion};
use leptos::{children::ViewFn, ev, html, portal::Portal, prelude::*};
use ui_headless::{
    TooltipPlacement, TooltipPositionOptions, TooltipTriggerMode, TooltipTriggerOptions,
    use_tooltip_position, use_tooltip_trigger,
};

fn next_id() -> u64 {
    use std::cell::Cell;
    thread_local! {
        static NEXT: Cell<u64> = const { Cell::new(1) };
    }
    NEXT.with(|cell| {
        let id = cell.get();
        cell.set(id + 1);
        id
    })
}

#[component]
pub fn Tooltip(
    #[prop(into)] content: ViewFn,
    children: Children,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] placement: TooltipPlacement,
    #[prop(optional, default = logic::DEFAULT_DELAY_MS)] delay_ms: u64,
    #[prop(optional, default = logic::DEFAULT_CLOSE_DELAY_MS)] close_delay_ms: u64,
    #[prop(optional)] trigger: TooltipTriggerMode,
    #[prop(optional, default = logic::DEFAULT_SHOULD_CLOSE_ON_PRESS)] should_close_on_press: bool,
    #[prop(optional)] motion: TooltipMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let has_custom_motion = motion != TooltipMotion::default();
    let has_custom_delays = logic::has_custom_delays(delay_ms, close_delay_ms);
    let has_custom_trigger_mode = trigger != TooltipTriggerMode::default();
    let has_custom_press_behavior = should_close_on_press != logic::DEFAULT_SHOULD_CLOSE_ON_PRESS;

    let (resolved_id, has_custom_id) = logic::resolve_id(id, format!("ui-tooltip-{}", next_id()));
    let trigger_mode = trigger;

    let trigger_aria = use_tooltip_trigger(
        Some(resolved_id),
        TooltipTriggerOptions {
            is_disabled: disabled,
            delay_ms,
            close_delay_ms,
            trigger,
            should_close_on_press,
        },
    );

    let tooltip_id: StoredValue<String> = StoredValue::new(trigger_aria.state.id().to_string());

    let open: Signal<bool> = trigger_aria.state.is_open().into();
    let presence = ui_headless::use_presence(open);

    let root_state = Memo::new(move |_| {
        logic::resolve_state(TooltipPartStateInput {
            slot: TooltipSlot::Root,
            open: open.get(),
            disabled,
            has_custom_class_name,
            has_custom_motion,
            has_custom_delays,
            has_custom_trigger_mode,
            has_custom_press_behavior,
            has_custom_id,
            trigger_attr: logic::trigger_attr(trigger_mode),
            press_behavior_attr: logic::press_behavior_attr(should_close_on_press),
        })
    });
    let root_class = logic::compose_class_name(class_name, root_state.get_untracked());

    let panel_state = Memo::new(move |_| {
        logic::resolve_state(TooltipPartStateInput {
            slot: TooltipSlot::Panel,
            open: open.get(),
            disabled,
            has_custom_class_name: false,
            has_custom_motion,
            has_custom_delays,
            has_custom_trigger_mode,
            has_custom_press_behavior,
            has_custom_id,
            trigger_attr: logic::trigger_attr(trigger_mode),
            press_behavior_attr: logic::press_behavior_attr(should_close_on_press),
        })
    });
    let panel_class = logic::compose_class_name(None, panel_state.get_untracked());
    let panel_class = StoredValue::new(panel_class);

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
        trigger_aria.handlers.on_focus.run(());

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

        trigger_aria.handlers.on_blur.run(());
    };

    let panel_vars =
        move || logic::compose_panel_vars(position.top_px.get(), position.left_px.get());

    view! {
        <span
            class=root_class
            data-slot=move || root_state.get().slot_attr
            data-state=move || root_state.get().state_attr
            data-open=move || root_state.get().is_open.then_some("true")
            data-closed=move || (!root_state.get().is_open).then_some("true")
            data-disabled=move || root_state.get().is_disabled.then_some("true")
            data-enabled=move || (!root_state.get().is_disabled).then_some("true")
            data-trigger=move || root_state.get().trigger_attr
            data-press-behavior=move || root_state.get().press_behavior_attr
            data-class-source=move || root_state.get().class_source_attr
            data-motion-source=move || root_state.get().motion_source_attr
            data-delay-source=move || root_state.get().delay_source_attr
            data-trigger-source=move || root_state.get().trigger_source_attr
            data-press-source=move || root_state.get().press_source_attr
            data-id-source=move || root_state.get().id_source_attr
            data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
            data-custom-delay=move || root_state.get().has_custom_delays.then_some("true")
            data-custom-trigger=move || root_state.get().has_custom_trigger_mode.then_some("true")
            data-custom-press=move || root_state.get().has_custom_press_behavior.then_some("true")
            data-custom-id=move || root_state.get().has_custom_id.then_some("true")
            node_ref=anchor_ref
            on:pointerenter=move |_| trigger_aria.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| trigger_aria.handlers.on_pointer_leave.run(())
            on:focusin=on_focus_in
            on:focusout=on_focus_out
            on:pointerdown=move |_| trigger_aria.handlers.on_pointer_down.run(())
            on:keydown=move |ev: ev::KeyboardEvent| trigger_aria.handlers.on_key_down.run(ev.key())
        >
            {children()}
            <Show when=move || presence.is_present.get()>
                <Portal>
                    <div
                        class=move || panel_class.with_value(|class_name| class_name.clone())
                        data-ui-overlay-portal=""
                        node_ref=panel_ref
                        id=move || tooltip_id.with_value(|id| id.clone())
                        role="tooltip"
                        style=panel_vars
                        data-placement=move || position.placement.get().as_str()
                        data-slot=move || panel_state.get().slot_attr
                        data-state=move || panel_state.get().state_attr
                        data-open=move || panel_state.get().is_open.then_some("true")
                        data-closed=move || (!panel_state.get().is_open).then_some("true")
                        data-disabled=move || panel_state.get().is_disabled.then_some("true")
                        data-enabled=move || (!panel_state.get().is_disabled).then_some("true")
                        data-trigger=move || panel_state.get().trigger_attr
                        data-press-behavior=move || panel_state.get().press_behavior_attr
                        data-class-source=move || panel_state.get().class_source_attr
                        data-motion-source=move || panel_state.get().motion_source_attr
                        data-delay-source=move || panel_state.get().delay_source_attr
                        data-trigger-source=move || panel_state.get().trigger_source_attr
                        data-press-source=move || panel_state.get().press_source_attr
                        data-id-source=move || panel_state.get().id_source_attr
                        data-custom-motion=move || panel_state.get().has_custom_motion.then_some("true")
                        data-custom-delay=move || panel_state.get().has_custom_delays.then_some("true")
                        data-custom-trigger=move || panel_state.get().has_custom_trigger_mode.then_some("true")
                        data-custom-press=move || panel_state.get().has_custom_press_behavior.then_some("true")
                        data-custom-id=move || panel_state.get().has_custom_id.then_some("true")
                    >
                        {move || content.with_value(|content| content.run())}
                    </div>
                </Portal>
            </Show>
        </span>
    }
}
