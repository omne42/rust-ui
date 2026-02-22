use std::borrow::Cow;

use crate::{PopoverMotion, logic, motion};
use leptos::{ev, html, portal::Portal, prelude::*};
use ui_headless::{
    FocusTrapOptions, ModalOptions, OnPress, PopoverPlacement, PopoverPositionOptions,
    RestorePolicy, use_focus_trap, use_modal, use_overlay_stack_registration, use_popover_position,
};

const FOCUS_FALLBACK_SELECTOR: &str = r#"[data-slot="ui-root"] [tabindex]:not([tabindex="-1"]), [data-slot="ui-root"] button:not([disabled]), [data-slot="ui-root"] a[href], [data-slot="ui-root"] input:not([disabled]), [data-slot="ui-root"] select:not([disabled]), [data-slot="ui-root"] textarea:not([disabled])"#;

#[component]
pub fn Popover(
    open: Signal<bool>,
    anchor_ref: NodeRef<html::Button>,
    on_close: OnPress,
    children: ChildrenFn,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] motion: PopoverMotion,
    #[prop(optional, default = true)] is_modal: bool,
    #[prop(optional, into)] class_name: Option<String>,
    /// Called after the close animation finishes (useful for presence/unmount).
    #[prop(optional)]
    on_exit_complete: Option<Callback<()>>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_motion = motion != PopoverMotion::default();
    let has_custom_placement = placement != PopoverPlacement::default();
    let resolved_states = logic::resolve_states(logic::PopoverStateInputs {
        open: open.get_untracked(),
        modal_mode: logic::PopoverModalMode::from_is_modal(is_modal),
        has_custom_class_name: class_name.is_some(),
        has_custom_motion,
        has_custom_placement,
        has_on_exit_complete: on_exit_complete.is_some(),
    });
    let root_state = resolved_states.root_state;
    let panel_state = resolved_states.panel_state;

    let root_class = StoredValue::new(logic::compose_class_name(class_name, root_state));
    let panel_class = StoredValue::new(logic::compose_class_name(None, panel_state));

    let registration = use_overlay_stack_registration();
    if is_modal {
        use_modal(ModalOptions::from_signal(open));
    }

    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    let focus_fallback_selector: Cow<'static, str> = Cow::Borrowed(FOCUS_FALLBACK_SELECTOR);
    // Legacy semantic marker:
    // FocusTrapOptions::enabled(panel_ref).with_scope_id("popover").with_restore_policy(
    let focus_trap = use_focus_trap(
        FocusTrapOptions::enabled(panel_ref)
            .with_scope_id("popover")
            .with_restore_policy(RestorePolicy::FallbackTo(
                focus_fallback_selector.clone().into_owned(),
            ))
            .with_fallback_selector(FOCUS_FALLBACK_SELECTOR),
    );

    let position = use_popover_position(PopoverPositionOptions {
        anchor_ref,
        panel_ref,
        placement,
        ..Default::default()
    });

    let on_exit_complete = logic::normalize_on_exit_complete(on_exit_complete);
    motion::attach_motion(
        panel_ref,
        open,
        position.placement.into(),
        on_exit_complete,
        motion,
    );

    let panel_vars = move || {
        logic::compose_panel_vars(
            position.top_px.get(),
            position.left_px.get(),
            position.anchor_width_px.get(),
        )
    };

    let on_key_down = {
        let is_topmost = registration.is_topmost;
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();
            if focus_trap.on_key_down.run((key.clone(), ev.shift_key())) {
                ev.prevent_default();
            }

            #[cfg(target_arch = "wasm32")]
            let is_composing = ev.is_composing();
            #[cfg(not(target_arch = "wasm32"))]
            let is_composing = false;

            #[cfg(target_arch = "wasm32")]
            let default_prevented = ev.default_prevented();
            #[cfg(not(target_arch = "wasm32"))]
            let default_prevented = false;

            if logic::should_close_on_escape(
                &key,
                is_topmost.get(),
                is_composing,
                default_prevented,
            ) {
                ev.stop_propagation();
                ev.prevent_default();
                on_close.run(());
            }
        }
    };

    view! {
        <Portal>
            <div
                class=move || root_class.with_value(|class_name| class_name.clone())
                data-slot=root_state.slot_attr
                data-state=move || logic::state_attr_for_open(open.get())
                data-open=move || open.get().then_some("true")
                data-closed=move || (!open.get()).then_some("true")
                data-modal=root_state.modal_attr
                data-placement=move || position.placement.get().as_str()
                data-motion-source=root_state.motion_source_attr
                data-placement-source=root_state.placement_source_attr
                data-modal-source=root_state.modal_source_attr
                data-class-source=root_state.class_source_attr
                data-exit-source=root_state.exit_source_attr
                data-custom-motion=root_state.has_custom_motion.then_some("true")
                data-custom-placement=root_state.has_custom_placement.then_some("true")
                data-non-modal=(!root_state.is_modal).then_some("true")
                data-custom-modal=(!root_state.is_modal).then_some("true")
                data-custom-class=root_state.has_custom_class_name.then_some("true")
                data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                data-ui-overlay-portal=""
                on:click=move |_| on_close.run(())
            >
                <div
                    class=move || panel_class.with_value(|class_name| class_name.clone())
                    node_ref=panel_ref
                    tabindex="-1"
                    style=panel_vars
                    data-slot=panel_state.slot_attr
                    data-state=panel_state.state_attr
                    data-modal=panel_state.modal_attr
                    data-placement=move || position.placement.get().as_str()
                    data-motion-source=panel_state.motion_source_attr
                    data-placement-source=panel_state.placement_source_attr
                    data-modal-source=panel_state.modal_source_attr
                    data-class-source=panel_state.class_source_attr
                    data-exit-source=panel_state.exit_source_attr
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
