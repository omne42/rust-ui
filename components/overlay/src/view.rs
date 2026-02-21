use super::{OverlayMotion, logic, motion};
use leptos::{ev, html, portal::Portal, prelude::*};
use std::borrow::Cow;
use ui_headless::{
    FocusTrapOptions, ModalOptions, OnPress, RestorePolicy, use_focus_trap, use_modal,
    use_overlay_stack_registration,
};

const FOCUS_FALLBACK_SELECTOR: &str = r#"[data-slot="ui-root"] [tabindex]:not([tabindex="-1"]), [data-slot="ui-root"] button:not([disabled]), [data-slot="ui-root"] a[href], [data-slot="ui-root"] input:not([disabled]), [data-slot="ui-root"] select:not([disabled]), [data-slot="ui-root"] textarea:not([disabled])"#;

#[component]
pub fn Overlay(
    open: Signal<bool>,
    on_close: OnPress,
    children: ChildrenFn,
    #[prop(optional)] aria_labelledby: Option<String>,
    #[prop(optional)] aria_describedby: Option<String>,
    #[prop(optional, default = logic::DEFAULT_ROLE)] role: &'static str,
    #[prop(optional, default = logic::DEFAULT_DISMISSABLE)] is_dismissable: bool,
    #[prop(optional, default = logic::DEFAULT_KEYBOARD_DISMISS_DISABLED)]
    is_keyboard_dismiss_disabled: bool,
    #[prop(optional)] motion: OverlayMotion,
    #[prop(optional, into)] class_name: Option<String>,
    // Called after the close animation finishes (useful for presence/unmount).
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_motion = motion != OverlayMotion::default();
    let has_custom_role = role != logic::DEFAULT_ROLE;
    let aria_labelledby = logic::normalize_optional_text(aria_labelledby);
    let aria_describedby = logic::normalize_optional_text(aria_describedby);
    let resolved_states = logic::resolve_states(logic::OverlayStateInputs {
        open: open.get_untracked(),
        dismiss_mode: logic::OverlayDismissMode::from_is_dismissable(is_dismissable),
        keyboard_dismiss_mode: logic::OverlayKeyboardDismissMode::from_is_disabled(
            is_keyboard_dismiss_disabled,
        ),
        has_custom_role,
        has_custom_aria_labelledby: aria_labelledby.is_some(),
        has_custom_aria_describedby: aria_describedby.is_some(),
        has_custom_class_name: class_name.is_some(),
        has_custom_motion,
        has_on_exit_complete: on_exit_complete.is_some(),
    });
    let root_state = resolved_states.root_state;
    let backdrop_state = resolved_states.backdrop_state;
    let panel_state = resolved_states.panel_state;

    let root_class = StoredValue::new(logic::compose_class_name(class_name, root_state));
    let backdrop_class = StoredValue::new(logic::compose_class_name(None, backdrop_state));
    let panel_class = StoredValue::new(logic::compose_class_name(None, panel_state));

    let registration = use_overlay_stack_registration();
    use_modal(ModalOptions::from_signal(open));

    let aria_labelledby: Signal<Option<String>> = aria_labelledby.into();
    let aria_describedby: Signal<Option<String>> = aria_describedby.into();

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    let on_exit_complete = logic::normalize_on_exit_complete(on_exit_complete);
    motion::attach_motion(root_ref, open, on_exit_complete, motion);

    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    let focus_fallback_selector: Cow<'static, str> = Cow::Borrowed(FOCUS_FALLBACK_SELECTOR);
    let focus_trap = use_focus_trap(
        FocusTrapOptions::enabled(panel_ref)
            .with_scope_id("overlay")
            .with_restore_policy(RestorePolicy::FallbackTo(
                focus_fallback_selector.clone().into_owned(),
            ))
            .with_fallback_selector(focus_fallback_selector.as_ref()),
    );

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
                is_keyboard_dismiss_disabled,
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
                class=move || root_class.with_value(|class_name: &String| class_name.clone())
                data-slot=root_state.slot_attr
                data-state=move || logic::state_attr_for_open(open.get())
                data-open=move || open.get().then_some("true")
                data-closed=move || (!open.get()).then_some("true")
                data-dismiss=root_state.dismiss_attr
                data-keyboard-dismiss=root_state.keyboard_dismiss_attr
                data-dismissable=is_dismissable.then_some("true")
                data-keyboard-dismiss-disabled=is_keyboard_dismiss_disabled.then_some("true")
                data-motion-source=root_state.motion_source_attr
                data-role-source=root_state.role_source_attr
                data-aria-labelledby-source=root_state.aria_labelledby_source_attr
                data-aria-describedby-source=root_state.aria_describedby_source_attr
                data-class-source=root_state.class_source_attr
                data-dismiss-source=root_state.dismiss_source_attr
                data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr
                data-exit-source=root_state.exit_source_attr
                data-custom-motion=root_state.has_custom_motion.then_some("true")
                data-custom-role=root_state.has_custom_role.then_some("true")
                data-custom-aria-labelledby=root_state.has_custom_aria_labelledby.then_some("true")
                data-custom-aria-describedby=root_state.has_custom_aria_describedby.then_some("true")
                data-custom-class=root_state.has_custom_class_name.then_some("true")
                data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                data-ui-overlay-portal=""
                node_ref=root_ref
                on:keydown=on_key_down
            >
                <div
                    class=move || backdrop_class.with_value(|class_name: &String| class_name.clone())
                    data-slot=backdrop_state.slot_attr
                    data-state=backdrop_state.state_attr
                    on:click=move |_| {
                        if is_dismissable {
                            on_close.run(());
                        }
                    }
                ></div>
                <div
                    class=move || panel_class.with_value(|class_name: &String| class_name.clone())
                    data-slot=panel_state.slot_attr
                    data-state=panel_state.state_attr
                    data-dismiss=panel_state.dismiss_attr
                    data-keyboard-dismiss=panel_state.keyboard_dismiss_attr
                    data-role=role
                    role=role
                    aria-modal="true"
                    aria-labelledby=move || aria_labelledby.get()
                    aria-describedby=move || aria_describedby.get()
                    tabindex="-1"
                    node_ref=panel_ref
                    on:click=move |ev: ev::MouseEvent| ev.stop_propagation()
                    on:pointerdown=move |ev: ev::PointerEvent| ev.stop_propagation()
                >
                    {children()}
                </div>
            </div>
        </Portal>
    }
}
