use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    ResizableOrientation, SplitView, View, ViewBackground, ViewBorder, ViewPadding, ViewRadius,
};

pub(super) fn split_view() -> AnyView {
    let (split_raw, set_split_raw) = signal(62.0_f64);
    let split: Signal<f64> = Signal::derive(move || split_raw.get());
    let on_split_change = Callback::new(move |next: f64| set_split_raw.set(next));

    let (marker_split_raw, set_marker_split_raw) = signal(58.0_f64);
    let marker_split: Signal<f64> = Signal::derive(move || marker_split_raw.get());
    let marker_on_split_change = Callback::new(move |next: f64| set_marker_split_raw.set(next));

    let horizontal_code = r#"<SplitView
  orientation=ResizableOrientation::Horizontal
  default_split_percent=40.0
  with_handle=true
  first=move || view! { <div>\"Navigation\"</div> }
  second=move || view! { <div>\"Inspector\"</div> }
/>"#;

    let vertical_code = r#"let (split_raw, set_split_raw) = signal(62.0_f64);
let split: Signal<f64> = Signal::derive(move || split_raw.get());

<SplitView
  orientation=ResizableOrientation::Vertical
  split_percent=split
  on_split_percent_change=Callback::new(move |next| set_split_raw.set(next))
  min_split_percent=30.0
  max_split_percent=80.0
  with_handle=true
  class_name=\"docs-split-view-custom\".to_string()
  first=move || view! { <div>\"Header\"</div> }
  second=move || view! { <div>\"Body\"</div> }
/>"#;

    let markers_code = r#"let (split_raw, set_split_raw) = signal(58.0_f64);
let split: Signal<f64> = Signal::derive(move || split_raw.get());

<SplitView
  orientation=ResizableOrientation::Vertical
  split_percent=split
  on_split_percent_change=Callback::new(move |next| set_split_raw.set(next))
  default_split_percent=55.0
  min_split_percent=25.0
  max_split_percent=75.0
  with_handle=true
  aria_label=\"Markers split panel\".to_string()
  class_name=\"docs-split-view-state\".to_string()
  first=move || view! { <div>\"Left\"</div> }
  second=move || view! { <div>\"Right\"</div> }
/>"#;

    view! {
        <ComponentPage
            title="SplitView"
            slug="split-view"
            group="Layout"
            description="Spectrum-compatible SplitView alias for upstream naming parity, preserving Resizable controlled/uncontrolled split contracts and HeroUI-level drag/keyboard handle interaction behavior."
        >
            <Playground title="Horizontal + Default Split" code=horizontal_code>
                <SplitView
                    orientation=ResizableOrientation::Horizontal
                    default_split_percent=40.0
                    with_handle=true
                    first=move || {
                        view! {
                            <View
                                background=ViewBackground::Subtle
                                border=ViewBorder::Subtle
                                padding=ViewPadding::Md
                                radius=ViewRadius::None
                            >
                                <strong>"Navigation"</strong>
                            </View>
                        }
                    }
                    second=move || {
                        view! {
                            <View
                                background=ViewBackground::Default
                                border=ViewBorder::None
                                padding=ViewPadding::Md
                                radius=ViewRadius::None
                            >
                                <strong>"Inspector"</strong>
                            </View>
                        }
                    }
                />
            </Playground>

            <Playground title="Controlled + Vertical Bounds" code=vertical_code>
                <div class="docs-stack docs-stack--tight">
                    <SplitView
                        orientation=ResizableOrientation::Vertical
                        split_percent=split
                        on_split_percent_change=on_split_change
                        min_split_percent=30.0
                        max_split_percent=80.0
                        with_handle=true
                        aria_label="Split analytics regions".to_string()
                        class_name="docs-split-view-custom".to_string()
                        first=move || {
                            view! {
                                <View
                                    background=ViewBackground::Subtle
                                    border=ViewBorder::Subtle
                                    padding=ViewPadding::Md
                                    radius=ViewRadius::None
                                >
                                    <strong>"Header"</strong>
                                </View>
                            }
                        }
                        second=move || {
                            view! {
                                <View
                                    background=ViewBackground::Default
                                    border=ViewBorder::None
                                    padding=ViewPadding::Md
                                    radius=ViewRadius::None
                                >
                                    <strong>"Body"</strong>
                                </View>
                            }
                        }
                    />
                    <span class="ui-muted">
                        "controlled split: "
                        {move || format!("{:.1}%", split_raw.get())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-orientation`, `data-split-mode`, `data-handle`, `data-default-split-source`, `data-bounds-source`, `data-label-source`, `data-class-source`, and `data-handler-source`."
                code=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <SplitView
                        orientation=ResizableOrientation::Vertical
                        split_percent=marker_split
                        on_split_percent_change=marker_on_split_change
                        default_split_percent=55.0
                        min_split_percent=25.0
                        max_split_percent=75.0
                        with_handle=true
                        aria_label="Markers split panel".to_string()
                        class_name="docs-split-view-state".to_string()
                        first=move || {
                            view! {
                                <View
                                    background=ViewBackground::Subtle
                                    border=ViewBorder::Subtle
                                    padding=ViewPadding::Md
                                    radius=ViewRadius::None
                                >
                                    <strong>"Left"</strong>
                                </View>
                            }
                        }
                        second=move || {
                            view! {
                                <View
                                    background=ViewBackground::Default
                                    border=ViewBorder::None
                                    padding=ViewPadding::Md
                                    radius=ViewRadius::None
                                >
                                    <strong>"Right"</strong>
                                </View>
                            }
                        }
                    />
                    <span class="ui-muted">
                        "marker split: "
                        {move || format!("{:.1}%", marker_split_raw.get())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
