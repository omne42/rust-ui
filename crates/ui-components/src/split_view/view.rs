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
    let default_split_percent = default_split_percent.unwrap_or(50.0);
    let on_split_percent_change =
        on_split_percent_change.unwrap_or_else(|| Callback::new(|_: f64| {}));
    let aria_label = aria_label.unwrap_or_default();
    let class_name = class_name.unwrap_or_default();

    let first_uncontrolled = first.clone();
    let second_uncontrolled = second.clone();

    if let Some(split_percent) = split_percent {
        view! {
            <Resizable
                orientation=orientation
                split_percent=split_percent
                default_split_percent=default_split_percent
                on_split_percent_change=on_split_percent_change
                min_split_percent=min_split_percent
                max_split_percent=max_split_percent
                disabled=disabled
                with_handle=with_handle
                aria_label=aria_label
                class_name=class_name
                first=first
                second=second
            />
        }
        .into_any()
    } else {
        view! {
            <Resizable
                orientation=orientation
                default_split_percent=default_split_percent
                on_split_percent_change=on_split_percent_change
                min_split_percent=min_split_percent
                max_split_percent=max_split_percent
                disabled=disabled
                with_handle=with_handle
                aria_label=aria_label
                class_name=class_name
                first=first_uncontrolled
                second=second_uncontrolled
            />
        }
        .into_any()
    }
}
