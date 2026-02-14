use crate::resizable::logic::{self, ResizableOrientation, ResizableStateInput};
use leptos::children::ViewFn;
use leptos::{ev, html, prelude::*};
use ui_headless as overlay_open;

#[component]
pub fn Resizable(
    #[prop(optional)] orientation: ResizableOrientation,
    #[prop(optional)] split_percent: Option<Signal<f64>>,
    #[prop(optional)] default_split_percent: Option<f64>,
    #[prop(optional)] on_split_percent_change: Option<Callback<f64>>,
    #[prop(optional, default = crate::resizable::DEFAULT_MIN_SPLIT_PERCENT)] min_split_percent: f64,
    #[prop(optional, default = crate::resizable::DEFAULT_MAX_SPLIT_PERCENT)] max_split_percent: f64,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] with_handle: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(into)] first: ViewFn,
    #[prop(into)] second: ViewFn,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let aria_label = logic::normalize_aria_label(aria_label);
    let bounds = logic::normalize_bounds(min_split_percent, max_split_percent);
    let default_split_percent = logic::normalize_split(default_split_percent, bounds);

    let is_controlled = split_percent.is_some();
    let split_state = overlay_open::use_controllable_state(
        split_percent,
        Some(default_split_percent),
        on_split_percent_change,
    );
    let split_percent = split_state.value;
    let request_split_percent_change = split_state.request_change;

    let first = StoredValue::new(first);
    let second = StoredValue::new(second);

    let (dragging, set_dragging) = signal(false);
    let (drag_start_position, set_drag_start_position) = signal(0.0_f64);
    let (drag_start_split_percent, set_drag_start_split_percent) =
        signal(split_percent.get_untracked());

    let root_ref: NodeRef<html::Div> = NodeRef::new();

    let state = Signal::derive(move || {
        logic::resolve_state(ResizableStateInput {
            orientation,
            split_percent: split_percent.get(),
            bounds,
            disabled,
            dragging: dragging.get(),
            is_controlled,
            with_handle,
            has_custom_class_name,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let pointer_position = move |event: &ev::PointerEvent| match orientation {
        ResizableOrientation::Horizontal => event.client_x() as f64,
        ResizableOrientation::Vertical => event.client_y() as f64,
    };

    let on_handle_pointer_down = move |event: ev::PointerEvent| {
        if disabled {
            return;
        }

        set_dragging.set(true);
        set_drag_start_position.set(pointer_position(&event));
        set_drag_start_split_percent.set(state.get_untracked().split_percent);
        event.prevent_default();
    };

    let on_pointer_move = move |event: ev::PointerEvent| {
        if disabled || !dragging.get_untracked() {
            return;
        }

        let Some(root) = root_ref.get() else {
            return;
        };

        let extent = match orientation {
            ResizableOrientation::Horizontal => root.client_width() as f64,
            ResizableOrientation::Vertical => root.client_height() as f64,
        };

        let next = logic::split_from_drag(
            drag_start_split_percent.get_untracked(),
            drag_start_position.get_untracked(),
            pointer_position(&event),
            extent,
            bounds,
        );

        request_split_percent_change.run(next);
    };

    let on_pointer_up = move |_| {
        if dragging.get_untracked() {
            set_dragging.set(false);
        }
    };

    let on_handle_key_down = move |event: ev::KeyboardEvent| {
        if disabled {
            return;
        }

        let key = event.key();
        let Some(delta) = logic::split_step_for_key(&key, orientation, event.shift_key()) else {
            return;
        };

        event.prevent_default();

        let next = logic::clamp_split(state.get_untracked().split_percent + delta, bounds);
        request_split_percent_change.run(next);
    };

    let handle_tab_index = if disabled { -1 } else { 0 };

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            data-slot="resizable"
            data-orientation=move || state.get().orientation_attr
            data-state=move || state.get().state_attr
            data-disabled=move || state.get().disabled.then_some("true")
            data-enabled=move || state.get().enabled.then_some("true")
            data-dragging=move || state.get().dragging.then_some("true")
            data-idle=move || state.get().idle.then_some("true")
            data-controlled=move || state.get().is_controlled.then_some("true")
            data-uncontrolled=move || state.get().is_uncontrolled.then_some("true")
            data-handle=move || state.get().handle_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:pointerleave=on_pointer_up
        >
            <div class="ui-resizable__panel ui-resizable__panel--first" data-slot="resizable-panel-first">
                {first.get_value().run()}
            </div>

            <div
                class="ui-resizable__handle"
                data-slot="resizable-handle"
                data-disabled=move || state.get().disabled.then_some("true")
                data-dragging=move || state.get().dragging.then_some("true")
                data-with-handle=move || state.get().with_handle.then_some("true")
                role="separator"
                tabindex=handle_tab_index
                aria-label=aria_label
                aria-orientation=move || state.get().orientation_attr
                aria-valuemin=move || format!("{:.2}", state.get().min_split_percent)
                aria-valuemax=move || format!("{:.2}", state.get().max_split_percent)
                aria-valuenow=move || format!("{:.2}", state.get().split_percent)
                aria-disabled=move || state.get().disabled.then_some("true")
                on:pointerdown=on_handle_pointer_down
                on:keydown=on_handle_key_down
            >
                <span class="ui-resizable__handle-grip" data-slot="resizable-handle-grip">
                    <span class="ui-resizable__handle-dot"></span>
                    <span class="ui-resizable__handle-dot"></span>
                    <span class="ui-resizable__handle-dot"></span>
                </span>
            </div>

            <div class="ui-resizable__panel ui-resizable__panel--second" data-slot="resizable-panel-second">
                {second.get_value().run()}
            </div>
        </div>
    }
}
