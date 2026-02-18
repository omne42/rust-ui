use crate::resizable::{
    logic::{self, ResizableOrientation},
    motion::{self, ResizableMotion},
};
use leptos::children::ViewFn;
use leptos::{ev, html, prelude::*};
use ui_headless::{self as headless, A11yDirection, ResizableOptions};

fn render_panel(
    slot: &'static str,
    class_name: &'static str,
    content: StoredValue<ViewFn>,
) -> impl IntoView {
    view! {
        <div class=class_name data-slot=slot>
            {content.get_value().run()}
        </div>
    }
}

fn render_handle_grip() -> impl IntoView {
    view! {
        <span class="ui-resizable__handle-grip" data-slot="resizable-handle-grip">
            <span class="ui-resizable__handle-dot"></span>
            <span class="ui-resizable__handle-dot"></span>
            <span class="ui-resizable__handle-dot"></span>
        </span>
    }
}

#[component]
pub fn Resizable(
    #[prop(optional)] orientation: ResizableOrientation,
    #[prop(optional)] value: Option<Signal<f64>>,
    #[prop(optional)] split_percent: Option<Signal<f64>>,
    #[prop(optional)] default_value: Option<f64>,
    #[prop(optional)] default_split_percent: Option<f64>,
    #[prop(optional)] on_value_change: Option<Callback<f64>>,
    #[prop(optional)] on_split_percent_change: Option<Callback<f64>>,
    #[prop(optional, default = crate::resizable::DEFAULT_MIN_SPLIT_PERCENT)] min_split_percent: f64,
    #[prop(optional, default = crate::resizable::DEFAULT_MAX_SPLIT_PERCENT)] max_split_percent: f64,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_with_handle: Option<bool>,
    #[prop(optional)] with_handle: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: ResizableMotion,
    #[prop(into)] first: ViewFn,
    #[prop(into)] second: ViewFn,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let aria_label = logic::normalize_aria_label(aria_label);
    let bounds = logic::normalize_bounds(min_split_percent, max_split_percent);

    let value_axis = logic::normalize_value_axis(logic::ResizableValueAxisInput {
        value,
        split_percent,
        default_value,
        default_split_percent,
        on_value_change,
        on_split_percent_change,
        bounds,
    });
    let agent_contract = logic::resolve_agent_contract(value_axis.value_change_source);
    let disabled_state = logic::normalize_disabled(logic::ResizableDisabledInput {
        is_disabled,
        disabled,
    });
    let handle_state = logic::normalize_handle(logic::ResizableHandleInput {
        is_with_handle,
        with_handle,
    });

    let is_controlled = value_axis.value.is_some();
    let split_state = headless::use_controllable_state(
        value_axis.value,
        Some(value_axis.default_value),
        value_axis.on_value_change,
    );

    let first = StoredValue::new(first);
    let second = StoredValue::new(second);
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    let resizable_aria = headless::use_resizable(ResizableOptions {
        orientation,
        split_percent: split_state.value,
        bounds,
        is_disabled: disabled_state.is_disabled,
        is_controlled,
        with_handle: handle_state.with_handle,
        has_custom_class_name,
        aria_label,
        lang,
        dir,
        on_split_percent_change: split_state.request_change,
    });

    let class = Signal::derive(move || {
        logic::compose_class_name(class_name.get_value(), resizable_aria.state.resolved.get())
    });

    let motion = motion::sanitize_motion(motion);
    let inline_style = StoredValue::new(Some(motion::motion_style_vars(motion)));
    motion::attach_motion(root_ref, resizable_aria.state.is_dragging.into(), motion);

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            style=inline_style.get_value().unwrap_or_default()
            data-slot="resizable"
            data-orientation=move || resizable_aria.state.resolved.get().orientation_attr
            data-state=move || resizable_aria.state.resolved.get().state_attr
            data-disabled=move || resizable_aria.state.resolved.get().disabled.then_some("true")
            data-enabled=move || resizable_aria.state.resolved.get().enabled.then_some("true")
            data-dragging=move || resizable_aria.state.resolved.get().dragging.then_some("true")
            data-idle=move || resizable_aria.state.resolved.get().idle.then_some("true")
            data-controlled=move || resizable_aria.state.resolved.get().is_controlled.then_some("true")
            data-uncontrolled=move || resizable_aria.state.resolved.get().is_uncontrolled.then_some("true")
            data-handle=move || resizable_aria.state.resolved.get().handle_attr
            data-class-source=move || resizable_aria.state.resolved.get().class_source_attr
            data-custom-class=move || resizable_aria.state.resolved.get().has_custom_class_name.then_some("true")
            data-control-mode=value_axis.control_mode_attr
            data-value-source=value_axis.value_source_attr
            data-default-value-source=value_axis.default_value_source_attr
            data-value-change-source=value_axis.value_change_source_attr
            data-disabled-source=disabled_state.disabled_source_attr
            data-handle-source=handle_state.with_handle_source_attr
            data-ui-schema=agent_contract.schema_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action-model=agent_contract.action_model_attr
            data-ui-state-axis=agent_contract.state_axis_attr
            data-ui-source-axis=agent_contract.source_axis_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-stream-mode=agent_contract.stream_mode_attr
            data-ui-output-status=agent_contract.output_status_attr
            lang=move || resizable_aria.attrs.lang.clone()
            dir=move || resizable_aria.attrs.dir
            on:pointermove=move |event: ev::PointerEvent| {
                let Some(root) = root_ref.get() else {
                    return;
                };
                resizable_aria.handlers.on_pointer_move.run((
                    f64::from(event.client_x()),
                    f64::from(event.client_y()),
                    f64::from(root.client_width()),
                    f64::from(root.client_height()),
                ));
            }
            on:pointerup=move |_| resizable_aria.handlers.on_pointer_up.run(())
            on:pointerleave=move |_| resizable_aria.handlers.on_pointer_up.run(())
        >
            {render_panel(
                "resizable-panel-first",
                "ui-resizable__panel ui-resizable__panel--first",
                first
            )}

            <div
                class="ui-resizable__handle"
                data-slot="resizable-handle"
                data-disabled=move || resizable_aria.state.resolved.get().disabled.then_some("true")
                data-dragging=move || resizable_aria.state.resolved.get().dragging.then_some("true")
                data-with-handle=move || resizable_aria.state.resolved.get().with_handle.then_some("true")
                role=resizable_aria.handle_attrs.role
                tabindex=resizable_aria.handle_attrs.tabindex
                aria-label=move || resizable_aria.handle_attrs.aria_label.clone()
                aria-orientation=move || resizable_aria.handle_attrs.aria_orientation.get()
                aria-valuemin=move || resizable_aria.handle_attrs.aria_valuemin.get()
                aria-valuemax=move || resizable_aria.handle_attrs.aria_valuemax.get()
                aria-valuenow=move || resizable_aria.handle_attrs.aria_valuenow.get()
                aria-disabled=move || resizable_aria.handle_attrs.aria_disabled.get()
                lang=move || resizable_aria.handle_attrs.lang.clone()
                dir=move || resizable_aria.handle_attrs.dir
                on:pointerdown=move |event: ev::PointerEvent| {
                    if resizable_aria.handlers.on_handle_pointer_down.run((
                        f64::from(event.client_x()),
                        f64::from(event.client_y()),
                    )) {
                        event.prevent_default();
                    }
                }
                on:keydown=move |event: ev::KeyboardEvent| {
                    if resizable_aria
                        .handlers
                        .on_handle_key_down
                        .run((event.key(), event.shift_key()))
                    {
                        event.prevent_default();
                    }
                }
            >
                {render_handle_grip()}
            </div>

            {render_panel(
                "resizable-panel-second",
                "ui-resizable__panel ui-resizable__panel--second",
                second
            )}
        </div>
    }
}
