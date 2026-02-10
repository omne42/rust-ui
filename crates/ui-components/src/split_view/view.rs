use crate::split_view::{SplitViewStateInput, logic};
use crate::{Resizable, ResizableOrientation};
use leptos::children::ViewFn;
use leptos::prelude::*;

#[component]
pub fn SplitView(
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
    let class_name_for_wrapper = class_name.clone();
    let class_name_for_inner = class_name.clone().unwrap_or_default();
    let has_custom_class_name = class_name.is_some();

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let has_custom_default_split = default_split_percent.is_some();
    let default_split_percent = logic::default_split_percent(default_split_percent);

    let has_custom_change_handler = on_split_percent_change.is_some();
    let on_split_percent_change =
        on_split_percent_change.unwrap_or_else(|| Callback::new(|_: f64| {}));

    let is_controlled = split_percent.is_some();
    let has_custom_bounds = logic::has_custom_bounds(min_split_percent, max_split_percent);

    let state = Signal::derive(move || {
        logic::resolve_state(SplitViewStateInput {
            orientation,
            disabled,
            with_handle,
            is_controlled,
            has_custom_default_split,
            has_custom_bounds,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_change_handler,
        })
    });

    let class = Signal::derive(move || {
        logic::compose_class_name(class_name_for_wrapper.clone(), state.get())
    });

    let first_uncontrolled = first.clone();
    let second_uncontrolled = second.clone();

    if let Some(split_percent) = split_percent {
        view! {
            <div
                class=move || class.get()
                data-slot="split-view"
                data-state=move || state.get().state_attr
                data-orientation=move || state.get().orientation_attr
                data-split-mode=move || state.get().split_mode_attr
                data-handle=move || state.get().handle_attr
                data-default-split-source=move || state.get().default_split_source_attr
                data-bounds-source=move || state.get().bounds_source_attr
                data-label-source=move || state.get().label_source_attr
                data-class-source=move || state.get().class_source_attr
                data-handler-source=move || state.get().handler_source_attr
                data-default-split-percent=default_split_percent.to_string()
                data-min-split-percent=min_split_percent.to_string()
                data-max-split-percent=max_split_percent.to_string()
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-enabled=move || state.get().is_enabled.then_some("true")
                data-controlled=move || state.get().is_controlled.then_some("true")
                data-uncontrolled=move || (!state.get().is_controlled).then_some("true")
                data-with-handle=move || state.get().with_handle.then_some("true")
                data-custom-default=move || state.get().has_custom_default_split.then_some("true")
                data-custom-bounds=move || state.get().has_custom_bounds.then_some("true")
                data-custom-label=move || state.get().has_custom_aria_label.then_some("true")
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-custom-handler=move || state.get().has_custom_change_handler.then_some("true")
            >
                <Resizable
                    orientation=orientation
                    split_percent=split_percent
                    default_split_percent=default_split_percent
                    on_split_percent_change=on_split_percent_change
                    min_split_percent=min_split_percent
                    max_split_percent=max_split_percent
                    disabled=disabled
                    with_handle=with_handle
                    aria_label=aria_label.clone()
                    class_name=class_name_for_inner.clone()
                    first=first
                    second=second
                />
            </div>
        }
        .into_any()
    } else {
        view! {
            <div
                class=move || class.get()
                data-slot="split-view"
                data-state=move || state.get().state_attr
                data-orientation=move || state.get().orientation_attr
                data-split-mode=move || state.get().split_mode_attr
                data-handle=move || state.get().handle_attr
                data-default-split-source=move || state.get().default_split_source_attr
                data-bounds-source=move || state.get().bounds_source_attr
                data-label-source=move || state.get().label_source_attr
                data-class-source=move || state.get().class_source_attr
                data-handler-source=move || state.get().handler_source_attr
                data-default-split-percent=default_split_percent.to_string()
                data-min-split-percent=min_split_percent.to_string()
                data-max-split-percent=max_split_percent.to_string()
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-enabled=move || state.get().is_enabled.then_some("true")
                data-controlled=move || state.get().is_controlled.then_some("true")
                data-uncontrolled=move || (!state.get().is_controlled).then_some("true")
                data-with-handle=move || state.get().with_handle.then_some("true")
                data-custom-default=move || state.get().has_custom_default_split.then_some("true")
                data-custom-bounds=move || state.get().has_custom_bounds.then_some("true")
                data-custom-label=move || state.get().has_custom_aria_label.then_some("true")
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-custom-handler=move || state.get().has_custom_change_handler.then_some("true")
            >
                <Resizable
                    orientation=orientation
                    default_split_percent=default_split_percent
                    on_split_percent_change=on_split_percent_change
                    min_split_percent=min_split_percent
                    max_split_percent=max_split_percent
                    disabled=disabled
                    with_handle=with_handle
                    aria_label=aria_label
                    class_name=class_name_for_inner
                    first=first_uncontrolled
                    second=second_uncontrolled
                />
            </div>
        }
        .into_any()
    }
}
