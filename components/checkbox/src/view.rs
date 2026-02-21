use super::{CheckboxMotion, CheckboxSize, CheckboxVariant, logic, motion};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, CheckboxOptions, OnPress, use_checkbox};

const SLOT_CHECKBOX: &str = "checkbox";
const SLOT_CHECKBOX_BOX: &str = "checkbox-box";
const SLOT_CHECKBOX_INDICATOR: &str = "checkbox-indicator";
const SLOT_CHECKBOX_LABEL: &str = "checkbox-label";

const CHECK_ICON_VIEW_BOX: &str = "0 0 24 24";
const CHECK_ICON_STROKE_WIDTH: &str = "3.5";
const CHECK_ICON_STROKE_LINECAP: &str = "round";
const CHECK_ICON_STROKE_LINEJOIN: &str = "round";
const CHECK_ICON_PATH: &str = "M4.5 12.75l6 6 9-13.5";

fn render_checkbox_indicator_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox=CHECK_ICON_VIEW_BOX
            stroke_width=CHECK_ICON_STROKE_WIDTH
            stroke="currentColor"
            aria-hidden="true"
            focusable="false"
        >
            <path
                stroke_linecap=CHECK_ICON_STROKE_LINECAP
                stroke_linejoin=CHECK_ICON_STROKE_LINEJOIN
                d=CHECK_ICON_PATH
            />
        </svg>
    }
}

fn render_checkbox_indicator(indicator_ref: NodeRef<html::Span>) -> impl IntoView {
    view! {
        <span node_ref=indicator_ref class="ui-checkbox__indicator" data-slot=SLOT_CHECKBOX_INDICATOR>
            {render_checkbox_indicator_icon()}
        </span>
    }
}

#[component]
pub fn Checkbox(
    #[prop(optional)] is_checked: Option<ReadSignal<bool>>,
    #[prop(optional)] checked: Option<ReadSignal<bool>>,
    #[prop(optional)] on_checked_change: Option<WriteSignal<bool>>,
    #[prop(optional)] set_checked: Option<WriteSignal<bool>>,
    #[prop(optional)] default_checked: Option<bool>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    // Legacy callback alias; canonical write handler is on_checked_change.
    #[prop(optional)] on_change: Option<Callback<bool>>,
    #[prop(optional)] variant: CheckboxVariant,
    #[prop(optional)] size: CheckboxSize,
    #[prop(optional)] motion: CheckboxMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let root_ref: NodeRef<html::Button> = NodeRef::new();
    let checked_control = logic::resolve_checked_control(
        is_checked,
        checked,
        on_checked_change,
        set_checked,
        default_checked,
    );
    let checked = checked_control.checked;
    let on_checked_change = checked_control.on_checked_change;
    let control_mode = checked_control.mode;
    let checked_source_attr = checked_control.checked_source_attr;
    let handler_source_attr = checked_control.handler_source_attr;
    let disabled = logic::normalize_is_disabled(is_disabled, disabled);

    let toggle: OnPress = Callback::new(move |_| {
        let next = !checked.get_untracked();
        if let Some(on_checked_change) = on_checked_change {
            on_checked_change.set(next);
        }
        if let Some(on_change) = on_change {
            on_change.run(next);
        }
    });

    let aria = use_checkbox(CheckboxOptions {
        is_disabled: disabled,
        is_checked: checked,
        on_press: Some(toggle),
        lang,
        dir,
    });

    motion::attach_root_motion(
        root_ref,
        aria.state.is_hovered,
        aria.state.is_pressed,
        disabled,
        motion,
    );

    let indicator_ref: NodeRef<html::Span> = NodeRef::new();
    motion::attach_indicator_motion(indicator_ref, checked, motion);

    let render_state = Memo::new(move |_| {
        logic::derive_render_state(logic::CheckboxRenderStateInput {
            checked_state: logic::CheckboxCheckedState::from_bool(checked.get()),
            is_disabled: disabled,
            is_pressed: aria.state.is_pressed.get(),
            is_hovered: aria.state.is_hovered.get(),
            is_focused: aria.state.is_focused.get(),
            is_focus_visible: aria.state.is_focus_visible.get(),
            control_mode,
        })
    });
    let has_custom_motion = motion != CheckboxMotion::default();
    let motion_source_attr = if has_custom_motion {
        "custom"
    } else {
        "default"
    };
    let agent_contract = Memo::new(move |_| {
        logic::resolve_agent_contract(logic::CheckboxAgentContractInput {
            render_state: render_state.get(),
            checked_source_attr,
            handler_source_attr,
            has_custom_motion,
        })
    });

    let class = logic::compose_class_name(class_name, variant, size);

    view! {
        <button
            type="button"
            node_ref=root_ref
            class=class
            class:ui-checkbox--focus-visible=move || render_state.get().state.is_focus_visible
            disabled=disabled
            data-slot=SLOT_CHECKBOX
            data-state=move || render_state.get().state.data_state()
            data-checked=move || render_state.get().state.is_checked.then_some("true")
            data-unchecked=move || render_state.get().state.is_unchecked.then_some("true")
            data-disabled=move || render_state.get().state.is_disabled.then_some("true")
            data-enabled=move || render_state.get().state.is_enabled.then_some("true")
            data-hovered=move || render_state.get().state.is_hovered.then_some("true")
            data-pressed=move || render_state.get().state.is_pressed.then_some("true")
            data-focused=move || render_state.get().state.is_focused.then_some("true")
            data-focus-visible=move || render_state.get().state.is_focus_visible.then_some("true")
            data-state-source=move || render_state.get().state_source_attr
            data-checked-source=checked_source_attr
            data-handler-source=handler_source_attr
            data-motion-source=motion_source_attr
            data-custom-motion=has_custom_motion.then_some("true")
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-state-source=move || agent_contract.get().state_source
            data-ui-checked-source=move || agent_contract.get().checked_source
            data-ui-handler-source=move || agent_contract.get().handler_source
            data-ui-motion-source=move || agent_contract.get().motion_source
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-checked=move || aria.attrs.aria_checked.get()
            aria-label=aria_label
            lang=aria.attrs.lang
            dir=aria.attrs.dir
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:pointerenter=move |_| aria.handlers.hover.on_pointer_enter.run(())
            on:pointerleave=move |_| aria.handlers.hover.on_pointer_leave.run(())
            on:click=move |_| aria.handlers.press.on_click.run(())
            on:keydown=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_down.run(key) {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_up.run(key) {
                    ev.prevent_default();
                }
            }
            on:focus=move |_| aria.handlers.focus_ring.on_focus.run(())
            on:blur=move |_| {
                aria.handlers.press.on_blur.run(());
                aria.handlers.focus_ring.on_blur.run(());
            }
        >
            <span class="ui-checkbox__box" data-slot=SLOT_CHECKBOX_BOX>
                {render_checkbox_indicator(indicator_ref)}
            </span>
            <span class="ui-checkbox__label" data-slot=SLOT_CHECKBOX_LABEL>
                {children()}
            </span>
        </button>
    }
}
