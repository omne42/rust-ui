use crate::{TooltipMotion, TooltipPartStateInput, TooltipSlot, logic, motion};
use leptos::{children::ViewFn, ev, html, portal::Portal, prelude::*};
use ui_headless::a11y::{TooltipPanelA11yOptions, tooltip_panel_attrs};
use ui_headless::{
    TooltipFocusA11yOptions, TooltipPlacement, TooltipPositionOptions, TooltipTriggerMode,
    TooltipTriggerOptions, use_tooltip_focus_a11y, use_tooltip_position, use_tooltip_trigger,
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
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] placement: TooltipPlacement,
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, default = logic::DEFAULT_DELAY_MS)] delay_ms: u64,
    #[prop(optional, default = logic::DEFAULT_CLOSE_DELAY_MS)] close_delay_ms: u64,
    #[prop(optional)] trigger: TooltipTriggerMode,
    #[prop(optional, default = logic::DEFAULT_SHOULD_CLOSE_ON_PRESS)] should_close_on_press: bool,
    #[prop(optional)] motion: TooltipMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional, into)] dir: Option<String>,
) -> impl IntoView {
    let accessibility_state =
        logic::normalize_accessibility_state(logic::AccessibilityStateInput { is_disabled });
    let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {
        is_open,
        default_open,
        on_open_change,
    });
    let is_disabled = accessibility_state.is_disabled;
    let open = normalized_open_state.open;
    let default_open = normalized_open_state.default_open;
    let on_open_change = normalized_open_state.on_open_change;
    let is_controlled = normalized_open_state.is_controlled;
    let has_custom_open = normalized_open_state.has_custom_open;
    let has_custom_default_open = normalized_open_state.has_custom_default_open;
    let has_custom_on_open_change = normalized_open_state.has_custom_on_open_change;
    let open_mode_attr = normalized_open_state.open_mode_attr;
    let open_source_attr = normalized_open_state.open_source_attr;
    let default_open_source_attr = normalized_open_state.default_open_source_attr;
    let open_change_source_attr = normalized_open_state.open_change_source_attr;
    debug_assert_eq!(
        open_mode_attr,
        if is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        }
    );

    let class_name = logic::normalize_optional_text(class_name);
    let lang = StoredValue::new(logic::normalize_optional_text(lang));
    let dir = logic::normalize_a11y_direction(dir);
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
            is_disabled,
            delay_ms,
            close_delay_ms,
            trigger,
            should_close_on_press,
            open,
            default_open,
            on_open_change,
        },
    );

    let tooltip_id: StoredValue<String> = StoredValue::new(trigger_aria.state.id().to_string());

    let open = trigger_aria.state.is_open();
    let presence = ui_headless::use_presence(open);

    let root_state = Memo::new(move |_| {
        logic::resolve_state(TooltipPartStateInput {
            slot: TooltipSlot::Root,
            open: open.get(),
            disabled: is_disabled,
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
            disabled: is_disabled,
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

    let focus_handlers = use_tooltip_focus_a11y(TooltipFocusA11yOptions {
        anchor_ref,
        tooltip_id,
        is_open: open,
        on_focus: trigger_aria.handlers.on_focus,
        on_blur: trigger_aria.handlers.on_blur,
    });
    let on_focus_in = focus_handlers.on_focus_in;
    let on_focus_out = focus_handlers.on_focus_out;

    let panel_vars =
        move || logic::compose_panel_vars(position.top_px.get(), position.left_px.get());
    let panel_a11y = Memo::new(move |_| {
        tooltip_panel_attrs(TooltipPanelA11yOptions {
            tooltip_id: tooltip_id.with_value(|id| id.clone()),
            is_open: open.get(),
            lang: lang.with_value(|value| value.clone()),
            dir,
        })
    });

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
            data-open-mode=open_mode_attr
            data-open-source=open_source_attr
            data-default-open-source=default_open_source_attr
            data-open-change-source=open_change_source_attr
            data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
            data-custom-delay=move || root_state.get().has_custom_delays.then_some("true")
            data-custom-trigger=move || root_state.get().has_custom_trigger_mode.then_some("true")
            data-custom-press=move || root_state.get().has_custom_press_behavior.then_some("true")
            data-custom-id=move || root_state.get().has_custom_id.then_some("true")
            data-custom-open=has_custom_open.then_some("true")
            data-custom-default-open=has_custom_default_open.then_some("true")
            data-custom-open-change=has_custom_on_open_change.then_some("true")
            node_ref=anchor_ref
            on:pointerenter=move |_| trigger_aria.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| trigger_aria.handlers.on_pointer_leave.run(())
            on:focusin=move |ev: ev::FocusEvent| on_focus_in.run(ev)
            on:focusout=move |ev: ev::FocusEvent| on_focus_out.run(ev)
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
                        id=move || panel_a11y.get().attrs.id.clone()
                        role=move || panel_a11y.get().attrs.role
                        lang=move || panel_a11y.get().attrs.lang.clone()
                        dir=move || panel_a11y.get().attrs.dir
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
                        data-open-mode=open_mode_attr
                        data-open-source=open_source_attr
                        data-default-open-source=default_open_source_attr
                        data-open-change-source=open_change_source_attr
                        data-custom-motion=move || panel_state.get().has_custom_motion.then_some("true")
                        data-custom-delay=move || panel_state.get().has_custom_delays.then_some("true")
                        data-custom-trigger=move || panel_state.get().has_custom_trigger_mode.then_some("true")
                        data-custom-press=move || panel_state.get().has_custom_press_behavior.then_some("true")
                        data-custom-id=move || panel_state.get().has_custom_id.then_some("true")
                        data-custom-open=has_custom_open.then_some("true")
                        data-custom-default-open=has_custom_default_open.then_some("true")
                        data-custom-open-change=has_custom_on_open_change.then_some("true")
                    >
                        {move || content.with_value(|content| content.run())}
                    </div>
                </Portal>
            </Show>
        </span>
    }
}
